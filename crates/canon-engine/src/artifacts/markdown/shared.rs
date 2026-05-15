use super::{AuthoredSectionSpec, MISSING_AUTHORED_BODY_MARKER, MISSING_AUTHORED_DECISION_MARKER};

pub fn render_authored_artifact(
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

pub fn render_authored_section(authored_source: &str, section: &AuthoredSectionSpec<'_>) -> String {
    match extract_authored_h2_section(authored_source, section.canonical_heading, section.aliases) {
        Some(body) => format!("## {}\n\n{}", section.canonical_heading, body),
        None => render_missing_authored_body_block(section.canonical_heading),
    }
}

pub fn render_missing_authored_body_block(canonical_heading: &str) -> String {
    format!(
        "{MISSING_AUTHORED_BODY_MARKER}\n\nNOT CAPTURED - No `## {canonical_heading}` section was authored in the supplied brief.\nAuthor this section in the supplied brief and rerun."
    )
}

pub fn render_authored_decision_section(
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

pub fn extract_authored_h2_section(
    source: &str,
    canonical_heading: &str,
    aliases: &[&str],
) -> Option<String> {
    std::iter::once(canonical_heading)
        .chain(aliases.iter().copied())
        .find_map(|heading| extract_markdown_h2_section(source, heading))
}

pub fn extract_authored_section_or_marker(
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

pub fn render_discovery_bundle_summary(
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

pub fn system_shaping_context_gap(
    intent: Option<&str>,
    constraint: Option<&str>,
) -> Option<String> {
    if intent.is_some() && constraint.is_some() {
        None
    } else {
        Some(
            "Insufficient evidence: supply explicit `Intent:` and `Constraint:` markers in the system-shaping brief before system shaping can proceed."
                .to_string(),
        )
    }
}

/// Renders a PR review mode artifact for the given filename slug.
pub fn extract_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    extract_markdown_section(source, marker)
        .or_else(|| extract_inline_marker(source, normalized, marker))
}

pub fn render_string_list(values: &[String], empty_message: &str) -> String {
    if values.is_empty() {
        empty_message.to_string()
    } else {
        values.iter().map(|value| format!("- {value}")).collect::<Vec<_>>().join("\n")
    }
}

pub fn render_missing_authored_decision_block(canonical_heading: &str, guidance: &str) -> String {
    format!(
        "{MISSING_AUTHORED_DECISION_MARKER}\n\nDecision required - No `## {canonical_heading}` section was authored in the supplied brief.\n{guidance}\n\n## {canonical_heading}\n\nDecision required - maintainer confirmation is still missing for this section."
    )
}

pub fn extract_markdown_h2_section(source: &str, marker: &str) -> Option<String> {
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

pub fn is_matching_h2_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with("##") || trimmed.starts_with("###") {
        return false;
    }

    trimmed
        .strip_prefix("##")
        .map(str::trim)
        .is_some_and(|heading| heading.eq_ignore_ascii_case(marker))
}

pub fn extract_inline_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
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

pub fn looks_like_inline_marker(line: &str) -> bool {
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

pub fn extract_markdown_section(source: &str, marker: &str) -> Option<String> {
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

pub fn is_matching_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

pub fn is_section_boundary(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#')
        || trimmed.starts_with("Generated framing:")
        || trimmed.starts_with("Critique evidence:")
        || trimmed.starts_with("Validation evidence:")
        || trimmed.starts_with("Mutation posture:")
}

pub fn trim_multiline_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
    }
}
