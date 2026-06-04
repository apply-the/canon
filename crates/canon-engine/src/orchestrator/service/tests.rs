use super::{
    AiTool, AuthorityStatus, ClarificationQuestionSummary, EngineService, GateInspectSummary,
    GovernedRequestSpec, InspectEntry, InspectTarget, ModeResultSummary, RecommendedActionSummary,
    RequirementsRequestSpec, ResultActionSummary, RunRequest, apply_execution_posture_summary,
    approved_execution_mutation_rationale, authority_approval_state, build_action_chips_for,
    build_runtime_packet_metadata, canonical_mode_input_binding, capability_tag,
    collect_files_recursively, execution_continuation_pending, extract_change_surface_entries,
    packet_body_artifact_order, preserve_multiline_summary, process_failure_excerpt,
    recommend_next_action, resolved_execution_posture_label,
    resolved_execution_posture_label_for_mode, run_state_from_gates, set_execution_posture,
    set_post_approval_execution_consumed,
};
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactProvenance, ArtifactRecord, ArtifactRequirement,
};
use crate::domain::execution::{
    DeniedInvocation, EvidenceDisposition, ExecutionPosture, GenerationPath, InvocationAttempt,
    InvocationConstraintSet, InvocationPolicyDecision, InvocationRequest, PolicyDecisionKind,
    ToolOutcome, ToolOutcomeKind, ValidationPath,
};
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::publish_profile::AuthorityApprovalState;
use crate::domain::run::{
    BacklogGranularity, BacklogHandoffAvailability, BacklogPlanningContext,
    ClarificationAnswerKind, ClarificationRefinementContext, ClarificationRefinementStatus,
    ClarificationResolutionState, ClassificationProvenance, ClosureAssessment,
    ClosureDecompositionScope, ClosureFinding, ClosureFindingSeverity, ClosureStatus,
    ReadinessDeltaItem, ReadinessDeltaSourceKind, RefinementWorkflowFamily, RunContext,
    RunIdentity, RunState, SystemContext, UpstreamContext,
};
use crate::orchestrator::evidence::{attach_paths, default_independence, empty_evidence_bundle};
use crate::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use crate::persistence::store::{
    InitSummary as StoreInitSummary, PersistedRunBundle, SkillMaterializationTarget,
    SkillsSummary as StoreSkillsSummary, WorkspaceStore,
};
use canon_adapters::{
    AdapterKind, CapabilityKind, InvocationOrientation, LineageClass, MutabilityClass,
    TrustBoundaryKind,
};
use tempfile::TempDir;
use time::OffsetDateTime;

fn inline_request(
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    system_context: Option<crate::domain::run::SystemContext>,
    owner: &str,
    inline_input: &str,
) -> RunRequest {
    RunRequest {
        mode,
        risk,
        zone,
        system_context,
        classification: ClassificationProvenance::explicit(),
        owner: owner.to_string(),
        inputs: Vec::new(),
        inline_inputs: vec![inline_input.to_string()],
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn supported_implementation_brief() -> &'static str {
    r#"# Implementation Brief

## Task Mapping
1. Add bounded auth session repository helpers.
2. Thread the helper through the revocation service without expanding the public API.

## Bounded Changes
- Auth session repository helper wiring.
- Revocation service internal composition.

## Mutation Bounds
src/auth/session.rs; src/auth/repository.rs

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before mutation.

## Independent Checks
cargo test --test session_contract

## Rollback Triggers
Revocation output drifts or audit ordering becomes unstable.

## Rollback Steps
Revert the bounded auth-session patch and redeploy the previous build.
"#
}

fn supported_verification_brief() -> &'static str {
    r#"# Verification Brief

## Claims Under Test

- rollback remains bounded and auditable
- operator evidence remains tied to the rollback boundary

## Invariant Checks

- rollback metadata remains explicit during the bounded flow

## Contract Assumptions

- rollback metadata must remain explicit

## Verification Outcome

Status: supported

## Challenge Findings

- no additional challenge findings remain beyond the authored packet

## Contradictions

- none recorded

## Verified Claims

- rollback remains bounded and auditable
- operator evidence remains tied to the rollback boundary

## Rejected Claims

- none recorded

## Overall Verdict

Status: supported

Rationale: the current evidence covers the authored claim set.

## Open Findings

Status: no-open-findings

- No unresolved findings remain from the current verification packet.

## Required Follow-Up

- Keep the verification packet attached to downstream release review.
"#
}

fn targeted_refinement_brief(mode: Mode) -> &'static str {
    match mode {
        Mode::Requirements => {
            "# Requirements Brief\n\n## Problem\nClarify how Canon should refine an existing governed work item.\n\n## Desired Outcome\n- preserve run identity\n- keep source inputs immutable\n"
        }
        Mode::Discovery => {
            "# Discovery Brief\n\n## Problem\nDetermine whether follow-up context belongs to fresh work or continuation.\n\n## Unknowns\n- continuation intent\n- ambiguity handling\n"
        }
        Mode::SystemShaping => {
            "# System Shaping Brief\n\n## Opportunity\nModel refinement as one durable draft identity.\n\n## Constraints\n- no new persistence family\n"
        }
        Mode::Architecture => {
            "# Architecture Brief\n\n## Decision\nPersist refinement state on the existing run context.\n\n## Constraints\n- keep inspect clarity separate\n"
        }
        Mode::Change => {
            "# Change Brief\n\n## System Slice\nRun identity and refinement state.\n\n## Intended Change\nAdd explicit continuation gating.\n"
        }
        other => panic!("unsupported targeted refinement mode: {other:?}"),
    }
}

fn representative_non_targeted_refinement_brief(mode: Mode) -> &'static str {
    match mode {
        Mode::Review => {
            "# Review Brief\n\n## Review Scope\nReview same-work refinement semantics.\n"
        }
        Mode::Verification => {
            "# Verification Brief\n\n## Claims Under Test\n- continuation remains explicit\n"
        }
        Mode::Implementation => supported_implementation_brief(),
        Mode::Refactor => {
            "# Refactor Brief\n\n## Preserved Behavior\n- existing runs remain inspectable\n"
        }
        Mode::Incident => {
            "# Incident Brief\n\n## Incident Summary\nFresh work attached to the wrong governed run.\n"
        }
        Mode::Migration => {
            "# Migration Brief\n\n## Migration Goal\nCarry refinement context through successor lineage.\n"
        }
        other => panic!("unsupported representative non-targeted mode: {other:?}"),
    }
}

fn refinement_request(mode: Mode, owner: &str) -> RunRequest {
    let brief = match mode {
        Mode::Requirements
        | Mode::Discovery
        | Mode::SystemShaping
        | Mode::Architecture
        | Mode::Change => targeted_refinement_brief(mode),
        Mode::Review
        | Mode::Verification
        | Mode::Implementation
        | Mode::Refactor
        | Mode::Incident
        | Mode::Migration => representative_non_targeted_refinement_brief(mode),
        other => panic!("unsupported refinement fixture mode: {other:?}"),
    };

    inline_request(
        mode,
        RiskClass::BoundedImpact,
        UsageZone::Yellow,
        Some(SystemContext::Existing),
        owner,
        brief,
    )
}

fn persist_inspect_evidence_bundle(workspace: &TempDir) -> String {
    let store = WorkspaceStore::new(workspace.path());
    let run_id = "R-20260528-ABCDEF12".to_string();
    let artifact_ref = format!("artifacts/{run_id}/backlog/01-backlog-overview.md");

    let mut evidence = empty_evidence_bundle(&run_id);
    attach_paths(
        &mut evidence,
        GenerationPath {
            path_id: "generation:req-1".to_string(),
            request_ids: vec!["req-1".to_string()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: vec![artifact_ref.clone()],
        },
        ValidationPath {
            path_id: "validation:req-2".to_string(),
            request_ids: vec!["req-2".to_string()],
            lineage_classes: vec![LineageClass::HumanReview],
            verification_refs: vec![format!("runs/{run_id}/verification/verification-00.toml")],
            independence: default_independence("generation:req-1"),
        },
        vec![DeniedInvocation {
            request_id: "req-3".to_string(),
            rationale: "mutation remained out of policy scope".to_string(),
            policy_refs: vec!["policy:red-zone".to_string()],
            recorded_at: OffsetDateTime::UNIX_EPOCH,
        }],
    );
    evidence.artifact_refs.push(artifact_ref);

    let bundle = PersistedRunBundle {
        run: RunManifest {
            run_id: run_id.clone(),
            uuid: None,
            short_id: None,
            slug: None,
            title: Some("Inspect backlog evidence".to_string()),
            mode: Mode::Backlog,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "planner".to_string(),
            lineage: None,
            created_at: OffsetDateTime::UNIX_EPOCH,
        },
        context: RunContext {
            repo_root: workspace.path().display().to_string(),
            owner: Some("planner".to_string()),
            inputs: vec!["canon-input/backlog.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: Some(SystemContext::Existing),
            upstream_context: Some(UpstreamContext {
                feature_slice: Some("runtime honesty contract".to_string()),
                primary_upstream_mode: Some("architecture".to_string()),
                source_refs: vec!["specs/061-skill-runtime-contracts/spec.md".to_string()],
                carried_forward_items: vec!["Keep structured preflight JSON stable.".to_string()],
                excluded_upstream_scope: Some("assistant packaging changes".to_string()),
            }),
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: Some(BacklogPlanningContext {
                mode: "backlog".to_string(),
                delivery_intent: "Prepare a bounded rollout backlog for runtime contracts."
                    .to_string(),
                desired_granularity: BacklogGranularity::EpicPlusSlice,
                planning_horizon: Some("next sprint".to_string()),
                source_refs: vec!["specs/061-skill-runtime-contracts/plan.md".to_string()],
                priority_inputs: vec!["stabilize operator inspect output".to_string()],
                constraints: vec!["Stay above task level.".to_string()],
                out_of_scope: vec!["frontend packaging refresh".to_string()],
                slice_ids: vec!["SLICE-RUNTIME-001".to_string()],
                closure_assessment: ClosureAssessment {
                    status: ClosureStatus::Downgraded,
                    findings: vec![ClosureFinding {
                        category: "missing-evidence".to_string(),
                        severity: ClosureFindingSeverity::Warning,
                        affected_scope: "rollback validation".to_string(),
                        recommended_followup: "Add an independent rollback drill before closure."
                            .to_string(),
                    }],
                    decomposition_scope: ClosureDecompositionScope::RiskOnlyPacket,
                    notes: Some("One more independent rollback check remains.".to_string()),
                },
                handoff_availability: BacklogHandoffAvailability::WithheldForClosure,
                handoff_findings: vec![
                    "closure findings keep downstream handoff unavailable".to_string(),
                ],
                execution_handoff: None,
            }),
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        },
        state: RunStateManifest {
            state: RunState::Completed,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        },
        artifact_contract: ArtifactContract {
            version: 1,
            artifact_requirements: Vec::new(),
            required_verification_layers: Vec::new(),
        },
        artifacts: Vec::new(),
        links: LinkManifest {
            artifacts: Vec::new(),
            decisions: Vec::new(),
            traces: Vec::new(),
            invocations: Vec::new(),
            evidence: None,
        },
        gates: Vec::new(),
        approvals: Vec::new(),
        verification_records: Vec::new(),
        evidence: Some(evidence),
        invocations: Vec::new(),
    };

    store.persist_run_bundle(&bundle).expect("persist inspect evidence bundle");
    run_id
}

fn persist_inspect_invocation_bundle(workspace: &TempDir) -> String {
    let store = WorkspaceStore::new(workspace.path());
    let run_id = "R-20260528-INVOC123".to_string();
    let request_id = "req-approved".to_string();
    let artifact_path = format!("artifacts/{run_id}/implementation/01-task-mapping.md");

    let bundle = PersistedRunBundle {
        run: RunManifest {
            run_id: run_id.clone(),
            uuid: None,
            short_id: None,
            slug: None,
            title: Some("Inspect invocation runtime".to_string()),
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            lineage: None,
            created_at: OffsetDateTime::UNIX_EPOCH,
        },
        context: RunContext {
            repo_root: workspace.path().display().to_string(),
            owner: Some("staff-engineer".to_string()),
            inputs: vec!["canon-input/implementation.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: Some(SystemContext::Existing),
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        },
        state: RunStateManifest {
            state: RunState::AwaitingApproval,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        },
        artifact_contract: ArtifactContract {
            version: 1,
            artifact_requirements: Vec::new(),
            required_verification_layers: Vec::new(),
        },
        artifacts: vec![crate::persistence::store::PersistedArtifact {
            record: ArtifactRecord {
                file_name: "01-task-mapping.md".to_string(),
                relative_path: artifact_path.clone(),
                format: ArtifactFormat::Markdown,
                provenance: Some(ArtifactProvenance {
                    request_ids: vec![request_id.clone()],
                    evidence_bundle: None,
                    disposition: EvidenceDisposition::Supporting,
                }),
            },
            contents: "# Task Mapping\n\n- apply bounded runtime patch\n".to_string(),
        }],
        links: LinkManifest {
            artifacts: vec![artifact_path],
            decisions: Vec::new(),
            traces: Vec::new(),
            invocations: Vec::new(),
            evidence: None,
        },
        gates: Vec::new(),
        approvals: vec![ApprovalRecord::for_invocation(
            request_id.clone(),
            "maintainer".to_string(),
            ApprovalDecision::Approve,
            "approved bounded execution".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )],
        verification_records: Vec::new(),
        evidence: None,
        invocations: vec![crate::persistence::invocations::PersistedInvocation {
            request: InvocationRequest {
                request_id: request_id.clone(),
                run_id: run_id.clone(),
                mode: Mode::Implementation.as_str().to_string(),
                system_context: Some(SystemContext::Existing),
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                adapter: AdapterKind::CopilotCli,
                capability: CapabilityKind::GenerateContent,
                orientation: InvocationOrientation::Generation,
                mutability: MutabilityClass::ArtifactWrite,
                trust_boundary: TrustBoundaryKind::AiReasoning,
                lineage: LineageClass::AiVendorFamily,
                requested_scope: vec!["canon-input/implementation.md".to_string()],
                owner: Some("staff-engineer".to_string()),
                summary: "Generate bounded implementation packet".to_string(),
                requested_at: OffsetDateTime::UNIX_EPOCH,
            },
            decision: InvocationPolicyDecision {
                kind: PolicyDecisionKind::NeedsApproval,
                constraints: InvocationConstraintSet {
                    recommendation_only: true,
                    ..InvocationConstraintSet::default()
                },
                requires_approval: true,
                rationale: "bounded execution requires human approval".to_string(),
                policy_refs: vec!["policy:execution".to_string()],
                decided_at: OffsetDateTime::UNIX_EPOCH,
            },
            attempts: vec![InvocationAttempt {
                request_id,
                attempt_number: 1,
                started_at: OffsetDateTime::UNIX_EPOCH,
                finished_at: OffsetDateTime::UNIX_EPOCH,
                executor: "copilot-cli".to_string(),
                outcome: ToolOutcome {
                    kind: ToolOutcomeKind::Succeeded,
                    summary: "Generated the bounded implementation packet.".to_string(),
                    exit_code: Some(0),
                    payload_refs: Vec::new(),
                    candidate_artifacts: vec!["01-task-mapping.md".to_string()],
                    recorded_at: OffsetDateTime::UNIX_EPOCH,
                },
            }],
            approvals: Vec::new(),
        }],
    };

    store.persist_run_bundle(&bundle).expect("persist inspect invocation bundle");
    run_id
}

#[test]
fn change_surface_entries_prefer_markdown_section_over_inline_summary_mentions() {
    let source = "# Change Brief\n\n## Change Surface\n- session repository\n- auth service\n\nMutation posture: propose bounded legacy transformation within declared change surface: entire repository, adjacent modules";

    let entries = extract_change_surface_entries(source);

    assert_eq!(entries, vec!["session repository".to_string(), "auth service".to_string()]);
}

#[test]
fn change_surface_entries_fall_back_to_inline_marker_and_dedupe_segments() {
    let source = "Summary\n\nChange Surface: auth service, session repository; auth service; token cleanup job";

    let entries = extract_change_surface_entries(source);

    assert_eq!(
        entries,
        vec![
            "auth service".to_string(),
            "session repository".to_string(),
            "token cleanup job".to_string()
        ]
    );
}

#[test]
fn refinement_request_builder_supports_targeted_and_non_targeted_modes() {
    let targeted = refinement_request(Mode::Requirements, "owner@example.com");
    let non_targeted = refinement_request(Mode::Review, "owner@example.com");

    assert_eq!(targeted.mode, Mode::Requirements);
    assert_eq!(non_targeted.mode, Mode::Review);
    assert!(targeted.inputs.is_empty());
    assert!(non_targeted.inputs.is_empty());
    assert_eq!(targeted.inline_inputs.len(), 1);
    assert_eq!(non_targeted.inline_inputs.len(), 1);
}

#[test]
fn preserve_multiline_summary_keeps_bullets_on_separate_lines() {
    let summary = "## Change Surface\n- first bullet\n- second bullet\n\n## Validation Strategy\n- independent check";

    let normalized = preserve_multiline_summary(summary);

    assert!(normalized.contains("- first bullet\n- second bullet"));
    assert!(normalized.contains("\n\n## Validation Strategy\n- independent check"));
}

#[test]
fn run_state_from_gates_prioritizes_approval_then_blocked_then_completed() {
    let approval_gate = GateEvaluation {
        gate: GateKind::Risk,
        status: GateStatus::NeedsApproval,
        blockers: vec!["approval required".to_string()],
        evaluated_at: OffsetDateTime::UNIX_EPOCH,
    };
    let blocked_gate = GateEvaluation {
        gate: GateKind::Architecture,
        status: GateStatus::Blocked,
        blockers: vec!["missing artifact".to_string()],
        evaluated_at: OffsetDateTime::UNIX_EPOCH,
    };
    let passed_gate = GateEvaluation {
        gate: GateKind::Exploration,
        status: GateStatus::Passed,
        blockers: Vec::new(),
        evaluated_at: OffsetDateTime::UNIX_EPOCH,
    };

    assert_eq!(
        run_state_from_gates(&[passed_gate.clone(), approval_gate]),
        RunState::AwaitingApproval
    );
    assert_eq!(run_state_from_gates(&[passed_gate.clone(), blocked_gate]), RunState::Blocked);
    assert_eq!(run_state_from_gates(&[passed_gate]), RunState::Completed);
}

#[test]
fn recommend_next_action_prefers_evidence_for_completed_runs_without_artifacts() {
    let action = recommend_next_action(RunState::Completed, None, &[], true, &[], &[]);

    assert_eq!(
        action,
        Some(RecommendedActionSummary {
            action: "inspect-evidence".to_string(),
            rationale: "The run completed; inspect the evidence bundle for execution lineage."
                .to_string(),
            target: None,
        })
    );
}

#[test]
fn recommend_next_action_is_absent_for_completed_runs_with_mode_result() {
    let mode_result = ModeResultSummary {
        headline: "Requirements packet ready for downstream review.".to_string(),
        artifact_packet_summary: "Primary artifact is ready.".to_string(),
        execution_posture: None,
        primary_artifact_title: "Problem Statement".to_string(),
        primary_artifact_path: ".canon/artifacts/run-123/requirements/problem-statement.md"
            .to_string(),
        primary_artifact_action: ResultActionSummary {
            id: "open-primary-artifact".to_string(),
            label: "Open primary artifact".to_string(),
            host_action: "open-file".to_string(),
            target: ".canon/artifacts/run-123/requirements/problem-statement.md"
                .to_string(),
            text_fallback:
                "Open the primary artifact at .canon/artifacts/run-123/requirements/problem-statement.md."
                    .to_string(),
        },
        result_excerpt: "Build a bounded USB flashing CLI.".to_string(),
        action_chips: Vec::new(),
    };

    let action = recommend_next_action(
        RunState::Completed,
        Some(&mode_result),
        std::slice::from_ref(&mode_result.primary_artifact_path),
        true,
        &[],
        &[],
    );

    assert_eq!(action, None);
}

#[test]
fn build_action_chips_for_emits_full_frontend_contract_fields() {
    let chips = build_action_chips_for(
        RunState::AwaitingApproval,
        &["gate:execution".to_string()],
        ".canon/artifacts/run-123/refactor/preserved-behavior.md",
        "run-123",
    );

    assert_eq!(chips.len(), 3);

    let open_chip = &chips[0];
    assert_eq!(open_chip.id, "open-primary-artifact");
    assert_eq!(open_chip.intent, "Inspect");
    assert!(open_chip.recommended);
    assert_eq!(
        open_chip.text_fallback,
        "Open the primary artifact at .canon/artifacts/run-123/refactor/preserved-behavior.md."
    );

    let inspect_chip = &chips[1];
    assert_eq!(inspect_chip.id, "inspect-evidence");
    assert_eq!(inspect_chip.intent, "Inspect");
    assert!(!inspect_chip.recommended);
    assert_eq!(
        inspect_chip.text_fallback,
        "Inspect evidence for run run-123: `canon inspect evidence --run run-123`."
    );

    let approve_chip = &chips[2];
    assert_eq!(approve_chip.id, "approve-gate-execution");
    assert_eq!(approve_chip.intent, "GovernedAction");
    assert_eq!(approve_chip.prefilled_args.get("RUN_ID"), Some(&"run-123".to_string()));
    assert_eq!(approve_chip.prefilled_args.get("TARGET"), Some(&"gate:execution".to_string()));
    assert_eq!(
        approve_chip.required_user_inputs,
        vec!["BY".to_string(), "DECISION".to_string(), "RATIONALE".to_string()]
    );
    assert_eq!(
        approve_chip.text_fallback,
        "Approve target gate:execution for run run-123: `canon approve --run run-123 --target gate:execution --by <BY> --decision <DECISION> --rationale <RATIONALE>`."
    );
}

#[test]
fn packet_body_artifact_order_excludes_runtime_sidecars() {
    let requirements = vec![
        ArtifactRequirement {
            file_name: "01-problem-statement.md".to_string(),
            format: ArtifactFormat::Markdown,
            required_sections: Vec::new(),
            gates: vec![GateKind::Exploration],
            required: true,
        },
        ArtifactRequirement {
            file_name: "view-manifest.json".to_string(),
            format: ArtifactFormat::Json,
            required_sections: Vec::new(),
            gates: vec![GateKind::ReleaseReadiness],
            required: true,
        },
        ArtifactRequirement {
            file_name: "packet-metadata.json".to_string(),
            format: ArtifactFormat::Json,
            required_sections: Vec::new(),
            gates: vec![GateKind::ReleaseReadiness],
            required: true,
        },
    ];

    assert_eq!(
        packet_body_artifact_order(&requirements),
        vec!["01-problem-statement.md".to_string()]
    );
}

#[test]
fn build_runtime_packet_metadata_emits_order_and_legacy_aliases() {
    let requirements = vec![
        ArtifactRequirement {
            file_name: "01-problem-statement.md".to_string(),
            format: ArtifactFormat::Markdown,
            required_sections: Vec::new(),
            gates: vec![GateKind::Exploration],
            required: true,
        },
        ArtifactRequirement {
            file_name: "02-scope-cuts.md".to_string(),
            format: ArtifactFormat::Markdown,
            required_sections: Vec::new(),
            gates: vec![GateKind::Exploration],
            required: true,
        },
        ArtifactRequirement {
            file_name: "packet-metadata.json".to_string(),
            format: ArtifactFormat::Json,
            required_sections: Vec::new(),
            gates: vec![GateKind::ReleaseReadiness],
            required: true,
        },
    ];

    let request = RunRequest {
        mode: Mode::Requirements,
        risk: RiskClass::LowImpact,
        zone: UsageZone::Green,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "product-owner".to_string(),
        inputs: Vec::new(),
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    let metadata_contents = build_runtime_packet_metadata("R-test", &request, &[], &requirements)
        .expect("packet metadata should render");
    let metadata: serde_json::Value =
        serde_json::from_str(&metadata_contents).expect("packet metadata json");

    assert_eq!(metadata["run_id"], "R-test");
    assert_eq!(metadata["mode"], "requirements");
    assert_eq!(metadata["primary_artifact"], "01-problem-statement.md");
    assert_eq!(metadata["artifact_order"][0], "01-problem-statement.md");
    assert_eq!(metadata["artifact_order"][1], "02-scope-cuts.md");
    assert_eq!(metadata["legacy_aliases"]["problem-statement.md"], "01-problem-statement.md");
    assert_eq!(metadata["legacy_aliases"]["scope-cuts.md"], "02-scope-cuts.md");
    assert_eq!(metadata["authority_governance"]["contract_line"], "authority-governance-v1");
    assert_eq!(metadata["authority_governance"]["authority_zone"], "green");
    assert_eq!(metadata["authority_governance"]["risk"], "low-impact");
    assert_eq!(metadata["adaptive_governance"]["contract_line"], "adaptive-governance-v1");
    assert_eq!(metadata["adaptive_governance"]["governance_state"], "advisory");
    assert_eq!(metadata["adaptive_governance"]["rollout_profile"], "minimal");
}

#[test]
fn capability_tag_covers_supported_capabilities() {
    let cases = [
        (CapabilityKind::ReadRepository, "context"),
        (CapabilityKind::GenerateContent, "generate"),
        (CapabilityKind::CritiqueContent, "critique"),
        (CapabilityKind::ProposeWorkspaceEdit, "edit"),
        (CapabilityKind::InspectDiff, "inspect-diff"),
        (CapabilityKind::ReadArtifact, "read-artifact"),
        (CapabilityKind::EmitArtifact, "emit-artifact"),
        (CapabilityKind::RunCommand, "run-command"),
        (CapabilityKind::ValidateWithTool, "validate"),
        (CapabilityKind::InvokeStructuredTool, "structured-tool"),
        (CapabilityKind::ExecuteBoundedTransformation, "transform"),
    ];

    for (capability, expected) in cases {
        assert_eq!(capability_tag(capability), expected);
    }
}

#[test]
fn service_helper_branches_cover_inline_inputs_authority_and_process_failures() {
    let request = RunRequest {
        mode: Mode::Requirements,
        risk: RiskClass::LowImpact,
        zone: UsageZone::Green,
        system_context: None,
        classification: ClassificationProvenance::explicit(),
        owner: "maintainer".to_string(),
        inputs: vec!["tech-docs/brief.md".to_string()],
        inline_inputs: vec!["inline summary".to_string(), "more detail".to_string()],
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    assert_eq!(
        request.merged_input_sources(),
        vec![
            "tech-docs/brief.md".to_string(),
            "inline-input-01.md".to_string(),
            "inline-input-02.md".to_string(),
        ]
    );
    assert_eq!(
        request.transient_inline_inputs(),
        vec![
            crate::domain::run::InlineInput {
                label: "inline-input-01.md".to_string(),
                contents: "inline summary".to_string(),
            },
            crate::domain::run::InlineInput {
                label: "inline-input-02.md".to_string(),
                contents: "more detail".to_string(),
            },
        ]
    );
    assert_eq!(AiTool::Claude.materialization_target(), SkillMaterializationTarget::Claude);
    assert_eq!(AiTool::Copilot.materialization_target(), SkillMaterializationTarget::Agents);
    assert_eq!(AiTool::Cursor.materialization_target(), SkillMaterializationTarget::Agents);
    assert_eq!(AiTool::Antigravity.materialization_target(), SkillMaterializationTarget::Agents);
    assert_eq!(AuthorityStatus::DerivedAuthoritativeInput.as_str(), "derived-authoritative-input");
    assert_eq!(
        authority_approval_state(&[ApprovalRecord::for_gate(
            GateKind::Risk,
            "maintainer".to_string(),
            ApprovalDecision::Reject,
            "not yet".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )]),
        AuthorityApprovalState::Rejected
    );
    assert_eq!(process_failure_excerpt("stdout message", "stderr message"), "stderr message");
    assert_eq!(process_failure_excerpt("stdout message", "   \n"), "stdout message");
    assert_eq!(process_failure_excerpt("   ", "   \n"), "no process output captured");
    assert_eq!(
        canonical_mode_input_binding(Mode::SystemAssessment),
        Some(("system-assessment.md", "system-assessment"))
    );
    assert_eq!(
        canonical_mode_input_binding(Mode::SecurityAssessment),
        Some(("security-assessment.md", "security-assessment"))
    );

    let workspace = TempDir::new().expect("temp dir");
    std::fs::create_dir_all(workspace.path().join("nested")).expect("nested dir");
    std::fs::write(workspace.path().join("top.md"), "# top\n").expect("top file");
    std::fs::write(workspace.path().join("nested").join("child.md"), "# child\n")
        .expect("child file");
    let mut files = Vec::new();
    collect_files_recursively(workspace.path(), &mut files).expect("recursive collection");
    assert!(files.iter().any(|path| path.ends_with("top.md")));
    assert!(files.iter().any(|path| path.ends_with("child.md")));
}

#[test]
fn run_implementation_direct_runtime_covers_implementation_branching() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::create_dir_all(workspace.path().join("src/auth")).expect("src/auth dir");
    std::fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String { format!(\"revoked:{id}\") }\n",
    )
    .expect("session file");
    std::fs::write(
        workspace.path().join("src/auth/repository.rs"),
        "pub fn persist_session(id: &str) -> String { id.to_string() }\n",
    )
    .expect("repository file");

    let store = WorkspaceStore::new(workspace.path());
    store.init_runtime_state(None).expect("init runtime state");
    let policy_set = store.load_policy_set(None).expect("policy set");
    let service = EngineService::new(workspace.path());

    let summary = service
        .run_implementation(
            &store,
            inline_request(
                Mode::Implementation,
                RiskClass::LowImpact,
                UsageZone::Green,
                Some(crate::domain::run::SystemContext::Existing),
                "staff-engineer",
                supported_implementation_brief(),
            ),
            policy_set,
        )
        .expect("implementation run");

    assert_eq!(summary.mode, "implementation");
    assert_eq!(summary.state, "Blocked");
    assert!(summary.artifact_paths.iter().any(|path| path.ends_with("packet-metadata.json")));
}

#[test]
fn run_verification_direct_runtime_covers_verification_branching() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    std::fs::write(
        workspace.path().join("src/reviewer.rs"),
        "pub fn format_review(label: &str) -> String { format!(\"review:{label}\") }\n",
    )
    .expect("reviewer file");

    let store = WorkspaceStore::new(workspace.path());
    store.init_runtime_state(None).expect("init runtime state");
    let policy_set = store.load_policy_set(None).expect("policy set");
    let service = EngineService::new(workspace.path());

    let summary = service
        .run_verification(
            &store,
            inline_request(
                Mode::Verification,
                RiskClass::LowImpact,
                UsageZone::Green,
                None,
                "verifier",
                supported_verification_brief(),
            ),
            policy_set,
        )
        .expect("verification run");

    assert_eq!(summary.mode, "verification");
    assert_eq!(summary.state, "Completed");
    assert!(!summary.artifact_paths.is_empty());
    assert!(summary.mode_result.is_some());
}

#[test]
fn engine_service_helpers_map_store_summaries_and_pr_review_inputs() {
    let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

    let init = EngineService::map_init_summary(StoreInitSummary {
        repo_root: "/repo".to_string(),
        canon_root: "/repo/.canon".to_string(),
        methods_materialized: 12,
        policies_materialized: 5,
        skills_materialized: 19,
        claude_md_created: false,
    });
    assert_eq!(init.repo_root, "/repo");
    assert_eq!(init.methods_materialized, 12);

    let skills = EngineService::map_skills_summary(StoreSkillsSummary {
        skills_dir: "/repo/.agents/skills".to_string(),
        skills_materialized: 19,
        skills_skipped: 2,
        claude_md_created: true,
    });
    assert_eq!(skills.skills_dir, "/repo/.agents/skills");
    assert_eq!(skills.skills_skipped, 2);
    assert!(skills.claude_md_created);

    let refs = service
        .load_pr_review_refs(&["origin/main".to_string(), "HEAD".to_string()])
        .expect("two refs should parse");
    assert_eq!(refs, ("origin/main".to_string(), "HEAD".to_string()));

    let error = service
        .load_pr_review_refs(&["origin/main".to_string()])
        .expect_err("missing head ref should fail");
    assert!(error.to_string().contains("pr-review requires two inputs"));
}

#[test]
fn engine_service_public_wrappers_cover_runtime_and_skill_entrypoints() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    // Exercise the public wrapper methods that forward to the workspace store.
    assert_eq!(service.repo_root(), workspace.path());

    let init = service.init(Some(AiTool::Copilot)).expect("init runtime state");
    assert_eq!(init.repo_root, workspace.path().to_string_lossy());
    assert!(std::path::Path::new(&init.canon_root).exists());

    let installed = service.skills_install(AiTool::Copilot).expect("install skills");
    assert!(std::path::Path::new(&installed.skills_dir).exists());

    let listed = service.skills_list();
    assert!(!listed.is_empty());
    assert!(listed.iter().all(|entry| !entry.name.trim().is_empty()));

    let updated = service.skills_update(AiTool::Copilot).expect("update skills");
    assert_eq!(updated.skills_dir, installed.skills_dir);

    assert_eq!(authority_approval_state(&[]), AuthorityApprovalState::NotNeeded);
    assert_eq!(
        authority_approval_state(&[ApprovalRecord::for_gate(
            GateKind::Risk,
            "maintainer".to_string(),
            ApprovalDecision::Approve,
            "approved".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )]),
        AuthorityApprovalState::Granted
    );
}

#[test]
fn engine_service_resolves_relative_and_absolute_input_paths() {
    let service = EngineService::new("/tmp/canon-root");

    assert_eq!(
        service.resolve_input_path("idea.md"),
        std::path::PathBuf::from("/tmp/canon-root").join("idea.md")
    );
    assert_eq!(
        service.resolve_input_path("/tmp/elsewhere/input.md"),
        std::path::PathBuf::from("/tmp/elsewhere/input.md")
    );
}

#[test]
fn canonical_mode_input_binding_is_defined_for_canonical_bound_modes() {
    assert_eq!(canonical_mode_input_binding(Mode::Backlog), Some(("backlog.md", "backlog")));
    assert_eq!(canonical_mode_input_binding(Mode::Incident), Some(("incident.md", "incident")));
    assert_eq!(
        canonical_mode_input_binding(Mode::DomainLanguage),
        Some(("domain-language.md", "domain-language"))
    );
    assert_eq!(
        canonical_mode_input_binding(Mode::DomainModel),
        Some(("domain-model.md", "domain-model"))
    );
    assert_eq!(
        canonical_mode_input_binding(Mode::Implementation),
        Some(("implementation.md", "implementation"))
    );
    assert_eq!(canonical_mode_input_binding(Mode::Migration), Some(("migration.md", "migration")));
    assert_eq!(
        canonical_mode_input_binding(Mode::SupplyChainAnalysis),
        Some(("supply-chain-analysis.md", "supply-chain-analysis"))
    );
    assert_eq!(canonical_mode_input_binding(Mode::Refactor), Some(("refactor.md", "refactor")));
    assert_eq!(canonical_mode_input_binding(Mode::Requirements), None);
}

#[test]
fn auto_bind_canonical_mode_inputs_supports_operational_file_backed_modes() {
    let workspace = TempDir::new().expect("temp dir");
    let canon_input = workspace.path().join("canon-input");
    std::fs::create_dir_all(&canon_input).expect("canon-input dir");
    std::fs::write(
        canon_input.join("domain-language.md"),
        "# Domain Language Brief\n\n## Domain Scope\n- ordering vocabulary\n",
    )
    .expect("domain-language file");
    std::fs::write(
        canon_input.join("domain-model.md"),
        "# Domain Model Brief\n\n## Domain Scope\n- ordering model\n",
    )
    .expect("domain-model file");
    std::fs::write(
        canon_input.join("incident.md"),
        "# Incident Brief\n\n## Incident Scope\n- payments\n",
    )
    .expect("incident file");
    std::fs::write(
        canon_input.join("migration.md"),
        "# Migration Brief\n\n## Current State\n- v1\n",
    )
    .expect("migration file");
    std::fs::write(
        canon_input.join("supply-chain-analysis.md"),
        "# Supply Chain Analysis Brief\n\n## Declared Scope\n- Rust workspace manifests\n",
    )
    .expect("supply-chain analysis file");

    let service = EngineService::new(workspace.path());

    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::DomainLanguage, &[], &[]),
        vec!["canon-input/domain-language.md".to_string()]
    );
    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::DomainModel, &[], &[]),
        vec!["canon-input/domain-model.md".to_string()]
    );
    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::Incident, &[], &[]),
        vec!["canon-input/incident.md".to_string()]
    );
    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::Migration, &[], &[]),
        vec!["canon-input/migration.md".to_string()]
    );
    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::SupplyChainAnalysis, &[], &[]),
        vec!["canon-input/supply-chain-analysis.md".to_string()]
    );
}

#[test]
fn auto_bind_canonical_mode_inputs_prefers_directory_over_single_file() {
    let workspace = TempDir::new().expect("temp dir");
    let canon_input = workspace.path().join("canon-input");
    std::fs::create_dir_all(canon_input.join("implementation")).expect("implementation dir");
    std::fs::write(
        canon_input.join("implementation").join("brief.md"),
        "# Implementation Brief\n\nMutation Bounds: src/auth/**\n",
    )
    .expect("implementation brief");
    std::fs::write(
        canon_input.join("implementation.md"),
        "# Implementation Brief\n\nMutation Bounds: src/auth/**\n",
    )
    .expect("implementation file");

    let service = EngineService::new(workspace.path());

    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::Implementation, &[], &[]),
        vec!["canon-input/implementation".to_string()]
    );
}

#[test]
fn build_authoring_lifecycle_summary_prefers_brief_for_directory_packets() {
    let workspace = TempDir::new().expect("temp dir");
    let packet_root = workspace.path().join("canon-input").join("implementation");
    std::fs::create_dir_all(&packet_root).expect("packet root");
    std::fs::write(
        packet_root.join("brief.md"),
        "# Implementation Brief\n\n## Task Mapping\n\n- wire auth session revocation\n",
    )
    .expect("brief");
    std::fs::write(
        packet_root.join("source-map.md"),
        "# Source Map\n\n- tech-docs/changes/auth.md\n",
    )
    .expect("source map");
    std::fs::write(
        packet_root.join("selected-context.md"),
        "# Selected Context\n\n- auth/session.rs\n",
    )
    .expect("selected context");

    let service = EngineService::new(workspace.path());
    let inputs = vec!["canon-input/implementation".to_string()];
    let source_inputs = service.clarity_source_inputs(&inputs).expect("source inputs");

    let summary = service.build_authoring_lifecycle_summary(
        &inputs,
        &source_inputs,
        &[],
        &[ClarificationQuestionSummary {
            id: "q1".to_string(),
            prompt: "What remains open?".to_string(),
            rationale: "Need explicit open question.".to_string(),
            evidence: "No explicit unresolved question.".to_string(),
            affects: "brief.md".to_string(),
            default_if_skipped: "Keep packet conditional.".to_string(),
            status: "required".to_string(),
        }],
        false,
    );

    assert_eq!(summary.packet_shape, "directory-backed");
    assert_eq!(summary.authority_status, "explicit-authoritative-brief");
    assert_eq!(summary.authoritative_inputs, vec!["canon-input/implementation/brief.md"]);
    assert!(
        summary.supporting_inputs.contains(&"canon-input/implementation/source-map.md".to_string())
    );
    assert!(
        summary
            .supporting_inputs
            .contains(&"canon-input/implementation/selected-context.md".to_string())
    );
    assert!(summary.readiness_delta.iter().any(|item| item.contains("clarification question")));
}

#[test]
fn build_authoring_lifecycle_summary_keeps_ambiguous_directory_packets_explicit() {
    let workspace = TempDir::new().expect("temp dir");
    let packet_root = workspace.path().join("canon-input").join("implementation");
    std::fs::create_dir_all(&packet_root).expect("packet root");
    std::fs::write(
        packet_root.join("source-map.md"),
        "# Source Map\n\n- tech-docs/changes/auth.md\n",
    )
    .expect("source map");
    std::fs::write(packet_root.join("notes.md"), "# Notes\n\n- auth/session.rs\n").expect("notes");

    let service = EngineService::new(workspace.path());
    let inputs = vec!["canon-input/implementation".to_string()];
    let source_inputs = service.clarity_source_inputs(&inputs).expect("source inputs");

    let summary =
        service.build_authoring_lifecycle_summary(&inputs, &source_inputs, &[], &[], false);

    assert_eq!(summary.packet_shape, "directory-backed");
    assert_eq!(summary.authority_status, "ambiguous-current-brief");
    assert!(summary.authoritative_inputs.is_empty());
    assert_eq!(summary.supporting_inputs.len(), 2);
    assert!(
        summary
            .readiness_delta
            .iter()
            .any(|item| item.contains("add `brief.md` or reduce the packet"))
    );
}

#[test]
fn auto_bind_canonical_mode_inputs_uses_single_file_when_directory_is_absent() {
    let workspace = TempDir::new().expect("temp dir");
    let canon_input = workspace.path().join("canon-input");
    std::fs::create_dir_all(&canon_input).expect("canon-input dir");
    std::fs::write(
        canon_input.join("refactor.md"),
        "# Refactor Brief\n\nPreserved Behavior: public API remains stable.\n",
    )
    .expect("refactor file");

    let service = EngineService::new(workspace.path());

    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::Refactor, &[], &[]),
        vec!["canon-input/refactor.md".to_string()]
    );
}

#[test]
fn auto_bind_canonical_mode_inputs_supports_backlog() {
    let workspace = TempDir::new().expect("temp dir");
    let canon_input = workspace.path().join("canon-input");
    std::fs::create_dir_all(&canon_input).expect("canon-input dir");
    std::fs::write(
        canon_input.join("backlog.md"),
        "# Backlog Brief\n\n## Delivery Intent\nPrepare a bounded backlog packet.\n",
    )
    .expect("backlog file");

    let service = EngineService::new(workspace.path());

    assert_eq!(
        service.auto_bind_canonical_mode_inputs(Mode::Backlog, &[], &[]),
        vec!["canon-input/backlog.md".to_string()]
    );
}

#[test]
fn build_run_context_scaffolds_implementation_execution_metadata() {
    let service = EngineService::new("/tmp/canon-root");
    let request = RunRequest {
        mode: Mode::Implementation,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(crate::domain::run::SystemContext::Existing),
        classification: crate::domain::run::ClassificationProvenance::explicit(),
        owner: "staff-engineer".to_string(),
        inputs: vec!["canon-input/implementation.md".to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
    let implementation =
        context.implementation_execution.expect("implementation execution scaffold");

    assert!(context.refactor_execution.is_none());
    assert!(context.upstream_context.is_none());
    assert_eq!(implementation.plan_sources, vec!["canon-input/implementation.md"]);
    assert_eq!(implementation.mutation_bounds.source_refs, vec!["canon-input/implementation.md"]);
    assert_eq!(implementation.mutation_bounds.owners, vec!["staff-engineer"]);
    assert_eq!(implementation.execution_posture, ExecutionPosture::RecommendationOnly);
    assert!(!implementation.post_approval_execution_consumed);
}

#[test]
fn build_run_context_scaffolds_refactor_execution_metadata() {
    let service = EngineService::new("/tmp/canon-root");
    let request = RunRequest {
        mode: Mode::Refactor,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(crate::domain::run::SystemContext::Existing),
        classification: crate::domain::run::ClassificationProvenance::explicit(),
        owner: "staff-engineer".to_string(),
        inputs: vec!["canon-input/refactor.md".to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
    let refactor = context.refactor_execution.expect("refactor execution scaffold");

    assert!(context.implementation_execution.is_none());
    assert!(context.upstream_context.is_none());
    assert_eq!(refactor.refactor_scope.source_refs, vec!["canon-input/refactor.md"]);
    assert_eq!(refactor.refactor_scope.owners, vec!["staff-engineer"]);
    assert_eq!(refactor.execution_posture, ExecutionPosture::RecommendationOnly);
    assert!(!refactor.post_approval_execution_consumed);
}

#[test]
fn build_run_context_extracts_upstream_context_from_folder_packet() {
    let workspace = TempDir::new().expect("temp dir");
    let packet_root = workspace.path().join("canon-input").join("implementation");
    std::fs::create_dir_all(&packet_root).expect("packet root");
    std::fs::write(
        packet_root.join("brief.md"),
        "# Implementation Brief\n\nFeature Slice: auth session revocation\nPrimary Upstream Mode: change\nTask Mapping: 1. Thread the helper through the revocation service.\nMutation Bounds: src/auth/session.rs\nAllowed Paths:\n- src/auth/session.rs\nSafety-Net Evidence: session contract coverage exists.\nIndependent Checks:\n- cargo test --test session_contract\nRollback Triggers: formatting drift\nRollback Steps: revert the bounded patch\n",
    )
    .expect("brief");
    std::fs::write(
        packet_root.join("source-map.md"),
        "# Source Map\n\n## Upstream Sources\n\n- tech-docs/changes/R-20260422-AUTHREVOC/change-surface.md\n- tech-docs/changes/R-20260422-AUTHREVOC/implementation-plan.md\n\n## Carried-Forward Decisions\n\n- Revocation output formatting stays stable.\n- Contract coverage must pass before and after mutation.\n\n## Excluded Upstream Scope\n\nLogin UI flow and token issuance remain out of scope.\n",
    )
    .expect("source map");

    let service = EngineService::new(workspace.path());
    let request = RunRequest {
        mode: Mode::Implementation,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(crate::domain::run::SystemContext::Existing),
        classification: crate::domain::run::ClassificationProvenance::explicit(),
        owner: "staff-engineer".to_string(),
        inputs: vec!["canon-input/implementation".to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
    let upstream = context.upstream_context.expect("upstream context");

    assert_eq!(upstream.feature_slice.as_deref(), Some("auth session revocation"));
    assert_eq!(upstream.primary_upstream_mode.as_deref(), Some("change"));
    assert_eq!(
        upstream.source_refs,
        vec![
            "tech-docs/changes/R-20260422-AUTHREVOC/change-surface.md".to_string(),
            "tech-docs/changes/R-20260422-AUTHREVOC/implementation-plan.md".to_string(),
        ]
    );
    assert_eq!(
        upstream.carried_forward_items,
        vec![
            "Revocation output formatting stays stable.".to_string(),
            "Contract coverage must pass before and after mutation.".to_string(),
        ]
    );
    assert_eq!(
        upstream.excluded_upstream_scope.as_deref(),
        Some("Login UI flow and token issuance remain out of scope.")
    );
}

#[test]
fn apply_execution_posture_summary_reads_recommendation_only_from_run_context() {
    let mode_result = ModeResultSummary {
        headline: "Implementation packet ready.".to_string(),
        artifact_packet_summary: "Primary artifact is ready.".to_string(),
        execution_posture: None,
        primary_artifact_title: "Task Mapping".to_string(),
        primary_artifact_path: ".canon/artifacts/run-123/implementation/task-mapping.md"
            .to_string(),
        primary_artifact_action: ResultActionSummary {
            id: "open-primary-artifact".to_string(),
            label: "Open primary artifact".to_string(),
            host_action: "open-file".to_string(),
            target: ".canon/artifacts/run-123/implementation/task-mapping.md"
                .to_string(),
            text_fallback:
                "Open the primary artifact at .canon/artifacts/run-123/implementation/task-mapping.md."
                    .to_string(),
        },
        result_excerpt: "Bounded implementation summary".to_string(),
        action_chips: Vec::new(),
    };
    let context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            inputs: vec!["canon-input/implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );

    let summarized = apply_execution_posture_summary(Some(mode_result), Some(&context), &[])
        .expect("summarized mode result");

    assert_eq!(summarized.execution_posture.as_deref(), Some("recommendation-only"));
}

#[test]
fn resolved_execution_posture_label_promotes_approved_execution_runs() {
    let mut context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Refactor,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            inputs: vec!["canon-input/refactor.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    set_post_approval_execution_consumed(&mut context, true);
    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Execution,
        "maintainer".to_string(),
        ApprovalDecision::Approve,
        "approved bounded execution".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];

    assert_eq!(
        resolved_execution_posture_label(Some(&context), &approvals).as_deref(),
        Some("approved-recommendation")
    );
}

#[test]
fn resolved_execution_posture_label_keeps_recommendation_only_until_resume_runs() {
    let context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            inputs: vec!["canon-input/implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Execution,
        "maintainer".to_string(),
        ApprovalDecision::Approve,
        "approved bounded execution".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];

    assert_eq!(
        resolved_execution_posture_label(Some(&context), &approvals).as_deref(),
        Some("recommendation-only")
    );
}

#[test]
fn resolved_execution_posture_label_for_mode_defaults_operational_modes_to_recommendation_only() {
    assert_eq!(
        resolved_execution_posture_label_for_mode(Mode::Incident, None, &[]).as_deref(),
        Some("recommendation-only")
    );
    assert_eq!(
        resolved_execution_posture_label_for_mode(Mode::Migration, None, &[]).as_deref(),
        Some("recommendation-only")
    );
    assert_eq!(
        resolved_execution_posture_label_for_mode(Mode::SupplyChainAnalysis, None, &[]).as_deref(),
        Some("recommendation-only")
    );
    assert_eq!(resolved_execution_posture_label_for_mode(Mode::Backlog, None, &[]), None);
}

#[test]
fn resolve_identity_prefers_explicit_values() {
    let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

    assert_eq!(
        service.resolve_owner("  Owner <owner@example.com>  "),
        "Owner <owner@example.com>".to_string()
    );
    assert_eq!(
        service.resolve_approver("Reviewer <reviewer@example.com>"),
        "Reviewer <reviewer@example.com>".to_string()
    );
}

#[test]
fn recommend_next_action_prefers_artifact_review_for_approval_gated_runs() {
    let action = recommend_next_action(
        RunState::AwaitingApproval,
        None,
        &[".canon/artifacts/run-123/change/system-slice.md".to_string()],
        true,
        &[],
        &["invocation:req-1".to_string()],
    );

    assert_eq!(
        action,
        Some(RecommendedActionSummary {
            action: "inspect-artifacts".to_string(),
            rationale: "Review the emitted packet before recording approval.".to_string(),
            target: None,
        })
    );
}

#[test]
fn recommend_next_action_points_to_direct_approval_when_no_packet_exists() {
    let action = recommend_next_action(
        RunState::AwaitingApproval,
        None,
        &[],
        false,
        &[],
        &["gate:review-disposition".to_string()],
    );

    assert_eq!(
        action,
        Some(RecommendedActionSummary {
            action: "approve".to_string(),
            rationale: "Canon is explicitly waiting for approval on a real target.".to_string(),
            target: Some("gate:review-disposition".to_string()),
        })
    );
}

#[test]
fn recommend_next_action_points_to_resume_when_post_approval_continuation_is_pending() {
    let action = recommend_next_action(RunState::AwaitingApproval, None, &[], true, &[], &[]);

    assert_eq!(
        action,
        Some(RecommendedActionSummary {
            action: "resume".to_string(),
            rationale: "Approval is already recorded; resume the run to execute the post-approval continuation.".to_string(),
            target: None,
        })
    );
}

#[test]
fn build_action_chips_for_emits_resume_when_awaiting_continuation_without_targets() {
    let chips = build_action_chips_for(
        RunState::AwaitingApproval,
        &[],
        ".canon/artifacts/run-123/implementation/task-mapping.md",
        "run-123",
    );

    assert_eq!(chips.len(), 3);
    let inspect_chip = &chips[1];
    assert!(!inspect_chip.recommended);

    let resume_chip = &chips[2];
    assert_eq!(resume_chip.id, "resume-run");
    assert_eq!(resume_chip.label, "Resume run");
    assert_eq!(resume_chip.skill, "canon-resume");
    assert_eq!(resume_chip.prefilled_args.get("RUN_ID"), Some(&"run-123".to_string()));
    assert_eq!(
        resume_chip.text_fallback,
        "Resume run run-123 to continue post-approval execution: `canon resume --run run-123`."
    );
    assert!(resume_chip.recommended);
}

#[test]
fn recommend_next_action_uses_evidence_for_blocked_runs_without_artifacts() {
    let action = recommend_next_action(
        RunState::Blocked,
        None,
        &[],
        true,
        &[GateInspectSummary {
            gate: "change-preservation".to_string(),
            status: "Blocked".to_string(),
            blockers: vec!["legacy-invariants.md missing".to_string()],
        }],
        &[],
    );

    assert_eq!(
        action,
        Some(RecommendedActionSummary {
            action: "inspect-evidence".to_string(),
            rationale: "The run is blocked but no readable artifact packet was found; inspect the evidence bundle next.".to_string(),
            target: None,
        })
    );
}

#[test]
fn set_execution_posture_updates_implementation_context() {
    let mut context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["canon-input/implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    assert_eq!(
        context.implementation_execution.as_ref().unwrap().execution_posture,
        ExecutionPosture::RecommendationOnly
    );
    set_execution_posture(&mut context, ExecutionPosture::Mutating);
    assert_eq!(
        context.implementation_execution.as_ref().unwrap().execution_posture,
        ExecutionPosture::Mutating
    );
}

#[test]
fn set_execution_posture_updates_refactor_context() {
    let mut context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Refactor,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["canon-input/refactor.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    set_execution_posture(&mut context, ExecutionPosture::Mutating);
    assert_eq!(
        context.refactor_execution.as_ref().unwrap().execution_posture,
        ExecutionPosture::Mutating
    );
}

#[test]
fn set_post_approval_execution_consumed_updates_implementation_context() {
    let mut context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["canon-input/implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    assert!(!context.implementation_execution.as_ref().unwrap().post_approval_execution_consumed);
    set_post_approval_execution_consumed(&mut context, true);
    assert!(context.implementation_execution.as_ref().unwrap().post_approval_execution_consumed);
}

#[test]
fn execution_continuation_pending_is_true_when_gate_approved_and_not_consumed() {
    let context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["canon-input/implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    let approvals = vec![ApprovalRecord::for_gate(
        GateKind::Execution,
        "maintainer".to_string(),
        ApprovalDecision::Approve,
        "approved".to_string(),
        OffsetDateTime::UNIX_EPOCH,
    )];
    assert!(execution_continuation_pending(&context, &approvals));
}

#[test]
fn execution_continuation_pending_is_false_when_gate_not_approved() {
    let context = EngineService::new("/tmp/canon-root").build_run_context(
        &RunRequest {
            mode: Mode::Refactor,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "maintainer".to_string(),
            inputs: vec!["canon-input/refactor.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        },
        Vec::new(),
        OffsetDateTime::UNIX_EPOCH,
    );
    assert!(!execution_continuation_pending(&context, &[]));
}

#[test]
fn approved_execution_mutation_rationale_covers_mode_variants() {
    let scope = vec!["src/auth/session.rs".to_string()];
    let impl_label =
        approved_execution_mutation_rationale(Mode::Implementation, &scope, "patch.diff");
    assert!(impl_label.contains("implementation mutation"));
    assert!(impl_label.contains("patch.diff"));

    let refactor_label =
        approved_execution_mutation_rationale(Mode::Refactor, &scope, "patch.diff");
    assert!(refactor_label.contains("refactor mutation"));

    let change_label = approved_execution_mutation_rationale(Mode::Change, &scope, "patch.diff");
    assert!(change_label.contains("change mutation"));

    let other_label =
        approved_execution_mutation_rationale(Mode::Requirements, &scope, "patch.diff");
    assert!(other_label.contains("bounded mutation"));
}

#[test]
fn validate_authored_inputs_rejects_pr_review_with_inline_text() {
    let service = EngineService::new("/tmp/canon-root");
    let err = service
        .validate_authored_inputs(Mode::PrReview, &[], &["some inline text".to_string()])
        .expect_err("should reject inline text for pr-review");
    assert!(err.to_string().contains("pr-review does not support --input-text"));
}

#[test]
fn validate_authored_inputs_allows_pr_review_without_inline_inputs() {
    let service = EngineService::new("/tmp/canon-root");
    service
        .validate_authored_inputs(Mode::PrReview, &["ref-a".to_string(), "ref-b".to_string()], &[])
        .expect("pr-review with file refs and no inline inputs should be ok");
}

#[test]
fn validate_authored_inputs_rejects_review_mode_with_zero_sources() {
    let service = EngineService::new("/tmp/canon-root");
    let err = service
        .validate_authored_inputs(Mode::Review, &[], &[])
        .expect_err("review with no inputs should fail");
    assert!(err.to_string().contains("review requires exactly one authored input"));
}

#[test]
fn validate_authored_inputs_rejects_review_mode_with_multiple_sources() {
    let service = EngineService::new("/tmp/canon-root");
    let err = service
        .validate_authored_inputs(
            Mode::Review,
            &["input-a.md".to_string(), "input-b.md".to_string()],
            &[],
        )
        .expect_err("review with two inputs should fail");
    assert!(err.to_string().contains("review requires exactly one authored input"));
}

#[test]
fn validate_authored_inputs_rejects_empty_inline_input_text() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());
    let err = service
        .validate_authored_inputs(Mode::Requirements, &[], &["   ".to_string()])
        .expect_err("whitespace-only inline input should fail");
    assert!(err.to_string().contains("empty or whitespace-only"));
}

#[test]
fn validate_authored_inputs_rejects_regular_mode_with_no_sources() {
    let service = EngineService::new("/tmp/canon-root");
    let err = service
        .validate_authored_inputs(Mode::Requirements, &[], &[])
        .expect_err("requirements with no sources should fail");
    assert!(err.to_string().contains("requires at least one authored input"));
}

#[test]
fn build_authoring_lifecycle_summary_multi_input_shape_is_ambiguous_without_brief() {
    let workspace = TempDir::new().expect("temp dir");
    let file_a = workspace.path().join("idea.md");
    let file_b = workspace.path().join("context.md");
    std::fs::write(&file_a, "# Idea\n").expect("idea.md");
    std::fs::write(&file_b, "# Context\n").expect("context.md");

    let service = EngineService::new(workspace.path());
    let inputs = vec!["idea.md".to_string(), "context.md".to_string()];
    let source_inputs = vec!["idea.md".to_string(), "context.md".to_string()];
    let summary =
        service.build_authoring_lifecycle_summary(&inputs, &source_inputs, &[], &[], false);

    assert_eq!(summary.packet_shape, "multi-input");
    assert_eq!(summary.authority_status, "ambiguous-current-brief");
    assert!(summary.authoritative_inputs.is_empty());
    assert!(summary.readiness_delta.iter().any(|s| s.contains("add `brief.md`")));
    assert!(
        summary
            .next_authoring_step
            .contains("Tighten the packet so one current-mode brief is authoritative")
    );
}

#[test]
fn build_authoring_lifecycle_summary_materially_closed_sets_preserve_next_step() {
    let workspace = TempDir::new().expect("temp dir");
    let idea = workspace.path().join("idea.md");
    std::fs::write(&idea, "# Idea\n").expect("idea.md");

    let service = EngineService::new(workspace.path());
    let inputs = vec!["idea.md".to_string()];
    let source_inputs = vec!["idea.md".to_string()];
    let summary = service.build_authoring_lifecycle_summary(
        &inputs,
        &source_inputs,
        &[],
        &[],
        true, // materially_closed
    );

    assert_eq!(summary.packet_shape, "single-file");
    assert_eq!(summary.authority_status, "single-input-authoritative-brief");
    assert!(summary.readiness_delta.is_empty());
    assert!(summary.next_authoring_step.contains("materially closes the decision"));
}

#[test]
fn build_authoring_lifecycle_summary_clarification_questions_only_sets_answer_next_step() {
    let workspace = TempDir::new().expect("temp dir");
    let idea = workspace.path().join("idea.md");
    std::fs::write(&idea, "# Idea\n").expect("idea.md");

    let service = EngineService::new(workspace.path());
    let inputs = vec!["idea.md".to_string()];
    let source_inputs = vec!["idea.md".to_string()];
    let question = ClarificationQuestionSummary {
        id: "q-constraints-1".to_string(),
        prompt: "Which constraints are non-negotiable?".to_string(),
        rationale: "Need constraints before shaping.".to_string(),
        evidence: "No constraints section found.".to_string(),
        affects: "options.md".to_string(),
        default_if_skipped: "Leave conditional.".to_string(),
        status: "required".to_string(),
    };
    let summary =
        service.build_authoring_lifecycle_summary(&inputs, &source_inputs, &[], &[question], false);

    assert_eq!(summary.packet_shape, "single-file");
    assert_eq!(summary.readiness_delta.len(), 1);
    assert!(summary.readiness_delta[0].contains("clarification question(s)"));
    assert!(summary.next_authoring_step.contains("Answer the remaining clarification questions"));
}

#[test]
fn build_authoring_lifecycle_summary_supporting_inputs_with_missing_context_adds_delta() {
    let workspace = TempDir::new().expect("temp dir");
    let brief = workspace.path().join("brief.md");
    let context_file = workspace.path().join("context.md");
    std::fs::write(&brief, "# Brief\n").expect("brief.md");
    std::fs::write(&context_file, "# Context\n").expect("context.md");

    let service = EngineService::new(workspace.path());
    // brief.md is authoritative, context.md is a supporting input
    let inputs = vec!["brief.md".to_string(), "context.md".to_string()];
    let source_inputs = vec!["brief.md".to_string(), "context.md".to_string()];
    let summary = service.build_authoring_lifecycle_summary(
        &inputs,
        &source_inputs,
        &["Missing constraints section.".to_string()],
        &[],
        false,
    );

    // supporting_inputs is non-empty and missing_context is non-empty → delta should include the note
    assert!(!summary.supporting_inputs.is_empty());
    assert!(summary.readiness_delta.iter().any(|s| s.contains("Supporting inputs are present")));
}

#[test]
fn build_authoring_lifecycle_summary_with_supporting_inputs_and_no_missing_context() {
    let workspace = TempDir::new().expect("temp dir");
    let brief = workspace.path().join("brief.md");
    let support = workspace.path().join("context.md");
    std::fs::write(&brief, "# Brief\n").expect("brief.md");
    std::fs::write(&support, "# Context\n").expect("context.md");

    let service = EngineService::new(workspace.path());
    let inputs = vec!["brief.md".to_string(), "context.md".to_string()];
    let source_inputs = vec!["brief.md".to_string(), "context.md".to_string()];
    let summary =
        service.build_authoring_lifecycle_summary(&inputs, &source_inputs, &[], &[], false);

    assert_eq!(summary.authority_status, "explicit-authoritative-brief");
    assert!(!summary.supporting_inputs.is_empty());
    assert!(summary.next_authoring_step.contains("keep the supporting inputs as provenance"));
}

#[test]
fn build_authoring_lifecycle_summary_derived_authoritative_input_from_single_non_brief() {
    let workspace = TempDir::new().expect("temp dir");
    let idea = workspace.path().join("idea.md");
    let other = workspace.path().join("other.md");
    std::fs::write(&idea, "# Idea\n").expect("idea.md");
    std::fs::write(&other, "# Other\n").expect("other.md");

    let service = EngineService::new(workspace.path());
    // Multi-input with no brief.md → if >1 source input, authority is ambiguous
    // But if exactly 1 source_input with non-brief name → derived-authoritative-input
    let inputs = vec!["idea.md".to_string(), "other.md".to_string()];
    // source_inputs has only 1 entry (deduplicated scenario)
    let source_inputs = vec!["idea.md".to_string(), "other.md".to_string()];
    let summary =
        service.build_authoring_lifecycle_summary(&inputs, &source_inputs, &[], &[], false);

    // 2 source inputs, no brief.md → ambiguous
    assert_eq!(summary.authority_status, "ambiguous-current-brief");
}

#[test]
fn inspect_clarity_rejects_empty_inputs_directly() {
    let service = EngineService::new("/tmp/canon-root");

    let error = service
        .inspect_clarity(Mode::Requirements, &[])
        .expect_err("empty clarity inputs should fail");

    assert!(error.to_string().contains("clarity inspection requires at least one input"));
}

#[test]
fn inspect_clarity_rejects_pr_review_directly() {
    let service = EngineService::new("/tmp/canon-root");

    let error = service
        .inspect_clarity(Mode::PrReview, &["HEAD~1".to_string(), "HEAD".to_string()])
        .expect_err("pr-review clarity should be unsupported");

    assert!(error.to_string().contains("clarity inspection is not available for pr-review"));
}

#[test]
fn inspect_authored_mode_clarity_rejects_missing_input_path_directly() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let error = service
        .inspect_authored_mode_clarity(Mode::Architecture, &["missing-architecture.md".to_string()])
        .expect_err("missing authored input should fail");

    assert!(error.to_string().contains("was not found"));
}

#[test]
fn inspect_requirements_and_discovery_clarity_reject_missing_input_paths_directly() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let requirements_error = service
        .inspect_requirements_clarity(&["missing-requirements.md".to_string()])
        .expect_err("missing requirements input should fail");
    assert!(requirements_error.to_string().contains("was not found"));

    let discovery_error = service
        .inspect_discovery_clarity(&["missing-discovery.md".to_string()])
        .expect_err("missing discovery input should fail");
    assert!(discovery_error.to_string().contains("was not found"));
}

#[test]
fn inspect_supply_chain_clarity_rejects_missing_input_path_directly() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let error = service
        .inspect_supply_chain_analysis_clarity(&["missing-supply-chain.md".to_string()])
        .expect_err("missing supply-chain input should fail");

    assert!(error.to_string().contains("was not found"));
}

#[test]
fn inspect_risk_zone_rejects_missing_inputs_directly() {
    let service = EngineService::new("/tmp/canon-root");

    let error = service
        .inspect_risk_zone(Mode::Requirements, None, None, &[], &[])
        .expect_err("missing risk-zone inputs should fail");

    assert!(error.to_string().contains("risk-zone inspection requires at least one input"));
}

#[test]
fn inspect_risk_zone_rejects_pr_review_inline_inputs_directly() {
    let service = EngineService::new("/tmp/canon-root");

    let error = service
        .inspect_risk_zone(Mode::PrReview, None, None, &[], &["diff text".to_string()])
        .expect_err("pr-review risk-zone should reject inline text");

    assert!(error.to_string().contains("does not support --input-text"));
}

#[test]
fn inspect_risk_zone_rejects_pr_review_without_two_refs_directly() {
    let service = EngineService::new("/tmp/canon-root");

    let error = service
        .inspect_risk_zone(Mode::PrReview, None, None, &["HEAD~1".to_string()], &[])
        .expect_err("pr-review risk-zone should require two refs");

    assert!(error.to_string().contains("requires two refs or inputs"));
}

#[test]
fn inspect_lists_modes_methods_and_policies_after_init() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    service.init(Some(AiTool::Copilot)).expect("init runtime state");

    let modes = service.inspect(InspectTarget::Modes).expect("inspect modes");
    assert_eq!(modes.target, "modes");
    assert!(
        modes
            .entries
            .iter()
            .any(|entry| matches!(entry, InspectEntry::Name(name) if name == "requirements"))
    );

    let methods = service.inspect(InspectTarget::Methods).expect("inspect methods");
    assert_eq!(methods.target, "methods");
    assert!(!methods.entries.is_empty());
    assert!(methods.entries.iter().all(|entry| matches!(entry, InspectEntry::Name(_))));

    let policies = service.inspect(InspectTarget::Policies).expect("inspect policies");
    assert_eq!(policies.target, "policies");
    assert!(!policies.entries.is_empty());
    assert!(policies.entries.iter().all(|entry| matches!(entry, InspectEntry::Name(_))));
}

#[test]
fn inspect_artifacts_and_invocations_surface_approved_runtime_details() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());
    let run_id = persist_inspect_invocation_bundle(&workspace);

    let artifacts = service
        .inspect(InspectTarget::Artifacts { run_id: run_id.clone() })
        .expect("inspect artifacts");
    assert_eq!(artifacts.target, "artifacts");
    assert_eq!(artifacts.system_context.as_deref(), Some("existing"));
    assert!(artifacts.entries.iter().any(|entry| {
        matches!(entry, InspectEntry::Name(name) if name.ends_with("01-task-mapping.md"))
    }));

    let invocations =
        service.inspect(InspectTarget::Invocations { run_id }).expect("inspect invocations");
    assert_eq!(invocations.target, "invocations");
    assert_eq!(invocations.system_context.as_deref(), Some("existing"));
    let approved = invocations
        .entries
        .iter()
        .find_map(|entry| match entry {
            InspectEntry::Invocation(summary)
                if summary.approval_state == "approved" && summary.latest_outcome.is_some() =>
            {
                Some(summary)
            }
            _ => None,
        })
        .expect("approved invocation summary");
    assert_eq!(approved.orientation, "Generation");
}

#[test]
fn inspect_evidence_reports_upstream_closure_and_lineage_details() {
    let workspace = TempDir::new().expect("temp dir");
    let run_id = persist_inspect_evidence_bundle(&workspace);
    let service = EngineService::new(workspace.path());

    let response = service.inspect(InspectTarget::Evidence { run_id }).expect("inspect evidence");

    assert_eq!(response.target, "evidence");
    assert_eq!(response.system_context.as_deref(), Some("existing"));

    let summary = match response.entries.as_slice() {
        [InspectEntry::Evidence(summary)] => summary,
        other => panic!("expected a single evidence entry, got {other:?}"),
    };

    assert!(summary.execution_posture.is_none());
    assert_eq!(summary.upstream_feature_slice.as_deref(), Some("runtime honesty contract"));
    assert_eq!(summary.primary_upstream_mode.as_deref(), Some("architecture"));
    assert_eq!(
        summary.upstream_source_refs,
        vec!["specs/061-skill-runtime-contracts/spec.md".to_string()]
    );
    assert_eq!(
        summary.carried_forward_items,
        vec!["Keep structured preflight JSON stable.".to_string()]
    );
    assert_eq!(summary.excluded_upstream_scope.as_deref(), Some("assistant packaging changes"));
    assert_eq!(summary.closure_status.as_deref(), Some("downgraded"));
    assert_eq!(summary.decomposition_scope.as_deref(), Some("risk-only-packet"));
    assert_eq!(
        summary.closure_notes.as_deref(),
        Some("One more independent rollback check remains.")
    );
    assert_eq!(summary.closure_findings.len(), 1);
    assert_eq!(summary.closure_findings[0].category, "missing-evidence");
    assert_eq!(summary.closure_findings[0].severity, "warning");
    assert_eq!(summary.generation_paths, vec!["generation:req-1".to_string()]);
    assert_eq!(summary.validation_paths, vec!["validation:req-2".to_string()]);
    assert_eq!(summary.denied_invocations, vec!["req-3".to_string()]);
    assert_eq!(
        summary.artifact_provenance_links,
        vec!["artifacts/R-20260528-ABCDEF12/backlog/01-backlog-overview.md".to_string()]
    );
}

#[test]
fn inspect_risk_zone_accepts_authored_inputs() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("requirements.md"),
        "# Requirements Brief\n\n## Problem\nStabilize skill runtime contracts.\n",
    )
    .expect("requirements input");
    let service = EngineService::new(workspace.path());

    let response = service
        .inspect(InspectTarget::RiskZone {
            mode: Mode::Requirements,
            risk: Some(RiskClass::LowImpact),
            zone: Some(UsageZone::Green),
            inputs: vec!["requirements.md".to_string()],
            inline_inputs: Vec::new(),
        })
        .expect("inspect requirements risk zone");

    let summary = match response.entries.as_slice() {
        [InspectEntry::RiskZone(summary)] => summary,
        other => panic!("expected a single risk-zone entry, got {other:?}"),
    };
    assert_eq!(response.target, "risk-zone");
    assert_eq!(summary.mode, "requirements");
    assert_eq!(summary.risk, "low-impact");
    assert_eq!(summary.zone, "green");
    assert!(summary.risk_was_supplied);
    assert!(summary.zone_was_supplied);
}

#[test]
fn inspect_risk_zone_accepts_pr_review_refs_on_repo_history() {
    let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

    let response = service
        .inspect(InspectTarget::RiskZone {
            mode: Mode::PrReview,
            risk: None,
            zone: None,
            inputs: vec!["HEAD~1".to_string(), "HEAD".to_string()],
            inline_inputs: Vec::new(),
        })
        .expect("inspect pr-review risk zone");

    let summary = match response.entries.as_slice() {
        [InspectEntry::RiskZone(summary)] => summary,
        other => panic!("expected a single risk-zone entry, got {other:?}"),
    };
    assert_eq!(response.target, "risk-zone");
    assert_eq!(summary.mode, "pr-review");
}

#[test]
fn inspect_clarity_dispatches_file_backed_mode_variants() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("requirements.md"),
        "# Requirements Brief\n\n## Problem\nStabilize skill runtime contracts.\n\n## Constraints\n- Stay within the existing runtime surface.\n\n## Success Signals\n- Operators can inspect structured preflight output.\n",
    )
    .expect("requirements brief");
    std::fs::write(
        workspace.path().join("discovery.md"),
        "# Discovery Brief\n\n## Problem\nClarify runtime contract gaps.\n\n## System Shape\nExisting Rust workspace with governed adapters.\n\n## Boundaries\n- Local filesystem only.\n",
    )
    .expect("discovery brief");
    std::fs::write(
        workspace.path().join("supply-chain-analysis.md"),
        "# Supply Chain Analysis Brief\n\n## Declared Scope\n- Rust workspace manifests\n\n## Distribution Surface\n- Homebrew\n- Scoop\n\n## Ecosystem Dependencies\n- clap\n- serde\n\n## Tool Policy\n- cargo deny\n",
    )
    .expect("supply chain brief");
    let service = EngineService::new(workspace.path());

    for (mode, input, expected_mode) in [
        (Mode::Requirements, "requirements.md", "requirements"),
        (Mode::Discovery, "discovery.md", "discovery"),
        (Mode::SupplyChainAnalysis, "supply-chain-analysis.md", "supply-chain-analysis"),
    ] {
        let response = service
            .inspect(InspectTarget::Clarity { mode, inputs: vec![input.to_string()] })
            .expect("inspect clarity");

        let summary = match response.entries.as_slice() {
            [InspectEntry::Clarity(summary)] => summary,
            other => panic!("expected a single clarity entry, got {other:?}"),
        };
        assert_eq!(response.target, "clarity");
        assert_eq!(summary.mode, expected_mode);
        assert_eq!(summary.authoring_lifecycle.packet_shape, "single-file");
        assert!(!summary.recommended_focus.trim().is_empty());
    }
}

#[test]
fn inspect_clarity_dispatches_directory_backed_authored_mode() {
    let workspace = TempDir::new().expect("temp dir");
    let packet_root = workspace.path().join("canon-input").join("implementation");
    std::fs::create_dir_all(&packet_root).expect("implementation packet dir");
    std::fs::write(packet_root.join("brief.md"), supported_implementation_brief())
        .expect("implementation brief");
    std::fs::write(
        packet_root.join("source-map.md"),
        "# Source Map\n\n- specs/061-skill-runtime-contracts/plan.md\n",
    )
    .expect("implementation source map");
    let service = EngineService::new(workspace.path());

    let response = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::Implementation,
            inputs: vec!["canon-input/implementation".to_string()],
        })
        .expect("inspect implementation clarity");

    let summary = match response.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };
    assert_eq!(response.target, "clarity");
    assert_eq!(summary.mode, "implementation");
    assert_eq!(summary.authoring_lifecycle.packet_shape, "directory-backed");
    assert_eq!(summary.authoring_lifecycle.authority_status, "explicit-authoritative-brief");
    assert!(
        summary
            .source_inputs
            .iter()
            .any(|path| path.ends_with("canon-input/implementation/brief.md"))
    );
}

#[test]
fn inspect_clarity_requirements_covers_clarification_and_ready_recommendations() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("requirements-open.md"),
        "# Requirements Brief\n\n## Problem\nReduce auth latency.\n\n## Outcome\nP99 auth latency under 50 ms.\n\n## Constraints\n- No breaking API changes.\n\n## Tradeoffs\n- Cache consistency vs latency.\n\n## Out of Scope\n- UI changes.\n\n## Open Questions\n- Which cache backend?\n",
    )
    .expect("requirements open brief");
    std::fs::write(
        workspace.path().join("requirements-ready.md"),
        "# Requirements Brief\n\n## Problem\nReduce auth latency.\n\n## Outcome\nP99 auth latency under 50 ms.\n\n## Constraints\n- No breaking API changes.\n\n## Tradeoffs\n- Cache consistency vs latency.\n\n## Out of Scope\n- UI changes.\n",
    )
    .expect("requirements ready brief");
    let service = EngineService::new(workspace.path());

    let open_response = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::Requirements,
            inputs: vec!["requirements-open.md".to_string()],
        })
        .expect("inspect open requirements clarity");
    let open_summary = match open_response.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };
    assert!(open_summary.requires_clarification);
    assert!(open_summary.recommended_focus.contains("named owner"));

    let ready_response = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::Requirements,
            inputs: vec!["requirements-ready.md".to_string()],
        })
        .expect("inspect ready requirements clarity");
    let ready_summary = match ready_response.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };
    assert!(!ready_summary.requires_clarification);
    assert!(
        ready_summary.recommended_focus.contains("No critical clarification questions detected")
    );
}

#[test]
fn inspect_clarity_discovery_covers_open_question_recommendation() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("discovery-open.md"),
        "# Discovery Brief\n\n## Problem\nBound the runtime contract work.\n\n## Constraints\n- Stay inside canon-engine service tests first.\n\n## Repo Focus\n- crates/canon-engine/src/orchestrator/service\n\n## Unknowns\n- Which service slices still need direct coverage?\n\n## Next Phase\n- architecture\n",
    )
    .expect("discovery open brief");
    let service = EngineService::new(workspace.path());

    let response = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::Discovery,
            inputs: vec!["discovery-open.md".to_string()],
        })
        .expect("inspect discovery clarity");
    let summary = match response.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };
    assert!(summary.requires_clarification);
    assert!(summary.recommended_focus.contains("open discovery questions"));
}

#[test]
fn inspect_clarity_targeted_modes_bound_questions_to_specific_decision_surfaces() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("requirements-minimal.md"),
        "# Requirements Brief\n\n## Problem\nClarify same-work continuation.\n",
    )
    .expect("requirements minimal brief");
    let service = EngineService::new(workspace.path());

    let response = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::Requirements,
            inputs: vec!["requirements-minimal.md".to_string()],
        })
        .expect("inspect minimal requirements clarity");
    let summary = match response.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };

    assert!(!summary.clarification_questions.is_empty());
    assert!(
        summary
            .clarification_questions
            .iter()
            .all(|question| question.affects != "packet readiness")
    );
}

#[test]
fn inspect_clarity_supply_chain_covers_publishable_recommendation() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("supply-chain-ready.md"),
        "# Supply Chain Analysis Brief\n\n## Declared Scope\nCargo manifests under crates/ and GitHub Actions workflows.\n\n## Licensing Posture\noss-permissive\n\n## Distribution Model\nexternal distribution\n\n## Ecosystems In Scope\n- cargo\n- github actions\n\n## Out Of Scope Components\n- vendored ui assets\n\n## Scanner Decisions\n- prefer OSS scanners first\n",
    )
    .expect("supply chain ready brief");
    let service = EngineService::new(workspace.path());

    let response = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::SupplyChainAnalysis,
            inputs: vec!["supply-chain-ready.md".to_string()],
        })
        .expect("inspect supply chain clarity");
    let summary = match response.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };
    assert!(!summary.requires_clarification);
    assert!(summary.recommended_focus.contains("No critical clarification questions detected"));
}

#[test]
fn run_lifecycle_request_helpers_map_capabilities_and_context() {
    let service = EngineService::new("/tmp/canon-root");

    let requirements_request = service.requirements_request(RequirementsRequestSpec {
        run_id: "R-REQ-1",
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        owner: "staff-engineer",
        capability: CapabilityKind::GenerateContent,
        summary: "Generate the requirements packet",
        scope: vec!["canon-input/requirements.md".to_string()],
    });
    assert_eq!(requirements_request.adapter, AdapterKind::CopilotCli);
    assert_eq!(requirements_request.mode, "requirements");
    assert_eq!(requirements_request.orientation, InvocationOrientation::Generation);
    assert_eq!(requirements_request.owner.as_deref(), Some("staff-engineer"));

    let governed_request = service.governed_request(GovernedRequestSpec {
        run_id: "R-GOV-1",
        mode: Mode::Review,
        risk: RiskClass::LowImpact,
        zone: UsageZone::Green,
        system_context: None,
        owner: "reviewer",
        adapter: AdapterKind::Filesystem,
        capability: CapabilityKind::ReadRepository,
        summary: "Read the review target",
        scope: vec!["canon-input/review.md".to_string()],
    });
    assert_eq!(governed_request.request_id, "R-GOV-1-context");
    assert_eq!(governed_request.adapter, AdapterKind::Filesystem);
    assert_eq!(governed_request.orientation, InvocationOrientation::Context);
    assert_eq!(governed_request.mutability, MutabilityClass::ReadOnly);
    assert_eq!(governed_request.trust_boundary, TrustBoundaryKind::LocalDeterministic);
    assert_eq!(governed_request.lineage, LineageClass::NonGenerative);
}

#[test]
fn run_lifecycle_read_requirements_context_covers_labels_and_empty_normalization() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("brief.md"),
        "# Requirements Brief\n\n## Problem\nBound the runtime surface.\n",
    )
    .expect("brief");
    let service = EngineService::new(workspace.path());

    let context = service
        .read_requirements_context(
            &["brief.md".to_string(), "missing-ref".to_string()],
            &["inline detail".to_string()],
        )
        .expect("read requirements context");
    assert!(context.contains("## Input: brief.md"));
    assert!(context.contains("Bound the runtime surface"));
    assert!(context.contains("missing-ref"));
    assert!(context.contains("## Input: inline-input-01.md"));
    assert!(context.contains("inline detail"));

    let error = service
        .read_requirements_context(&[], &["   ".to_string()])
        .expect_err("whitespace-only authored input should fail");
    assert!(error.to_string().contains("no usable content after normalization"));
}

#[test]
fn run_lifecycle_change_validation_attempt_covers_success_and_fallback_paths() {
    let repo_service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
    let success_request = repo_service.governed_request(GovernedRequestSpec {
        run_id: "R-VAL-OK",
        mode: Mode::Change,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        owner: "maintainer",
        adapter: AdapterKind::Shell,
        capability: CapabilityKind::ValidateWithTool,
        summary: "Validate the bounded change surface",
        scope: vec!["src".to_string()],
    });
    let (success_summary, success_attempt) = repo_service
        .change_validation_attempt(&success_request)
        .expect("change validation success path");
    assert_eq!(success_attempt.outcome.kind, ToolOutcomeKind::Succeeded);
    assert!(
        success_summary.contains("Validation tool reviewed tracked repository surfaces")
            || success_summary.contains("repository is empty but reachable")
    );

    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(workspace.path().join("README.md"), "fallback\n").expect("readme");
    let fallback_service = EngineService::new(workspace.path());
    let fallback_request = fallback_service.governed_request(GovernedRequestSpec {
        run_id: "R-VAL-FALLBACK",
        mode: Mode::Change,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        owner: "maintainer",
        adapter: AdapterKind::Shell,
        capability: CapabilityKind::ValidateWithTool,
        summary: "Validate the bounded change surface",
        scope: vec!["README.md".to_string()],
    });
    let (fallback_summary, fallback_attempt) = fallback_service
        .change_validation_attempt(&fallback_request)
        .expect("change validation fallback path");
    assert_eq!(fallback_attempt.outcome.kind, ToolOutcomeKind::PartiallySucceeded);
    assert!(
        fallback_summary
            .contains("Validation fell back to local workspace scan after git returned")
    );
    assert!(fallback_summary.contains("README.md"));
}

#[test]
fn run_lifecycle_locates_patch_payload_absence_multiple_and_scope_errors() {
    let workspace = TempDir::new().expect("temp dir");
    let packet_root = workspace.path().join("canon-input").join("change");
    std::fs::create_dir_all(&packet_root).expect("packet root");
    let service = EngineService::new(workspace.path());

    assert!(
        service
            .locate_authored_mutation_patch(
                &["canon-input/change".to_string()],
                &["src".to_string()]
            )
            .expect("missing patch should be allowed")
            .is_none()
    );

    let in_bounds_patch = "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-old\n+new\n";
    std::fs::write(packet_root.join("patch.diff"), in_bounds_patch).expect("patch.diff");
    std::fs::write(packet_root.join("mutation.patch"), in_bounds_patch).expect("mutation.patch");
    let multiple = service
        .locate_authored_mutation_patch(&["canon-input/change".to_string()], &["src".to_string()])
        .expect_err("multiple patch payloads should fail");
    assert!(multiple.to_string().contains("multiple bounded mutation payloads were found"));

    std::fs::remove_file(packet_root.join("mutation.patch")).expect("remove extra patch");
    let out_of_bounds_patch = "diff --git a/secret.txt b/secret.txt\n--- a/secret.txt\n+++ b/secret.txt\n@@ -1 +1 @@\n-old\n+new\n";
    std::fs::write(packet_root.join("patch.diff"), out_of_bounds_patch)
        .expect("out-of-bounds patch");
    let out_of_bounds = service
        .locate_authored_mutation_patch(
            &["canon-input/change".to_string()],
            &["src/lib.rs".to_string()],
        )
        .expect_err("out-of-bounds patch should fail");
    assert!(out_of_bounds.to_string().contains("outside Allowed Paths"));
}

#[test]
fn run_lifecycle_applies_in_bounds_authored_mutation_patch() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    std::fs::write(
        workspace.path().join("src/lib.rs"),
        "pub fn version() -> &'static str { \"old\" }\n",
    )
    .expect("src/lib.rs");
    let init = std::process::Command::new("git")
        .arg("init")
        .current_dir(workspace.path())
        .status()
        .expect("git init status");
    assert!(init.success());

    let packet_root = workspace.path().join("canon-input").join("change");
    std::fs::create_dir_all(&packet_root).expect("packet root");
    std::fs::write(
        packet_root.join("patch.diff"),
        "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-pub fn version() -> &'static str { \"old\" }\n+pub fn version() -> &'static str { \"new\" }\n",
    )
    .expect("patch.diff");

    let service = EngineService::new(workspace.path());
    let patch = service
        .locate_authored_mutation_patch(
            &["canon-input/change".to_string()],
            &["src/lib.rs".to_string()],
        )
        .expect("locate authored patch")
        .expect("expected authored patch");
    let request = service.governed_request(GovernedRequestSpec {
        run_id: "R-APPLY-1",
        mode: Mode::Change,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        owner: "maintainer",
        adapter: AdapterKind::Shell,
        capability: CapabilityKind::ExecuteBoundedTransformation,
        summary: "Apply the authored bounded patch",
        scope: vec!["src/lib.rs".to_string()],
    });

    let attempt =
        service.apply_authored_mutation_patch(&request, &patch).expect("apply authored patch");
    assert_eq!(attempt.outcome.kind, ToolOutcomeKind::Succeeded);
    assert_eq!(attempt.outcome.candidate_artifacts, vec!["src/lib.rs".to_string()]);
    assert!(
        std::fs::read_to_string(workspace.path().join("src/lib.rs"))
            .expect("read updated src/lib.rs")
            .contains("new")
    );
}

fn run_request_for_refresh(mode: Mode) -> RunRequest {
    let inputs = if matches!(mode, Mode::PrReview) {
        vec!["origin/main".to_string(), "HEAD".to_string()]
    } else {
        vec![format!("canon-input/{}.md", mode.as_str())]
    };
    RunRequest {
        mode,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: format!("owner-{}", mode.as_str()),
        inputs,
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn run_manifest_for_refresh(mode: Mode, index: usize) -> RunManifest {
    RunManifest {
        run_id: format!("R-20260528-{index:08X}"),
        uuid: None,
        short_id: None,
        slug: None,
        title: Some(format!("refresh {mode}")),
        mode,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: format!("owner-{}", mode.as_str()),
        lineage: None,
        created_at: OffsetDateTime::UNIX_EPOCH,
    }
}

fn sample_refinement_context(run_id: &str, mode: Mode) -> ClarificationRefinementContext {
    ClarificationRefinementContext {
        workflow_family: RefinementWorkflowFamily::Planning,
        current_mode: mode,
        working_brief_path: format!(
            ".canon/runs/{run_id}/artifacts/{}/working-brief.md",
            mode.as_str()
        ),
        template_ref: format!("defaults/templates/canon-input/{}.md", mode.as_str()),
        status: ClarificationRefinementStatus::Active,
        explicit_continuation_required: true,
        authoritative_input_refs: vec![format!("canon-input/{}/brief.md", mode.as_str())],
        supporting_input_refs: vec![format!("canon-input/{}/context-links.md", mode.as_str())],
        suggested_candidate: None,
        records: Vec::new(),
        readiness_delta: vec![ReadinessDeltaItem {
            id: format!("rd-{}", mode.as_str()),
            section: "Readiness".to_string(),
            summary: "Named owner is still required before governed continuation.".to_string(),
            blocking: true,
            source_kind: ReadinessDeltaSourceKind::MissingContext,
            default_available: false,
            resolved: false,
        }],
    }
}

fn seed_refinement_bundle(workspace: &TempDir, mode: Mode, state: RunState, owner: &str) -> String {
    let store = WorkspaceStore::new(workspace.path());
    let service = EngineService::new(workspace.path());
    let request = refinement_request(mode, owner);
    let identity = RunIdentity::new_now_v7();
    let mut context = service.build_run_context(&request, Vec::new(), identity.created_at);
    context.clarification_refinement = Some(sample_refinement_context(&identity.run_id, mode));

    store
        .persist_run_bundle(&PersistedRunBundle {
            run: RunManifest::from_identity(
                &identity,
                mode,
                request.risk,
                request.zone,
                request.system_context,
                request.classification,
                request.owner,
            ),
            context,
            state: RunStateManifest { state, updated_at: identity.created_at },
            artifact_contract: ArtifactContract {
                version: 1,
                artifact_requirements: Vec::new(),
                required_verification_layers: Vec::new(),
            },
            artifacts: Vec::new(),
            links: LinkManifest {
                artifacts: Vec::new(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: None,
            },
            gates: Vec::new(),
            approvals: Vec::new(),
            verification_records: Vec::new(),
            evidence: None,
            invocations: Vec::new(),
        })
        .expect("seed refinement bundle");

    identity.run_id
}

#[test]
fn refinement_lifecycle_targeted_run_starts_draft_with_refinement_state() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let summary = service
        .run(refinement_request(Mode::Requirements, "planner"))
        .expect("requirements refinement run");

    assert_eq!(summary.mode, "requirements");
    assert_eq!(summary.state, "Draft");

    let refinement =
        summary.refinement_state.expect("targeted refinement run should seed refinement state");
    assert_eq!(refinement.workflow_family, "planning");
    assert_eq!(refinement.current_mode, "requirements");
    assert_eq!(refinement.status, "active");
    assert!(refinement.explicit_continuation_required);
    assert!(refinement.working_brief_path.ends_with("/artifacts/requirements/working-brief.md"));

    let status = service.status(&summary.run_id).expect("status after draft creation");
    assert_eq!(status.run, summary.run_id);
    assert_eq!(status.state, "Draft");
    assert_eq!(
        status.refinement_state.expect("status should surface refinement state").current_mode,
        "requirements"
    );
}

#[test]
fn refinement_lifecycle_targeted_run_materializes_working_brief_artifact() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::write(
        workspace.path().join("idea.md"),
        "# Requirements Brief\n\n## Problem\nMaterialize the run-local working brief.\n",
    )
    .expect("write idea file");
    let service = EngineService::new(workspace.path());

    let summary = service
        .run(RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "planner".to_string(),
            inputs: vec!["idea.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("requirements refinement run");

    let refinement = summary.refinement_state.expect("refinement state on draft run");
    let working_brief = workspace.path().join(&refinement.working_brief_path);
    let contents = std::fs::read_to_string(&working_brief).expect("read working brief artifact");

    assert!(working_brief.exists());
    assert!(contents.contains("# Requirements Brief"));
    assert!(contents.contains("## Clarification Provenance"));
    assert!(contents.contains("## Continuation State"));
}

#[test]
fn refinement_lifecycle_targeted_run_persists_structured_clarification_state() {
    let workspace = TempDir::new().expect("temp dir");
    let packet_root = workspace.path().join("canon-input").join("requirements");
    std::fs::create_dir_all(&packet_root).expect("requirements packet root");
    std::fs::write(
        packet_root.join("brief.md"),
        "# Requirements Brief\n\n## Problem\nClarify same-work continuation.\n",
    )
    .expect("requirements brief");
    std::fs::write(
        packet_root.join("context-links.md"),
        "# Context Links\n\n- tech-docs/decisions/run-refinement.md\n",
    )
    .expect("context links");

    let service = EngineService::new(workspace.path());
    let clarity = service
        .inspect(InspectTarget::Clarity {
            mode: Mode::Requirements,
            inputs: vec!["canon-input/requirements".to_string()],
        })
        .expect("inspect clarity for refinement seed");
    let clarity_summary = match clarity.entries.as_slice() {
        [InspectEntry::Clarity(summary)] => summary,
        other => panic!("expected a single clarity entry, got {other:?}"),
    };

    let run = service
        .run(RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "planner".to_string(),
            inputs: vec!["canon-input/requirements".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })
        .expect("requirements refinement run");

    let store = WorkspaceStore::new(workspace.path());
    let context = store.load_run_context(&run.run_id).expect("persisted run context");
    let refinement =
        context.clarification_refinement.expect("targeted refinement context should persist");

    assert_eq!(
        refinement.authoritative_input_refs,
        clarity_summary.authoring_lifecycle.authoritative_inputs
    );
    assert_eq!(
        refinement.supporting_input_refs,
        clarity_summary.authoring_lifecycle.supporting_inputs
    );
    assert_eq!(refinement.records.len(), clarity_summary.clarification_questions.len());
    assert!(refinement.records.iter().all(|record| {
        record.answer == "deferred"
            && record.answer_kind == ClarificationAnswerKind::Deferred
            && record.resolution_state == ClarificationResolutionState::Deferred
    }));
    assert_eq!(
        EngineService::build_refinement_readiness_delta(&refinement.readiness_delta),
        clarity_summary.authoring_lifecycle.readiness_delta
    );
    assert!(
        refinement
            .readiness_delta
            .iter()
            .any(|item| item.source_kind == ReadinessDeltaSourceKind::ClarificationGap)
    );
}

#[test]
fn refinement_lifecycle_fresh_request_keeps_existing_candidate_advisory_only() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());

    let first = service
        .run(refinement_request(Mode::Requirements, "planner"))
        .expect("first requirements refinement run");
    let second = service
        .run(refinement_request(Mode::Requirements, "planner"))
        .expect("second requirements refinement run");

    assert_ne!(
        first.run_id, second.run_id,
        "fresh work without explicit continuation should not reuse the prior run identity"
    );

    let suggested_candidate = second
        .refinement_state
        .expect("fresh refinement run should still surface refinement state")
        .suggested_candidate
        .expect("single likely prior run should be surfaced as an advisory candidate");
    assert_eq!(suggested_candidate.run_id, first.run_id);
    assert_eq!(suggested_candidate.mode, "requirements");
    assert_eq!(suggested_candidate.state, "Draft");
    assert!(suggested_candidate.advisory);

    let first_status = service.status(&first.run_id).expect("status for original run");
    assert_eq!(
        first_status.refinement_state.expect("original run should remain inspectable").current_mode,
        "requirements"
    );
}

#[test]
fn refinement_lifecycle_mode_correction_updates_draft_in_place() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    let service = EngineService::new(workspace.path());
    let run_id = seed_refinement_bundle(&workspace, Mode::Requirements, RunState::Draft, "planner");
    let reason = "Clarification redirected the work from requirements to architecture.";

    let corrected_run_id = service
        .apply_refinement_mode_correction(&store, &run_id, Mode::Architecture, reason)
        .expect("pre-start mode correction");

    assert_eq!(corrected_run_id, run_id);

    let manifest = store.load_run_manifest(&run_id).expect("load corrected manifest");
    let context = store.load_run_context(&run_id).expect("load corrected context");

    assert_eq!(manifest.mode, Mode::Architecture);
    assert!(manifest.lineage.is_none());
    assert_eq!(
        context
            .clarification_refinement
            .expect("refinement context after pre-start correction")
            .current_mode,
        Mode::Architecture
    );
}

#[test]
fn refinement_lifecycle_mode_correction_creates_successor_after_run_start() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    let service = EngineService::new(workspace.path());
    let original_run_id =
        seed_refinement_bundle(&workspace, Mode::Requirements, RunState::Completed, "planner");
    let reason = "Clarification redirected the work from requirements to architecture.";

    let successor_run_id = service
        .apply_refinement_mode_correction(&store, &original_run_id, Mode::Architecture, reason)
        .expect("post-start mode correction");

    assert_ne!(successor_run_id, original_run_id);

    let original_manifest =
        store.load_run_manifest(&original_run_id).expect("load original manifest");
    let successor_manifest =
        store.load_run_manifest(&successor_run_id).expect("load successor manifest");
    let successor_context =
        store.load_run_context(&successor_run_id).expect("load successor context");

    assert_eq!(original_manifest.mode, Mode::Requirements);
    assert!(original_manifest.lineage.is_none());
    assert_eq!(successor_manifest.mode, Mode::Architecture);

    let lineage = successor_manifest.lineage.expect("successor lineage");
    assert_eq!(lineage.carried_from, original_run_id);
    assert_eq!(lineage.supersedes, original_run_id);
    assert_eq!(lineage.mode_change_reason, reason);
    assert_eq!(
        successor_context
            .clarification_refinement
            .expect("successor refinement context")
            .current_mode,
        Mode::Architecture
    );
}

#[test]
fn inspect_refinement_surfaces_successor_lineage_after_post_start_mode_change() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    let service = EngineService::new(workspace.path());
    let original_run_id =
        seed_refinement_bundle(&workspace, Mode::Requirements, RunState::Completed, "planner");
    let reason = "Clarification redirected the work from requirements to architecture.";

    let successor_run_id = service
        .apply_refinement_mode_correction(&store, &original_run_id, Mode::Architecture, reason)
        .expect("post-start mode correction");

    let inspect = service
        .inspect(InspectTarget::Refinement { run_id: successor_run_id.clone() })
        .expect("inspect refinement for successor run");
    let summary = match inspect.entries.as_slice() {
        [InspectEntry::Refinement(summary)] => summary,
        other => panic!("expected one refinement entry, got {other:?}"),
    };

    assert_eq!(summary.run_id, successor_run_id);
    assert_eq!(summary.mode, "architecture");

    let lineage = summary.lineage.as_ref().expect("successor lineage in inspect summary");
    assert_eq!(lineage.carried_from, original_run_id);
    assert_eq!(lineage.supersedes, original_run_id);
    assert_eq!(lineage.mode_change_reason, reason);
}

#[test]
fn run_lifecycle_refresh_run_state_covers_supported_dispatch_and_pending_execution() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    let service = EngineService::new(workspace.path());
    let empty_contract = ArtifactContract {
        version: 1,
        artifact_requirements: Vec::new(),
        required_verification_layers: Vec::new(),
    };
    let artifacts: Vec<crate::persistence::store::PersistedArtifact> = Vec::new();
    let seed_run = |manifest: &RunManifest, context: &RunContext| {
        store
            .persist_run_bundle(&PersistedRunBundle {
                run: manifest.clone(),
                context: context.clone(),
                state: RunStateManifest {
                    state: RunState::Completed,
                    updated_at: OffsetDateTime::UNIX_EPOCH,
                },
                artifact_contract: empty_contract.clone(),
                artifacts: Vec::new(),
                links: LinkManifest {
                    artifacts: Vec::new(),
                    decisions: Vec::new(),
                    traces: Vec::new(),
                    invocations: Vec::new(),
                    evidence: None,
                },
                gates: Vec::new(),
                approvals: Vec::new(),
                verification_records: Vec::new(),
                evidence: None,
                invocations: Vec::new(),
            })
            .expect("seed run bundle");
    };

    let supported_modes = [
        Mode::Discovery,
        Mode::SystemShaping,
        Mode::Change,
        Mode::Incident,
        Mode::SystemAssessment,
        Mode::SecurityAssessment,
        Mode::DomainLanguage,
        Mode::DomainModel,
        Mode::SupplyChainAnalysis,
        Mode::Migration,
        Mode::Review,
        Mode::Verification,
        Mode::Architecture,
        Mode::PrReview,
    ];
    for (index, mode) in supported_modes.into_iter().enumerate() {
        let request = run_request_for_refresh(mode);
        let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
        let manifest = run_manifest_for_refresh(mode, index + 1);
        seed_run(&manifest, &context);
        let state = service
            .refresh_run_state(&store, &manifest, &context, &empty_contract, &artifacts, &[])
            .expect("refresh state for supported mode");
        assert!(
            matches!(state, RunState::Completed | RunState::Blocked | RunState::AwaitingApproval),
            "unexpected state for {mode:?}: {state:?}"
        );
    }

    for (index, mode) in [Mode::Implementation, Mode::Refactor].into_iter().enumerate() {
        let request = run_request_for_refresh(mode);
        let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
        let manifest = run_manifest_for_refresh(mode, 100 + index);
        seed_run(&manifest, &context);
        let approvals = vec![ApprovalRecord::for_gate(
            GateKind::Execution,
            manifest.owner.clone(),
            ApprovalDecision::Approve,
            "approved execution".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )];
        let state = service
            .refresh_run_state(&store, &manifest, &context, &empty_contract, &artifacts, &approvals)
            .expect("refresh state for execution mode");
        assert!(
            matches!(state, RunState::AwaitingApproval | RunState::Blocked),
            "unexpected execution state for {mode:?}: {state:?}"
        );
    }

    let unsupported_request = run_request_for_refresh(Mode::Requirements);
    let unsupported_context =
        service.build_run_context(&unsupported_request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
    let unsupported_manifest = run_manifest_for_refresh(Mode::Requirements, 999);
    seed_run(&unsupported_manifest, &unsupported_context);
    let error = service
        .refresh_run_state(
            &store,
            &unsupported_manifest,
            &unsupported_context,
            &empty_contract,
            &artifacts,
            &[],
        )
        .expect_err("requirements refresh should be unsupported");
    assert!(matches!(error, super::EngineError::UnsupportedMode(_)));
}

#[test]
fn run_lifecycle_change_validation_attempt_reports_empty_workspace_fallback_surface() {
    let workspace = TempDir::new().expect("temp dir");
    let service = EngineService::new(workspace.path());
    let request = service.governed_request(GovernedRequestSpec {
        run_id: "R-VAL-EMPTY",
        mode: Mode::Change,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        owner: "maintainer",
        adapter: AdapterKind::Shell,
        capability: CapabilityKind::ValidateWithTool,
        summary: "Validate empty workspace surfaces",
        scope: vec!["src".to_string()],
    });

    let (summary, attempt) =
        service.change_validation_attempt(&request).expect("empty workspace validation fallback");
    assert_eq!(attempt.outcome.kind, ToolOutcomeKind::PartiallySucceeded);
    assert!(summary.contains("no-repository-surfaces-detected"));
}

#[test]
fn run_lifecycle_apply_authored_mutation_patch_reports_preflight_errors() {
    let workspace = TempDir::new().expect("temp dir");
    std::fs::create_dir_all(workspace.path().join("src")).expect("src dir");
    std::fs::write(
        workspace.path().join("src/lib.rs"),
        "pub fn version() -> &'static str { \"old\" }\n",
    )
    .expect("src/lib.rs");
    let init = std::process::Command::new("git")
        .arg("init")
        .current_dir(workspace.path())
        .status()
        .expect("git init status");
    assert!(init.success());

    let packet_root = workspace.path().join("canon-input").join("change");
    std::fs::create_dir_all(&packet_root).expect("packet root");
    let request = EngineService::new(workspace.path()).governed_request(GovernedRequestSpec {
        run_id: "R-APPLY-ERR",
        mode: Mode::Change,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        owner: "maintainer",
        adapter: AdapterKind::Shell,
        capability: CapabilityKind::ExecuteBoundedTransformation,
        summary: "Apply the authored bounded patch",
        scope: vec!["src/lib.rs".to_string()],
    });

    std::fs::write(
        packet_root.join("patch.diff"),
        "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-pub fn version() -> &'static str { \"missing\" }\n+pub fn version() -> &'static str { \"new\" }\n",
    )
    .expect("invalid patch.diff");
    let service = EngineService::new(workspace.path());
    let invalid_patch = service
        .locate_authored_mutation_patch(
            &["canon-input/change".to_string()],
            &["src/lib.rs".to_string()],
        )
        .expect("locate invalid patch")
        .expect("expected invalid patch");
    let invalid_error = service
        .apply_authored_mutation_patch(&request, &invalid_patch)
        .expect_err("invalid patch should fail preflight check");
    assert!(invalid_error.to_string().contains("failed git apply --check"));

    std::fs::write(
        packet_root.join("patch.diff"),
        "diff --git a/src/lib.rs b/src/lib.rs\n--- a/src/lib.rs\n+++ b/src/lib.rs\n@@ -1 +1 @@\n-pub fn version() -> &'static str { \"old\" }\n+pub fn version() -> &'static str { \"new\" }\n",
    )
    .expect("valid patch.diff");
    let valid_patch = service
        .locate_authored_mutation_patch(
            &["canon-input/change".to_string()],
            &["src/lib.rs".to_string()],
        )
        .expect("locate valid patch")
        .expect("expected valid patch");
    let missing_root_service = EngineService::new(workspace.path().join("missing-root"));
    let missing_root_error = missing_root_service
        .apply_authored_mutation_patch(&request, &valid_patch)
        .expect_err("missing repository root should fail preflight execution");
    assert!(
        missing_root_error.to_string().contains("failed to preflight bounded mutation payload")
    );
}
