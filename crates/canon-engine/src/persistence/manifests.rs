use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::RunState;
use crate::domain::run::{ClassificationProvenance, SystemContext};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkManifest {
    pub artifacts: Vec<String>,
    pub decisions: Vec<String>,
    pub traces: Vec<String>,
    pub invocations: Vec<String>,
    pub evidence: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunManifest {
    pub run_id: String,
    pub mode: Mode,
    pub risk: RiskClass,
    pub zone: UsageZone,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    #[serde(default)]
    pub classification: ClassificationProvenance,
    pub owner: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunStateManifest {
    pub state: RunState,
    pub updated_at: OffsetDateTime,
}
