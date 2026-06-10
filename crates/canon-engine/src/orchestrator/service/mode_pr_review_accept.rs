//! Orchestrator for the `pr-review accept` phase.
//!
//! Validates the LLM-authored `reviewer-output.md` against the review context,
//! enforces 7-layer coverage completeness, produces `canonical-review-output.json`,
//! and updates the run state to `reviewer_output_accepted` or `reviewer_output_rejected`.

use std::fs;
use std::path::Path;

use crate::review::onion::{LayerStatus, RunState};
use crate::review::validate::validate_reviewer_output;

use super::EngineService;

/// Filename for the reviewer output submitted by the LLM.
const REVIEWER_OUTPUT_FILE: &str = "reviewer-output.md";

/// Filename for the canonical review output produced by accept.
const CANONICAL_REVIEW_OUTPUT_FILE: &str = "canonical-review-output.json";

/// The seven review layer slugs in order (from spec 075).
pub(crate) const LAYER_SLUGS: &[&str] = &[
    "early-signal",
    "application-source",
    "high-risk-surfaces",
    "related-context",
    "logical-stress",
    "tests",
    "coverage-accounting",
];

impl EngineService {
    /// Runs the accept phase: validates reviewer output, enforces layer
    /// coverage, and persists results.
    pub fn run_pr_review_accept(&self, run_id: &str) -> Result<(), String> {
        let run_dir = self.canon_runtime_dir().join("runs").join(run_id).join("pr-review");
        let reviewer_output_path = run_dir.join(REVIEWER_OUTPUT_FILE);

        let reviewer_output = fs::read_to_string(&reviewer_output_path)
            .map_err(|e| format!("read reviewer output: {e}"))?;

        // ── 7-layer coverage validation (T032, T033, FR-027, FR-028) ──
        check_layer_coverage(&run_dir)?;

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

/// Validates that all 7 layers have either a non-empty `output.md` with
/// content, or a deferral with a non-empty reason recorded.
///
/// Layers with only `instructions.md` but no reviewer `output.md` are
/// rejected. Per FR-027 and FR-028, Canon must not infer layer completion
/// from instruction presence alone.
fn check_layer_coverage(run_dir: &Path) -> Result<(), String> {
    let layers_dir = run_dir.join("layers");
    if !layers_dir.exists() {
        return Ok(());
    }

    let errors: Vec<String> = LAYER_SLUGS
        .iter()
        .enumerate()
        .filter_map(|(idx, slug)| validate_single_layer(&layers_dir, slug, idx + 1))
        .collect();

    if !errors.is_empty() {
        return Err(format!(
            "Layer coverage validation failed:\n{}",
            errors.iter().map(|e| format!("  - {e}")).collect::<Vec<_>>().join("\n")
        ));
    }
    Ok(())
}

/// Validates a single layer directory, returning an error string if the
/// layer is incomplete, or `None` if it passes.
fn validate_single_layer(layers_dir: &Path, slug: &str, ordinal: usize) -> Option<String> {
    let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
    if !layer_dir.exists() {
        return Some(format!("Layer {ordinal} ({slug}): directory missing"));
    }

    let output_path = layer_dir.join("output.md");
    let output_content = fs::read_to_string(&output_path).ok();
    let has_output =
        output_content.as_ref().map(|c| c.trim().len() > PLACEHOLDER_MIN_LEN).unwrap_or(false);

    let deferred_path = layer_dir.join("deferral.toml");
    let has_deferral = deferred_path.exists();

    if !has_output && !has_deferral {
        return Some(format!(
            "Layer {ordinal} ({slug}): missing output.md (non-placeholder) and no deferral recorded"
        ));
    }

    if has_deferral {
        let content = fs::read_to_string(&deferred_path).unwrap_or_default();
        if !content.contains("reason") || content.trim().len() < 20 {
            return Some(format!(
                "Layer {ordinal} ({slug}): deferral present but reason is empty or missing"
            ));
        }
        return None;
    }

    if let Some(ref content) = output_content
        && let Err(rejection) = validate_layer_output_quality(slug, ordinal, content)
    {
        return Some(rejection);
    }

    None
}

/// Minimum length of a non-placeholder output.md to distinguish from the
/// empty placeholder that defaults to `"# {slug} Output\n\n*No output yet.*\n"`.
const PLACEHOLDER_MIN_LEN: usize = 30;

/// Validates that a layer's output meets quality standards beyond mere
/// existence. Returns `Err(reason)` if the output is too generic or does
/// not reference concrete review targets.
///
/// Canon-executed layers (early-signal, coverage-accounting) are exempt
/// from target-reference validation since they produce deterministic
/// findings.
fn validate_layer_output_quality(slug: &str, ordinal: usize, content: &str) -> Result<(), String> {
    // Canon-executed layers are validated differently
    if slug == "early-signal" || slug == "coverage-accounting" {
        return Ok(());
    }

    let trimmed = content.trim();

    // Reject output that's just a generic header with no substantive content
    if trimmed.len() < 100 {
        return Err(format!(
            "Layer {ordinal} ({slug}): output.md content is too short ({len} chars). \
             Must contain substantive review findings or explicit no-finding records.",
            len = trimmed.len(),
        ));
    }

    // Reject output that doesn't reference the layer name (check both
    // hyphenated slug form and human-readable space-separated form)
    let lower = trimmed.to_lowercase();
    let human_form = slug.to_lowercase().replace('-', " ");
    if !lower.contains(slug) && !lower.contains(&human_form) {
        return Err(format!(
            "Layer {ordinal} ({slug}): output.md does not reference the layer name. \
             Output must be layer-specific, not a generic placeholder."
        ));
    }

    // Reject output with no finding or reviewed-file markers for semantic layers
    let has_finding = trimmed.contains("### Finding")
        || trimmed.contains("### Reviewed")
        || trimmed.contains("**Severity**")
        || trimmed.contains("**Path**");
    if !has_finding {
        return Err(format!(
            "Layer {ordinal} ({slug}): output.md contains no finding or reviewed-file records. \
             Expected `### Finding` or `### Reviewed` blocks referencing concrete targets."
        ));
    }

    Ok(())
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

    #[test]
    fn check_layer_coverage_passes_when_layers_dir_is_missing() {
        let dir = TempDir::new().unwrap();
        // No layers dir at all — should pass (fall through)
        assert!(check_layer_coverage(dir.path()).is_ok());
    }

    #[test]
    fn check_layer_coverage_rejects_missing_layer_output_and_deferral() {
        let dir = TempDir::new().unwrap();
        let layers_dir = dir.path().join("layers");
        // Layer 1 has valid output (longer than placeholder)
        let layer1 = layers_dir.join("01-early-signal");
        fs::create_dir_all(&layer1).unwrap();
        fs::write(
            layer1.join("output.md"),
            "# Early Signal Pass Review\n\nAll checks passed. No issues found.\n\nThis is valid output.\n",
        ).unwrap();
        // Layer 2 has no output.md and no deferral
        let layer2 = layers_dir.join("02-application-source");
        fs::create_dir_all(&layer2).unwrap();

        let result = check_layer_coverage(dir.path());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("application-source"), "expected layer 2 error, got: {err}");
    }

    #[test]
    fn check_layer_coverage_accepts_deferral() {
        let dir = TempDir::new().unwrap();
        let layers_dir = dir.path().join("layers");
        for (idx, slug) in LAYER_SLUGS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = layers_dir.join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            if ordinal == 4 {
                fs::write(
                    layer_dir.join("deferral.toml"),
                    "reason = \"edge-case analysis deferred due to time budget\"\n",
                )
                .unwrap();
            } else {
                let content = format!(
                    "# {slug} Review\n\nDetailed findings for layer {ordinal}.\n\n### Reviewed: src/example.rs\n- **Depth**: deep\n- **Concerns inspected**: correctness, error handling\n- **Result**: no finding\n\nReview complete.\n"
                );
                fs::write(layer_dir.join("output.md"), content).unwrap();
            }
        }
        assert!(check_layer_coverage(dir.path()).is_ok());
    }

    // ── run_pr_review_accept integration tests ───────────────────────────

    /// Helper: sets up a complete `.canon/runs/{run_id}/pr-review/` directory
    /// for `run_pr_review_accept`.
    fn setup_accept_run_dir(workspace: &std::path::Path, run_id: &str) {
        let run_dir = workspace.join(".canon").join("runs").join(run_id).join("pr-review");
        fs::create_dir_all(&run_dir).unwrap();

        // Valid reviewer output JSON
        let reviewer_output = serde_json::json!({
            "schema_version": "1.0",
            "findings": [
                {
                    "id": "f1",
                    "severity": "minor",
                    "path": "src/a.rs",
                    "line": 10,
                    "comment_id": "c1",
                    "layer": "early-signal"
                }
            ],
            "recommendation": "Approve"
        });
        fs::write(run_dir.join("reviewer-output.md"), reviewer_output.to_string()).unwrap();

        // Changed files
        fs::write(run_dir.join("changed-files.tsv"), "src/a.rs\n").unwrap();

        // Run state with layers
        let run_state = serde_json::json!({
            "state": "prepared",
            "layers": {
                "early-signal": {"status": "completed"},
                "application-source": {"status": "completed"},
                "high-risk-surfaces": {"status": "completed"},
                "related-context": {"status": "skipped"},
                "logical-stress": {"status": "completed"},
                "tests": {"status": "completed"},
                "coverage-accounting": {"status": "completed"}
            }
        });
        fs::write(run_dir.join("run-state.json"), run_state.to_string()).unwrap();

        // 7 layer directories with output.md
        for (idx, slug) in LAYER_SLUGS.iter().enumerate() {
            let ordinal = idx + 1;
            let layer_dir = run_dir.join("layers").join(format!("{:02}-{}", ordinal, slug));
            fs::create_dir_all(&layer_dir).unwrap();
            let output = format!(
                "# {slug} Review\n\nFindings for layer {ordinal}. All checks passed.\n\n### Reviewed: src/example.rs\n- **Depth**: deep\n- **Concerns inspected**: correctness, error handling\n- **Result**: no finding\n\nDetailed analysis complete.\n"
            );
            fs::write(layer_dir.join("output.md"), output).unwrap();
        }
    }

    #[test]
    fn run_pr_review_accept_succeeds_with_full_fixture() {
        let workspace = TempDir::new().unwrap();
        setup_accept_run_dir(workspace.path(), "test-accept-engine");
        let service = EngineService::new(workspace.path());
        let result = service.run_pr_review_accept("test-accept-engine");
        assert!(result.is_ok(), "expected ok, got {:?}", result.err());

        // Verify canonical output was written
        let output_path = workspace
            .path()
            .join(".canon")
            .join("runs")
            .join("test-accept-engine")
            .join("pr-review")
            .join("canonical-review-output.json");
        assert!(output_path.exists(), "canonical output not written");
    }

    #[test]
    fn run_pr_review_accept_fails_when_reviewer_output_missing() {
        let workspace = TempDir::new().unwrap();
        let run_dir =
            workspace.path().join(".canon").join("runs").join("no-output").join("pr-review");
        fs::create_dir_all(&run_dir).unwrap();
        // No reviewer-output.md
        let service = EngineService::new(workspace.path());
        let result = service.run_pr_review_accept("no-output");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("read reviewer output"));
    }
}
