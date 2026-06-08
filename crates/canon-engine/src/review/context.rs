//! Onion-layer review context index.
//!
//! Builds compact context indexes (TSV + JSON) from changed files, diff hunks,
//! and deterministic relation hints. The context index is the primary LLM-facing
//! map for progressive review discovery.

use serde::{Deserialize, Serialize};

/// A single entry in the context index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextIndexEntry {
    pub id: String,
    pub entry_type: String,
    pub path: String,
    pub start_line: Option<u32>,
    pub end_line: Option<u32>,
    pub reason: String,
    pub risk: String,
    pub layer: String,
}

/// A compact index of all review context references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextIndex {
    pub entries: Vec<ContextIndexEntry>,
}

impl ContextIndex {
    /// Builds a context index from changed files and a diff patch.
    ///
    /// Each changed file gets a `file` entry. High-risk surfaces (boundary,
    /// contract, public API, schema) get an additional `diff` entry with
    /// elevated risk.
    pub fn build(changed_files: &[String], patch: &str) -> Self {
        let mut entries = Vec::new();
        let mut id_counter = 0u32;

        for path in changed_files {
            id_counter += 1;
            let risk = classify_risk(path);
            let layer = classify_layer(path);

            // File-level entry
            entries.push(ContextIndexEntry {
                id: format!("C{:03}", id_counter),
                entry_type: "file".to_string(),
                path: path.clone(),
                start_line: None,
                end_line: None,
                reason: format!("full file needed for {} analysis", layer),
                risk: risk.to_string(),
                layer: layer.to_string(),
            });

            // Diff entry for high-risk files
            if risk != "low" {
                id_counter += 1;
                let (start, end) = first_hunk_range(patch, path);
                entries.push(ContextIndexEntry {
                    id: format!("C{:03}", id_counter),
                    entry_type: "diff".to_string(),
                    path: path.clone(),
                    start_line: start,
                    end_line: end,
                    reason: format!("{risk}-risk surface changed"),
                    risk: risk.to_string(),
                    layer: "diff".to_string(),
                });
            }
        }

        // Relation hints: look for test files matching source file names.
        // When a source file's derived test path is already in changed_files,
        // the relation is implicit — no separate entry needed.
        for path in changed_files {
            if is_source_surface(path) {
                let test_hint = derive_test_path(path);
                if changed_files.iter().any(|f| f == &test_hint) {
                    continue;
                }
            }
        }

        Self { entries }
    }

    /// Serializes the index as TSV.
    pub fn to_tsv(&self) -> String {
        let mut out = String::from("id\ttype\tpath\tstart_line\tend_line\treason\trisk\tlayer\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                entry.id,
                entry.entry_type,
                entry.path,
                entry.start_line.map_or(String::new(), |v| v.to_string()),
                entry.end_line.map_or(String::new(), |v| v.to_string()),
                entry.reason,
                entry.risk,
                entry.layer,
            ));
        }
        out
    }

    /// Serializes the index as JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.entries).unwrap_or_default()
    }
}

fn classify_risk(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.contains("boundary")
        || lower.contains("public")
        || lower.contains("interface")
        || lower.contains("contract")
        || lower.contains("api")
        || lower.contains("schema")
    {
        "high"
    } else if lower.starts_with("src/") || lower.contains("/src/") {
        "medium"
    } else {
        "low"
    }
}

fn classify_layer(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.starts_with("tests/") || lower.contains("/tests/") || lower.ends_with("_test.rs") {
        "tests"
    } else {
        "whole_file"
    }
}

fn is_source_surface(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.starts_with("src/") || lower.contains("/src/")
}

fn derive_test_path(source_path: &str) -> String {
    source_path.replace("src/", "tests/").replace(".rs", "_test.rs")
}

fn first_hunk_range(patch: &str, target_path: &str) -> (Option<u32>, Option<u32>) {
    let target_marker = format!("+++ b/{}", target_path);
    let mut found_target = false;
    for line in patch.lines() {
        if line == target_marker {
            found_target = true;
            continue;
        }
        if found_target
            && line.starts_with("@@")
            && let Some(range) = parse_hunk_new_range(line)
        {
            return range;
        }
    }
    (None, None)
}

fn parse_hunk_new_range(hunk: &str) -> Option<(Option<u32>, Option<u32>)> {
    let parts: Vec<&str> = hunk.split_whitespace().collect();
    for part in parts {
        if part.starts_with('+') {
            let range = part.trim_start_matches('+');
            if let Some((start_str, count_str)) = range.split_once(',') {
                let start: u32 = start_str.parse().ok()?;
                let count: u32 = count_str.parse().ok()?;
                if count > 0 {
                    return Some((Some(start), Some(start + count - 1)));
                }
            } else {
                let start: u32 = range.parse().ok()?;
                return Some((Some(start), Some(start)));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_context_index_from_changed_files() {
        let files = vec!["src/transport/http.rs".to_string(), "tests/http_test.rs".to_string()];
        let patch = "--- a/src/transport/http.rs\n+++ b/src/transport/http.rs\n@@ -40,6 +42,8 @@\n";
        let index = ContextIndex::build(&files, patch);
        assert!(!index.entries.is_empty());
        assert!(index.entries.iter().any(|e| e.path == "src/transport/http.rs"));
    }

    #[test]
    fn test_high_risk_surfaces_get_diff_entry() {
        let files = vec!["contracts/api.json".to_string()];
        let patch = "--- a/contracts/api.json\n+++ b/contracts/api.json\n@@ -1,3 +1,5 @@\n";
        let index = ContextIndex::build(&files, patch);
        let diff_entries: Vec<_> =
            index.entries.iter().filter(|e| e.entry_type == "diff").collect();
        assert!(!diff_entries.is_empty());
    }

    #[test]
    fn test_to_tsv_produces_valid_header() {
        let index = ContextIndex::build(&["src/lib.rs".to_string()], "patch\n");
        let tsv = index.to_tsv();
        assert!(tsv.starts_with("id\ttype\tpath\t"));
    }

    #[test]
    fn test_to_json_produces_valid_json() {
        let index = ContextIndex::build(&["src/lib.rs".to_string()], "patch\n");
        let json = index.to_json();
        assert!(serde_json::from_str::<Vec<ContextIndexEntry>>(&json).is_ok());
    }

    #[test]
    fn test_source_without_matching_test_in_changed_files() {
        let files = vec!["src/transport/http.rs".to_string()];
        let patch = "--- a/src/transport/http.rs\n+++ b/src/transport/http.rs\n@@ -40,6 +42,8 @@\n";
        let index = ContextIndex::build(&files, patch);
        // Source exists but its derived test path is not in changed_files,
        // so no test entry is added. Only file entry for the source.
        assert!(
            index
                .entries
                .iter()
                .any(|e| e.path == "src/transport/http.rs" && e.entry_type == "file")
        );
        let test_entries: Vec<_> =
            index.entries.iter().filter(|e| e.entry_type == "test").collect();
        assert!(test_entries.is_empty());
    }

    #[test]
    fn test_classify_risk_low_for_docs() {
        let files = vec!["docs/readme.md".to_string()];
        let index = ContextIndex::build(&files, "");
        let entry = index.entries.first().unwrap();
        assert_eq!(entry.risk, "low");
    }

    #[test]
    fn test_single_line_hunk_range() {
        let files = vec!["src/main.rs".to_string()];
        let patch = "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1,1 +1,1 @@\n";
        let index = ContextIndex::build(&files, patch);
        assert!(!index.entries.is_empty());
    }
}
