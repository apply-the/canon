use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn run_pr_review(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let (base_ref, head_ref) = self.load_pr_review_refs(&request.inputs)?;
        let is_worktree = head_ref == "WORKTREE";
        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let diff_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::PrReview,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::InspectDiff,
            summary: "inspect diff surfaces and bounded patch context",
            scope: input_scope.clone(),
        });
        let diff_decision = invocation_runtime::evaluate_request_policy(&diff_request, &policy_set);

        let shell = ShellAdapter;
        let diff = if is_worktree {
            shell.git_diff_worktree(&base_ref, &self.repo_root)
        } else {
            shell.git_diff(&base_ref, &head_ref, &self.repo_root)
        }
        .map_err(|error| {
            EngineError::Validation(format!("unable to collect pr-review diff: {error}"))
        })?;

        let changed_files_ref = store.persist_invocation_payload_text(
            &run_id,
            &diff_request.request_id,
            "changed-files.txt",
            &diff.changed_files_text,
        )?;
        let patch_ref = store.persist_invocation_payload_text(
            &run_id,
            &diff_request.request_id,
            "diff.patch",
            &diff.patch,
        )?;
        let diff_attempt = self.completed_attempt(
            &diff_request,
            1,
            "shell:git-diff",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Inspected {} changed surface(s) between {base_ref} and {head_ref}.",
                    diff.changed_files.len()
                ),
                exit_code: Some(0),
                payload_refs: vec![changed_files_ref, patch_ref],
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::PrReview,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique the reviewed diff and preserve reviewer-facing evidence",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_input = format!(
            "Review {base_ref}..{head_ref} across {} changed surface(s): {}",
            diff.changed_files.len(),
            if diff.changed_files.is_empty() {
                "no changed surfaces detected".to_string()
            } else {
                diff.changed_files.join(", ")
            }
        );
        let copilot = CopilotCliAdapter;
        let critique_output = copilot.critique(&critique_input);
        let critique_attempt = self.completed_attempt(
            &critique_request,
            1,
            &critique_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: critique_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let review_packet = ReviewPacket::from_evidence(
            &diff.base_ref,
            &diff.head_ref,
            diff.changed_files.clone(),
            &diff.patch,
            &critique_output.summary,
        );
        let review_summary = ReviewSummary::from_evidence(&review_packet, false);
        let artifact_paths = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                format!("artifacts/{}/{}/{}", run_id, request.mode.as_str(), requirement.file_name)
            })
            .collect::<Vec<_>>();
        let generation_path = GenerationPath {
            path_id: format!("generation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_paths.clone(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", diff_request.request_id),
            request_ids: vec![diff_request.request_id.clone()],
            lineage_classes: vec![LineageClass::NonGenerative],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                diff_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", diff_request.request_id),
                    request_ids: vec![diff_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::NonGenerative],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        diff_request.request_id
                    )],
                    independence: evidence_builder::default_independence(&generation_path.path_id),
                },
            ),
        };

        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| PersistedArtifact {
                record: ArtifactRecord {
                    file_name: requirement.file_name.clone(),
                    relative_path: format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    ),
                    format: requirement.format,
                    provenance: Some(crate::domain::artifact::ArtifactProvenance {
                        request_ids: vec![
                            diff_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition:
                            crate::domain::execution::EvidenceDisposition::NeedsDisposition,
                    }),
                },
                contents: render_pr_review_artifact(
                    &requirement.file_name,
                    &review_packet,
                    &review_summary,
                ),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_pr_review_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::PrReviewGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                denied_invocations: &[],
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);
        let mut verification_records = verification_runner::pr_review_verification_records(
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids =
                vec![diff_request.request_id.clone(), critique_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }
        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", diff_request.request_id),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
            state: RunStateManifest { state, updated_at: now },
            artifact_contract: artifact_contract.clone(),
            links: LinkManifest {
                artifacts: artifact_paths,
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals,
            evidence: Some(evidence),
            invocations: vec![
                PersistedInvocation {
                    request: diff_request,
                    decision: diff_decision,
                    attempts: vec![diff_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        store.persist_adapter_invocations(&run_id, &diff.invocations)?;
        self.summarize_run(
            store,
            RunSummarySpec {
                run_id: &run_id,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                state,
                artifact_count: bundle.artifacts.len(),
            },
        )
    }
}
