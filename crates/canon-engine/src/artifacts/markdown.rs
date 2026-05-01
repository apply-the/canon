use crate::domain::run::BacklogPlanningContext;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;
use crate::review::findings::{FindingCategory, ReviewFinding, ReviewPacket};
use crate::review::summary::{ReviewSummary, summary_severity_label};

struct AuthoredSectionSpec<'a> {
    canonical_heading: &'a str,
    aliases: &'a [&'a str],
}

pub const MISSING_AUTHORED_BODY_MARKER: &str = "## Missing Authored Body";
pub const MISSING_AUTHORED_DECISION_MARKER: &str = "## Missing Authored Decision";

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

fn render_missing_authored_decision_block(canonical_heading: &str, guidance: &str) -> String {
    format!(
        "{MISSING_AUTHORED_DECISION_MARKER}\n\nDecision required - No `## {canonical_heading}` section was authored in the supplied brief.\n{guidance}\n\n## {canonical_heading}\n\nDecision required - maintainer confirmation is still missing for this section."
    )
}

fn render_authored_decision_section(
    authored_source: &str,
    canonical_heading: &str,
    aliases: &[&str],
    guidance: &str,
) -> String {
    match extract_authored_h2_section(authored_source, canonical_heading, aliases) {
        Some(body) => format!("## {}\n\n{}", canonical_heading, body),
        None => format!(
            "## {}\n\n{}",
            canonical_heading,
            render_missing_authored_decision_block(canonical_heading, guidance)
        ),
    }
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

pub fn render_security_assessment_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let assessment_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Assessment Scope",
        &[],
        &["assessment scope"],
    )
    .unwrap_or_else(|| "assessment scope not yet authored".to_string());
    let in_scope_assets = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "In-Scope Assets",
        &[],
        &["in-scope assets"],
    )
    .unwrap_or_else(|| "in-scope assets not yet authored".to_string());
    let summary = format!(
        "Bounded security assessment for {} covering {}.",
        truncate_context_excerpt(&assessment_scope, 80),
        truncate_context_excerpt(&in_scope_assets, 80)
    );

    match file_name {
        "assessment-overview.md" => render_authored_artifact(
            "Assessment Overview",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assessment Scope", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "In-Scope Assets", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Trust Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Out Of Scope", aliases: &[] },
            ],
        ),
        "threat-model.md" => render_authored_artifact(
            "Threat Model",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Threat Inventory", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Attacker Goals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Threats", aliases: &[] },
            ],
        ),
        "risk-register.md" => render_authored_artifact(
            "Risk Register",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Risk Findings", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Likelihood And Impact", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Proposed Owners", aliases: &[] },
            ],
        ),
        "mitigations.md" => render_authored_artifact(
            "Mitigations",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Recommended Controls", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Tradeoffs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Sequencing Notes", aliases: &[] },
            ],
        ),
        "assumptions-and-gaps.md" => render_authored_artifact(
            "Assumptions And Gaps",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assumptions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Evidence Gaps", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Unobservable Surfaces", aliases: &[] },
            ],
        ),
        "compliance-anchors.md" => render_authored_artifact(
            "Compliance Anchors",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Applicable Standards", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Control Families", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Scope Limits", aliases: &[] },
            ],
        ),
        "assessment-evidence.md" => render_authored_artifact(
            "Assessment Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Source Inputs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Independent Checks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Verification", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_system_assessment_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let assessment_objective = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Assessment Objective",
        &[],
        &["assessment objective"],
    )
    .unwrap_or_else(|| "assessment objective not yet authored".to_string());
    let stakeholders = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Stakeholders",
        &[],
        &["stakeholders"],
    )
    .unwrap_or_else(|| "stakeholders not yet authored".to_string());
    let summary = format!(
        "Bounded system assessment for {} with reader context {}.",
        truncate_context_excerpt(&assessment_objective, 80),
        truncate_context_excerpt(&stakeholders, 80)
    );

    match file_name {
        "assessment-overview.md" => render_authored_artifact(
            "Assessment Overview",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assessment Objective", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Stakeholders", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Primary Concerns", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Assessment Posture", aliases: &[] },
            ],
        ),
        "coverage-map.md" => render_authored_artifact(
            "Coverage Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Stakeholder Concerns", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Assessed Views", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Partial Or Skipped Coverage",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Confidence By Surface", aliases: &[] },
            ],
        ),
        "asset-inventory.md" => render_authored_artifact(
            "Asset Inventory",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assessed Assets", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Critical Dependencies", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Notes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ownership Signals", aliases: &[] },
            ],
        ),
        "functional-view.md" => render_authored_artifact(
            "Functional View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Responsibilities", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Primary Flows", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Observed Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Notes", aliases: &[] },
            ],
        ),
        "component-view.md" => render_authored_artifact(
            "Component View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Components", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Responsibilities", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Interfaces", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Notes", aliases: &[] },
            ],
        ),
        "deployment-view.md" => render_authored_artifact(
            "Deployment View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Execution Environments", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Network And Runtime Boundaries",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Deployment Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Coverage Gaps", aliases: &[] },
            ],
        ),
        "technology-view.md" => render_authored_artifact(
            "Technology View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Technology Stack", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Platform Dependencies", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Version Or Lifecycle Signals",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Evidence Gaps", aliases: &[] },
            ],
        ),
        "integration-view.md" => render_authored_artifact(
            "Integration View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Integrations", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Data Exchanges", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Trust And Failure Boundaries",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Inference Notes", aliases: &[] },
            ],
        ),
        "risk-register.md" => render_authored_artifact(
            "Risk Register",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Observed Risks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Risk Triggers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Impact Notes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Likely Follow-On Modes", aliases: &[] },
            ],
        ),
        "assessment-evidence.md" => render_authored_artifact(
            "Assessment Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Observed Findings",
                    aliases: &["FACT Findings"],
                },
                AuthoredSectionSpec {
                    canonical_heading: "Inferred Findings",
                    aliases: &["INFERENCE Findings"],
                },
                AuthoredSectionSpec {
                    canonical_heading: "Assessment Gaps",
                    aliases: &["GAP Findings"],
                },
                AuthoredSectionSpec { canonical_heading: "Evidence Sources", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

pub fn render_supply_chain_analysis_artifact(file_name: &str, brief_summary: &str) -> String {
    let normalized = brief_summary.to_lowercase();
    let declared_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Declared Scope",
        &[],
        &["declared scope"],
    )
    .unwrap_or_else(|| "declared scope not yet authored".to_string());
    let ecosystems_in_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Ecosystems In Scope",
        &[],
        &["ecosystems in scope"],
    )
    .unwrap_or_else(|| "ecosystems in scope not yet authored".to_string());
    let summary = format!(
        "Bounded supply-chain analysis for {} across {}.",
        truncate_context_excerpt(&declared_scope, 80),
        truncate_context_excerpt(&ecosystems_in_scope, 80)
    );

    match file_name {
        "analysis-overview.md" => format!(
            "# Analysis Overview\n\n## Summary\n\n{summary}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n",
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec { canonical_heading: "Declared Scope", aliases: &[] }
            ),
            render_authored_decision_section(
                brief_summary,
                "Licensing Posture",
                &[],
                "Record the repository licensing posture explicitly and rerun."
            ),
            render_authored_decision_section(
                brief_summary,
                "Distribution Model",
                &[],
                "Record whether the analyzed dependencies are distributed externally or internal-only and rerun."
            ),
            render_authored_decision_section(
                brief_summary,
                "Ecosystems In Scope",
                &[],
                "Record which ecosystems remain in scope for the packet and rerun."
            ),
            render_authored_decision_section(
                brief_summary,
                "Out Of Scope Components",
                &[],
                "Record the explicit out-of-scope components and rerun."
            ),
        ),
        "sbom-bundle.md" => format!(
            "# SBOM Bundle\n\n## Summary\n\n{summary}\n\n{}\n\n{}\n\n{}\n\n{}\n",
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec {
                    canonical_heading: "Scanner Selection Rationale",
                    aliases: &[],
                }
            ),
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec { canonical_heading: "SBOM Outputs", aliases: &[] }
            ),
            render_authored_decision_section(
                brief_summary,
                "Scanner Decisions",
                &[],
                "Record non-OSS tool policy and any installed, skipped, or replaced scanner decisions, then rerun."
            ),
            render_supply_chain_coverage_gaps_section(brief_summary),
        ),
        "vulnerability-triage.md" => render_authored_artifact(
            "Vulnerability Triage",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Findings By Severity", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Exploitability Notes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Triage Decisions", aliases: &[] },
            ],
        ),
        "license-compliance.md" => render_authored_artifact(
            "License Compliance",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Compatibility Classes", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Flagged Incompatibilities",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Obligations", aliases: &[] },
            ],
        ),
        "legacy-posture.md" => render_authored_artifact(
            "Legacy Posture",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Outdated Dependencies", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "End Of Life Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Abandonment Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Modernization Slices", aliases: &[] },
            ],
        ),
        "policy-decisions.md" => format!(
            "# Policy Decisions\n\n## Summary\n\n{summary}\n\n{}\n\n{}\n\n{}\n",
            render_authored_decision_section(
                brief_summary,
                "Scanner Decisions",
                &[],
                "Record non-OSS tool policy and any installed, skipped, or replaced scanner decisions, then rerun."
            ),
            render_supply_chain_coverage_gaps_section(brief_summary),
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec { canonical_heading: "Deferred Verification", aliases: &[] }
            ),
        ),
        "analysis-evidence.md" => render_authored_artifact(
            "Analysis Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Source Inputs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Independent Checks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Verification", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

fn render_supply_chain_coverage_gaps_section(authored_source: &str) -> String {
    if let Some(body) = extract_authored_h2_section(authored_source, "Coverage Gaps", &[]) {
        return format!("## Coverage Gaps\n\n{body}");
    }

    if let Some(scanner_decisions) =
        extract_authored_h2_section(authored_source, "Scanner Decisions", &[])
    {
        let normalized = scanner_decisions.to_lowercase();
        if normalized.contains("skipped") || normalized.contains("replaced") {
            return format!(
                "## Coverage Gaps\n\nCoverage gap derived from recorded scanner decisions.\n\n{}\n\n- Impacted artifacts: sbom-bundle.md, vulnerability-triage.md, license-compliance.md, legacy-posture.md, policy-decisions.md\n- Next action: install the missing scanner or document an approved replacement and rerun the packet.",
                truncate_context_excerpt(&scanner_decisions, 320)
            );
        }
    }

    render_missing_authored_body_block("Coverage Gaps")
}

pub fn render_migration_artifact(file_name: &str, brief_summary: &str) -> String {
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
                AuthoredSectionSpec { canonical_heading: "Recommendation", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ecosystem Health", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Decisions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Approval Notes", aliases: &[] },
            ],
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
                AuthoredSectionSpec { canonical_heading: "Options Matrix", aliases: &[] },
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

pub fn render_review_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
    _validation_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let review_target = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Review Target",
        &[],
        &["review target"],
    )
    .unwrap_or_else(|| "review target not yet authored".to_string());
    let evidence_basis = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Evidence Basis",
        &[],
        &["evidence basis"],
    )
    .unwrap_or_else(|| "evidence basis not yet authored".to_string());
    let summary = format!(
        "- Review target: {}\n- Evidence basis: {}",
        truncate_context_excerpt(&review_target, 120),
        truncate_context_excerpt(&evidence_basis, 120),
    );

    match file_name {
        "review-brief.md" => render_authored_artifact(
            "Review Brief",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Review Target", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Evidence Basis", aliases: &[] },
            ],
        ),
        "boundary-assessment.md" => render_authored_artifact(
            "Boundary Assessment",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Boundary Findings",
                    aliases: &["Boundary Concern"],
                },
                AuthoredSectionSpec { canonical_heading: "Ownership Notes", aliases: &[] },
            ],
        ),
        "missing-evidence.md" => render_authored_artifact(
            "Missing Evidence",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Missing Evidence",
                    aliases: &["Open Concern"],
                },
                AuthoredSectionSpec { canonical_heading: "Collection Priorities", aliases: &[] },
            ],
        ),
        "decision-impact.md" => render_authored_artifact(
            "Decision Impact",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Decision Impact",
                    aliases: &["Pending Decision"],
                },
                AuthoredSectionSpec { canonical_heading: "Reversibility Concerns", aliases: &[] },
            ],
        ),
        "review-disposition.md" => render_authored_artifact(
            "Review Disposition",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Final Disposition", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Accepted Risks", aliases: &[] },
            ],
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_verification_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
    _validation_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let claims_under_test = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Claims Under Test",
        &[],
        &["claims under test"],
    )
    .unwrap_or_else(|| "claims under test not yet authored".to_string());
    let contract_assumptions = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Contract Assumptions",
        &["Contract Surface"],
        &["contract assumptions", "contract surface"],
    )
    .unwrap_or_else(|| "contract assumptions not yet authored".to_string());
    let overall_verdict = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Overall Verdict",
        &[],
        &["overall verdict"],
    )
    .unwrap_or_else(|| "overall verdict not yet authored".to_string());
    let verification_summary = format!(
        "- Claims under test: {}\n- Contract assumptions: {}\n- Verdict: {}",
        truncate_context_excerpt(&claims_under_test, 120),
        truncate_context_excerpt(&contract_assumptions, 120),
        truncate_context_excerpt(&overall_verdict, 120),
    );

    match file_name {
        "invariants-checklist.md" => render_authored_artifact(
            "Invariants Checklist",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Claims Under Test", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Invariant Checks",
                    aliases: &["Risk Boundary"],
                },
            ],
        ),
        "contract-matrix.md" => render_authored_artifact(
            "Contract Matrix",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Contract Assumptions",
                    aliases: &["Contract Surface"],
                },
                AuthoredSectionSpec { canonical_heading: "Verification Outcome", aliases: &[] },
            ],
        ),
        "adversarial-review.md" => render_authored_artifact(
            "Adversarial Review",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Challenge Findings",
                    aliases: &["Challenge Focus"],
                },
                AuthoredSectionSpec { canonical_heading: "Contradictions", aliases: &[] },
            ],
        ),
        "verification-report.md" => render_authored_artifact(
            "Verification Report",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Verified Claims", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Rejected Claims", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Overall Verdict", aliases: &[] },
            ],
        ),
        "unresolved-findings.md" => render_authored_artifact(
            "Unresolved Findings",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Open Findings", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Required Follow-Up",
                    aliases: &["Required Follow-up"],
                },
            ],
        ),
        other => render_markdown(other, context_summary),
    }
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
        render_incident_artifact, render_migration_artifact, render_missing_authored_body_block,
        render_pr_review_artifact, render_requirements_artifact,
        render_requirements_artifact_from_evidence, render_review_artifact,
        render_system_shaping_artifact, render_verification_artifact,
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
    fn render_review_artifacts_preserve_authored_status_sections() {
        let disposition = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\n## Review Target\n\n- bounded service boundary package.\n\n## Evidence Basis\n\n- owned interfaces, current tests, and rollback notes.\n\n## Final Disposition\n\nStatus: ready-with-review-notes\n\nRationale: the review packet is bounded enough for downstream inspection.\n\n## Accepted Risks\n\n- residual review notes remain bounded to this package.",
            "",
            "",
            "",
        );

        assert!(disposition.contains("## Final Disposition\n\nStatus: ready-with-review-notes"));
        assert!(disposition.contains(
            "## Accepted Risks\n\n- residual review notes remain bounded to this package."
        ));
    }

    #[test]
    fn render_review_artifacts_emit_missing_marker_for_absent_final_disposition() {
        let disposition = render_review_artifact(
            "review-disposition.md",
            "# Review Brief\n\n## Accepted Risks\n\n- residual review notes remain bounded to this package.",
            "",
            "",
            "",
        );

        assert!(disposition.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(disposition.contains("`## Final Disposition`"));
    }

    #[test]
    fn render_verification_artifacts_preserve_authored_status_sections() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Claims Under Test\n\n- rollback remains bounded and auditable\n\n## Contract Assumptions\n\n- rollback metadata remains explicit\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: supported\n\nRationale: the current evidence covers the authored claim set.",
            "",
            "",
            "",
        );

        assert!(report.contains("## Overall Verdict\n\nStatus: supported"));
        assert!(report.contains("## Verified Claims\n\n- rollback remains bounded and auditable"));
    }

    #[test]
    fn render_verification_artifacts_emit_missing_marker_for_absent_overall_verdict() {
        let report = render_verification_artifact(
            "verification-report.md",
            "# Verification Brief\n\n## Verified Claims\n\n- rollback remains bounded and auditable\n\n## Rejected Claims\n\n- none recorded",
            "",
            "",
            "",
        );

        assert!(report.contains(MISSING_AUTHORED_BODY_MARKER));
        assert!(report.contains("`## Overall Verdict`"));
    }

    #[test]
    fn render_verification_artifacts_preserve_multiline_sections() {
        let review = render_verification_artifact(
            "adversarial-review.md",
            "# Verification Brief\n\n## Challenge Findings\n\n- First challenge finding\n- Second challenge finding\n\n## Contradictions\n\n- First contradiction\n- Second contradiction",
            "",
            "",
            "",
        );

        assert!(
            review.contains("## Contradictions\n\n- First contradiction\n- Second contradiction")
        );
        assert!(review.contains(
            "## Challenge Findings\n\n- First challenge finding\n- Second challenge finding"
        ));
        assert!(!review.contains("- First contradiction - Second contradiction"));
    }

    #[test]
    fn render_incident_and_migration_artifacts_preserve_all_named_sections() {
        let incident_source = "# Incident Brief\n\n## Incident Scope\n\npayments-api and checkout flow only.\n\n## Trigger And Current State\n\nelevated 5xx responses after the deploy.\n\n## Operational Constraints\n\n- no autonomous remediation\n\n## Known Facts\n\n- rollback remains available\n\n## Working Hypotheses\n\n- retry amplification is exhausting the service\n\n## Evidence Gaps\n\n- saturation evidence is incomplete\n\n## Impacted Surfaces\n\n- payments-api\n\n## Propagation Paths\n\n- checkout request path\n\n## Confidence And Unknowns\n\n- medium confidence\n\n## Immediate Actions\n\n- disable retries\n\n## Ordered Sequence\n\n1. capture blast radius\n2. disable retries\n\n## Stop Conditions\n\n- error rate stabilizes\n\n## Decision Points\n\n- decide whether rollback is still required\n\n## Approved Actions\n\n- disable retries in the bounded surface\n\n## Deferred Actions\n\n- schema changes stay out of scope\n\n## Verification Checks\n\n- confirm 5xx rate drops\n\n## Release Readiness\n\n- remain recommendation-only until owner approval\n\n## Follow-Up Work\n\n- add a saturation dashboard\n";

        let hypothesis = render_incident_artifact("hypothesis-log.md", incident_source);
        let blast_radius = render_incident_artifact("blast-radius-map.md", incident_source);
        let containment = render_incident_artifact("containment-plan.md", incident_source);
        let decision = render_incident_artifact("incident-decision-record.md", incident_source);
        let follow_up = render_incident_artifact("follow-up-verification.md", incident_source);
        let incident_fallback = render_incident_artifact("custom-incident.md", incident_source);

        assert!(hypothesis.contains("## Known Facts\n\n- rollback remains available"));
        assert!(blast_radius.contains("## Propagation Paths\n\n- checkout request path"));
        assert!(containment.contains("## Stop Conditions\n\n- error rate stabilizes"));
        assert!(
            decision.contains("## Decision Points\n\n- decide whether rollback is still required")
        );
        assert!(
            follow_up.contains(
                "## Release Readiness\n\n- remain recommendation-only until owner approval"
            )
        );
        assert!(
            incident_fallback.starts_with("# custom-incident.md\n\n## Summary\n\n# Incident Brief")
        );

        let migration_source = "# Migration Brief\n\n## Current State\n\nauth-v1 serves login and token refresh traffic.\n\n## Target State\n\nauth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\nlogin and token refresh only.\n\n## Guaranteed Compatibility\n\n- existing tokens continue to validate\n\n## Temporary Incompatibilities\n\n- admin reporting stays on v1\n\n## Coexistence Rules\n\n- dual-write session metadata\n\n## Ordered Steps\n\n1. enable shadow reads\n2. start dual-write\n\n## Parallelizable Work\n\n- docs and dashboards can update in parallel\n\n## Cutover Criteria\n\n- token validation remains stable\n\n## Rollback Triggers\n\n- elevated login errors\n\n## Fallback Paths\n\n- route bounded traffic back to auth-v1\n\n## Re-Entry Criteria\n\n- regressions are resolved and revalidated\n\n## Verification Checks\n\n- login and token validation pass against auth-v2\n\n## Residual Risks\n\n- admin reporting remains temporarily inconsistent\n\n## Release Readiness\n\n- keep recommendation-only posture until owner accepts the packet\n\n## Migration Decisions\n\n- retain dual-write during cutover\n\n## Deferred Decisions\n\n- move admin reporting later\n\n## Approval Notes\n\n- migration lead sign-off is required\n";

        let compatibility = render_migration_artifact("compatibility-matrix.md", migration_source);
        let sequencing = render_migration_artifact("sequencing-plan.md", migration_source);
        let fallback = render_migration_artifact("fallback-plan.md", migration_source);
        let verification =
            render_migration_artifact("migration-verification-report.md", migration_source);
        let decision = render_migration_artifact("decision-record.md", migration_source);
        let migration_fallback = render_migration_artifact("custom-migration.md", migration_source);

        assert!(compatibility.contains("## Coexistence Rules\n\n- dual-write session metadata"));
        assert!(sequencing.contains("## Cutover Criteria\n\n- token validation remains stable"));
        assert!(fallback.contains("## Fallback Paths\n\n- route bounded traffic back to auth-v1"));
        assert!(
            verification.contains(
                "## Residual Risks\n\n- admin reporting remains temporarily inconsistent"
            )
        );
        assert!(decision.contains("## Approval Notes\n\n- migration lead sign-off is required"));
        assert!(
            migration_fallback
                .starts_with("# custom-migration.md\n\n## Summary\n\n# Migration Brief")
        );
    }

    #[test]
    fn render_review_and_verification_auxiliary_artifacts_support_aliases_and_fallbacks() {
        let review_source = "# Review Brief\n\n## Review Target\n\n- bounded service boundary package\n\n## Evidence Basis\n\n- owned interfaces and rollback notes\n\n## Boundary Concern\n\n- a shared DTO crosses the intended boundary\n\n## Ownership Notes\n\n- reviewer owns the final decision\n\n## Open Concern\n\n- a production trace sample is still missing\n\n## Collection Priorities\n\n- capture one bounded trace\n\n## Pending Decision\n\n- decide whether the shared DTO remains acceptable\n\n## Reversibility Concerns\n\n- rollback semantics would be harder to preserve after wider adoption\n";

        let boundary = render_review_artifact("boundary-assessment.md", review_source, "", "", "");
        let missing = render_review_artifact("missing-evidence.md", review_source, "", "", "");
        let decision = render_review_artifact("decision-impact.md", review_source, "", "", "");
        let review_fallback = render_review_artifact("custom-review.md", review_source, "", "", "");

        assert!(
            boundary
                .contains("## Boundary Findings\n\n- a shared DTO crosses the intended boundary")
        );
        assert!(
            missing.contains("## Missing Evidence\n\n- a production trace sample is still missing")
        );
        assert!(
            decision.contains(
                "## Decision Impact\n\n- decide whether the shared DTO remains acceptable"
            )
        );
        assert!(review_fallback.starts_with("# custom-review.md\n\n## Summary\n\n# Review Brief"));

        let verification_source = "# Verification Brief\n\n## Claims Under Test\n\n- rollback remains bounded and auditable\n\n## Contract Surface\n\n- rollback metadata remains explicit\n\n## Verification Outcome\n\n- contract assumptions hold for the bounded target\n\n## Challenge Focus\n\n- stress the rollback metadata path\n\n## Contradictions\n\n- none recorded\n\n## Open Findings\n\n- add one more rollback stress probe\n\n## Required Follow-up\n\n- implement the additional probe before release readiness passes\n";

        let contract =
            render_verification_artifact("contract-matrix.md", verification_source, "", "", "");
        let adversarial =
            render_verification_artifact("adversarial-review.md", verification_source, "", "", "");
        let unresolved =
            render_verification_artifact("unresolved-findings.md", verification_source, "", "", "");
        let verification_fallback =
            render_verification_artifact("custom-verification.md", verification_source, "", "", "");

        assert!(
            contract.contains("## Contract Assumptions\n\n- rollback metadata remains explicit")
        );
        assert!(
            adversarial.contains("## Challenge Findings\n\n- stress the rollback metadata path")
        );
        assert!(unresolved.contains(
            "## Required Follow-Up\n\n- implement the additional probe before release readiness passes"
        ));
        assert!(
            verification_fallback
                .starts_with("# custom-verification.md\n\n## Summary\n\n# Verification Brief")
        );
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
