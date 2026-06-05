#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchInterval {
    pub line_start: usize,
    pub line_end: usize,
    pub hunk_header: String,
}

const PATCH_TARGET_PREFIX: &str = "+++ b/";
const PATCH_NULL_TARGET: &str = "/dev/null";
const PATCH_HUNK_PREFIX: &str = "@@";

/// Parses a raw patch to extract the intervals of lines that were modified for a given surface.
pub fn parse_hunks(patch: &str, target_surface: &str) -> Vec<PatchInterval> {
    let mut current_surface: Option<&str> = None;
    let mut intervals = Vec::new();

    for line in patch.lines() {
        if let Some(surface) = line.strip_prefix(PATCH_TARGET_PREFIX) {
            current_surface = if surface == PATCH_NULL_TARGET { None } else { Some(surface) };
            continue;
        }

        if line.starts_with(PATCH_HUNK_PREFIX)
            && current_surface.is_some_and(|surface| surface == target_surface)
            && let Some((start, end)) = parse_patch_range_from_hunk(line)
        {
            intervals.push(PatchInterval {
                line_start: start,
                line_end: end,
                hunk_header: line.to_string(),
            });
        }
    }

    intervals
}

fn parse_patch_range_from_hunk(hunk_header: &str) -> Option<(usize, usize)> {
    let added_range = hunk_header.split_whitespace().find(|part| part.starts_with('+'))?;
    let added_range = added_range.strip_prefix('+')?;

    let (line_start, line_count) = match added_range.split_once(',') {
        Some((start, count)) => (start, count),
        None => (added_range, "1"),
    };

    let line_start = line_start.parse::<usize>().ok()?;
    let line_count = line_count.parse::<usize>().ok()?;

    if line_count == 0 {
        return None;
    }

    let line_end = line_start.checked_add(line_count.checked_sub(1)?)?;
    Some((line_start, line_end))
}

/// Validates a line number against the diff intervals.
/// If the line falls within a valid hunk interval, returns `Ok(Some(line))`.
/// If the line is invalid but a hunk can be inferred (e.g. only one hunk exists, or we fall back to general),
/// returns `Err(hunk_header)`.
pub fn validate_and_map_line(
    target_surface: &str,
    line: u32,
    patch: &str,
) -> Result<u32, Option<String>> {
    let hunks = parse_hunks(patch, target_surface);

    if hunks.is_empty() {
        return Err(None);
    }

    // Exact match
    for hunk in &hunks {
        if line as usize >= hunk.line_start && line as usize <= hunk.line_end {
            return Ok(line);
        }
    }

    // Hallucination fallback: return the first hunk's header to downgrade it
    Err(Some(hunks[0].hunk_header.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_PATCH: &str = "\
--- a/src/main.rs
+++ b/src/main.rs
@@ -10,3 +10,4 @@
 line 10
 line 11
 line 12
+line 13
@@ -50,2 +51,2 @@
 line 51
+line 52
";

    #[test]
    fn maps_valid_line_correctly() {
        let result = validate_and_map_line("src/main.rs", 13, MOCK_PATCH);
        assert_eq!(result, Ok(13));

        let result2 = validate_and_map_line("src/main.rs", 52, MOCK_PATCH);
        assert_eq!(result2, Ok(52));
    }

    #[test]
    fn downgrades_hallucinated_line() {
        // Line 99 is not in the patch
        let result = validate_and_map_line("src/main.rs", 99, MOCK_PATCH);
        assert_eq!(result, Err(Some("@@ -10,3 +10,4 @@".to_string())));
    }

    #[test]
    fn returns_err_none_if_surface_not_in_patch() {
        let result = validate_and_map_line("src/unknown.rs", 13, MOCK_PATCH);
        assert_eq!(result, Err(None));
    }
}
