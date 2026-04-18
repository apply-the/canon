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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_digest_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClassificationSource {
    Explicit,
    InferredConfirmed,
    InferredOverridden,
}

impl ClassificationSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Explicit => "explicit",
            Self::InferredConfirmed => "inferred-confirmed",
            Self::InferredOverridden => "inferred-overridden",
        }
    }
}

impl std::str::FromStr for ClassificationSource {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "explicit" => Ok(Self::Explicit),
            "inferred-confirmed" => Ok(Self::InferredConfirmed),
            "inferred-overridden" => Ok(Self::InferredOverridden),
            other => Err(format!("unsupported classification source: {other}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationFieldProvenance {
    pub source: ClassificationSource,
    pub rationale: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signals: Vec<String>,
}

impl ClassificationFieldProvenance {
    pub fn new(
        source: ClassificationSource,
        rationale: impl Into<String>,
        signals: Vec<String>,
    ) -> Self {
        Self { source, rationale: rationale.into(), signals }
    }

    pub fn explicit_risk() -> Self {
        Self::new(
            ClassificationSource::Explicit,
            "Risk class was supplied explicitly at run start.",
            Vec::new(),
        )
    }

    pub fn explicit_zone() -> Self {
        Self::new(
            ClassificationSource::Explicit,
            "Usage zone was supplied explicitly at run start.",
            Vec::new(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationProvenance {
    pub risk: ClassificationFieldProvenance,
    pub zone: ClassificationFieldProvenance,
}

impl ClassificationProvenance {
    pub fn explicit() -> Self {
        Self {
            risk: ClassificationFieldProvenance::explicit_risk(),
            zone: ClassificationFieldProvenance::explicit_zone(),
        }
    }
}

impl Default for ClassificationProvenance {
    fn default() -> Self {
        Self::explicit()
    }
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
    pub evidence_bundle_ref: Option<String>,
    pub pending_invocation_ids: Vec<String>,
}
