//! Markdown and plain-text parsing helpers used by multiple service sub-modules.
//!
//! All functions here are pure text transformations with no domain-type dependencies.

// ── Markdown list / bullet parsing ──────────────────────────────────────────

pub(crate) fn split_context_items(block: &str) -> Vec<String> {
    let items = block
        .lines()
        .filter_map(|line| trim_list_item(line.trim()))
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();

    if !items.is_empty() {
        return items;
    }

    let condensed = condense_context_block(block, 220);
    if condensed.is_empty() { Vec::new() } else { vec![condensed] }
}

pub(crate) fn trim_list_item(line: &str) -> Option<String> {
    if !is_meaningful_context_line(line) {
        return None;
    }

    if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        return Some(item.trim().to_string());
    }

    let mut digits = 0usize;
    for character in line.chars() {
        if character.is_ascii_digit() {
            digits += 1;
            continue;
        }

        if digits > 0 && (character == '.' || character == ')') {
            return Some(line[digits + 1..].trim().to_string());
        }

        break;
    }

    None
}

pub(crate) fn condense_context_block(value: &str, max_chars: usize) -> String {
    let filtered = value
        .lines()
        .map(str::trim)
        .filter(|line| is_meaningful_context_line(line))
        .collect::<Vec<_>>();
    let candidate =
        if filtered.is_empty() { trim_context_block(value) } else { filtered.join(" ") };
    truncate_context_excerpt(&candidate, max_chars)
}

pub(crate) fn is_meaningful_context_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty()
        && !trimmed.starts_with('#')
        && !trimmed.starts_with("## Input:")
        && !trimmed.starts_with("![](")
        && !trimmed.starts_with('|')
        && !trimmed.starts_with("Page ")
        && !trimmed.eq_ignore_ascii_case("internal")
        && !trimmed.starts_with("Version ")
        && !trimmed.starts_with("Author:")
        && !trimmed.starts_with("Prepared by:")
        && !trimmed.starts_with("Checked by:")
        && !trimmed.starts_with("Approved by:")
        && !trimmed.starts_with("Revision")
}

pub(crate) fn truncate_context_excerpt(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    let char_count = trimmed.chars().count();
    if char_count <= max_chars {
        return trimmed.to_string();
    }

    let truncated = trimmed.chars().take(max_chars).collect::<String>();
    let safe = truncated.rfind(char::is_whitespace).map(|index| truncated[..index].trim());
    match safe {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}..."),
        _ => format!("{}...", truncated.trim()),
    }
}

pub(crate) fn trim_context_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
    }
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

// ── Section / marker extraction ──────────────────────────────────────────────

/// Extracts a section under a matching heading.  Stops at the next heading.
pub(crate) fn extract_context_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_context_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if next_line.trim().starts_with('#') {
                break;
            }
            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_context_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
    }

    None
}

fn is_matching_context_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }
    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

/// Tries a markdown heading match first, then an inline `Key: value` marker.
pub(crate) fn extract_context_marker(
    source: &str,
    normalized: &str,
    markers: &[&str],
) -> Option<String> {
    markers.iter().find_map(|marker| {
        extract_context_section(source, marker)
            .or_else(|| extract_context_inline_marker(source, normalized, marker))
    })
}

pub(crate) fn extract_context_inline_marker(
    source: &str,
    normalized: &str,
    marker: &str,
) -> Option<String> {
    let marker_with_colon = format!("{marker}:");
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let line = remainder.lines().next()?.trim();
    if line.is_empty() { None } else { Some(line.to_string()) }
}

pub(crate) fn extract_context_list(
    source: &str,
    normalized: &str,
    markers: &[&str],
) -> Vec<String> {
    extract_context_marker(source, normalized, markers)
        .map(|value| split_context_items(&value))
        .unwrap_or_default()
}

// ── Markdown-section parser (stops at any heading or sentinel boundary) ──────

pub(crate) fn extract_markdown_section(source: &str, marker: &str) -> Option<String> {
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
        || trimmed.starts_with("Validation evidence:")
        || trimmed.starts_with("Mutation posture:")
}

/// Tries heading match first, then inline `Key: value` with multi-line continuation.
pub(crate) fn extract_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    extract_markdown_section(source, marker)
        .or_else(|| extract_inline_marker(source, normalized, marker))
}

pub(crate) fn extract_inline_marker(
    source: &str,
    normalized: &str,
    marker: &str,
) -> Option<String> {
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

    let section = trim_context_block(&section_lines.join("\n"));
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

// ── High-level entry extractors ──────────────────────────────────────────────

pub(crate) fn extract_marker_entries(source: &str, marker: &str) -> Vec<String> {
    let normalized = source.to_lowercase();
    let Some(raw_surface) = extract_marker(source, &normalized, marker) else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    for line in raw_surface.lines() {
        let trimmed = line.trim().trim_start_matches(['-', '*', '+']).trim();
        if trimmed.is_empty() {
            continue;
        }

        for segment in trimmed.split(';').flat_map(|segment| segment.split(',')) {
            let value = segment.trim();
            if value.is_empty() {
                continue;
            }
            if !entries.iter().any(|existing: &String| existing.eq_ignore_ascii_case(value)) {
                entries.push(value.to_string());
            }
        }
    }

    entries
}

pub(crate) fn extract_first_marker_entries(source: &str, markers: &[&str]) -> Vec<String> {
    markers
        .iter()
        .find_map(|marker| {
            let entries = extract_marker_entries(source, marker);
            (!entries.is_empty()).then_some(entries)
        })
        .unwrap_or_default()
}

pub(crate) fn extract_execution_scope_entries(source: &str, markers: &[&str]) -> Vec<String> {
    extract_first_marker_entries(source, markers)
}

pub(crate) fn extract_change_surface_entries(source: &str) -> Vec<String> {
    extract_execution_scope_entries(source, &["change surface"])
}

// ── Text helpers ─────────────────────────────────────────────────────────────

pub(crate) fn first_meaningful_line(source: &str) -> String {
    source
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            "Bound the problem to the current repository before moving into a planning or execution mode."
                .to_string()
        })
}

pub(crate) fn infer_discovery_next_phase(source: &str) -> String {
    let normalized = source.to_lowercase();
    if normalized.contains("architecture") || normalized.contains("boundary") {
        "Translate this discovery packet into architecture mode with named boundaries, invariants, and explicit tradeoffs."
            .to_string()
    } else if normalized.contains("legacy")
        || normalized.contains("existing")
        || normalized.contains("change")
    {
        "Translate this discovery packet into change mode with preserved invariants and a bounded change surface."
            .to_string()
    } else if normalized.contains("new capability")
        || normalized.contains("system-shaping")
        || normalized.contains("system shaping")
        || normalized.contains("new system")
    {
        "Translate this discovery packet into system-shaping mode with explicit capability boundaries and phased delivery options."
            .to_string()
    } else {
        "Translate this discovery packet into requirements mode with a bounded problem statement, constraints, options, and scope cuts."
            .to_string()
    }
}

pub(crate) fn render_repo_surface_block(repo_surfaces: &[String]) -> String {
    if repo_surfaces.is_empty() {
        "- no-repository-surfaces-detected".to_string()
    } else {
        repo_surfaces.iter().map(|surface| format!("- {surface}")).collect::<Vec<_>>().join("\n")
    }
}

// ── Counting / label helpers (used by summarizers and clarity) ───────────────

pub(crate) fn count_markdown_entries(block: &str) -> usize {
    let count = block.lines().filter(|line| trim_list_item(line.trim()).is_some()).count();
    if count == 0 && !block.trim().is_empty() && !block.contains("NOT CAPTURED") {
        1
    } else {
        count
    }
}

pub(crate) fn count_context_items_without_placeholders(
    block: &str,
    placeholders: &[&str],
) -> usize {
    if block.contains("NOT CAPTURED") {
        return 0;
    }

    split_context_items(block)
        .into_iter()
        .filter(|item| {
            !placeholders.iter().any(|placeholder| item.eq_ignore_ascii_case(placeholder))
        })
        .count()
}

pub(crate) fn extract_labeled_usize(block: &str, label: &str) -> Option<usize> {
    extract_labeled_context_value(block, label)?.parse().ok()
}

pub(crate) fn extract_labeled_context_value(block: &str, label: &str) -> Option<String> {
    let prefix = format!("{}:", label.to_ascii_lowercase());

    block.lines().find_map(|line| {
        let normalized = trim_list_item(line.trim()).unwrap_or_else(|| line.trim().to_string());
        if !normalized.to_ascii_lowercase().starts_with(&prefix) {
            return None;
        }
        let value = normalized[normalized.find(':')? + 1..].trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    })
}

pub(crate) fn extract_result_section(
    contents: &str,
    section: &str,
    missing_section: &str,
    fallback: &str,
) -> (String, bool) {
    if let Some(value) = extract_context_section(contents, section) {
        return (value, false);
    }

    if let Some(value) = extract_context_section(contents, missing_section) {
        let trimmed = trim_context_block(&value);
        if trimmed.starts_with("NOT CAPTURED") {
            return (trimmed, true);
        }
        return (format!("NOT CAPTURED - {trimmed}"), true);
    }

    (fallback.to_string(), true)
}

pub(crate) fn count_missing_context_markers<T>(sections: impl IntoIterator<Item = T>) -> usize
where
    T: AsRef<str>,
{
    sections.into_iter().filter(|section| section.as_ref().contains("NOT CAPTURED")).count()
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_context_excerpt_returns_full_string_when_within_limit() {
        let value = "short text";
        assert_eq!(truncate_context_excerpt(value, 100), "short text");
    }

    #[test]
    fn truncate_context_excerpt_truncates_at_word_boundary() {
        let value = "one two three four five";
        let result = truncate_context_excerpt(value, 10);
        assert!(result.ends_with("..."), "should end with ellipsis: {result}");
        assert!(result.len() <= 13, "should be ≤ original limit + ellipsis: {result}");
    }

    #[test]
    fn split_context_items_parses_bullet_list() {
        let block = "- first item\n- second item\n- third item";
        let items = split_context_items(block);
        assert_eq!(items, vec!["first item", "second item", "third item"]);
    }

    #[test]
    fn split_context_items_falls_back_to_condensed_block() {
        let block = "This is a plain paragraph without bullets.";
        let items = split_context_items(block);
        assert_eq!(items.len(), 1);
        assert!(items[0].contains("plain paragraph"));
    }

    #[test]
    fn extract_context_section_finds_heading() {
        let source = "# Run\n\nSome text\n\n## Problem\nThis is the problem.\n\n## Next\nOther";
        let result = extract_context_section(source, "Problem");
        assert_eq!(result, Some("This is the problem.".to_string()));
    }

    #[test]
    fn extract_context_section_returns_none_for_missing_heading() {
        let source = "## Summary\nSome text";
        assert!(extract_context_section(source, "Problem").is_none());
    }

    #[test]
    fn extract_result_section_does_not_duplicate_not_captured_prefix() {
        let source = "## Missing Authored Body\n\nNOT CAPTURED - No `## Change Surface` section was authored in the supplied brief.";
        let result = extract_result_section(
            source,
            "Change Surface",
            "Missing Authored Body",
            "NOT CAPTURED - Change surface section is missing.",
        );
        assert_eq!(
            result,
            (
                "NOT CAPTURED - No `## Change Surface` section was authored in the supplied brief."
                    .to_string(),
                true,
            )
        );
    }

    #[test]
    fn extract_context_section_case_insensitive() {
        let source = "## PROBLEM\nContent here.";
        assert!(extract_context_section(source, "problem").is_some());
    }

    #[test]
    fn count_markdown_entries_counts_bullets() {
        let block = "- alpha\n- beta\n- gamma";
        assert_eq!(count_markdown_entries(block), 3);
    }

    #[test]
    fn count_markdown_entries_returns_one_for_non_empty_non_bullet_block() {
        let block = "Some free text without bullets.";
        assert_eq!(count_markdown_entries(block), 1);
    }

    #[test]
    fn count_markdown_entries_returns_zero_for_not_captured() {
        let block = "NOT CAPTURED - section is missing.";
        assert_eq!(count_markdown_entries(block), 0);
    }

    #[test]
    fn count_missing_context_markers_counts_not_captured_strings() {
        let sections =
            vec!["NOT CAPTURED - missing", "Valid content", "NOT CAPTURED - also missing"];
        assert_eq!(count_missing_context_markers(sections), 2);
    }

    #[test]
    fn extract_change_surface_entries_parses_markdown_section() {
        let source = "## Change Surface\n- auth module\n- session repository\n\n## Other\nignored";
        let entries = extract_change_surface_entries(source);
        assert_eq!(entries, vec!["auth module", "session repository"]);
    }

    #[test]
    fn extract_change_surface_entries_parses_inline_marker() {
        let source = "Change Surface: auth service, session repository";
        let entries = extract_change_surface_entries(source);
        assert_eq!(entries, vec!["auth service", "session repository"]);
    }

    #[test]
    fn extract_result_section_returns_section_content_and_false() {
        let contents = "## Task Mapping\nDo the work.\n\n## Next\nOther";
        let (value, missing) =
            extract_result_section(contents, "Task Mapping", "Missing Context", "fallback");
        assert!(!missing);
        assert!(value.contains("Do the work."));
    }

    #[test]
    fn extract_result_section_returns_fallback_and_true_when_missing() {
        let contents = "## Other Section\nContent";
        let (value, missing) =
            extract_result_section(contents, "Task Mapping", "Missing Context", "fallback");
        assert!(missing);
        assert_eq!(value, "fallback");
    }

    #[test]
    fn infer_discovery_next_phase_returns_architecture_for_boundary_keywords() {
        let source = "We need to define the architecture boundaries.";
        assert!(infer_discovery_next_phase(source).contains("architecture"));
    }

    #[test]
    fn infer_discovery_next_phase_returns_requirements_as_default() {
        let source = "We need to understand the project better.";
        assert!(infer_discovery_next_phase(source).contains("requirements"));
    }

    #[test]
    fn render_repo_surface_block_produces_bulleted_list() {
        let surfaces = vec!["src/".to_string(), "tests/".to_string()];
        let result = render_repo_surface_block(&surfaces);
        assert!(result.contains("- src/"));
        assert!(result.contains("- tests/"));
    }

    #[test]
    fn render_repo_surface_block_handles_empty_input() {
        assert_eq!(render_repo_surface_block(&[]), "- no-repository-surfaces-detected");
    }

    #[test]
    fn count_context_items_without_placeholders_excludes_placeholders() {
        let block = "- first item\n- second item";
        let count = count_context_items_without_placeholders(block, &["second item"]);
        assert_eq!(count, 1);
    }

    #[test]
    fn extract_labeled_context_value_finds_status_label() {
        let block = "Status: awaiting-disposition\nRationale: needs review";
        assert_eq!(
            extract_labeled_context_value(block, "Status"),
            Some("awaiting-disposition".to_string())
        );
    }

    #[test]
    fn is_meaningful_context_line_rejects_headings_and_blank_lines() {
        assert!(!is_meaningful_context_line("## Section Header"));
        assert!(!is_meaningful_context_line(""));
        assert!(!is_meaningful_context_line("   "));
        assert!(is_meaningful_context_line("- bullet item"));
        assert!(is_meaningful_context_line("Plain text content"));
    }
}
