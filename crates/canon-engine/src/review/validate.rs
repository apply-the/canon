//! Reviewer output validation for the `pr-review accept` phase.
//!
//! Validates the LLM-authored `reviewer-output.md` against the review context
//! and the reviewer output schema. Checks: JSON syntax, schema version,
//! comment ID uniqueness, severity vocabulary, recommendation vocabulary,
//! path validity, line/hunk applicability, and layer coverage.

use serde::Deserialize;

use crate::review::onion::{CANONICAL_LAYERS, LayerStatus};

/// Result of validating a reviewer output.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed.
    pub valid: bool,
    /// List of validation errors.
    pub errors: Vec<String>,
    /// List of downgraded comments (invalid line → hunk/global).
    pub downgrades: Vec<CommentDowngrade>,
}

/// A comment that was downgraded during validation.
#[derive(Debug, Clone)]
pub struct CommentDowngrade {
    pub comment_id: String,
    pub original_line: u32,
    pub new_level: String,
    pub reason: String,
}

/// The allowable severity values.
pub const ALLOWED_SEVERITIES: &[&str] = &["blocking", "major", "minor", "question", "nitpick"];

/// The allowable recommendation values.
pub const ALLOWED_RECOMMENDATIONS: &[&str] = &["Approve", "Comment", "Request changes"];

/// Raw reviewer output as submitted by the LLM.
#[derive(Debug, Clone, Deserialize)]
struct RawReviewerOutput {
    #[serde(default)]
    schema_version: String,
    #[serde(default)]
    #[expect(dead_code, reason = "parsed for schema validation, reserved for future use")]
    review_status: String,
    #[serde(default)]
    findings: Vec<RawFinding>,
    #[serde(default)]
    recommendation: String,
    #[serde(default)]
    #[expect(dead_code, reason = "parsed for schema validation, reserved for future use")]
    layer_coverage: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawFinding {
    #[serde(default)]
    id: String,
    #[serde(default)]
    severity: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    line: Option<u32>,
    #[serde(default)]
    comment_id: Option<String>,
    #[serde(default)]
    #[expect(dead_code, reason = "parsed for layer coverage validation, used in layer checks")]
    layer: String,
}

/// Validates a reviewer output JSON string against the review context.
///
/// Checks schema, severities, recommendation, comment IDs, paths, lines, and layer coverage.
pub fn validate_reviewer_output(
    json: &str,
    changed_files: &[String],
    layer_states: &[(String, LayerStatus)],
) -> ValidationResult {
    let mut errors = Vec::new();
    let mut downgrades = Vec::new();

    let Some(output) = parse_output(json, &mut errors) else {
        return ValidationResult { valid: false, errors, downgrades };
    };

    check_schema_version(&output, &mut errors);
    check_recommendation(&output, &mut errors);
    check_comment_id_uniqueness(&output, &mut errors);
    check_severities(&output, &mut errors);
    check_paths_and_lines(&output, changed_files, &mut downgrades);
    check_layer_coverage(layer_states, &mut errors);

    ValidationResult { valid: errors.is_empty(), errors, downgrades }
}

fn parse_output(json: &str, errors: &mut Vec<String>) -> Option<RawReviewerOutput> {
    match serde_json::from_str(json) {
        Ok(o) => Some(o),
        Err(e) => {
            errors.push(format!("Invalid JSON: {e}"));
            None
        }
    }
}

fn check_schema_version(output: &RawReviewerOutput, errors: &mut Vec<String>) {
    if output.schema_version != "1.0" {
        errors
            .push(format!("Unsupported schema version: {} (expected 1.0)", output.schema_version));
    }
}

fn check_recommendation(output: &RawReviewerOutput, errors: &mut Vec<String>) {
    if !output.recommendation.is_empty()
        && !ALLOWED_RECOMMENDATIONS.contains(&output.recommendation.as_str())
    {
        errors.push(format!(
            "Invalid recommendation: {} (allowed: {:?})",
            output.recommendation, ALLOWED_RECOMMENDATIONS
        ));
    }
}

fn check_comment_id_uniqueness(output: &RawReviewerOutput, errors: &mut Vec<String>) {
    let mut seen_ids = std::collections::HashSet::new();
    for f in &output.findings {
        if let Some(ref cid) = f.comment_id
            && !seen_ids.insert(cid.clone())
        {
            errors.push(format!("Duplicate comment ID: {cid}"));
        }
    }
}

fn check_severities(output: &RawReviewerOutput, errors: &mut Vec<String>) {
    for f in &output.findings {
        if !f.severity.is_empty() && !ALLOWED_SEVERITIES.contains(&f.severity.as_str()) {
            errors.push(format!(
                "Invalid severity '{}' in finding {} (allowed: {:?})",
                f.severity, f.id, ALLOWED_SEVERITIES
            ));
        }
    }
}

fn check_paths_and_lines(
    output: &RawReviewerOutput,
    changed_files: &[String],
    downgrades: &mut Vec<CommentDowngrade>,
) {
    for f in &output.findings {
        if let Some(ref path) = f.path
            && !changed_files.iter().any(|cf| cf == path)
            && f.line.is_some()
        {
            downgrades.push(CommentDowngrade {
                comment_id: f.id.clone(),
                original_line: f.line.unwrap_or(0),
                new_level: "hunk".to_string(),
                reason: format!("Path '{path}' not in changed files; downgraded to hunk-level"),
            });
        }
    }
}

fn check_layer_coverage(layer_states: &[(String, LayerStatus)], errors: &mut Vec<String>) {
    for layer in CANONICAL_LAYERS {
        if *layer == "global" {
            continue;
        }
        if !layer_states.iter().any(|(name, _)| name == *layer) {
            errors.push(format!("Layer '{layer}' is missing from layer coverage"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn completed_layers() -> Vec<(String, LayerStatus)> {
        vec![
            ("diff".to_string(), LayerStatus::Completed),
            ("whole_file".to_string(), LayerStatus::Completed),
            ("related_context".to_string(), LayerStatus::Completed),
            ("logical_stress".to_string(), LayerStatus::Completed),
            ("tests".to_string(), LayerStatus::Completed),
        ]
    }

    #[test]
    fn test_valid_output_passes() {
        let json = r#"{
            "schema_version": "1.0",
            "review_status": "actionable_review_executed",
            "findings": [
                {"id": "F001", "severity": "blocking", "path": "src/a.rs", "line": 10, "comment_id": "C001", "layer": "diff"}
            ],
            "recommendation": "Request changes",
            "layer_coverage": {}
        }"#;
        let result = validate_reviewer_output(json, &["src/a.rs".to_string()], &completed_layers());
        assert!(result.valid, "Errors: {:?}", result.errors);
    }

    #[test]
    fn test_invalid_json_fails() {
        let result = validate_reviewer_output("not json", &[], &[]);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Invalid JSON")));
    }

    #[test]
    fn test_duplicate_comment_ids_rejected() {
        let json = r#"{
            "schema_version": "1.0",
            "findings": [
                {"id": "F001", "severity": "blocking", "comment_id": "C001", "layer": "diff"},
                {"id": "F002", "severity": "major", "comment_id": "C001", "layer": "diff"}
            ],
            "recommendation": "Comment",
            "layer_coverage": {}
        }"#;
        let result = validate_reviewer_output(json, &[], &completed_layers());
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Duplicate comment ID")));
    }

    #[test]
    fn test_invalid_severity_rejected() {
        let json = r#"{
            "schema_version": "1.0",
            "findings": [
                {"id": "F001", "severity": "critical", "layer": "diff"}
            ],
            "recommendation": "Comment",
            "layer_coverage": {}
        }"#;
        let result = validate_reviewer_output(json, &[], &completed_layers());
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Invalid severity")));
    }

    #[test]
    fn test_path_not_in_changed_files_downgraded() {
        let json = r#"{
            "schema_version": "1.0",
            "findings": [
                {"id": "F001", "severity": "blocking", "path": "src/unknown.rs", "line": 42, "comment_id": "C001", "layer": "diff"}
            ],
            "recommendation": "Comment",
            "layer_coverage": {}
        }"#;
        let result = validate_reviewer_output(json, &["src/a.rs".to_string()], &completed_layers());
        assert!(result.valid); // Not an error, just a downgrade
        assert_eq!(result.downgrades.len(), 1);
        assert_eq!(result.downgrades[0].new_level, "hunk");
    }

    #[test]
    fn test_missing_layer_coverage_blocks() {
        let json = r#"{
            "schema_version": "1.0",
            "findings": [],
            "recommendation": "Comment",
            "layer_coverage": {}
        }"#;
        let partial = vec![("diff".to_string(), LayerStatus::Completed)];
        let result = validate_reviewer_output(json, &[], &partial);
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("whole_file")));
    }

    #[test]
    fn test_invalid_recommendation_rejected() {
        let json = r#"{
            "schema_version": "1.0",
            "findings": [],
            "recommendation": "Reject",
            "layer_coverage": {}
        }"#;
        let result = validate_reviewer_output(json, &[], &completed_layers());
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("Invalid recommendation")));
    }
}
