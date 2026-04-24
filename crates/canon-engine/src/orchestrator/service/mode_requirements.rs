use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn run_requirements(
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
        let context_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
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
            run_id: &run_id,
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
            run_id: &run_id,
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
        let generation_policy_attempt =
            self.policy_decision_attempt(&generation_request, &generation_decision);
        let denied_edit_policy_attempt =
            self.policy_decision_attempt(&denied_edit_request, &denied_edit_decision);

        if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
            let evidence = EvidenceBundle {
                run_id: run_id.clone(),
                generation_paths: Vec::new(),
                validation_paths: Vec::new(),
                denied_invocations,
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
                        denied_edit_request.request_id
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
                approvals: Vec::new(),
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
                        request: denied_edit_request,
                        decision: denied_edit_decision,
                        attempts: vec![denied_edit_policy_attempt],
                        approvals: Vec::new(),
                    },
                ],
            };
            store.persist_run_bundle(&bundle)?;
            let details = self.collect_run_runtime_details(
                store,
                &bundle.run.run_id,
                bundle.run.mode,
                bundle.state.state,
            )?;
            return Ok(RunSummary {
                run_id,
                uuid: Some(run_uuid.clone()),
                owner: bundle.run.owner.clone(),
                mode: bundle.run.mode.as_str().to_string(),
                risk: bundle.run.risk.as_str().to_string(),
                zone: bundle.run.zone.as_str().to_string(),
                system_context: bundle
                    .run
                    .system_context
                    .map(|context| context.as_str().to_string()),
                state: format!("{:?}", bundle.state.state),
                artifact_count: 0,
                invocations_total: details.invocations_total,
                invocations_denied: details.invocations_denied,
                invocations_pending_approval: details.pending_invocation_approvals,
                blocking_classification: details.blocking_classification,
                blocked_gates: details.blocked_gates,
                approval_targets: details.approval_targets,
                artifact_paths: details.artifact_paths,
                closure_status: details.closure_status,
                decomposition_scope: details.decomposition_scope,
                closure_findings: details.closure_findings,
                closure_notes: details.closure_notes,
                mode_result: details.mode_result,
                recommended_next_action: details.recommended_next_action,
            });
        }

        let requirements_brief = RequirementsBrief::from_context(context_summary, &input_scope);
        let brief_summary = requirements_brief.summary();
        let copilot = CopilotCliAdapter;
        let generation_output = copilot.generate_requirements(RequirementsGenerationInput {
            problem: &requirements_brief.problem,
            outcome: &requirements_brief.outcome,
            constraints: &requirements_brief.constraints,
            tradeoffs: &requirements_brief.tradeoffs,
            out_of_scope: &requirements_brief.out_of_scope,
            open_questions: &requirements_brief.open_questions,
            source_refs: &requirements_brief.source_refs,
        });
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

        let critique_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
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
        let critique_output = copilot.critique_requirements(
            &requirements_brief.problem,
            &requirements_brief.outcome,
            &requirements_brief.constraints,
            &requirements_brief.out_of_scope,
            &requirements_brief.open_questions,
            &generation_output.summary,
        );
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
            derived_artifacts: artifact_contract
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
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_requirements_artifact_from_evidence(
                    &requirement.file_name,
                    &brief_summary,
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
            &artifact_contract,
            &gate_inputs,
            &request.owner,
            &denied_invocations,
            true,
        );
        let state = run_state_from_gates(&gates);
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
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations,
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
                    denied_edit_request.request_id
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
            artifact_contract,
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
            approvals: Vec::new(),
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
                    request: denied_edit_request,
                    decision: denied_edit_decision,
                    attempts: vec![denied_edit_policy_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        let details = self.collect_run_runtime_details(
            store,
            &bundle.run.run_id,
            bundle.run.mode,
            bundle.state.state,
        )?;

        Ok(RunSummary {
            run_id,
            uuid: Some(run_uuid.clone()),
            owner: bundle.run.owner.clone(),
            mode: bundle.run.mode.as_str().to_string(),
            risk: bundle.run.risk.as_str().to_string(),
            zone: bundle.run.zone.as_str().to_string(),
            system_context: bundle.run.system_context.map(|context| context.as_str().to_string()),
            state: format!("{:?}", bundle.state.state),
            artifact_count: bundle.artifacts.len(),
            invocations_total: details.invocations_total,
            invocations_denied: details.invocations_denied,
            invocations_pending_approval: details.pending_invocation_approvals,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            closure_status: details.closure_status,
            decomposition_scope: details.decomposition_scope,
            closure_findings: details.closure_findings,
            closure_notes: details.closure_notes,
            mode_result: details.mode_result,
            recommended_next_action: details.recommended_next_action,
        })
    }
}
