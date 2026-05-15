use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

use crate::review::findings::{FindingSeverity, ReviewPacket};

/// Overall readiness verdict derived from the set of [`ReviewFinding`](crate::review::findings::ReviewFinding)s.
///
/// The disposition is computed deterministically from the presence of
/// `MustFix` findings and the `approval_recorded` flag; callers must not
/// set it manually.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ReviewDisposition {
    /// No `MustFix` findings remain: the review is ready with informational notes.
    ReadyWithReviewNotes,
    /// `MustFix` findings exist and no explicit approval has been recorded.
    AwaitingDisposition,
    /// `MustFix` findings exist but an explicit reviewer approval was recorded.
    AcceptedWithApproval,
}

impl ReviewDisposition {
    /// Returns the kebab-case string representation of the disposition.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

/// The rendered verdict for a completed pr-review run.
///
/// `ReviewSummary` is produced from a [`ReviewPacket`] and used as the
/// primary input to the `review-summary.md` artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSummary {
    /// Computed readiness verdict for this review.
    pub disposition: ReviewDisposition,
    /// Human-readable explanation of how the disposition was reached.
    pub rationale: String,
    /// Titles of all `MustFix` findings that require explicit disposition.
    pub must_fix_findings: Vec<String>,
    /// Titles of all `Note` findings recorded as accepted risks.
    pub accepted_risks: Vec<String>,
}

impl ReviewSummary {
    /// Builds a [`ReviewSummary`] from diff inspection results only.
    ///
    /// Use [`from_evidence`](Self::from_evidence) when critique evidence from
    /// a governed verification run should also be reflected in the rationale.
    pub fn from_packet(packet: &ReviewPacket, approval_recorded: bool) -> Self {
        let must_fix_findings = packet
            .must_fix_findings()
            .into_iter()
            .map(|finding| finding.title.clone())
            .collect::<Vec<_>>();
        let accepted_risks = packet
            .note_findings()
            .into_iter()
            .map(|finding| finding.title.clone())
            .collect::<Vec<_>>();

        let (disposition, rationale) = if must_fix_findings.is_empty() {
            (
                ReviewDisposition::ReadyWithReviewNotes,
                "Ready with review notes because the changed surface stays bounded and no must-fix findings remain unresolved.".to_string(),
            )
        } else if approval_recorded {
            (
                ReviewDisposition::AcceptedWithApproval,
                "Explicit reviewer approval accepted the remaining must-fix findings with named ownership.".to_string(),
            )
        } else {
            (
                ReviewDisposition::AwaitingDisposition,
                "Must-fix findings require explicit disposition before readiness can pass."
                    .to_string(),
            )
        };

        Self { disposition, rationale, must_fix_findings, accepted_risks }
    }

    /// Builds a [`ReviewSummary`] and appends governed evidence context to the rationale.
    ///
    /// Delegates to [`from_packet`](Self::from_packet), then extends `rationale`
    /// to note that the review bundle includes linked critique evidence.
    pub fn from_evidence(packet: &ReviewPacket, approval_recorded: bool) -> Self {
        let mut summary = Self::from_packet(packet, approval_recorded);
        summary.rationale = format!(
            "{} Governed diff inspection and critique evidence remain linked from the review bundle.",
            summary.rationale
        );
        summary
    }
}

/// Returns `"must-fix"` if any finding in `packet` has `MustFix` severity,
/// otherwise `"review-notes"`.
pub fn summary_severity_label(packet: &ReviewPacket) -> &'static str {
    if packet.findings.iter().any(|finding| matches!(finding.severity, FindingSeverity::MustFix)) {
        "must-fix"
    } else {
        "review-notes"
    }
}

#[cfg(test)]
mod tests {
    use super::{ReviewDisposition, ReviewSummary, summary_severity_label};
    use crate::review::findings::ReviewPacket;

    #[test]
    fn review_summary_is_ready_when_packet_has_only_notes() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/lib.rs".to_string(), "tests/lib_test.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );

        let summary = ReviewSummary::from_packet(&packet, false);

        assert_eq!(summary.disposition, ReviewDisposition::ReadyWithReviewNotes);
        assert!(summary.must_fix_findings.is_empty());
        assert_eq!(summary.accepted_risks, vec!["No material duplication concerns inferred"]);
        assert_eq!(summary_severity_label(&packet), "review-notes");
    }

    #[test]
    fn review_summary_awaits_disposition_without_approval_for_must_fix_findings() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["contracts/schema.json".to_string(), "src/lib.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );

        let summary = ReviewSummary::from_packet(&packet, false);

        assert_eq!(summary.disposition, ReviewDisposition::AwaitingDisposition);
        assert!(summary.must_fix_findings.contains(&"Contract-facing files changed".to_string()));
        assert_eq!(summary_severity_label(&packet), "must-fix");
    }

    #[test]
    fn review_summary_accepts_must_fix_findings_when_approval_is_recorded() {
        let packet = ReviewPacket::from_diff(
            "origin/main",
            "HEAD",
            vec!["src/boundary/router.rs".to_string()],
            "@@ -1 +1 @@\n-old\n+new\n",
        );

        let summary = ReviewSummary::from_evidence(&packet, true);

        assert_eq!(summary.disposition, ReviewDisposition::AcceptedWithApproval);
        assert!(
            summary
                .rationale
                .contains("Governed diff inspection and critique evidence remain linked")
        );
        assert!(
            summary.must_fix_findings.contains(&"Boundary-marked surfaces changed".to_string())
        );
    }
}
