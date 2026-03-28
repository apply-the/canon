use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::gate::GateKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approve,
    Reject,
}

impl ApprovalDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Reject => "reject",
        }
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
