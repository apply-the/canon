//! Mode-result summarizers, action-chip builder, and related helpers.
//!
//! All functions here produce display-ready data from `PersistedArtifact` slices
//! and run state; they do not write to disk or mutate domain state.

use std::collections::BTreeMap;

use crate::domain::mode::Mode;
use crate::domain::run::RunState;
use crate::persistence::store::PersistedArtifact;

use super::context_parse::{
    count_context_items_without_placeholders, count_markdown_entries,
    count_missing_context_markers, extract_context_section, extract_labeled_context_value,
    extract_labeled_usize, extract_result_section, truncate_context_excerpt,
};
use super::{ActionChip, ModeResultSummary, ResultActionSummary};

// ── Mode-result dispatch ──────────────────────────────────────────────────────

pub(crate) fn summarize_mode_result(
    mode: Mode,
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    match mode {
        Mode::Requirements => summarize_requirements_mode_result(artifacts),
        Mode::Discovery => summarize_discovery_mode_result(artifacts),
        Mode::SystemShaping => summarize_system_shaping_mode_result(artifacts),
        Mode::Architecture => summarize_architecture_mode_result(artifacts),
        Mode::Change => summarize_change_mode_result(artifacts),
        Mode::Implementation => summarize_implementation_mode_result(artifacts),
        Mode::Refactor => summarize_refactor_mode_result(artifacts),
        Mode::Review => summarize_review_mode_result(artifacts),
        Mode::Verification => summarize_verification_mode_result(artifacts),
        Mode::PrReview => summarize_pr_review_mode_result(artifacts),
        _ => None,
    }
}

// ── Action chip builder ───────────────────────────────────────────────────────

pub(crate) fn build_action_chips_for(
    state: RunState,
    approval_targets: &[String],
    primary_artifact_path: &str,
    run_id: &str,
) -> Vec<ActionChip> {
    let mut chips: Vec<ActionChip> = Vec::new();

    if !primary_artifact_path.is_empty() {
        let mut args = BTreeMap::new();
        args.insert("PATH".to_string(), primary_artifact_path.to_string());
        chips.push(ActionChip {
            id: "open-primary-artifact".to_string(),
            label: "Open primary artifact".to_string(),
            skill: "host:open-file".to_string(),
            intent: "Inspect".to_string(),
            prefilled_args: args,
            required_user_inputs: Vec::new(),
            visibility_condition: "primary_artifact_path is non-empty".to_string(),
            recommended: false,
            text_fallback: format!("Open the primary artifact at {primary_artifact_path}."),
        });
    }

    if matches!(state, RunState::AwaitingApproval | RunState::Completed) && !run_id.is_empty() {
        let mut args = BTreeMap::new();
        args.insert("RUN_ID".to_string(), run_id.to_string());
        chips.push(ActionChip {
            id: "inspect-evidence".to_string(),
            label: "Inspect evidence".to_string(),
            skill: "canon-inspect-evidence".to_string(),
            intent: "Inspect".to_string(),
            prefilled_args: args,
            required_user_inputs: Vec::new(),
            visibility_condition: "state is AwaitingApproval or Completed".to_string(),
            recommended: matches!(state, RunState::AwaitingApproval)
                && !approval_targets.is_empty(),
            text_fallback: format!("Use $canon-inspect-evidence for run {run_id}."),
        });
    }

    if matches!(state, RunState::AwaitingApproval) && !run_id.is_empty() {
        for target in approval_targets {
            let mut args = BTreeMap::new();
            args.insert("RUN_ID".to_string(), run_id.to_string());
            args.insert("TARGET".to_string(), target.clone());
            chips.push(ActionChip {
                id: format!("approve-{}", target.replace(':', "-")),
                label: "Approve generation...".to_string(),
                skill: "canon-approve".to_string(),
                intent: "GovernedAction".to_string(),
                prefilled_args: args,
                required_user_inputs: vec![
                    "BY".to_string(),
                    "DECISION".to_string(),
                    "RATIONALE".to_string(),
                ],
                visibility_condition:
                    "state is AwaitingApproval and Canon issued an approval target".to_string(),
                recommended: false,
                text_fallback: format!(
                    "Review the packet for run {run_id}, then approve using $canon-approve."
                ),
            });
        }

        if approval_targets.is_empty() {
            let mut args = BTreeMap::new();
            args.insert("RUN_ID".to_string(), run_id.to_string());
            chips.push(ActionChip {
                id: "resume-run".to_string(),
                label: "Resume run".to_string(),
                skill: "canon-resume".to_string(),
                intent: "GovernedAction".to_string(),
                prefilled_args: args,
                required_user_inputs: Vec::new(),
                visibility_condition:
                    "state is AwaitingApproval and Canon has no remaining approval targets"
                        .to_string(),
                recommended: true,
                text_fallback: format!(
                    "Use $canon-resume for run {run_id} to continue post-approval execution."
                ),
            });
        }
    }

    chips
}

// ── Primary-artifact action ───────────────────────────────────────────────────

pub(crate) fn primary_artifact_action_for(path: &str) -> ResultActionSummary {
    ResultActionSummary {
        id: "open-primary-artifact".to_string(),
        label: "Open primary artifact".to_string(),
        host_action: "open-file".to_string(),
        target: path.to_string(),
        text_fallback: format!("Open the primary artifact at {path}."),
    }
}

// ── Mode summarizers ──────────────────────────────────────────────────────────

fn summarize_requirements_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "problem-statement.md")?;
    let constraints_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "constraints.md");
    let scope_cuts_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "scope-cuts.md");
    let decision_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "decision-checklist.md");

    let problem = extract_context_section(&primary.contents, "Problem")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem statement summary is missing.".to_string());
    let constraints = constraints_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Constraints"))
        .unwrap_or_else(|| "NOT CAPTURED - Constraints section is missing.".to_string());
    let scope_cuts = scope_cuts_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Scope Cuts"))
        .unwrap_or_else(|| "NOT CAPTURED - Scope cuts section is missing.".to_string());
    let open_questions = decision_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Questions"))
        .unwrap_or_else(|| "NOT CAPTURED - Open questions section is missing.".to_string());

    let missing_context_markers = [&problem, &constraints, &scope_cuts, &open_questions]
        .into_iter()
        .filter(|section| section.contains("NOT CAPTURED"))
        .count();
    let constraint_count = count_markdown_entries(&constraints);
    let scope_cut_count = count_markdown_entries(&scope_cuts);
    let open_question_count = count_markdown_entries(&open_questions);

    let headline = if missing_context_markers == 0 {
        "Requirements packet ready for downstream review.".to_string()
    } else {
        format!(
            "Requirements packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact is ready. Packet captures {constraint_count} constraint point(s), {scope_cut_count} scope cut(s), and {open_question_count} open question(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Constraints: {constraint_count}; scope cuts: {scope_cut_count}; open questions: {open_question_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Statement".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_discovery_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "problem-map.md")?;
    let unknowns_artifact = artifacts
        .iter()
        .find(|artifact| artifact.record.file_name == "unknowns-and-assumptions.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "context-boundary.md");

    let problem_domain = extract_context_section(&primary.contents, "Problem Domain")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem domain summary is missing.".to_string());
    let repo_signals = extract_context_section(&primary.contents, "Repo Signals")
        .unwrap_or_else(|| "NOT CAPTURED - Repository signals are missing.".to_string());
    let next_phase = extract_context_section(&primary.contents, "Downstream Handoff")
        .or_else(|| {
            boundary_artifact.and_then(|artifact| {
                extract_context_section(&artifact.contents, "Translation Trigger")
            })
        })
        .unwrap_or_else(|| "NOT CAPTURED - Next-phase handoff is missing.".to_string());
    let unknowns = unknowns_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Unknowns"))
        .unwrap_or_else(|| "NOT CAPTURED - Unknowns section is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&problem_domain, &repo_signals, &next_phase, &unknowns]);
    let repo_signal_count = count_markdown_entries(&repo_signals);
    let unknown_count = count_markdown_entries(&unknowns);

    let headline = if missing_context_markers == 0 {
        "Discovery packet ready for downstream translation.".to_string()
    } else {
        format!(
            "Discovery packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = format!(
        "Primary artifact maps {repo_signal_count} repository signal(s) and {unknown_count} unknown or assumption set(s). Next phase: {}.",
        truncate_context_excerpt(&next_phase, 120)
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Map".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem_domain, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_system_shaping_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "system-shape.md")?;
    let capability_map =
        artifacts.iter().find(|artifact| artifact.record.file_name == "capability-map.md");
    let delivery_options =
        artifacts.iter().find(|artifact| artifact.record.file_name == "delivery-options.md");
    let risk_hotspots =
        artifacts.iter().find(|artifact| artifact.record.file_name == "risk-hotspots.md");

    let system_shape = extract_context_section(&primary.contents, "System Shape")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - System shape summary is missing.".to_string());
    let boundary_decisions = extract_context_section(&primary.contents, "Boundary Decisions")
        .unwrap_or_else(|| "NOT CAPTURED - Boundary decisions are missing.".to_string());
    let capabilities = capability_map
        .and_then(|artifact| extract_context_section(&artifact.contents, "Capabilities"))
        .unwrap_or_else(|| "NOT CAPTURED - Capability map is missing.".to_string());
    let delivery_phases = delivery_options
        .and_then(|artifact| extract_context_section(&artifact.contents, "Delivery Phases"))
        .unwrap_or_else(|| "NOT CAPTURED - Delivery phases are missing.".to_string());
    let hotspots = risk_hotspots
        .and_then(|artifact| extract_context_section(&artifact.contents, "Hotspots"))
        .unwrap_or_else(|| "NOT CAPTURED - Risk hotspots are missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &system_shape,
        &boundary_decisions,
        &capabilities,
        &delivery_phases,
        &hotspots,
    ]);
    let capability_count = count_markdown_entries(&capabilities);
    let delivery_count = count_markdown_entries(&delivery_phases);
    let hotspot_count = count_markdown_entries(&hotspots);

    let headline = if missing_context_markers == 0 {
        "System-shaping packet ready for downstream architecture or delivery planning.".to_string()
    } else {
        format!(
            "System-shaping packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = format!(
        "Primary artifact names {capability_count} capability slice(s), {delivery_count} delivery phase set(s), and {hotspot_count} risk hotspot set(s)."
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "System Shape".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&system_shape, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_architecture_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts
        .iter()
        .find(|artifact| artifact.record.file_name == "architecture-decisions.md")?;
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "invariants.md");
    let tradeoff_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "tradeoff-matrix.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "boundary-map.md");

    let decisions = extract_context_section(&primary.contents, "Decisions")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Architecture decisions are missing.".to_string());
    let tradeoffs = extract_context_section(&primary.contents, "Tradeoffs")
        .or_else(|| {
            tradeoff_artifact
                .and_then(|artifact| extract_context_section(&artifact.contents, "Scores"))
        })
        .unwrap_or_else(|| "NOT CAPTURED - Architecture tradeoffs are missing.".to_string());
    let invariants = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Invariants are missing.".to_string());
    let boundaries = boundary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Boundaries"))
        .unwrap_or_else(|| "NOT CAPTURED - Boundary map is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&decisions, &tradeoffs, &invariants, &boundaries]);
    let decision_count = count_markdown_entries(&decisions);
    let tradeoff_count = count_markdown_entries(&tradeoffs);
    let invariant_count = count_markdown_entries(&invariants);
    let boundary_count = count_markdown_entries(&boundaries);

    let headline = if missing_context_markers == 0 {
        "Architecture packet ready for downstream implementation or review.".to_string()
    } else {
        format!(
            "Architecture packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = format!(
        "Primary artifact records {decision_count} decision set(s), {tradeoff_count} tradeoff set(s), {invariant_count} invariant set(s), and {boundary_count} boundary set(s)."
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Architecture Decisions".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&decisions, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_change_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "change-surface.md")?;
    let legacy_invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "legacy-invariants.md");
    let validation_strategy_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "validation-strategy.md");
    let system_slice_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "system-slice.md");

    let (change_surface, change_surface_missing) = extract_result_section(
        &primary.contents,
        "Change Surface",
        "Missing Context",
        "NOT CAPTURED - Change surface section is missing.",
    );
    let (legacy_invariants, legacy_missing) = legacy_invariants_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Legacy Invariants",
                "Missing Context",
                "NOT CAPTURED - Legacy invariants section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Legacy invariants artifact is missing.".to_string(), true)
        });
    let (validation_strategy, validation_missing) = validation_strategy_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Validation Strategy",
                "Missing Context",
                "NOT CAPTURED - Validation strategy section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Validation strategy artifact is missing.".to_string(), true)
        });
    let (system_slice, system_slice_missing) = system_slice_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "System Slice",
                "Missing Context",
                "NOT CAPTURED - System slice section is missing.",
            )
        })
        .unwrap_or_else(|| ("NOT CAPTURED - System slice artifact is missing.".to_string(), true));

    let missing_context_markers =
        [change_surface_missing, legacy_missing, validation_missing, system_slice_missing]
            .into_iter()
            .filter(|missing| *missing)
            .count();
    let change_surface_count = count_markdown_entries(&change_surface);
    let legacy_invariant_count = count_markdown_entries(&legacy_invariants);
    let validation_count = count_markdown_entries(&validation_strategy);

    let headline = if missing_context_markers == 0 {
        "Change packet ready for bounded change review.".to_string()
    } else {
        format!(
            "Change packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact names {change_surface_count} change-surface point(s). Packet also captures {legacy_invariant_count} legacy invariant(s) and {validation_count} validation check set(s) for the bounded slice {}.",
            truncate_context_excerpt(&system_slice, 90)
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Change surface: {change_surface_count}; legacy invariants: {legacy_invariant_count}; validation checks: {validation_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Change Surface".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&change_surface, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_implementation_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "task-mapping.md")?;
    let mutation_bounds_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "mutation-bounds.md");
    let validation_hooks_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "validation-hooks.md");

    let (task_mapping, task_mapping_missing) = extract_result_section(
        &primary.contents,
        "Task Mapping",
        "Missing Context",
        "NOT CAPTURED - Task mapping section is missing.",
    );
    let (bounded_changes, bounded_changes_missing) = extract_result_section(
        &primary.contents,
        "Bounded Changes",
        "Missing Context",
        "NOT CAPTURED - Bounded changes section is missing.",
    );
    let (allowed_paths, allowed_paths_missing) = mutation_bounds_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Allowed Paths",
                "Missing Context",
                "NOT CAPTURED - Allowed paths section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Mutation bounds artifact is missing.".to_string(), true)
        });
    let (safety_net_evidence, safety_net_missing) = validation_hooks_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Safety-Net Evidence",
                "Missing Context",
                "NOT CAPTURED - Safety-net evidence section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Validation hooks artifact is missing.".to_string(), true)
        });

    let missing_context_markers =
        [task_mapping_missing, bounded_changes_missing, allowed_paths_missing, safety_net_missing]
            .into_iter()
            .filter(|missing| *missing)
            .count();
    let task_count = count_markdown_entries(&task_mapping);
    let allowed_path_count = count_markdown_entries(&allowed_paths);
    let safety_net_count = count_markdown_entries(&safety_net_evidence);

    let headline = if missing_context_markers == 0 {
        "Implementation packet ready for bounded execution review.".to_string()
    } else {
        format!(
            "Implementation packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact maps {task_count} task set(s) across {allowed_path_count} allowed path set(s) with {safety_net_count} safety-net evidence set(s). Bounded changes: {}.",
            truncate_context_excerpt(&bounded_changes, 90)
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Tasks: {task_count}; allowed paths: {allowed_path_count}; safety-net evidence: {safety_net_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Task Mapping".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&task_mapping, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_refactor_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "preserved-behavior.md")?;
    let scope_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "refactor-scope.md");
    let contract_drift_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "contract-drift-check.md");
    let no_feature_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "no-feature-addition.md");

    let (preserved_behavior, preserved_missing) = extract_result_section(
        &primary.contents,
        "Preserved Behavior",
        "Missing Context",
        "NOT CAPTURED - Preserved behavior section is missing.",
    );
    let (_refactor_scope, scope_missing) = scope_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Refactor Scope",
                "Missing Context",
                "NOT CAPTURED - Refactor scope section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Refactor scope artifact is missing.".to_string(), true)
        });
    let (allowed_paths, allowed_paths_missing) = scope_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Allowed Paths",
                "Missing Context",
                "NOT CAPTURED - Allowed paths section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Refactor scope artifact is missing.".to_string(), true)
        });
    let (contract_drift, drift_missing) = contract_drift_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Contract Drift",
                "Missing Context",
                "NOT CAPTURED - Contract drift section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Contract drift artifact is missing.".to_string(), true)
        });
    let (feature_audit, feature_audit_missing) = no_feature_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Feature Audit",
                "Missing Context",
                "NOT CAPTURED - Feature audit section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - No-feature-addition artifact is missing.".to_string(), true)
        });

    let missing_context_markers = [
        preserved_missing,
        scope_missing,
        allowed_paths_missing,
        drift_missing,
        feature_audit_missing,
    ]
    .into_iter()
    .filter(|missing| *missing)
    .count();
    let preserved_count = count_markdown_entries(&preserved_behavior);
    let allowed_path_count = count_markdown_entries(&allowed_paths);
    let feature_audit_count = count_markdown_entries(&feature_audit);

    let headline = if missing_context_markers == 0 {
        "Refactor packet ready for preservation review.".to_string()
    } else {
        format!(
            "Refactor packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact names {preserved_count} preserved-behavior set(s) across {allowed_path_count} allowed path set(s). Contract drift note: {}. Feature audit sets: {feature_audit_count}.",
            truncate_context_excerpt(&contract_drift, 90)
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Preserved behavior: {preserved_count}; allowed paths: {allowed_path_count}; feature audit: {feature_audit_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Preserved Behavior".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&preserved_behavior, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_review_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "review-disposition.md")?;
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "boundary-assessment.md");
    let missing_evidence_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "missing-evidence.md");

    let final_disposition = extract_context_section(&primary.contents, "Final Disposition")
        .unwrap_or_else(|| "NOT CAPTURED - Final disposition section is missing.".to_string());
    let accepted_risks = extract_context_section(&primary.contents, "Accepted Risks")
        .unwrap_or_else(|| "NOT CAPTURED - Accepted risks section is missing.".to_string());
    let boundary_findings = boundary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Boundary Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Boundary findings section is missing.".to_string());
    let missing_evidence = missing_evidence_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Missing Evidence"))
        .unwrap_or_else(|| "NOT CAPTURED - Missing evidence section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &final_disposition,
        &accepted_risks,
        &boundary_findings,
        &missing_evidence,
    ]);
    let disposition_status = extract_labeled_context_value(&final_disposition, "Status")
        .unwrap_or_else(|| "unknown-disposition".to_string());
    let rationale = extract_labeled_context_value(&final_disposition, "Rationale")
        .unwrap_or_else(|| truncate_context_excerpt(&final_disposition, 320));
    let boundary_count = count_context_items_without_placeholders(
        &boundary_findings,
        &["No boundary expansion beyond the authored review target was detected."],
    );
    let accepted_risk_count = count_context_items_without_placeholders(
        &accepted_risks,
        &[
            "No accepted risks recorded while disposition is still pending.",
            "Residual review notes remain bounded to the current package and can be inspected through the emitted artifacts.",
        ],
    );

    let headline = if missing_context_markers == 0 {
        match disposition_status.as_str() {
            "awaiting-disposition" => {
                "Review packet requires explicit disposition before release-readiness can pass."
                    .to_string()
            }
            "accepted-with-approval" => {
                "Review packet completed with explicit approval for the remaining concerns."
                    .to_string()
            }
            _ => "Review packet ready for downstream inspection and bounded follow-up.".to_string(),
        }
    } else {
        format!(
            "Review packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact records `{disposition_status}` disposition with {boundary_count} boundary finding set(s) and {accepted_risk_count} accepted-risk or review-note set(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Boundary findings: {boundary_count}; accepted risks: {accepted_risk_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Review Disposition".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&rationale, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_verification_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "verification-report.md")?;
    let unresolved_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "unresolved-findings.md");
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "invariants-checklist.md");

    let verified_claims = extract_context_section(&primary.contents, "Verified Claims")
        .unwrap_or_else(|| "NOT CAPTURED - Verified claims section is missing.".to_string());
    let rejected_claims = extract_context_section(&primary.contents, "Rejected Claims")
        .unwrap_or_else(|| "NOT CAPTURED - Rejected claims section is missing.".to_string());
    let overall_verdict = extract_context_section(&primary.contents, "Overall Verdict")
        .unwrap_or_else(|| "NOT CAPTURED - Overall verdict section is missing.".to_string());
    let open_findings = unresolved_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Open findings section is missing.".to_string());
    let claims_under_test = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Claims Under Test"))
        .unwrap_or_else(|| "NOT CAPTURED - Claims under test section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &verified_claims,
        &rejected_claims,
        &overall_verdict,
        &open_findings,
        &claims_under_test,
    ]);
    let verdict_status = extract_labeled_context_value(&overall_verdict, "Status")
        .unwrap_or_else(|| "unknown-verdict".to_string());
    let open_findings_status = extract_labeled_context_value(&open_findings, "Status")
        .unwrap_or_else(|| "unknown-open-findings".to_string());
    let claim_count = count_context_items_without_placeholders(
        &claims_under_test,
        &["The current invariants are bounded enough for recorded verification."],
    );
    let open_finding_count = count_context_items_without_placeholders(
        &open_findings,
        &["No unresolved findings remain from the current verification target."],
    );

    let headline = if missing_context_markers == 0 {
        if open_findings_status == "unresolved-findings-open" {
            format!(
                "Verification found {open_finding_count} unresolved finding(s) and blocked release readiness."
            )
        } else {
            format!(
                "Verification packet completed with `{verdict_status}` verdict across {claim_count} claim set(s)."
            )
        }
    } else {
        format!(
            "Verification packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact records `{verdict_status}` verdict with {claim_count} claim set(s) under test and {open_finding_count} unresolved finding set(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Claim sets: {claim_count}; open findings: {open_finding_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Verification Report".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&overall_verdict, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_pr_review_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "review-summary.md")?;
    let pr_analysis_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "pr-analysis.md");

    let final_disposition = extract_context_section(&primary.contents, "Final Disposition")
        .unwrap_or_else(|| "NOT CAPTURED - Final disposition section is missing.".to_string());
    let severity = extract_context_section(&primary.contents, "Severity")
        .unwrap_or_else(|| "NOT CAPTURED - Severity section is missing.".to_string());
    let must_fix_findings = extract_context_section(&primary.contents, "Must-Fix Findings")
        .unwrap_or_else(|| "NOT CAPTURED - Must-fix findings section is missing.".to_string());
    let accepted_risks = extract_context_section(&primary.contents, "Accepted Risks")
        .unwrap_or_else(|| "NOT CAPTURED - Accepted risks section is missing.".to_string());
    let changed_modules = pr_analysis_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Changed Modules"))
        .unwrap_or_else(|| "NOT CAPTURED - Changed modules section is missing.".to_string());
    let inferred_intent = pr_analysis_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Inferred Intent"))
        .unwrap_or_else(|| "NOT CAPTURED - Inferred intent section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &final_disposition,
        &severity,
        &must_fix_findings,
        &accepted_risks,
        &changed_modules,
        &inferred_intent,
    ]);
    let disposition_status = extract_labeled_context_value(&final_disposition, "Status")
        .unwrap_or_else(|| "unknown-disposition".to_string());
    let rationale = extract_labeled_context_value(&final_disposition, "Rationale")
        .unwrap_or_else(|| truncate_context_excerpt(&final_disposition, 320));
    let overall_severity = extract_labeled_context_value(&severity, "Overall severity")
        .unwrap_or_else(|| {
            if must_fix_findings.contains("No must-fix findings remain.") {
                "review-notes".to_string()
            } else {
                "must-fix".to_string()
            }
        });
    let must_fix_count =
        extract_labeled_usize(&severity, "Must-fix findings").unwrap_or_else(|| {
            count_context_items_without_placeholders(
                &must_fix_findings,
                &["No must-fix findings remain."],
            )
        });
    let review_note_count = extract_labeled_usize(&severity, "Review notes").unwrap_or_else(|| {
        count_context_items_without_placeholders(&accepted_risks, &["No accepted risks recorded."])
    });
    let changed_surface_count = count_context_items_without_placeholders(
        &changed_modules,
        &["No changed surfaces detected."],
    );

    let headline = if missing_context_markers == 0 {
        match disposition_status.as_str() {
            "ready-with-review-notes" => format!(
                "PR review completed with {review_note_count} review note(s) and no unresolved must-fix findings."
            ),
            "awaiting-disposition" => format!(
                "PR review found {must_fix_count} must-fix finding(s) and is waiting for explicit disposition."
            ),
            "accepted-with-approval" => {
                "PR review completed with explicit approval for the remaining must-fix findings."
                    .to_string()
            }
            _ => format!("PR review completed with disposition `{disposition_status}`."),
        }
    } else {
        format!(
            "PR review packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact records `{disposition_status}` disposition with `{overall_severity}` severity across {changed_surface_count} changed surface(s), {must_fix_count} must-fix finding(s), and {review_note_count} review note(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Changed surfaces: {changed_surface_count}; must-fix findings: {must_fix_count}; review notes: {review_note_count}."
        )
    };
    let result_excerpt = if rationale.contains("NOT CAPTURED") {
        truncate_context_excerpt(&inferred_intent, 320)
    } else {
        truncate_context_excerpt(&rationale, 320)
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Review Summary".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt,
        action_chips: Vec::new(),
    })
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::artifact::{ArtifactFormat, ArtifactRecord};

    fn make_artifact(file_name: &str, contents: &str) -> PersistedArtifact {
        PersistedArtifact {
            record: ArtifactRecord {
                file_name: file_name.to_string(),
                relative_path: format!("artifacts/run-test/mode/{file_name}"),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: contents.to_string(),
        }
    }

    #[test]
    fn build_action_chips_for_includes_open_artifact_chip() {
        let chips = build_action_chips_for(
            RunState::Completed,
            &[],
            ".canon/artifacts/task-mapping.md",
            "run-123",
        );
        assert!(chips.iter().any(|c| c.id == "open-primary-artifact"));
    }

    #[test]
    fn build_action_chips_for_includes_inspect_evidence_when_awaiting_approval() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            ".canon/artifacts/task-mapping.md",
            "run-123",
        );
        assert!(chips.iter().any(|c| c.id == "inspect-evidence"));
    }

    #[test]
    fn build_action_chips_for_includes_approve_chip_for_each_target() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            ".canon/artifacts/task-mapping.md",
            "run-123",
        );
        assert!(chips.iter().any(|c| c.id == "approve-gate-execution"));
    }

    #[test]
    fn build_action_chips_for_includes_resume_chip_when_no_targets() {
        let chips =
            build_action_chips_for(RunState::AwaitingApproval, &[], ".canon/artifact.md", "run-1");
        assert!(chips.iter().any(|c| c.id == "resume-run"));
        let resume = chips.iter().find(|c| c.id == "resume-run").unwrap();
        assert!(resume.recommended);
    }

    #[test]
    fn approve_chip_text_fallback_is_human_readable() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            "",
            "run-42",
        );
        let approve = chips.iter().find(|c| c.id == "approve-gate-execution").unwrap();
        assert!(
            !approve.text_fallback.contains("gate:execution"),
            "text_fallback should not expose internal gate IDs: {}",
            approve.text_fallback
        );
        assert!(approve.text_fallback.contains("run-42"));
    }

    #[test]
    fn summarize_requirements_mode_result_returns_none_when_primary_missing() {
        let artifacts = vec![make_artifact("constraints.md", "## Constraints\n- item")];
        assert!(summarize_mode_result(Mode::Requirements, &artifacts).is_none());
    }

    #[test]
    fn summarize_requirements_mode_result_produces_summary() {
        let artifacts = vec![make_artifact(
            "problem-statement.md",
            "## Problem\nReduce auth latency.\n\n## Summary\nFix auth.",
        )];
        let summary = summarize_mode_result(Mode::Requirements, &artifacts);
        assert!(summary.is_some());
        let s = summary.unwrap();
        assert!(s.headline.contains("Requirements packet"));
        assert!(s.primary_artifact_title == "Problem Statement");
    }

    #[test]
    fn summarize_implementation_mode_result_returns_none_when_primary_missing() {
        let artifacts = vec![make_artifact("mutation-bounds.md", "## Allowed Paths\n- src/**")];
        assert!(summarize_mode_result(Mode::Implementation, &artifacts).is_none());
    }

    #[test]
    fn summarize_implementation_mode_result_headline_indicates_missing_context() {
        let artifacts = vec![make_artifact("task-mapping.md", "## Task Mapping\n- task 1")];
        let summary = summarize_mode_result(Mode::Implementation, &artifacts).unwrap();
        // mutation-bounds and validation-hooks are missing, so there should be missing markers
        assert!(summary.headline.contains("missing-context marker"));
    }

    #[test]
    fn primary_artifact_action_for_populates_correct_fields() {
        let action = primary_artifact_action_for(".canon/artifacts/task-mapping.md");
        assert_eq!(action.id, "open-primary-artifact");
        assert_eq!(action.target, ".canon/artifacts/task-mapping.md");
        assert!(action.text_fallback.contains(".canon/artifacts/task-mapping.md"));
    }

    #[test]
    fn summarize_mode_result_returns_none_for_unknown_mode() {
        // Mode::Incident and Mode::Migration are not implemented
        let artifacts = vec![make_artifact("some-file.md", "content")];
        assert!(summarize_mode_result(Mode::Incident, &artifacts).is_none());
    }
}
