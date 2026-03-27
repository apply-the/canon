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
    pub gate: GateKind,
    pub by: String,
    pub decision: ApprovalDecision,
    pub rationale: String,
    pub recorded_at: OffsetDateTime,
}
