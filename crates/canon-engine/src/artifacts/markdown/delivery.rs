use crate::domain::artifact::artifact_slug;
use crate::domain::run::BacklogPlanningContext;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;

use super::shared::{
    extract_authored_h2_section, extract_authored_section_or_marker, extract_marker,
    render_authored_artifact, render_string_list,
};
use super::{AuthoredSectionSpec, render_markdown};

mod backlog;

const MARKER_OWNER: &str = "owner";
const MARKER_RISK: &str = "risk level";
const MARKER_ZONE: &str = "zone";
const DEFAULT_RISK: &str = "unspecified-risk";
const DEFAULT_ZONE: &str = "unspecified-zone";
const DEFAULT_OWNER: &str = "bounded-system-maintainer";

macro_rules! artifact_match {
    ($file_name:expr, $default_summary:expr, $brief_summary:expr, {
        $(
            $slug:expr => $title:expr, [ $($heading:expr),* $(,)? ] $(=> $custom_summary:expr)?
        ),* $(,)?
    }) => {
        match $file_name {
            $(
                $slug => {
                    let mut _summary_to_use = $default_summary;
                    $(
                        _summary_to_use = $custom_summary;
                    )?
                    render_authored_artifact(
                        $title,
                        _summary_to_use,
                        $brief_summary,
                        &[
                            $(
                                AuthoredSectionSpec { canonical_heading: $heading, aliases: &[] },
                            )*
                        ]
                    )
                }
            )*
            other => render_markdown(other, $brief_summary),
        }
    };
}

fn extract_required_section(
    brief_summary: &str,
    normalized: &str,
    heading: &str,
    fallback: &str,
) -> String {
    let marker = heading.to_lowercase();
    extract_authored_section_or_marker(brief_summary, normalized, heading, &[], &[&marker])
        .unwrap_or_else(|| fallback.to_string())
}

fn extract_optional_section(
    brief_summary: &str,
    normalized: &str,
    heading: &str,
) -> Option<String> {
    let marker = heading.to_lowercase();
    extract_authored_section_or_marker(brief_summary, normalized, heading, &[], &[&marker])
}

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
    let DeliveryMarkers { owner, risk_level, zone } =
        extract_delivery_markers(brief_summary, &normalized, default_owner);
    let summary = render_change_bundle_summary(
        file_name,
        &system_slice,
        &change_focus,
        &owner,
        &risk_level,
        &zone,
    );

    artifact_match!(file_name, &summary, brief_summary, {
        "system-slice.md" => "System Slice", ["System Slice", "Domain Slice", "Excluded Areas"],
        "legacy-invariants.md" => "Legacy Invariants", ["Legacy Invariants", "Domain Invariants", "Forbidden Normalization"],
        "change-surface.md" => "Change Surface", ["Change Surface", "Ownership", "Cross-Context Risks"],
        "implementation-plan.md" => "Implementation Plan", ["Implementation Plan", "Sequencing"],
        "validation-strategy.md" => "Validation Strategy", ["Validation Strategy", "Independent Checks"],
        "decision-record.md" => "Decision Record", ["Decision Record", "Decision Drivers", "Options Considered", "Decision Evidence", "Boundary Tradeoffs", "Recommendation", "Why Not The Others", "Consequences", "Unresolved Questions"],
    })
}

/// Renders an incident mode artifact for the given filename slug.
pub fn render_incident_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let incident_scope = extract_required_section(
        brief_summary,
        &normalized,
        "Incident Scope",
        "incident scope not yet authored",
    );
    let trigger_and_current_state = extract_required_section(
        brief_summary,
        &normalized,
        "Trigger And Current State",
        "trigger and current state not yet authored",
    );
    let summary = format!(
        "Bounded incident packet for {}. Current state: {}.",
        truncate_context_excerpt(&incident_scope, 120),
        truncate_context_excerpt(&trigger_and_current_state, 100)
    );

    artifact_match!(file_name, &summary, brief_summary, {
        "incident-frame.md" => "Incident Frame", ["Incident Scope", "Trigger And Current State", "Operational Constraints"],
        "hypothesis-log.md" => "Hypothesis Log", ["Known Facts", "Working Hypotheses", "Evidence Gaps"],
        "blast-radius-map.md" => "Blast Radius Map", ["Impacted Surfaces", "Propagation Paths", "Confidence And Unknowns"],
        "containment-plan.md" => "Containment Plan", ["Immediate Actions", "Ordered Sequence", "Stop Conditions"],
        "incident-decision-record.md" => "Incident Decision Record", ["Decision Points", "Approved Actions", "Deferred Actions"],
        "follow-up-verification.md" => "Follow-Up Verification", ["Verification Checks", "Release Readiness", "Follow-Up Work"],
    })
}

/// Renders a security assessment mode artifact for the given filename slug.
pub fn render_migration_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let current_state = extract_required_section(
        brief_summary,
        &normalized,
        "Current State",
        "current state not yet authored",
    );
    let target_state = extract_required_section(
        brief_summary,
        &normalized,
        "Target State",
        "target state not yet authored",
    );
    let summary = format!(
        "Bounded migration packet from {} to {}.",
        truncate_context_excerpt(&current_state, 80),
        truncate_context_excerpt(&target_state, 80)
    );

    artifact_match!(file_name, &summary, brief_summary, {
        "source-target-map.md" => "Source-Target Map", ["Current State", "Target State", "Transition Boundaries"],
        "compatibility-matrix.md" => "Compatibility Matrix", ["Guaranteed Compatibility", "Temporary Incompatibilities", "Coexistence Rules", "Options Matrix"],
        "sequencing-plan.md" => "Sequencing Plan", ["Ordered Steps", "Parallelizable Work", "Cutover Criteria"],
        "fallback-plan.md" => "Fallback Plan", ["Rollback Triggers", "Fallback Paths", "Re-Entry Criteria", "Adoption Implications"],
        "migration-verification-report.md" => "Migration Verification Report", ["Verification Checks", "Residual Risks", "Release Readiness"],
        "decision-record.md" => "Decision Record", ["Migration Decisions", "Tradeoff Analysis", "Decision Evidence", "Recommendation", "Why Not The Others", "Ecosystem Health", "Deferred Decisions", "Approval Notes"],
    })
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
    let task_mapping = extract_optional_section(brief_summary, &normalized, "Task Mapping");
    let mutation_bounds = extract_optional_section(brief_summary, &normalized, "Mutation Bounds");
    let mutation_posture = extract_marker(brief_summary, &normalized, "mutation posture")
        .unwrap_or(
            "Recommendation-only posture remains active until a later run is explicitly allowed to mutate."
                .to_string(),
        );
    let DeliveryMarkers { owner, risk_level, zone } =
        extract_delivery_markers(brief_summary, &normalized, default_owner);
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

    let completion_summary =
        format!("{}\n- Mutation posture: {}", summary, compact_summary_line(&mutation_posture));

    artifact_match!(file_name, &summary, brief_summary, {
        "task-mapping.md" => "Task Mapping", ["Task Mapping", "Bounded Changes"],
        "mutation-bounds.md" => "Mutation Bounds", ["Mutation Bounds", "Allowed Paths"],
        "implementation-notes.md" => "Implementation Notes", ["Executed Changes", "Candidate Frameworks", "Options Matrix", "Decision Evidence", "Recommendation", "Task Linkage"],
        "completion-evidence.md" => "Completion Evidence", ["Completion Evidence", "Adoption Implications", "Remaining Risks"] => &completion_summary,
        "validation-hooks.md" => "Validation Hooks", ["Ecosystem Health", "Safety-Net Evidence", "Independent Checks"],
        "rollback-notes.md" => "Rollback Notes", ["Rollback Triggers", "Rollback Steps"],
    })
}

/// Renders a refactor mode artifact for the given filename slug.
pub fn render_refactor_artifact(
    file_name: &str,
    brief_summary: &str,
    default_owner: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let preserved_behavior =
        extract_optional_section(brief_summary, &normalized, "Preserved Behavior");
    let refactor_scope = extract_optional_section(brief_summary, &normalized, "Refactor Scope");
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

    artifact_match!(file_name, &summary, brief_summary, {
        "preserved-behavior.md" => "Preserved Behavior", ["Preserved Behavior", "Approved Exceptions"],
        "refactor-scope.md" => "Refactor Scope", ["Refactor Scope", "Allowed Paths"],
        "structural-rationale.md" => "Structural Rationale", ["Structural Rationale", "Untouched Surface"],
        "regression-evidence.md" => "Regression Evidence", ["Safety-Net Evidence", "Regression Findings"],
        "contract-drift-check.md" => "Contract Drift Check", ["Contract Drift", "Reviewer Notes"],
        "no-feature-addition.md" => "No Feature Addition", ["Feature Audit", "Decision"],
    })
}

/// Renders a debugging mode artifact for the given filename slug.
pub fn render_debugging_artifact(
    file_name: &str,
    brief_summary: &str,
    default_owner: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let defect_description = extract_required_section(
        brief_summary,
        &normalized,
        "Defect Description",
        "Capture the defect description before debugging can proceed.",
    );

    let DeliveryMarkers { owner, risk_level, zone } =
        extract_delivery_markers(brief_summary, &normalized, default_owner);

    let summary =
        render_debugging_bundle_summary(file_name, &defect_description, &owner, &risk_level, &zone);

    artifact_match!(file_name, &summary, brief_summary, {
        "context-map.md" => "Context Map", ["Context Map", "Defect Description", "Stakeholder Impact"],
        "reproduction-harness.md" => "Reproduction Harness", ["Reproduction Harness", "Red State Verification"],
        "root-cause-isolation.md" => "Root Cause Isolation", ["Root Cause Isolation", "Fault Chain", "Isolation Proof"],
        "fix-application.md" => "Fix Application", ["Fix Application", "Bounded Changes", "Invariant Preservation"],
        "verification-summary.md" => "Verification Summary", ["Verification Summary", "Green State", "No Regression Evidence"],
    })
}

fn render_change_bundle_summary(
    current_file: &str,
    system_slice: &str,
    intended_change: &str,
    owner: &str,
    risk_level: &str,
    zone: &str,
) -> String {
    let detail_links = render_detail_links(
        current_file,
        &[
            "system-slice.md",
            "legacy-invariants.md",
            "change-surface.md",
            "implementation-plan.md",
            "validation-strategy.md",
            "decision-record.md",
        ],
    );

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
    let detail_links = render_detail_links(
        current_file,
        &[
            "task-mapping.md",
            "mutation-bounds.md",
            "implementation-notes.md",
            "completion-evidence.md",
            "validation-hooks.md",
            "rollback-notes.md",
        ],
    );

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
    let detail_links = render_detail_links(
        current_file,
        &[
            "preserved-behavior.md",
            "refactor-scope.md",
            "structural-rationale.md",
            "regression-evidence.md",
            "contract-drift-check.md",
            "no-feature-addition.md",
        ],
    );

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

fn render_debugging_bundle_summary(
    current_file: &str,
    defect_description: &str,
    owner: &str,
    risk_level: &str,
    zone: &str,
) -> String {
    let detail_links = render_detail_links(
        current_file,
        &[
            "context-map.md",
            "reproduction-harness.md",
            "root-cause-isolation.md",
            "fix-application.md",
            "verification-summary.md",
        ],
    );

    format!(
        "- Defect: {}\n- Owner / risk / zone: `{}` / `{}` / `{}`\n- Details: {}",
        compact_summary_line(defect_description),
        compact_summary_line(owner),
        compact_summary_line(risk_level),
        compact_summary_line(zone),
        detail_links,
    )
}

fn compact_summary_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn render_detail_links(current_file: &str, files: &[&str]) -> String {
    files
        .iter()
        .filter(|file_name| **file_name != current_file)
        .map(|file_name| format!("[{file_name}]({file_name})"))
        .collect::<Vec<_>>()
        .join(", ")
}

struct DeliveryMarkers {
    owner: String,
    risk_level: String,
    zone: String,
}

fn extract_delivery_markers(
    brief_summary: &str,
    normalized: &str,
    default_owner: &str,
) -> DeliveryMarkers {
    let owner = extract_marker(brief_summary, normalized, MARKER_OWNER)
        .unwrap_or_else(|| owner_default(default_owner));
    let risk_level = extract_marker(brief_summary, normalized, MARKER_RISK)
        .unwrap_or_else(|| DEFAULT_RISK.to_string());
    let zone = extract_marker(brief_summary, normalized, MARKER_ZONE)
        .unwrap_or_else(|| DEFAULT_ZONE.to_string());

    DeliveryMarkers { owner, risk_level, zone }
}

fn owner_default(default_owner: &str) -> String {
    let trimmed = default_owner.trim();
    if trimmed.is_empty() { DEFAULT_OWNER.to_string() } else { trimmed.to_string() }
}
