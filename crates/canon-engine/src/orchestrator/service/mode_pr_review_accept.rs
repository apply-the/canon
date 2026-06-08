//! Orchestrator for the `pr-review accept` phase.
//!
//! Validates the LLM-authored `reviewer-output.md` against the review context,
//! produces `canonical-review-output.json`, and updates the run state to
//! `reviewer_output_accepted` or `reviewer_output_rejected`.

use std::fs;
use std::path::Path;

use crate::review::onion::{LayerStatus, RunState};
use crate::review::validate::validate_reviewer_output;

use super::EngineService;

/// Filename for the reviewer output submitted by the LLM.
const REVIEWER_OUTPUT_FILE: &str = "reviewer-output.md";

/// Filename for the canonical review output produced by accept.
const CANONICAL_REVIEW_OUTPUT_FILE: &str = "canonical-review-output.json";

impl EngineService {
    /// Runs the accept phase: validates reviewer output and persists results.
    pub fn run_pr_review_accept(&self, run_id: &str) -> Result<(), String> {
        let run_dir = self.repo_root.join(".canon").join("runs").join(run_id).join("pr-review");
        let reviewer_output_path = run_dir.join(REVIEWER_OUTPUT_FILE);

        let reviewer_output = fs::read_to_string(&reviewer_output_path)
            .map_err(|e| format!("read reviewer output: {e}"))?;

        let changed_files = read_changed_files(&run_dir)?;
        let layer_states = read_layer_states(&run_dir)?;

        let result = validate_reviewer_output(&reviewer_output, &changed_files, &layer_states);

        let new_state = if result.valid {
            RunState::ReviewerOutputAccepted
        } else {
            RunState::ReviewerOutputRejected
        };

        write_canonical_output(&run_dir, &result)?;
        write_run_state(&run_dir, new_state)?;

        Ok(())
    }
}

/// Reads the list of changed files from `changed-files.tsv`.
fn read_changed_files(run_dir: &Path) -> Result<Vec<String>, String> {
    let path = run_dir.join("changed-files.tsv");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("read changed-files.tsv: {e}"))?;
    Ok(content
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| l.split('\t').next().unwrap_or(l).trim().to_string())
        .collect())
}

/// Reads the layer completion states from `run-state.json`.
fn read_layer_states(run_dir: &Path) -> Result<Vec<(String, LayerStatus)>, String> {
    let path = run_dir.join("run-state.json");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("read run-state.json: {e}"))?;
    let state: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("parse run-state.json: {e}"))?;

    let layers = state
        .get("layers")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .map(|(name, val)| {
                    let status = match val.get("status").and_then(|s| s.as_str()) {
                        Some("completed") => LayerStatus::Completed,
                        Some("skipped") => LayerStatus::SkippedWithReason,
                        Some("failed") => LayerStatus::Failed,
                        _ => LayerStatus::Completed,
                    };
                    (name.clone(), status)
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(layers)
}

/// Writes the canonical review output as JSON.
fn write_canonical_output(
    run_dir: &Path,
    result: &crate::review::validate::ValidationResult,
) -> Result<(), String> {
    let output_path = run_dir.join(CANONICAL_REVIEW_OUTPUT_FILE);
    let json = serde_json::json!({
        "valid": result.valid,
        "errors": &result.errors,
        "downgrades": result.downgrades.iter().map(|d| serde_json::json!({
            "comment_id": d.comment_id,
            "original_line": d.original_line,
            "new_level": d.new_level,
            "reason": d.reason,
        })).collect::<Vec<_>>(),
    });
    let content = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("serialize canonical output: {e}"))?;
    fs::write(&output_path, &content).map_err(|e| format!("write canonical output: {e}"))?;
    Ok(())
}

/// Writes the updated run state.
fn write_run_state(run_dir: &Path, state: RunState) -> Result<(), String> {
    let path = run_dir.join("run-state.json");

    let mut current: serde_json::Value = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| format!("read run-state.json: {e}"))?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        serde_json::json!({})
    };

    if let Some(obj) = current.as_object_mut() {
        obj.insert("state".to_string(), serde_json::json!(state.as_str()));
        obj.insert("updated_at".to_string(), serde_json::json!(chrono_now()));
    }

    let content =
        serde_json::to_string_pretty(&current).map_err(|e| format!("serialize run-state: {e}"))?;
    fs::write(&path, &content).map_err(|e| format!("write run-state.json: {e}"))?;
    Ok(())
}

/// Returns the current UTC time as an ISO 8601 string for traceability.
fn chrono_now() -> String {
    // time 0.3.x — `now_utc()` is infallible; serialize via serde for ISO 8601.
    let now = time::OffsetDateTime::now_utc();
    serde_json::to_value(now)
        .ok()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_read_changed_files_tsv_single_column() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("changed-files.tsv"), "src/a.rs\nsrc/b.rs\n").unwrap();
        let files = read_changed_files(dir.path()).unwrap();
        assert_eq!(files, vec!["src/a.rs", "src/b.rs"]);
    }

    #[test]
    fn test_read_changed_files_empty_when_missing() {
        let dir = TempDir::new().unwrap();
        let files = read_changed_files(dir.path()).unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_read_changed_files_skips_comments() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("changed-files.tsv"), "# header\nsrc/a.rs\nsrc/b.rs\tcontract\n")
            .unwrap();
        let files = read_changed_files(dir.path()).unwrap();
        assert_eq!(files, vec!["src/a.rs", "src/b.rs"]);
    }

    #[test]
    fn test_read_layer_states_from_run_state() {
        let dir = TempDir::new().unwrap();
        let state = serde_json::json!({
            "state": "test_review_recorded",
            "layers": {
                "diff": {"status": "completed"},
                "whole_file": {"status": "completed"},
                "related_context": {"status": "skipped"},
                "logical_stress": {"status": "failed"},
                "tests": {"status": "completed"}
            }
        });
        fs::write(dir.path().join("run-state.json"), state.to_string()).unwrap();
        let layers = read_layer_states(dir.path()).unwrap();
        assert_eq!(layers.len(), 5);
        // Use key-based lookup since JSON object order is not guaranteed.
        let map: std::collections::HashMap<String, LayerStatus> = layers.into_iter().collect();
        assert_eq!(map["diff"], LayerStatus::Completed);
        assert_eq!(map["related_context"], LayerStatus::SkippedWithReason);
        assert_eq!(map["logical_stress"], LayerStatus::Failed);
    }

    #[test]
    fn test_read_layer_states_empty_when_missing() {
        let dir = TempDir::new().unwrap();
        let layers = read_layer_states(dir.path()).unwrap();
        assert!(layers.is_empty());
    }

    #[test]
    fn test_write_canonical_output_produces_valid_json() {
        let dir = TempDir::new().unwrap();
        let result = crate::review::validate::ValidationResult {
            valid: true,
            errors: Vec::new(),
            downgrades: Vec::new(),
        };
        write_canonical_output(dir.path(), &result).unwrap();
        let content = fs::read_to_string(dir.path().join("canonical-review-output.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["valid"], true);
        assert!(parsed["errors"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_write_run_state_updates_existing() {
        let dir = TempDir::new().unwrap();
        let initial = serde_json::json!({"state": "prepared"});
        fs::write(dir.path().join("run-state.json"), initial.to_string()).unwrap();

        write_run_state(dir.path(), RunState::ReviewerOutputAccepted).unwrap();

        let content = fs::read_to_string(dir.path().join("run-state.json")).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["state"], "reviewer_output_accepted");
    }
}
