//! Onion-layer review state machine.
//!
//! Defines the 14-state lifecycle for the five-layer onion review workflow:
//! diff → whole-file → related-context → logical-stress → tests.
//! Each layer must end in a terminal state before `finalize` can proceed.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// The run state in the onion-layer review lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Prepared,
    AwaitingDiffReview,
    DiffReviewRecorded,
    AwaitingWholeFileReview,
    WholeFileReviewRecorded,
    AwaitingRelatedContextReview,
    RelatedContextReviewRecorded,
    AwaitingStressReview,
    StressReviewRecorded,
    AwaitingTestReview,
    TestReviewRecorded,
    ReviewerOutputAccepted,
    ReviewerOutputRejected,
    Finalized,
}

impl RunState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::AwaitingDiffReview => "awaiting_diff_review",
            Self::DiffReviewRecorded => "diff_review_recorded",
            Self::AwaitingWholeFileReview => "awaiting_whole_file_review",
            Self::WholeFileReviewRecorded => "whole_file_review_recorded",
            Self::AwaitingRelatedContextReview => "awaiting_related_context_review",
            Self::RelatedContextReviewRecorded => "related_context_review_recorded",
            Self::AwaitingStressReview => "awaiting_stress_review",
            Self::StressReviewRecorded => "stress_review_recorded",
            Self::AwaitingTestReview => "awaiting_test_review",
            Self::TestReviewRecorded => "test_review_recorded",
            Self::ReviewerOutputAccepted => "reviewer_output_accepted",
            Self::ReviewerOutputRejected => "reviewer_output_rejected",
            Self::Finalized => "finalized",
        }
    }

    /// Returns the canonical layer name associated with this state, if applicable.
    pub fn layer_name(&self) -> Option<&'static str> {
        match self {
            Self::AwaitingDiffReview | Self::DiffReviewRecorded => Some("diff"),
            Self::AwaitingWholeFileReview | Self::WholeFileReviewRecorded => Some("whole_file"),
            Self::AwaitingRelatedContextReview | Self::RelatedContextReviewRecorded => {
                Some("related_context")
            }
            Self::AwaitingStressReview | Self::StressReviewRecorded => Some("logical_stress"),
            Self::AwaitingTestReview | Self::TestReviewRecorded => Some("tests"),
            _ => None,
        }
    }

    /// Whether this state is a terminal state for the run.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Finalized | Self::ReviewerOutputAccepted | Self::ReviewerOutputRejected
        )
    }

    /// Whether this state is a layer-level terminal state.
    pub fn is_layer_terminal(&self) -> bool {
        matches!(
            self,
            Self::DiffReviewRecorded
                | Self::WholeFileReviewRecorded
                | Self::RelatedContextReviewRecorded
                | Self::StressReviewRecorded
                | Self::TestReviewRecorded
        )
    }
}

/// The terminal status of a single review layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerStatus {
    Completed,
    SkippedWithReason,
    Failed,
}

/// Record produced when a layer is skipped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkipRecord {
    pub layer: String,
    pub reason: String,
    pub decision_source: String,
    pub coverage_impact: String,
    pub downgrades_recommendation: bool,
    pub timestamp: String,
}

/// Record produced when a layer fails.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureRecord {
    pub layer: String,
    pub failure_reason: String,
    pub partial_output_exists: bool,
    pub review_can_continue: bool,
    pub recommendation_impact: String,
    pub timestamp: String,
}

/// The canonical layer names.
pub const CANONICAL_LAYERS: &[&str] =
    &["diff", "whole_file", "related_context", "logical_stress", "tests", "global"];

/// Advances the run state to the next layer's awaiting state.
///
/// Returns the new state or an error if the current state does not support advancing.
pub fn advance_layer(state: RunState) -> Result<RunState, String> {
    match state {
        RunState::DiffReviewRecorded => Ok(RunState::AwaitingWholeFileReview),
        RunState::WholeFileReviewRecorded => Ok(RunState::AwaitingRelatedContextReview),
        RunState::RelatedContextReviewRecorded => Ok(RunState::AwaitingStressReview),
        RunState::StressReviewRecorded => Ok(RunState::AwaitingTestReview),
        _ => Err(format!(
            "cannot advance from state `{}` — current layer must be recorded first",
            state.as_str()
        )),
    }
}

/// Records a layer as completed, advancing to its recorded state.
pub fn record_layer_completed(state: RunState) -> Result<RunState, String> {
    match state {
        RunState::AwaitingDiffReview => Ok(RunState::DiffReviewRecorded),
        RunState::AwaitingWholeFileReview => Ok(RunState::WholeFileReviewRecorded),
        RunState::AwaitingRelatedContextReview => Ok(RunState::RelatedContextReviewRecorded),
        RunState::AwaitingStressReview => Ok(RunState::StressReviewRecorded),
        RunState::AwaitingTestReview => Ok(RunState::TestReviewRecorded),
        _ => Err(format!(
            "cannot record layer completion from state `{}` — not in an awaiting state",
            state.as_str()
        )),
    }
}

/// Creates a skip record for the given layer.
pub fn skip_record(
    layer: &str,
    reason: &str,
    decision_source: &str,
    coverage_impact: &str,
    downgrades_recommendation: bool,
) -> SkipRecord {
    SkipRecord {
        layer: layer.to_string(),
        reason: reason.to_string(),
        decision_source: decision_source.to_string(),
        coverage_impact: coverage_impact.to_string(),
        downgrades_recommendation,
        timestamp: OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default(),
    }
}

/// Creates a failure record for the given layer.
pub fn failure_record(
    layer: &str,
    failure_reason: &str,
    partial_output_exists: bool,
    review_can_continue: bool,
    recommendation_impact: &str,
) -> FailureRecord {
    FailureRecord {
        layer: layer.to_string(),
        failure_reason: failure_reason.to_string(),
        partial_output_exists,
        review_can_continue,
        recommendation_impact: recommendation_impact.to_string(),
        timestamp: OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default(),
    }
}

/// Checks whether `finalize` may proceed given the layer completion states.
///
/// Returns Ok if all 5 layers have terminal states, or a list of missing layers.
pub fn validate_layer_coverage(states: &[(String, LayerStatus)]) -> Result<(), Vec<String>> {
    let mut missing = Vec::new();
    let required = &["diff", "whole_file", "related_context", "logical_stress", "tests"];
    for layer in required {
        if !states.iter().any(|(name, _)| name == *layer) {
            missing.push(layer.to_string());
        }
    }
    if missing.is_empty() { Ok(()) } else { Err(missing) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_state_as_str_all_variants() {
        assert_eq!(RunState::Prepared.as_str(), "prepared");
        assert_eq!(RunState::AwaitingDiffReview.as_str(), "awaiting_diff_review");
        assert_eq!(RunState::DiffReviewRecorded.as_str(), "diff_review_recorded");
        assert_eq!(RunState::AwaitingWholeFileReview.as_str(), "awaiting_whole_file_review");
        assert_eq!(RunState::WholeFileReviewRecorded.as_str(), "whole_file_review_recorded");
        assert_eq!(
            RunState::AwaitingRelatedContextReview.as_str(),
            "awaiting_related_context_review"
        );
        assert_eq!(
            RunState::RelatedContextReviewRecorded.as_str(),
            "related_context_review_recorded"
        );
        assert_eq!(RunState::AwaitingStressReview.as_str(), "awaiting_stress_review");
        assert_eq!(RunState::StressReviewRecorded.as_str(), "stress_review_recorded");
        assert_eq!(RunState::AwaitingTestReview.as_str(), "awaiting_test_review");
        assert_eq!(RunState::TestReviewRecorded.as_str(), "test_review_recorded");
        assert_eq!(RunState::ReviewerOutputAccepted.as_str(), "reviewer_output_accepted");
        assert_eq!(RunState::ReviewerOutputRejected.as_str(), "reviewer_output_rejected");
        assert_eq!(RunState::Finalized.as_str(), "finalized");
    }

    #[test]
    fn test_advance_through_all_layers() {
        let s = record_layer_completed(RunState::AwaitingDiffReview).unwrap();
        assert_eq!(s, RunState::DiffReviewRecorded);
        let s = advance_layer(s).unwrap();
        assert_eq!(s, RunState::AwaitingWholeFileReview);
        let s = record_layer_completed(s).unwrap();
        assert_eq!(s, RunState::WholeFileReviewRecorded);
        let s = advance_layer(s).unwrap();
        assert_eq!(s, RunState::AwaitingRelatedContextReview);
        let s = record_layer_completed(s).unwrap();
        assert_eq!(s, RunState::RelatedContextReviewRecorded);
        let s = advance_layer(s).unwrap();
        assert_eq!(s, RunState::AwaitingStressReview);
        let s = record_layer_completed(s).unwrap();
        assert_eq!(s, RunState::StressReviewRecorded);
        let s = advance_layer(s).unwrap();
        assert_eq!(s, RunState::AwaitingTestReview);
        let s = record_layer_completed(s).unwrap();
        assert_eq!(s, RunState::TestReviewRecorded);
    }

    #[test]
    fn test_advance_from_non_recorded_state_fails() {
        assert!(advance_layer(RunState::Prepared).is_err());
        assert!(advance_layer(RunState::Finalized).is_err());
    }

    #[test]
    fn test_record_completed_from_non_awaiting_state_fails() {
        assert!(record_layer_completed(RunState::Prepared).is_err());
        assert!(record_layer_completed(RunState::Finalized).is_err());
    }

    #[test]
    fn test_is_terminal() {
        assert!(RunState::Finalized.is_terminal());
        assert!(RunState::ReviewerOutputAccepted.is_terminal());
        assert!(RunState::ReviewerOutputRejected.is_terminal());
        assert!(!RunState::Prepared.is_terminal());
        assert!(!RunState::DiffReviewRecorded.is_terminal());
    }

    #[test]
    fn test_layer_name_mapping() {
        assert_eq!(RunState::AwaitingDiffReview.layer_name(), Some("diff"));
        assert_eq!(RunState::DiffReviewRecorded.layer_name(), Some("diff"));
        assert_eq!(RunState::AwaitingWholeFileReview.layer_name(), Some("whole_file"));
        assert_eq!(RunState::WholeFileReviewRecorded.layer_name(), Some("whole_file"));
        assert_eq!(RunState::AwaitingRelatedContextReview.layer_name(), Some("related_context"));
        assert_eq!(RunState::RelatedContextReviewRecorded.layer_name(), Some("related_context"));
        assert_eq!(RunState::AwaitingStressReview.layer_name(), Some("logical_stress"));
        assert_eq!(RunState::StressReviewRecorded.layer_name(), Some("logical_stress"));
        assert_eq!(RunState::AwaitingTestReview.layer_name(), Some("tests"));
        assert_eq!(RunState::TestReviewRecorded.layer_name(), Some("tests"));
        assert_eq!(RunState::Prepared.layer_name(), None);
        assert_eq!(RunState::ReviewerOutputAccepted.layer_name(), None);
        assert_eq!(RunState::Finalized.layer_name(), None);
    }

    #[test]
    fn test_is_layer_terminal() {
        assert!(!RunState::Prepared.is_layer_terminal());
        assert!(!RunState::AwaitingDiffReview.is_layer_terminal());
        assert!(RunState::DiffReviewRecorded.is_layer_terminal());
        assert!(RunState::WholeFileReviewRecorded.is_layer_terminal());
        assert!(RunState::RelatedContextReviewRecorded.is_layer_terminal());
        assert!(RunState::StressReviewRecorded.is_layer_terminal());
        assert!(RunState::TestReviewRecorded.is_layer_terminal());
        assert!(!RunState::ReviewerOutputAccepted.is_layer_terminal());
        assert!(!RunState::Finalized.is_layer_terminal());
    }

    #[test]
    fn test_skip_record_creation() {
        let record = skip_record("logical_stress", "no async code", "operator", "minimal", false);
        assert_eq!(record.layer, "logical_stress");
        assert_eq!(record.reason, "no async code");
        assert!(!record.downgrades_recommendation);
    }

    #[test]
    fn test_failure_record_creation() {
        let record = failure_record("whole_file", "LLM error", false, true, "downgrade_to_comment");
        assert_eq!(record.layer, "whole_file");
        assert!(record.review_can_continue);
        assert_eq!(record.recommendation_impact, "downgrade_to_comment");
    }

    #[test]
    fn test_validate_layer_coverage_all_present() {
        let states = vec![
            ("diff".to_string(), LayerStatus::Completed),
            ("whole_file".to_string(), LayerStatus::Completed),
            ("related_context".to_string(), LayerStatus::SkippedWithReason),
            ("logical_stress".to_string(), LayerStatus::Completed),
            ("tests".to_string(), LayerStatus::Completed),
        ];
        assert!(validate_layer_coverage(&states).is_ok());
    }

    #[test]
    fn test_validate_layer_coverage_missing_layers() {
        let states = vec![
            ("diff".to_string(), LayerStatus::Completed),
            ("whole_file".to_string(), LayerStatus::Completed),
        ];
        let err = validate_layer_coverage(&states).unwrap_err();
        assert!(err.contains(&"related_context".to_string()));
        assert!(err.contains(&"logical_stress".to_string()));
        assert!(err.contains(&"tests".to_string()));
    }
}
