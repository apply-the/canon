use super::EngineService;
use super::*;

struct RequirementsResumeState<'a> {
    manifest: &'a RunManifest,
    context: &'a RunContext,
    contract: &'a crate::domain::artifact::ArtifactContract,
    approvals: &'a [ApprovalRecord],
    artifacts: &'a [PersistedArtifact],
}

impl EngineService {
    /// Executes a managed run according to the provided request parameters.
    /// This handles policy validation, risk classification, and artifact emission.
    pub fn run(&self, mut request: RunRequest) -> Result<RunSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        store.init_runtime_state(None)?;

        let policy_root = request.policy_root.as_deref().map(|root| {
            let root = PathBuf::from(root);
            if root.is_absolute() { root } else { self.repo_root.join(root) }
        });
        let policy_set = store.load_policy_set(policy_root.as_deref())?;
        classifier::validate_system_context(request.mode, request.system_context)
            .map_err(EngineError::Validation)?;
        let owner_supplied_explicitly = !request.owner.trim().is_empty();
        request.owner = self.resolve_owner(&request.owner);
        classifier::classify_owner_requirement(&policy_set, request.risk, &request.owner).map_err(
            |error| {
                if !owner_supplied_explicitly && request.owner.trim().is_empty() {
                    EngineError::Validation(format!(
                        "{error}; pass --owner or configure git user.name and user.email"
                    ))
                } else {
                    EngineError::Validation(error)
                }
            },
        )?;
        request.inputs = self.auto_bind_canonical_mode_inputs(
            request.mode,
            &request.inputs,
            &request.inline_inputs,
        );
        self.validate_authored_inputs(request.mode, &request.inputs, &request.inline_inputs)?;

        if Self::is_targeted_refinement_mode(request.mode) {
            return self.start_refinement_draft(&store, request, &policy_set);
        }

        match request.mode {
            Mode::Requirements => self.run_requirements(&store, request, policy_set),
            Mode::Discovery => self.run_discovery(&store, request, policy_set),
            Mode::SystemShaping => self.run_system_shaping(&store, request, policy_set),
            Mode::Change => self.run_change(&store, request, policy_set),
            Mode::Backlog => self.run_backlog(&store, request, policy_set),
            Mode::Incident => self.run_incident(&store, request, policy_set),
            Mode::SystemAssessment => self.run_system_assessment(&store, request, policy_set),
            Mode::SecurityAssessment => self.run_security_assessment(&store, request, policy_set),
            Mode::SupplyChainAnalysis => {
                self.run_supply_chain_analysis(&store, request, policy_set)
            }
            Mode::Implementation => self.run_implementation(&store, request, policy_set),
            Mode::Migration => self.run_migration(&store, request, policy_set),
            Mode::Refactor => self.run_refactor(&store, request, policy_set),
            Mode::Architecture => self.run_architecture(&store, request, policy_set),
            Mode::Review => self.run_review(&store, request, policy_set),
            Mode::Verification => self.run_verification(&store, request, policy_set),
            Mode::PrReview => self.run_pr_review(&store, request, policy_set),
            Mode::DomainLanguage => self.run_domain_language(&store, request, policy_set),
            Mode::DomainModel => self.run_domain_model(&store, request, policy_set),
        }
    }

    /// Records an approval decision (approve or reject) against a gate or invocation target.
    pub fn approve(
        &self,
        run_id: &str,
        target: &str,
        by: &str,
        decision: ApprovalDecision,
        rationale: &str,
    ) -> Result<ApprovalSummary, EngineError> {
        let approver_supplied_explicitly = !by.trim().is_empty();
        let approver = self.resolve_approver(by);
        if !approver_supplied_explicitly && approver.trim().is_empty() {
            return Err(EngineError::Validation(
                "missing approver identity; pass --by or configure git user.name and user.email"
                    .to_string(),
            ));
        }

        let store = WorkspaceStore::new(&self.repo_root);
        let canonical = self.resolve_run(run_id)?;
        let run_id = canonical.as_str();
        let manifest = store.load_run_manifest(run_id)?;
        let contract = store.load_artifact_contract(run_id)?;
        let context = store.load_run_context(run_id)?;
        let mut approvals = store.load_approval_records(run_id)?;
        let artifacts =
            store.load_persisted_artifacts(run_id, manifest.mode, &contract).unwrap_or_default();

        let target_label = target.to_string();
        let approval = if let Some(gate) = target.strip_prefix("gate:") {
            ApprovalRecord::for_gate(
                gate.parse::<GateKind>()
                    .map_err(|error| EngineError::Validation(error.to_string()))?,
                approver.clone(),
                decision,
                rationale.to_string(),
                OffsetDateTime::now_utc(),
            )
        } else if let Some(request_id) = target.strip_prefix("invocation:") {
            ApprovalRecord::for_invocation(
                request_id.to_string(),
                approver,
                decision,
                rationale.to_string(),
                OffsetDateTime::now_utc(),
            )
        } else {
            return Err(EngineError::Validation(format!("unsupported approval target `{target}`")));
        };
        store.persist_approval_record(run_id, &approval)?;
        approvals.push(approval.clone());

        let state = if approval.gate.is_some() {
            self.refresh_run_state(&store, &manifest, &context, &contract, &artifacts, &approvals)?
        } else {
            store.load_run_state(run_id)?.state
        };

        Ok(ApprovalSummary {
            run_id: run_id.to_string(),
            target: target_label,
            approved_by: approval.by.clone(),
            recorded_at: approval.recorded_at.to_string(),
            decision: decision.as_str().to_string(),
            state: format!("{state:?}"),
        })
    }

    /// Attempts to resume a previously blocked run from its persisted state.
    pub fn resume(&self, run_id: &str) -> Result<RunSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let canonical = self.resolve_run(run_id)?;
        let run_id = canonical.as_str();
        let manifest = store.load_run_manifest(run_id)?;
        let mut context = store.load_run_context(run_id)?;

        if !resume::input_fingerprints_match(&self.repo_root, &context.input_fingerprints)? {
            return Err(EngineError::Validation(format!(
                "stale run `{run_id}`: input context changed; fork or rerun instead"
            )));
        }

        if Self::capture_explicit_continuation(&mut context) {
            store.persist_run_context(run_id, &context)?;
        }

        let contract = store.load_artifact_contract(run_id)?;
        let approvals = store.load_approval_records(run_id)?;
        let artifacts =
            store.load_persisted_artifacts(run_id, manifest.mode, &contract).unwrap_or_default();

        let resume_state = RequirementsResumeState {
            manifest: &manifest,
            context: &context,
            contract: &contract,
            approvals: &approvals,
            artifacts: &artifacts,
        };

        if let Some(summary) =
            self.resume_requirements_without_artifacts(&store, run_id, &resume_state)?
        {
            return Ok(summary);
        }

        let should_resume_change_execution = Self::should_resume_change_execution(
            manifest.mode,
            artifacts.is_empty(),
            &context,
            &approvals,
        );

        if should_resume_change_execution {
            let policy_set = store.load_policy_set(None)?;
            let request = RunRequest {
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                system_context: context.system_context,
                classification: manifest.classification.clone(),
                owner: manifest.owner.clone(),
                inputs: self.resume_inputs(&context),
                inline_inputs: Vec::new(),
                excluded_paths: context.excluded_paths.clone(),
                policy_root: None,
                method_root: None,
            };
            return self.execute_change(
                &store,
                request,
                policy_set,
                manifest.to_identity().ok_or_else(|| {
                    EngineError::Validation(format!(
                        "run `{run_id}` is missing identity metadata; cannot resume"
                    ))
                })?,
                approvals.clone(),
            );
        }

        let state =
            self.refresh_run_state(&store, &manifest, &context, &contract, &artifacts, &approvals)?;

        self.summarize_run(
            &store,
            RunSummarySpec {
                run_id,
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                state,
                artifact_count: artifacts.len(),
            },
        )
    }

    fn resume_requirements_without_artifacts(
        &self,
        store: &WorkspaceStore,
        run_id: &str,
        state: &RequirementsResumeState<'_>,
    ) -> Result<Option<RunSummary>, EngineError> {
        if !matches!(state.manifest.mode, Mode::Requirements) || !state.artifacts.is_empty() {
            return Ok(None);
        }

        let generation_request_id = format!("{run_id}-generate");
        let approved_generation = state.approvals.iter().any(|approval| {
            approval.matches_invocation(&generation_request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });

        if !approved_generation {
            return self
                .summarize_run(
                    store,
                    RunSummarySpec {
                        run_id,
                        mode: state.manifest.mode,
                        risk: state.manifest.risk,
                        zone: state.manifest.zone,
                        state: RunState::AwaitingApproval,
                        artifact_count: 0,
                    },
                )
                .map(Some);
        }

        let policy_set = store.load_policy_set(None)?;
        let request = RunRequest {
            mode: state.manifest.mode,
            risk: state.manifest.risk,
            zone: state.manifest.zone,
            system_context: state.context.system_context,
            classification: state.manifest.classification.clone(),
            owner: state.manifest.owner.clone(),
            inputs: self.resume_inputs(state.context),
            inline_inputs: Vec::new(),
            excluded_paths: state.context.excluded_paths.clone(),
            policy_root: None,
            method_root: None,
        };
        let input_scope = request.merged_input_sources();
        let now = OffsetDateTime::now_utc();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.requirements_request(RequirementsRequestSpec {
            run_id,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            capability: CapabilityKind::ReadRepository,
            summary: "capture repository and idea context",
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured requirements context from {} input(s).",
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );
        let generation_request = self.requirements_request(RequirementsRequestSpec {
            run_id,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded requirements framing",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let denied_edit_request = self.requirements_request(RequirementsRequestSpec {
            run_id,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            capability: CapabilityKind::ProposeWorkspaceEdit,
            summary: "attempt workspace mutation from requirements mode",
            scope: input_scope.clone(),
        });
        let denied_edit_decision =
            invocation_runtime::evaluate_request_policy(&denied_edit_request, &policy_set);
        let denied_invocations = if matches!(denied_edit_decision.kind, PolicyDecisionKind::Deny) {
            vec![DeniedInvocation {
                request_id: denied_edit_request.request_id.clone(),
                rationale: denied_edit_decision.rationale.clone(),
                policy_refs: denied_edit_decision.policy_refs.clone(),
                recorded_at: denied_edit_decision.decided_at,
            }]
        } else {
            Vec::new()
        };

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
                candidate_artifacts: state
                    .contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );
        let critique_request = self.requirements_request(RequirementsRequestSpec {
            run_id,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique generated requirements framing",
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

        let generation_path = GenerationPath {
            path_id: format!("generation:{}", generation_request.request_id),
            request_ids: vec![generation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: state
                .contract
                .artifact_requirements
                .iter()
                .map(|requirement| {
                    format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    )
                })
                .collect(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                critique_request.request_id
            )],
            independence: evidence_builder::default_independence(&generation_path.path_id),
        };
        let validation_path = ValidationPath {
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &validation_path,
            ),
            ..validation_path
        };
        let denied_summary = if denied_invocations.is_empty() {
            "No governed invocations were denied during requirements mode.".to_string()
        } else {
            denied_invocations
                .iter()
                .map(|denied| denied.rationale.clone())
                .collect::<Vec<_>>()
                .join(" ")
        };
        let artifacts = state
            .contract
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
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_requirements_artifact_from_evidence(
                    &requirement.file_name,
                    &context_summary,
                    &context_summary,
                    &generation_output.summary,
                    &critique_output.summary,
                    &denied_summary,
                ),
            })
            .collect::<Vec<_>>();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_requirements_gates(
            state.contract,
            &gate_inputs,
            &request.owner,
            &denied_invocations,
            true,
        );
        let run_state = run_state_from_gates(&gates);
        let artifact_paths = artifacts
            .iter()
            .map(|artifact| artifact.record.relative_path.clone())
            .collect::<Vec<_>>();
        let mut verification_records =
            verification_runner::requirements_verification_records(&artifact_paths);
        for record in &mut verification_records {
            record.request_ids =
                vec![generation_request.request_id.clone(), critique_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }
        let evidence = EvidenceBundle {
            run_id: run_id.to_string(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations,
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths,
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    denied_edit_request.request_id
                ),
            ],
            approval_refs: state
                .approvals
                .iter()
                .filter_map(|approval| {
                    approval
                        .invocation_request_id
                        .as_ref()
                        .map(|request_id| format!("runs/{run_id}/approvals/{}", request_id))
                })
                .collect(),
        };

        let bundle = PersistedRunBundle {
            run: state.manifest.clone(),
            context: state.context.clone(),
            state: RunStateManifest { state: run_state, updated_at: now },
            artifact_contract: state.contract.clone(),
            links: LinkManifest {
                artifacts: artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals: state.approvals.to_vec(),
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
                    approvals: state
                        .approvals
                        .iter()
                        .filter(|approval| approval.matches_invocation(&generation_request_id))
                        .cloned()
                        .collect(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: denied_edit_request,
                    decision: denied_edit_decision,
                    attempts: Vec::new(),
                    approvals: Vec::new(),
                },
            ],
        };
        store.persist_run_bundle(&bundle)?;

        self.summarize_run(
            store,
            RunSummarySpec {
                run_id,
                mode: state.manifest.mode,
                risk: state.manifest.risk,
                zone: state.manifest.zone,
                state: run_state,
                artifact_count: bundle.artifacts.len(),
            },
        )
        .map(Some)
    }

    fn should_resume_change_execution(
        mode: Mode,
        artifacts_empty: bool,
        context: &RunContext,
        approvals: &[ApprovalRecord],
    ) -> bool {
        match mode {
            Mode::Change => artifacts_empty,
            Mode::Implementation | Mode::Refactor => {
                execution_continuation_pending(context, approvals)
            }
            _ => false,
        }
    }

    fn capture_explicit_continuation(context: &mut RunContext) -> bool {
        let Some(refinement) = context.clarification_refinement.as_mut() else {
            return false;
        };

        let had_pending_explicit_gate = refinement.explicit_continuation_required;
        let had_suggested_candidate = refinement.suggested_candidate.is_some();
        refinement.explicit_continuation_required = false;
        refinement.suggested_candidate = None;

        had_pending_explicit_gate || had_suggested_candidate
    }

    /// Returns a full status summary for the named run, including gates, approvals, and actions.
    pub fn status(&self, run: &str) -> Result<StatusSummary, EngineError> {
        let _ = all_mode_profiles();
        let store = WorkspaceStore::new(&self.repo_root);
        let canonical = self.resolve_run(run)?;
        let run = canonical.as_str();
        let manifest = store.load_run_manifest(run)?;
        let state = store.load_run_state(run)?;
        let details = self.collect_run_runtime_details(&store, run, manifest.mode, state.state)?;

        Ok(StatusSummary {
            run: run.to_string(),
            owner: manifest.owner,
            state: format!("{:?}", state.state),
            system_context: details.system_context.map(|context| context.as_str().to_string()),
            invocations_total: details.invocations_total,
            pending_invocation_approvals: details.pending_invocation_approvals,
            validation_independence_satisfied: details.validation_independence_satisfied,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            closure_status: details.closure_status,
            decomposition_scope: details.decomposition_scope,
            closure_findings: details.closure_findings,
            closure_notes: details.closure_notes,
            possible_actions: details.possible_actions,
            refinement_state: details.refinement_state,
            mode_result: details.mode_result,
            recommended_next_action: details.recommended_next_action,
        })
    }

    /// Publishes the artifacts from the named run to the given destination path.
    pub fn publish(
        &self,
        run: &str,
        to: Option<PathBuf>,
        adr: bool,
    ) -> Result<PublishSummary, EngineError> {
        let canonical = self.resolve_run(run)?;
        publish_run(&self.repo_root, &canonical, to.as_deref(), adr)
    }

    /// Publishes the artifacts from the named run using a fully-specified publish profile.
    pub fn publish_with_profile(
        &self,
        run: &str,
        profile: crate::domain::publish_profile::PublishProfile,
        to: Option<PathBuf>,
    ) -> Result<PublishSummary, EngineError> {
        let canonical = self.resolve_run(run)?;
        crate::orchestrator::publish::publish_run_with_profile(
            &self.repo_root,
            &canonical,
            profile,
            to.as_deref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::domain::approval::ApprovalDecision;
    use crate::domain::execution::{ExecutionPosture, MutationBounds, MutationExpansionPolicy};
    use crate::domain::run::{
        ClarificationRefinementContext, ClarificationRefinementStatus,
        ContinuationCandidateSummary, InputFingerprint, InputSourceKind, RefinementWorkflowFamily,
        RunIdentity,
    };

    fn requirements_file_request(input: &str, owner: &str) -> RunRequest {
        RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: owner.to_string(),
            inputs: vec![input.to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    fn implementation_context() -> RunContext {
        RunContext {
            repo_root: "/tmp/repo".to_string(),
            owner: Some("Owner <owner@example.com>".to_string()),
            inputs: vec!["brief.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: vec![InputFingerprint {
                path: "brief.md".to_string(),
                source_kind: InputSourceKind::Path,
                size_bytes: 10,
                modified_unix_seconds: 1_700_000_000,
                content_digest_sha256: Some("abc".to_string()),
                snapshot_ref: None,
            }],
            system_context: None,
            upstream_context: None,
            implementation_execution: Some(ImplementationExecutionContext {
                plan_sources: vec!["canon-input/implementation.md".to_string()],
                mutation_bounds: MutationBounds {
                    declared_paths: Vec::new(),
                    owners: Vec::new(),
                    source_refs: Vec::new(),
                    expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                },
                task_targets: Vec::new(),
                safety_net: Vec::new(),
                execution_posture: ExecutionPosture::RecommendationOnly,
                rollback_expectations: Vec::new(),
                post_approval_execution_consumed: false,
            }),
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::now_utc(),
        }
    }

    #[test]
    fn run_dispatches_targeted_refinement_modes_to_draft_refinement() {
        let workspace = tempdir().expect("tempdir");
        std::fs::write(
            workspace.path().join("brief.md"),
            "# Requirements Brief\n\n## Problem\nBound it.\n\n## Outcome\nShip it.\n",
        )
        .expect("write brief");
        let service = EngineService::new(workspace.path());

        let summary = service
            .run(requirements_file_request("brief.md", "Owner <owner@example.com>"))
            .expect("requirements run");
        assert_eq!(summary.state, "Draft");
        assert_eq!(
            summary.refinement_state.expect("refinement state").current_mode,
            "requirements"
        );
    }

    #[test]
    fn approve_rejects_unsupported_targets() {
        let workspace = tempdir().expect("tempdir");
        std::fs::write(
            workspace.path().join("brief.md"),
            "# Requirements Brief\n\n## Problem\nBound it.\n\n## Outcome\nShip it.\n",
        )
        .expect("write brief");
        let service = EngineService::new(workspace.path());
        let summary = service
            .run(requirements_file_request("brief.md", "Owner <owner@example.com>"))
            .expect("requirements run");

        let unsupported = service
            .approve(
                &summary.run_id,
                "other:thing",
                "Reviewer <reviewer@example.com>",
                ApprovalDecision::Approve,
                "ok",
            )
            .expect_err("unsupported target should fail");
        assert!(unsupported.to_string().contains("unsupported approval target `other:thing`"));
    }

    #[test]
    fn resume_rejects_stale_input_fingerprints_after_source_changes() {
        let workspace = tempdir().expect("tempdir");
        let brief = workspace.path().join("brief.md");
        std::fs::write(
            &brief,
            "# Requirements Brief\n\n## Problem\nBound it.\n\n## Outcome\nShip it.\n",
        )
        .expect("write brief");
        let service = EngineService::new(workspace.path());
        let summary = service
            .run(requirements_file_request("brief.md", "Owner <owner@example.com>"))
            .expect("requirements run");

        std::fs::write(
            &brief,
            "# Requirements Brief\n\n## Problem\nChanged input.\n\n## Outcome\nShip it.\n",
        )
        .expect("rewrite brief");

        let error = service.resume(&summary.run_id).expect_err("stale inputs should fail");
        assert!(error.to_string().contains("stale run"));
        assert!(error.to_string().contains("fork or rerun instead"));
    }

    #[test]
    fn should_resume_change_execution_only_for_change_family_conditions() {
        let context = implementation_context();
        let approvals = vec![ApprovalRecord::for_gate(
            GateKind::Execution,
            "Reviewer <reviewer@example.com>".to_string(),
            ApprovalDecision::Approve,
            "ok".to_string(),
            OffsetDateTime::now_utc(),
        )];

        assert!(EngineService::should_resume_change_execution(Mode::Change, true, &context, &[],));
        assert!(EngineService::should_resume_change_execution(
            Mode::Implementation,
            false,
            &context,
            &approvals,
        ));
        assert!(!EngineService::should_resume_change_execution(
            Mode::Review,
            true,
            &context,
            &approvals,
        ));
    }

    #[test]
    fn capture_explicit_continuation_clears_flags_and_candidates() {
        let mut context = implementation_context();
        assert!(!EngineService::capture_explicit_continuation(&mut context));

        context.clarification_refinement = Some(ClarificationRefinementContext {
            workflow_family: RefinementWorkflowFamily::Planning,
            current_mode: Mode::Requirements,
            working_brief_path: ".canon/runs/R/artifacts/requirements/working-brief.md".to_string(),
            template_ref: "defaults/templates/canon-input/requirements.md".to_string(),
            status: ClarificationRefinementStatus::Active,
            explicit_continuation_required: true,
            authoritative_input_refs: vec!["brief.md".to_string()],
            supporting_input_refs: Vec::new(),
            suggested_candidate: Some(ContinuationCandidateSummary {
                run_id: RunIdentity::new_now_v7().run_id,
                mode: Mode::Requirements,
                state: RunState::Draft,
                match_reason: "same authoritative input fingerprint".to_string(),
                advisory: true,
            }),
            records: Vec::new(),
            readiness_delta: Vec::new(),
        });

        assert!(EngineService::capture_explicit_continuation(&mut context));
        let refinement = context.clarification_refinement.expect("refinement");
        assert!(!refinement.explicit_continuation_required);
        assert!(refinement.suggested_candidate.is_none());
    }
}
