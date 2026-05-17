use crate::domain::artifact::artifact_slug;
use crate::domain::run::BacklogPlanningContext;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;

use super::shared::{
    extract_authored_h2_section, extract_authored_section_or_marker, extract_marker,
    render_authored_artifact, render_string_list,
};
use super::{AuthoredSectionSpec, render_markdown};

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
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let delivery_intent = extract_marker(brief_summary, &normalized, "delivery intent")
        .unwrap_or_else(|| planning_context.delivery_intent.clone());
    let planning_horizon = extract_marker(brief_summary, &normalized, "planning horizon")
        .or_else(|| planning_context.planning_horizon.clone())
        .unwrap_or_else(|| "No explicit planning horizon was authored.".to_string());
    let source_refs = extract_marker(brief_summary, &normalized, "source references")
        .or_else(|| extract_marker(brief_summary, &normalized, "source inputs"))
        .unwrap_or_else(|| {
            render_string_list(
                &planning_context.source_refs,
                "- No explicit source references were recorded.",
            )
        });
    let priorities =
        extract_marker(brief_summary, &normalized, "priorities").unwrap_or_else(|| {
            render_string_list(
                &planning_context.priority_inputs,
                "- No explicit planning priorities were recorded.",
            )
        });
    let constraints =
        extract_marker(brief_summary, &normalized, "constraints").unwrap_or_else(|| {
            render_string_list(
                &planning_context.constraints,
                "- No explicit planning constraints were recorded.",
            )
        });
    let out_of_scope =
        extract_marker(brief_summary, &normalized, "out of scope").unwrap_or_else(|| {
            render_string_list(
                &planning_context.out_of_scope,
                "- No explicit exclusions were recorded.",
            )
        });
    let closure_findings = extract_marker(brief_summary, &normalized, "closure findings")
        .unwrap_or_else(|| {
            if planning_context.closure_assessment.findings.is_empty() {
                "- No closure findings remain open.".to_string()
            } else {
                planning_context
                    .closure_assessment
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
        });
    let generated_framing = extract_marker(brief_summary, &normalized, "generated framing")
        .unwrap_or_else(|| "Generated backlog framing was not recorded.".to_string());
    let critique_evidence = extract_marker(brief_summary, &normalized, "critique evidence")
        .unwrap_or_else(|| "Critique evidence was not recorded.".to_string());
    let validation_evidence = extract_marker(brief_summary, &normalized, "validation evidence")
        .unwrap_or_else(|| "Validation evidence was not recorded.".to_string());
    let decomposition_posture = planning_context.closure_assessment.decomposition_scope.as_str();

    // Authored bodies. When the agent (or the user) supplies these sections in
    // the backlog input, we render them verbatim and skip the templated
    // fallback. This is what turns backlog mode into a real backlog instead of
    // a boilerplate echo of the brief.
    let authored_epic_tree = extract_marker(brief_summary, &normalized, "epic tree");
    let authored_capability_map =
        extract_marker(brief_summary, &normalized, "capability to epic map")
            .or_else(|| extract_marker(brief_summary, &normalized, "capability map"))
            .or_else(|| extract_marker(brief_summary, &normalized, "capability mapping"));
    let authored_dependency_map = extract_marker(brief_summary, &normalized, "dependency map")
        .or_else(|| extract_marker(brief_summary, &normalized, "dependencies"));
    let authored_delivery_slices = extract_marker(brief_summary, &normalized, "delivery slices")
        .or_else(|| extract_marker(brief_summary, &normalized, "slices"));
    let authored_sequencing = extract_marker(brief_summary, &normalized, "sequencing plan")
        .or_else(|| extract_marker(brief_summary, &normalized, "sequencing"));
    let authored_acceptance = extract_marker(brief_summary, &normalized, "acceptance anchors")
        .or_else(|| extract_marker(brief_summary, &normalized, "acceptance criteria"));
    let authored_planning_risks = extract_marker(brief_summary, &normalized, "planning risks")
        .or_else(|| extract_marker(brief_summary, &normalized, "risks"));

    match file_name {
        "backlog-overview.md" => render_backlog_overview_artifact(
            &delivery_intent,
            &generated_framing,
            &planning_horizon,
            &source_refs,
            decomposition_posture,
        ),
        "epic-tree.md" => render_epic_tree_artifact(
            &delivery_intent,
            authored_epic_tree.as_deref(),
            planning_context.desired_granularity.as_str(),
            &out_of_scope,
            &source_refs,
        ),
        "capability-to-epic-map.md" => render_capability_to_epic_map_artifact(
            &delivery_intent,
            authored_capability_map.as_deref(),
            &source_refs,
            &closure_findings,
        ),
        "dependency-map.md" => render_dependency_map_artifact(
            &delivery_intent,
            authored_dependency_map.as_deref(),
            &constraints,
            &closure_findings,
        ),
        "delivery-slices.md" => render_delivery_slices_artifact(
            &delivery_intent,
            authored_delivery_slices.as_deref(),
            &out_of_scope,
            &source_refs,
        ),
        "sequencing-plan.md" => render_sequencing_plan_artifact(
            &delivery_intent,
            authored_sequencing.as_deref(),
            &priorities,
        ),
        "acceptance-anchors.md" => render_acceptance_anchors_artifact(
            &delivery_intent,
            authored_acceptance.as_deref(),
            &source_refs,
        ),
        "planning-risks.md" => render_planning_risks_artifact(
            &delivery_intent,
            authored_planning_risks.as_deref(),
            &closure_findings,
            &critique_evidence,
            &validation_evidence,
        ),
        other => render_markdown(other, brief_summary),
    }
}

fn render_backlog_overview_artifact(
    delivery_intent: &str,
    generated_framing: &str,
    planning_horizon: &str,
    source_refs: &str,
    decomposition_posture: &str,
) -> String {
    format!(
        "# Backlog Overview\n\n## Summary\n\n{}\n\n## Scope\n\n{}\n\n## Planning Horizon\n\n{}\n\n## Source Inputs\n\n{}\n\n## Delivery Intent\n\n{}\n\n## Decomposition Posture\n\n{}\n",
        delivery_intent,
        truncate_context_excerpt(generated_framing, 260),
        planning_horizon,
        source_refs,
        delivery_intent,
        decomposition_posture,
    )
}

fn render_epic_tree_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    desired_granularity: &str,
    out_of_scope: &str,
    source_refs: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Epic Tree\n\n## Summary\n\n{}\n\n## Epic Tree\n\n{}\n\n## Scope Boundaries\n\n- Preserve planning-only granularity at {}.\n- Keep excluded work explicit: {}\n\n## Source Trace Links\n\n{}\n",
            delivery_intent,
            body,
            desired_granularity,
            truncate_context_excerpt(out_of_scope, 200),
            source_refs,
        ),
        None => format!(
            "# Epic Tree\n\n## Summary\n\n{}\n\n## Epic Tree\n\nNo authored epic tree was recorded for this packet.\n\n## Scope Boundaries\n\n- Preserve planning-only granularity at {}.\n- Keep excluded work explicit: {}\n\n## Source Trace Links\n\n{}\n\n## Missing Authored Body\n\nNo `## Epic Tree` section was authored in the backlog input. Canon did not synthesize placeholder epics because that would look like approved decomposition. Author a real epic tree in the backlog input and rerun.\n",
            delivery_intent,
            desired_granularity,
            truncate_context_excerpt(out_of_scope, 200),
            source_refs,
        ),
    }
}

fn render_capability_to_epic_map_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    source_refs: &str,
    closure_findings: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Capability To Epic Map\n\n## Summary\n\n{}\n\n## Capability Mapping\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Planning Gaps\n\n{}\n",
            delivery_intent, body, source_refs, closure_findings,
        ),
        None => format!(
            "# Capability To Epic Map\n\n## Summary\n\n{}\n\n## Capability Mapping\n\nNo authored capability-to-epic map was recorded for this packet.\n\n## Source Trace Links\n\n{}\n\n## Planning Gaps\n\n{}\n\n## Missing Authored Body\n\nNo `## Capability To Epic Map` section was authored in the backlog input. Canon did not infer capability-to-epic mappings from priorities alone because that would look like approved reasoning.\n",
            delivery_intent, source_refs, closure_findings,
        ),
    }
}

fn render_dependency_map_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    constraints: &str,
    closure_findings: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Dependency Map\n\n## Summary\n\n{}\n\n## Dependencies\n\n{}\n\n## Blocking Edges\n\n{}\n\n## External Dependencies\n\n- Any external blockers must remain visible in planning risks before downstream implementation work starts.\n",
            delivery_intent, body, closure_findings,
        ),
        None => format!(
            "# Dependency Map\n\n## Summary\n\n{}\n\n## Dependencies\n\nNo authored dependency map was recorded for this packet. Shared planning constraints remain: {}\n\n## Blocking Edges\n\n{}\n\n## External Dependencies\n\n- Source references remain the upstream dependency basis.\n- Any external blockers must remain visible in planning risks before downstream implementation work starts.\n\n## Missing Authored Body\n\nNo `## Dependency Map` section was authored in the backlog input. Canon kept only explicit constraints and closure findings instead of inventing dependency edges.\n",
            delivery_intent,
            truncate_context_excerpt(constraints, 220),
            closure_findings,
        ),
    }
}

fn render_delivery_slices_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    out_of_scope: &str,
    source_refs: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Delivery Slices\n\n## Summary\n\n{}\n\n## Delivery Slices\n\n{}\n\n## Slice Boundaries\n\n- Slices stay above task level and stop at implementation-ready decomposition.\n- Excluded work stays explicit: {}\n\n## Dependency Links\n\n{}\n",
            delivery_intent,
            body,
            truncate_context_excerpt(out_of_scope, 220),
            source_refs,
        ),
        None => format!(
            "# Delivery Slices\n\n## Summary\n\n{}\n\n## Delivery Slices\n\nNo authored delivery slices were recorded for this packet.\n\n## Slice Boundaries\n\n- Slices stay above task level and stop at implementation-ready decomposition.\n- Excluded work stays explicit: {}\n\n## Dependency Links\n\n{}\n\n## Missing Authored Body\n\nNo `## Delivery Slices` section was authored in the backlog input. Canon did not synthesize example slices because that would look like approved decomposition.\n",
            delivery_intent,
            truncate_context_excerpt(out_of_scope, 220),
            source_refs,
        ),
    }
}

fn render_sequencing_plan_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    priorities: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Sequencing Plan\n\n## Summary\n\n{}\n\n## Sequencing\n\n{}\n\n## Ordering Rationale\n\n{}\n\n## Readiness Signals\n\n- A downstream implementation reader can identify one bounded slice, its dependencies, and its acceptance anchor without hidden context.\n- Closure findings remain explicit if they still weaken confidence.\n",
            delivery_intent,
            body,
            truncate_context_excerpt(priorities, 220),
        ),
        None => format!(
            "# Sequencing Plan\n\n## Summary\n\n{}\n\n## Sequencing\n\nNo authored sequencing plan was recorded for this packet.\n\n## Ordering Rationale\n\n{}\n\n## Readiness Signals\n\n- A downstream implementation reader can identify one bounded slice, its dependencies, and its acceptance anchor without hidden context.\n- Closure findings remain explicit if they still weaken confidence.\n\n## Missing Authored Body\n\nNo `## Sequencing` or `## Sequencing Plan` section was authored in the backlog input. Canon did not invent sequencing steps from priorities alone.\n",
            delivery_intent,
            truncate_context_excerpt(priorities, 220),
        ),
    }
}

fn render_acceptance_anchors_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    source_refs: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Acceptance Anchors\n\n## Summary\n\n{}\n\n## Acceptance Anchors\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Deferred Detail\n\n- Task breakdown, tracker-specific work items, and executable test plans remain downstream work.\n",
            delivery_intent, body, source_refs,
        ),
        None => format!(
            "# Acceptance Anchors\n\n## Summary\n\n{}\n\n## Acceptance Anchors\n\nNo authored acceptance anchors were recorded for this packet.\n\n## Source Trace Links\n\n{}\n\n## Deferred Detail\n\n- Task breakdown, tracker-specific work items, and executable test plans remain downstream work.\n\n## Missing Authored Body\n\nNo `## Acceptance Anchors` section was authored in the backlog input. Canon did not fabricate acceptance anchors from traceability alone.\n",
            delivery_intent, source_refs,
        ),
    }
}

fn render_planning_risks_artifact(
    delivery_intent: &str,
    authored_body: Option<&str>,
    closure_findings: &str,
    critique_evidence: &str,
    validation_evidence: &str,
) -> String {
    match authored_body {
        Some(body) => format!(
            "# Planning Risks\n\n## Summary\n\n{}\n\n## Closure Findings\n\n{}\n\n## Planning Risks\n\n{}\n\n## Follow-Up Triggers\n\n- Return to architecture or change when closure findings stay blocking.\n- Strengthen the authored backlog brief when exclusions or priorities remain vague.\n- Re-run backlog only after the bounded upstream packet becomes more credible.\n",
            delivery_intent, closure_findings, body,
        ),
        None => format!(
            "# Planning Risks\n\n## Summary\n\n{}\n\n## Closure Findings\n\n{}\n\n## Planning Risks\n\nNo authored planning risks were recorded for this packet.\n\n## Evidence Boundaries\n\n- Critique evidence remains: {}\n- Validation evidence remains: {}\n- Granularity drift risk remains explicit: backlog output must stay above task level.\n\n## Missing Authored Body\n\nNo `## Planning Risks` section was authored in the backlog input. Canon preserved closure findings and cited evidence boundaries instead of inventing risk bullets.\n\n## Follow-Up Triggers\n\n- Return to architecture or change when closure findings stay blocking.\n- Strengthen the authored backlog brief when exclusions or priorities remain vague.\n- Re-run backlog only after the bounded upstream packet becomes more credible.\n",
            delivery_intent,
            closure_findings,
            truncate_context_excerpt(critique_evidence, 220),
            truncate_context_excerpt(validation_evidence, 220),
        ),
    }
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
