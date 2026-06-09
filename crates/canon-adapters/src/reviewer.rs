//! Provider-neutral reviewer adapter contract for actionable pr-review.
//!
//! The [`ReviewerAdapter`] trait defines the interface that any AI/LLM
//! reviewer must satisfy. Concrete implementations (stub, CLI, local model,
//! remote model) are selected at configuration time.

use serde::{Deserialize, Serialize};

/// Structured output from a reviewer adapter invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerOutput {
    /// Status of the review execution.
    pub status: ReviewerStatus,
    /// The reviewer adapter kind that produced this output.
    pub reviewer: String,
    /// Actionable findings produced by the reviewer.
    pub findings: Vec<ReviewerFinding>,
    /// Review coverage metadata.
    pub coverage: ReviewerCoverage,
}

/// Whether the reviewer executed successfully, failed, or was not configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerStatus {
    /// Reviewer executed and returned valid output.
    Executed,
    /// Reviewer was invoked but returned invalid or no output.
    Failed,
    /// No reviewer adapter is configured.
    NotConfigured,
}

impl ReviewerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Executed => "actionable_review_executed",
            Self::Failed => "actionable_review_failed",
            Self::NotConfigured => "actionable_review_not_configured",
        }
    }
}

/// A single finding from the reviewer, which may become a canonical comment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerFinding {
    /// Stable finding ID (assigned by the reviewer or the runtime).
    pub id: String,
    /// File path, when the finding applies to a specific file.
    pub path: Option<String>,
    /// Line number, when the finding applies to a specific line.
    pub line: Option<u32>,
    /// Side (LEFT/RIGHT) when line is present.
    pub side: Option<String>,
    /// Diff hunk header when exact line is not determined.
    pub hunk_header: Option<String>,
    /// Severity: blocking, major, minor, question, nitpick.
    pub severity: ReviewerSeverity,
    /// Conventional comment type: issue, suggestion, question, nitpick, praise.
    pub kind: String,
    /// Brief summary of the finding.
    pub summary: String,
    /// Why this finding matters.
    pub why_it_matters: String,
    /// Suggested remediation.
    pub suggested_remediation: String,
    /// Optional suggested code change.
    pub suggested_change: Option<String>,
}

/// Severity levels for reviewer findings, ordered from most to least critical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReviewerSeverity {
    Blocking,
    Major,
    Minor,
    Question,
    Nitpick,
}

impl ReviewerSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Question => "question",
            Self::Nitpick => "nitpick",
        }
    }

    /// Severity ordering for sorting: blocking > major > minor > question > nitpick.
    pub fn order(&self) -> u8 {
        match self {
            Self::Blocking => 0,
            Self::Major => 1,
            Self::Minor => 2,
            Self::Question => 3,
            Self::Nitpick => 4,
        }
    }
}

/// Coverage metadata from the reviewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerCoverage {
    /// Total changed files in the diff.
    pub changed_files_count: u32,
    /// Files inspected deeply by the reviewer.
    pub files_inspected_deeply: Vec<String>,
    /// Files skipped by the reviewer.
    pub files_skipped: Vec<String>,
    /// Whether the review was exhaustive.
    pub exhaustive: bool,
    /// Coverage limitations, if any.
    pub limitations: Vec<String>,
}

/// Input to a reviewer adapter.
#[derive(Debug, Clone)]
pub struct ReviewerInput {
    /// The raw diff patch.
    pub patch: String,
    /// List of changed file paths.
    pub changed_files: Vec<String>,
    /// Base ref for the review.
    pub base_ref: String,
    /// Head ref for the review.
    pub head_ref: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reviewer_status_as_str() {
        assert_eq!(ReviewerStatus::Executed.as_str(), "actionable_review_executed");
        assert_eq!(ReviewerStatus::Failed.as_str(), "actionable_review_failed");
        assert_eq!(ReviewerStatus::NotConfigured.as_str(), "actionable_review_not_configured");
    }

    #[test]
    fn test_reviewer_severity_as_str() {
        assert_eq!(ReviewerSeverity::Blocking.as_str(), "blocking");
        assert_eq!(ReviewerSeverity::Major.as_str(), "major");
        assert_eq!(ReviewerSeverity::Minor.as_str(), "minor");
        assert_eq!(ReviewerSeverity::Question.as_str(), "question");
        assert_eq!(ReviewerSeverity::Nitpick.as_str(), "nitpick");
    }

    #[test]
    fn test_reviewer_severity_order() {
        assert_eq!(ReviewerSeverity::Blocking.order(), 0);
        assert_eq!(ReviewerSeverity::Major.order(), 1);
        assert_eq!(ReviewerSeverity::Minor.order(), 2);
        assert_eq!(ReviewerSeverity::Question.order(), 3);
        assert_eq!(ReviewerSeverity::Nitpick.order(), 4);
    }

    #[test]
    fn test_reviewer_severity_ordering() {
        let mut v = vec![
            ReviewerSeverity::Nitpick,
            ReviewerSeverity::Major,
            ReviewerSeverity::Blocking,
            ReviewerSeverity::Question,
            ReviewerSeverity::Minor,
        ];
        v.sort();
        assert_eq!(
            v,
            vec![
                ReviewerSeverity::Blocking,
                ReviewerSeverity::Major,
                ReviewerSeverity::Minor,
                ReviewerSeverity::Question,
                ReviewerSeverity::Nitpick,
            ]
        );
    }

    #[test]
    fn test_reviewer_output_serialization() {
        let output = ReviewerOutput {
            status: ReviewerStatus::Executed,
            reviewer: "test".to_string(),
            findings: vec![],
            coverage: ReviewerCoverage {
                changed_files_count: 1,
                files_inspected_deeply: vec![],
                files_skipped: vec![],
                exhaustive: true,
                limitations: vec![],
            },
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"status\":\"executed\""));
        let parsed: ReviewerOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.status, ReviewerStatus::Executed);
    }

    #[test]
    fn test_reviewer_finding_creation() {
        let finding = ReviewerFinding {
            id: "F001".to_string(),
            path: Some("src/main.rs".to_string()),
            line: Some(42),
            side: Some("RIGHT".to_string()),
            hunk_header: None,
            severity: ReviewerSeverity::Blocking,
            kind: "issue".to_string(),
            summary: "Critical bug".to_string(),
            why_it_matters: "Causes crash".to_string(),
            suggested_remediation: "Add null check".to_string(),
            suggested_change: Some("if x.is_none() { return; }".to_string()),
        };
        assert_eq!(finding.id, "F001");
        assert_eq!(finding.path, Some("src/main.rs".to_string()));
        assert_eq!(finding.severity, ReviewerSeverity::Blocking);
    }
}
