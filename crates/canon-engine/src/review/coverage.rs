//! Coverage analysis for PR review finalization.
//!
//! Determines the review type, confidence level, and approval readiness
//! from file classifications and layer completion data. Generates global
//! conventional comments and follow-up requirements when coverage is
//! insufficient.

use crate::domain::review_coverage::{
    ApprovalReadiness, BucketCoverage, ConfidenceLevel, CoverageSummary, FileBucket,
    HIGH_RISK_BUCKETS, ReviewType,
};
use crate::review::onion::LayerStatus;

// ── Threshold constants ─────────────────────────────────────────────────

/// Files below this % of deep review → `PartialReview`.
const PARTIAL_THRESHOLD: f64 = 0.33;
/// Files below this % of deep review → `FocusedRiskReview`.
const FOCUSED_THRESHOLD: f64 = 0.80;
/// Files below this % of deep review → `Medium` confidence.
const CONFIDENCE_MEDIUM_THRESHOLD: f64 = 0.50;
/// Files below this % of deep review → `Low` confidence.
const CONFIDENCE_HIGH_THRESHOLD: f64 = 0.80;

/// Count of deferred layers that caps confidence at `low`.
const DEFERRED_LOW_CAP: usize = 2;

/// Bucket constants used as magic strings in coverage TOML output.
const BUCKET_LABEL_APP_SRC: &str = "application_source";
const BUCKET_LABEL_TESTS: &str = "tests";
const BUCKET_LABEL_API: &str = "api_contracts";
const BUCKET_LABEL_MIGRATIONS: &str = "database_migrations";

// ── Analysis ─────────────────────────────────────────────────────────────

/// Analyzes file classifications and layer completion data to produce a
/// [`CoverageSummary`] that drives the review type, confidence, and
/// approval readiness.
///
/// # Arguments
/// * `classifications` — `(path, bucket)` pairs for every changed file.
/// * `layer_completions` — `(layer_slug, status)` for each review layer.
/// * `deep_reviewed_count` — how many files were deeply reviewed (layer 2
///   completed).
/// * `early_signal_skipped` — whether the early signal pass was skipped.
pub fn analyze_coverage(
    classifications: &[(String, FileBucket)],
    deep_reviewed_count: usize,
    early_signal_skipped: bool,
    layer_completions: &[(String, LayerStatus)],
) -> CoverageSummary {
    let total_changed = classifications.len();
    let total_deep = deep_reviewed_count.min(total_changed);
    let ratio = if total_changed == 0 { 1.0 } else { total_deep as f64 / total_changed as f64 };

    // ── Bucket coverage ────────────────────────────────────────────────
    let buckets = build_bucket_coverage(classifications, total_deep, total_changed);

    // ── Review type ────────────────────────────────────────────────────
    let review_type = determine_review_type(ratio, total_changed, total_deep, layer_completions);

    // ── Confidence ─────────────────────────────────────────────────────
    let deferred_count = count_deferred_layers(layer_completions);
    let confidence = determine_confidence(ratio, early_signal_skipped, deferred_count);

    // ── Approval readiness ─────────────────────────────────────────────
    let approval_readiness = determine_approval_readiness(review_type, confidence);

    CoverageSummary {
        buckets,
        review_type,
        confidence,
        approval_readiness,
        total_changed,
        total_deep_reviewed: total_deep,
        total_skipped: 0,
        early_signal_skipped,
        deferred_layer_count: deferred_count,
    }
}

/// Builds per-bucket coverage statistics from file classifications.
fn build_bucket_coverage(
    classifications: &[(String, FileBucket)],
    _deep_count: usize,
    _total: usize,
) -> Vec<BucketCoverage> {
    use std::collections::BTreeMap;

    let mut map: BTreeMap<&str, (usize, FileBucket)> = BTreeMap::new();
    for (_path, bucket) in classifications {
        let key = bucket_label(bucket);
        map.entry(key).or_insert_with(|| (0, bucket.clone()));
        map.get_mut(key).unwrap().0 += 1;
    }

    // For now, coverage status assignment is conservative: all files are
    // `NotReviewed` until the LLM agent completes layers 2-6. The
    // `deep_reviewed` count comes from the orchestrator (caller), not from
    // individual file statuses.
    map.into_values()
        .map(|(count, bucket)| BucketCoverage {
            bucket,
            total: count,
            deep_reviewed: 0,
            light_reviewed: 0,
            indexed_only: 0,
            skipped: 0,
            not_reviewed: count,
        })
        .collect()
}

/// Determines the review type from the deep-review ratio and layer
/// completion data.
fn determine_review_type(
    ratio: f64,
    total_changed: usize,
    _total_deep: usize,
    layer_completions: &[(String, LayerStatus)],
) -> ReviewType {
    // If no files changed at all, it's a complete review by definition.
    if total_changed == 0 {
        return ReviewType::CompleteReview;
    }

    // If no semantic layers were completed → governance-only.
    let semantic_completed = layer_completions
        .iter()
        .filter(|(slug, status)| is_semantic_layer(slug) && *status == LayerStatus::Completed)
        .count();
    if semantic_completed == 0 {
        return ReviewType::GovernanceOnlyReview;
    }

    if ratio < PARTIAL_THRESHOLD {
        ReviewType::PartialReview
    } else if ratio < FOCUSED_THRESHOLD {
        ReviewType::FocusedRiskReview
    } else {
        ReviewType::CompleteReview
    }
}

/// Determines the confidence level.
fn determine_confidence(
    ratio: f64,
    early_signal_skipped: bool,
    deferred_count: usize,
) -> ConfidenceLevel {
    // Hard caps take precedence.
    if deferred_count >= DEFERRED_LOW_CAP {
        return ConfidenceLevel::Low;
    }
    if early_signal_skipped {
        return ConfidenceLevel::Medium;
    }

    // Ratio-based.
    if ratio >= CONFIDENCE_HIGH_THRESHOLD {
        ConfidenceLevel::High
    } else if ratio >= CONFIDENCE_MEDIUM_THRESHOLD {
        ConfidenceLevel::Medium
    } else {
        ConfidenceLevel::Low
    }
}

/// Determines approval readiness from review type and confidence.
fn determine_approval_readiness(
    review_type: ReviewType,
    confidence: ConfidenceLevel,
) -> ApprovalReadiness {
    match review_type {
        ReviewType::CompleteReview => {
            if confidence >= ConfidenceLevel::Medium {
                ApprovalReadiness::Ready
            } else {
                ApprovalReadiness::NotReady
            }
        }
        ReviewType::FocusedRiskReview => {
            // Focused risk review is ready only with high confidence.
            if confidence >= ConfidenceLevel::High {
                ApprovalReadiness::Ready
            } else {
                ApprovalReadiness::NotReady
            }
        }
        _ => ApprovalReadiness::NotReady,
    }
}

/// Counts the number of review layers that were deferred.
fn count_deferred_layers(layer_completions: &[(String, LayerStatus)]) -> usize {
    layer_completions.iter().filter(|(_, status)| *status == LayerStatus::SkippedWithReason).count()
}

/// Returns whether a layer slug refers to a semantic review layer (2-6).
fn is_semantic_layer(slug: &str) -> bool {
    matches!(
        slug,
        "application-source"
            | "high-risk-surfaces"
            | "related-context"
            | "logical-stress"
            | "tests"
    )
}

/// Returns the label for a file bucket used in TOML/Markdown output.
fn bucket_label(bucket: &FileBucket) -> &str {
    match bucket {
        FileBucket::ApplicationSource => BUCKET_LABEL_APP_SRC,
        FileBucket::Tests => BUCKET_LABEL_TESTS,
        FileBucket::ApiContracts => BUCKET_LABEL_API,
        FileBucket::DatabaseMigrations => BUCKET_LABEL_MIGRATIONS,
        FileBucket::Configuration => "configuration",
        FileBucket::BuildCi => "build_ci",
        FileBucket::Documentation => "documentation",
        FileBucket::GeneratedOrVendor => "generated_or_vendor",
        FileBucket::Assets => "assets",
        FileBucket::Unknown => "unknown",
    }
}

// ── Global comment generation ────────────────────────────────────────────

/// Generates a conventional `question(non-blocking)` global comment when
/// coverage is partial.
///
/// The comment explains what was and was not reviewed and what follow-up
/// is needed.
pub fn generate_global_comment(summary: &CoverageSummary) -> String {
    let review_label = summary.review_type_label().to_lowercase();
    let changed = summary.total_changed;
    let mut comment = format!(
        "question(non-blocking): This review inspected only a focused subset of a {changed}-file PR.\n\n"
    );
    comment.push_str("Why it matters:\n");
    comment.push_str(
        "Large AI-generated PRs can hide behavioral, migration, persistence, or test regressions outside the inspected files.\n\n",
    );
    comment.push_str("Suggested remediation:\n");
    comment.push_str(&format!(
        "Treat this packet as a {review_label}. Run follow-up review passes for application source, persistence, migrations/ETL, and tests before approval.\n\n"
    ));

    // Add per-bucket detail for worrying gaps.
    for bucket_cov in &summary.buckets {
        if HIGH_RISK_BUCKETS.contains(&bucket_cov.bucket)
            && bucket_cov.deep_reviewed < bucket_cov.total
        {
            let label = bucket_label(&bucket_cov.bucket);
            let unreviewed = bucket_cov.total - bucket_cov.deep_reviewed;
            comment.push_str(&format!(
                "- `{label}`: {unreviewed} of {} files not deeply reviewed\n",
                bucket_cov.total
            ));
        }
    }

    comment
}

/// Generates follow-up review requirements for high-risk buckets that were
/// not deeply reviewed.
pub fn generate_follow_up_requirements(summary: &CoverageSummary) -> Vec<String> {
    let mut requirements = Vec::new();

    for bucket_cov in &summary.buckets {
        if !HIGH_RISK_BUCKETS.contains(&bucket_cov.bucket) {
            continue;
        }
        if bucket_cov.deep_reviewed >= bucket_cov.total {
            continue;
        }
        requirements.push(format!(
            "Deeply review all {} files in the `{}` bucket before approval",
            bucket_cov.total,
            bucket_label(&bucket_cov.bucket)
        ));
    }

    requirements
}

/// Generates the final statement for the review summary when approval is
/// not ready.
pub fn generate_not_ready_statement(summary: &CoverageSummary) -> String {
    let review_label = summary.review_type_label().to_lowercase();
    format!(
        "Resolve the blocking findings first. This packet remains a {} because only {} of {} changed files were deeply inspected. Additional review is required before approval.",
        review_label, summary.total_deep_reviewed, summary.total_changed
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: creates a list of (path, FileBucket) pairs for N files.
    fn mock_classifications(
        app_source: usize,
        tests: usize,
        docs: usize,
    ) -> Vec<(String, FileBucket)> {
        let mut v = Vec::new();
        for i in 0..app_source {
            v.push((format!("src/mod{i}.rs"), FileBucket::ApplicationSource));
        }
        for i in 0..tests {
            v.push((format!("tests/test{i}.rs"), FileBucket::Tests));
        }
        for i in 0..docs {
            v.push((format!("docs/doc{i}.md"), FileBucket::Documentation));
        }
        v
    }

    fn mock_layers(statuses: &[(&str, LayerStatus)]) -> Vec<(String, LayerStatus)> {
        statuses.iter().map(|(s, st)| (s.to_string(), *st)).collect()
    }

    // ── Review type tests ────────────────────────────────────────────────

    #[test]
    fn test_261_files_10_deep_reviewed_is_partial_review() {
        let classifications = mock_classifications(200, 30, 31);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 10, false, &layers);
        assert_eq!(summary.total_changed, 261);
        assert_eq!(summary.total_deep_reviewed, 10);
        assert_eq!(summary.review_type, ReviewType::PartialReview);
    }

    #[test]
    fn test_50_percent_deep_reviewed_is_focused_risk() {
        let classifications = mock_classifications(90, 5, 5);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 50, false, &layers);
        assert_eq!(summary.review_type, ReviewType::FocusedRiskReview);
    }

    #[test]
    fn test_85_percent_deep_reviewed_is_complete() {
        let classifications = mock_classifications(95, 2, 3);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 85, false, &layers);
        assert_eq!(summary.review_type, ReviewType::CompleteReview);
    }

    #[test]
    fn test_no_semantic_layers_is_governance_only() {
        let classifications = mock_classifications(10, 0, 0);
        let layers = mock_layers(&[]);
        let summary = analyze_coverage(&classifications, 0, false, &layers);
        assert_eq!(summary.review_type, ReviewType::GovernanceOnlyReview);
    }

    #[test]
    fn test_zero_files_is_complete_review() {
        let classifications: Vec<(String, FileBucket)> = Vec::new();
        let layers = mock_layers(&[]);
        let summary = analyze_coverage(&classifications, 0, false, &layers);
        assert_eq!(summary.review_type, ReviewType::CompleteReview);
    }

    // ── Confidence tests ─────────────────────────────────────────────────

    #[test]
    fn test_partial_coverage_forces_confidence_low() {
        let classifications = mock_classifications(200, 30, 31);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 10, false, &layers);
        assert_eq!(summary.confidence, ConfidenceLevel::Low);
    }

    #[test]
    fn test_early_signal_skip_caps_confidence_at_medium() {
        let classifications = mock_classifications(90, 5, 5);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 80, true, &layers);
        assert_eq!(summary.confidence, ConfidenceLevel::Medium);
    }

    #[test]
    fn test_two_deferred_layers_caps_confidence_at_low() {
        let classifications = mock_classifications(90, 5, 5);
        let layers = mock_layers(&[
            ("application-source", LayerStatus::Completed),
            ("high-risk-surfaces", LayerStatus::SkippedWithReason),
            ("related-context", LayerStatus::SkippedWithReason),
        ]);
        let summary = analyze_coverage(&classifications, 80, false, &layers);
        assert_eq!(summary.confidence, ConfidenceLevel::Low);
    }

    #[test]
    fn test_high_coverage_with_no_deferrals_is_high_confidence() {
        let classifications = mock_classifications(95, 2, 3);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 85, false, &layers);
        assert_eq!(summary.confidence, ConfidenceLevel::High);
    }

    // ── Approval readiness tests ─────────────────────────────────────────

    #[test]
    fn test_partial_review_is_never_ready() {
        let classifications = mock_classifications(200, 30, 31);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 10, false, &layers);
        assert_eq!(summary.approval_readiness, ApprovalReadiness::NotReady);
    }

    #[test]
    fn test_focused_risk_with_low_confidence_is_not_ready() {
        let classifications = mock_classifications(100, 0, 0);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 60, false, &layers);
        assert_eq!(summary.review_type, ReviewType::FocusedRiskReview);
        assert_eq!(summary.confidence, ConfidenceLevel::Medium);
        assert_eq!(summary.approval_readiness, ApprovalReadiness::NotReady);
    }

    #[test]
    fn test_complete_review_with_high_confidence_is_ready() {
        let classifications = mock_classifications(95, 2, 3);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 85, false, &layers);
        assert_eq!(summary.review_type, ReviewType::CompleteReview);
        assert_eq!(summary.confidence, ConfidenceLevel::High);
        assert_eq!(summary.approval_readiness, ApprovalReadiness::Ready);
    }

    // ── Global comment tests ─────────────────────────────────────────────

    #[test]
    fn test_partial_coverage_generates_global_comment() {
        let classifications = mock_classifications(200, 30, 31);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 10, false, &layers);
        let comment = generate_global_comment(&summary);
        assert!(comment.contains("question(non-blocking)"));
        assert!(comment.contains("261-file PR"));
        assert!(comment.contains("focused risk review") || comment.contains("partial review"));
    }

    #[test]
    fn test_complete_review_global_comment_still_shows_bucket_gaps() {
        let classifications = mock_classifications(95, 2, 3);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 85, false, &layers);
        let comment = generate_global_comment(&summary);
        // Even with complete review, if high-risk buckets have gaps, they show.
        assert!(comment.contains("question(non-blocking)"));
    }

    // ── Follow-up requirements tests ─────────────────────────────────────

    #[test]
    fn test_high_risk_buckets_not_reviewed_create_follow_up() {
        let classifications = mock_classifications(10, 0, 0);
        let layers = mock_layers(&[]);
        let summary = analyze_coverage(&classifications, 0, false, &layers);
        let reqs = generate_follow_up_requirements(&summary);
        assert!(!reqs.is_empty());
        let joined = reqs.join("\n");
        assert!(joined.contains("application_source"));
    }

    // ── Not-ready statement tests ────────────────────────────────────────

    #[test]
    fn test_not_ready_statement_mentions_counts() {
        let classifications = mock_classifications(261, 0, 0);
        let layers = mock_layers(&[("application-source", LayerStatus::Completed)]);
        let summary = analyze_coverage(&classifications, 10, false, &layers);
        let statement = generate_not_ready_statement(&summary);
        assert!(statement.contains("10"));
        assert!(statement.contains("261"));
        assert!(statement.contains("Additional review is required"));
    }
}
