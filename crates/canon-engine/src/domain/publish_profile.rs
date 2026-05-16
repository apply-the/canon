use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::domain::mode::{GovernedExpertiseKind, IntendedPersona, Mode, StageRoleHint};
use crate::domain::policy::{AuthorityZone, ChangeClass, RiskClass, UsageZone};

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
/// Contract line string for the first authority-governance metadata slice.
pub const AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE: &str = "authority-governance-v1";
/// Contract line string for the first adaptive-governance companion slice.
pub const ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE: &str = "adaptive-governance-v1";
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

/// Approval state exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityApprovalState {
    NotNeeded,
    Requested,
    Granted,
    Rejected,
    Expired,
}

impl AuthorityApprovalState {
    /// Returns true when the authority posture still requires a human gate.
    pub const fn requires_gate(self) -> bool {
        matches!(self, Self::Requested)
    }
}

/// Packet readiness exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityPacketReadiness {
    Pending,
    Incomplete,
    Reusable,
    Rejected,
}

/// Risk vocabulary exported through `authority-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthorityRiskClass {
    LowImpact,
    BoundedImpact,
    SystemicImpact,
}

impl AuthorityRiskClass {
    /// Maps the runtime risk class into the stable contract vocabulary.
    pub const fn from_risk_class(risk: RiskClass) -> Self {
        match risk {
            RiskClass::LowImpact => Self::LowImpact,
            RiskClass::BoundedImpact => Self::BoundedImpact,
            RiskClass::SystemicImpact => Self::SystemicImpact,
        }
    }
}

/// Governance-state vocabulary exported through `adaptive-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdaptiveGovernanceState {
    Advisory,
    Catch,
    Rule,
    Hook,
}

impl AdaptiveGovernanceState {
    /// Returns the kebab-case string representation of this governance state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Catch => "catch",
            Self::Rule => "rule",
            Self::Hook => "hook",
        }
    }
}

impl std::fmt::Display for AdaptiveGovernanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Rollout-profile vocabulary exported through `adaptive-governance-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdaptiveRolloutProfile {
    Minimal,
    Guided,
    Governed,
    Strict,
}

impl AdaptiveRolloutProfile {
    /// Returns the kebab-case string representation of this rollout profile.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Guided => "guided",
            Self::Governed => "governed",
            Self::Strict => "strict",
        }
    }
}

impl std::fmt::Display for AdaptiveRolloutProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Typed `adaptive-governance-v1` envelope published with governed packet metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdaptiveGovernanceV1Envelope {
    pub contract_line: String,
    pub governance_state: AdaptiveGovernanceState,
    pub rollout_profile: AdaptiveRolloutProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_rationale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_rationale: Option<String>,
}

/// Typed runtime inputs used to build an `adaptive-governance-v1` envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdaptiveGovernanceV1RuntimeInputs {
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approval_state: AuthorityApprovalState,
    pub packet_readiness: AuthorityPacketReadiness,
}

impl AdaptiveGovernanceV1Envelope {
    /// Builds the first-slice adaptive-governance companion from existing Canon runtime inputs.
    pub fn from_runtime_inputs(inputs: AdaptiveGovernanceV1RuntimeInputs) -> Self {
        let AdaptiveGovernanceV1RuntimeInputs { risk, zone, approval_state, packet_readiness } =
            inputs;

        let governance_state = if matches!(zone, UsageZone::Red) {
            AdaptiveGovernanceState::Hook
        } else if approval_state.requires_gate() {
            AdaptiveGovernanceState::Rule
        } else if matches!(
            packet_readiness,
            AuthorityPacketReadiness::Pending
                | AuthorityPacketReadiness::Incomplete
                | AuthorityPacketReadiness::Rejected
        ) {
            AdaptiveGovernanceState::Catch
        } else {
            AdaptiveGovernanceState::Advisory
        };

        let rollout_profile = if matches!(zone, UsageZone::Red) {
            AdaptiveRolloutProfile::Strict
        } else if approval_state.requires_gate() || matches!(risk, RiskClass::SystemicImpact) {
            AdaptiveRolloutProfile::Governed
        } else if matches!(risk, RiskClass::BoundedImpact | RiskClass::LowImpact)
            && matches!(zone, UsageZone::Yellow)
            || matches!(risk, RiskClass::BoundedImpact)
        {
            AdaptiveRolloutProfile::Guided
        } else {
            AdaptiveRolloutProfile::Minimal
        };

        Self {
            contract_line: ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE.to_string(),
            governance_state,
            rollout_profile,
            state_rationale: None,
            profile_rationale: None,
        }
    }
}

/// Typed `authority-governance-v1` envelope published with governed packet metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityGovernanceV1Envelope {
    pub contract_line: String,
    pub authority_zone: AuthorityZone,
    pub change_class: ChangeClass,
    pub intended_persona: IntendedPersona,
    pub approval_state: AuthorityApprovalState,
    pub packet_readiness: AuthorityPacketReadiness,
    pub risk: AuthorityRiskClass,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub persona_anti_behaviors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_artifact: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_order: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub promotion_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage_role_hints: Vec<StageRoleHint>,
}

/// Typed runtime inputs used to build an `authority-governance-v1` envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityGovernanceV1RuntimeInputs {
    pub mode: Mode,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub approval_state: AuthorityApprovalState,
    pub packet_readiness: AuthorityPacketReadiness,
    pub primary_artifact: Option<String>,
    pub artifact_order: Vec<String>,
    pub promotion_refs: Vec<String>,
}

impl AuthorityGovernanceV1Envelope {
    /// Builds the first-slice authority-governance envelope from existing Canon runtime inputs.
    pub fn from_runtime_inputs(inputs: AuthorityGovernanceV1RuntimeInputs) -> Self {
        let AuthorityGovernanceV1RuntimeInputs {
            mode,
            risk,
            zone,
            approval_state,
            packet_readiness,
            primary_artifact,
            artifact_order,
            promotion_refs,
        } = inputs;
        let persona = mode.intended_persona_profile();
        let authority_zone = if approval_state.requires_gate() {
            AuthorityZone::Restricted
        } else {
            match zone {
                UsageZone::Green => AuthorityZone::Green,
                UsageZone::Yellow => AuthorityZone::Yellow,
                UsageZone::Red => AuthorityZone::Red,
            }
        };

        Self {
            contract_line: AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE.to_string(),
            authority_zone,
            change_class: ChangeClass::from_risk_and_mode(risk, mode),
            intended_persona: persona.intended_persona,
            approval_state,
            packet_readiness,
            risk: AuthorityRiskClass::from_risk_class(risk),
            persona_anti_behaviors: persona.persona_anti_behaviors,
            primary_artifact,
            artifact_order,
            promotion_refs,
            stage_role_hints: mode.stage_role_hints(),
        }
    }
}

/// Canon-owned metadata carrier families for indexable artifact classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactMetadataCarrier {
    /// Metadata is discovered from the managed block marker family and the
    /// adjacent surface metadata sidecar for the same repo-visible document.
    ManagedSurfaceEnvelope,
    /// Metadata is discovered from a `packet-metadata.json` or
    /// `<surface>.packet-metadata.json` sidecar adjacent to the published
    /// artifact or surface.
    PacketMetadataSidecar,
}

impl ArtifactMetadataCarrier {
    /// Returns the kebab-case string representation of this carrier.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManagedSurfaceEnvelope => "managed-surface-envelope",
            Self::PacketMetadataSidecar => "packet-metadata-sidecar",
        }
    }

    /// Returns the consumer-facing discovery rule for reading this carrier's metadata.
    pub fn discovery_rule(self) -> &'static str {
        match self {
            Self::ManagedSurfaceEnvelope => {
                "Read the project-memory managed-block start marker for producer attribution and use the adjacent <surface>.packet-metadata.json sidecar for the full promoted lineage envelope."
            }
            Self::PacketMetadataSidecar => {
                "Read packet-metadata.json for packet roots or <surface>.packet-metadata.json adjacent to the published surface to discover the canonical indexing metadata."
            }
        }
    }
}

impl std::fmt::Display for ArtifactMetadataCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Supported V1 artifact classes that downstream consumers may index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IndexableArtifactClass {
    /// Stable or pending repo-visible project-memory documents updated through
    /// Canon-managed blocks.
    ManagedSurface,
    /// Proposal files emitted instead of mutating a stable project-memory
    /// target.
    ProposalArtifact,
    /// Evidence bundles emitted under `docs/evidence/` or another readable
    /// evidence-facing destination.
    EvidenceBundle,
    /// Append-only index or summary surfaces used for visibility without stable
    /// overwrite.
    IndexSurface,
}

impl IndexableArtifactClass {
    /// Returns the kebab-case string representation of this class.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManagedSurface => "managed-surface",
            Self::ProposalArtifact => "proposal-artifact",
            Self::EvidenceBundle => "evidence-bundle",
            Self::IndexSurface => "index-surface",
        }
    }

    /// Returns the metadata carrier family for this indexable artifact class.
    pub fn metadata_carrier(self) -> ArtifactMetadataCarrier {
        match self {
            Self::ManagedSurface => ArtifactMetadataCarrier::ManagedSurfaceEnvelope,
            Self::ProposalArtifact | Self::EvidenceBundle | Self::IndexSurface => {
                ArtifactMetadataCarrier::PacketMetadataSidecar
            }
        }
    }

    /// Returns the consumer-facing discovery rule for reading this class.
    pub fn discovery_rule(self) -> &'static str {
        self.metadata_carrier().discovery_rule()
    }

    /// Returns a slice containing all known `IndexableArtifactClass` variants.
    pub fn all() -> &'static [IndexableArtifactClass] {
        &[Self::ManagedSurface, Self::ProposalArtifact, Self::EvidenceBundle, Self::IndexSurface]
    }
}

impl std::fmt::Display for IndexableArtifactClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The publication target class used for routing packet output to the appropriate surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationTargetClass {
    /// Stable project-memory surface.
    Stable,
    /// Pending or draft surface.
    Pending,
    /// Proposal artifact destination.
    Proposal,
    /// Evidence-facing surface.
    Evidence,
    /// Append-only index surface.
    Index,
}

impl PublicationTargetClass {
    /// Returns the kebab-case string representation of this target class.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Pending => "pending",
            Self::Proposal => "proposal",
            Self::Evidence => "evidence",
            Self::Index => "index",
        }
    }

    /// Derives the publication target class from a promotion state and update strategy.
    pub fn for_publication(promotion: PromotionState, strategy: UpdateStrategy) -> Self {
        match strategy {
            UpdateStrategy::ProposalFiles => Self::Proposal,
            UpdateStrategy::ManagedBlocks if promotion.targets_stable_surface() => Self::Stable,
            UpdateStrategy::ManagedBlocks => Self::Pending,
            UpdateStrategy::AppendOnlyIndex if promotion.targets_evidence_only() => Self::Evidence,
            UpdateStrategy::AppendOnlyIndex if promotion.targets_pending_surface() => Self::Index,
            UpdateStrategy::AppendOnlyIndex => Self::Pending,
        }
    }
}

impl std::fmt::Display for PublicationTargetClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Governed expertise input metadata captured alongside published artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpertiseInputMetadata {
    /// The expertise kind associated with the governed mode.
    pub expertise_kind: GovernedExpertiseKind,
    /// The domain families provided by the expertise input.
    pub domain_families: Vec<String>,
}

impl ExpertiseInputMetadata {
    /// Constructs a new metadata record from the given expertise kind and domain families.
    ///
    /// Returns `None` if `domain_families` is empty after normalization.
    pub fn new(
        expertise_kind: GovernedExpertiseKind,
        domain_families: Vec<String>,
    ) -> Option<Self> {
        let domain_families = normalize_domain_families(domain_families);
        if domain_families.is_empty() {
            None
        } else {
            Some(Self { expertise_kind, domain_families })
        }
    }

    /// Returns a normalized copy of this metadata, or `None` if the domain families are empty.
    pub fn normalized(&self) -> Option<Self> {
        Self::new(self.expertise_kind, self.domain_families.clone())
    }
}

/// Normalizes domain family strings: trims whitespace and deduplicates.
pub fn normalize_domain_families(domain_families: Vec<String>) -> Vec<String> {
    let mut unique = BTreeSet::new();
    for family in domain_families {
        let trimmed = family.trim();
        if !trimmed.is_empty() {
            unique.insert(trimmed.to_string());
        }
    }
    unique.into_iter().collect()
}

/// Classifies a governed expertise input from the run mode and provided domain families.
pub fn classify_governed_expertise_input(
    mode: Mode,
    domain_families: Vec<String>,
) -> Option<ExpertiseInputMetadata> {
    ExpertiseInputMetadata::new(mode.governed_expertise_kind()?, domain_families)
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

#[cfg(test)]
mod authority_governance_tests {
    use super::{
        ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE, AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE,
        AdaptiveGovernanceState, AdaptiveGovernanceV1Envelope, AdaptiveGovernanceV1RuntimeInputs,
        AdaptiveRolloutProfile, AuthorityApprovalState, AuthorityGovernanceV1Envelope,
        AuthorityGovernanceV1RuntimeInputs, AuthorityPacketReadiness, AuthorityRiskClass,
    };
    use crate::domain::mode::{IntendedPersona, Mode};
    use crate::domain::policy::{AuthorityZone, ChangeClass, RiskClass, UsageZone};

    #[test]
    fn authority_governance_envelope_derives_required_fields() {
        let envelope = AuthorityGovernanceV1Envelope::from_runtime_inputs(
            AuthorityGovernanceV1RuntimeInputs {
                mode: Mode::Architecture,
                risk: RiskClass::SystemicImpact,
                zone: UsageZone::Yellow,
                approval_state: AuthorityApprovalState::Requested,
                packet_readiness: AuthorityPacketReadiness::Incomplete,
                primary_artifact: Some("01-architecture-summary.md".to_string()),
                artifact_order: vec!["01-architecture-summary.md".to_string()],
                promotion_refs: Vec::new(),
            },
        );

        assert_eq!(envelope.contract_line, AUTHORITY_GOVERNANCE_V1_CONTRACT_LINE);
        assert_eq!(envelope.authority_zone, AuthorityZone::Restricted);
        assert_eq!(envelope.change_class, ChangeClass::SystemicImpact);
        assert_eq!(envelope.intended_persona, IntendedPersona::SystemArchitect);
        assert_eq!(envelope.packet_readiness, AuthorityPacketReadiness::Incomplete);
        assert_eq!(envelope.risk, AuthorityRiskClass::SystemicImpact);
        assert!(!envelope.persona_anti_behaviors.is_empty());
    }

    #[test]
    fn authority_governance_envelope_omits_optional_fields_when_empty() {
        let envelope = AuthorityGovernanceV1Envelope::from_runtime_inputs(
            AuthorityGovernanceV1RuntimeInputs {
                mode: Mode::Requirements,
                risk: RiskClass::LowImpact,
                zone: UsageZone::Green,
                approval_state: AuthorityApprovalState::NotNeeded,
                packet_readiness: AuthorityPacketReadiness::Reusable,
                primary_artifact: None,
                artifact_order: Vec::new(),
                promotion_refs: Vec::new(),
            },
        );

        let value = serde_json::to_value(&envelope).unwrap();
        assert_eq!(value["authority_zone"], "green");
        assert_eq!(value["change_class"], "low-impact");
        assert_eq!(value["risk"], "low-impact");
        assert!(value.get("primary_artifact").is_none());
        assert!(value.get("promotion_refs").is_none());
    }

    #[test]
    fn adaptive_governance_envelope_derives_guided_advisory_companion() {
        let envelope =
            AdaptiveGovernanceV1Envelope::from_runtime_inputs(AdaptiveGovernanceV1RuntimeInputs {
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                approval_state: AuthorityApprovalState::NotNeeded,
                packet_readiness: AuthorityPacketReadiness::Reusable,
            });

        assert_eq!(envelope.contract_line, ADAPTIVE_GOVERNANCE_V1_CONTRACT_LINE);
        assert_eq!(envelope.governance_state, AdaptiveGovernanceState::Advisory);
        assert_eq!(envelope.rollout_profile, AdaptiveRolloutProfile::Guided);
        assert!(envelope.state_rationale.is_none());
        assert!(envelope.profile_rationale.is_none());
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
    fn publish_profile_serde_round_trip() {
        let json = serde_json::to_string(&PublishProfile::ProjectMemory).unwrap();
        assert_eq!(json, "\"project-memory\"");
        let back: PublishProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(back, PublishProfile::ProjectMemory);
    }

    #[test]
    fn indexable_artifact_classes_have_stable_carriers_and_rules() {
        let managed_surface = IndexableArtifactClass::ManagedSurface;
        assert_eq!(managed_surface.as_str(), "managed-surface");
        assert_eq!(
            managed_surface.metadata_carrier(),
            ArtifactMetadataCarrier::ManagedSurfaceEnvelope
        );
        assert!(managed_surface.discovery_rule().contains("managed-block"));

        let packet_backed = [
            IndexableArtifactClass::ProposalArtifact,
            IndexableArtifactClass::EvidenceBundle,
            IndexableArtifactClass::IndexSurface,
        ];

        for artifact_class in packet_backed {
            assert_eq!(
                artifact_class.metadata_carrier(),
                ArtifactMetadataCarrier::PacketMetadataSidecar
            );
            assert!(
                artifact_class.discovery_rule().contains(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME)
            );
        }
    }

    #[test]
    fn indexable_artifact_class_inventory_is_explicit() {
        let actual = IndexableArtifactClass::all()
            .iter()
            .map(|artifact_class| artifact_class.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec!["managed-surface", "proposal-artifact", "evidence-bundle", "index-surface",]
        );
    }

    #[test]
    fn lineage_metadata_serde_round_trip() {
        let meta = LineageMetadata {
            contract_version: PROJECT_MEMORY_CONTRACT_VERSION.into(),
            producer: CANON_PRODUCER.into(),
            source_ref: "canon-run:run-abc".into(),
            source_artifacts: vec!["artifact-1.md".into()],
            promotion_state: PromotionState::Auto,
            promoted_at: "2026-05-13T00:00:00Z".into(),
            content_digest: "sha256:abc123".into(),
            mode: Some("architecture".into()),
            stage: Some("architecture".into()),
            owner: Some("Owner <owner@example.com>".into()),
            risk: Some("bounded-impact".into()),
            zone: Some("yellow".into()),
            approval_state: Some("approved".into()),
            packet_readiness: Some("complete".into()),
            promotion_profile: Some(PublishProfile::ProjectMemory),
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
        assert_eq!(back.promotion_state, PromotionState::AutoIfApproved);
        assert_eq!(back.packet_readiness.as_deref(), Some("complete"));
        assert_eq!(back.promotion_profile, Some(PublishProfile::ProjectMemory));
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

    #[test]
    fn metadata_carrier_serde_round_trip() {
        let json = serde_json::to_string(&ArtifactMetadataCarrier::PacketMetadataSidecar).unwrap();
        assert_eq!(json, "\"packet-metadata-sidecar\"");
        let back: ArtifactMetadataCarrier = serde_json::from_str(&json).unwrap();
        assert_eq!(back, ArtifactMetadataCarrier::PacketMetadataSidecar);
    }

    #[test]
    fn metadata_and_publication_target_strings_are_stable() {
        for (carrier, expected) in [
            (ArtifactMetadataCarrier::ManagedSurfaceEnvelope, "managed-surface-envelope"),
            (ArtifactMetadataCarrier::PacketMetadataSidecar, "packet-metadata-sidecar"),
        ] {
            assert_eq!(carrier.as_str(), expected);
            assert_eq!(carrier.to_string(), expected);
            assert!(!carrier.discovery_rule().is_empty());
        }

        for (artifact_class, expected, carrier) in [
            (
                IndexableArtifactClass::ManagedSurface,
                "managed-surface",
                ArtifactMetadataCarrier::ManagedSurfaceEnvelope,
            ),
            (
                IndexableArtifactClass::ProposalArtifact,
                "proposal-artifact",
                ArtifactMetadataCarrier::PacketMetadataSidecar,
            ),
            (
                IndexableArtifactClass::EvidenceBundle,
                "evidence-bundle",
                ArtifactMetadataCarrier::PacketMetadataSidecar,
            ),
            (
                IndexableArtifactClass::IndexSurface,
                "index-surface",
                ArtifactMetadataCarrier::PacketMetadataSidecar,
            ),
        ] {
            assert_eq!(artifact_class.as_str(), expected);
            assert_eq!(artifact_class.to_string(), expected);
            assert_eq!(artifact_class.metadata_carrier(), carrier);
            assert_eq!(artifact_class.discovery_rule(), carrier.discovery_rule());
        }

        for (target_class, expected) in [
            (PublicationTargetClass::Stable, "stable"),
            (PublicationTargetClass::Pending, "pending"),
            (PublicationTargetClass::Proposal, "proposal"),
            (PublicationTargetClass::Evidence, "evidence"),
            (PublicationTargetClass::Index, "index"),
        ] {
            assert_eq!(target_class.as_str(), expected);
            assert_eq!(target_class.to_string(), expected);
        }
    }

    #[test]
    fn publication_target_class_maps_project_memory_outcomes() {
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::Auto,
                UpdateStrategy::ManagedBlocks,
            ),
            PublicationTargetClass::Stable
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::PendingIndex,
                UpdateStrategy::ManagedBlocks,
            ),
            PublicationTargetClass::Pending
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::PendingIndex,
                UpdateStrategy::ProposalFiles,
            ),
            PublicationTargetClass::Proposal
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::EvidenceOnly,
                UpdateStrategy::AppendOnlyIndex,
            ),
            PublicationTargetClass::Evidence
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::IndexOnly,
                UpdateStrategy::AppendOnlyIndex,
            ),
            PublicationTargetClass::Index
        );
        assert_eq!(
            PublicationTargetClass::for_publication(
                PromotionState::Auto,
                UpdateStrategy::AppendOnlyIndex,
            ),
            PublicationTargetClass::Pending
        );
    }

    #[test]
    fn expertise_input_metadata_normalizes_domain_families() {
        let metadata = classify_governed_expertise_input(
            Mode::DomainLanguage,
            vec!["systems".to_string(), String::new(), "systems".to_string()],
        )
        .expect("supported expertise metadata");

        assert_eq!(metadata.expertise_kind, GovernedExpertiseKind::DomainLanguage);
        assert_eq!(metadata.domain_families, vec!["systems".to_string()]);
        assert!(
            classify_governed_expertise_input(Mode::Review, vec!["systems".to_string()]).is_none()
        );
        assert!(classify_governed_expertise_input(Mode::DomainModel, Vec::new()).is_none());

        let unnormalized = ExpertiseInputMetadata {
            expertise_kind: GovernedExpertiseKind::DomainModel,
            domain_families: vec![
                " bounded-contexts ".to_string(),
                String::new(),
                "bounded-contexts".to_string(),
                "services".to_string(),
            ],
        };
        assert_eq!(
            unnormalized.normalized(),
            Some(ExpertiseInputMetadata {
                expertise_kind: GovernedExpertiseKind::DomainModel,
                domain_families: vec!["bounded-contexts".to_string(), "services".to_string(),],
            })
        );

        let empty = ExpertiseInputMetadata {
            expertise_kind: GovernedExpertiseKind::DomainLanguage,
            domain_families: vec!["   ".to_string()],
        };
        assert_eq!(empty.normalized(), None);
    }
}
