use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn run_incident(
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

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture incident packet context and bounded operational surface",
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured incident context from {} authored input(s).",
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded incident containment packet",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let copilot = CopilotCliAdapter;
        let generation_output = copilot.generate(&context_summary);
        let generation_attempt = self.completed_attempt(
            &generation_request,
            1,
            &generation_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: generation_output.summary.clone(),
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

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique incident containment posture, evidence gaps, and follow-up readiness",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot.critique(&generation_output.summary);
        let critique_attempt = self.completed_attempt(
            &critique_request,
            1,
            &critique_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: critique_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let validation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: "validate incident packet against repository surface",
            scope: input_scope.clone(),
        });
        let validation_decision =
            invocation_runtime::evaluate_request_policy(&validation_request, &policy_set);
        let (_validation_summary, validation_attempt) =
            self.change_validation_attempt(&validation_request)?;

        let artifact_paths = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                format!("artifacts/{}/{}/{}", run_id, request.mode.as_str(), requirement.file_name)
            })
            .collect::<Vec<_>>();
        let generation_path = GenerationPath {
            path_id: format!("generation:{}", generation_request.request_id),
            request_ids: vec![generation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_paths.clone(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", validation_request.request_id),
            request_ids: vec![validation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::NonGenerative],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                validation_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", validation_request.request_id),
                    request_ids: vec![validation_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::NonGenerative],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        validation_request.request_id
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
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                            validation_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_incident_artifact(&requirement.file_name, &context_summary),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_incident_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::IncidentGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                validation_independence_satisfied: validation_path.independence.sufficient,
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "incident",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        verification_runner::attach_runtime_lineage(
            &mut verification_records,
            &[
                generation_request.request_id.clone(),
                critique_request.request_id.clone(),
                validation_request.request_id.clone(),
            ],
            &validation_path.path_id,
            &evidence_path,
        );

        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path.clone()],
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    validation_request.request_id
                ),
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
                artifacts: artifact_paths.clone(),
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
                    request: context_request,
                    decision: context_decision,
                    attempts: vec![context_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: generation_request,
                    decision: generation_decision,
                    attempts: vec![generation_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: validation_request,
                    decision: validation_decision,
                    attempts: vec![validation_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
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
