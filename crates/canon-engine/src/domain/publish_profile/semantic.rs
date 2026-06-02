use super::publication::UpdateStrategy;
use super::*;

/// Contract line string for the first semantic artifact descriptor slice.
pub const SEMANTIC_ARTIFACT_CONTRACT_LINE_V1: &str = "v1";

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
