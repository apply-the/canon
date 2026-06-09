//! Deterministic stub reviewer adapter for testing.
//!
//! Returns fixed structured findings so tests can verify the full
//! pr-review pipeline without live LLM calls.

use crate::reviewer::{
    ReviewerCoverage, ReviewerFinding, ReviewerInput, ReviewerOutput, ReviewerStatus,
};

/// A reviewer adapter that returns pre-configured stub output.
///
/// The stub does not inspect the actual diff. It returns whatever
/// findings were configured at construction time.
pub struct StubReviewerAdapter {
    status: ReviewerStatus,
    findings: Vec<ReviewerFinding>,
    coverage: ReviewerCoverage,
}

impl StubReviewerAdapter {
    /// Creates a stub that returns the given findings and coverage.
    pub fn new(
        status: ReviewerStatus,
        findings: Vec<ReviewerFinding>,
        coverage: ReviewerCoverage,
    ) -> Self {
        Self { status, findings, coverage }
    }

    /// Creates an empty stub with executed status (valid empty review).
    pub fn empty_executed(changed_files: Vec<String>) -> Self {
        Self {
            status: ReviewerStatus::Executed,
            findings: Vec::new(),
            coverage: ReviewerCoverage {
                changed_files_count: changed_files.len() as u32,
                files_inspected_deeply: changed_files,
                files_skipped: Vec::new(),
                exhaustive: true,
                limitations: Vec::new(),
            },
        }
    }

    /// Creates a failed stub.
    pub fn failed() -> Self {
        Self {
            status: ReviewerStatus::Failed,
            findings: Vec::new(),
            coverage: ReviewerCoverage {
                changed_files_count: 0,
                files_inspected_deeply: Vec::new(),
                files_skipped: Vec::new(),
                exhaustive: false,
                limitations: vec!["Reviewer adapter failed to produce valid output.".to_string()],
            },
        }
    }

    /// Creates a not-configured stub.
    pub fn not_configured(changed_files: Vec<String>) -> Self {
        let count = changed_files.len() as u32;
        Self {
            status: ReviewerStatus::NotConfigured,
            findings: Vec::new(),
            coverage: ReviewerCoverage {
                changed_files_count: count,
                files_inspected_deeply: Vec::new(),
                files_skipped: changed_files,
                exhaustive: false,
                limitations: vec!["No actionable reviewer adapter is configured. Only governance inspection was performed.".to_string()],
            },
        }
    }

    /// Invokes the stub reviewer, returning the pre-configured output.
    pub fn review(&self, _input: &ReviewerInput) -> ReviewerOutput {
        ReviewerOutput {
            status: self.status,
            reviewer: "stub-reviewer".to_string(),
            findings: self.findings.clone(),
            coverage: self.coverage.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reviewer::ReviewerSeverity;

    #[test]
    fn test_empty_executed_returns_executed_status() {
        let stub = StubReviewerAdapter::empty_executed(vec!["src/a.rs".to_string()]);
        let input = ReviewerInput {
            patch: String::new(),
            changed_files: vec!["src/a.rs".to_string()],
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
        };
        let output = stub.review(&input);
        assert_eq!(output.status, ReviewerStatus::Executed);
        assert_eq!(output.reviewer, "stub-reviewer");
        assert_eq!(output.coverage.changed_files_count, 1);
        assert!(output.coverage.exhaustive);
        assert!(output.findings.is_empty());
    }

    #[test]
    fn test_failed_returns_failed_status() {
        let stub = StubReviewerAdapter::failed();
        let input = ReviewerInput {
            patch: String::new(),
            changed_files: vec![],
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
        };
        let output = stub.review(&input);
        assert_eq!(output.status, ReviewerStatus::Failed);
        assert!(!output.coverage.exhaustive);
        assert_eq!(output.coverage.limitations.len(), 1);
    }

    #[test]
    fn test_not_configured_returns_not_configured_status() {
        let files = vec!["src/a.rs".to_string(), "src/b.rs".to_string()];
        let stub = StubReviewerAdapter::not_configured(files.clone());
        let input = ReviewerInput {
            patch: String::new(),
            changed_files: files,
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
        };
        let output = stub.review(&input);
        assert_eq!(output.status, ReviewerStatus::NotConfigured);
        assert_eq!(output.coverage.changed_files_count, 2);
        assert_eq!(output.coverage.files_skipped.len(), 2);
        assert!(!output.coverage.exhaustive);
    }

    #[test]
    fn test_new_with_custom_findings() {
        let findings = vec![ReviewerFinding {
            id: "F001".to_string(),
            path: Some("src/a.rs".to_string()),
            line: Some(10),
            side: Some("RIGHT".to_string()),
            hunk_header: None,
            severity: ReviewerSeverity::Blocking,
            kind: "issue".to_string(),
            summary: "Critical bug".to_string(),
            why_it_matters: "Security".to_string(),
            suggested_remediation: "Fix it".to_string(),
            suggested_change: None,
        }];
        let coverage = ReviewerCoverage {
            changed_files_count: 1,
            files_inspected_deeply: vec!["src/a.rs".to_string()],
            files_skipped: vec![],
            exhaustive: true,
            limitations: vec![],
        };
        let stub = StubReviewerAdapter::new(ReviewerStatus::Executed, findings, coverage);
        let input = ReviewerInput {
            patch: String::new(),
            changed_files: vec!["src/a.rs".to_string()],
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
        };
        let output = stub.review(&input);
        assert_eq!(output.findings.len(), 1);
        assert_eq!(output.findings[0].id, "F001");
    }
}
