//! Coverage-aware review types for PR review rendering.
//!
//! These types enable Canon to classify changed files into risk buckets,
//! track per-file coverage status, determine the overall review type
//! (complete, focused-risk, partial, governance-only), compute confidence
//! levels, and gate approval readiness.
//!
//! All types are serializable so they can be embedded in review artifacts
//! (JSON, TOML, Markdown tables).

use serde::{Deserialize, Serialize};

/// Classification bucket for a changed file.
///
/// Determined by path-pattern heuristics during the prepare phase.
/// Canon never performs semantic content analysis — only path-based
/// classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileBucket {
    /// Application source files (`.rs` under `src/`, excluding `tests/`).
    ApplicationSource,
    /// Test files (`.rs` under `tests/`, `test/`, `testing/`).
    Tests,
    /// API contract files (JSON/YAML under `api/`, `openapi/`, `contracts/`,
    /// `schemas/`).
    ApiContracts,
    /// Database migration files (SQL files, files under `migrations/`, `etl/`).
    DatabaseMigrations,
    /// Configuration files (`.toml`, `.json`, `.yaml`, `.env` at root or under
    /// `config/`).
    Configuration,
    /// Build/CI files (`.github/workflows/`, `Dockerfile`, `Makefile`, etc.).
    BuildCi,
    /// Documentation files (`.md`, `.adoc`, `.rst`).
    Documentation,
    /// Generated or vendored files (`generated/`, `vendor/`, `node_modules/`,
    /// `dist/`).
    GeneratedOrVendor,
    /// Asset files (images, fonts, binaries).
    Assets,
    /// Files that do not match any known bucket pattern.
    Unknown,
}

/// Per-file coverage status within a review.
///
/// Assigned during the finalize phase based on which layers actually
/// inspected the file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageStatus {
    /// File was fully reviewed by the application-source layer (layer 2).
    DeepReviewed,
    /// File received diff-level or surface-level review (layers 3-6).
    LightReviewed,
    /// File appears in the context index but was not directly reviewed.
    IndexedOnly,
    /// File was explicitly skipped with a recorded reason.
    SkippedWithReason(String),
    /// File was not reviewed at all.
    NotReviewed,
}

/// The type of review performed.
///
/// Determined from the ratio of deeply-reviewed files to total changed files
/// and the number of completed review layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewType {
    /// All (or nearly all) changed files were deeply reviewed.
    CompleteReview,
    /// A focused subset of high-risk files was deeply reviewed; most files
    /// received light or no review.
    FocusedRiskReview,
    /// Only a small fraction of files received any review.
    PartialReview,
    /// No semantic review layers were completed; only governance-level
    /// checks (early signal, validation) were applied.
    GovernanceOnlyReview,
}

/// Confidence level in the review's completeness.
///
/// Computed from coverage depth, layer completion, and whether the early
/// signal pass was skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
    /// Low confidence: most files were not reviewed or multiple layers were
    /// deferred.
    Low,
    /// Medium confidence: a substantial subset was reviewed, or early signal
    /// was skipped.
    Medium,
    /// High confidence: most files were deeply reviewed and all layers were
    /// completed.
    High,
}

/// Whether the review packet is ready for approval.
///
/// A review is ready only when coverage is sufficient (CompleteReview or
/// FocusedRiskReview with high confidence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalReadiness {
    /// The review has sufficient coverage for approval.
    Ready,
    /// The review does not have sufficient coverage; additional review is
    /// required before approval.
    NotReady,
}

/// Aggregated coverage statistics for a single file bucket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BucketCoverage {
    /// The bucket being reported.
    pub bucket: FileBucket,
    /// Total number of changed files in this bucket.
    pub total: usize,
    /// Number of deeply reviewed files.
    pub deep_reviewed: usize,
    /// Number of light-reviewed files.
    pub light_reviewed: usize,
    /// Number of indexed-only files.
    pub indexed_only: usize,
    /// Number of skipped files.
    pub skipped: usize,
    /// Number of not-reviewed files.
    pub not_reviewed: usize,
}

impl BucketCoverage {
    /// Creates a new empty `BucketCoverage` for the given bucket.
    pub const fn empty(bucket: FileBucket) -> Self {
        Self {
            bucket,
            total: 0,
            deep_reviewed: 0,
            light_reviewed: 0,
            indexed_only: 0,
            skipped: 0,
            not_reviewed: 0,
        }
    }
}

/// The complete coverage summary produced during finalize.
///
/// Drives the review type, confidence level, approval readiness, global
/// comments, and follow-up requirements rendered into the final artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageSummary {
    /// Per-bucket coverage statistics.
    pub buckets: Vec<BucketCoverage>,
    /// The determined review type.
    pub review_type: ReviewType,
    /// The computed confidence level.
    pub confidence: ConfidenceLevel,
    /// Whether the review packet is ready for approval.
    pub approval_readiness: ApprovalReadiness,
    /// Total number of changed files.
    pub total_changed: usize,
    /// Total number of deeply reviewed files.
    pub total_deep_reviewed: usize,
    /// Number of files that were skipped with a reason.
    pub total_skipped: usize,
    /// Whether the early signal pass was skipped.
    pub early_signal_skipped: bool,
    /// Number of review layers that were deferred (via deferral.toml).
    pub deferred_layer_count: usize,
}

impl CoverageSummary {
    /// Returns the review type as a human-readable label.
    pub fn review_type_label(&self) -> &'static str {
        match self.review_type {
            ReviewType::CompleteReview => "Complete Review",
            ReviewType::FocusedRiskReview => "Focused Risk Review",
            ReviewType::PartialReview => "Partial Review",
            ReviewType::GovernanceOnlyReview => "Governance-Only Review",
        }
    }

    /// Returns the confidence level as a human-readable label.
    pub fn confidence_label(&self) -> &'static str {
        match self.confidence {
            ConfidenceLevel::Low => "low",
            ConfidenceLevel::Medium => "medium",
            ConfidenceLevel::High => "high",
        }
    }

    /// Returns the approval readiness as a human-readable label.
    pub const fn approval_readiness_label(&self) -> &'static str {
        match self.approval_readiness {
            ApprovalReadiness::Ready => "ready",
            ApprovalReadiness::NotReady => "not ready",
        }
    }

    /// Returns the ratio of deeply reviewed files to total changed files.
    pub fn deep_review_ratio(&self) -> f64 {
        if self.total_changed == 0 {
            return 1.0;
        }
        self.total_deep_reviewed as f64 / self.total_changed as f64
    }
}

/// A high-risk bucket that requires follow-up when not deeply reviewed.
pub const HIGH_RISK_BUCKETS: &[FileBucket] =
    &[FileBucket::ApplicationSource, FileBucket::ApiContracts, FileBucket::DatabaseMigrations];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_coverage_empty_is_all_zeros() {
        let bc = BucketCoverage::empty(FileBucket::ApplicationSource);
        assert_eq!(bc.total, 0);
        assert_eq!(bc.deep_reviewed, 0);
        assert_eq!(bc.light_reviewed, 0);
        assert_eq!(bc.indexed_only, 0);
        assert_eq!(bc.skipped, 0);
        assert_eq!(bc.not_reviewed, 0);
    }

    #[test]
    fn review_type_labels_are_human_readable() {
        let summary = CoverageSummary {
            buckets: Vec::new(),
            review_type: ReviewType::FocusedRiskReview,
            confidence: ConfidenceLevel::Medium,
            approval_readiness: ApprovalReadiness::NotReady,
            total_changed: 261,
            total_deep_reviewed: 10,
            total_skipped: 0,
            early_signal_skipped: false,
            deferred_layer_count: 0,
        };
        assert_eq!(summary.review_type_label(), "Focused Risk Review");
        assert_eq!(summary.confidence_label(), "medium");
        assert_eq!(summary.approval_readiness_label(), "not ready");
    }

    #[test]
    fn deep_review_ratio_calculates_correctly() {
        let summary = CoverageSummary {
            buckets: Vec::new(),
            review_type: ReviewType::PartialReview,
            confidence: ConfidenceLevel::Low,
            approval_readiness: ApprovalReadiness::NotReady,
            total_changed: 200,
            total_deep_reviewed: 50,
            total_skipped: 0,
            early_signal_skipped: false,
            deferred_layer_count: 0,
        };
        assert!((summary.deep_review_ratio() - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn deep_review_ratio_zero_total_is_one() {
        let summary = CoverageSummary {
            buckets: Vec::new(),
            review_type: ReviewType::CompleteReview,
            confidence: ConfidenceLevel::High,
            approval_readiness: ApprovalReadiness::Ready,
            total_changed: 0,
            total_deep_reviewed: 0,
            total_skipped: 0,
            early_signal_skipped: false,
            deferred_layer_count: 0,
        };
        assert!((summary.deep_review_ratio() - 1.0).abs() < f64::EPSILON);
    }
}
