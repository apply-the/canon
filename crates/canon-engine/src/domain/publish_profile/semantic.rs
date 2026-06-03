//! Typed semantic publication helpers for Canon publish-profile metadata.

use super::publication::UpdateStrategy;
use super::*;

/// Contract line string for the first semantic artifact descriptor slice.
pub const SEMANTIC_ARTIFACT_CONTRACT_LINE_V1: &str = "v1";
/// Stable contract line for the legacy governed reasoning posture publication.
pub const GOVERNED_REASONING_POSTURE_CONTRACT_LINE_V1: &str = "governed_reasoning_posture_v1";
/// Stable contract line for the current governed reasoning posture publication.
pub const GOVERNED_REASONING_POSTURE_CONTRACT_LINE_V2: &str = "governed_reasoning_posture_v2";
/// Stable schema version for the governed reasoning posture v2 payload.
pub const GOVERNED_REASONING_POSTURE_SCHEMA_VERSION_V2: &str = "v2";
/// Minimum supported Boundline version for the governed reasoning posture v1 line.
pub const GOVERNED_REASONING_POSTURE_V1_BOUNDLINE_MIN: &str = "0.62.0";
/// Maximum exclusive Boundline version for the governed reasoning posture v1 line.
pub const GOVERNED_REASONING_POSTURE_V1_BOUNDLINE_MAX_EXCLUSIVE: &str = "0.63.0";
/// Minimum supported Canon version for the governed reasoning posture v1 line.
pub const GOVERNED_REASONING_POSTURE_V1_CANON_MIN: &str = "0.63.1";
/// Maximum exclusive Canon version for the governed reasoning posture v1 line.
pub const GOVERNED_REASONING_POSTURE_V1_CANON_MAX_EXCLUSIVE: &str = "0.65.0";
/// Minimum supported Boundline version for the governed reasoning posture v2 line.
pub const GOVERNED_REASONING_POSTURE_V2_BOUNDLINE_MIN: &str = "0.63.0";
/// Maximum exclusive Boundline version for the governed reasoning posture v2 line.
pub const GOVERNED_REASONING_POSTURE_V2_BOUNDLINE_MAX_EXCLUSIVE: &str = "0.65.0";
/// Minimum supported Canon version for the governed reasoning posture v2 line.
pub const GOVERNED_REASONING_POSTURE_V2_CANON_MIN: &str = "0.65.0";
/// Maximum exclusive Canon version for the governed reasoning posture v2 line.
pub const GOVERNED_REASONING_POSTURE_V2_CANON_MAX_EXCLUSIVE: &str = "0.65.0";

/// Canon-owned contract line vocabulary for governed reasoning posture payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReasoningPostureContractLine {
    #[serde(rename = "governed_reasoning_posture_v1")]
    V1,
    #[serde(rename = "governed_reasoning_posture_v2")]
    V2,
}

impl ReasoningPostureContractLine {
    /// Returns the stable serialized string for this contract line.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1 => GOVERNED_REASONING_POSTURE_CONTRACT_LINE_V1,
            Self::V2 => GOVERNED_REASONING_POSTURE_CONTRACT_LINE_V2,
        }
    }
}

impl std::fmt::Display for ReasoningPostureContractLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Selector vocabulary for choosing a governed reasoning posture profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureSelectorKind {
    ProfileFamily,
    ProfileId,
}

/// Canon-supported profile-family vocabulary for governed reasoning posture payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureProfileFamily {
    SelfConsistency,
    BlindReview,
    HeterogeneousReview,
    Reflexion,
    DebateEnabled,
}

/// Canon-supported explicit profile identifiers for governed reasoning posture payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureProfileId {
    BoundedSelfConsistency,
    IndependentPairReview,
    HeterogeneousSecurityReview,
    BoundedReflexion,
}

/// Typed selector helper for governed reasoning posture payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureProfileSelector {
    pub selector_kind: ReasoningPostureSelectorKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_profile_family: Option<ReasoningPostureProfileFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_profile_id: Option<ReasoningPostureProfileId>,
}

impl ReasoningPostureProfileSelector {
    /// Validates that exactly one selector branch is populated.
    pub fn validate(&self) -> Result<(), String> {
        match self.selector_kind {
            ReasoningPostureSelectorKind::ProfileFamily => {
                if self.required_profile_family.is_none() {
                    return Err(
                        "profile_family selectors require required_profile_family".to_string()
                    );
                }
                if self.required_profile_id.is_some() {
                    return Err("profile_family selectors cannot also publish required_profile_id"
                        .to_string());
                }
            }
            ReasoningPostureSelectorKind::ProfileId => {
                if self.required_profile_id.is_none() {
                    return Err("profile_id selectors require required_profile_id".to_string());
                }
                if self.required_profile_family.is_some() {
                    return Err("profile_id selectors cannot also publish required_profile_family"
                        .to_string());
                }
            }
        }

        Ok(())
    }
}

/// Distinctness dimensions carried through the minimum-independence guidance block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureIndependenceDimension {
    RouteDistinct,
    ProviderDistinct,
    ContextDistinct,
    PromptPatternDistinct,
}

/// Non-negotiable independence minima for governed reasoning posture payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureIndependenceHardMinima {
    pub route_distinct: bool,
    pub provider_distinct: bool,
    pub context_distinct: bool,
    pub prompt_pattern_distinct: bool,
    pub minimum_participants: u8,
}

impl ReasoningPostureIndependenceHardMinima {
    /// Validates that the hard minima describe a feasible baseline posture.
    pub fn validate(&self) -> Result<(), String> {
        if self.minimum_participants < 2 {
            return Err("minimum_independence requires at least two participants".to_string());
        }

        if !self.route_distinct
            && !self.provider_distinct
            && !self.context_distinct
            && !self.prompt_pattern_distinct
        {
            return Err("minimum_independence requires at least one distinct dimension".to_string());
        }

        Ok(())
    }

    /// Returns true when the given dimension is enabled by the hard minima.
    pub const fn is_dimension_enabled(
        &self,
        dimension: ReasoningPostureIndependenceDimension,
    ) -> bool {
        match dimension {
            ReasoningPostureIndependenceDimension::RouteDistinct => self.route_distinct,
            ReasoningPostureIndependenceDimension::ProviderDistinct => self.provider_distinct,
            ReasoningPostureIndependenceDimension::ContextDistinct => self.context_distinct,
            ReasoningPostureIndependenceDimension::PromptPatternDistinct => {
                self.prompt_pattern_distinct
            }
        }
    }
}

/// Optional guidance that may strengthen or elaborate the hard minima.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureIndependenceGuidance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_minimum_participants: Option<u8>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_distinct_dimensions: Vec<ReasoningPostureIndependenceDimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance_notes_ref: Option<String>,
}

impl ReasoningPostureIndependenceGuidance {
    /// Validates that the guidance does not weaken the hard minima.
    pub fn validate_against(
        &self,
        hard_minima: &ReasoningPostureIndependenceHardMinima,
    ) -> Result<(), String> {
        if self
            .recommended_minimum_participants
            .is_some_and(|value| value < hard_minima.minimum_participants)
        {
            return Err(
                "minimum_independence guidance cannot weaken minimum_participants".to_string()
            );
        }

        if self
            .preferred_distinct_dimensions
            .iter()
            .any(|dimension| !hard_minima.is_dimension_enabled(*dimension))
        {
            return Err("minimum_independence guidance cannot prefer disabled distinct dimensions"
                .to_string());
        }

        Ok(())
    }
}

/// Typed helper for the full minimum-independence block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureMinimumIndependence {
    pub hard_minima: ReasoningPostureIndependenceHardMinima,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance: Option<ReasoningPostureIndependenceGuidance>,
}

impl ReasoningPostureMinimumIndependence {
    /// Validates the full minimum-independence block.
    pub fn validate(&self) -> Result<(), String> {
        self.hard_minima.validate()?;
        if let Some(guidance) = &self.guidance {
            guidance.validate_against(&self.hard_minima)?;
        }
        Ok(())
    }
}

/// Supported fail-closed modes for governed reasoning posture validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureRejectionMode {
    FailClosed,
}

/// Supported validation rules for required confidence handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureConfidenceValidationRule {
    ReferenceKindRequired,
    EvidenceBackedProvenanceRequired,
}

/// Supported confidence-handoff states for governed reasoning posture payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureConfidenceHandoffState {
    None,
    Required,
}

/// Typed helper for the confidence-handoff block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureConfidenceHandoff {
    pub state: ReasoningPostureConfidenceHandoffState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumer_obligation: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_rules: Vec<ReasoningPostureConfidenceValidationRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_ref_ids: Vec<String>,
    pub rejection_mode: ReasoningPostureRejectionMode,
}

impl ReasoningPostureConfidenceHandoff {
    /// Validates the confidence-handoff block for the selected state.
    pub fn validate(&self) -> Result<(), String> {
        let has_required_fields =
            self.consumer_obligation.as_deref().is_some_and(|value| !value.trim().is_empty())
                || !self.validation_rules.is_empty()
                || !self.evidence_ref_ids.is_empty();

        match self.state {
            ReasoningPostureConfidenceHandoffState::None => {
                if has_required_fields {
                    return Err(
                        "confidence_handoff state none cannot publish required-handoff fields"
                            .to_string(),
                    );
                }
            }
            ReasoningPostureConfidenceHandoffState::Required => {
                if self.consumer_obligation.as_deref().map(str::trim).unwrap_or_default().is_empty()
                {
                    return Err(
                        "confidence_handoff state required needs consumer_obligation".to_string()
                    );
                }
                if self.validation_rules.is_empty() {
                    return Err(
                        "confidence_handoff state required needs validation_rules".to_string()
                    );
                }
                if self.evidence_ref_ids.is_empty() {
                    return Err(
                        "confidence_handoff state required needs evidence_ref_ids".to_string()
                    );
                }
            }
        }

        Ok(())
    }
}

/// Supported provenance states for governed reasoning posture payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureProvenanceState {
    Minimal,
    EvidenceBacked,
}

/// Stable provenance reference kinds for governed reasoning posture payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningPostureReferenceKind {
    Packet,
    Artifact,
    StableDoc,
    ValidationReport,
    Fixture,
}

/// Typed provenance reference helper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureProvenanceReference {
    pub reference_kind: ReasoningPostureReferenceKind,
    pub reference_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ReasoningPostureProvenanceReference {
    /// Validates one provenance reference.
    pub fn validate(&self) -> Result<(), String> {
        if self.reference_id.trim().is_empty() {
            return Err("provenance references require reference_id".to_string());
        }
        Ok(())
    }
}

/// Typed helper for the provenance block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureProvenance {
    pub state: ReasoningPostureProvenanceState,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<ReasoningPostureProvenanceReference>,
}

impl ReasoningPostureProvenance {
    /// Validates provenance requirements against the selected handoff state.
    pub fn validate_against_handoff(
        &self,
        handoff_state: ReasoningPostureConfidenceHandoffState,
    ) -> Result<(), String> {
        if self.references.is_empty() {
            return Err("provenance requires at least one reference".to_string());
        }
        for reference in &self.references {
            reference.validate()?;
        }
        if matches!(handoff_state, ReasoningPostureConfidenceHandoffState::Required)
            && !matches!(self.state, ReasoningPostureProvenanceState::EvidenceBacked)
        {
            return Err("required confidence handoff needs evidence-backed provenance".to_string());
        }
        if matches!(self.state, ReasoningPostureProvenanceState::Minimal)
            && self.references.iter().any(|reference| {
                matches!(reference.reference_kind, ReasoningPostureReferenceKind::ValidationReport)
            })
        {
            return Err("minimal provenance cannot publish validation-report evidence".to_string());
        }

        Ok(())
    }
}

/// Typed compatibility window helper for governed reasoning posture payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningPostureCompatibilityWindow {
    pub boundline_min: String,
    pub boundline_max_exclusive: String,
    pub canon_min: String,
    pub canon_max_exclusive: String,
    pub contract_line: ReasoningPostureContractLine,
}

impl ReasoningPostureCompatibilityWindow {
    /// Returns the canonical compatibility window for the given contract line.
    pub fn expected_for_contract_line(contract_line: ReasoningPostureContractLine) -> Self {
        match contract_line {
            ReasoningPostureContractLine::V1 => Self {
                boundline_min: GOVERNED_REASONING_POSTURE_V1_BOUNDLINE_MIN.to_string(),
                boundline_max_exclusive: GOVERNED_REASONING_POSTURE_V1_BOUNDLINE_MAX_EXCLUSIVE
                    .to_string(),
                canon_min: GOVERNED_REASONING_POSTURE_V1_CANON_MIN.to_string(),
                canon_max_exclusive: GOVERNED_REASONING_POSTURE_V1_CANON_MAX_EXCLUSIVE.to_string(),
                contract_line,
            },
            ReasoningPostureContractLine::V2 => Self {
                boundline_min: GOVERNED_REASONING_POSTURE_V2_BOUNDLINE_MIN.to_string(),
                boundline_max_exclusive: GOVERNED_REASONING_POSTURE_V2_BOUNDLINE_MAX_EXCLUSIVE
                    .to_string(),
                canon_min: GOVERNED_REASONING_POSTURE_V2_CANON_MIN.to_string(),
                canon_max_exclusive: GOVERNED_REASONING_POSTURE_V2_CANON_MAX_EXCLUSIVE.to_string(),
                contract_line,
            },
        }
    }

    /// Validates that the compatibility window matches the declared contract line.
    pub fn validate_against_contract_line(
        &self,
        contract_line: ReasoningPostureContractLine,
    ) -> Result<(), String> {
        let expected = Self::expected_for_contract_line(contract_line);
        if self.contract_line != contract_line {
            return Err(format!(
                "compatibility window contract line `{}` does not match payload line `{}`",
                self.contract_line, contract_line,
            ));
        }
        if self.boundline_min != expected.boundline_min
            || self.boundline_max_exclusive != expected.boundline_max_exclusive
            || self.canon_min != expected.canon_min
            || self.canon_max_exclusive != expected.canon_max_exclusive
        {
            return Err(format!(
                "compatibility window does not match the canonical {} release pair",
                contract_line,
            ));
        }

        Ok(())
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
    /// Evidence bundles emitted under `tech-docs/evidence/` or another readable
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

/// Typed indexing metadata published with Canon-owned sidecars.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactIndexingMetadata {
    /// The stable artifact class exposed to downstream consumers.
    pub artifact_class: IndexableArtifactClass,
    /// The normative metadata carrier family for this artifact class.
    pub metadata_carrier: ArtifactMetadataCarrier,
    /// The discovery rule consumers should apply for this artifact class.
    pub discovery_rule: String,
}

/// Canon semantic eligibility posture for one published artifact surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticEligibilityState {
    Eligible,
    Excluded,
}

impl SemanticEligibilityState {
    /// Returns the kebab-case string representation of this eligibility state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::Excluded => "excluded",
        }
    }
}

impl std::fmt::Display for SemanticEligibilityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Canon-owned semantic provenance boundary for one published artifact surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticProvenanceBoundary {
    Surface,
    ManagedBlock,
    Section,
}

impl SemanticProvenanceBoundary {
    /// Returns the stable serialized representation of this provenance boundary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::ManagedBlock => "managed_block",
            Self::Section => "section",
        }
    }
}

impl std::fmt::Display for SemanticProvenanceBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Canon-owned semantic descriptor carried through existing packet metadata surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticArtifactDescriptor {
    pub semantic_contract_line: String,
    pub semantic_eligibility: SemanticEligibilityState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_provenance_boundary: Option<SemanticProvenanceBoundary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_provenance_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_exclusion_reason: Option<String>,
}

impl SemanticArtifactDescriptor {
    /// Validates the semantic descriptor shape before it is published.
    pub fn validate(&self) -> Result<(), String> {
        if self.semantic_contract_line.trim().is_empty() {
            return Err("semantic descriptor requires a semantic_contract_line".to_string());
        }

        for label in &self.semantic_labels {
            if label.trim().is_empty() {
                return Err(
                    "semantic descriptor labels must not be empty when provided".to_string()
                );
            }
        }

        if self.semantic_provenance_boundary.is_none() {
            return Err("semantic descriptor requires semantic_provenance_boundary".to_string());
        }
        if self.semantic_provenance_ref.as_deref().map(str::trim).unwrap_or_default().is_empty() {
            return Err("semantic descriptor requires semantic_provenance_ref".to_string());
        }

        if self.semantic_exclusion_reason.as_deref().is_some_and(|reason| reason.trim().is_empty())
        {
            return Err("semantic exclusion reason must not be empty when provided".to_string());
        }

        Ok(())
    }
}

impl ArtifactIndexingMetadata {
    /// Builds typed indexing metadata from the effective publication class and
    /// update strategy, rejecting ambiguous combinations.
    pub fn for_publication(
        publication_target_class: PublicationTargetClass,
        update_strategy: UpdateStrategy,
    ) -> Result<Self, String> {
        let artifact_class =
            indexable_artifact_class_for_publication(publication_target_class, update_strategy)?;

        Ok(Self {
            artifact_class,
            metadata_carrier: artifact_class.metadata_carrier(),
            discovery_rule: artifact_class.discovery_rule().to_string(),
        })
    }

    /// Validates that the carrier and discovery rule remain aligned with the
    /// declared artifact class.
    pub fn validate(&self) -> Result<(), String> {
        if self.metadata_carrier != self.artifact_class.metadata_carrier() {
            return Err(format!(
                "artifact class `{}` requires metadata carrier `{}`; found `{}`",
                self.artifact_class,
                self.artifact_class.metadata_carrier(),
                self.metadata_carrier,
            ));
        }
        if self.discovery_rule.trim() != self.artifact_class.discovery_rule() {
            return Err(format!(
                "artifact class `{}` requires discovery rule `{}`",
                self.artifact_class,
                self.artifact_class.discovery_rule(),
            ));
        }
        Ok(())
    }
}

/// Resolves the stable indexable artifact class for one publication target.
pub fn indexable_artifact_class_for_publication(
    publication_target_class: PublicationTargetClass,
    update_strategy: UpdateStrategy,
) -> Result<IndexableArtifactClass, String> {
    match (publication_target_class, update_strategy) {
        (
            PublicationTargetClass::Stable | PublicationTargetClass::Pending,
            UpdateStrategy::ManagedBlocks,
        ) => Ok(IndexableArtifactClass::ManagedSurface),
        (PublicationTargetClass::Proposal, UpdateStrategy::ProposalFiles) => {
            Ok(IndexableArtifactClass::ProposalArtifact)
        }
        (PublicationTargetClass::Evidence, UpdateStrategy::AppendOnlyIndex) => {
            Ok(IndexableArtifactClass::EvidenceBundle)
        }
        (PublicationTargetClass::Index, UpdateStrategy::AppendOnlyIndex) => {
            Ok(IndexableArtifactClass::IndexSurface)
        }
        (target_class, strategy) => Err(format!(
            "unsupported artifact indexing mapping for target class `{}` with update strategy `{}`",
            target_class, strategy
        )),
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

#[cfg(test)]
mod reasoning_posture_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn reasoning_posture_contract_line_strings_and_serialization_are_stable() {
        let cases = [
            (ReasoningPostureContractLine::V1, GOVERNED_REASONING_POSTURE_CONTRACT_LINE_V1),
            (ReasoningPostureContractLine::V2, GOVERNED_REASONING_POSTURE_CONTRACT_LINE_V2),
        ];

        for (contract_line, expected) in cases {
            assert_eq!(contract_line.as_str(), expected);
            assert_eq!(contract_line.to_string(), expected);
            assert_eq!(serde_json::to_value(contract_line).unwrap(), json!(expected));
        }
    }

    #[test]
    fn reasoning_posture_vocabularies_serialize_to_stable_strings() {
        for (selector_kind, expected) in [
            (ReasoningPostureSelectorKind::ProfileFamily, "profile_family"),
            (ReasoningPostureSelectorKind::ProfileId, "profile_id"),
        ] {
            assert_eq!(serde_json::to_value(selector_kind).unwrap(), json!(expected));
        }

        for (family, expected) in [
            (ReasoningPostureProfileFamily::SelfConsistency, "self_consistency"),
            (ReasoningPostureProfileFamily::BlindReview, "blind_review"),
            (ReasoningPostureProfileFamily::HeterogeneousReview, "heterogeneous_review"),
            (ReasoningPostureProfileFamily::Reflexion, "reflexion"),
            (ReasoningPostureProfileFamily::DebateEnabled, "debate_enabled"),
        ] {
            assert_eq!(serde_json::to_value(family).unwrap(), json!(expected));
        }

        for (profile_id, expected) in [
            (ReasoningPostureProfileId::BoundedSelfConsistency, "bounded_self_consistency"),
            (ReasoningPostureProfileId::IndependentPairReview, "independent_pair_review"),
            (
                ReasoningPostureProfileId::HeterogeneousSecurityReview,
                "heterogeneous_security_review",
            ),
            (ReasoningPostureProfileId::BoundedReflexion, "bounded_reflexion"),
        ] {
            assert_eq!(serde_json::to_value(profile_id).unwrap(), json!(expected));
        }

        for (dimension, expected) in [
            (ReasoningPostureIndependenceDimension::RouteDistinct, "route_distinct"),
            (ReasoningPostureIndependenceDimension::ProviderDistinct, "provider_distinct"),
            (ReasoningPostureIndependenceDimension::ContextDistinct, "context_distinct"),
            (
                ReasoningPostureIndependenceDimension::PromptPatternDistinct,
                "prompt_pattern_distinct",
            ),
        ] {
            assert_eq!(serde_json::to_value(dimension).unwrap(), json!(expected));
        }

        for (rule, expected) in [
            (
                ReasoningPostureConfidenceValidationRule::ReferenceKindRequired,
                "reference_kind_required",
            ),
            (
                ReasoningPostureConfidenceValidationRule::EvidenceBackedProvenanceRequired,
                "evidence_backed_provenance_required",
            ),
        ] {
            assert_eq!(serde_json::to_value(rule).unwrap(), json!(expected));
        }

        for (state, expected) in [
            (ReasoningPostureConfidenceHandoffState::None, "none"),
            (ReasoningPostureConfidenceHandoffState::Required, "required"),
        ] {
            assert_eq!(serde_json::to_value(state).unwrap(), json!(expected));
        }

        for (state, expected) in [
            (ReasoningPostureProvenanceState::Minimal, "minimal"),
            (ReasoningPostureProvenanceState::EvidenceBacked, "evidence_backed"),
        ] {
            assert_eq!(serde_json::to_value(state).unwrap(), json!(expected));
        }

        for (kind, expected) in [
            (ReasoningPostureReferenceKind::Packet, "packet"),
            (ReasoningPostureReferenceKind::Artifact, "artifact"),
            (ReasoningPostureReferenceKind::StableDoc, "stable_doc"),
            (ReasoningPostureReferenceKind::ValidationReport, "validation_report"),
            (ReasoningPostureReferenceKind::Fixture, "fixture"),
        ] {
            assert_eq!(serde_json::to_value(kind).unwrap(), json!(expected));
        }

        assert_eq!(
            serde_json::to_value(ReasoningPostureRejectionMode::FailClosed).unwrap(),
            json!("fail_closed")
        );
    }

    #[test]
    fn reasoning_posture_profile_selector_rejects_both_selector_branches() {
        let selector = ReasoningPostureProfileSelector {
            selector_kind: ReasoningPostureSelectorKind::ProfileFamily,
            required_profile_family: Some(ReasoningPostureProfileFamily::BlindReview),
            required_profile_id: Some(ReasoningPostureProfileId::IndependentPairReview),
        };

        let error = selector.validate().unwrap_err();
        assert!(error.contains("cannot also publish required_profile_id"));
    }

    #[test]
    fn reasoning_posture_profile_selector_covers_profile_id_and_missing_family_branches() {
        let profile_id_selector = ReasoningPostureProfileSelector {
            selector_kind: ReasoningPostureSelectorKind::ProfileId,
            required_profile_family: None,
            required_profile_id: Some(ReasoningPostureProfileId::IndependentPairReview),
        };
        assert!(profile_id_selector.validate().is_ok());

        let missing_family_selector = ReasoningPostureProfileSelector {
            selector_kind: ReasoningPostureSelectorKind::ProfileFamily,
            required_profile_family: None,
            required_profile_id: None,
        };
        let error = missing_family_selector.validate().unwrap_err();
        assert!(error.contains("require required_profile_family"));

        let conflicting_profile_id_selector = ReasoningPostureProfileSelector {
            selector_kind: ReasoningPostureSelectorKind::ProfileId,
            required_profile_family: Some(ReasoningPostureProfileFamily::BlindReview),
            required_profile_id: Some(ReasoningPostureProfileId::IndependentPairReview),
        };
        let error = conflicting_profile_id_selector.validate().unwrap_err();
        assert!(error.contains("cannot also publish required_profile_family"));
    }

    #[test]
    fn reasoning_posture_hard_minima_cover_impossible_and_dimension_queries() {
        let impossible = ReasoningPostureIndependenceHardMinima {
            route_distinct: true,
            provider_distinct: true,
            context_distinct: true,
            prompt_pattern_distinct: true,
            minimum_participants: 1,
        };
        let error = impossible.validate().unwrap_err();
        assert!(error.contains("at least two participants"));

        let no_distinct_dimensions = ReasoningPostureIndependenceHardMinima {
            route_distinct: false,
            provider_distinct: false,
            context_distinct: false,
            prompt_pattern_distinct: false,
            minimum_participants: 2,
        };
        let error = no_distinct_dimensions.validate().unwrap_err();
        assert!(error.contains("at least one distinct dimension"));

        let valid = ReasoningPostureIndependenceHardMinima {
            route_distinct: true,
            provider_distinct: true,
            context_distinct: false,
            prompt_pattern_distinct: true,
            minimum_participants: 2,
        };
        assert!(valid.validate().is_ok());
        assert!(valid.is_dimension_enabled(ReasoningPostureIndependenceDimension::RouteDistinct));
        assert!(
            valid.is_dimension_enabled(ReasoningPostureIndependenceDimension::ProviderDistinct)
        );
        assert!(
            !valid.is_dimension_enabled(ReasoningPostureIndependenceDimension::ContextDistinct)
        );
        assert!(
            valid
                .is_dimension_enabled(ReasoningPostureIndependenceDimension::PromptPatternDistinct)
        );
    }

    #[test]
    fn reasoning_posture_guidance_cannot_weaken_minimum_participants() {
        let minimum_independence = ReasoningPostureMinimumIndependence {
            hard_minima: ReasoningPostureIndependenceHardMinima {
                route_distinct: true,
                provider_distinct: true,
                context_distinct: true,
                prompt_pattern_distinct: true,
                minimum_participants: 3,
            },
            guidance: Some(ReasoningPostureIndependenceGuidance {
                recommended_minimum_participants: Some(2),
                preferred_distinct_dimensions: vec![
                    ReasoningPostureIndependenceDimension::RouteDistinct,
                ],
                guidance_notes_ref: None,
            }),
        };

        let error = minimum_independence.validate().unwrap_err();
        assert!(error.contains("cannot weaken minimum_participants"));
    }

    #[test]
    fn reasoning_posture_guidance_rejects_disabled_dimensions_and_accepts_stronger_guidance() {
        let hard_minima = ReasoningPostureIndependenceHardMinima {
            route_distinct: true,
            provider_distinct: true,
            context_distinct: false,
            prompt_pattern_distinct: true,
            minimum_participants: 2,
        };

        let error = ReasoningPostureIndependenceGuidance {
            recommended_minimum_participants: Some(3),
            preferred_distinct_dimensions: vec![
                ReasoningPostureIndependenceDimension::ContextDistinct,
            ],
            guidance_notes_ref: None,
        }
        .validate_against(&hard_minima)
        .unwrap_err();
        assert!(error.contains("cannot prefer disabled distinct dimensions"));

        let minimum_independence = ReasoningPostureMinimumIndependence {
            hard_minima,
            guidance: Some(ReasoningPostureIndependenceGuidance {
                recommended_minimum_participants: Some(3),
                preferred_distinct_dimensions: vec![
                    ReasoningPostureIndependenceDimension::RouteDistinct,
                    ReasoningPostureIndependenceDimension::ProviderDistinct,
                ],
                guidance_notes_ref: Some("validation-report:reasoning-posture-v2".to_string()),
            }),
        };
        assert!(minimum_independence.validate().is_ok());
    }

    #[test]
    fn reasoning_posture_confidence_handoff_covers_none_and_required_states() {
        let none_state = ReasoningPostureConfidenceHandoff {
            state: ReasoningPostureConfidenceHandoffState::None,
            consumer_obligation: None,
            validation_rules: Vec::new(),
            evidence_ref_ids: Vec::new(),
            rejection_mode: ReasoningPostureRejectionMode::FailClosed,
        };
        assert!(none_state.validate().is_ok());

        let contradictory_none_state = ReasoningPostureConfidenceHandoff {
            state: ReasoningPostureConfidenceHandoffState::None,
            consumer_obligation: Some("require_structured_confidence_review".to_string()),
            validation_rules: Vec::new(),
            evidence_ref_ids: Vec::new(),
            rejection_mode: ReasoningPostureRejectionMode::FailClosed,
        };
        let error = contradictory_none_state.validate().unwrap_err();
        assert!(error.contains("state none cannot publish required-handoff fields"));

        let missing_obligation = ReasoningPostureConfidenceHandoff {
            state: ReasoningPostureConfidenceHandoffState::Required,
            consumer_obligation: None,
            validation_rules: vec![ReasoningPostureConfidenceValidationRule::ReferenceKindRequired],
            evidence_ref_ids: vec!["validation-report:reasoning-posture-v2".to_string()],
            rejection_mode: ReasoningPostureRejectionMode::FailClosed,
        };
        let error = missing_obligation.validate().unwrap_err();
        assert!(error.contains("needs consumer_obligation"));

        let missing_rules = ReasoningPostureConfidenceHandoff {
            state: ReasoningPostureConfidenceHandoffState::Required,
            consumer_obligation: Some("require_structured_confidence_review".to_string()),
            validation_rules: Vec::new(),
            evidence_ref_ids: vec!["validation-report:reasoning-posture-v2".to_string()],
            rejection_mode: ReasoningPostureRejectionMode::FailClosed,
        };
        let error = missing_rules.validate().unwrap_err();
        assert!(error.contains("needs validation_rules"));

        let missing_evidence = ReasoningPostureConfidenceHandoff {
            state: ReasoningPostureConfidenceHandoffState::Required,
            consumer_obligation: Some("require_structured_confidence_review".to_string()),
            validation_rules: vec![ReasoningPostureConfidenceValidationRule::ReferenceKindRequired],
            evidence_ref_ids: Vec::new(),
            rejection_mode: ReasoningPostureRejectionMode::FailClosed,
        };
        let error = missing_evidence.validate().unwrap_err();
        assert!(error.contains("needs evidence_ref_ids"));

        let valid_required = ReasoningPostureConfidenceHandoff {
            state: ReasoningPostureConfidenceHandoffState::Required,
            consumer_obligation: Some("require_structured_confidence_review".to_string()),
            validation_rules: vec![
                ReasoningPostureConfidenceValidationRule::ReferenceKindRequired,
                ReasoningPostureConfidenceValidationRule::EvidenceBackedProvenanceRequired,
            ],
            evidence_ref_ids: vec!["validation-report:reasoning-posture-v2".to_string()],
            rejection_mode: ReasoningPostureRejectionMode::FailClosed,
        };
        assert!(valid_required.validate().is_ok());
    }

    #[test]
    fn reasoning_posture_required_handoff_needs_evidence_backed_provenance() {
        let provenance = ReasoningPostureProvenance {
            state: ReasoningPostureProvenanceState::Minimal,
            references: vec![ReasoningPostureProvenanceReference {
                reference_kind: ReasoningPostureReferenceKind::StableDoc,
                reference_id: "tech-docs/integration/governed-reasoning-posture-contract.md"
                    .to_string(),
                description: None,
            }],
        };

        let error = provenance
            .validate_against_handoff(ReasoningPostureConfidenceHandoffState::Required)
            .unwrap_err();
        assert!(error.contains("evidence-backed provenance"));
    }

    #[test]
    fn reasoning_posture_provenance_covers_reference_and_state_failures() {
        let error = ReasoningPostureProvenance {
            state: ReasoningPostureProvenanceState::Minimal,
            references: Vec::new(),
        }
        .validate_against_handoff(ReasoningPostureConfidenceHandoffState::None)
        .unwrap_err();
        assert!(error.contains("at least one reference"));

        let error = ReasoningPostureProvenanceReference {
            reference_kind: ReasoningPostureReferenceKind::StableDoc,
            reference_id: "   ".to_string(),
            description: None,
        }
        .validate()
        .unwrap_err();
        assert!(error.contains("require reference_id"));

        let minimal_with_validation_report = ReasoningPostureProvenance {
            state: ReasoningPostureProvenanceState::Minimal,
            references: vec![ReasoningPostureProvenanceReference {
                reference_kind: ReasoningPostureReferenceKind::ValidationReport,
                reference_id: "validation-report:reasoning-posture-v2".to_string(),
                description: None,
            }],
        };
        let error = minimal_with_validation_report
            .validate_against_handoff(ReasoningPostureConfidenceHandoffState::None)
            .unwrap_err();
        assert!(error.contains("validation-report evidence"));

        let valid_evidence_backed = ReasoningPostureProvenance {
            state: ReasoningPostureProvenanceState::EvidenceBacked,
            references: vec![ReasoningPostureProvenanceReference {
                reference_kind: ReasoningPostureReferenceKind::Artifact,
                reference_id: "packet:reasoning-posture-v2".to_string(),
                description: Some("Packet artifact".to_string()),
            }],
        };
        assert!(
            valid_evidence_backed
                .validate_against_handoff(ReasoningPostureConfidenceHandoffState::Required)
                .is_ok()
        );
    }

    #[test]
    fn reasoning_posture_compatibility_window_matches_v2_constants() {
        let window = ReasoningPostureCompatibilityWindow::expected_for_contract_line(
            ReasoningPostureContractLine::V2,
        );

        assert!(window.validate_against_contract_line(ReasoningPostureContractLine::V2).is_ok());
        assert_eq!(window.boundline_min, GOVERNED_REASONING_POSTURE_V2_BOUNDLINE_MIN);
        assert_eq!(window.canon_max_exclusive, GOVERNED_REASONING_POSTURE_V2_CANON_MAX_EXCLUSIVE);
    }

    #[test]
    fn reasoning_posture_compatibility_window_covers_v1_and_mismatch_paths() {
        let v1_window = ReasoningPostureCompatibilityWindow::expected_for_contract_line(
            ReasoningPostureContractLine::V1,
        );
        assert!(v1_window.validate_against_contract_line(ReasoningPostureContractLine::V1).is_ok());

        let mismatched_line = ReasoningPostureCompatibilityWindow {
            contract_line: ReasoningPostureContractLine::V1,
            ..ReasoningPostureCompatibilityWindow::expected_for_contract_line(
                ReasoningPostureContractLine::V2,
            )
        };
        let error = mismatched_line
            .validate_against_contract_line(ReasoningPostureContractLine::V2)
            .unwrap_err();
        assert!(error.contains("does not match payload line"));

        let mismatched_versions = ReasoningPostureCompatibilityWindow {
            boundline_min: GOVERNED_REASONING_POSTURE_V1_BOUNDLINE_MIN.to_string(),
            ..ReasoningPostureCompatibilityWindow::expected_for_contract_line(
                ReasoningPostureContractLine::V2,
            )
        };
        let error = mismatched_versions
            .validate_against_contract_line(ReasoningPostureContractLine::V2)
            .unwrap_err();
        assert!(error.contains("canonical governed_reasoning_posture_v2 release pair"));
    }
}
