//! Publication-state helpers for Canon-owned publish-profile metadata.

use std::collections::BTreeSet;

use super::semantic::ReasoningPostureContractLine;
use super::*;

/// Contract version string for project-memory promotion lineage.
pub const PROJECT_MEMORY_CONTRACT_VERSION: &str = "v1";
/// Canonical producer identifier for Canon-owned managed blocks.
pub const CANON_PRODUCER: &str = "canon";
/// Marker family name for Canon-owned managed blocks in project-visible surfaces.
pub const PROJECT_MEMORY_MANAGED_BLOCK_MARKER: &str = "project-memory:managed";
/// Filename for the project-memory packet metadata sidecar.
pub const PROJECT_MEMORY_PACKET_METADATA_FILE_NAME: &str = "packet-metadata.json";
/// Contract version string for governed expertise input metadata.
pub const GOVERNED_EXPERTISE_INPUT_CONTRACT_VERSION: &str = "v1";
/// Required field names for V1 lineage metadata envelopes.
pub const REQUIRED_V1_LINEAGE_FIELDS: &[&str] = &[
    "contract_version",
    "producer",
    "source_ref",
    "source_artifacts",
    "promotion_state",
    "promoted_at",
    "content_digest",
];
/// Optional field names recognized in V1 lineage metadata envelopes.
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

/// Stable publication-state vocabulary for governed reasoning posture releases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReasoningPosturePublicationStatus {
    Active,
    Legacy,
}

impl ReasoningPosturePublicationStatus {
    /// Returns the stable serialized string for this publication status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Legacy => "legacy",
        }
    }
}

impl std::fmt::Display for ReasoningPosturePublicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One published reasoning-posture line within a release surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPosturePublicationLine {
    pub contract_line: ReasoningPostureContractLine,
    pub publication_status: ReasoningPosturePublicationStatus,
}

/// Typed helper for validating active-versus-legacy publication rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPosturePublicationSet {
    pub published_lines: Vec<ReasoningPosturePublicationLine>,
}

impl ReasoningPosturePublicationSet {
    /// Returns the active contract line when the set is well-formed.
    pub fn active_line(&self) -> Option<ReasoningPostureContractLine> {
        self.published_lines.iter().find_map(|line| {
            matches!(line.publication_status, ReasoningPosturePublicationStatus::Active)
                .then_some(line.contract_line)
        })
    }

    /// Validates the Canon-owned active-versus-legacy publication invariants.
    pub fn validate(&self) -> Result<(), String> {
        if self.published_lines.is_empty() {
            return Err("reasoning posture publication sets require at least one line".to_string());
        }
        if self.published_lines.len() > 2 {
            return Err("reasoning posture publication sets support at most two lines".to_string());
        }

        let mut unique_lines = BTreeSet::new();
        let mut active_count = 0_usize;
        let mut legacy_count = 0_usize;

        for line in &self.published_lines {
            if !unique_lines.insert(line.contract_line) {
                return Err(format!(
                    "reasoning posture publication sets cannot repeat contract line `{}`",
                    line.contract_line,
                ));
            }

            match line.publication_status {
                ReasoningPosturePublicationStatus::Active => active_count += 1,
                ReasoningPosturePublicationStatus::Legacy => legacy_count += 1,
            }
        }

        if active_count != 1 {
            return Err(
                "reasoning posture publication sets require exactly one active line".to_string()
            );
        }

        if self.published_lines.len() == 2 {
            if legacy_count != 1 {
                return Err(
                    "dual-line reasoning posture publication requires exactly one legacy line"
                        .to_string(),
                );
            }

            let has_v1_legacy = self.published_lines.iter().any(|line| {
                matches!(line.contract_line, ReasoningPostureContractLine::V1)
                    && matches!(line.publication_status, ReasoningPosturePublicationStatus::Legacy)
            });
            let has_v2_active = self.published_lines.iter().any(|line| {
                matches!(line.contract_line, ReasoningPostureContractLine::V2)
                    && matches!(line.publication_status, ReasoningPosturePublicationStatus::Active)
            });

            if !has_v1_legacy || !has_v2_active {
                return Err(
                    "dual-line reasoning posture publication requires v1 legacy and v2 active"
                        .to_string(),
                );
            }
        }

        Ok(())
    }
}

/// A publish profile determines how Canon routes governed output into
/// project-visible surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublishProfile {
    /// Route output through the project-memory promotion policy.
    ProjectMemory,
}

impl PublishProfile {
    /// Returns the kebab-case string representation of this publish profile.
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
    /// Returns the kebab-case string representation of this promotion state.
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
    /// Returns the kebab-case string representation of this update strategy.
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
    /// Identifier of the tool or agent that owns this managed block.
    pub producer: String,
    /// Reference to the Canon run or source artifact that produced the managed block.
    pub source_ref: String,
    /// Contract version string for this managed block format.
    pub contract_version: String,
}

impl ManagedBlockDescriptor {
    /// Constructs a Canon-owned managed block descriptor for the given source reference.
    pub fn canon(source_ref: impl Into<String>) -> Self {
        Self {
            producer: CANON_PRODUCER.to_string(),
            source_ref: source_ref.into(),
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.to_string(),
        }
    }

    /// Returns the HTML comment start marker for this managed block.
    pub fn start_marker(&self) -> String {
        format!(
            "<!-- {PROJECT_MEMORY_MANAGED_BLOCK_MARKER}:start producer=\"{}\" source_ref=\"{}\" contract_version=\"{}\" -->",
            self.producer, self.source_ref, self.contract_version
        )
    }

    /// Returns the static HTML comment end marker for Canon-managed blocks.
    pub fn end_marker() -> &'static str {
        "<!-- project-memory:managed:end -->"
    }
}

/// Lineage metadata emitted with every project-memory promoted output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageMetadata {
    /// V1 contract version for this lineage envelope.
    pub contract_version: String,
    /// Identifier of the producer that emitted this output.
    pub producer: String,
    #[serde(alias = "source_run")]
    /// Reference to the Canon run that produced this output.
    pub source_ref: String,
    /// Artifact filenames included in this promotion.
    pub source_artifacts: Vec<String>,
    /// The promotion state applied during this publication.
    pub promotion_state: PromotionState,
    #[serde(alias = "published_at")]
    /// ISO-8601 timestamp of when this output was promoted.
    pub promoted_at: String,
    /// SHA-256 or equivalent digest of the promoted content.
    pub content_digest: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Governed mode that produced this output, if recorded.
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Authoring stage within the mode, if applicable.
    pub stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Named human owner at promotion time, if any.
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Risk class of the run at promotion time, if recorded.
    pub risk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Usage zone of the run at promotion time, if recorded.
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    /// Approval state of the run at promotion time, if recorded.
    pub approval_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default, alias = "readiness")]
    /// Packet readiness label at promotion time, if recorded.
    pub packet_readiness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default, alias = "profile")]
    /// Publish profile used for this promotion, if recorded.
    pub promotion_profile: Option<PublishProfile>,
}

impl LineageMetadata {
    /// Returns the required V1 lineage field names.
    pub fn required_field_names() -> &'static [&'static str] {
        REQUIRED_V1_LINEAGE_FIELDS
    }

    /// Returns the optional V1 lineage field names.
    pub fn optional_field_names() -> &'static [&'static str] {
        OPTIONAL_V1_LINEAGE_FIELDS
    }
}

/// Per-mode promotion policy entry loaded from `publish-profiles.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModePromotionPolicy {
    /// The governed mode this policy entry applies to.
    pub mode: String,
    /// The default promotion state when no explicit override is present.
    pub default_promotion_state: PromotionState,
    /// The default update strategy when no explicit override is present.
    pub default_update_strategy: UpdateStrategy,
}

/// Top-level policy file shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishProfilesPolicy {
    /// Contract version string for this policy file.
    pub contract_version: String,
    /// The per-mode promotion policy entries.
    pub profiles: Vec<ModePromotionPolicy>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reasoning_posture_publication_status_strings_and_display_are_stable() {
        for (status, expected) in [
            (ReasoningPosturePublicationStatus::Active, "active"),
            (ReasoningPosturePublicationStatus::Legacy, "legacy"),
        ] {
            assert_eq!(status.as_str(), expected);
            assert_eq!(status.to_string(), expected);
            assert_eq!(serde_json::to_string(&status).unwrap(), format!("\"{expected}\""));
        }
    }

    #[test]
    fn reasoning_posture_publication_set_accepts_v2_active_v1_legacy() {
        let publication_set = ReasoningPosturePublicationSet {
            published_lines: vec![
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V1,
                    publication_status: ReasoningPosturePublicationStatus::Legacy,
                },
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Active,
                },
            ],
        };

        assert!(publication_set.validate().is_ok());
        assert_eq!(publication_set.active_line(), Some(ReasoningPostureContractLine::V2));
    }

    #[test]
    fn reasoning_posture_publication_set_rejects_dual_active_lines() {
        let publication_set = ReasoningPosturePublicationSet {
            published_lines: vec![
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V1,
                    publication_status: ReasoningPosturePublicationStatus::Active,
                },
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Active,
                },
            ],
        };

        let error = publication_set.validate().unwrap_err();
        assert!(error.contains("exactly one active line"));
    }

    #[test]
    fn reasoning_posture_publication_set_rejects_empty_duplicate_wrong_dual_line_and_too_many() {
        let empty = ReasoningPosturePublicationSet { published_lines: Vec::new() };
        let error = empty.validate().unwrap_err();
        assert!(error.contains("at least one line"));

        let duplicate = ReasoningPosturePublicationSet {
            published_lines: vec![
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Active,
                },
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Legacy,
                },
            ],
        };
        let error = duplicate.validate().unwrap_err();
        assert!(error.contains("cannot repeat contract line"));

        let wrong_dual_line = ReasoningPosturePublicationSet {
            published_lines: vec![
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V1,
                    publication_status: ReasoningPosturePublicationStatus::Active,
                },
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Legacy,
                },
            ],
        };
        let error = wrong_dual_line.validate().unwrap_err();
        assert!(error.contains("v1 legacy and v2 active"));

        let too_many = ReasoningPosturePublicationSet {
            published_lines: vec![
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V1,
                    publication_status: ReasoningPosturePublicationStatus::Legacy,
                },
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Active,
                },
                ReasoningPosturePublicationLine {
                    contract_line: ReasoningPostureContractLine::V2,
                    publication_status: ReasoningPosturePublicationStatus::Legacy,
                },
            ],
        };
        let error = too_many.validate().unwrap_err();
        assert!(error.contains("at most two lines"));
    }
}
