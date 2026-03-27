use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::artifact::ArtifactContract;
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunState {
    Draft,
    ContextCaptured,
    Classified,
    Contracted,
    Gated,
    Executing,
    AwaitingApproval,
    Verifying,
    Completed,
    Blocked,
    Failed,
    Aborted,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContext {
    pub repo_root: String,
    pub owner: Option<String>,
    pub inputs: Vec<String>,
    pub excluded_paths: Vec<String>,
    pub input_fingerprints: Vec<InputFingerprint>,
    pub captured_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputFingerprint {
    pub path: String,
    pub size_bytes: u64,
    pub modified_unix_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Run {
    pub id: String,
    pub mode: Option<Mode>,
    pub risk: Option<RiskClass>,
    pub zone: Option<UsageZone>,
    pub state: RunState,
    pub created_at: OffsetDateTime,
    pub artifact_contract: Option<ArtifactContract>,
}
