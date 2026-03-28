use serde::{Deserialize, Serialize};

use crate::review::findings::{FindingSeverity, ReviewPacket};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDisposition {
    ReadyWithReviewNotes,
    AwaitingDisposition,
    AcceptedWithApproval,
}

impl ReviewDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReadyWithReviewNotes => "ready-with-review-notes",
            Self::AwaitingDisposition => "awaiting-disposition",
            Self::AcceptedWithApproval => "accepted-with-approval",
        }
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
