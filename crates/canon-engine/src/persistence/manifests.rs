use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::RunState;
use crate::domain::run::{
    ClassificationProvenance, RunIdentity, SystemContext, short_id_from_uuid,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkManifest {
    pub artifacts: Vec<String>,
    pub decisions: Vec<String>,
    pub traces: Vec<String>,
    pub invocations: Vec<String>,
    pub evidence: Option<String>,
}

/// Persisted run manifest.
///
/// `run_id` is the canonical filesystem key. For runs created after feature
/// 009-run-id-display lands, `run_id` is the human-facing display id
/// `R-YYYYMMDD-SHORTID`. For legacy runs created before this feature, the
/// field still holds a raw UUID string and the `uuid`, `short_id` fields
/// are absent on disk; they are reconstructed in-memory by [`RunManifest::canonicalize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunManifest {
    pub run_id: String,
    /// Canonical machine identity. `None` only on legacy on-disk manifests
    /// that pre-date feature 009; reconstruct via [`Self::canonicalize`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// First 8 hex characters of the lowercase canonical `uuid`. `None` only
    /// on legacy manifests; reconstruct via [`Self::canonicalize`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
    /// Optional descriptive slug. Metadata only; never identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Optional human-readable title. Metadata only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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

impl RunManifest {
    /// Construct a new manifest from a freshly minted [`RunIdentity`] plus
    /// run-request metadata. Produced manifests have no slug / title; later
    /// work may set them as descriptive metadata.
    pub fn from_identity(
        identity: &RunIdentity,
        mode: Mode,
        risk: RiskClass,
        zone: UsageZone,
        system_context: Option<SystemContext>,
        classification: ClassificationProvenance,
        owner: String,
    ) -> Self {
        Self {
            run_id: identity.run_id.clone(),
            uuid: Some(identity.uuid.as_simple().to_string()),
            short_id: Some(identity.short_id.clone()),
            slug: None,
            title: None,
            mode,
            risk,
            zone,
            system_context,
            classification,
            owner,
            created_at: identity.created_at,
        }
    }

    /// Reconstruct missing identity fields in-memory for legacy manifests.
    /// Called after deserialization. For modern manifests this is a no-op.
    pub fn canonicalize(mut self) -> Self {
        if self.uuid.is_none() {
            // Legacy: `run_id` is a raw UUID string.
            if let Ok(parsed) = self.run_id.parse::<uuid::Uuid>() {
                self.uuid = Some(parsed.as_simple().to_string());
                self.short_id = Some(short_id_from_uuid(&parsed));
            }
        }
        if self.short_id.is_none()
            && let Some(u) = self.uuid.as_deref().and_then(|s| s.parse::<uuid::Uuid>().ok())
        {
            self.short_id = Some(short_id_from_uuid(&u));
        }
        self
    }

    /// Return the canonical machine identity. Falls back to the legacy
    /// `run_id` value when `uuid` is absent on a not-yet-canonicalized
    /// manifest.
    pub fn uuid_or_legacy(&self) -> String {
        self.uuid.clone().unwrap_or_else(|| self.run_id.clone())
    }

    /// Reconstruct a [`RunIdentity`] from this (already-canonicalized)
    /// manifest. For legacy manifests where the manifest `run_id` is a UUID
    /// string, this preserves the legacy `run_id` so on-disk paths and links
    /// remain stable. Returns `None` if the uuid is missing or unparseable
    /// (which should not happen after `canonicalize`).
    pub fn to_identity(&self) -> Option<RunIdentity> {
        let uuid: uuid::Uuid = self.uuid.as_deref()?.parse().ok()?;
        let short_id = self.short_id.clone().unwrap_or_else(|| short_id_from_uuid(&uuid));
        Some(RunIdentity {
            uuid,
            run_id: self.run_id.clone(),
            short_id,
            created_at: self.created_at,
        })
    }

    /// True when this manifest was loaded from a legacy UUID-keyed run
    /// directory (the `run_id` is a raw UUID rather than `R-YYYYMMDD-...`).
    pub fn is_legacy_layout(&self) -> bool {
        !crate::domain::run::is_canonical_display_id(&self.run_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunStateManifest {
    pub state: RunState,
    pub updated_at: OffsetDateTime,
}
