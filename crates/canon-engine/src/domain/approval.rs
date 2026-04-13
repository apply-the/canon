use serde::{Deserialize, Serialize};
use strum_macros::{Display, IntoStaticStr};
use time::OffsetDateTime;

use crate::domain::gate::GateKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum ApprovalDecision {
    Approve,
    Reject,
}

impl ApprovalDecision {
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl std::str::FromStr for ApprovalDecision {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "approve" | "Approve" => Ok(Self::Approve),
            "reject" | "Reject" => Ok(Self::Reject),
            other => Err(format!("unsupported approval decision: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub gate: Option<GateKind>,
    pub invocation_request_id: Option<String>,
    pub by: String,
    pub decision: ApprovalDecision,
    pub rationale: String,
    pub recorded_at: OffsetDateTime,
}

impl ApprovalRecord {
    pub fn for_gate(
        gate: GateKind,
        by: String,
        decision: ApprovalDecision,
        rationale: String,
        recorded_at: OffsetDateTime,
    ) -> Self {
        Self { gate: Some(gate), invocation_request_id: None, by, decision, rationale, recorded_at }
    }

    pub fn for_invocation(
        request_id: String,
        by: String,
        decision: ApprovalDecision,
        rationale: String,
        recorded_at: OffsetDateTime,
    ) -> Self {
        Self {
            gate: None,
            invocation_request_id: Some(request_id),
            by,
            decision,
            rationale,
            recorded_at,
        }
    }

    pub fn matches_gate(&self, gate: GateKind) -> bool {
        self.gate == Some(gate)
    }

    pub fn matches_invocation(&self, request_id: &str) -> bool {
        self.invocation_request_id.as_deref() == Some(request_id)
    }

    pub fn is_approved(&self) -> bool {
        matches!(self.decision, ApprovalDecision::Approve)
    }

    pub fn target_label(&self) -> String {
        match (&self.gate, &self.invocation_request_id) {
            (Some(gate), _) => format!("gate:{}", gate.as_str()),
            (_, Some(request_id)) => format!("invocation:{request_id}"),
            _ => "unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use time::OffsetDateTime;

    use super::{ApprovalDecision, ApprovalRecord};
    use crate::domain::gate::GateKind;

    #[test]
    fn approval_decision_round_trips_supported_values() {
        assert_eq!(ApprovalDecision::Approve.as_str(), "approve");
        assert_eq!(ApprovalDecision::Reject.as_str(), "reject");
        assert_eq!(
            ApprovalDecision::from_str("approve").expect("approve should parse"),
            ApprovalDecision::Approve
        );
        assert_eq!(
            ApprovalDecision::from_str("Approve").expect("Approve should parse"),
            ApprovalDecision::Approve
        );
        assert_eq!(
            ApprovalDecision::from_str("reject").expect("reject should parse"),
            ApprovalDecision::Reject
        );
        assert_eq!(
            ApprovalDecision::from_str("Reject").expect("Reject should parse"),
            ApprovalDecision::Reject
        );
    }

    #[test]
    fn approval_decision_rejects_unknown_values() {
        let error = ApprovalDecision::from_str("defer").expect_err("unknown decision should fail");

        assert_eq!(error, "unsupported approval decision: defer");
    }

    #[test]
    fn approval_record_helpers_report_gate_and_invocation_targets() {
        let gate_record = ApprovalRecord::for_gate(
            GateKind::Risk,
            "Owner <owner@example.com>".to_string(),
            ApprovalDecision::Approve,
            "accepted".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        );
        assert!(gate_record.matches_gate(GateKind::Risk));
        assert!(!gate_record.matches_invocation("req-7"));
        assert!(gate_record.is_approved());
        assert_eq!(gate_record.target_label(), "gate:risk");

        let invocation_record = ApprovalRecord::for_invocation(
            "req-7".to_string(),
            "Owner <owner@example.com>".to_string(),
            ApprovalDecision::Reject,
            "rejected".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        );
        assert!(!invocation_record.matches_gate(GateKind::Risk));
        assert!(invocation_record.matches_invocation("req-7"));
        assert!(!invocation_record.is_approved());
        assert_eq!(invocation_record.target_label(), "invocation:req-7");
    }
}
