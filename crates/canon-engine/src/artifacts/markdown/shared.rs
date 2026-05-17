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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_string_list_formats_items_with_dashes() {
        let items = vec!["alpha".to_string(), "beta".to_string()];
        let result = render_string_list(&items, "none");
        assert_eq!(result, "- alpha\n- beta");
    }

    #[test]
    fn render_string_list_returns_empty_message_when_empty() {
        let result = render_string_list(&[], "nothing here");
        assert_eq!(result, "nothing here");
    }

    #[test]
    fn render_missing_authored_decision_block_contains_heading_and_guidance() {
        let result = render_missing_authored_decision_block("Risk Level", "Add a risk section.");
        assert!(result.contains("## Risk Level"));
        assert!(result.contains("Add a risk section."));
        assert!(result.contains("Missing Authored Decision"));
        assert!(result.contains("Decision required"));
    }

    #[test]
    fn extract_markdown_h2_section_finds_matching_section() {
        let source = "# Title\n\n## Problem\n\nThis is the problem.\n\n## Next\n\nother";
        let result = extract_markdown_h2_section(source, "Problem");
        assert_eq!(result.as_deref(), Some("This is the problem."));
    }

    #[test]
    fn extract_markdown_h2_section_returns_none_when_absent() {
        let source = "# Title\n\n## Other\n\nsome content";
        let result = extract_markdown_h2_section(source, "Missing");
        assert!(result.is_none());
    }

    #[test]
    fn extract_markdown_h2_section_returns_none_for_empty_body() {
        let source = "## Problem\n\n## Next\n\ncontent";
        let result = extract_markdown_h2_section(source, "Problem");
        assert!(result.is_none());
    }

    #[test]
    fn is_matching_h2_heading_matches_exact_h2() {
        assert!(is_matching_h2_heading("## Problem", "Problem"));
        assert!(is_matching_h2_heading("## problem", "Problem")); // case-insensitive
    }

    #[test]
    fn is_matching_h2_heading_rejects_h3_and_h1() {
        assert!(!is_matching_h2_heading("### Problem", "Problem"));
        assert!(!is_matching_h2_heading("# Problem", "Problem"));
        assert!(!is_matching_h2_heading("Some text", "Problem"));
    }

    #[test]
    fn is_matching_heading_matches_any_heading_level() {
        assert!(is_matching_heading("# Title", "Title"));
        assert!(is_matching_heading("## Section", "Section"));
        assert!(is_matching_heading("### Sub", "Sub"));
    }

    #[test]
    fn is_matching_heading_is_case_insensitive() {
        assert!(is_matching_heading("## RISK LEVEL", "risk level"));
    }

    #[test]
    fn is_matching_heading_rejects_non_headings() {
        assert!(!is_matching_heading("plain text", "plain text"));
    }

    #[test]
    fn system_shaping_context_gap_returns_none_when_both_supplied() {
        let result = system_shaping_context_gap(Some("Build a new service"), Some("Minimal deps"));
        assert!(result.is_none());
    }

    #[test]
    fn system_shaping_context_gap_returns_error_when_intent_missing() {
        let result = system_shaping_context_gap(None, Some("Minimal deps"));
        assert!(result.is_some());
    }

    #[test]
    fn system_shaping_context_gap_returns_error_when_constraint_missing() {
        let result = system_shaping_context_gap(Some("Build a new service"), None);
        assert!(result.is_some());
    }

    #[test]
    fn extract_inline_marker_finds_value_after_colon() {
        let source = "risk: bounded-impact\nother: stuff";
        let normalized = source.to_lowercase();
        let result = extract_inline_marker(source, &normalized, "risk");
        assert_eq!(result.as_deref(), Some("bounded-impact"));
    }

    #[test]
    fn extract_inline_marker_returns_none_when_absent() {
        let source = "zone: green";
        let normalized = source.to_lowercase();
        let result = extract_inline_marker(source, &normalized, "risk");
        assert!(result.is_none());
    }

    #[test]
    fn looks_like_inline_marker_detects_key_colon_pattern() {
        assert!(looks_like_inline_marker("Risk Level: high"));
        assert!(!looks_like_inline_marker("- bullet item"));
        assert!(!looks_like_inline_marker("* another bullet"));
        assert!(!looks_like_inline_marker("no colon here"));
    }

    #[test]
    fn trim_multiline_block_strips_leading_and_trailing_blank_lines() {
        let input = "\n\n  first line\n  second line\n\n";
        let result = trim_multiline_block(input);
        assert_eq!(result, "  first line\n  second line");
    }

    #[test]
    fn trim_multiline_block_returns_empty_for_all_blank() {
        assert_eq!(trim_multiline_block("\n\n\n"), String::new());
    }

    #[test]
    fn is_section_boundary_detects_headings_and_framing_markers() {
        assert!(is_section_boundary("## Next Section"));
        assert!(is_section_boundary("Generated framing: some text"));
        assert!(is_section_boundary("Critique evidence: some text"));
        assert!(!is_section_boundary("Regular text here"));
    }

    #[test]
    fn render_discovery_bundle_summary_includes_detail_links() {
        let result = render_discovery_bundle_summary(
            "problem-map.md",
            "The problem",
            "Some constraints",
            "Next: architecture",
        );
        // problem-map.md is the current_file so it should be excluded from links.
        assert!(!result.contains("[problem-map.md](problem-map.md)"));
        assert!(result.contains("unknowns-and-assumptions.md"));
        assert!(result.contains("The problem"));
    }

    #[test]
    fn render_discovery_bundle_summary_uses_multiline_format_for_long_content() {
        // Content with a newline triggers the multiline branch in format_field (line 91).
        let long_problem = "line one\nline two";
        let result = render_discovery_bundle_summary(
            "unknowns-and-assumptions.md",
            long_problem,
            "no constraints",
            "next step",
        );
        assert!(result.contains("- **Problem:**\n\n  line one\n  line two"));
    }

    #[test]
    fn render_missing_authored_body_block_contains_heading_and_capture_message() {
        let result = render_missing_authored_body_block("Decision");
        assert!(result.contains("## Decision"));
        assert!(result.contains("NOT CAPTURED"));
    }

    #[test]
    fn render_authored_decision_section_returns_body_when_heading_present() {
        let source = "## Risk Level\n\nBounded impact only.\n";
        let result = render_authored_decision_section(source, "Risk Level", &[], "Add risk.");
        assert!(result.contains("Bounded impact only."));
        assert!(!result.contains("Missing Authored Decision"));
    }

    #[test]
    fn render_authored_decision_section_returns_missing_block_when_heading_absent() {
        let source = "## Some Other Section\n\nsome content\n";
        let result = render_authored_decision_section(source, "Risk Level", &[], "Add risk.");
        assert!(result.contains("Missing Authored Decision"));
        assert!(result.contains("Add risk."));
    }

    #[test]
    fn extract_authored_h2_section_falls_back_to_alias() {
        let source = "## Risk\n\nBounded impact.\n";
        // Canonical heading doesn't match, but alias "Risk" does.
        let result = extract_authored_h2_section(source, "Risk Level", &["Risk"]);
        assert_eq!(result.as_deref(), Some("Bounded impact."));
    }

    #[test]
    fn extract_authored_h2_section_returns_none_when_no_match() {
        let source = "## Other\n\nsome content\n";
        let result = extract_authored_h2_section(source, "Risk Level", &[]);
        assert!(result.is_none());
    }

    #[test]
    fn extract_inline_marker_finds_multiline_value() {
        // Value is on the next line, not the same line as the key.
        let source = "risk:\nbounded-impact\nother: stuff";
        let normalized = source.to_lowercase();
        let result = extract_inline_marker(source, &normalized, "risk");
        assert_eq!(result.as_deref(), Some("bounded-impact"));
    }

    #[test]
    fn extract_markdown_section_finds_section_body() {
        let source = "# Title\n\n## Problem\n\nThis is the problem body.\n\n## Next\n\nother";
        let result = extract_markdown_section(source, "Problem");
        assert!(result.is_some());
        assert!(result.unwrap().contains("This is the problem body."));
    }

    #[test]
    fn extract_markdown_section_returns_none_when_absent() {
        let source = "## Other\n\ncontent";
        let result = extract_markdown_section(source, "Missing");
        assert!(result.is_none());
    }
}
