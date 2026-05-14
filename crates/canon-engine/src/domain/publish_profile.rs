use serde::{Deserialize, Serialize};

pub const PROJECT_MEMORY_CONTRACT_VERSION: &str = "v1";
pub const CANON_PRODUCER: &str = "canon";
pub const PROJECT_MEMORY_MANAGED_BLOCK_MARKER: &str = "project-memory:managed";
pub const REQUIRED_V1_LINEAGE_FIELDS: &[&str] = &[
    "contract_version",
    "producer",
    "source_ref",
    "source_artifacts",
    "promotion_state",
    "promoted_at",
    "content_digest",
];
pub const OPTIONAL_V1_LINEAGE_FIELDS: &[&str] = &[
    "mode",
    "stage",
    "owner",
    "risk",
    "zone",
    "approval_state",
    "packet_readiness",
    "promotion_profile",
];

/// A publish profile determines how Canon routes governed output into
/// project-visible surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublishProfile {
    /// Route output through the project-memory promotion policy.
    ProjectMemory,
}

impl PublishProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProjectMemory => "project-memory",
        }
    }
}

impl std::fmt::Display for PublishProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for PublishProfile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "project-memory" => Ok(Self::ProjectMemory),
            other => Err(format!("unknown publish profile: {other}")),
        }
    }
}

/// The Canon-owned promotion state that determines which publication
/// surfaces a governed packet may update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PromotionState {
    /// Output is promoted automatically to the stable surface.
    Auto,
    /// Output is promoted only when approval state and readiness meet policy.
    AutoIfApproved,
    /// Canon updates pending or audit surfaces only.
    PendingIndex,
    /// Canon records the event in index or audit surfaces without mutating
    /// stable targets.
    IndexOnly,
    /// Canon updates evidence-facing output without promoting to stable
    /// documents.
    EvidenceOnly,
    /// Stable publication requires an explicit manual action.
    Manual,
}

impl PromotionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::AutoIfApproved => "auto-if-approved",
            Self::PendingIndex => "pending-index",
            Self::IndexOnly => "index-only",
            Self::EvidenceOnly => "evidence-only",
            Self::Manual => "manual",
        }
    }

    /// Returns true when this state permits writing to stable project-memory
    /// surfaces.
    pub fn targets_stable_surface(self) -> bool {
        matches!(self, Self::Auto | Self::AutoIfApproved)
    }

    /// Returns true when this state routes to evidence-facing output only.
    pub fn targets_evidence_only(self) -> bool {
        matches!(self, Self::EvidenceOnly)
    }

    /// Returns true when this state routes to pending or audit surfaces.
    pub fn targets_pending_surface(self) -> bool {
        matches!(self, Self::PendingIndex | Self::IndexOnly)
    }
}

impl std::fmt::Display for PromotionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The Canon-owned strategy for modifying a project-visible document
/// without destructive overwrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateStrategy {
    /// Update only the Canon-managed range; preserve curated text outside.
    ManagedBlocks,
    /// Emit a proposal artifact rather than overwriting the stable target.
    ProposalFiles,
    /// Append entries to an index surface without rewriting existing entries.
    AppendOnlyIndex,
}

impl UpdateStrategy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManagedBlocks => "managed-blocks",
            Self::ProposalFiles => "proposal-files",
            Self::AppendOnlyIndex => "append-only-index",
        }
    }
}

impl std::fmt::Display for UpdateStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Descriptor for a producer-neutral managed block in repo-visible surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedBlockDescriptor {
    pub producer: String,
    pub source_ref: String,
    pub contract_version: String,
}

impl ManagedBlockDescriptor {
    pub fn canon(source_ref: impl Into<String>) -> Self {
        Self {
            producer: CANON_PRODUCER.to_string(),
            source_ref: source_ref.into(),
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.to_string(),
        }
    }

    pub fn start_marker(&self) -> String {
        format!(
            "<!-- {PROJECT_MEMORY_MANAGED_BLOCK_MARKER}:start producer=\"{}\" source_ref=\"{}\" contract_version=\"{}\" -->",
            self.producer, self.source_ref, self.contract_version
        )
    }

    pub fn end_marker() -> &'static str {
        "<!-- project-memory:managed:end -->"
    }
}

/// Lineage metadata emitted with every project-memory promoted output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageMetadata {
    pub contract_version: String,
    pub producer: String,
    #[serde(alias = "source_run")]
    pub source_ref: String,
    pub source_artifacts: Vec<String>,
    pub promotion_state: String,
    #[serde(alias = "published_at")]
    pub promoted_at: String,
    pub content_digest: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub risk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub approval_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default, alias = "readiness")]
    pub packet_readiness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default, alias = "profile")]
    pub promotion_profile: Option<String>,
}

impl LineageMetadata {
    pub fn required_field_names() -> &'static [&'static str] {
        REQUIRED_V1_LINEAGE_FIELDS
    }

    pub fn optional_field_names() -> &'static [&'static str] {
        OPTIONAL_V1_LINEAGE_FIELDS
    }
}

/// Per-mode promotion policy entry loaded from `publish-profiles.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModePromotionPolicy {
    pub mode: String,
    pub default_promotion_state: PromotionState,
    pub default_update_strategy: UpdateStrategy,
}

/// Top-level policy file shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishProfilesPolicy {
    pub contract_version: String,
    pub profiles: Vec<ModePromotionPolicy>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promotion_state_serde_round_trip() {
        for state in [
            PromotionState::Auto,
            PromotionState::AutoIfApproved,
            PromotionState::PendingIndex,
            PromotionState::IndexOnly,
            PromotionState::EvidenceOnly,
            PromotionState::Manual,
        ] {
            let json = serde_json::to_string(&state).unwrap();
            let back: PromotionState = serde_json::from_str(&json).unwrap();
            assert_eq!(state, back, "round-trip failed for {state:?}");
        }
    }

    #[test]
    fn promotion_state_kebab_case_serialization() {
        assert_eq!(
            serde_json::to_string(&PromotionState::AutoIfApproved).unwrap(),
            "\"auto-if-approved\""
        );
        assert_eq!(
            serde_json::to_string(&PromotionState::PendingIndex).unwrap(),
            "\"pending-index\""
        );
        assert_eq!(
            serde_json::to_string(&PromotionState::EvidenceOnly).unwrap(),
            "\"evidence-only\""
        );
    }

    #[test]
    fn update_strategy_serde_round_trip() {
        for strategy in [
            UpdateStrategy::ManagedBlocks,
            UpdateStrategy::ProposalFiles,
            UpdateStrategy::AppendOnlyIndex,
        ] {
            let json = serde_json::to_string(&strategy).unwrap();
            let back: UpdateStrategy = serde_json::from_str(&json).unwrap();
            assert_eq!(strategy, back);
        }
    }

    #[test]
    fn publish_profile_from_str() {
        assert_eq!(
            "project-memory".parse::<PublishProfile>().unwrap(),
            PublishProfile::ProjectMemory
        );
        assert!("unknown".parse::<PublishProfile>().is_err());
    }

    #[test]
    fn lineage_metadata_serde_round_trip() {
        let meta = LineageMetadata {
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.into(),
            producer: CANON_PRODUCER.into(),
            source_ref: "canon-run:run-abc".into(),
            source_artifacts: vec!["artifact-1.md".into()],
            promotion_state: "auto".into(),
            promoted_at: "2026-05-13T00:00:00Z".into(),
            content_digest: "sha256:abc123".into(),
            mode: Some("architecture".into()),
            stage: Some("architecture".into()),
            owner: Some("Owner <owner@example.com>".into()),
            risk: Some("bounded-impact".into()),
            zone: Some("yellow".into()),
            approval_state: Some("approved".into()),
            packet_readiness: Some("complete".into()),
            promotion_profile: Some("project-memory".into()),
        };
        let json = serde_json::to_string(&meta).unwrap();
        let back: LineageMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(meta, back);
    }

    #[test]
    fn lineage_metadata_deserializes_legacy_aliases() {
        let legacy_json = r#"{
            "contract_version": "0.1.0",
            "producer": "canon",
            "source_run": "run-abc",
            "source_artifacts": ["artifact-1.md"],
            "promotion_state": "auto-if-approved",
            "published_at": "2026-05-13T00:00:00Z",
            "content_digest": "sha256:legacy",
            "mode": "architecture",
            "approval_state": "Completed",
            "readiness": "complete",
            "profile": "project-memory"
        }"#;

        let back: LineageMetadata = serde_json::from_str(legacy_json).unwrap();
        assert_eq!(back.source_ref, "run-abc");
        assert_eq!(back.promoted_at, "2026-05-13T00:00:00Z");
        assert_eq!(back.packet_readiness.as_deref(), Some("complete"));
        assert_eq!(back.promotion_profile.as_deref(), Some("project-memory"));
    }

    #[test]
    fn managed_block_descriptor_renders_v1_marker() {
        let descriptor = ManagedBlockDescriptor::canon("canon-run:R-test");
        assert_eq!(
            descriptor.start_marker(),
            "<!-- project-memory:managed:start producer=\"canon\" source_ref=\"canon-run:R-test\" contract_version=\"v1\" -->"
        );
        assert_eq!(ManagedBlockDescriptor::end_marker(), "<!-- project-memory:managed:end -->");
    }

    #[test]
    fn lineage_field_lists_match_v1_contract() {
        assert_eq!(LineageMetadata::required_field_names(), REQUIRED_V1_LINEAGE_FIELDS);
        assert_eq!(LineageMetadata::optional_field_names(), OPTIONAL_V1_LINEAGE_FIELDS);
    }

    #[test]
    fn promotion_state_surface_targeting() {
        assert!(PromotionState::Auto.targets_stable_surface());
        assert!(PromotionState::AutoIfApproved.targets_stable_surface());
        assert!(!PromotionState::PendingIndex.targets_stable_surface());
        assert!(!PromotionState::IndexOnly.targets_stable_surface());
        assert!(!PromotionState::EvidenceOnly.targets_stable_surface());
        assert!(!PromotionState::Manual.targets_stable_surface());

        assert!(PromotionState::EvidenceOnly.targets_evidence_only());
        assert!(!PromotionState::Auto.targets_evidence_only());

        assert!(PromotionState::PendingIndex.targets_pending_surface());
        assert!(PromotionState::IndexOnly.targets_pending_surface());
        assert!(!PromotionState::Auto.targets_pending_surface());
    }

    #[test]
    fn publish_profiles_policy_serde() {
        let policy = PublishProfilesPolicy {
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.into(),
            profiles: vec![ModePromotionPolicy {
                mode: "architecture".into(),
                default_promotion_state: PromotionState::AutoIfApproved,
                default_update_strategy: UpdateStrategy::ManagedBlocks,
            }],
        };
        let json = serde_json::to_string(&policy).unwrap();
        let back: PublishProfilesPolicy = serde_json::from_str(&json).unwrap();
        assert_eq!(policy, back);
    }

    #[test]
    fn publish_profile_display() {
        assert_eq!(PublishProfile::ProjectMemory.to_string(), "project-memory");
    }

    #[test]
    fn promotion_state_as_str_and_display() {
        let cases = [
            (PromotionState::Auto, "auto"),
            (PromotionState::AutoIfApproved, "auto-if-approved"),
            (PromotionState::PendingIndex, "pending-index"),
            (PromotionState::IndexOnly, "index-only"),
            (PromotionState::EvidenceOnly, "evidence-only"),
            (PromotionState::Manual, "manual"),
        ];
        for (state, expected) in cases {
            assert_eq!(state.as_str(), expected);
            assert_eq!(state.to_string(), expected);
        }
    }

    #[test]
    fn update_strategy_as_str_and_display() {
        let cases = [
            (UpdateStrategy::ManagedBlocks, "managed-blocks"),
            (UpdateStrategy::ProposalFiles, "proposal-files"),
            (UpdateStrategy::AppendOnlyIndex, "append-only-index"),
        ];
        for (strategy, expected) in cases {
            assert_eq!(strategy.as_str(), expected);
            assert_eq!(strategy.to_string(), expected);
        }
    }
}
