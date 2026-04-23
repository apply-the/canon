//! Unified-diff / mutation-patch path parsing and scope-enforcement helpers.

use std::path::{Path, PathBuf};

use super::EngineError;

// ── Payload discovery ─────────────────────────────────────────────────────────

pub(crate) fn mutation_payload_candidates_for(resolved: &Path) -> Vec<PathBuf> {
    if resolved.is_dir() {
        return known_mutation_payload_names().iter().map(|name| resolved.join(name)).collect();
    }

    let Some(parent) = resolved.parent() else {
        return Vec::new();
    };

    let mut candidates =
        known_mutation_payload_names().iter().map(|name| parent.join(name)).collect::<Vec<_>>();
    if let Some(stem) = resolved.file_stem().and_then(|stem| stem.to_str()) {
        candidates.push(parent.join(format!("{stem}.diff")));
        candidates.push(parent.join(format!("{stem}.patch")));
    }
    candidates
}

pub(crate) fn is_known_mutation_payload_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            let lower = name.to_ascii_lowercase();
            known_mutation_payload_names().iter().any(|candidate| *candidate == lower)
        })
        .unwrap_or(false)
}

pub(crate) fn known_mutation_payload_names() -> &'static [&'static str] {
    &[
        "patch.diff",
        "mutation.diff",
        "mutation.patch",
        "execution.diff",
        "execution.patch",
        "bounded.diff",
        "bounded.patch",
    ]
}

// ── Diff path parsing ─────────────────────────────────────────────────────────

pub(crate) fn parse_unified_diff_paths(patch: &str) -> Result<Vec<String>, EngineError> {
    let mut changed_paths = Vec::new();

    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let mut parts = rest.split_whitespace();
            for raw in parts.by_ref().take(2) {
                if let Some(path) = normalize_diff_path(raw) {
                    push_unique_path(&mut changed_paths, path);
                }
            }
            continue;
        }

        if let Some(raw) = line.strip_prefix("--- ").or_else(|| line.strip_prefix("+++ "))
            && let Some(path) = normalize_diff_path(raw.trim())
        {
            push_unique_path(&mut changed_paths, path);
        }
    }

    if changed_paths.is_empty() {
        return Err(EngineError::Validation(
            "bounded mutation payload does not declare any changed file paths".to_string(),
        ));
    }

    Ok(changed_paths)
}

pub(crate) fn normalize_diff_path(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches('"');
    if trimmed == "/dev/null" {
        return None;
    }

    let stripped =
        trimmed.strip_prefix("a/").or_else(|| trimmed.strip_prefix("b/")).unwrap_or(trimmed);
    let normalized = normalize_repo_relative_path(stripped);
    (!normalized.is_empty()).then_some(normalized)
}

pub(crate) fn push_unique_path(paths: &mut Vec<String>, candidate: String) {
    if !paths.iter().any(|existing| existing == &candidate) {
        paths.push(candidate);
    }
}

// ── Scope enforcement ─────────────────────────────────────────────────────────

pub(crate) fn path_within_allowed_scope(path: &str, allowed_paths: &[String]) -> bool {
    let normalized_path = normalize_repo_relative_path(path);
    if normalized_path.is_empty() {
        return false;
    }

    allowed_paths.iter().any(|entry| {
        normalized_scope_prefix(entry).is_some_and(|allowed| {
            normalized_path == allowed || normalized_path.starts_with(&format!("{allowed}/"))
        })
    })
}

pub(crate) fn normalized_scope_prefix(entry: &str) -> Option<String> {
    let normalized = normalize_repo_relative_path(entry);
    if normalized.is_empty() {
        return None;
    }

    let trimmed = normalized
        .strip_suffix("/**")
        .or_else(|| normalized.strip_suffix("/*"))
        .unwrap_or(normalized.as_str())
        .trim_end_matches('/');
    (!trimmed.is_empty()).then_some(trimmed.to_string())
}

pub(crate) fn normalize_repo_relative_path(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_matches('/')
        .to_string()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_diff_path_strips_a_prefix() {
        assert_eq!(normalize_diff_path("a/src/lib.rs"), Some("src/lib.rs".to_string()));
    }

    #[test]
    fn normalize_diff_path_strips_b_prefix() {
        assert_eq!(normalize_diff_path("b/src/lib.rs"), Some("src/lib.rs".to_string()));
    }

    #[test]
    fn normalize_diff_path_returns_none_for_dev_null() {
        assert!(normalize_diff_path("/dev/null").is_none());
    }

    #[test]
    fn normalize_diff_path_handles_quoted_paths() {
        assert_eq!(normalize_diff_path("\"a/src/lib.rs\""), Some("src/lib.rs".to_string()));
    }

    #[test]
    fn parse_unified_diff_paths_extracts_from_git_diff_header() {
        let patch = "diff --git a/src/main.rs b/src/main.rs\nindex abc..def 100644\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1 +1 @@\n-old\n+new";
        let paths = parse_unified_diff_paths(patch).expect("should parse");
        assert!(paths.contains(&"src/main.rs".to_string()));
    }

    #[test]
    fn parse_unified_diff_paths_errors_on_empty_patch() {
        let result = parse_unified_diff_paths("not a real diff");
        assert!(result.is_err());
    }

    #[test]
    fn push_unique_path_deduplicates() {
        let mut paths = vec!["src/lib.rs".to_string()];
        push_unique_path(&mut paths, "src/lib.rs".to_string());
        push_unique_path(&mut paths, "src/main.rs".to_string());
        assert_eq!(paths, vec!["src/lib.rs", "src/main.rs"]);
    }

    #[test]
    fn path_within_allowed_scope_matches_exact_path() {
        let allowed = vec!["src/lib.rs".to_string()];
        assert!(path_within_allowed_scope("src/lib.rs", &allowed));
        assert!(!path_within_allowed_scope("src/main.rs", &allowed));
    }

    #[test]
    fn path_within_allowed_scope_matches_directory_prefix() {
        let allowed = vec!["src/**".to_string()];
        assert!(path_within_allowed_scope("src/lib.rs", &allowed));
        assert!(path_within_allowed_scope("src/sub/mod.rs", &allowed));
        assert!(!path_within_allowed_scope("tests/main.rs", &allowed));
    }

    #[test]
    fn normalize_repo_relative_path_strips_dot_slash_prefix() {
        assert_eq!(normalize_repo_relative_path("./src/lib.rs"), "src/lib.rs");
    }

    #[test]
    fn normalize_repo_relative_path_converts_backslashes() {
        assert_eq!(normalize_repo_relative_path(r"src\lib.rs"), "src/lib.rs");
    }

    #[test]
    fn is_known_mutation_payload_file_recognizes_known_names() {
        use std::path::Path;
        assert!(is_known_mutation_payload_file(Path::new("mutation.diff")));
        assert!(is_known_mutation_payload_file(Path::new("patch.diff")));
        assert!(!is_known_mutation_payload_file(Path::new("random.rs")));
    }
}
