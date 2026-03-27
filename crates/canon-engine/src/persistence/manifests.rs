use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::RunState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkManifest {
    pub artifacts: Vec<String>,
    pub decisions: Vec<String>,
    pub traces: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunManifest {
    pub run_id: String,
    pub mode: Mode,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub owner: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunStateManifest {
    pub state: RunState,
    pub updated_at: OffsetDateTime,
}
