use crate::domain::artifact::artifact_slug;

use super::MISSING_AUTHORED_BODY_MARKER;
use super::shared::{
    extract_authored_h2_section, extract_marker, render_authored_artifact, trim_multiline_block,
};
use super::{AuthoredSectionSpec, render_markdown};

pub fn render_architecture_artifact(
    file_name: &str,
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
) -> String {
    let file_name = artifact_slug(file_name);
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
        "architecture-overview.md" => {
            render_architecture_overview(context_summary, &decision_focus, &constraint)
        }
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
        "readiness-assessment.md" => render_architecture_readiness_assessment(
            context_summary,
            generation_summary,
            critique_summary,
        ),
        "system-context.md" => {
            render_c4_artifact("System Context", "System Context", &[], context_summary)
        }
        "system-context.mmd" => render_architecture_mermaid_artifact(
            "System Context",
            "System Context",
            &[],
            context_summary,
        ),
        "container-view.md" => {
            render_c4_artifact("Container View", "Containers", &[], context_summary)
        }
        "container-view.mmd" => render_architecture_mermaid_artifact(
            "Container View",
            "Containers",
            &[],
            context_summary,
        ),
        "deployment-view.md" => render_c4_artifact(
            "Deployment View",
            "Deployment",
            &["Deployment View", "Deployment Topology"],
            context_summary,
        ),
        "deployment-view.mmd" => render_architecture_mermaid_artifact(
            "Deployment View",
            "Deployment",
            &["Deployment View", "Deployment Topology"],
            context_summary,
        ),
        "component-view.md" => {
            render_c4_artifact("Component View", "Components", &[], context_summary)
        }
        "component-view.mmd" => render_architecture_mermaid_artifact(
            "Component View",
            "Components",
            &[],
            context_summary,
        ),
        "dynamic-view.md" => render_c4_artifact(
            "Dynamic View",
            "Dynamic View",
            &["Dynamic Flow", "Dynamic Flows"],
            context_summary,
        ),
        "dynamic-view.mmd" => render_architecture_mermaid_artifact(
            "Dynamic View",
            "Dynamic View",
            &["Dynamic Flow", "Dynamic Flows"],
            context_summary,
        ),
        other => render_markdown(other, context_summary),
    }
}

/// Returns `true` if the named architecture artifact should be included based on the context summary.
pub fn architecture_artifact_enabled(file_name: &str, context_summary: &str) -> bool {
    let file_name = artifact_slug(file_name);
    match file_name {
        "component-view.md" | "component-view.mmd" => {
            architecture_view_authored(file_name, context_summary)
        }
        "dynamic-view.md" | "dynamic-view.mmd" => {
            architecture_view_authored(file_name, context_summary)
        }
        _ => true,
    }
}

/// Returns `true` if an architecture artifact's body sections have been authored by a human.
pub fn architecture_view_authored(file_name: &str, context_summary: &str) -> bool {
    let (canonical_heading, aliases) = architecture_view_heading(file_name);
    extract_authored_h2_section(context_summary, canonical_heading, aliases).is_some()
}

/// Shared marker emitted by C4 architecture artifacts when the authored brief
/// did not include the canonical H2 section. Tests rely on this exact text.
pub const C4_MISSING_AUTHORED_BODY_MARKER: &str = MISSING_AUTHORED_BODY_MARKER;

pub fn extract_paragraph_nodes(body: &str) -> Vec<String> {
    body.split("\n\n").map(trim_multiline_block).filter(|paragraph| !paragraph.is_empty()).collect()
}

pub fn first_paragraph(body: &str) -> Option<&str> {
    body.split("\n\n").map(str::trim).find(|paragraph| {
        !paragraph.is_empty() && !paragraph.starts_with("-") && !paragraph.starts_with("*")
    })
}

pub fn mermaid_label(text: &str) -> String {
    text.replace('"', "'").replace('\n', " ")
}

pub fn render_linear_mermaid(_title: &str, body: &str) -> String {
    let nodes = {
        let bullets = extract_bullet_items(body);
        if bullets.is_empty() { extract_paragraph_nodes(body) } else { bullets }
    };

    if nodes.is_empty() {
        return "flowchart LR\n    missing[\"No structured view content was authored.\"]"
            .to_string();
    }

    let mut lines = vec!["flowchart LR".to_string()];
    for (index, node) in nodes.iter().enumerate() {
        lines.push(format!("    n{index}[\"{}\"]", mermaid_label(node)));
        if index > 0 {
            lines.push(format!("    n{} --> n{index}", index - 1));
        }
    }

    lines.join("\n")
}

fn render_c4_artifact(
    title: &str,
    canonical_heading: &str,
    aliases: &[&str],
    context_summary: &str,
) -> String {
    let canonical = format!("## {canonical_heading}");
    match extract_authored_h2_section(context_summary, canonical_heading, aliases) {
        Some(body) if !body.trim().is_empty() => format!("# {title}\n\n{canonical}\n\n{body}\n"),
        _ => format!(
            "# {title}\n\n{canonical}\n\n{marker_heading}\n\nNo `{canonical}` section was authored in the supplied brief.\nAuthor this section in the architecture brief and rerun.\n",
            marker_heading = C4_MISSING_AUTHORED_BODY_MARKER,
        ),
    }
}

fn render_architecture_overview(
    context_summary: &str,
    decision_focus: &str,
    constraint: &str,
) -> String {
    let primary_decision = extract_authored_h2_section(context_summary, "Decision", &[])
        .unwrap_or_else(|| decision_focus.to_string());
    let key_constraints = extract_authored_h2_section(context_summary, "Constraints", &[])
        .unwrap_or_else(|| constraint.to_string());

    let included_views = architecture_overview_views(context_summary, true);
    let omitted_views = architecture_overview_views(context_summary, false);

    format!(
        "# Architecture Overview\n\n## Summary\n\nDecision focus: {decision_focus}\n\n## Primary Decision\n\n{primary_decision}\n\n## Key Constraints\n\n{key_constraints}\n\n## Included Views\n\n{included_views}\n\n## Omitted Views\n\n{omitted_views}\n\n## Review Guidance\n\nStart with this overview, then inspect `architecture-decisions.md` for rationale, `context-map.md` for domain boundaries, and `view-manifest.json` for the machine-readable packet map.\n"
    )
}

fn architecture_overview_views(context_summary: &str, included: bool) -> String {
    let view_specs = [
        ("system-context.md", "System Context", true, "system-context.md", "system-context.mmd"),
        ("container-view.md", "Container View", true, "container-view.md", "container-view.mmd"),
        (
            "deployment-view.md",
            "Deployment View",
            true,
            "deployment-view.md",
            "deployment-view.mmd",
        ),
        ("component-view.md", "Component View", false, "component-view.md", "component-view.mmd"),
        ("dynamic-view.md", "Dynamic View", false, "dynamic-view.md", "dynamic-view.mmd"),
    ];

    let mut sections = Vec::new();
    for (file_name, title, required, markdown_artifact, mermaid_artifact) in view_specs {
        let authored = architecture_view_authored(file_name, context_summary);
        let is_included = required || authored;
        if is_included != included {
            continue;
        }

        if included {
            let coverage_note = if authored {
                format!("- {title}: emitted as `{markdown_artifact}` and `{mermaid_artifact}`.")
            } else {
                format!(
                    "- {title}: emitted as `{markdown_artifact}` and `{mermaid_artifact}` with an explicit omission note because no authored section was supplied."
                )
            };
            let mermaid = render_architecture_artifact(mermaid_artifact, context_summary, "", "");
            sections.push(format!("{coverage_note}\n\n### {title}\n\n```mermaid\n{mermaid}\n```"));
        } else {
            sections.push(format!(
                "- {title}: omitted because no corresponding authored section was supplied in the architecture brief."
            ));
        }
    }

    if sections.is_empty() { "- None.".to_string() } else { sections.join("\n\n") }
}

fn render_architecture_mermaid_artifact(
    title: &str,
    canonical_heading: &str,
    aliases: &[&str],
    context_summary: &str,
) -> String {
    match extract_authored_h2_section(context_summary, canonical_heading, aliases) {
        Some(body) if !body.trim().is_empty() => render_architecture_mermaid_body(title, &body),
        _ => render_missing_architecture_mermaid(canonical_heading),
    }
}

fn render_architecture_mermaid_body(title: &str, body: &str) -> String {
    if title == "System Context" {
        render_system_context_mermaid(body)
    } else {
        render_linear_mermaid(title, body)
    }
}

fn render_system_context_mermaid(body: &str) -> String {
    let bullets = extract_bullet_items(body);
    let summary = first_paragraph(body).unwrap_or("System context");
    let mut lines =
        vec!["flowchart LR".to_string(), format!("    system[\"{}\"]", mermaid_label(summary))];

    if bullets.is_empty() {
        lines.push("    system_note[\"Author external actors in bullet form for a richer system-context diagram.\"]".to_string());
        lines.push("    system_note -.-> system".to_string());
    } else {
        for (index, bullet) in bullets.iter().enumerate() {
            let actor_id = format!("actor{}", index + 1);
            lines.push(format!("    {actor_id}[\"{}\"]", mermaid_label(bullet)));
            lines.push(format!("    {actor_id} --> system"));
        }
    }

    lines.join("\n")
}

fn render_missing_architecture_mermaid(canonical_heading: &str) -> String {
    format!(
        "flowchart TD\n    missing[\"No `## {canonical_heading}` section was authored in the supplied brief.\"]"
    )
}

fn extract_bullet_items(body: &str) -> Vec<String> {
    body.lines()
        .map(str::trim)
        .filter_map(|line| {
            line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")).map(trim_multiline_block)
        })
        .filter(|line| !line.is_empty())
        .collect()
}

fn render_architecture_readiness_assessment(
    context_summary: &str,
    generation_summary: &str,
    critique_summary: &str,
) -> String {
    let normalized = context_summary.to_lowercase();
    let working_assumptions = extract_authored_h2_section(
        context_summary,
        "Working Assumptions",
        &["Assumptions"],
    )
    .unwrap_or_else(|| {
        "No explicit working assumptions were authored. Treat the unresolved questions and blockers below as the remaining readiness constraints.".to_string()
    });
    let unresolved_questions = extract_authored_h2_section(
        context_summary,
        "Unresolved Questions",
        &["Open Questions", "Boundary Risks And Open Questions"],
    )
    .unwrap_or_else(|| "No explicit unresolved questions were authored.".to_string());
    let readiness_status = if unresolved_questions.starts_with("No explicit unresolved questions") {
        "Architecture analysis is bounded enough for downstream planning; the packet can move forward once the accepted risks and approvals stay acceptable.".to_string()
    } else {
        "Architecture analysis is materially useful, but downstream planning remains conditional on the working assumptions and unresolved questions below.".to_string()
    };
    let recommended_next_mode = architecture_recommended_next_mode(context_summary, &normalized);

    format!(
        "# Readiness Assessment\n\n## Summary\n\n{context_summary}\n\n## Readiness Status\n\n{readiness_status}\n\n## Working Assumptions\n\n{working_assumptions}\n\n## Unresolved Questions\n\n{unresolved_questions}\n\n## Blockers\n\n{critique_summary}\n\n## Accepted Risks\n\n{generation_summary}\n\n## Recommended Next Mode\n\n{recommended_next_mode}\n"
    )
}

fn architecture_recommended_next_mode(context_summary: &str, normalized: &str) -> String {
    let missing_decision_focus = extract_marker(context_summary, normalized, "decision focus")
        .or_else(|| extract_authored_h2_section(context_summary, "Decision", &[]))
        .is_none();
    if missing_decision_focus {
        return "discovery: the problem and decision surface are still too blurry to compare architecture tradeoffs honestly.".to_string();
    }

    let missing_boundary = extract_authored_h2_section(context_summary, "Bounded Contexts", &[])
        .or_else(|| extract_authored_h2_section(context_summary, "Candidate Boundaries", &[]))
        .is_none();
    if missing_boundary {
        return "requirements: bound the scope, constraints, and decision surface more explicitly before rerunning architecture.".to_string();
    }

    let missing_structure = extract_authored_h2_section(context_summary, "Options Considered", &[])
        .or_else(|| extract_authored_h2_section(context_summary, "Containers", &[]))
        .is_none();
    if missing_structure {
        return "system-shaping: shape the capability boundaries and candidate structure before asking architecture mode to compare tradeoffs.".to_string();
    }

    "change: carry the selected structure into a bounded change plan once approvals land and the assumptions stay acceptable.".to_string()
}

fn architecture_view_heading(file_name: &str) -> (&'static str, &'static [&'static str]) {
    let file_name = artifact_slug(file_name);
    match file_name {
        "system-context.md" | "system-context.mmd" => ("System Context", &[]),
        "container-view.md" | "container-view.mmd" => ("Containers", &[]),
        "deployment-view.md" | "deployment-view.mmd" => {
            ("Deployment", &["Deployment View", "Deployment Topology"])
        }
        "component-view.md" | "component-view.mmd" => ("Components", &[]),
        "dynamic-view.md" | "dynamic-view.mmd" => {
            ("Dynamic View", &["Dynamic Flow", "Dynamic Flows"])
        }
        _ => ("System Context", &[]),
    }
}
