use super::{
    EngineService, GateInspectSummary, ModeResultSummary, RecommendedActionSummary,
    ResultActionSummary, RunRequest, apply_execution_posture_summary,
    approved_execution_mutation_rationale, build_action_chips_for, canonical_mode_input_binding,
    capability_tag, execution_continuation_pending, extract_change_surface_entries,
    preserve_multiline_summary, recommend_next_action, resolved_execution_posture_label,
    resolved_execution_posture_label_for_mode, run_state_from_gates, set_execution_posture,
    set_post_approval_execution_consumed,
};
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::execution::ExecutionPosture;
use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::RunState;
use crate::persistence::store::{
    InitSummary as StoreInitSummary, SkillsSummary as StoreSkillsSummary,
};
use canon_adapters::CapabilityKind;
use tempfile::TempDir;
use time::OffsetDateTime;

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
        "# Source Map\n\n## Upstream Sources\n\n- docs/changes/R-20260422-AUTHREVOC/change-surface.md\n- docs/changes/R-20260422-AUTHREVOC/implementation-plan.md\n\n## Carried-Forward Decisions\n\n- Revocation output formatting stays stable.\n- Contract coverage must pass before and after mutation.\n\n## Excluded Upstream Scope\n\nLogin UI flow and token issuance remain out of scope.\n",
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
            "docs/changes/R-20260422-AUTHREVOC/change-surface.md".to_string(),
            "docs/changes/R-20260422-AUTHREVOC/implementation-plan.md".to_string(),
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
