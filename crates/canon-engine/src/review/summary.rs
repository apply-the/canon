use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};

use crate::review::findings::{FindingSeverity, ReviewPacket};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ReviewDisposition {
    ReadyWithReviewNotes,
    AwaitingDisposition,
    AcceptedWithApproval,
}

impl ReviewDisposition {
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub disposition: ReviewDisposition,
    pub rationale: String,
    pub must_fix_findings: Vec<String>,
    pub accepted_risks: Vec<String>,
}

impl ReviewSummary {
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

    pub fn from_evidence(packet: &ReviewPacket, approval_recorded: bool) -> Self {
        let mut summary = Self::from_packet(packet, approval_recorded);
        summary.rationale = format!(
            "{} Governed diff inspection and critique evidence remain linked from the review bundle.",
            summary.rationale
        );
        summary
    }
}

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
