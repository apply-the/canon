use crate::orchestrator::service::ModeResultSummary;
use crate::orchestrator::service::context_parse::{
    count_markdown_entries, count_missing_context_markers, extract_context_section,
    extract_result_section, truncate_context_excerpt,
};
use crate::persistence::store::PersistedArtifact;

use super::{
    packet_output_quality_artifact_prefix, packet_output_quality_headline, packet_primary_artifact,
    primary_artifact_action_for,
};

pub(super) fn summarize_change_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary_artifact = packet_primary_artifact(artifacts, "system-slice.md")?;
    let summary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "change-surface.md")?;
    let legacy_invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "legacy-invariants.md");
    let validation_strategy_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "validation-strategy.md");
    let system_slice_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "system-slice.md");

    let (change_surface, change_surface_missing) = extract_result_section(
        &summary_artifact.contents,
        "Change Surface",
        "Missing Authored Body",
        "NOT CAPTURED - Change surface section is missing.",
    );
    let (legacy_invariants, legacy_missing) = legacy_invariants_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Legacy Invariants",
                "Missing Authored Body",
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
                "Missing Authored Body",
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
                "Missing Authored Body",
                "NOT CAPTURED - System slice section is missing.",
            )
        })
        .unwrap_or_else(|| ("NOT CAPTURED - System slice artifact is missing.".to_string(), true));
    let (domain_slice, domain_slice_missing) = system_slice_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Domain Slice",
                "Missing Authored Body",
                "NOT CAPTURED - Domain slice section is missing.",
            )
        })
        .unwrap_or_else(|| ("NOT CAPTURED - Domain slice section is missing.".to_string(), true));
    let (domain_invariants, domain_invariants_missing) = legacy_invariants_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Domain Invariants",
                "Missing Authored Body",
                "NOT CAPTURED - Domain invariants section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Domain invariants section is missing.".to_string(), true)
        });
    let (cross_context_risks, cross_context_risks_missing) = Some(summary_artifact)
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Cross-Context Risks",
                "Missing Authored Body",
                "NOT CAPTURED - Cross-context risks section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Cross-context risks section is missing.".to_string(), true)
        });

    let missing_context_markers = [
        change_surface_missing,
        legacy_missing,
        validation_missing,
        system_slice_missing,
        domain_slice_missing,
        domain_invariants_missing,
        cross_context_risks_missing,
    ]
    .into_iter()
    .filter(|missing| *missing)
    .count();
    let change_surface_count = count_markdown_entries(&change_surface);
    let legacy_invariant_count = count_markdown_entries(&legacy_invariants);
    let domain_invariant_count = count_markdown_entries(&domain_invariants);
    let cross_context_risk_count = count_markdown_entries(&cross_context_risks);
    let validation_count = count_markdown_entries(&validation_strategy);

    let headline = packet_output_quality_headline(
        "Change",
        missing_context_markers,
        0,
        "",
        "bounded change review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet names {change_surface_count} change-surface point(s). Packet also captures {legacy_invariant_count} legacy invariant(s), {domain_invariant_count} domain invariant set(s), {cross_context_risk_count} cross-context risk set(s), and {validation_count} validation check set(s) for the bounded slice {} / {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&system_slice, 90),
            truncate_context_excerpt(&domain_slice, 90)
        )
    } else {
        format!(
            "{} Change surface: {change_surface_count}; legacy invariants: {legacy_invariant_count}; domain invariants: {domain_invariant_count}; cross-context risks: {cross_context_risk_count}; validation checks: {validation_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "System Slice".to_string(),
        primary_artifact_path: format!(".canon/{}", primary_artifact.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary_artifact.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&change_surface, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_backlog_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "backlog-overview.md")?;
    let risks_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "planning-risks.md");
    let slices_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "delivery-slices.md");

    let delivery_intent = extract_context_section(&primary.contents, "Delivery Intent")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Delivery intent summary is missing.".to_string());
    let posture = extract_context_section(&primary.contents, "Decomposition Posture")
        .unwrap_or_else(|| "NOT CAPTURED - Decomposition posture is missing.".to_string());
    let planning_horizon = extract_context_section(&primary.contents, "Planning Horizon")
        .unwrap_or_else(|| "NOT CAPTURED - Planning horizon is missing.".to_string());
    let closure_findings = risks_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Closure Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Closure findings are missing.".to_string());
    let slice_count = slices_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Delivery Slices"))
        .map(|section| count_markdown_entries(&section))
        .unwrap_or(0);

    let full_packet = artifacts.iter().any(|artifact| artifact.record.slug() == "epic-tree.md");
    let missing_context_markers = count_missing_context_markers([
        &delivery_intent,
        &posture,
        &planning_horizon,
        &closure_findings,
    ]);

    let headline = if full_packet {
        packet_output_quality_headline(
            "Backlog",
            missing_context_markers,
            0,
            "",
            "downstream execution planning",
        )
    } else {
        "Backlog packet is structurally complete only and remains closure-limited with planning risks explicit.".to_string()
    };
    let artifact_packet_summary = if full_packet {
        format!(
            "{} Packet stays planning-level and records {slice_count} delivery slice set(s). Planning horizon: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&planning_horizon, 120)
        )
    } else {
        format!(
            "Primary artifact is structurally complete only and decomposition stayed risk-only. Closure findings: {}.",
            truncate_context_excerpt(&closure_findings, 140)
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Backlog Overview".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&delivery_intent, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_implementation_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| artifact.record.slug() == "task-mapping.md")?;
    let mutation_bounds_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "mutation-bounds.md");
    let validation_hooks_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "validation-hooks.md");

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

    let headline = packet_output_quality_headline(
        "Implementation",
        missing_context_markers,
        0,
        "",
        "bounded execution review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet maps {task_count} task set(s) across {allowed_path_count} allowed path set(s) with {safety_net_count} safety-net evidence set(s). Bounded changes: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&bounded_changes, 90)
        )
    } else {
        format!(
            "{} Tasks: {task_count}; allowed paths: {allowed_path_count}; safety-net evidence: {safety_net_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
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

pub(super) fn summarize_refactor_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "preserved-behavior.md")?;
    let scope_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "refactor-scope.md");
    let contract_drift_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "contract-drift-check.md");
    let no_feature_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "no-feature-addition.md");

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

    let headline = packet_output_quality_headline(
        "Refactor",
        missing_context_markers,
        0,
        "",
        "preservation review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet names {preserved_count} preserved-behavior set(s) across {allowed_path_count} allowed path set(s). Contract drift note: {}. Feature audit sets: {feature_audit_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&contract_drift, 90)
        )
    } else {
        format!(
            "{} Preserved behavior: {preserved_count}; allowed paths: {allowed_path_count}; feature audit: {feature_audit_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
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

pub(super) fn summarize_migration_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "source-target-map.md")?;
    let compatibility_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "compatibility-matrix.md");
    let fallback_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "fallback-plan.md");

    let current_state = extract_context_section(&primary.contents, "Current State")
        .unwrap_or_else(|| "NOT CAPTURED - Current state section is missing.".to_string());
    let target_state = extract_context_section(&primary.contents, "Target State")
        .unwrap_or_else(|| "NOT CAPTURED - Target state section is missing.".to_string());
    let transition_boundaries = extract_context_section(&primary.contents, "Transition Boundaries")
        .unwrap_or_else(|| "NOT CAPTURED - Transition boundaries section is missing.".to_string());
    let guaranteed_compatibility = compatibility_artifact
        .and_then(|artifact| {
            extract_context_section(&artifact.contents, "Guaranteed Compatibility")
        })
        .unwrap_or_else(|| {
            "NOT CAPTURED - Guaranteed compatibility section is missing.".to_string()
        });
    let rollback_triggers = fallback_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Rollback Triggers"))
        .unwrap_or_else(|| "NOT CAPTURED - Rollback triggers section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &current_state,
        &target_state,
        &transition_boundaries,
        &guaranteed_compatibility,
        &rollback_triggers,
    ]);
    let compatibility_count = count_markdown_entries(&guaranteed_compatibility);
    let rollback_trigger_count = count_markdown_entries(&rollback_triggers);

    let headline = packet_output_quality_headline(
        "Migration",
        missing_context_markers,
        0,
        "",
        "governed transition review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet bounds the transition from {} to {} with {compatibility_count} compatibility guarantee set(s) and {rollback_trigger_count} rollback trigger set(s).",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&current_state, 90),
            truncate_context_excerpt(&target_state, 90)
        )
    } else {
        format!(
            "{} Compatibility guarantees: {compatibility_count}; rollback triggers: {rollback_trigger_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Source-Target Map".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&transition_boundaries, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_debugging_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| artifact.record.slug() == "context-map.md")?;

    let defect_description = extract_context_section(&primary.contents, "Defect Description")
        .unwrap_or_else(|| "NOT CAPTURED - Defect description is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([&defect_description]);

    let headline = packet_output_quality_headline(
        "Debugging",
        missing_context_markers,
        0,
        "",
        "bounded debugging execution",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet captures defect: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&defect_description, 120)
        )
    } else {
        format!(
            "{} Defect description is missing.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Context Map".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&defect_description, 320),
        action_chips: Vec::new(),
    })
}

#[cfg(test)]
mod debugging_summarizer_tests {
    use super::*;
    use crate::domain::artifact::ArtifactRecord;
    use crate::persistence::store::PersistedArtifact;

    #[test]
    fn test_summarize_debugging_mode_result_missing_description() {
        let artifact = PersistedArtifact {
            record: ArtifactRecord {
                file_name: "01-context-map.md".to_string(),
                relative_path: "debugging/01-context-map.md".to_string(),
                format: crate::domain::artifact::ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "# Context Map\nNo description here.".to_string(),
        };
        let summary = summarize_debugging_mode_result(&[artifact]).unwrap();
        assert!(summary.result_excerpt.contains("NOT CAPTURED"));
        assert!(summary.artifact_packet_summary.contains("Defect description is missing"));
    }
}
