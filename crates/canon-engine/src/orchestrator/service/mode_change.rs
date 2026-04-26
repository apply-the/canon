use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn run_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        self.execute_change(store, request, policy_set, identity, Vec::new())
    }

    pub(super) fn run_implementation(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        self.execute_change(store, request, policy_set, identity, Vec::new())
    }

    pub(super) fn run_refactor(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        self.execute_change(store, request, policy_set, identity, Vec::new())
    }

    pub(super) fn execute_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
        identity: RunIdentity,
        approvals: Vec<ApprovalRecord>,
    ) -> Result<RunSummary, EngineError> {
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let brief_summary = self.load_input_summary(&request.inputs, &request.inline_inputs)?;
        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let (
            context_request_summary,
            context_attempt_summary,
            generation_request_summary,
            validation_request_summary,
            declared_execution_scope,
            mutation_summary,
        ) = match request.mode {
            Mode::Change => {
                let declared_change_surface = extract_change_surface_entries(&brief_summary);
                let mutation_summary = if declared_change_surface.is_empty() {
                    "propose bounded legacy transformation without mutating the workspace"
                        .to_string()
                } else {
                    format!(
                        "propose bounded legacy transformation within declared change surface: {}",
                        declared_change_surface.join(", ")
                    )
                };
                (
                    "capture change brief and repository context",
                    "Captured change brief and bounded repository context.",
                    "generate bounded change framing",
                    "validate change framing against repository context",
                    declared_change_surface,
                    mutation_summary,
                )
            }
            Mode::Implementation => {
                let declared_mutation_bounds = extract_execution_scope_entries(
                    &brief_summary,
                    &["allowed paths", "mutation bounds"],
                );
                let mutation_summary = if declared_mutation_bounds.is_empty() {
                    "propose bounded implementation guidance without mutating the workspace"
                        .to_string()
                } else {
                    format!(
                        "propose bounded implementation guidance within declared mutation bounds: {}",
                        declared_mutation_bounds.join(", ")
                    )
                };
                (
                    "capture implementation brief and bounded repository context",
                    "Captured implementation brief and bounded repository context.",
                    "generate bounded implementation packet",
                    "validate implementation safety-net evidence against repository context",
                    declared_mutation_bounds,
                    mutation_summary,
                )
            }
            Mode::Refactor => {
                let declared_refactor_scope = extract_execution_scope_entries(
                    &brief_summary,
                    &["allowed paths", "refactor scope"],
                );
                let mutation_summary = if declared_refactor_scope.is_empty() {
                    "propose bounded structural refactor guidance without mutating the workspace"
                        .to_string()
                } else {
                    format!(
                        "propose bounded structural refactor guidance within declared refactor scope: {}",
                        declared_refactor_scope.join(", ")
                    )
                };
                (
                    "capture refactor brief and bounded repository context",
                    "Captured refactor brief and bounded repository context.",
                    "generate bounded refactor packet",
                    "validate refactor preservation evidence against repository context",
                    declared_refactor_scope,
                    mutation_summary,
                )
            }
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: context_request_summary,
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
                summary: context_attempt_summary.to_string(),
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
            summary: generation_request_summary,
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let generation_policy_attempt =
            self.policy_decision_attempt(&generation_request, &generation_decision);

        let mutation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ExecuteBoundedTransformation,
            summary: &mutation_summary,
            scope: declared_execution_scope.clone(),
        });
        let mut mutation_decision =
            invocation_runtime::evaluate_request_policy(&mutation_request, &policy_set);

        let approved_generation = approvals.iter().any(|approval| {
            approval.matches_invocation(&generation_request.request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });
        let approved_mutation = approvals.iter().any(|approval| {
            approval.matches_invocation(&mutation_request.request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });
        let execution_gate_approved = execution_gate_is_approved(&approvals);
        let mutation_patch = if matches!(request.mode, Mode::Implementation | Mode::Refactor) {
            self.locate_authored_mutation_patch(&request.inputs, &declared_execution_scope)?
        } else {
            None
        };

        if execution_gate_approved
            && mutation_patch.is_some()
            && matches!(
                mutation_decision.kind,
                PolicyDecisionKind::Allow | PolicyDecisionKind::AllowConstrained
            )
            && !mutation_decision.constraints.patch_disabled
        {
            mutation_decision.constraints.recommendation_only = false;
            mutation_decision.requires_approval = false;
            mutation_decision.rationale = approved_execution_mutation_rationale(
                request.mode,
                &declared_execution_scope,
                mutation_patch
                    .as_ref()
                    .map(|patch| patch.relative_path.as_str())
                    .unwrap_or_default(),
            );
        }

        let mutation_attempt = if matches!(
            mutation_decision.kind,
            PolicyDecisionKind::Allow | PolicyDecisionKind::AllowConstrained
        ) && !mutation_decision.constraints.recommendation_only
            && !mutation_decision.constraints.patch_disabled
        {
            match mutation_patch.as_ref() {
                Some(patch) => self.apply_authored_mutation_patch(&mutation_request, patch)?,
                None => self.policy_decision_attempt(&mutation_request, &mutation_decision),
            }
        } else {
            self.policy_decision_attempt(&mutation_request, &mutation_decision)
        };

        if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval)
            && !approved_generation
        {
            let evidence = EvidenceBundle {
                run_id: run_id.clone(),
                generation_paths: Vec::new(),
                validation_paths: Vec::new(),
                denied_invocations: Vec::new(),
                trace_refs: vec![format!("traces/{run_id}.jsonl")],
                artifact_refs: Vec::new(),
                decision_refs: vec![
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        context_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        generation_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        mutation_request.request_id
                    ),
                ],
                approval_refs: approval_record_refs(&run_id, &approvals),
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
                state: RunStateManifest { state: RunState::AwaitingApproval, updated_at: now },
                artifact_contract,
                links: LinkManifest {
                    artifacts: Vec::new(),
                    decisions: Vec::new(),
                    traces: Vec::new(),
                    invocations: Vec::new(),
                    evidence: Some(evidence_path),
                },
                verification_records: Vec::new(),
                artifacts: Vec::new(),
                gates: Vec::new(),
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
                        attempts: vec![generation_policy_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: mutation_request,
                        decision: mutation_decision,
                        attempts: vec![mutation_attempt],
                        approvals: Vec::new(),
                    },
                ],
            };
            store.persist_run_bundle(&bundle)?;
            return self.summarize_run(
                store,
                RunSummarySpec {
                    run_id: &run_id,
                    mode: request.mode,
                    risk: request.risk,
                    zone: request.zone,
                    state: RunState::AwaitingApproval,
                    artifact_count: 0,
                },
            );
        }

        let copilot = CopilotCliAdapter;
        let generation_context = if context_summary == brief_summary {
            brief_summary.clone()
        } else {
            format!("{brief_summary}\n\n{context_summary}")
        };
        let generation_output = copilot.generate(&generation_context);
        let generation_attempt = self.completed_attempt(
            &generation_request,
            if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
                2
            } else {
                1
            },
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

        let validation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: validation_request_summary,
            scope: input_scope.clone(),
        });
        let validation_decision =
            invocation_runtime::evaluate_request_policy(&validation_request, &policy_set);
        let (validation_summary, validation_attempt) =
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

        let evidence_backed_summary = format!(
            "{brief_summary}\n\nGenerated framing: {}\n\nValidation evidence: {}\n\nMutation posture: {}",
            generation_output.summary, validation_summary, mutation_attempt.outcome.summary
        );
        let default_owner = self.resolve_owner("");
        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                let contents = match request.mode {
                    Mode::Change => render_change_artifact(
                        &requirement.file_name,
                        &evidence_backed_summary,
                        &default_owner,
                    ),
                    Mode::Implementation => render_implementation_artifact(
                        &requirement.file_name,
                        &brief_summary,
                        &default_owner,
                    ),
                    Mode::Refactor => render_refactor_artifact(
                        &requirement.file_name,
                        &brief_summary,
                        &default_owner,
                    ),
                    other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
                };

                Ok(PersistedArtifact {
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
                                validation_request.request_id.clone(),
                                mutation_request.request_id.clone(),
                            ],
                            evidence_bundle: Some(evidence_path.clone()),
                            disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                        }),
                    },
                    contents,
                })
            })
            .collect::<Result<Vec<_>, EngineError>>()?;

        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = match request.mode {
            Mode::Change => gatekeeper::evaluate_change_gates(
                &artifact_contract,
                &gate_inputs,
                gatekeeper::ChangeGateContext {
                    owner: &request.owner,
                    risk: request.risk,
                    zone: request.zone,
                    approvals: &approvals,
                    system_context: request.system_context,
                    validation_independence_satisfied: validation_path.independence.sufficient,
                    evidence_complete: true,
                },
            ),
            Mode::Implementation => gatekeeper::evaluate_implementation_gates(
                &artifact_contract,
                &gate_inputs,
                gatekeeper::ImplementationGateContext {
                    owner: &request.owner,
                    risk: request.risk,
                    zone: request.zone,
                    approvals: &approvals,
                    system_context: request.system_context,
                    validation_independence_satisfied: validation_path.independence.sufficient,
                    evidence_complete: true,
                },
            ),
            Mode::Refactor => gatekeeper::evaluate_refactor_gates(
                &artifact_contract,
                &gate_inputs,
                gatekeeper::RefactorGateContext {
                    owner: &request.owner,
                    risk: request.risk,
                    zone: request.zone,
                    approvals: &approvals,
                    system_context: request.system_context,
                    validation_independence_satisfied: validation_path.independence.sufficient,
                    evidence_complete: true,
                },
            ),
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };
        let mut state = run_state_from_gates(&gates);
        if mutation_decision.requires_approval
            && !approved_mutation
            && !matches!(state, RunState::Blocked)
        {
            state = RunState::AwaitingApproval;
        }

        let mut verification_records = verification_runner::change_verification_records(
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids =
                vec![generation_request.request_id.clone(), validation_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }

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
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    validation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", mutation_request.request_id),
            ],
            approval_refs: approval_record_refs(&run_id, &approvals),
        };

        let generation_attempts =
            if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
                vec![generation_policy_attempt, generation_attempt]
            } else {
                vec![generation_attempt]
            };

        let mut run_context = self.build_run_context(&request, input_fingerprints, now);
        if matches!(mutation_attempt.outcome.kind, ToolOutcomeKind::Succeeded) {
            set_execution_posture(&mut run_context, ExecutionPosture::Mutating);
        }
        if execution_gate_approved {
            set_post_approval_execution_consumed(&mut run_context, true);
        }

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
            context: run_context,
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
                    attempts: generation_attempts,
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: validation_request,
                    decision: validation_decision,
                    attempts: vec![validation_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: mutation_request,
                    decision: mutation_decision,
                    attempts: vec![mutation_attempt],
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
