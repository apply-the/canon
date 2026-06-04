use crate::domain::artifact::artifact_slug;

use super::shared::{
    extract_authored_h2_section, extract_marker, render_authored_artifact,
    render_discovery_bundle_summary, system_shaping_context_gap,
};
use super::{AuthoredSectionSpec, render_markdown};

pub fn render_requirements_artifact(file_name: &str, idea_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
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
        "prd.md" => format!(
            "# Product Requirements Document\n\n## Summary\n\n{idea_summary}\n\n## Problem\n\nThe team needs a bounded product-facing requirements packet that can be read as one coherent document.\n\n## Outcome\n\nStakeholders can review a single PRD while retaining the sectional packet for deeper inspection.\n\n## Constraints\n\n- Keep the packet local-first and auditable.\n- Preserve explicit human ownership and approval checkpoints.\n\n## Recommended Path\n\nRender one additive PRD beside the sectional requirements files so publish stays readable without breaking existing artifact consumers.\n\n## Tradeoffs\n\n- A consolidated PRD adds one more artifact to maintain.\n- The sectional files still exist because they remain useful for focused review.\n\n## Scope Cuts\n\n- No publish-engine rewrite in this slice.\n\n## Decision Checklist\n\n- [x] The consolidated PRD is additive.\n- [x] The publish flow remains governed by existing gates.\n"
        ),
        other => render_markdown(other, idea_summary),
    }
}

/// Renders a requirements artifact using captured evidence rather than a bare summary.
pub fn render_requirements_artifact_from_evidence(
    file_name: &str,
    idea_summary: &str,
    authored_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
    _denied_summary: &str,
) -> String {
    let file_name = artifact_slug(file_name);
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
        "prd.md" => render_authored_artifact(
            "Product Requirements Document",
            idea_summary,
            authored_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Problem", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Outcome", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Constraints", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Non-Negotiables", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Options", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommended Path", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Tradeoffs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Consequences", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Scope Cuts", aliases: &["Out of Scope"] },
                AuthoredSectionSpec { canonical_heading: "Deferred Work", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Decision Checklist", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Open Questions", aliases: &[] },
            ],
        ),
        other => render_requirements_artifact(other, idea_summary),
    }
}

/// Renders a discovery mode artifact for the given filename slug.
pub fn render_discovery_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
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

/// Renders a system-shaping mode artifact for the given filename slug.
pub fn render_system_shaping_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
) -> String {
    let file_name = artifact_slug(file_name);
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
                AuthoredSectionSpec { canonical_heading: "Why Not The Others", aliases: &[] },
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

/// Renders a brainstorming mode artifact for the given filename slug.
pub fn render_brainstorming_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let authored_summary = "Brainstorming context capture.";

    match file_name {
        "context.md" => render_authored_artifact(
            "Context",
            authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Context", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Goals", aliases: &[] },
            ],
        ),
        "options.md" => render_authored_artifact(
            "Options",
            authored_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Options", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Recommended Option", aliases: &[] },
            ],
        ),
        "tradeoffs.md" => render_authored_artifact(
            "Tradeoffs",
            authored_summary,
            context_summary,
            &[AuthoredSectionSpec { canonical_heading: "Tradeoffs", aliases: &[] }],
        ),
        "spikes.md" => render_authored_artifact(
            "Spikes",
            authored_summary,
            context_summary,
            &[AuthoredSectionSpec { canonical_heading: "Spikes", aliases: &[] }],
        ),
        "open-questions.md" => render_authored_artifact(
            "Open Questions",
            authored_summary,
            context_summary,
            &[AuthoredSectionSpec { canonical_heading: "Open Questions", aliases: &[] }],
        ),
        other => render_markdown(other, context_summary),
    }
}
