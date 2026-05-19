use crate::domain::artifact::artifact_slug;
use crate::domain::run::BacklogPlanningContext;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;

use super::shared::{
    extract_authored_h2_section, extract_authored_section_or_marker, extract_marker,
    render_authored_artifact, render_string_list,
};
use super::{AuthoredSectionSpec, render_markdown};

mod backlog;

pub fn render_change_artifact(file_name: &str, brief_summary: &str, default_owner: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let system_slice = extract_marker(brief_summary, &normalized, "system slice")
        .unwrap_or("Map the bounded subsystem before change planning.".to_string());
    let change_focus = extract_marker(brief_summary, &normalized, "intended change")
        .or_else(|| extract_authored_h2_section(brief_summary, "Implementation Plan", &[]))
        .or_else(|| extract_authored_h2_section(brief_summary, "Decision Record", &[]))
        .unwrap_or(
            "Bound the intended change explicitly before implementation expands the surface area."
                .to_string(),
        );
    let owner = extract_marker(brief_summary, &normalized, "owner")
        .unwrap_or_else(|| owner_default(default_owner));
    let risk_level = extract_marker(brief_summary, &normalized, "risk level")
        .unwrap_or("unspecified-risk".to_string());
    let zone = extract_marker(brief_summary, &normalized, "zone")
        .unwrap_or("unspecified-zone".to_string());
    let summary = render_change_bundle_summary(
        file_name,
        &system_slice,
        &change_focus,
        &owner,
        &risk_level,
        &zone,
    );

    match file_name {
        "system-slice.md" => render_authored_artifact(
            "System Slice",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "System Slice", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Domain Slice", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Excluded Areas", aliases: &[] },
            ],
        ),
        "legacy-invariants.md" => render_authored_artifact(
            "Legacy Invariants",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Legacy Invariants", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Domain Invariants", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Forbidden Normalization", aliases: &[] },
            ],
        ),
        "change-surface.md" => render_authored_artifact(
            "Change Surface",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Change Surface", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ownership", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Cross-Context Risks", aliases: &[] },
            ],
        ),
        "implementation-plan.md" => render_authored_artifact(
            "Implementation Plan",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Implementation Plan", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Sequencing", aliases: &[] },
            ],
        ),
        "validation-strategy.md" => render_authored_artifact(
            "Validation Strategy",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Validation Strategy", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Independent Checks", aliases: &[] },
            ],
        ),
        "decision-record.md" => render_authored_artifact(
            "Decision Record",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Decision Record", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Drivers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Options Considered", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Tradeoffs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommendation", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Why Not The Others", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Consequences", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Unresolved Questions", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders an incident mode artifact for the given filename slug.
pub fn render_incident_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let incident_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Incident Scope",
        &[],
        &["incident scope"],
    )
    .unwrap_or_else(|| "incident scope not yet authored".to_string());
    let trigger_and_current_state = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Trigger And Current State",
        &[],
        &["trigger and current state"],
    )
    .unwrap_or_else(|| "trigger and current state not yet authored".to_string());
    let summary = format!(
        "Bounded incident packet for {}. Current state: {}.",
        truncate_context_excerpt(&incident_scope, 120),
        truncate_context_excerpt(&trigger_and_current_state, 100)
    );

    match file_name {
        "incident-frame.md" => render_authored_artifact(
            "Incident Frame",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Incident Scope", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Trigger And Current State",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Operational Constraints", aliases: &[] },
            ],
        ),
        "hypothesis-log.md" => render_authored_artifact(
            "Hypothesis Log",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Known Facts", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Working Hypotheses", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Evidence Gaps", aliases: &[] },
            ],
        ),
        "blast-radius-map.md" => render_authored_artifact(
            "Blast Radius Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Impacted Surfaces", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Propagation Paths", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence And Unknowns", aliases: &[] },
            ],
        ),
        "containment-plan.md" => render_authored_artifact(
            "Containment Plan",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Immediate Actions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ordered Sequence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Stop Conditions", aliases: &[] },
            ],
        ),
        "incident-decision-record.md" => render_authored_artifact(
            "Incident Decision Record",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Decision Points", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Approved Actions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Actions", aliases: &[] },
            ],
        ),
        "follow-up-verification.md" => render_authored_artifact(
            "Follow-Up Verification",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Verification Checks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Release Readiness", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Follow-Up Work", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders a security assessment mode artifact for the given filename slug.
pub fn render_migration_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let current_state = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Current State",
        &[],
        &["current state"],
    )
    .unwrap_or_else(|| "current state not yet authored".to_string());
    let target_state = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Target State",
        &[],
        &["target state"],
    )
    .unwrap_or_else(|| "target state not yet authored".to_string());
    let summary = format!(
        "Bounded migration packet from {} to {}.",
        truncate_context_excerpt(&current_state, 80),
        truncate_context_excerpt(&target_state, 80)
    );

    match file_name {
        "source-target-map.md" => render_authored_artifact(
            "Source-Target Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Current State", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Target State", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Transition Boundaries", aliases: &[] },
            ],
        ),
        "compatibility-matrix.md" => render_authored_artifact(
            "Compatibility Matrix",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Guaranteed Compatibility", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Temporary Incompatibilities",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Coexistence Rules", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Options Matrix", aliases: &[] },
            ],
        ),
        "sequencing-plan.md" => render_authored_artifact(
            "Sequencing Plan",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Ordered Steps", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Parallelizable Work", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Cutover Criteria", aliases: &[] },
            ],
        ),
        "fallback-plan.md" => render_authored_artifact(
            "Fallback Plan",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Rollback Triggers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Fallback Paths", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Re-Entry Criteria", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Adoption Implications", aliases: &[] },
            ],
        ),
        "migration-verification-report.md" => render_authored_artifact(
            "Migration Verification Report",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Verification Checks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Residual Risks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Release Readiness", aliases: &[] },
            ],
        ),
        "decision-record.md" => render_authored_artifact(
            "Decision Record",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Migration Decisions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Tradeoff Analysis", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommendation", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Why Not The Others", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ecosystem Health", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Decisions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Approval Notes", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders a backlog mode artifact for the given filename slug.
pub fn render_backlog_artifact(
    file_name: &str,
    brief_summary: &str,
    planning_context: &BacklogPlanningContext,
) -> String {
    backlog::render_backlog_artifact(file_name, brief_summary, planning_context)
}

/// Renders an implementation mode artifact for the given filename slug.
pub fn render_implementation_artifact(
    file_name: &str,
    brief_summary: &str,
    default_owner: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let task_mapping = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Task Mapping",
        &[],
        &["task mapping", "implementation plan"],
    );
    let _bounded_changes = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Bounded Changes",
        &[],
        &["bounded changes", "allowed paths", "mutation bounds"],
    );
    let mutation_bounds = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Mutation Bounds",
        &[],
        &["mutation bounds"],
    );
    let _allowed_paths = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Allowed Paths",
        &[],
        &["allowed paths"],
    );
    let _safety_net_evidence = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Safety-Net Evidence",
        &[],
        &["safety-net evidence", "safety net evidence"],
    );
    let _independent_checks = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Independent Checks",
        &[],
        &["independent checks", "validation strategy"],
    );
    let _rollback_triggers = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Rollback Triggers",
        &[],
        &["rollback triggers"],
    );
    let _rollback_steps = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Rollback Steps",
        &[],
        &["rollback steps"],
    );
    let _validation_evidence = extract_marker(brief_summary, &normalized, "validation evidence")
        .unwrap_or(
            "Validation evidence was recorded through the governed validation command.".to_string(),
        );
    let mutation_posture = extract_marker(brief_summary, &normalized, "mutation posture")
        .unwrap_or(
            "Recommendation-only posture remains active until a later run is explicitly allowed to mutate."
                .to_string(),
        );
    let owner = extract_marker(brief_summary, &normalized, "owner")
        .unwrap_or_else(|| owner_default(default_owner));
    let risk_level = extract_marker(brief_summary, &normalized, "risk level")
        .unwrap_or("unspecified-risk".to_string());
    let zone = extract_marker(brief_summary, &normalized, "zone")
        .unwrap_or("unspecified-zone".to_string());
    let summary = render_implementation_bundle_summary(
        file_name,
        task_mapping.as_deref().unwrap_or(
            "Capture an explicit implementation task map before bounded execution guidance can proceed.",
        ),
        mutation_bounds.as_deref().unwrap_or(
            "Declare bounded mutation scope before implementation guidance can proceed.",
        ),
        &owner,
        &risk_level,
        &zone,
    );

    match file_name {
        "task-mapping.md" => render_authored_artifact(
            "Task Mapping",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Task Mapping", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Bounded Changes", aliases: &[] },
            ],
        ),
        "mutation-bounds.md" => render_authored_artifact(
            "Mutation Bounds",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Mutation Bounds", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Allowed Paths", aliases: &[] },
            ],
        ),
        "implementation-notes.md" => render_authored_artifact(
            "Implementation Notes",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Executed Changes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Candidate Frameworks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Options Matrix", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommendation", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Task Linkage", aliases: &[] },
            ],
        ),
        "completion-evidence.md" => render_authored_artifact(
            "Completion Evidence",
            &format!("{summary}\n- Mutation posture: {}", compact_summary_line(&mutation_posture)),
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Completion Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Adoption Implications", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Remaining Risks", aliases: &[] },
            ],
        ),
        "validation-hooks.md" => render_authored_artifact(
            "Validation Hooks",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Ecosystem Health", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Safety-Net Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Independent Checks", aliases: &[] },
            ],
        ),
        "rollback-notes.md" => render_authored_artifact(
            "Rollback Notes",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Rollback Triggers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Rollback Steps", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders a refactor mode artifact for the given filename slug.
pub fn render_refactor_artifact(
    file_name: &str,
    brief_summary: &str,
    default_owner: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let preserved_behavior = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Preserved Behavior",
        &[],
        &["preserved behavior"],
    );
    let _approved_exceptions = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Approved Exceptions",
        &[],
        &["approved exceptions"],
    )
    .unwrap_or("None.".to_string());
    let refactor_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Refactor Scope",
        &[],
        &["refactor scope"],
    );
    let _allowed_paths = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Allowed Paths",
        &[],
        &["allowed paths"],
    );
    let _structural_rationale = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Structural Rationale",
        &[],
        &["structural rationale"],
    );
    let _untouched_surface = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Untouched Surface",
        &[],
        &["untouched surface"],
    );
    let _safety_net_evidence = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Safety-Net Evidence",
        &[],
        &["safety-net evidence", "safety net evidence"],
    );
    let _regression_findings = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Regression Findings",
        &[],
        &["regression findings"],
    )
    .unwrap_or("No regression findings are accepted in the bounded refactor packet.".to_string());
    let _contract_drift = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Contract Drift",
        &[],
        &["contract drift"],
    );
    let _reviewer_notes = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Reviewer Notes",
        &[],
        &["reviewer notes"],
    )
    .unwrap_or("Reviewer confirmation is required before any drift is accepted.".to_string());
    let _feature_audit = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Feature Audit",
        &[],
        &["feature audit"],
    );
    let _decision = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Decision",
        &[],
        &["decision"],
    );
    let owner = extract_marker(brief_summary, &normalized, "owner")
        .unwrap_or_else(|| owner_default(default_owner));
    let risk_level = extract_marker(brief_summary, &normalized, "risk level")
        .unwrap_or("unspecified-risk".to_string());
    let zone = extract_marker(brief_summary, &normalized, "zone")
        .unwrap_or("unspecified-zone".to_string());
    let summary = render_refactor_bundle_summary(
        file_name,
        preserved_behavior.as_deref().unwrap_or(
            "Capture the externally meaningful behavior before structural work can proceed.",
        ),
        refactor_scope
            .as_deref()
            .unwrap_or("Declare the bounded refactor scope before structural work can proceed."),
        &owner,
        &risk_level,
        &zone,
    );

    match file_name {
        "preserved-behavior.md" => render_authored_artifact(
            "Preserved Behavior",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Preserved Behavior", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Approved Exceptions", aliases: &[] },
            ],
        ),
        "refactor-scope.md" => render_authored_artifact(
            "Refactor Scope",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Refactor Scope", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Allowed Paths", aliases: &[] },
            ],
        ),
        "structural-rationale.md" => render_authored_artifact(
            "Structural Rationale",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Structural Rationale", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Untouched Surface", aliases: &[] },
            ],
        ),
        "regression-evidence.md" => render_authored_artifact(
            "Regression Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Safety-Net Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Regression Findings", aliases: &[] },
            ],
        ),
        "contract-drift-check.md" => render_authored_artifact(
            "Contract Drift Check",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Contract Drift", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Reviewer Notes", aliases: &[] },
            ],
        ),
        "no-feature-addition.md" => render_authored_artifact(
            "No Feature Addition",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Feature Audit", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

fn render_change_bundle_summary(
    current_file: &str,
    system_slice: &str,
    intended_change: &str,
    owner: &str,
    risk_level: &str,
    zone: &str,
) -> String {
    let detail_links = [
        "system-slice.md",
        "legacy-invariants.md",
        "change-surface.md",
        "implementation-plan.md",
        "validation-strategy.md",
        "decision-record.md",
    ]
    .into_iter()
    .filter(|file_name| *file_name != current_file)
    .map(|file_name| format!("[{file_name}]({file_name})"))
    .collect::<Vec<_>>()
    .join(", ");

    format!(
        "- Bounded slice: `{}`\n- Intended change: {}\n- Owner / risk / zone: `{}` / `{}` / `{}`\n- Details: {}",
        compact_summary_line(system_slice),
        compact_summary_line(intended_change),
        compact_summary_line(owner),
        compact_summary_line(risk_level),
        compact_summary_line(zone),
        detail_links,
    )
}

fn render_implementation_bundle_summary(
    current_file: &str,
    task_mapping: &str,
    mutation_bounds: &str,
    owner: &str,
    risk_level: &str,
    zone: &str,
) -> String {
    let detail_links = [
        "task-mapping.md",
        "mutation-bounds.md",
        "implementation-notes.md",
        "completion-evidence.md",
        "validation-hooks.md",
        "rollback-notes.md",
    ]
    .into_iter()
    .filter(|file_name| *file_name != current_file)
    .map(|file_name| format!("[{file_name}]({file_name})"))
    .collect::<Vec<_>>()
    .join(", ");

    format!(
        "- Task scope: {}\n- Mutation bounds: `{}`\n- Owner / risk / zone: `{}` / `{}` / `{}`\n- Details: {}",
        compact_summary_line(task_mapping),
        compact_summary_line(mutation_bounds),
        compact_summary_line(owner),
        compact_summary_line(risk_level),
        compact_summary_line(zone),
        detail_links,
    )
}

fn render_refactor_bundle_summary(
    current_file: &str,
    preserved_behavior: &str,
    refactor_scope: &str,
    owner: &str,
    risk_level: &str,
    zone: &str,
) -> String {
    let detail_links = [
        "preserved-behavior.md",
        "refactor-scope.md",
        "structural-rationale.md",
        "regression-evidence.md",
        "contract-drift-check.md",
        "no-feature-addition.md",
    ]
    .into_iter()
    .filter(|file_name| *file_name != current_file)
    .map(|file_name| format!("[{file_name}]({file_name})"))
    .collect::<Vec<_>>()
    .join(", ");

    format!(
        "- Preserved behavior: {}\n- Refactor scope: `{}`\n- Owner / risk / zone: `{}` / `{}` / `{}`\n- Details: {}",
        compact_summary_line(preserved_behavior),
        compact_summary_line(refactor_scope),
        compact_summary_line(owner),
        compact_summary_line(risk_level),
        compact_summary_line(zone),
        detail_links,
    )
}

fn compact_summary_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn owner_default(default_owner: &str) -> String {
    let trimmed = default_owner.trim();
    if trimmed.is_empty() { "bounded-system-maintainer".to_string() } else { trimmed.to_string() }
}
