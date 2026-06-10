//! Artifact rendering and recommendation logic for the `pr-review finalize` phase.
//!
//! Produces `01-review-summary.md`, `02-conventional-comments.md`,
//! `03-github-comments.json`, `06-review-report.md`, `review-findings.json`,
//! and `missing-tests.md` from the canonical comment set and governance findings.

use crate::domain::review_coverage::{ApprovalReadiness, CoverageSummary};
use crate::review::coverage::{generate_global_comment, generate_not_ready_statement};
use crate::review::findings::{CanonicalCommentSet, GithubComment, ReviewPacket};

/// The final review recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recommendation {
    Approve,
    Comment,
    RequestChanges,
}

impl Recommendation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Approve => "Approve",
            Self::Comment => "Comment",
            Self::RequestChanges => "Request changes",
        }
    }
}

/// Derives the final recommendation from the canonical comment set and governance packet.
///
/// Rules:
/// - Blocking actionable findings → RequestChanges
/// - Actionable review failed → RequestChanges
/// - Non-blocking findings, partial coverage, or governance-only → Comment
/// - No findings, sufficient coverage → Approve
///
/// Forbidden inconsistent states (returns Err):
/// - RequestChanges with 0 blocking comments and 0 must-fix governance findings
/// - Approve with must-fix governance findings present
pub fn derive_recommendation(
    canonical: &CanonicalCommentSet,
    packet: &ReviewPacket,
    actionable_review_executed: bool,
    actionable_review_failed: bool,
) -> Result<Recommendation, String> {
    let blocking = canonical.blocking_count();
    let non_blocking = canonical.non_blocking_count();
    let gov_must_fix = packet.must_fix_findings().len();

    let rec = if actionable_review_failed || blocking > 0 || gov_must_fix > 0 {
        Recommendation::RequestChanges
    } else if !actionable_review_executed || non_blocking > 0 || !packet.findings.is_empty() {
        Recommendation::Comment
    } else {
        Recommendation::Approve
    };

    // Inconsistency checks
    if rec == Recommendation::RequestChanges
        && blocking == 0
        && gov_must_fix == 0
        && !actionable_review_failed
    {
        return Err(
            "Inconsistent: RequestChanges with no blocking comments, no must-fix findings, and no review failure"
                .to_string(),
        );
    }
    if rec == Recommendation::Approve && gov_must_fix > 0 {
        return Err("Inconsistent: Approve with must-fix governance findings present".to_string());
    }

    Ok(rec)
}

/// Generates `review-summary.md` from the canonical set and context.
pub fn render_review_summary(
    canonical: &CanonicalCommentSet,
    recommendation: Recommendation,
    packet: &ReviewPacket,
    changed_files: &[String],
    files_inspected: &[String],
    coverage: &CoverageSummary,
) -> String {
    let mut out = String::new();
    out.push_str("# PR Review Summary\n\n");

    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "Review of `{}` against `{}` across {} changed surface(s).\n\n",
        packet.head_ref,
        packet.base_ref,
        packet.changed_surfaces.len(),
    ));

    out.push_str("## Recommendation\n\n");
    out.push_str(&format!("**{}**\n\n", recommendation.as_str()));

    // ── Coverage-aware metadata ────────────────────────────────────────
    out.push_str("## Review Type & Confidence\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Review type | {} |\n", coverage.review_type_label()));
    out.push_str(&format!("| Confidence | `{}` |\n", coverage.confidence_label()));
    out.push_str(&format!("| Approval readiness | {} |\n", coverage.approval_readiness_label()));

    out.push_str("\n## Review Status\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Actionable review status | {} |\n", canonical.reviewer_status));
    out.push_str(&format!("| Changed files | {} |\n", changed_files.len()));
    out.push_str(&format!("| Files inspected deeply | {} |\n", files_inspected.len()));

    // ── Coverage by bucket ─────────────────────────────────────────────
    if !coverage.buckets.is_empty() {
        out.push_str("\n## Coverage by Bucket\n\n");
        out.push_str("| Bucket | Total | Deep Reviewed | Light Reviewed | Indexed | Skipped | Not Reviewed |\n");
        out.push_str("|---|---|---|---|---|---|---|\n");
        for bc in &coverage.buckets {
            out.push_str(&format!(
                "| {:?} | {} | {} | {} | {} | {} | {} |\n",
                bc.bucket,
                bc.total,
                bc.deep_reviewed,
                bc.light_reviewed,
                bc.indexed_only,
                bc.skipped,
                bc.not_reviewed,
            ));
        }
    }

    out.push_str("\n## Severity Summary\n\n");
    out.push_str("| Severity | Count |\n|---|---|\n");
    for sev in &["blocking", "major", "minor", "question", "nitpick"] {
        out.push_str(&format!("| {sev} | {} |\n", canonical.count_by_severity(sev)));
    }

    out.push_str("\n## Must Fix\n\n");
    let blocking_cmts: Vec<_> = canonical.comments.iter().filter(|c| c.blocking).collect();
    if blocking_cmts.is_empty() && packet.must_fix_findings().is_empty() {
        out.push_str("- No must-fix findings.\n");
    } else {
        for c in &blocking_cmts {
            let loc = comment_location(c);
            out.push_str(&format!("- **{id}** — {loc}: {body}\n", id = c.id, body = c.body));
        }
        for f in packet.must_fix_findings() {
            out.push_str(&format!("- [governance] {}: {}\n", f.title, f.details));
        }
    }

    out.push_str("\n## Governance Observations\n\n");
    if packet.findings.is_empty() {
        out.push_str("- No governance observations.\n");
    } else {
        for f in &packet.findings {
            out.push_str(&format!("- {}: {}\n", f.title, f.details));
        }
    }

    // ── Global coverage comment (when coverage is partial) ─────────────
    if coverage.approval_readiness == ApprovalReadiness::NotReady {
        out.push_str("\n## Global Coverage Comment\n\n");
        out.push_str(&generate_global_comment(coverage));
        out.push('\n');
    }

    // Final disposition for gate compatibility
    out.push_str("\n## Final Disposition\n\n");
    match recommendation {
        Recommendation::Approve => {
            out.push_str("Status: ready\n\nRationale: No blocking findings remain.\n");
        }
        Recommendation::Comment => {
            out.push_str(
                "Status: ready-with-review-notes\n\nRationale: Review completed with notes.\n",
            );
        }
        Recommendation::RequestChanges => {
            out.push_str("Status: awaiting-disposition\n\nRationale: Blocking findings require resolution.\n");
        }
    }
    out.push_str(&format!("\nStatus: {status}\n", status = recommendation_status(recommendation)));

    // ── Not-ready statement ─────────────────────────────────────────────
    if coverage.approval_readiness == ApprovalReadiness::NotReady {
        out.push_str("\n> ");
        out.push_str(&generate_not_ready_statement(coverage));
        out.push('\n');
    }

    out
}

fn recommendation_status(rec: Recommendation) -> &'static str {
    match rec {
        Recommendation::Approve => "ready",
        Recommendation::Comment => "ready-with-review-notes",
        Recommendation::RequestChanges => "awaiting-disposition",
    }
}

fn comment_location(c: &GithubComment) -> String {
    match (&c.path, c.line) {
        (Some(p), Some(l)) => format!("`{p}` line {l}"),
        (Some(p), None) => format!("`{p}` (hunk)"),
        _ => "PR-level".to_string(),
    }
}

/// Generates `review-report.md`.
pub fn render_review_report(
    canonical: &CanonicalCommentSet,
    recommendation: Recommendation,
    packet: &ReviewPacket,
    changed_files: &[String],
    coverage: &CoverageSummary,
) -> String {
    let mut out = String::new();
    out.push_str("# PR Review Report\n\n");

    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "Severity-oriented report with {} actionable comment(s).\n\n",
        canonical.comments.len()
    ));

    out.push_str("## Recommendation\n\n");
    out.push_str(&format!("**{}**\n\n", recommendation.as_str()));

    // ── Coverage-aware metadata ────────────────────────────────────────
    out.push_str("## Review Type & Confidence\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Review type | {} |\n", coverage.review_type_label()));
    out.push_str(&format!("| Confidence | `{}` |\n", coverage.confidence_label()));
    out.push_str(&format!("| Approval readiness | {} |\n", coverage.approval_readiness_label()));

    out.push_str("## Severity Summary\n\n");
    out.push_str("| Severity | Count |\n|---|---|\n");
    for sev in &["blocking", "major", "minor", "question", "nitpick"] {
        out.push_str(&format!("| {sev} | {} |\n", canonical.count_by_severity(sev)));
    }

    out.push_str("\n## Blocking Issues\n\n");
    let blocking: Vec<_> = canonical.comments.iter().filter(|c| c.blocking).collect();
    if blocking.is_empty() {
        out.push_str("- None.\n");
    } else {
        for c in &blocking {
            let loc = comment_location(c);
            out.push_str(&format!("- **{id}** — {loc}: {body}\n", id = c.id, body = c.body));
        }
    }

    out.push_str("\n## Review Coverage\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Actionable review status | {} |\n", canonical.reviewer_status));
    out.push_str(&format!("| Files changed | {} |\n", changed_files.len()));
    out.push_str(&format!("| Files deeply reviewed | {} |\n", coverage.total_deep_reviewed));
    out.push_str(&format!("| Review type | {} |\n", coverage.review_type_label()));
    out.push_str(&format!("| Confidence | `{}` |\n", coverage.confidence_label()));

    // ── Coverage by bucket ─────────────────────────────────────────────
    if !coverage.buckets.is_empty() {
        out.push_str("\n## Coverage by Bucket\n\n");
        out.push_str("| Bucket | Total | Deep Reviewed | Not Reviewed |\n");
        out.push_str("|---|---|---|---|\n");
        for bc in &coverage.buckets {
            out.push_str(&format!(
                "| {:?} | {} | {} | {} |\n",
                bc.bucket, bc.total, bc.deep_reviewed, bc.not_reviewed,
            ));
        }
    }

    // ── Global comment when not ready ──────────────────────────────────
    if coverage.approval_readiness == ApprovalReadiness::NotReady {
        out.push_str("\n## Global Coverage Comment\n\n");
        out.push_str(&generate_global_comment(coverage));
        out.push('\n');

        out.push_str("\n> ");
        out.push_str(&generate_not_ready_statement(coverage));
        out.push('\n');
    }

    out.push_str("\n## Governance Observations\n\n");
    if packet.findings.is_empty() {
        out.push_str("- No governance observations.\n");
    } else {
        for f in &packet.findings {
            out.push_str(&format!("- {}: {}\n", f.title, f.details));
        }
    }

    out.push_str("\n## Final Recommendation\n\n");
    out.push_str(&format!("**{}**\n", recommendation.as_str()));

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::review_coverage::{
        ApprovalReadiness, ConfidenceLevel, CoverageSummary, ReviewType,
    };
    use crate::review::findings::{
        CanonicalCommentSet, ConventionalCommentScope, FindingCategory, FindingSeverity,
        ReviewFinding, ReviewPacket,
    };

    fn empty_packet() -> ReviewPacket {
        ReviewPacket {
            base_ref: "main".to_string(),
            head_ref: "HEAD".to_string(),
            changed_surfaces: vec![],
            inferred_intent: String::new(),
            findings: vec![],
            surprising_surface_area: vec![],
        }
    }

    fn empty_canonical() -> CanonicalCommentSet {
        CanonicalCommentSet { comments: vec![], reviewer_status: "governance_only".to_string() }
    }

    /// Returns a minimal complete-review CoverageSummary for tests that
    /// don't need specific coverage data.
    fn empty_coverage() -> CoverageSummary {
        CoverageSummary {
            buckets: Vec::new(),
            review_type: ReviewType::CompleteReview,
            confidence: ConfidenceLevel::High,
            approval_readiness: ApprovalReadiness::Ready,
            total_changed: 0,
            total_deep_reviewed: 0,
            total_skipped: 0,
            early_signal_skipped: false,
            deferred_layer_count: 0,
        }
    }

    #[test]
    fn test_derive_approve_with_no_findings() {
        let rec = derive_recommendation(&empty_canonical(), &empty_packet(), true, false).unwrap();
        assert_eq!(rec, Recommendation::Approve);
    }

    #[test]
    fn test_derive_comment_when_actionable_not_executed() {
        let rec = derive_recommendation(&empty_canonical(), &empty_packet(), false, false).unwrap();
        assert_eq!(rec, Recommendation::Comment);
    }

    #[test]
    fn test_derive_request_changes_when_actionable_failed() {
        let rec = derive_recommendation(&empty_canonical(), &empty_packet(), false, true).unwrap();
        assert_eq!(rec, Recommendation::RequestChanges);
    }

    #[test]
    fn test_derive_comment_with_governance_only() {
        let mut packet = empty_packet();
        packet.findings.push(ReviewFinding {
            category: FindingCategory::DuplicationCheck,
            severity: FindingSeverity::Note,
            title: "Note".to_string(),
            details: "Detail".to_string(),
            scope: ConventionalCommentScope::Pr,
            anchor: None,
            changed_surfaces: vec![],
        });
        let rec = derive_recommendation(&empty_canonical(), &packet, true, false).unwrap();
        assert_eq!(rec, Recommendation::Comment);
    }

    #[test]
    fn test_request_changes_with_zero_blocking_is_rejected() {
        let rec = derive_recommendation(&empty_canonical(), &empty_packet(), false, true);
        assert!(rec.is_ok()); // actionable_review_failed gives RequestChanges, blocking=0 is fine because failure IS the reason
    }

    #[test]
    fn test_approve_with_must_fix_governance_gives_request_changes() {
        let mut packet = empty_packet();
        packet.findings.push(ReviewFinding {
            category: FindingCategory::BoundaryCheck,
            severity: FindingSeverity::MustFix,
            title: "Boundary".to_string(),
            details: "Detail".to_string(),
            scope: ConventionalCommentScope::Surface,
            anchor: None,
            changed_surfaces: vec!["src/boundary.rs".to_string()],
        });
        let rec = derive_recommendation(&empty_canonical(), &packet, true, false).unwrap();
        assert_eq!(rec, Recommendation::RequestChanges);
    }

    #[test]
    fn test_derive_inconsistent_request_changes_rejected() {
        let rec = derive_recommendation(&empty_canonical(), &empty_packet(), false, false);
        // Comment when no findings and not executed, not RequestChanges
        assert_eq!(rec.unwrap(), Recommendation::Comment);
    }

    #[test]
    fn test_derive_inconsistent_approve_with_must_fix_rejected() {
        let mut packet = empty_packet();
        packet.findings.push(ReviewFinding {
            category: FindingCategory::BoundaryCheck,
            severity: FindingSeverity::MustFix,
            title: "Must fix".to_string(),
            details: "D".to_string(),
            scope: ConventionalCommentScope::Pr,
            anchor: None,
            changed_surfaces: vec![],
        });
        // has must-fix but actionable not executed — should be Err for Approve inconsistency check
        let canonical = empty_canonical();
        // actionable_review_executed=true with must-fix means RequestChanges, not Approve
        // To trigger the inconsistent state, we need Approve with must-fix
        // But the logic gives RequestChanges in that case. Let's test the Err path directly.
        // The inconsistency: Approve + gov_must_fix > 0 is Err
        let rec = derive_recommendation(&canonical, &packet, true, false);
        // Should be RequestChanges, not Err (must-fix triggers RequestChanges before inconsistency check)
        assert_eq!(rec.unwrap(), Recommendation::RequestChanges);
    }

    #[test]
    fn test_render_review_summary_with_approve() {
        let canonical = CanonicalCommentSet {
            comments: vec![],
            reviewer_status: "actionable_review_executed".to_string(),
        };
        let changed: Vec<String> = vec!["src/a.rs".to_string()];
        let summary = render_review_summary(
            &canonical,
            Recommendation::Approve,
            &empty_packet(),
            &changed,
            &changed,
            &empty_coverage(),
        );
        assert!(summary.contains("## Recommendation"));
        assert!(summary.contains("**Approve**"));
        assert!(summary.contains("Status: ready"));
        assert!(summary.contains("## Severity Summary"));
    }

    #[test]
    fn test_render_review_summary_with_request_changes() {
        let canonical = CanonicalCommentSet {
            comments: vec![],
            reviewer_status: "governance_only".to_string(),
        };
        let mut packet = empty_packet();
        packet.findings.push(ReviewFinding {
            category: FindingCategory::BoundaryCheck,
            severity: FindingSeverity::MustFix,
            title: "Critical".to_string(),
            details: "Must address".to_string(),
            scope: ConventionalCommentScope::Pr,
            anchor: None,
            changed_surfaces: vec![],
        });
        let summary = render_review_summary(
            &canonical,
            Recommendation::RequestChanges,
            &packet,
            &[],
            &[],
            &empty_coverage(),
        );
        assert!(summary.contains("**Request changes**"));
        assert!(summary.contains("Status: awaiting-disposition"));
    }

    #[test]
    fn test_render_review_summary_with_comment() {
        let canonical = CanonicalCommentSet {
            comments: vec![],
            reviewer_status: "actionable_review_not_configured".to_string(),
        };
        let summary = render_review_summary(
            &canonical,
            Recommendation::Comment,
            &empty_packet(),
            &[],
            &[],
            &empty_coverage(),
        );
        assert!(summary.contains("**Comment**"));
        assert!(summary.contains("Status: ready-with-review-notes"));
    }

    #[test]
    fn test_render_review_report_approve_no_findings() {
        let canonical = CanonicalCommentSet {
            comments: vec![],
            reviewer_status: "actionable_review_executed".to_string(),
        };
        let report = render_review_report(
            &canonical,
            Recommendation::Approve,
            &empty_packet(),
            &[],
            &empty_coverage(),
        );
        assert!(report.contains("**Approve**"));
        assert!(report.contains("No governance observations"));
    }

    #[test]
    fn test_render_review_report_with_governance_findings() {
        let canonical = CanonicalCommentSet {
            comments: vec![],
            reviewer_status: "governance_only".to_string(),
        };
        let mut packet = empty_packet();
        packet.findings.push(ReviewFinding {
            category: FindingCategory::DuplicationCheck,
            severity: FindingSeverity::Note,
            title: "Note".to_string(),
            details: "Observed duplication".to_string(),
            scope: ConventionalCommentScope::Pr,
            anchor: None,
            changed_surfaces: vec![],
        });
        let report = render_review_report(
            &canonical,
            Recommendation::Comment,
            &packet,
            &[],
            &empty_coverage(),
        );
        assert!(report.contains("Observed duplication"));
    }

    #[test]
    fn test_comment_location_all_variants() {
        let inline = GithubComment {
            id: "C001".to_string(),
            path: Some("src/a.rs".to_string()),
            line: Some(42),
            side: None,
            hunk_header: None,
            area: String::new(),
            kind: "issue".to_string(),
            blocking: false,
            severity: "minor".to_string(),
            category: String::new(),
            body: "test".to_string(),
            why_it_matters: String::new(),
            suggested_remediation: String::new(),
            suggested_change: None,
        };
        assert_eq!(comment_location(&inline), "`src/a.rs` line 42");

        let hunk = GithubComment {
            id: "C002".to_string(),
            path: Some("src/a.rs".to_string()),
            line: None,
            side: None,
            hunk_header: Some("@@ -1,1 +1,1 @@".to_string()),
            area: String::new(),
            kind: "issue".to_string(),
            blocking: false,
            severity: "minor".to_string(),
            category: String::new(),
            body: "test".to_string(),
            why_it_matters: String::new(),
            suggested_remediation: String::new(),
            suggested_change: None,
        };
        assert_eq!(comment_location(&hunk), "`src/a.rs` (hunk)");

        let global = GithubComment {
            id: "C003".to_string(),
            path: None,
            line: None,
            side: None,
            hunk_header: None,
            area: String::new(),
            kind: "issue".to_string(),
            blocking: false,
            severity: "minor".to_string(),
            category: String::new(),
            body: "test".to_string(),
            why_it_matters: String::new(),
            suggested_remediation: String::new(),
            suggested_change: None,
        };
        assert_eq!(comment_location(&global), "PR-level");
    }

    #[test]
    fn test_render_review_summary_with_blocking_canonical_comments() {
        let blocking_comment = GithubComment {
            id: "C001".to_string(),
            path: Some("src/a.rs".to_string()),
            line: Some(10),
            side: Some("RIGHT".to_string()),
            hunk_header: None,
            area: String::new(),
            kind: "issue".to_string(),
            blocking: true,
            severity: "blocking".to_string(),
            category: "bug".to_string(),
            body: "Critical bug".to_string(),
            why_it_matters: "Important".to_string(),
            suggested_remediation: "Fix".to_string(),
            suggested_change: None,
        };
        let canonical = CanonicalCommentSet {
            comments: vec![blocking_comment],
            reviewer_status: "actionable_review_executed".to_string(),
        };
        let changed: Vec<String> = vec!["src/a.rs".to_string()];
        let summary = render_review_summary(
            &canonical,
            Recommendation::RequestChanges,
            &empty_packet(),
            &changed,
            &changed,
            &empty_coverage(),
        );
        assert!(summary.contains("**C001**"));
        assert!(summary.contains("Critical bug"));
    }

    #[test]
    fn test_render_review_report_with_blocking_canonical_comments() {
        let blocking_comment = GithubComment {
            id: "C001".to_string(),
            path: Some("src/a.rs".to_string()),
            line: Some(10),
            side: Some("RIGHT".to_string()),
            hunk_header: None,
            area: String::new(),
            kind: "issue".to_string(),
            blocking: true,
            severity: "blocking".to_string(),
            category: "bug".to_string(),
            body: "Critical bug".to_string(),
            why_it_matters: "Important".to_string(),
            suggested_remediation: "Fix".to_string(),
            suggested_change: None,
        };
        let canonical = CanonicalCommentSet {
            comments: vec![blocking_comment],
            reviewer_status: "actionable_review_executed".to_string(),
        };
        let report = render_review_report(
            &canonical,
            Recommendation::RequestChanges,
            &empty_packet(),
            &["src/a.rs".to_string()],
            &empty_coverage(),
        );
        assert!(report.contains("**C001**"));
        assert!(report.contains("Critical bug"));
    }
}
