use super::EngineService;
use super::*;

use crate::artifacts::contract::backlog_contract_for_closure;
use crate::artifacts::markdown::render_backlog_artifact;
use crate::domain::artifact::ArtifactProvenance;
use crate::domain::execution::EvidenceDisposition;
use crate::domain::run::{
    BacklogGranularity, BacklogPlanningContext, ClosureAssessment, ClosureDecompositionScope,
    ClosureFinding, ClosureFindingSeverity, ClosureStatus,
};

impl EngineService {
    pub(super) fn run_backlog(
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

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Backlog,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture backlog planning context and bounded upstream sources",
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;
        let planning_context = build_backlog_planning_context(&context_summary, &input_scope);
        artifact_contract =
            backlog_contract_for_closure(&artifact_contract, &planning_context.closure_assessment);
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured backlog planning context from {} authored input(s).",
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
            mode: Mode::Backlog,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded backlog planning packet",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let copilot = CopilotCliAdapter;
        let generation_output =
            copilot.generate(&backlog_generation_prompt(&planning_context, &context_summary));
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
            mode: Mode::Backlog,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique backlog packet for closure drift and missing traceability",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot
            .critique(&backlog_critique_prompt(&planning_context, &generation_output.summary));
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
            mode: Mode::Backlog,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: "validate backlog packet against repository context",
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

        let evidence_backed_summary = backlog_evidence_backed_summary(
            &planning_context,
            &generation_output.summary,
            &critique_output.summary,
            &validation_summary,
            &context_summary,
        );
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
                    provenance: Some(ArtifactProvenance {
                        request_ids: vec![
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                            validation_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_backlog_artifact(
                    &requirement.file_name,
                    &evidence_backed_summary,
                    &planning_context,
                ),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_backlog_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::BacklogGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                approvals: &approvals,
                validation_independence_satisfied: validation_path.independence.sufficient,
                evidence_complete: true,
                closure_assessment: &planning_context.closure_assessment,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "backlog",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids = vec![
                generation_request.request_id.clone(),
                critique_request.request_id.clone(),
                validation_request.request_id.clone(),
            ];
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

        let mut run_context = self.build_run_context(&request, input_fingerprints, now);
        run_context.backlog_planning = Some(planning_context);

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

fn build_backlog_planning_context(
    context_summary: &str,
    source_inputs: &[String],
) -> BacklogPlanningContext {
    let normalized = context_summary.to_lowercase();
    let delivery_intent = extract_marker(context_summary, &normalized, "delivery intent")
        .or_else(|| extract_marker(context_summary, &normalized, "problem"))
        .or_else(|| extract_marker(context_summary, &normalized, "goal"))
        .map(|value| condense_context_block(&value, 220))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Delivery Intent` section in the backlog input."
                .to_string()
        });
    let desired_granularity = extract_marker(context_summary, &normalized, "desired granularity")
        .and_then(|value| BacklogGranularity::from_label(&value))
        .unwrap_or(BacklogGranularity::EpicPlusSlice);
    let planning_horizon = extract_marker(context_summary, &normalized, "planning horizon")
        .map(|value| condense_context_block(&value, 120));
    let source_refs = {
        let refs = extract_first_marker_entries(
            context_summary,
            &[
                "source refs",
                "source references",
                "upstream sources",
                "context links",
                "source inputs",
            ],
        );
        if refs.is_empty() { source_inputs.to_vec() } else { refs }
    };
    let priority_inputs = extract_first_marker_entries(
        context_summary,
        &["priorities", "priority inputs", "delivery priorities"],
    );
    let constraints = extract_first_marker_entries(
        context_summary,
        &["constraints", "planning constraints", "non-negotiables"],
    );
    let out_of_scope = extract_first_marker_entries(
        context_summary,
        &["out of scope", "out-of-scope", "excluded scope", "deferred work"],
    );
    let closure_assessment =
        assess_backlog_closure(&delivery_intent, desired_granularity, &source_refs, &out_of_scope);

    BacklogPlanningContext {
        mode: "backlog".to_string(),
        delivery_intent,
        desired_granularity,
        planning_horizon,
        source_refs,
        priority_inputs,
        constraints,
        out_of_scope,
        closure_assessment,
    }
}

fn assess_backlog_closure(
    delivery_intent: &str,
    _desired_granularity: BacklogGranularity,
    source_refs: &[String],
    out_of_scope: &[String],
) -> ClosureAssessment {
    let mut findings = Vec::new();

    if delivery_intent.contains("NOT CAPTURED") {
        findings.push(ClosureFinding {
            category: "missing-capability-boundary".to_string(),
            severity: ClosureFindingSeverity::Blocking,
            affected_scope: "whole-run".to_string(),
            recommended_followup:
                "Add an explicit delivery intent or return to architecture/change before decomposing work."
                    .to_string(),
        });
    }

    if source_refs.is_empty() {
        findings.push(ClosureFinding {
            category: "contradictory-source".to_string(),
            severity: ClosureFindingSeverity::Blocking,
            affected_scope: "whole-run".to_string(),
            recommended_followup:
                "Attach bounded upstream source references before turning the brief into a backlog packet."
                    .to_string(),
        });
    }

    if out_of_scope.is_empty() {
        findings.push(ClosureFinding {
            category: "missing-exclusion".to_string(),
            severity: ClosureFindingSeverity::Warning,
            affected_scope: "whole-run".to_string(),
            recommended_followup:
                "Record explicit exclusions so downstream implementation planning does not drift."
                    .to_string(),
        });
    }

    let blocking_findings = findings
        .iter()
        .filter(|finding| matches!(finding.severity, ClosureFindingSeverity::Blocking))
        .count();
    let (status, decomposition_scope, notes) = if blocking_findings > 0 {
        (
            ClosureStatus::Blocked,
            ClosureDecompositionScope::RiskOnlyPacket,
            Some(
                "Backlog decomposition remained closure-limited because required source boundaries were missing."
                    .to_string(),
            ),
        )
    } else if !findings.is_empty() {
        (
            ClosureStatus::Downgraded,
            ClosureDecompositionScope::RiskOnlyPacket,
            Some(
                "Backlog packet is usable, but planning weaknesses remain explicit and should be addressed before execution planning."
                    .to_string(),
            ),
        )
    } else {
        (ClosureStatus::Sufficient, ClosureDecompositionScope::FullPacket, None)
    };

    ClosureAssessment { status, findings, decomposition_scope, notes }
}

fn backlog_generation_prompt(
    planning_context: &BacklogPlanningContext,
    context_summary: &str,
) -> String {
    format!(
        "# Backlog Brief\n\n## Delivery Intent\n{}\n\n## Desired Granularity\n{}\n\n## Planning Horizon\n{}\n\n## Source References\n{}\n\n## Priorities\n{}\n\n## Constraints\n{}\n\n## Out of Scope\n{}\n\n## Closure Status\n{}\n\n## Existing Context\n{}",
        planning_context.delivery_intent,
        planning_context.desired_granularity.as_str(),
        planning_context
            .planning_horizon
            .clone()
            .unwrap_or_else(|| "No explicit planning horizon was authored.".to_string()),
        markdown_list(
            &planning_context.source_refs,
            "- No explicit source references were recorded.",
        ),
        markdown_list(
            &planning_context.priority_inputs,
            "- No explicit planning priorities were recorded.",
        ),
        markdown_list(
            &planning_context.constraints,
            "- No explicit planning constraints were recorded.",
        ),
        markdown_list(&planning_context.out_of_scope, "- No explicit exclusions were recorded.",),
        planning_context.closure_assessment.status.as_str(),
        context_summary,
    )
}

fn backlog_critique_prompt(
    planning_context: &BacklogPlanningContext,
    generation_summary: &str,
) -> String {
    format!(
        "# Backlog Critique Target\n\n## Delivery Intent\n{}\n\n## Closure Status\n{}\n\n## Source References\n{}\n\n## Generated Backlog Framing\n{}\n\n## Challenge\nCheck whether the backlog packet remains above task level, preserves source traceability, and stays explicit about closure weaknesses.",
        planning_context.delivery_intent,
        planning_context.closure_assessment.status.as_str(),
        markdown_list(
            &planning_context.source_refs,
            "- No explicit source references were recorded.",
        ),
        generation_summary,
    )
}

fn backlog_evidence_backed_summary(
    planning_context: &BacklogPlanningContext,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
    authored_context: &str,
) -> String {
    format!(
        "# Backlog Brief\n\n## Delivery Intent\n{}\n\n## Desired Granularity\n{}\n\n## Planning Horizon\n{}\n\n## Source References\n{}\n\n## Priorities\n{}\n\n## Constraints\n{}\n\n## Out of Scope\n{}\n\n## Closure Status\n{}\n\n## Closure Findings\n{}\n\nGenerated framing: {}\n\nCritique evidence: {}\n\nValidation evidence: {}\n\n## Authored Backlog Body\n\n{}",
        planning_context.delivery_intent,
        planning_context.desired_granularity.as_str(),
        planning_context
            .planning_horizon
            .clone()
            .unwrap_or_else(|| "No explicit planning horizon was authored.".to_string()),
        markdown_list(
            &planning_context.source_refs,
            "- No explicit source references were recorded.",
        ),
        markdown_list(
            &planning_context.priority_inputs,
            "- No explicit planning priorities were recorded.",
        ),
        markdown_list(
            &planning_context.constraints,
            "- No explicit planning constraints were recorded.",
        ),
        markdown_list(&planning_context.out_of_scope, "- No explicit exclusions were recorded.",),
        planning_context.closure_assessment.status.as_str(),
        render_closure_findings(&planning_context.closure_assessment),
        generation_summary,
        critique_summary,
        validation_summary,
        authored_context,
    )
}

fn render_closure_findings(assessment: &ClosureAssessment) -> String {
    if assessment.findings.is_empty() {
        "- No closure findings remain open.".to_string()
    } else {
        assessment
            .findings
            .iter()
            .map(|finding| {
                format!(
                    "- [{}] {} on {}. Follow-up: {}",
                    finding.severity.as_str(),
                    finding.category,
                    finding.affected_scope,
                    finding.recommended_followup
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn markdown_list(values: &[String], empty_message: &str) -> String {
    if values.is_empty() {
        empty_message.to_string()
    } else {
        values.iter().map(|value| format!("- {value}")).collect::<Vec<_>>().join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BacklogGranularity, ClosureDecompositionScope, ClosureStatus, backlog_critique_prompt,
        backlog_evidence_backed_summary, backlog_generation_prompt, build_backlog_planning_context,
    };

    #[test]
    fn planning_context_extracts_sections_and_downgrades_when_exclusions_are_missing() {
        let summary = concat!(
            "# Backlog Brief\n\n",
            "## Delivery Intent\n",
            "Prepare a bounded delivery backlog for auth session hardening.\n\n",
            "## Desired Granularity\n",
            "epic-plus-slice-plus-story-candidate\n\n",
            "## Planning Horizon\n",
            "next two delivery increments\n\n",
            "## Source References\n",
            "- docs/architecture/decisions/R-20260422-AUTHREVOC/decision-summary.md\n\n",
            "## Priorities\n",
            "- Ship the rollback-safe slice first.\n\n",
            "## Constraints\n",
            "- Keep the packet above task level.\n"
        );

        let context =
            build_backlog_planning_context(summary, &["canon-input/backlog.md".to_string()]);

        assert_eq!(
            context.delivery_intent,
            "Prepare a bounded delivery backlog for auth session hardening."
        );
        assert_eq!(
            context.desired_granularity,
            BacklogGranularity::EpicPlusSlicePlusStoryCandidate
        );
        assert_eq!(context.planning_horizon.as_deref(), Some("next two delivery increments"));
        assert_eq!(
            context.source_refs,
            vec!["docs/architecture/decisions/R-20260422-AUTHREVOC/decision-summary.md"]
        );
        assert_eq!(context.priority_inputs, vec!["Ship the rollback-safe slice first."]);
        assert_eq!(context.constraints, vec!["Keep the packet above task level."]);
        assert!(context.out_of_scope.is_empty());
        assert_eq!(context.closure_assessment.status, ClosureStatus::Downgraded);
        assert_eq!(
            context.closure_assessment.decomposition_scope,
            ClosureDecompositionScope::RiskOnlyPacket
        );
        assert_eq!(context.closure_assessment.findings.len(), 1);
        assert_eq!(context.closure_assessment.findings[0].category, "missing-exclusion");
    }

    #[test]
    fn planning_context_falls_back_to_bound_inputs_and_blocks_without_delivery_intent() {
        let summary = concat!(
            "# Backlog Brief\n\n",
            "## Desired Granularity\n",
            "epic-plus-slice\n\n",
            "## Constraints\n",
            "- Keep the packet above task level.\n"
        );

        let context = build_backlog_planning_context(
            summary,
            &[
                "canon-input/backlog/brief.md".to_string(),
                "canon-input/backlog/priorities.md".to_string(),
            ],
        );

        assert!(context.delivery_intent.contains("NOT CAPTURED"));
        assert_eq!(
            context.source_refs,
            vec!["canon-input/backlog/brief.md", "canon-input/backlog/priorities.md",]
        );
        assert_eq!(context.closure_assessment.status, ClosureStatus::Blocked);
        assert_eq!(
            context.closure_assessment.decomposition_scope,
            ClosureDecompositionScope::RiskOnlyPacket
        );
        assert!(
            context
                .closure_assessment
                .findings
                .iter()
                .any(|finding| finding.category == "missing-capability-boundary")
        );
    }

    #[test]
    fn backlog_prompts_and_summary_keep_closure_context_visible() {
        let summary = concat!(
            "# Backlog Brief\n\n",
            "## Delivery Intent\n",
            "Prepare a bounded delivery backlog for auth session hardening.\n\n",
            "## Desired Granularity\n",
            "epic-plus-slice\n\n",
            "## Planning Horizon\n",
            "next two releases\n\n",
            "## Source References\n",
            "- docs/changes/auth-session.md\n\n",
            "## Priorities\n",
            "- Ship the rollback-safe slice first.\n\n",
            "## Constraints\n",
            "- Keep the packet above task level.\n"
        );
        let context =
            build_backlog_planning_context(summary, &["canon-input/backlog.md".to_string()]);

        let generation_prompt = backlog_generation_prompt(&context, summary);
        let critique_prompt = backlog_critique_prompt(&context, "Generated framing placeholder");
        let evidence_summary = backlog_evidence_backed_summary(
            &context,
            "Generated framing placeholder",
            "Critique placeholder",
            "Validation placeholder",
            summary,
        );

        assert!(generation_prompt.contains("## Closure Status\ndowngraded"));
        assert!(generation_prompt.contains("- docs/changes/auth-session.md"));
        assert!(critique_prompt.contains("## Closure Status\ndowngraded"));
        assert!(critique_prompt.contains("Generated framing placeholder"));
        assert!(evidence_summary.contains("## Closure Findings"));
        assert!(evidence_summary.contains("missing-exclusion"));
        assert!(evidence_summary.contains("Validation placeholder"));
    }
}
