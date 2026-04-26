use crate::domain::run::BacklogPlanningContext;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;
use crate::review::findings::{FindingCategory, ReviewFinding, ReviewPacket};
use crate::review::summary::{ReviewSummary, summary_severity_label};

struct AuthoredSectionSpec<'a> {
    canonical_heading: &'a str,
    aliases: &'a [&'a str],
}

pub const MISSING_AUTHORED_BODY_MARKER: &str = "## Missing Authored Body";

pub fn render_markdown(title: &str, summary: &str) -> String {
    format!("# {title}\n\n## Summary\n\n{summary}\n")
}

pub fn render_requirements_artifact(file_name: &str, idea_summary: &str) -> String {
    match file_name {
        "problem-statement.md" => format!(
            "# Problem Statement\n\n## Summary\n\n{idea_summary}\n\n## Problem\n\nThe team needs a bounded statement of work before AI-assisted generation expands the solution space.\n\n## Boundary\n\nThis run is limited to framing the problem, constraints, and decision surface for the proposed change.\n\n## Success Signal\n\nStakeholders can decide whether to proceed using explicit constraints, exclusions, and recorded tradeoffs.\n"
        ),
        "constraints.md" => format!(
            "# Constraints\n\n## Summary\n\n{idea_summary}\n\n## Constraints\n\n- Keep the implementation local-first and auditable.\n- Preserve explicit human ownership and approval checkpoints.\n- Prefer filesystem persistence over transient chat memory.\n\n## Non-Negotiables\n\n- Risk and zone classification happen before generation.\n- Artifacts must remain inspectable and reusable across later steps.\n"
        ),
        "options.md" => format!(
            "# Options\n\n## Summary\n\n{idea_summary}\n\n## Options\n\n1. Deliver a minimal governed CLI focused on requirements artifacts first.\n2. Expand into a broader mode surface only after the governance spine is stable.\n\n## Recommended Path\n\nStart with the narrow CLI slice so the method, artifacts, and gates are trustworthy before deeper execution modes are added.\n"
        ),
        "tradeoffs.md" => format!(
            "# Tradeoffs\n\n## Summary\n\n{idea_summary}\n\n## Tradeoffs\n\n- Favoring governability reduces raw generation speed.\n- Durable artifacts add upfront structure but improve reviewability.\n- Typed modes constrain flexibility in exchange for safer execution.\n\n## Consequences\n\nThe product will feel opinionated by design, and that opinionation is what keeps acceleration reviewable.\n"
        ),
        "scope-cuts.md" => format!(
            "# Scope Cuts\n\n## Summary\n\n{idea_summary}\n\n## Scope Cuts\n\n- No autonomous multi-agent orchestration in v0.1.\n- No IDE-first experience or plugin marketplace in v0.1.\n- No distributed execution or hosted control plane in v0.1.\n\n## Deferred Work\n\nFuture slices may add broader adapter support only after the local governance path remains stable.\n"
        ),
        "decision-checklist.md" => format!(
            "# Decision Checklist\n\n## Summary\n\n{idea_summary}\n\n## Decision Checklist\n\n- [x] The mode and scope are explicit.\n- [x] Risk classification is recorded before execution.\n- [x] The artifact bundle captures constraints, options, tradeoffs, and cuts.\n- [x] The next stage can review the bundle without relying on chat history.\n\n## Open Questions\n\n- Which downstream mode should consume this bundle first?\n- Does the current artifact contract need stricter organization-specific policy overrides?\n"
        ),
        other => render_markdown(other, idea_summary),
    }
}

pub fn render_requirements_artifact_from_evidence(
    file_name: &str,
    idea_summary: &str,
    authored_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
    _denied_summary: &str,
) -> String {
    match file_name {
        "problem-statement.md" => render_authored_artifact(
            "Problem Statement",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Problem", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Outcome", aliases: &[] },
            ],
        ),
        "constraints.md" => render_authored_artifact(
            "Constraints",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Constraints", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Non-Negotiables", aliases: &[] },
            ],
        ),
        "options.md" => render_authored_artifact(
            "Options",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Options", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommended Path", aliases: &[] },
            ],
        ),
        "tradeoffs.md" => render_authored_artifact(
            "Tradeoffs",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Tradeoffs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Consequences", aliases: &[] },
            ],
        ),
        "scope-cuts.md" => render_authored_artifact(
            "Scope Cuts",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Scope Cuts", aliases: &["Out of Scope"] },
                AuthoredSectionSpec { canonical_heading: "Deferred Work", aliases: &[] },
            ],
        ),
        "decision-checklist.md" => render_authored_artifact(
            "Decision Checklist",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Decision Checklist", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Open Questions", aliases: &[] },
            ],
        ),
        other => render_requirements_artifact(other, idea_summary),
    }
}

pub fn render_discovery_artifact(file_name: &str, brief_summary: &str) -> String {
    let problem =
        extract_authored_h2_section(brief_summary, "Problem Domain", &[]).unwrap_or_else(|| {
            "NOT CAPTURED - No `## Problem Domain` section was authored in the supplied brief."
                .to_string()
        });
    let constraints = extract_authored_h2_section(brief_summary, "Constraints", &[])
        .unwrap_or_else(|| {
            "NOT CAPTURED - No `## Constraints` section was authored in the supplied brief."
                .to_string()
        });
    let next_phase = extract_authored_h2_section(brief_summary, "Next-Phase Shape", &[])
        .or_else(|| extract_authored_h2_section(brief_summary, "Translation Trigger", &[]))
        .or_else(|| extract_authored_h2_section(brief_summary, "Downstream Handoff", &[]))
        .unwrap_or_else(|| {
            "NOT CAPTURED - No discovery handoff section was authored in the supplied brief."
                .to_string()
        });
    let summary = render_discovery_bundle_summary(file_name, &problem, &constraints, &next_phase);

    match file_name {
        "problem-map.md" => render_authored_artifact(
            "Problem Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Problem Domain", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Repo Surface", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Immediate Tensions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Downstream Handoff", aliases: &[] },
            ],
        ),
        "unknowns-and-assumptions.md" => render_authored_artifact(
            "Unknowns And Assumptions",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Unknowns", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Assumptions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Validation Targets", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Levels", aliases: &[] },
            ],
        ),
        "context-boundary.md" => render_authored_artifact(
            "Context Boundary",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "In-Scope Context", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Repo Surface", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Out-of-Scope Context", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Translation Trigger", aliases: &[] },
            ],
        ),
        "exploration-options.md" => render_authored_artifact(
            "Exploration Options",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Options", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Constraints", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommended Direction", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Next-Phase Shape", aliases: &[] },
            ],
        ),
        "decision-pressure-points.md" => render_authored_artifact(
            "Decision Pressure Points",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Pressure Points", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Blocking Decisions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Open Questions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommended Owner", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_system_shaping_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let intent = extract_marker(context_summary, &normalized, "intent");
    let constraint = extract_marker(context_summary, &normalized, "constraint")
        .or_else(|| extract_marker(context_summary, &normalized, "constraints"));
    let system_shaping_gap = system_shaping_context_gap(intent.as_deref(), constraint.as_deref());
    let authored_summary = if let Some(gap) = system_shaping_gap.as_deref() {
        gap.to_string()
    } else {
        format!(
            "Intent: {}\nConstraint: {}",
            intent.as_deref().unwrap_or_default(),
            constraint.as_deref().unwrap_or_default()
        )
    };

    match file_name {
        "system-shape.md" => render_authored_artifact(
            "System Shape",
            &authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "System Shape", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Decisions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Domain Responsibilities", aliases: &[] },
            ],
        ),
        "domain-model.md" => render_authored_artifact(
            "Domain Model",
            &authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Candidate Bounded Contexts",
                    aliases: &[],
                },
                AuthoredSectionSpec {
                    canonical_heading: "Core And Supporting Domain Hypotheses",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Ubiquitous Language", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Domain Invariants", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Boundary Risks And Open Questions",
                    aliases: &[],
                },
            ],
        ),
        "architecture-outline.md" => render_authored_artifact(
            "Architecture Outline",
            &authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Structural Options", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Selected Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Rationale", aliases: &[] },
            ],
        ),
        "capability-map.md" => render_authored_artifact(
            "Capability Map",
            &authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Capabilities", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Dependencies", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Gaps", aliases: &[] },
            ],
        ),
        "delivery-options.md" => render_authored_artifact(
            "Delivery Options",
            &authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Delivery Phases", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Sequencing Rationale", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Risk per Phase", aliases: &[] },
            ],
        ),
        "risk-hotspots.md" => render_authored_artifact(
            "Risk Hotspots",
            &authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Hotspots", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Mitigation Status", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Unresolved Risks", aliases: &[] },
            ],
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_architecture_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let decision_focus = extract_marker(context_summary, &normalized, "decision focus")
        .or_else(|| extract_authored_h2_section(context_summary, "Decision", &[]))
        .unwrap_or_else(|| {
            "NOT CAPTURED - No architecture decision focus was authored in the supplied brief."
                .to_string()
        });
    let constraint = extract_marker(context_summary, &normalized, "constraint")
        .or_else(|| extract_authored_h2_section(context_summary, "Constraints", &[]))
        .unwrap_or_else(|| {
            "NOT CAPTURED - No architecture constraint was authored in the supplied brief."
                .to_string()
        });
    let architecture_summary =
        format!("Decision focus: {decision_focus}\nConstraint: {constraint}");

    match file_name {
        "architecture-decisions.md" => render_authored_artifact(
            "Architecture Decisions",
            &architecture_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Decision", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Constraints", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Drivers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommendation", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Consequences", aliases: &["Risks"] },
            ],
        ),
        "invariants.md" => format!(
            "# Invariants\n\n## Summary\n\n{context_summary}\n\n## Invariants\n\n{generation_summary}\n\n## Rationale\n\n{critique_summary}\n\n## Verification Hooks\n\n- Downstream modes must be able to validate these invariants against emitted evidence.\n"
        ),
        "tradeoff-matrix.md" => render_authored_artifact(
            "Tradeoff Matrix",
            &architecture_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Options Considered", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Evaluation Criteria", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Pros", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Cons", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Why Not The Others", aliases: &[] },
            ],
        ),
        "boundary-map.md" => format!(
            "# Boundary Map\n\n## Summary\n\n{context_summary}\n\n## Boundaries\n\n{generation_summary}\n\n## Ownership\n\n- Ownership must remain explicit for each named boundary before implementation begins.\n\n## Crossing Rules\n\n{critique_summary}\n"
        ),
        "context-map.md" => render_authored_artifact(
            "Context Map",
            &architecture_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Bounded Contexts", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Context Relationships", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Integration Seams", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Anti-Corruption Candidates",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Ownership Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Shared Invariants", aliases: &[] },
            ],
        ),
        "readiness-assessment.md" => format!(
            "# Readiness Assessment\n\n## Summary\n\n{context_summary}\n\n## Readiness Status\n\nArchitecture analysis is ready for downstream consumption once approvals and unresolved questions are addressed.\n\n## Blockers\n\n{critique_summary}\n\n## Accepted Risks\n\n{generation_summary}\n"
        ),
        "system-context.md" => {
            render_c4_artifact("System Context", "system context", context_summary)
        }
        "container-view.md" => render_c4_artifact("Container View", "containers", context_summary),
        "component-view.md" => render_c4_artifact("Component View", "components", context_summary),
        other => render_markdown(other, context_summary),
    }
}

/// Shared marker emitted by C4 architecture artifacts when the authored brief
/// did not include the canonical H2 section. Tests rely on this exact text.
pub const C4_MISSING_AUTHORED_BODY_MARKER: &str = MISSING_AUTHORED_BODY_MARKER;

fn render_c4_artifact(title: &str, marker: &str, context_summary: &str) -> String {
    let normalized = context_summary.to_lowercase();
    let canonical = canonical_c4_heading(marker);
    match extract_marker(context_summary, &normalized, marker) {
        Some(body) if !body.trim().is_empty() => {
            format!("# {title}\n\n{canonical}\n\n{body}\n")
        }
        _ => {
            format!(
                "# {title}\n\n{canonical}\n\n{marker_heading}\n\nNo `{canonical}` section was authored in the supplied brief.\nAuthor this section in the architecture brief and rerun.\n",
                marker_heading = C4_MISSING_AUTHORED_BODY_MARKER,
            )
        }
    }
}

fn render_authored_artifact(
    title: &str,
    summary: &str,
    authored_source: &str,
    sections: &[AuthoredSectionSpec<'_>],
) -> String {
    let rendered_sections = sections
        .iter()
        .map(|section| render_authored_section(authored_source, section))
        .collect::<Vec<_>>()
        .join("\n\n");

    format!("# {title}\n\n## Summary\n\n{summary}\n\n{rendered_sections}\n")
}

fn render_authored_section(authored_source: &str, section: &AuthoredSectionSpec<'_>) -> String {
    match extract_authored_h2_section(authored_source, section.canonical_heading, section.aliases) {
        Some(body) => format!("## {}\n\n{}", section.canonical_heading, body),
        None => render_missing_authored_body_block(section.canonical_heading),
    }
}

fn render_missing_authored_body_block(canonical_heading: &str) -> String {
    format!(
        "{MISSING_AUTHORED_BODY_MARKER}\n\nNOT CAPTURED - No `## {canonical_heading}` section was authored in the supplied brief.\nAuthor this section in the supplied brief and rerun."
    )
}

fn extract_authored_h2_section(
    source: &str,
    canonical_heading: &str,
    aliases: &[&str],
) -> Option<String> {
    std::iter::once(canonical_heading)
        .chain(aliases.iter().copied())
        .find_map(|heading| extract_markdown_h2_section(source, heading))
}

fn extract_markdown_h2_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_h2_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if is_section_boundary(next_line) {
                break;
            }

            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_multiline_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
        return None;
    }

    None
}

fn is_matching_h2_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with("##") || trimmed.starts_with("###") {
        return false;
    }

    trimmed
        .strip_prefix("##")
        .map(str::trim)
        .is_some_and(|heading| heading.eq_ignore_ascii_case(marker))
}

fn canonical_c4_heading(marker: &str) -> &'static str {
    match marker {
        "system context" => "## System Context",
        "containers" => "## Containers",
        "components" => "## Components",
        _ => "## (unknown C4 section)",
    }
}

pub fn render_change_artifact(file_name: &str, brief_summary: &str, default_owner: &str) -> String {
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
                AuthoredSectionSpec { canonical_heading: "Boundary Tradeoffs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Consequences", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Unresolved Questions", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_incident_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let incident_scope = extract_marker(brief_summary, &normalized, "incident scope")
        .unwrap_or_else(|| "NOT CAPTURED - Incident scope is missing.".to_string());
    let trigger_and_current_state =
        extract_marker(brief_summary, &normalized, "trigger and current state")
            .unwrap_or_else(|| "NOT CAPTURED - Trigger and current state are missing.".to_string());
    let operational_constraints =
        extract_marker(brief_summary, &normalized, "operational constraints")
            .unwrap_or_else(|| "NOT CAPTURED - Operational constraints are missing.".to_string());
    let known_facts = extract_marker(brief_summary, &normalized, "known facts")
        .unwrap_or_else(|| "NOT CAPTURED - Known facts are missing.".to_string());
    let working_hypotheses = extract_marker(brief_summary, &normalized, "working hypotheses")
        .unwrap_or_else(|| "NOT CAPTURED - Working hypotheses are missing.".to_string());
    let evidence_gaps = extract_marker(brief_summary, &normalized, "evidence gaps")
        .unwrap_or_else(|| "NOT CAPTURED - Evidence gaps are missing.".to_string());
    let impacted_surfaces = extract_marker(brief_summary, &normalized, "impacted surfaces")
        .unwrap_or_else(|| "NOT CAPTURED - Impacted surfaces are missing.".to_string());
    let propagation_paths = extract_marker(brief_summary, &normalized, "propagation paths")
        .unwrap_or_else(|| "NOT CAPTURED - Propagation paths are missing.".to_string());
    let confidence_and_unknowns =
        extract_marker(brief_summary, &normalized, "confidence and unknowns")
            .unwrap_or_else(|| "NOT CAPTURED - Confidence and unknowns are missing.".to_string());
    let immediate_actions = extract_marker(brief_summary, &normalized, "immediate actions")
        .unwrap_or_else(|| "NOT CAPTURED - Immediate actions are missing.".to_string());
    let ordered_sequence = extract_marker(brief_summary, &normalized, "ordered sequence")
        .unwrap_or_else(|| "NOT CAPTURED - Ordered sequence is missing.".to_string());
    let stop_conditions = extract_marker(brief_summary, &normalized, "stop conditions")
        .unwrap_or_else(|| "NOT CAPTURED - Stop conditions are missing.".to_string());
    let decision_points = extract_marker(brief_summary, &normalized, "decision points")
        .unwrap_or_else(|| "NOT CAPTURED - Decision points are missing.".to_string());
    let approved_actions = extract_marker(brief_summary, &normalized, "approved actions")
        .unwrap_or_else(|| "NOT CAPTURED - Approved actions are missing.".to_string());
    let deferred_actions = extract_marker(brief_summary, &normalized, "deferred actions")
        .unwrap_or_else(|| "NOT CAPTURED - Deferred actions are missing.".to_string());
    let verification_checks = extract_marker(brief_summary, &normalized, "verification checks")
        .unwrap_or_else(|| "NOT CAPTURED - Verification checks are missing.".to_string());
    let release_readiness = extract_marker(brief_summary, &normalized, "release readiness")
        .unwrap_or_else(|| "NOT CAPTURED - Release readiness posture is missing.".to_string());
    let follow_up_work = extract_marker(brief_summary, &normalized, "follow-up work")
        .unwrap_or_else(|| "NOT CAPTURED - Follow-up work is missing.".to_string());
    let summary =
        format!("Bounded incident packet for {}.", truncate_context_excerpt(&incident_scope, 120));

    match file_name {
        "incident-frame.md" => format!(
            "# Incident Frame\n\n## Summary\n\n{summary}\n\n## Incident Scope\n\n{incident_scope}\n\n## Trigger And Current State\n\n{trigger_and_current_state}\n\n## Operational Constraints\n\n{operational_constraints}\n"
        ),
        "hypothesis-log.md" => format!(
            "# Hypothesis Log\n\n## Summary\n\n{summary}\n\n## Known Facts\n\n{known_facts}\n\n## Working Hypotheses\n\n{working_hypotheses}\n\n## Evidence Gaps\n\n{evidence_gaps}\n"
        ),
        "blast-radius-map.md" => format!(
            "# Blast Radius Map\n\n## Summary\n\n{summary}\n\n## Impacted Surfaces\n\n{impacted_surfaces}\n\n## Propagation Paths\n\n{propagation_paths}\n\n## Confidence And Unknowns\n\n{confidence_and_unknowns}\n"
        ),
        "containment-plan.md" => format!(
            "# Containment Plan\n\n## Summary\n\n{summary}\n\n## Immediate Actions\n\n{immediate_actions}\n\n## Ordered Sequence\n\n{ordered_sequence}\n\n## Stop Conditions\n\n{stop_conditions}\n"
        ),
        "incident-decision-record.md" => format!(
            "# Incident Decision Record\n\n## Summary\n\n{summary}\n\n## Decision Points\n\n{decision_points}\n\n## Approved Actions\n\n{approved_actions}\n\n## Deferred Actions\n\n{deferred_actions}\n"
        ),
        "follow-up-verification.md" => format!(
            "# Follow-Up Verification\n\n## Summary\n\n{summary}\n\n## Verification Checks\n\n{verification_checks}\n\n## Release Readiness\n\n{release_readiness}\n\n## Follow-Up Work\n\n{follow_up_work}\n"
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_migration_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let current_state = extract_marker(brief_summary, &normalized, "current state")
        .unwrap_or_else(|| "NOT CAPTURED - Current state is missing.".to_string());
    let target_state = extract_marker(brief_summary, &normalized, "target state")
        .unwrap_or_else(|| "NOT CAPTURED - Target state is missing.".to_string());
    let transition_boundaries = extract_marker(brief_summary, &normalized, "transition boundaries")
        .unwrap_or_else(|| "NOT CAPTURED - Transition boundaries are missing.".to_string());
    let guaranteed_compatibility =
        extract_marker(brief_summary, &normalized, "guaranteed compatibility")
            .unwrap_or_else(|| "NOT CAPTURED - Guaranteed compatibility is missing.".to_string());
    let temporary_incompatibilities =
        extract_marker(brief_summary, &normalized, "temporary incompatibilities").unwrap_or_else(
            || "NOT CAPTURED - Temporary incompatibilities are missing.".to_string(),
        );
    let coexistence_rules = extract_marker(brief_summary, &normalized, "coexistence rules")
        .unwrap_or_else(|| "NOT CAPTURED - Coexistence rules are missing.".to_string());
    let ordered_steps = extract_marker(brief_summary, &normalized, "ordered steps")
        .unwrap_or_else(|| "NOT CAPTURED - Ordered steps are missing.".to_string());
    let parallelizable_work = extract_marker(brief_summary, &normalized, "parallelizable work")
        .unwrap_or_else(|| "NOT CAPTURED - Parallelizable work is missing.".to_string());
    let cutover_criteria = extract_marker(brief_summary, &normalized, "cutover criteria")
        .unwrap_or_else(|| "NOT CAPTURED - Cutover criteria are missing.".to_string());
    let rollback_triggers = extract_marker(brief_summary, &normalized, "rollback triggers")
        .unwrap_or_else(|| "NOT CAPTURED - Rollback triggers are missing.".to_string());
    let fallback_paths = extract_marker(brief_summary, &normalized, "fallback paths")
        .unwrap_or_else(|| "NOT CAPTURED - Fallback paths are missing.".to_string());
    let re_entry_criteria = extract_marker(brief_summary, &normalized, "re-entry criteria")
        .unwrap_or_else(|| "NOT CAPTURED - Re-entry criteria are missing.".to_string());
    let verification_checks = extract_marker(brief_summary, &normalized, "verification checks")
        .unwrap_or_else(|| "NOT CAPTURED - Verification checks are missing.".to_string());
    let residual_risks = extract_marker(brief_summary, &normalized, "residual risks")
        .unwrap_or_else(|| "NOT CAPTURED - Residual risks are missing.".to_string());
    let release_readiness = extract_marker(brief_summary, &normalized, "release readiness")
        .unwrap_or_else(|| "NOT CAPTURED - Release readiness posture is missing.".to_string());
    let migration_decisions = extract_marker(brief_summary, &normalized, "migration decisions")
        .unwrap_or_else(|| "NOT CAPTURED - Migration decisions are missing.".to_string());
    let deferred_decisions = extract_marker(brief_summary, &normalized, "deferred decisions")
        .unwrap_or_else(|| "NOT CAPTURED - Deferred decisions are missing.".to_string());
    let approval_notes = extract_marker(brief_summary, &normalized, "approval notes")
        .unwrap_or_else(|| "NOT CAPTURED - Approval notes are missing.".to_string());
    let summary = format!(
        "Bounded migration packet from {} to {}.",
        truncate_context_excerpt(&current_state, 80),
        truncate_context_excerpt(&target_state, 80)
    );

    match file_name {
        "source-target-map.md" => format!(
            "# Source-Target Map\n\n## Summary\n\n{summary}\n\n## Current State\n\n{current_state}\n\n## Target State\n\n{target_state}\n\n## Transition Boundaries\n\n{transition_boundaries}\n"
        ),
        "compatibility-matrix.md" => format!(
            "# Compatibility Matrix\n\n## Summary\n\n{summary}\n\n## Guaranteed Compatibility\n\n{guaranteed_compatibility}\n\n## Temporary Incompatibilities\n\n{temporary_incompatibilities}\n\n## Coexistence Rules\n\n{coexistence_rules}\n"
        ),
        "sequencing-plan.md" => format!(
            "# Sequencing Plan\n\n## Summary\n\n{summary}\n\n## Ordered Steps\n\n{ordered_steps}\n\n## Parallelizable Work\n\n{parallelizable_work}\n\n## Cutover Criteria\n\n{cutover_criteria}\n"
        ),
        "fallback-plan.md" => format!(
            "# Fallback Plan\n\n## Summary\n\n{summary}\n\n## Rollback Triggers\n\n{rollback_triggers}\n\n## Fallback Paths\n\n{fallback_paths}\n\n## Re-Entry Criteria\n\n{re_entry_criteria}\n"
        ),
        "migration-verification-report.md" => format!(
            "# Migration Verification Report\n\n## Summary\n\n{summary}\n\n## Verification Checks\n\n{verification_checks}\n\n## Residual Risks\n\n{residual_risks}\n\n## Release Readiness\n\n{release_readiness}\n"
        ),
        "decision-record.md" => format!(
            "# Decision Record\n\n## Summary\n\n{summary}\n\n## Migration Decisions\n\n{migration_decisions}\n\n## Deferred Decisions\n\n{deferred_decisions}\n\n## Approval Notes\n\n{approval_notes}\n"
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_backlog_artifact(
    file_name: &str,
    brief_summary: &str,
    planning_context: &BacklogPlanningContext,
) -> String {
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
        "backlog-overview.md" => format!(
            "# Backlog Overview\n\n## Summary\n\n{}\n\n## Scope\n\n{}\n\n## Planning Horizon\n\n{}\n\n## Source Inputs\n\n{}\n\n## Delivery Intent\n\n{}\n\n## Decomposition Posture\n\n{}\n",
            delivery_intent,
            truncate_context_excerpt(&generated_framing, 260),
            planning_horizon,
            source_refs,
            delivery_intent,
            decomposition_posture,
        ),
        "epic-tree.md" => match authored_epic_tree {
            Some(body) => format!(
                "# Epic Tree\n\n## Summary\n\n{}\n\n## Epic Tree\n\n{}\n\n## Scope Boundaries\n\n- Preserve planning-only granularity at {}.\n- Keep excluded work explicit: {}\n\n## Source Trace Links\n\n{}\n",
                delivery_intent,
                body,
                planning_context.desired_granularity.as_str(),
                truncate_context_excerpt(&out_of_scope, 200),
                source_refs,
            ),
            None => format!(
                "# Epic Tree\n\n## Summary\n\n{}\n\n## Epic Tree\n\n- Initiative: {}\n- Epic 1: Establish a bounded foundation for {}\n- Epic 2: Deliver visible slices without descending into task plans\n\n## Scope Boundaries\n\n- Preserve planning-only granularity at {}.\n- Keep excluded work explicit: {}\n\n## Source Trace Links\n\n{}\n\n## Missing Authored Body\n\nNo `## Epic Tree` section was authored in the backlog input; the entries above are placeholders. Add a real epic tree to the input to replace them.\n",
                delivery_intent,
                truncate_context_excerpt(&delivery_intent, 120),
                truncate_context_excerpt(&delivery_intent, 120),
                planning_context.desired_granularity.as_str(),
                truncate_context_excerpt(&out_of_scope, 200),
                source_refs,
            ),
        },
        "capability-to-epic-map.md" => match authored_capability_map {
            Some(body) => format!(
                "# Capability To Epic Map\n\n## Summary\n\n{}\n\n## Capability Mapping\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Planning Gaps\n\n{}\n",
                delivery_intent, body, source_refs, closure_findings,
            ),
            None => format!(
                "# Capability To Epic Map\n\n## Summary\n\n{}\n\n## Capability Mapping\n\n- Source capability set remains anchored to the authored delivery intent.\n- Priority inputs shape which epic lands first: {}\n\n## Source Trace Links\n\n{}\n\n## Planning Gaps\n\n{}\n\n## Missing Authored Body\n\nNo `## Capability To Epic Map` section was authored in the backlog input.\n",
                delivery_intent,
                truncate_context_excerpt(&priorities, 200),
                source_refs,
                closure_findings,
            ),
        },
        "dependency-map.md" => match authored_dependency_map {
            Some(body) => format!(
                "# Dependency Map\n\n## Summary\n\n{}\n\n## Dependencies\n\n{}\n\n## Blocking Edges\n\n{}\n\n## External Dependencies\n\n- Any external blockers must remain visible in planning risks before downstream implementation work starts.\n",
                delivery_intent, body, closure_findings,
            ),
            None => format!(
                "# Dependency Map\n\n## Summary\n\n{}\n\n## Dependencies\n\n- Shared planning constraints: {}\n- Source references remain the upstream dependency basis.\n\n## Blocking Edges\n\n{}\n\n## External Dependencies\n\n- Any external blockers must remain visible in planning risks before downstream implementation work starts.\n\n## Missing Authored Body\n\nNo `## Dependency Map` section was authored in the backlog input.\n",
                delivery_intent,
                truncate_context_excerpt(&constraints, 220),
                closure_findings,
            ),
        },
        "delivery-slices.md" => match authored_delivery_slices {
            Some(body) => format!(
                "# Delivery Slices\n\n## Summary\n\n{}\n\n## Delivery Slices\n\n{}\n\n## Slice Boundaries\n\n- Slices stay above task level and stop at implementation-ready decomposition.\n- Excluded work stays explicit: {}\n\n## Dependency Links\n\n{}\n",
                delivery_intent,
                body,
                truncate_context_excerpt(&out_of_scope, 220),
                source_refs,
            ),
            None => format!(
                "# Delivery Slices\n\n## Summary\n\n{}\n\n## Delivery Slices\n\n- Slice 1: Establish the bounded planning and dependency spine.\n- Slice 2: Deliver the first user-visible outcome tied to the highest-priority source input.\n- Slice 3: Address the highest planning risk before broader rollout.\n\n## Slice Boundaries\n\n- Slices stay above task level and stop at implementation-ready decomposition.\n- Excluded work stays explicit: {}\n\n## Dependency Links\n\n{}\n\n## Missing Authored Body\n\nNo `## Delivery Slices` section was authored in the backlog input; the slices above are placeholders.\n",
                delivery_intent,
                truncate_context_excerpt(&out_of_scope, 220),
                source_refs,
            ),
        },
        "sequencing-plan.md" => match authored_sequencing {
            Some(body) => format!(
                "# Sequencing Plan\n\n## Summary\n\n{}\n\n## Sequencing\n\n{}\n\n## Ordering Rationale\n\n{}\n\n## Readiness Signals\n\n- A downstream implementation reader can identify one bounded slice, its dependencies, and its acceptance anchor without hidden context.\n- Closure findings remain explicit if they still weaken confidence.\n",
                delivery_intent,
                body,
                truncate_context_excerpt(&priorities, 220),
            ),
            None => format!(
                "# Sequencing Plan\n\n## Summary\n\n{}\n\n## Sequencing\n\n1. Establish the bounded foundation implied by the source inputs.\n2. Deliver the first slice that resolves the highest-priority planning pressure.\n3. Sequence follow-on slices only after named dependency blockers are visible.\n\n## Ordering Rationale\n\n{}\n\n## Readiness Signals\n\n- A downstream implementation reader can identify one bounded slice, its dependencies, and its acceptance anchor without hidden context.\n- Closure findings remain explicit if they still weaken confidence.\n\n## Missing Authored Body\n\nNo `## Sequencing` or `## Sequencing Plan` section was authored in the backlog input.\n",
                delivery_intent,
                truncate_context_excerpt(&priorities, 220),
            ),
        },
        "acceptance-anchors.md" => match authored_acceptance {
            Some(body) => format!(
                "# Acceptance Anchors\n\n## Summary\n\n{}\n\n## Acceptance Anchors\n\n{}\n\n## Source Trace Links\n\n{}\n\n## Deferred Detail\n\n- Task breakdown, tracker-specific work items, and executable test plans remain downstream work.\n",
                delivery_intent, body, source_refs,
            ),
            None => format!(
                "# Acceptance Anchors\n\n## Summary\n\n{}\n\n## Acceptance Anchors\n\n- Anchor A: the first delivery slice is bounded enough for downstream implementation planning.\n- Anchor B: dependency blockers are named rather than implied.\n- Anchor C: priority and source traceability remain readable in the packet.\n\n## Source Trace Links\n\n{}\n\n## Deferred Detail\n\n- Task breakdown, tracker-specific work items, and executable test plans remain downstream work.\n\n## Missing Authored Body\n\nNo `## Acceptance Anchors` section was authored in the backlog input.\n",
                delivery_intent, source_refs,
            ),
        },
        "planning-risks.md" => match authored_planning_risks {
            Some(body) => format!(
                "# Planning Risks\n\n## Summary\n\n{}\n\n## Closure Findings\n\n{}\n\n## Planning Risks\n\n{}\n\n## Follow-Up Triggers\n\n- Return to architecture or change when closure findings stay blocking.\n- Strengthen the authored backlog brief when exclusions or priorities remain vague.\n- Re-run backlog only after the bounded upstream packet becomes more credible.\n",
                delivery_intent, closure_findings, body,
            ),
            None => format!(
                "# Planning Risks\n\n## Summary\n\n{}\n\n## Closure Findings\n\n{}\n\n## Planning Risks\n\n- Sequencing uncertainty: {}\n- Hidden dependency risk: {}\n- Granularity drift risk: backlog output must stay above task level.\n\n## Follow-Up Triggers\n\n- Return to architecture or change when closure findings stay blocking.\n- Strengthen the authored backlog brief when exclusions or priorities remain vague.\n- Re-run backlog only after the bounded upstream packet becomes more credible.\n",
                delivery_intent,
                closure_findings,
                truncate_context_excerpt(&critique_evidence, 220),
                truncate_context_excerpt(&validation_evidence, 220),
            ),
        },
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_implementation_artifact(
    file_name: &str,
    brief_summary: &str,
    default_owner: &str,
) -> String {
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
                AuthoredSectionSpec { canonical_heading: "Task Linkage", aliases: &[] },
            ],
        ),
        "completion-evidence.md" => render_authored_artifact(
            "Completion Evidence",
            &format!("{summary}\n- Mutation posture: {}", compact_summary_line(&mutation_posture)),
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Completion Evidence", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Remaining Risks", aliases: &[] },
            ],
        ),
        "validation-hooks.md" => render_authored_artifact(
            "Validation Hooks",
            &summary,
            brief_summary,
            &[
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

pub fn render_refactor_artifact(
    file_name: &str,
    brief_summary: &str,
    default_owner: &str,
) -> String {
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

fn extract_authored_section_or_marker(
    source: &str,
    normalized_source: &str,
    canonical_heading: &str,
    heading_aliases: &[&str],
    marker_aliases: &[&str],
) -> Option<String> {
    extract_authored_h2_section(source, canonical_heading, heading_aliases).or_else(|| {
        marker_aliases.iter().find_map(|marker| extract_marker(source, normalized_source, marker))
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

fn preserve_markdown_block(value: &str) -> String {
    let mut lines = Vec::new();
    let mut previous_blank = false;

    for raw_line in value.lines() {
        let line = raw_line.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.is_empty() {
            if !previous_blank && !lines.is_empty() {
                lines.push(String::new());
            }
            previous_blank = true;
        } else {
            lines.push(line);
            previous_blank = false;
        }
    }

    lines.join("\n").trim().to_string()
}

fn render_verification_summary(
    claims_under_test: &str,
    evidence_basis: Option<&str>,
    contract_assumptions: &str,
    challenge_focus: Option<&str>,
    validation_summary: &str,
    verdict: &str,
    has_open_findings: bool,
) -> String {
    let mut lines = Vec::new();

    if let Some(line) = verification_summary_field("Claims under test", Some(claims_under_test)) {
        lines.push(line);
    }
    if let Some(line) = verification_summary_field("Evidence basis", evidence_basis) {
        lines.push(line);
    }
    if let Some(line) = verification_summary_field("Contract surface", Some(contract_assumptions)) {
        lines.push(line);
    }
    if has_open_findings
        && let Some(line) = verification_summary_field("Challenge focus", challenge_focus)
    {
        lines.push(line);
    }

    lines.push(format!("- Verdict: {verdict}"));
    lines.push(if has_open_findings {
        "- Open findings: unresolved follow-up remains recorded in this packet.".to_string()
    } else {
        "- Open findings: none recorded.".to_string()
    });

    if let Some(line) = verification_summary_field("Validation evidence", Some(validation_summary))
    {
        lines.push(line);
    }

    lines.join("\n")
}

fn verification_summary_field(label: &str, value: Option<&str>) -> Option<String> {
    let summary = verification_summary_excerpt(value?)?;
    Some(format!("- {label}: {summary}"))
}

fn verification_summary_excerpt(value: &str) -> Option<String> {
    let items = value.lines().filter_map(verification_summary_line).collect::<Vec<_>>();

    if items.is_empty() {
        let compact = compact_summary_line(value);
        if compact.is_empty() { None } else { Some(compact) }
    } else {
        let mut summary = items.iter().take(2).cloned().collect::<Vec<_>>().join("; ");
        if items.len() > 2 {
            summary.push_str(&format!("; +{} more", items.len() - 2));
        }
        Some(summary)
    }
}

fn verification_summary_line(line: &str) -> Option<String> {
    let trimmed = trim_markdown_list_prefix(line.trim());
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let lowered = trimmed.to_ascii_lowercase();
    if lowered.starts_with("status:") || lowered.starts_with("rationale:") {
        return None;
    }

    let compact = compact_summary_line(trimmed);
    if compact.is_empty() { None } else { Some(compact) }
}

fn trim_markdown_list_prefix(value: &str) -> &str {
    let trimmed = value.trim_start();

    for prefix in ["- ", "* ", "+ "] {
        if let Some(stripped) = trimmed.strip_prefix(prefix) {
            return stripped.trim_start();
        }
    }

    let digit_count = trimmed.bytes().take_while(|byte| byte.is_ascii_digit()).count();
    if digit_count > 0
        && trimmed.as_bytes().get(digit_count) == Some(&b'.')
        && trimmed.as_bytes().get(digit_count + 1) == Some(&b' ')
    {
        return trimmed[digit_count + 2..].trim_start();
    }

    trimmed
}

pub fn render_review_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> String {
    let status = review_disposition_status(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let missing_evidence_open = review_missing_evidence_open(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let evidence_basis = if context_summary.contains("## Input:") {
        "- Review grounded in explicit authored inputs captured for this run.\n- Validation evidence remains linked from the persisted run bundle.".to_string()
    } else {
        "- Review grounded in the bounded authored packet supplied to this run.\n- Validation evidence remains linked from the persisted run bundle.".to_string()
    };
    let boundary_findings = if contains_case_insensitive(critique_summary, "scope drift")
        || contains_case_insensitive(context_summary, "boundary")
        || contains_case_insensitive(generation_summary, "boundary")
    {
        preserve_markdown_block(critique_summary)
    } else {
        "- No boundary expansion beyond the authored review target was detected.".to_string()
    };
    let ownership_notes = if contains_case_insensitive(context_summary, "owner") {
        "- Ownership remains anchored to the named run owner and supplied review target."
            .to_string()
    } else {
        "- Confirm the accountable reviewer before downstream acceptance.".to_string()
    };
    let missing_evidence = if missing_evidence_open {
        format!(
            "Status: missing-evidence-open\n\n{}\n\n{}",
            preserve_markdown_block(critique_summary),
            preserve_markdown_block(validation_summary)
        )
    } else {
        "Status: evidence-bounded\n\n- No critical evidence gaps were detected from the authored package.".to_string()
    };
    let collection_priorities = if missing_evidence_open {
        "- Capture the missing evidence before accepting the packet as release-ready.\n- Keep any remaining accepted risk explicit in the disposition artifact.".to_string()
    } else {
        "- Preserve the current evidence bundle for any later approval or downstream implementation review.".to_string()
    };
    let decision_impact = if generation_summary.trim().is_empty() {
        "- No decision impact summary was generated for this packet.".to_string()
    } else {
        preserve_markdown_block(generation_summary)
    };
    let reversibility = if status == "awaiting-disposition" {
        "- Downstream work should stop until the remaining review concerns receive explicit disposition.".to_string()
    } else {
        "- The packet remains reversible because the current concerns are recorded as bounded review notes.".to_string()
    };
    let accepted_risks = if status == "awaiting-disposition" {
        "- No accepted risks recorded while disposition is still pending.".to_string()
    } else {
        "- Residual review notes remain bounded to the current package and can be inspected through the emitted artifacts.".to_string()
    };
    let rationale = if status == "awaiting-disposition" {
        "The review packet records unresolved concerns or missing evidence that require explicit human disposition before release-readiness can pass.".to_string()
    } else {
        "The review packet is bounded enough for downstream inspection and no unresolved must-fix concerns require disposition approval.".to_string()
    };

    match file_name {
        "review-brief.md" => format!(
            "# Review Brief\n\n## Summary\n\n{}\n\n## Review Target\n\n{}\n\n## Evidence Basis\n\n{}\n",
            compact_summary_line(context_summary),
            preserve_markdown_block(generation_summary),
            evidence_basis,
        ),
        "boundary-assessment.md" => format!(
            "# Boundary Assessment\n\n## Summary\n\n{}\n\n## Boundary Findings\n\n{}\n\n## Ownership Notes\n\n{}\n",
            compact_summary_line(context_summary),
            boundary_findings,
            ownership_notes,
        ),
        "missing-evidence.md" => format!(
            "# Missing Evidence\n\n## Summary\n\n{}\n\n## Missing Evidence\n\n{}\n\n## Collection Priorities\n\n{}\n",
            compact_summary_line(context_summary),
            missing_evidence,
            collection_priorities,
        ),
        "decision-impact.md" => format!(
            "# Decision Impact\n\n## Summary\n\n{}\n\n## Decision Impact\n\n{}\n\n## Reversibility Concerns\n\n{}\n",
            compact_summary_line(context_summary),
            decision_impact,
            reversibility,
        ),
        "review-disposition.md" => format!(
            "# Review Disposition\n\n## Summary\n\n{}\n\n## Final Disposition\n\nStatus: {}\n\nRationale: {}\n\n## Accepted Risks\n\n{}\n",
            compact_summary_line(context_summary),
            status,
            rationale,
            accepted_risks,
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_verification_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> String {
    let has_open_findings = verification_has_open_findings(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let verdict = verification_verdict(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    );
    let claims_under_test = verification_section_from_sources(
        generation_summary,
        Some(context_summary),
        &["claims under test"],
    )
    .unwrap_or_else(|| {
        "- No explicit claims under test were captured for this verification packet.".to_string()
    });
    let evidence_basis = verification_section_from_sources(
        generation_summary,
        Some(context_summary),
        &["evidence basis"],
    );
    let contract_assumptions = verification_section_from_sources(
        generation_summary,
        Some(context_summary),
        &["contract assumptions", "contract surface"],
    )
    .unwrap_or_else(|| {
        "- Contract assumptions were not explicitly authored for this verification packet."
            .to_string()
    });
    let challenge_findings = append_markdown_blocks(
        verification_section_from_sources(
            critique_summary,
            Some(context_summary),
            &["challenge findings", "challenge focus"],
        )
        .unwrap_or_else(|| {
            if has_open_findings {
                "- The verification packet still carries challenge findings that require explicit follow-up."
                    .to_string()
            } else {
                "- No additional challenge findings were recorded beyond the authored verification packet."
                    .to_string()
            }
        }),
        Some(preserve_markdown_block(validation_summary)),
    );
    let contradictions = verification_section_from_sources(
        critique_summary,
        Some(context_summary),
        &["contradictions"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            "- The verification packet still contains contradictions or evidence gaps that keep the verdict open."
                .to_string()
        } else {
            "- No direct contradictions were identified from the current verification packet."
                .to_string()
        }
    });
    let verified_claims = verification_section_from_sources(
        critique_summary,
        Some(generation_summary),
        &["verified claims"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            "- The packet captures explicit claims and evidence basis, but the strongest assurances remain under challenge."
                .to_string()
        } else {
            claims_under_test.clone()
        }
    });
    let rejected_claims =
        verification_section_from_sources(critique_summary, None, &["rejected claims"])
            .unwrap_or_else(|| {
                if has_open_findings {
                    contradictions.clone()
                } else {
                    "- No rejected claims were inferred from the current verification target."
                        .to_string()
                }
            });
    let open_findings = verification_section_from_sources(
        critique_summary,
        None,
        &["open findings"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            format!("Status: unresolved-findings-open\n\n{contradictions}")
        } else {
            "Status: no-open-findings\n\n- No unresolved findings remain from the current verification target."
                .to_string()
        }
    });
    let required_follow_up = verification_section_from_sources(
        critique_summary,
        None,
        &["required follow-up"],
    )
    .unwrap_or_else(|| {
        if has_open_findings {
            "- Resolve or explicitly challenge the open contradictions before treating the packet as release-ready.\n- Preserve the unresolved findings for downstream inspection."
                .to_string()
        } else {
            "- Keep the verification packet attached to any downstream release or approval discussion."
                .to_string()
        }
    });
    let overall_verdict = verification_section_from_sources(
        critique_summary,
        None,
        &["overall verdict"],
    )
    .unwrap_or_else(|| {
        format!(
            "Status: {verdict}\nRationale: Verification verdict was derived from the current packet because no explicit overall verdict section was emitted."
        )
    });
    let challenge_focus = verification_section_from_sources(
        context_summary,
        Some(critique_summary),
        &["challenge focus", "challenge findings"],
    );
    let verification_summary = render_verification_summary(
        &claims_under_test,
        evidence_basis.as_deref(),
        &contract_assumptions,
        challenge_focus.as_deref(),
        validation_summary,
        verdict,
        has_open_findings,
    );

    match file_name {
        "invariants-checklist.md" => format!(
            "# Invariants Checklist\n\n## Summary\n\n{}\n\n## Claims Under Test\n\n{}\n\n## Invariant Checks\n\n{}\n",
            verification_summary,
            claims_under_test,
            if has_open_findings {
                "- Some invariant checks remain unresolved against the current evidence bundle."
            } else {
                "- The current invariants are bounded enough for recorded verification."
            },
        ),
        "contract-matrix.md" => format!(
            "# Contract Matrix\n\n## Summary\n\n{}\n\n## Contract Assumptions\n\n{}\n\n## Verification Outcome\n\nStatus: {}\n",
            verification_summary, contract_assumptions, verdict,
        ),
        "adversarial-review.md" => format!(
            "# Adversarial Review\n\n## Summary\n\n{}\n\n## Challenge Findings\n\n{}\n\n## Contradictions\n\n{}\n",
            verification_summary, challenge_findings, contradictions,
        ),
        "verification-report.md" => format!(
            "# Verification Report\n\n## Summary\n\n{}\n\n## Verified Claims\n\n{}\n\n## Rejected Claims\n\n{}\n\n## Overall Verdict\n\n{}\n",
            verification_summary, verified_claims, rejected_claims, overall_verdict,
        ),
        "unresolved-findings.md" => format!(
            "# Unresolved Findings\n\n## Summary\n\n{}\n\n## Open Findings\n\n{}\n\n## Required Follow-up\n\n{}\n",
            verification_summary, open_findings, required_follow_up,
        ),
        other => render_markdown(other, context_summary),
    }
}

fn review_disposition_status(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> &'static str {
    if review_missing_evidence_open(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    ) || contains_case_insensitive(critique_summary, "must-fix")
        || contains_case_insensitive(generation_summary, "must-fix")
        || contains_case_insensitive(critique_summary, "blocking")
    {
        "awaiting-disposition"
    } else {
        "ready-with-review-notes"
    }
}

fn review_missing_evidence_open(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> bool {
    [context_summary, generation_summary, critique_summary, validation_summary].into_iter().any(
        |value| {
            value.contains("NOT CAPTURED")
                || contains_case_insensitive(value, "missing evidence")
                || contains_case_insensitive(value, "insufficient evidence")
        },
    )
}

fn verification_has_open_findings(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> bool {
    let critique_normalized = critique_summary.to_lowercase();
    if let Some(open_findings) =
        extract_marker(critique_summary, &critique_normalized, "open findings")
        && let Some(status) = extract_labeled_block_value(&open_findings, "Status")
    {
        return status.eq_ignore_ascii_case("unresolved-findings-open");
    }

    [context_summary, generation_summary, critique_summary, validation_summary].into_iter().any(
        |value| {
            value.contains("NOT CAPTURED")
                || contains_case_insensitive(value, "unsupported")
                || contains_case_insensitive(value, "contradiction")
                || contains_case_insensitive(value, "unresolved")
                || contains_case_insensitive(value, "insufficient evidence")
        },
    )
}

fn verification_verdict(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
    validation_summary: &str,
) -> &'static str {
    let critique_normalized = critique_summary.to_lowercase();
    if let Some(overall_verdict) =
        extract_marker(critique_summary, &critique_normalized, "overall verdict")
        && let Some(status) = extract_labeled_block_value(&overall_verdict, "Status")
    {
        return match status.to_ascii_lowercase().as_str() {
            "supported" => "supported",
            "unsupported" => "unsupported",
            "mixed" => "mixed",
            _ => "mixed",
        };
    }

    if !verification_has_open_findings(
        context_summary,
        generation_summary,
        critique_summary,
        validation_summary,
    ) {
        "supported"
    } else if context_summary.contains("NOT CAPTURED")
        || contains_case_insensitive(critique_summary, "contradiction")
        || contains_case_insensitive(validation_summary, "contradiction")
    {
        "unsupported"
    } else {
        "mixed"
    }
}

fn verification_section_from_sources(
    primary_source: &str,
    secondary_source: Option<&str>,
    markers: &[&str],
) -> Option<String> {
    extract_any_marker(primary_source, markers)
        .or_else(|| secondary_source.and_then(|source| extract_any_marker(source, markers)))
}

fn extract_any_marker(source: &str, markers: &[&str]) -> Option<String> {
    let normalized = source.to_lowercase();

    markers.iter().find_map(|marker| {
        extract_marker(source, &normalized, marker)
            .map(|value| preserve_markdown_block(&value))
            .filter(|value| !value.is_empty())
    })
}

fn append_markdown_blocks(primary: String, secondary: Option<String>) -> String {
    let Some(secondary) =
        secondary.map(|value| value.trim().to_string()).filter(|value| !value.is_empty())
    else {
        return primary;
    };

    if primary.trim().is_empty() {
        secondary
    } else if primary.contains(&secondary) {
        primary
    } else {
        format!("{primary}\n\n{secondary}")
    }
}

fn extract_labeled_block_value(block: &str, label: &str) -> Option<String> {
    let prefix = format!("{}:", label.to_ascii_lowercase());

    block.lines().find_map(|line| {
        let trimmed = line.trim();
        if !trimmed.to_ascii_lowercase().starts_with(&prefix) {
            return None;
        }

        let value = trimmed[trimmed.find(':')? + 1..].trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    })
}

fn contains_case_insensitive(value: &str, needle: &str) -> bool {
    value.to_lowercase().contains(&needle.to_lowercase())
}

fn render_discovery_bundle_summary(
    current_file: &str,
    problem: &str,
    constraints: &str,
    next_phase: &str,
) -> String {
    let detail_links = [
        "problem-map.md",
        "unknowns-and-assumptions.md",
        "context-boundary.md",
        "exploration-options.md",
        "decision-pressure-points.md",
    ]
    .into_iter()
    .filter(|file_name| *file_name != current_file)
    .map(|file_name| format!("[{file_name}]({file_name})"))
    .collect::<Vec<_>>()
    .join(", ");

    let format_field = |label: &str, content: &str| {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return String::new();
        }
        if trimmed.contains('\n') || trimmed.starts_with('-') || trimmed.len() > 100 {
            format!("- **{label}:**\n\n  {}", trimmed.replace("\n", "\n  "))
        } else {
            format!("- **{label}:** {trimmed}")
        }
    };

    let mut parts = Vec::new();
    let prob = format_field("Problem", problem);
    if !prob.is_empty() {
        parts.push(prob);
    }

    let con = format_field("Constraints", constraints);
    if !con.is_empty() {
        parts.push(con);
    }

    let phase = format_field("Next phase", next_phase);
    if !phase.is_empty() {
        parts.push(phase);
    }

    parts.push(format!("- **Details:** {detail_links}"));
    parts.join("\n")
}

fn system_shaping_context_gap(intent: Option<&str>, constraint: Option<&str>) -> Option<String> {
    if intent.is_some() && constraint.is_some() {
        None
    } else {
        Some(
            "Insufficient evidence: supply explicit `Intent:` and `Constraint:` markers in the system-shaping brief before system shaping can proceed."
                .to_string(),
        )
    }
}

pub fn render_pr_review_artifact(
    file_name: &str,
    packet: &ReviewPacket,
    summary: &ReviewSummary,
) -> String {
    match file_name {
        "pr-analysis.md" => format!(
            "# PR Analysis\n\n## Summary\n\nReviewing `{}` against `{}` across {} changed surface(s).\n\n## Evidence Posture\n\n- Review packet derived from governed diff inspection and critique evidence.\n- Artifact provenance remains linked from the run evidence bundle.\n\n## Scope Summary\n\n- Base ref: `{}`\n- Head ref: `{}`\n- Changed surface count: {}\n\n## Changed Modules\n\n{}\n\n## Inferred Intent\n\n{}\n\n## Surprising Surface Area\n\n{}\n",
            packet.head_ref,
            packet.base_ref,
            packet.changed_surfaces.len(),
            packet.base_ref,
            packet.head_ref,
            packet.changed_surfaces.len(),
            render_surface_list(&packet.changed_surfaces, "- No changed surfaces detected."),
            packet.inferred_intent,
            render_surface_list(
                &packet.surprising_surface_area,
                "- No surprising surface area inferred from the diff."
            ),
        ),
        "boundary-check.md" => format!(
            "# Boundary Check\n\n## Summary\n\nBoundary review for changed surfaces between `{}` and `{}`.\n\n## Boundary Findings\n\n{}\n\n## Ownership Breaks\n\n{}\n\n## Unauthorized Structural Impact\n\nStatus: {}\n\n{}\n",
            packet.base_ref,
            packet.head_ref,
            render_findings(
                &packet.findings_for(FindingCategory::BoundaryCheck),
                "- No boundary findings detected."
            ),
            ownership_breaks(packet),
            if packet.findings_for(FindingCategory::BoundaryCheck).is_empty() {
                "no-structural-impact-detected"
            } else {
                "structural-impact-reviewed"
            },
            if packet.findings_for(FindingCategory::BoundaryCheck).is_empty() {
                "- No unauthorized structural impact inferred."
            } else {
                "- Boundary-marked files changed and remain explicit in the review packet."
            },
        ),
        "conventional-comments.md" => format!(
            "# Conventional Comments\n\n## Summary\n\nReviewer-facing conventional comments derived from {} persisted finding(s) for `{}` against `{}`.\n\n## Evidence Posture\n\n- Comment kinds are deterministically mapped from persisted review findings.\n- Entries remain surface-scoped and do not fabricate line-level anchors.\n- Approval posture remains anchored by `review-summary.md`.\n\n## Conventional Comments\n\n{}\n\n## Traceability\n\n- Review summary status: `{}`\n- Changed surfaces: {}\n- Source packet: `review-summary.md` and `pr-analysis.md`\n",
            packet.findings.len(),
            packet.head_ref,
            packet.base_ref,
            render_conventional_comments(packet),
            summary.disposition.as_str(),
            if packet.changed_surfaces.is_empty() {
                "none".to_string()
            } else {
                packet.changed_surfaces.join(", ")
            },
        ),
        "duplication-check.md" => format!(
            "# Duplication Check\n\n## Summary\n\nDuplication review for the bounded diff.\n\n## Duplicate Behavior\n\n{}\n\n## Canonical Owner Conflicts\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::DuplicationCheck),
                "- No duplicate behavior concerns inferred."
            ),
            if packet.findings_for(FindingCategory::DuplicationCheck).is_empty() {
                "- No canonical owner conflicts inferred."
            } else {
                "- Canonical ownership needs review where duplicate logic surfaced."
            },
        ),
        "contract-drift.md" => format!(
            "# Contract Drift\n\n## Summary\n\nContract review for the changed surfaces.\n\n## Interface Drift\n\n{}\n\n## Compatibility Concerns\n\nStatus: {}\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::ContractDrift),
                "- No contract drift inferred."
            ),
            if packet.findings_for(FindingCategory::ContractDrift).is_empty() {
                "no-contract-drift-detected"
            } else {
                "explicit-contract-drift"
            },
            if packet.findings_for(FindingCategory::ContractDrift).is_empty() {
                "- No compatibility concerns inferred from the reviewed diff."
            } else {
                "- Compatibility risk remains explicit until reviewer disposition is recorded."
            },
        ),
        "missing-tests.md" => format!(
            "# Missing Tests\n\n## Summary\n\nVerification coverage review for the diff.\n\n## Missing Invariant Checks\n\n{}\n\n## Missing Contract Checks\n\n{}\n\n## Weak or Mirrored Tests\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::MissingTests),
                "- No missing invariant checks inferred."
            ),
            if packet.findings_for(FindingCategory::MissingTests).is_empty() {
                "- No missing contract checks inferred."
            } else {
                "- Contract-facing changes should carry explicit test evidence."
            },
            if packet.findings_for(FindingCategory::MissingTests).is_empty() {
                "- Updated tests moved with the changed surface."
            } else {
                "- The current diff changes source files without parallel verification updates."
            },
        ),
        "decision-impact.md" => format!(
            "# Decision Impact\n\n## Summary\n\nDecision-impact review for the bounded diff.\n\n## Implied Decisions\n\n{}\n\n## Absent Decision Records\n\n{}\n\n## Reversibility Concerns\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::DecisionImpact),
                "- No hidden decision impact inferred."
            ),
            if packet.findings_for(FindingCategory::DecisionImpact).is_empty() {
                "- No absent decision records were inferred from the changed surfaces."
            } else {
                "- Structural or contract-facing changes imply decisions that are not yet explicitly accepted."
            },
            if packet.findings_for(FindingCategory::DecisionImpact).is_empty() {
                "- Reversibility concerns remain bounded."
            } else {
                "- Reverting the current change may require downstream coordination because high-impact surfaces changed."
            },
        ),
        "review-summary.md" => format!(
            "# Review Summary\n\n## Summary\n\nStructured review completed for `{}` against `{}`.\n\n## Evidence\n\n- Governed diff inspection and critique were both recorded before disposition.\n- Review artifacts remain derived from the persisted evidence bundle.\n\n## Severity\n\n- Overall severity: {}\n- Must-fix findings: {}\n- Review notes: {}\n\n## Must-Fix Findings\n\n{}\n\n## Accepted Risks\n\n{}\n\n## Final Disposition\n\nStatus: {}\n\nRationale: {}\n",
            packet.head_ref,
            packet.base_ref,
            summary_severity_label(packet),
            summary.must_fix_findings.len(),
            packet.note_findings().len(),
            render_findings(&packet.must_fix_findings(), "- No must-fix findings remain."),
            render_string_list(&summary.accepted_risks, "- No accepted risks recorded."),
            summary.disposition.as_str(),
            summary.rationale,
        ),
        other => render_markdown(other, &packet.inferred_intent),
    }
}

fn extract_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    extract_markdown_section(source, marker)
        .or_else(|| extract_inline_marker(source, normalized, marker))
}

fn extract_inline_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    let marker_with_colon = format!("{marker}:");
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let mut lines = remainder.lines();
    let line = lines.next()?.trim();
    if !line.is_empty() {
        return Some(line.to_string());
    }

    let mut section_lines = Vec::new();
    for next_line in lines {
        let trimmed = next_line.trim_end();
        let normalized_line = trimmed.trim();

        if normalized_line.is_empty() {
            if !section_lines.is_empty() {
                break;
            }
            continue;
        }

        if looks_like_inline_marker(normalized_line) || normalized_line.starts_with('#') {
            break;
        }

        section_lines.push(trimmed);
    }

    let section = trim_multiline_block(&section_lines.join("\n"));
    if section.is_empty() { None } else { Some(section) }
}

fn looks_like_inline_marker(line: &str) -> bool {
    if line.starts_with(['-', '*', '+']) {
        return false;
    }

    let Some((prefix, _)) = line.split_once(':') else {
        return false;
    };
    let prefix = prefix.trim();
    !prefix.is_empty()
        && prefix.chars().all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ' ' | '-' | '_'))
}

fn extract_markdown_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if is_section_boundary(next_line) {
                break;
            }

            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_multiline_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
    }

    None
}

fn is_matching_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

fn is_section_boundary(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#')
        || trimmed.starts_with("Generated framing:")
        || trimmed.starts_with("Critique evidence:")
        || trimmed.starts_with("Validation evidence:")
        || trimmed.starts_with("Mutation posture:")
}

fn trim_multiline_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
    }
}

fn render_surface_list(values: &[String], empty_message: &str) -> String {
    render_string_list(values, empty_message)
}

fn render_string_list(values: &[String], empty_message: &str) -> String {
    if values.is_empty() {
        empty_message.to_string()
    } else {
        values.iter().map(|value| format!("- {value}")).collect::<Vec<_>>().join("\n")
    }
}

fn render_findings(findings: &[&ReviewFinding], empty_message: &str) -> String {
    if findings.is_empty() {
        return empty_message.to_string();
    }

    findings
        .iter()
        .map(|finding| {
            format!(
                "- [{}] {}: {}\n  - Surfaces: {}",
                finding.severity.as_str(),
                finding.title,
                finding.details,
                if finding.changed_surfaces.is_empty() {
                    "none".to_string()
                } else {
                    finding.changed_surfaces.join(", ")
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_conventional_comments(packet: &ReviewPacket) -> String {
    if packet.findings.is_empty() {
        return "- thought: No findings were recorded for this review packet.".to_string();
    }

    packet
        .findings
        .iter()
        .map(|finding| {
            format!(
                "- {}: {}\n  - Why: {}\n  - Surfaces: {}",
                finding.conventional_comment_kind(),
                finding.title,
                finding.details,
                if finding.changed_surfaces.is_empty() {
                    "none".to_string()
                } else {
                    finding.changed_surfaces.join(", ")
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn ownership_breaks(packet: &ReviewPacket) -> String {
    let boundary_findings = packet.findings_for(FindingCategory::BoundaryCheck);
    if boundary_findings.is_empty() {
        "- No ownership breaks inferred from changed surfaces.".to_string()
    } else {
        "- Reviewer ownership is required before boundary-marked changes can be treated as acceptable output.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MISSING_AUTHORED_BODY_MARKER, extract_authored_h2_section, extract_marker,
        render_architecture_artifact, render_change_artifact, render_discovery_artifact,
        render_missing_authored_body_block, render_pr_review_artifact,
        render_requirements_artifact, render_requirements_artifact_from_evidence,
        render_review_artifact, render_system_shaping_artifact, render_verification_artifact,
        trim_markdown_list_prefix, verification_summary_excerpt,
    };
    use crate::review::findings::{FindingCategory, FindingSeverity, ReviewFinding, ReviewPacket};
    use crate::review::summary::{ReviewDisposition, ReviewSummary};

    #[test]
    fn extract_marker_prefers_markdown_section_over_inline_mentions() {
        let source = "# Change Brief\n\n## Change Surface\n- bounded module\n- stable interface\n\nMutation posture: propose bounded legacy transformation within declared change surface: workspace, adjacent modules";
        let normalized = source.to_lowercase();

        let marker = extract_marker(source, &normalized, "change surface").expect("change surface");

        assert_eq!(marker, "- bounded module\n- stable interface");
    }

    #[test]
    fn render_change_surface_preserves_markdown_bullets() {
        let source = "# Change Brief\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for null arguments.\n\n## Change Surface\n- Public API entrypoints\n- Debug logging only\n\n## Owner\nLead Eng\n\n## Risk Level\nlow-impact\n\n## Zone\ngreen\n";

        let rendered = render_change_artifact("change-surface.md", source, "");

        assert!(
            rendered
                .contains("## Change Surface\n\n- Public API entrypoints\n- Debug logging only")
        );
        assert!(rendered.contains("- Owner / risk / zone: `Lead Eng` / `low-impact` / `green`"));
    }

    #[test]
    fn render_change_validation_strategy_preserves_markdown_bullets() {
        let source = "# Change Brief\n\n## System Slice\nSchema validation\n\n## Intended Change\nAdd debug logging for null arguments.\n\n## Validation Strategy\n- Unit tests\n- Log assertion checks\n";

        let rendered = render_change_artifact("validation-strategy.md", source, "");

        assert!(
            rendered.contains("## Validation Strategy\n\n- Unit tests\n- Log assertion checks")
        );
    }

    #[test]
    fn render_requirements_artifacts_cover_named_templates_and_fallback() {
        let summary = "Bound the requirements work before planning";
        let authored = "# Requirements Brief\n\n## Constraints\n\n- Keep the implementation local-first and auditable.\n\n## Non-Negotiables\n\n- Preserve explicit human ownership.\n\n## Tradeoffs\n\n- Favoring governability reduces raw generation speed.\n\n## Consequences\n\n- The product will feel opinionated by design.\n";

        let constraints = render_requirements_artifact("constraints.md", summary);
        let fallback = render_requirements_artifact("custom-note.md", summary);
        let evidence = render_requirements_artifact_from_evidence(
            "tradeoffs.md",
            summary,
            authored,
            "generated framing",
            "critique note",
            "denied mutation request remained visible",
        );
        let missing = render_requirements_artifact_from_evidence(
            "problem-statement.md",
            summary,
            "# Requirements Brief\n\n## Problem\n\nBound the requirements work before planning.\n",
            "generated framing",
            "critique note",
            "denied mutation request remained visible",
        );

        assert!(constraints.contains("## Non-Negotiables"));
        assert!(constraints.contains("Risk and zone classification happen before generation."));
        assert!(fallback.starts_with(
            "# custom-note.md\n\n## Summary\n\nBound the requirements work before planning"
        ));
        assert!(
            evidence
                .contains("## Tradeoffs\n\n- Favoring governability reduces raw generation speed.")
        );
        assert!(
            evidence.contains("## Consequences\n\n- The product will feel opinionated by design.")
        );
        assert!(missing.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(missing.contains("`## Outcome`"));
    }

    #[test]
    fn authored_h2_extraction_requires_exact_heading_level_and_documented_aliases() {
        let near_miss = "# Requirements Brief\n\n### Problem\n\nThis should not count.\n";
        let alias = "# Requirements Brief\n\n## Out of Scope\n\n- No GUI\n";

        assert!(extract_authored_h2_section(near_miss, "Problem", &[]).is_none());
        assert_eq!(
            extract_authored_h2_section(alias, "Scope Cuts", &["Out of Scope"]),
            Some("- No GUI".to_string())
        );
    }

    #[test]
    fn missing_authored_body_block_names_the_canonical_heading() {
        let rendered = render_missing_authored_body_block("Outcome");

        assert!(rendered.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(rendered.contains("NOT CAPTURED - No `## Outcome` section was authored"));
    }

    #[test]
    fn render_change_artifact_reports_missing_context_and_default_metadata() {
        let source = "# Change Brief\n\n## System Slice\nSession repository\n\n## Intended Change\nStabilize resumable execution\n";

        let invariants = render_change_artifact("legacy-invariants.md", source, "");
        let decision = render_change_artifact("decision-record.md", source, "");

        assert!(invariants.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(invariants.contains("`## Legacy Invariants`"));
        assert!(decision.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(decision.contains("`## Decision Record`"));
        assert!(decision.contains("- Owner / risk / zone: `bounded-system-maintainer` / `unspecified-risk` / `unspecified-zone`"));
    }

    #[test]
    fn render_pr_review_artifacts_handle_empty_and_populated_findings() {
        let review_notes_packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );
        let review_notes_summary = ReviewSummary::from_packet(&review_notes_packet, false);
        let boundary = render_pr_review_artifact(
            "boundary-check.md",
            &review_notes_packet,
            &review_notes_summary,
        );

        assert!(boundary.contains("- No boundary findings detected."));
        assert!(boundary.contains("Status: no-structural-impact-detected"));

        let must_fix_packet = ReviewPacket {
            base_ref: "origin/main".to_string(),
            head_ref: "feature".to_string(),
            changed_surfaces: vec!["contracts/public-api.json".to_string()],
            inferred_intent: "Review contract drift on a public API change.".to_string(),
            surprising_surface_area: vec!["contracts/public-api.json".to_string()],
            findings: vec![
                ReviewFinding {
                    category: FindingCategory::ContractDrift,
                    severity: FindingSeverity::MustFix,
                    title: "Contract-facing files changed".to_string(),
                    details: "Compatibility drift needs explicit reviewer acceptance.".to_string(),
                    changed_surfaces: vec!["contracts/public-api.json".to_string()],
                },
                ReviewFinding {
                    category: FindingCategory::DecisionImpact,
                    severity: FindingSeverity::Note,
                    title: "Decision note".to_string(),
                    details: "A broader acceptance note should be recorded.".to_string(),
                    changed_surfaces: vec!["contracts/public-api.json".to_string()],
                },
            ],
        };
        let must_fix_summary = ReviewSummary {
            disposition: ReviewDisposition::AcceptedWithApproval,
            rationale: "Explicit reviewer approval accepted the remaining must-fix findings with named ownership.".to_string(),
            must_fix_findings: vec!["Contract-facing files changed".to_string()],
            accepted_risks: vec!["Decision note".to_string()],
        };

        let contract =
            render_pr_review_artifact("contract-drift.md", &must_fix_packet, &must_fix_summary);
        let conventional_comments = render_pr_review_artifact(
            "conventional-comments.md",
            &must_fix_packet,
            &must_fix_summary,
        );
        let summary =
            render_pr_review_artifact("review-summary.md", &must_fix_packet, &must_fix_summary);

        assert!(contract.contains("Status: explicit-contract-drift"));
        assert!(contract.contains(
            "Compatibility risk remains explicit until reviewer disposition is recorded."
        ));
        assert!(conventional_comments.contains("issue:"));
        assert!(conventional_comments.contains("thought:"));
        assert!(conventional_comments.contains("contracts/public-api.json"));
        assert!(summary.contains("Overall severity: must-fix"));
        assert!(summary.contains("Status: accepted-with-approval"));
        assert!(summary.contains("- Decision note"));
    }

    #[test]
    fn render_review_artifacts_distinguish_ready_and_pending_disposition_packets() {
        let ready = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\nReview target stays bounded to a named service boundary.",
            "Review packet preserved evidence basis, boundary findings, and decision impact.",
            "Challenge the proposed review packet for evidence coverage, hidden scope growth, ownership clarity, and acceptance rationale.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );
        let pending = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\nMissing evidence remains for rollback coverage on this package.",
            "Must-fix review note remains open for the release boundary.",
            "Scope drift introduced a must-fix follow-up before acceptance.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );

        assert!(ready.contains("Status: ready-with-review-notes"));
        assert!(pending.contains("Status: awaiting-disposition"));
    }

    #[test]
    fn render_verification_artifacts_distinguish_supported_and_blocked_packets() {
        let supported = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\nClaims Under Test: rollback remains bounded and auditable.\nEvidence Basis: contract notes and repository checks.\nContract Surface: rollback metadata stays explicit.",
            "## Claims Under Test\n\n- rollback remains bounded and auditable\n\n## Evidence Basis\n\n- contract notes\n- repository checks\n\n## Contract Assumptions\n\n- rollback metadata stays explicit",
            "## Challenge Findings\n\n- No additional challenge findings were recorded beyond the authored verification packet.\n\n## Contradictions\n\n- No direct contradictions were identified from the current verification packet.\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n\n## Rejected Claims\n\n- No rejected claims were inferred from the current verification target.\n\n## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification target.\n\n## Required Follow-up\n\n- Keep the verification packet attached to downstream release or approval discussion.\n\n## Overall Verdict\n\nStatus: supported\nRationale: No explicit contradiction or proof gap remained in the normalized packet.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );
        let blocked = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\nClaims Under Test: an unresolved contradiction remains between the authored claim and the captured contract.\nEvidence Basis: unsupported rollback guarantee still lacks concrete proof.",
            "## Claims Under Test\n\n- an unresolved contradiction remains between the authored claim and the captured contract\n\n## Evidence Basis\n\n- unsupported rollback guarantee still lacks concrete proof\n\n## Contract Assumptions\n\n- rollback metadata stays explicit",
            "## Challenge Findings\n\n- The authored claim already signals a contradiction or missing-evidence path.\n\n## Contradictions\n\n- The authored claim under test already records a contradiction or unresolved support gap.\n\n## Verified Claims\n\n- The evidence basis is explicit enough for downstream inspection and follow-up.\n\n## Rejected Claims\n\n- Still unsupported from the current packet: an unresolved contradiction remains between the authored claim and the captured contract\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- Resolve the contradiction before treating the packet as supported.\n\n## Required Follow-up\n\n- Resolve the contradictions or proof gaps before treating the packet as supported.\n\n## Overall Verdict\n\nStatus: unsupported\nRationale: The packet still carries unresolved findings against the named claim.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs, tests/lib_test.rs",
        );

        assert!(supported.contains("Status: supported"));
        assert!(blocked.contains("Status: unsupported"));
    }

    #[test]
    fn render_verification_artifacts_map_structured_sections_without_prompt_echo() {
        let context = "# Verification Brief\n\n## Claims Under Test\n- CSS sanitization blocks hostile style execution\n- CSS URL filtering stays bounded\n\n## Evidence Basis\n- README assertions\n- parser tests\n\n## Contract Surface\n- style attributes and CSS URLs stay conservative\n\n## Challenge Focus\n- look for unsupported jumps from parser coverage to full CSS-XSS coverage";
        let generation = "## Claims Under Test\n\n- CSS sanitization blocks hostile style execution\n- CSS URL filtering stays bounded\n\n## Evidence Basis\n\n- README assertions\n- parser tests\n\n## Contract Assumptions\n\n- style attributes and CSS URLs stay conservative";
        let critique = "## Challenge Findings\n\n- Authored challenge focus remains open until explicit evidence answers it: look for unsupported jumps from parser coverage to full CSS-XSS coverage\n\n## Contradictions\n\n- The packet still needs explicit adversarial proof for its broadest CSS-XSS assurances.\n\n## Verified Claims\n\n- The evidence basis is explicit enough for downstream inspection and follow-up.\n\n## Rejected Claims\n\n- Still unsupported from the current packet: CSS sanitization blocks hostile style execution\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- Answer this authored challenge focus with explicit evidence or narrow the affected claim: look for unsupported jumps from parser coverage to full CSS-XSS coverage\n\n## Required Follow-up\n\n- Address each authored challenge-focus item with explicit evidence or narrow the affected claim.\n\n## Overall Verdict\n\nStatus: unsupported\nRationale: The packet still carries unresolved CSS evidence gaps.";

        let contract = render_verification_artifact(
            "contract-matrix.md",
            context,
            generation,
            critique,
            "Validation tool reviewed tracked repository surfaces: README.md, src/css.rs",
        );
        let report = render_verification_artifact(
            "verification-report.md",
            context,
            generation,
            critique,
            "Validation tool reviewed tracked repository surfaces: README.md, src/css.rs",
        );

        assert!(contract.contains("style attributes and CSS URLs stay conservative"));
        assert!(report.contains("Still unsupported from the current packet: CSS sanitization blocks hostile style execution"));
        assert!(
            report.contains("Rationale: The packet still carries unresolved CSS evidence gaps.")
        );
        assert!(!report.contains("Challenge the verification packet for claim support strength"));
        assert!(!report.contains("# Verification Brief Claims Under Test:"));
        assert!(report.contains("- Claims under test: CSS sanitization blocks hostile style execution; CSS URL filtering stays bounded"));
        assert!(report.contains("- Evidence basis: README assertions; parser tests"));
        assert!(report.contains("- Verdict: unsupported"));
    }

    #[test]
    fn verification_summary_excerpt_skips_headings_status_and_rationale() {
        let summary = verification_summary_excerpt(
            "# Verification Brief\n\n## Claims Under Test\n- claim one\n- claim two\nStatus: unsupported\nRationale: still open",
        )
        .expect("summary excerpt");

        assert_eq!(summary, "claim one; claim two");
    }

    #[test]
    fn verification_summary_excerpt_limits_extra_list_items() {
        let summary = verification_summary_excerpt(
            "1. first evidence anchor\n2. second evidence anchor\n3. third evidence anchor",
        )
        .expect("summary excerpt");

        assert_eq!(summary, "first evidence anchor; second evidence anchor; +1 more");
    }

    #[test]
    fn trim_markdown_list_prefix_handles_numbered_and_unordered_lists() {
        assert_eq!(trim_markdown_list_prefix("- first claim"), "first claim");
        assert_eq!(trim_markdown_list_prefix("* second claim"), "second claim");
        assert_eq!(trim_markdown_list_prefix("12. numbered claim"), "numbered claim");
        assert_eq!(trim_markdown_list_prefix("plain sentence"), "plain sentence");
    }

    #[test]
    fn render_verification_artifact_summary_omits_challenge_focus_when_supported() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Claims Under Test\n- rollback remains bounded\n\n## Evidence Basis\n- audit log checks\n\n## Contract Surface\n- rollback metadata stays explicit\n\n## Challenge Focus\n- verify cross-module ownership stays explicit",
            "## Claims Under Test\n\n- rollback remains bounded\n\n## Evidence Basis\n\n- audit log checks\n\n## Contract Assumptions\n\n- rollback metadata stays explicit",
            "## Challenge Findings\n\n- No additional challenge findings were recorded beyond the authored verification packet.\n\n## Contradictions\n\n- No direct contradictions were identified from the current verification packet.\n\n## Verified Claims\n\n- rollback remains bounded\n\n## Rejected Claims\n\n- No rejected claims were inferred from the current verification target.\n\n## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification target.\n\n## Required Follow-up\n\n- Keep the verification packet attached to downstream review.\n\n## Overall Verdict\n\nStatus: supported\nRationale: The packet is sufficiently supported.",
            "Validation tool reviewed tracked repository surfaces: src/lib.rs",
        );

        assert!(report.contains("- Open findings: none recorded."));
        assert!(!report.contains("- Challenge focus:"));
    }

    #[test]
    fn render_verification_artifact_summary_includes_challenge_focus_when_unresolved() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Claims Under Test\n- css sanitization blocks hostile style execution\n\n## Evidence Basis\n- parser tests\n\n## Contract Surface\n- css URLs stay conservative\n\n## Challenge Focus\n- prove broad css-xss coverage instead of parser-only confidence",
            "## Claims Under Test\n\n- css sanitization blocks hostile style execution\n\n## Evidence Basis\n\n- parser tests\n\n## Contract Assumptions\n\n- css URLs stay conservative",
            "## Challenge Findings\n\n- Broad css-xss coverage remains unsupported.\n\n## Contradictions\n\n- Parser coverage does not prove the broader claim.\n\n## Verified Claims\n\n- parser tests are present\n\n## Rejected Claims\n\n- css sanitization blocks hostile style execution\n\n## Open Findings\n\nStatus: unresolved-findings-open\n\n- Explicit contradiction remains open.\n\n## Required Follow-up\n\n- Narrow the claim or add adversarial proof.\n\n## Overall Verdict\n\nStatus: unsupported\nRationale: The broadest claim is still unsupported.",
            "Validation tool reviewed tracked repository surfaces: README.md, src/css.rs",
        );

        assert!(report.contains(
            "- Challenge focus: prove broad css-xss coverage instead of parser-only confidence"
        ));
        assert!(
            report
                .contains("- Open findings: unresolved follow-up remains recorded in this packet.")
        );
    }

    #[test]
    fn render_review_artifacts_preserve_multiline_bullets() {
        let artifact = render_review_artifact(
            "missing-evidence.md",
            "Review packet remains bounded.",
            "Decision impact remains bounded.",
            "Missing evidence:\n- Missing rollback rehearsal\n- Missing owner sign-off",
            "- Validation confirmed only partial repository coverage\n- Follow-up evidence still required",
        );

        assert!(
            artifact.contains(
                "Missing evidence:\n- Missing rollback rehearsal\n- Missing owner sign-off"
            )
        );
        assert!(artifact.contains(
            "- Validation confirmed only partial repository coverage\n- Follow-up evidence still required"
        ));
        assert!(!artifact.contains("- Missing rollback rehearsal - Missing owner sign-off"));
    }

    #[test]
    fn render_verification_artifacts_preserve_multiline_sections() {
        let invariants = render_verification_artifact(
            "invariants-checklist.md",
            "Verification remains bounded.",
            "## Claims Under Test\n- Claim one\n- Claim two",
            "## Contradictions\n- First contradiction\n- Second contradiction",
            "- Validation requires an independent follow-up",
        );
        let review = render_verification_artifact(
            "adversarial-review.md",
            "Verification remains bounded.",
            "## Claims Under Test\n- Claim one\n- Claim two",
            "## Contradictions\n- First contradiction\n- Second contradiction",
            "- Validation requires an independent follow-up",
        );

        assert!(
            review.contains("## Contradictions\n\n- First contradiction\n- Second contradiction")
        );
        assert!(invariants.contains("## Claims Under Test\n\n- Claim one\n- Claim two"));
        assert!(!review.contains("- First contradiction - Second contradiction"));
    }

    #[test]
    fn render_analysis_mode_artifacts_include_required_sections() {
        let discovery = render_discovery_artifact(
            "context-boundary.md",
            "# Discovery Brief\n\n## Problem Domain\n\nExplore a bounded notification routing problem.\n\n## Repo Surface\n\n- src/router.rs\n- tests/router_contract.rs\n\n## Immediate Tensions\n\n- Retry ownership is still unclear.\n\n## Downstream Handoff\n\nTranslate this discovery packet into architecture mode with named boundaries.\n\n## Unknowns\n\n- Which caller owns retry policy?\n\n## Assumptions\n\n- Routing ownership should remain explicit.\n\n## Validation Targets\n\n- Check src/router.rs and tests/router_contract.rs.\n\n## Confidence Levels\n\n- Medium until retry ownership is explicit.\n\n## In-Scope Context\n\n- Notification routing only.\n\n## Out-of-Scope Context\n\n- No implementation edits.\n\n## Translation Trigger\n\nTranslate this discovery packet into architecture mode with named boundaries.\n\n## Options\n\n1. Stay in discovery.\n\n## Constraints\n\n- Preserve current routing ownership boundaries.\n\n## Recommended Direction\n\nStay bounded to routing ownership.\n\n## Next-Phase Shape\n\nUse architecture mode to capture boundary choices.\n\n## Pressure Points\n\n- Retry ownership remains unresolved.\n\n## Blocking Decisions\n\n- Decide where retry ownership lives.\n\n## Open Questions\n\n- Which caller owns retry policy?\n\n## Recommended Owner\n\n- routing-architect\n",
        );
        let system_shaping = render_system_shaping_artifact(
            "system-shape.md",
            "Shape a new notification delivery capability.",
            "Separate ingest, routing, and delivery responsibilities.",
            "Keep delivery phase boundaries explicit and reversible.",
        );
        let architecture = render_architecture_artifact(
            "tradeoff-matrix.md",
            "Evaluate architectural boundaries for routing state.",
            "Compare centralized and partitioned routing designs.",
            "Partitioned routing better preserves ownership boundaries.",
        );

        assert!(discovery.contains("## In-Scope Context"));
        assert!(discovery.contains("## Repo Surface"));
        assert!(discovery.contains("## Translation Trigger"));
        assert!(system_shaping.contains("## System Shape"));
        assert!(system_shaping.contains("## Boundary Decisions"));
        assert!(architecture.contains("## Evaluation Criteria"));
        assert!(architecture.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(architecture.contains("`## Why Not The Others`"));
    }
}
