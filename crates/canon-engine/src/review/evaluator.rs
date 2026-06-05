use super::diff::validate_and_map_line;
use super::findings::{GithubComment, MissingTest, ReviewCoverage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluatorPayload {
    pub github_comments: Vec<GithubComment>,
    pub missing_tests: Vec<MissingTest>,
    pub review_coverage: Option<ReviewCoverage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Approve,
    Comment,
    RequestChanges,
}

/// Evaluates the LLM-generated payload against the diff.
pub fn evaluate_diff(
    patch: &str,
    changed_files: u32,
    patch_lines: u32,
    llm_payload: &str,
) -> Result<EvaluatorPayload, String> {
    // Large diff threshold check (>20 files or >500 lines)
    let is_large_diff = changed_files > 20 || patch_lines > 500;

    let mut payload: EvaluatorPayload =
        serde_json::from_str(llm_payload).map_err(|e| e.to_string())?;

    if is_large_diff && payload.review_coverage.is_none() {
        return Err(
            "Large diffs (>20 files or >500 lines) require a review_coverage block.".to_string()
        );
    }

    // Map and validate line numbers for github comments
    for comment in &mut payload.github_comments {
        if let (Some(path), Some(line)) = (&comment.path, comment.line) {
            match validate_and_map_line(path, line, patch) {
                Ok(valid_line) => {
                    comment.line = Some(valid_line);
                }
                Err(hunk_fallback) => {
                    // Downgrade finding to hunk or general
                    comment.line = None;
                    if let Some(hunk) = hunk_fallback {
                        comment.hunk_header = Some(hunk);
                    } else {
                        comment.hunk_header = None;
                    }
                }
            }
        }
    }

    // Check SC-003: 100% of missing test findings map to behavior
    for mt in &payload.missing_tests {
        if mt.affected_behavior.trim().is_empty() {
            return Err(format!(
                "MissingTest {} is invalid: affected_behavior must be explicit.",
                mt.id
            ));
        }
    }

    Ok(payload)
}

/// Derives the final decision from the evaluated payload.
pub fn derive_decision(
    payload: &EvaluatorPayload,
    packet: &super::findings::ReviewPacket,
) -> Decision {
    let has_blocking_comment = payload.github_comments.iter().any(|c| c.blocking);
    let has_blocking_test = payload.missing_tests.iter().any(|m| m.blocking);
    let has_deterministic_must_fix = packet
        .findings
        .iter()
        .any(|f| matches!(f.severity, super::findings::FindingSeverity::MustFix));

    if has_blocking_comment || has_blocking_test || has_deterministic_must_fix {
        Decision::RequestChanges
    } else if !payload.github_comments.is_empty()
        || !payload.missing_tests.is_empty()
        || !packet.findings.is_empty()
    {
        Decision::Comment
    } else {
        Decision::Approve
    }
}
