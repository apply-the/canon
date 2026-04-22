use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::artifact::ArtifactContract;
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};

/// Canonical identity for a Canon run.
///
/// `uuid` is the immutable machine identity (UUIDv7 today).
/// `run_id` is the human-facing display id `R-YYYYMMDD-SHORTID` derived
/// deterministically from `uuid` and the UTC date of `created_at`.
/// `short_id` is the first 8 hex characters of the lowercase canonical
/// UUID string. See `specs/009-run-id-display/contracts/run-identity-contract.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunIdentity {
    pub uuid: Uuid,
    pub run_id: String,
    pub short_id: String,
    pub created_at: OffsetDateTime,
}

impl RunIdentity {
    /// Generate a fresh identity using `Uuid::now_v7()` and `OffsetDateTime::now_utc()`.
    pub fn new_now_v7() -> Self {
        Self::from_parts(Uuid::now_v7(), OffsetDateTime::now_utc())
    }

    /// Build a [`RunIdentity`] from a known UUID and timestamp.
    /// Used by the manifest read shim to reconstruct identity for legacy runs.
    pub fn from_parts(uuid: Uuid, created_at: OffsetDateTime) -> Self {
        let short_id = short_id_from_uuid(&uuid);
        let date = created_at.to_offset(time::UtcOffset::UTC).date();
        let run_id = format!(
            "R-{:04}{:02}{:02}-{}",
            date.year(),
            u8::from(date.month()),
            date.day(),
            short_id
        );
        Self { uuid, run_id, short_id, created_at }
    }
}

/// First 8 hex characters of the canonical lowercase UUID string.
pub fn short_id_from_uuid(uuid: &Uuid) -> String {
    let buf = uuid.as_simple().to_string();
    buf[..8].to_string()
}

/// Validate the canonical display-id shape `^R-\d{8}-[0-9a-f]{8}$`.
pub fn is_canonical_display_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 19 {
        return false;
    }
    if &bytes[..2] != b"R-" || bytes[10] != b'-' {
        return false;
    }
    bytes[2..10].iter().all(|b| b.is_ascii_digit())
        && bytes[11..].iter().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

// Mode names only describe the governed work type; system state stays explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemContext {
    New,
    Existing,
}

impl SystemContext {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Existing => "existing",
        }
    }
}

impl std::str::FromStr for SystemContext {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "new" => Ok(Self::New),
            "existing" => Ok(Self::Existing),
            other => Err(format!("unsupported system context: {other}")),
        }
    }
}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    #[serde(default, skip)]
    pub inline_inputs: Vec<InlineInput>,
    pub captured_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineInput {
    pub label: String,
    pub contents: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputSourceKind {
    #[default]
    Path,
    Inline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputFingerprint {
    pub path: String,
    #[serde(default)]
    pub source_kind: InputSourceKind,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    pub state: RunState,
    pub created_at: OffsetDateTime,
    pub artifact_contract: Option<ArtifactContract>,
    pub evidence_bundle_ref: Option<String>,
    pub pending_invocation_ids: Vec<String>,
}
