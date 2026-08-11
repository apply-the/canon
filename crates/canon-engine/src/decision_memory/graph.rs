//! Typed decision-memory nodes, edges, conflicts, and graph invariants.

use std::collections::{BTreeMap, BTreeSet};

use canon_contracts::{Profile, RecordOutcomeRequest};
use serde::{Deserialize, Serialize};

use super::digest::is_sha256;
use super::{ContentDigest, DecisionMemoryError};

/// Frozen internal decision-memory graph schema.
pub const DECISION_MEMORY_SCHEMA_VERSION: &str = "canon-decision-memory-v1";
/// Frozen public contract line consumed by the graph.
pub const DECISION_MEMORY_CONTRACT_LINE: &str = "canon-contracts/1.0";

/// Stable identity for one internal decision-memory node.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(String);

impl NodeId {
    /// Creates a stable node identity.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the stable wire value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Freshness state bound to exact content rather than display text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum FreshnessState {
    /// The recorded dependencies still match their admitted content.
    Fresh,
    /// A typed dependency changed after the node was recorded.
    Stale {
        /// Exact changed digest that caused the invalidation.
        observed_digest: String,
        /// Stable human-readable reason.
        reason: String,
        /// Ordered dependency path from the changed node.
        reason_chain: Vec<NodeId>,
    },
}

impl FreshnessState {
    /// Reports whether this node is stale.
    pub const fn is_stale(&self) -> bool {
        matches!(self, Self::Stale { .. })
    }
}

/// Common immutable envelope for every typed graph node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeEnvelope<T> {
    id: NodeId,
    content_digest: ContentDigest,
    /// Stable source provenance, never a local absolute path.
    pub provenance: String,
    /// Graph revision at which this node was introduced.
    pub revision_introduced: u64,
    /// Current exact-binding freshness.
    pub freshness: FreshnessState,
    /// Historical nodes explicitly superseded by this node.
    pub supersedes: Vec<NodeId>,
    /// Typed node content.
    pub content: T,
}

impl<T: Serialize> NodeEnvelope<T> {
    /// Constructs a fresh typed node and computes its content digest.
    pub fn new(
        id: NodeId,
        provenance: impl Into<String>,
        content: T,
    ) -> Result<Self, DecisionMemoryError> {
        require_identity(&id)?;
        require_safe_projection_text(id.as_str(), "node identity")?;
        let provenance = provenance.into();
        require_safe_projection_text(&provenance, "node provenance")?;
        let content_digest =
            ContentDigest::compute("decision-memory-node", &(&provenance, &content))?;
        Ok(Self {
            id,
            content_digest,
            provenance,
            revision_introduced: 0,
            freshness: FreshnessState::Fresh,
            supersedes: Vec::new(),
            content,
        })
    }

    /// Returns the stable node identity.
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// Returns the canonical content digest.
    pub fn content_digest(&self) -> &ContentDigest {
        &self.content_digest
    }

    fn recompute_content_digest(&self) -> Result<ContentDigest, DecisionMemoryError> {
        ContentDigest::compute("decision-memory-node", &(&self.provenance, &self.content))
    }
}

/// Terminal governance decision content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionContent {
    /// Source governance bundle identity.
    pub bundle_id: String,
    /// Immutable decision rationale.
    pub rationale: String,
    /// Considered alternatives.
    pub alternatives: Vec<String>,
    /// Assumptions supporting the decision.
    pub assumptions: Vec<String>,
    /// Triggers that require reconsideration.
    pub triggers: Vec<String>,
    /// Named owners.
    pub owners: Vec<String>,
    /// Deterministic terminal status.
    pub decision: super::GovernanceDecision,
}

/// Governance packet content bound to its exact digest and revision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketContent {
    /// Stable governance profile.
    pub profile: Profile,
    /// Exact packet digest.
    pub packet_digest: String,
    /// Packet revision.
    pub revision: u64,
    /// Named packet owner.
    pub owner: String,
}

/// Subject artifact binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactContent {
    /// Stable artifact identity.
    pub artifact_identity: String,
    /// Exact subject revision.
    pub revision: String,
    /// Exact artifact content digest.
    pub content_digest: String,
}

/// One governed claim and its owning packet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimContent {
    /// Exact packet identity.
    pub packet_id: NodeId,
    /// Stable claim text.
    pub statement: String,
}

/// Immutable assumption recorded with a decision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionContent {
    /// Assumption text.
    pub statement: String,
    /// Named owner.
    pub owner: String,
}

/// Alternative considered by a decision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlternativeContent {
    /// Alternative description.
    pub description: String,
    /// Why it was not selected.
    pub disposition: String,
}

/// Condition that makes a decision eligible for reconsideration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerContent {
    /// Stable trigger expression.
    pub condition: String,
    /// Named owner responsible for observation.
    pub owner: String,
}

/// An approval node uses the internal exact-binding metadata.
pub type ApprovalNode = NodeEnvelope<super::ApprovalContent>;
/// An artifact node uses an exact revision and digest.
pub type ArtifactNode = NodeEnvelope<ArtifactContent>;
/// An assumption node preserves decision history.
pub type AssumptionNode = NodeEnvelope<AssumptionContent>;
/// An alternative node preserves considered options.
pub type AlternativeNode = NodeEnvelope<AlternativeContent>;
/// A claim node is owned by an exact packet.
pub type ClaimNode = NodeEnvelope<ClaimContent>;
/// A decision node records one terminal deterministic result.
pub type DecisionNode = NodeEnvelope<DecisionContent>;
/// An evidence node records externally supplied or deterministic evidence.
pub type EvidenceNode = NodeEnvelope<super::EvidenceContent>;
/// A packet node records an exact packet binding.
pub type PacketNode = NodeEnvelope<PacketContent>;
/// A risk-acceptance node records named, bounded authority.
pub type RiskAcceptanceNode = NodeEnvelope<super::RiskAcceptanceContent>;
/// A trigger node records reconsideration conditions.
pub type TriggerNode = NodeEnvelope<TriggerContent>;
/// A verification requirement node records exact claim policy.
pub type VerificationRequirementNode = NodeEnvelope<super::VerificationRequirementContent>;

/// All typed node variants stored in decision memory.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "node", rename_all = "snake_case")]
pub enum DecisionMemoryNode {
    /// Terminal decision.
    Decision(DecisionNode),
    /// Governance packet.
    Packet(PacketNode),
    /// Subject artifact.
    Artifact(ArtifactNode),
    /// Governed claim.
    Claim(ClaimNode),
    /// Supplied evidence.
    Evidence(EvidenceNode),
    /// Named approval.
    Approval(ApprovalNode),
    /// Required verification.
    VerificationRequirement(VerificationRequirementNode),
    /// Decision assumption.
    Assumption(AssumptionNode),
    /// Considered alternative.
    Alternative(AlternativeNode),
    /// Reconsideration trigger.
    Trigger(TriggerNode),
    /// Named risk acceptance.
    RiskAcceptance(RiskAcceptanceNode),
}

impl DecisionMemoryNode {
    /// Returns the stable node identity.
    pub fn id(&self) -> &NodeId {
        match self {
            Self::Decision(node) => node.id(),
            Self::Packet(node) => node.id(),
            Self::Artifact(node) => node.id(),
            Self::Claim(node) => node.id(),
            Self::Evidence(node) => node.id(),
            Self::Approval(node) => node.id(),
            Self::VerificationRequirement(node) => node.id(),
            Self::Assumption(node) => node.id(),
            Self::Alternative(node) => node.id(),
            Self::Trigger(node) => node.id(),
            Self::RiskAcceptance(node) => node.id(),
        }
    }

    /// Returns the canonical content digest.
    pub fn content_digest(&self) -> &ContentDigest {
        match self {
            Self::Decision(node) => node.content_digest(),
            Self::Packet(node) => node.content_digest(),
            Self::Artifact(node) => node.content_digest(),
            Self::Claim(node) => node.content_digest(),
            Self::Evidence(node) => node.content_digest(),
            Self::Approval(node) => node.content_digest(),
            Self::VerificationRequirement(node) => node.content_digest(),
            Self::Assumption(node) => node.content_digest(),
            Self::Alternative(node) => node.content_digest(),
            Self::Trigger(node) => node.content_digest(),
            Self::RiskAcceptance(node) => node.content_digest(),
        }
    }

    pub(crate) fn freshness(&self) -> &FreshnessState {
        match self {
            Self::Decision(node) => &node.freshness,
            Self::Packet(node) => &node.freshness,
            Self::Artifact(node) => &node.freshness,
            Self::Claim(node) => &node.freshness,
            Self::Evidence(node) => &node.freshness,
            Self::Approval(node) => &node.freshness,
            Self::VerificationRequirement(node) => &node.freshness,
            Self::Assumption(node) => &node.freshness,
            Self::Alternative(node) => &node.freshness,
            Self::Trigger(node) => &node.freshness,
            Self::RiskAcceptance(node) => &node.freshness,
        }
    }

    pub(crate) fn freshness_mut(&mut self) -> &mut FreshnessState {
        match self {
            Self::Decision(node) => &mut node.freshness,
            Self::Packet(node) => &mut node.freshness,
            Self::Artifact(node) => &mut node.freshness,
            Self::Claim(node) => &mut node.freshness,
            Self::Evidence(node) => &mut node.freshness,
            Self::Approval(node) => &mut node.freshness,
            Self::VerificationRequirement(node) => &mut node.freshness,
            Self::Assumption(node) => &mut node.freshness,
            Self::Alternative(node) => &mut node.freshness,
            Self::Trigger(node) => &mut node.freshness,
            Self::RiskAcceptance(node) => &mut node.freshness,
        }
    }

    pub(crate) fn introduced_mut(&mut self) -> &mut u64 {
        match self {
            Self::Decision(node) => &mut node.revision_introduced,
            Self::Packet(node) => &mut node.revision_introduced,
            Self::Artifact(node) => &mut node.revision_introduced,
            Self::Claim(node) => &mut node.revision_introduced,
            Self::Evidence(node) => &mut node.revision_introduced,
            Self::Approval(node) => &mut node.revision_introduced,
            Self::VerificationRequirement(node) => &mut node.revision_introduced,
            Self::Assumption(node) => &mut node.revision_introduced,
            Self::Alternative(node) => &mut node.revision_introduced,
            Self::Trigger(node) => &mut node.revision_introduced,
            Self::RiskAcceptance(node) => &mut node.revision_introduced,
        }
    }

    pub(crate) fn supersedes_mut(&mut self) -> &mut Vec<NodeId> {
        match self {
            Self::Decision(node) => &mut node.supersedes,
            Self::Packet(node) => &mut node.supersedes,
            Self::Artifact(node) => &mut node.supersedes,
            Self::Claim(node) => &mut node.supersedes,
            Self::Evidence(node) => &mut node.supersedes,
            Self::Approval(node) => &mut node.supersedes,
            Self::VerificationRequirement(node) => &mut node.supersedes,
            Self::Assumption(node) => &mut node.supersedes,
            Self::Alternative(node) => &mut node.supersedes,
            Self::Trigger(node) => &mut node.supersedes,
            Self::RiskAcceptance(node) => &mut node.supersedes,
        }
    }

    pub(crate) fn provenance(&self) -> &str {
        match self {
            Self::Decision(node) => &node.provenance,
            Self::Packet(node) => &node.provenance,
            Self::Artifact(node) => &node.provenance,
            Self::Claim(node) => &node.provenance,
            Self::Evidence(node) => &node.provenance,
            Self::Approval(node) => &node.provenance,
            Self::VerificationRequirement(node) => &node.provenance,
            Self::Assumption(node) => &node.provenance,
            Self::Alternative(node) => &node.provenance,
            Self::Trigger(node) => &node.provenance,
            Self::RiskAcceptance(node) => &node.provenance,
        }
    }

    pub(crate) fn revision_introduced(&self) -> u64 {
        match self {
            Self::Decision(node) => node.revision_introduced,
            Self::Packet(node) => node.revision_introduced,
            Self::Artifact(node) => node.revision_introduced,
            Self::Claim(node) => node.revision_introduced,
            Self::Evidence(node) => node.revision_introduced,
            Self::Approval(node) => node.revision_introduced,
            Self::VerificationRequirement(node) => node.revision_introduced,
            Self::Assumption(node) => node.revision_introduced,
            Self::Alternative(node) => node.revision_introduced,
            Self::Trigger(node) => node.revision_introduced,
            Self::RiskAcceptance(node) => node.revision_introduced,
        }
    }

    pub(crate) fn supersedes(&self) -> &[NodeId] {
        match self {
            Self::Decision(node) => &node.supersedes,
            Self::Packet(node) => &node.supersedes,
            Self::Artifact(node) => &node.supersedes,
            Self::Claim(node) => &node.supersedes,
            Self::Evidence(node) => &node.supersedes,
            Self::Approval(node) => &node.supersedes,
            Self::VerificationRequirement(node) => &node.supersedes,
            Self::Assumption(node) => &node.supersedes,
            Self::Alternative(node) => &node.supersedes,
            Self::Trigger(node) => &node.supersedes,
            Self::RiskAcceptance(node) => &node.supersedes,
        }
    }

    pub(crate) const fn kind(&self) -> &'static str {
        match self {
            Self::Decision(_) => "decision",
            Self::Packet(_) => "packet",
            Self::Artifact(_) => "artifact",
            Self::Claim(_) => "claim",
            Self::Evidence(_) => "evidence",
            Self::Approval(_) => "approval",
            Self::VerificationRequirement(_) => "verification_requirement",
            Self::Assumption(_) => "assumption",
            Self::Alternative(_) => "alternative",
            Self::Trigger(_) => "trigger",
            Self::RiskAcceptance(_) => "risk_acceptance",
        }
    }

    fn validate_content(&self) -> Result<(), DecisionMemoryError> {
        match self {
            Self::Packet(node) if !is_sha256(&node.content.packet_digest) => {
                Err(DecisionMemoryError::MalformedDigest { identity: node.id().to_string() })
            }
            Self::Artifact(node) if !is_sha256(&node.content.content_digest) => {
                Err(DecisionMemoryError::MalformedDigest { identity: node.id().to_string() })
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn recorded_digest_is_valid(&self) -> bool {
        let recomputed = match self {
            Self::Decision(node) => node.recompute_content_digest(),
            Self::Packet(node) => node.recompute_content_digest(),
            Self::Artifact(node) => node.recompute_content_digest(),
            Self::Claim(node) => node.recompute_content_digest(),
            Self::Evidence(node) => node.recompute_content_digest(),
            Self::Approval(node) => node.recompute_content_digest(),
            Self::VerificationRequirement(node) => node.recompute_content_digest(),
            Self::Assumption(node) => node.recompute_content_digest(),
            Self::Alternative(node) => node.recompute_content_digest(),
            Self::Trigger(node) => node.recompute_content_digest(),
            Self::RiskAcceptance(node) => node.recompute_content_digest(),
        };
        recomputed.is_ok_and(|digest| &digest == self.content_digest())
    }
}

/// Relationship semantics between exact graph identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyKind {
    /// Exact subject or packet binding.
    Binding,
    /// Named authority dependency.
    Authority,
    /// Evidence dependency.
    Evidence,
    /// A change must invalidate downstream nodes.
    FreshnessImpact,
    /// Explicit historical succession.
    Supersedes,
}

impl DependencyKind {
    /// Returns the frozen relationship wire label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Binding => "binding",
            Self::Authority => "authority",
            Self::Evidence => "evidence",
            Self::FreshnessImpact => "freshness_impact",
            Self::Supersedes => "supersedes",
        }
    }
}

/// Directed typed dependency edge.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source node.
    pub source: NodeId,
    /// Dependent target node.
    pub target: NodeId,
    /// Exact relationship semantics.
    pub kind: DependencyKind,
}

/// Internal mutation journal used to reconstruct freshness and supersession exactly.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub(crate) enum DecisionMemoryEvent {
    /// One exact bundle was admitted to repository decision memory.
    BundleAdmission {
        /// Stable bundle identity.
        bundle_id: String,
        /// Exact decision-memory revision.
        decision_memory_revision: u64,
    },
    /// One deterministic validation result was recorded.
    Validation {
        /// Stable bundle identity.
        bundle_id: String,
        /// Exact decision-memory revision.
        decision_memory_revision: u64,
    },
    /// One exact digest change propagated through typed dependency edges.
    StalePropagation {
        /// Changed node identity.
        changed_node: NodeId,
        /// Newly observed content digest.
        observed_digest: String,
        /// Stable propagation reason.
        reason: String,
    },
    /// One exact historical decision superseded another.
    Supersession {
        /// Earlier decision identity.
        previous: NodeId,
        /// Later decision identity.
        successor: NodeId,
        /// Stable supersession reason.
        reason: String,
    },
    /// One exact Boundline terminal outcome was admitted.
    OutcomeRecorded {
        /// Complete typed event and assigned revision.
        outcome: Box<PublicationOutcomeEvent>,
    },
}

impl DependencyEdge {
    /// Creates a typed directed edge.
    pub fn new(source: NodeId, target: NodeId, kind: DependencyKind) -> Self {
        Self { source, target, kind }
    }

    pub(crate) const fn impacts_freshness(&self) -> bool {
        matches!(
            self.kind,
            DependencyKind::FreshnessImpact | DependencyKind::Authority | DependencyKind::Evidence
        )
    }
}

/// Idempotent insertion result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsertOutcome {
    /// A new exact node was recorded.
    Inserted,
    /// The same ID and digest was already recorded.
    Replayed,
}

/// One terminal Boundline outcome retained in the ordered decision-memory journal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationOutcomeEvent {
    /// Complete frozen public request that was admitted.
    pub request: RecordOutcomeRequest,
    /// Monotonic decision-memory revision assigned to this event.
    pub decision_memory_revision: u64,
    /// Complete ordered deterministic validation trace.
    pub validation_phases: Vec<super::OutcomeValidationPhase>,
    /// Proof that Canon created no semantic or provider side effects.
    pub execution_audit: super::ExecutionAuditCounters,
}

/// Repository-local typed decision-memory graph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionMemoryGraph {
    /// Internal graph schema identity.
    pub schema_version: String,
    /// Public contract line associated with this graph.
    pub contract_line: String,
    /// Monotonic graph revision.
    pub revision: u64,
    nodes: BTreeMap<NodeId, DecisionMemoryNode>,
    edges: BTreeSet<DependencyEdge>,
    events: Vec<DecisionMemoryEvent>,
}

impl DecisionMemoryGraph {
    /// Creates an empty graph for the frozen Canon contract line.
    pub fn new() -> Self {
        Self {
            schema_version: DECISION_MEMORY_SCHEMA_VERSION.to_string(),
            contract_line: DECISION_MEMORY_CONTRACT_LINE.to_string(),
            revision: 0,
            nodes: BTreeMap::new(),
            edges: BTreeSet::new(),
            events: Vec::new(),
        }
    }

    /// Inserts a node once or replays an identical recorded node.
    pub fn insert_or_replay(
        &mut self,
        mut node: DecisionMemoryNode,
    ) -> Result<InsertOutcome, DecisionMemoryError> {
        node.validate_content()?;
        if let Some(recorded) = self.nodes.get(node.id()) {
            return if recorded.content_digest() == node.content_digest() {
                Ok(InsertOutcome::Replayed)
            } else {
                Err(DecisionMemoryError::ContentConflict { node_id: node.id().clone() })
            };
        }
        self.revision = self.revision.saturating_add(1);
        *node.introduced_mut() = self.revision;
        self.nodes.insert(node.id().clone(), node);
        Ok(InsertOutcome::Inserted)
    }

    /// Adds a typed edge after reference and cycle validation.
    pub fn add_edge(&mut self, edge: DependencyEdge) -> Result<(), DecisionMemoryError> {
        self.require_node(&edge.source)?;
        self.require_node(&edge.target)?;
        if self.edges.contains(&edge) {
            return Ok(());
        }
        if edge.impacts_freshness() && self.reaches(&edge.target, &edge.source, true) {
            return Err(DecisionMemoryError::DependencyCycle { node_id: edge.source });
        }
        self.edges.insert(edge);
        self.revision = self.revision.saturating_add(1);
        Ok(())
    }

    /// Removes a node while preserving edges for fail-closed corruption tests.
    pub fn remove_node(&mut self, node_id: &NodeId) -> Result<(), DecisionMemoryError> {
        self.require_node(node_id)?;
        self.nodes.remove(node_id);
        self.revision = self.revision.saturating_add(1);
        Ok(())
    }

    /// Records explicit supersession while retaining both historical nodes.
    pub fn supersede(
        &mut self,
        previous: &NodeId,
        successor: &NodeId,
        reason: impl Into<String>,
    ) -> Result<(), DecisionMemoryError> {
        self.require_node(previous)?;
        self.require_node(successor)?;
        let reason = reason.into();
        require_safe_projection_text(&reason, "supersession reason")?;
        let next = self
            .nodes
            .get_mut(successor)
            .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: successor.clone() })?;
        if !next.supersedes_mut().contains(previous) {
            next.supersedes_mut().push(previous.clone());
            next.supersedes_mut().sort();
        }
        let successor_digest = next.content_digest().sha256.clone();
        let old = self
            .nodes
            .get_mut(previous)
            .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: previous.clone() })?;
        *old.freshness_mut() = FreshnessState::Stale {
            observed_digest: successor_digest,
            reason: reason.clone(),
            reason_chain: vec![previous.clone(), successor.clone()],
        };
        self.edges.insert(DependencyEdge::new(
            successor.clone(),
            previous.clone(),
            DependencyKind::Supersedes,
        ));
        self.revision = self.revision.saturating_add(1);
        self.events.push(DecisionMemoryEvent::Supersession {
            previous: previous.clone(),
            successor: successor.clone(),
            reason,
        });
        Ok(())
    }

    /// Returns one node by exact identity.
    pub fn node(&self, node_id: &NodeId) -> Result<&DecisionMemoryNode, DecisionMemoryError> {
        self.nodes
            .get(node_id)
            .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: node_id.clone() })
    }

    /// Iterates nodes in stable identity order.
    pub fn nodes(&self) -> impl Iterator<Item = &DecisionMemoryNode> {
        self.nodes.values()
    }

    /// Iterates edges in stable typed order.
    pub fn edges(&self) -> impl Iterator<Item = &DependencyEdge> {
        self.edges.iter()
    }

    /// Returns exact node freshness.
    pub fn freshness(&self, node_id: &NodeId) -> Result<&FreshnessState, DecisionMemoryError> {
        Ok(self.node(node_id)?.freshness())
    }

    /// Computes the deterministic graph digest.
    pub fn digest(&self) -> Result<ContentDigest, DecisionMemoryError> {
        ContentDigest::compute("decision-memory-graph", self)
    }

    /// Returns terminal publication outcomes in journal order.
    pub fn publication_outcomes(&self) -> impl Iterator<Item = &PublicationOutcomeEvent> {
        self.events.iter().filter_map(|event| match event {
            DecisionMemoryEvent::OutcomeRecorded { outcome } => Some(outcome.as_ref()),
            DecisionMemoryEvent::BundleAdmission { .. }
            | DecisionMemoryEvent::Validation { .. }
            | DecisionMemoryEvent::StalePropagation { .. }
            | DecisionMemoryEvent::Supersession { .. } => None,
        })
    }

    pub(crate) fn record_publication_outcome(
        &mut self,
        request: RecordOutcomeRequest,
        validation_phases: Vec<super::OutcomeValidationPhase>,
        execution_audit: super::ExecutionAuditCounters,
    ) -> PublicationOutcomeEvent {
        self.revision = self.revision.saturating_add(1);
        let outcome = PublicationOutcomeEvent {
            request,
            decision_memory_revision: self.revision,
            validation_phases,
            execution_audit,
        };
        self.events
            .push(DecisionMemoryEvent::OutcomeRecorded { outcome: Box::new(outcome.clone()) });
        outcome
    }

    pub(crate) fn node_mut(
        &mut self,
        node_id: &NodeId,
    ) -> Result<&mut DecisionMemoryNode, DecisionMemoryError> {
        self.nodes
            .get_mut(node_id)
            .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: node_id.clone() })
    }

    pub(crate) fn dependents(&self, source: &NodeId) -> Vec<NodeId> {
        self.edges
            .iter()
            .filter(|edge| edge.source == *source && edge.impacts_freshness())
            .map(|edge| edge.target.clone())
            .collect()
    }

    pub(crate) fn dangling_edges(&self) -> Vec<DependencyEdge> {
        self.edges
            .iter()
            .filter(|edge| {
                !self.nodes.contains_key(&edge.source) || !self.nodes.contains_key(&edge.target)
            })
            .cloned()
            .collect()
    }

    pub(crate) fn has_freshness_cycle(&self) -> bool {
        self.nodes.keys().any(|node| {
            let mut visiting = BTreeSet::new();
            let mut visited = BTreeSet::new();
            self.cycle_from(node, &mut visiting, &mut visited)
        })
    }

    pub(crate) fn has_invalid_node_digest(&self) -> bool {
        self.nodes.values().any(|node| !node.recorded_digest_is_valid())
    }

    pub(crate) fn has_invalid_declared_freshness(&self) -> bool {
        self.nodes.values().any(|node| match node {
            DecisionMemoryNode::Evidence(value) => {
                !value.content.fresh && value.freshness == FreshnessState::Fresh
            }
            DecisionMemoryNode::Approval(value) => {
                !value.content.fresh && value.freshness == FreshnessState::Fresh
            }
            DecisionMemoryNode::RiskAcceptance(value) => {
                !value.content.fresh && value.freshness == FreshnessState::Fresh
            }
            _ => false,
        })
    }

    pub(crate) fn events(&self) -> &[DecisionMemoryEvent] {
        &self.events
    }

    pub(crate) fn record_stale_event(
        &mut self,
        changed_node: NodeId,
        observed_digest: String,
        reason: String,
    ) {
        self.events.push(DecisionMemoryEvent::StalePropagation {
            changed_node,
            observed_digest,
            reason,
        });
    }

    pub(crate) fn record_bundle_admission_event(
        &mut self,
        bundle_id: String,
        decision_memory_revision: u64,
    ) {
        self.events
            .push(DecisionMemoryEvent::BundleAdmission { bundle_id, decision_memory_revision });
    }

    pub(crate) fn record_validation_event(
        &mut self,
        bundle_id: String,
        decision_memory_revision: u64,
    ) {
        self.events.push(DecisionMemoryEvent::Validation { bundle_id, decision_memory_revision });
    }

    fn require_node(&self, node_id: &NodeId) -> Result<(), DecisionMemoryError> {
        if self.nodes.contains_key(node_id) {
            Ok(())
        } else {
            Err(DecisionMemoryError::NodeNotFound { node_id: node_id.clone() })
        }
    }

    fn reaches(&self, start: &NodeId, target: &NodeId, freshness_only: bool) -> bool {
        let mut pending = vec![start.clone()];
        let mut visited = BTreeSet::new();
        while let Some(current) = pending.pop() {
            if &current == target {
                return true;
            }
            if !visited.insert(current.clone()) {
                continue;
            }
            pending.extend(
                self.edges
                    .iter()
                    .filter(|edge| {
                        edge.source == current && (!freshness_only || edge.impacts_freshness())
                    })
                    .map(|edge| edge.target.clone()),
            );
        }
        false
    }

    fn cycle_from(
        &self,
        current: &NodeId,
        visiting: &mut BTreeSet<NodeId>,
        visited: &mut BTreeSet<NodeId>,
    ) -> bool {
        if visiting.contains(current) {
            return true;
        }
        if !visited.insert(current.clone()) {
            return false;
        }
        visiting.insert(current.clone());
        let cycle = self
            .edges
            .iter()
            .filter(|edge| edge.source == *current && edge.impacts_freshness())
            .any(|edge| self.cycle_from(&edge.target, visiting, visited));
        visiting.remove(current);
        cycle
    }
}

impl Default for DecisionMemoryGraph {
    fn default() -> Self {
        Self::new()
    }
}

fn require_identity(node_id: &NodeId) -> Result<(), DecisionMemoryError> {
    if node_id.as_str().trim().is_empty() {
        Err(DecisionMemoryError::invalid_structure("node identity must not be blank"))
    } else {
        Ok(())
    }
}

pub(crate) fn require_safe_projection_text(
    value: &str,
    field: &str,
) -> Result<(), DecisionMemoryError> {
    let lowered = value.to_ascii_lowercase();
    let windows_absolute = value.as_bytes().get(1) == Some(&b':')
        && value.as_bytes().get(2).is_some_and(|byte| matches!(byte, b'/' | b'\\'));
    let sensitive =
        ["token=", "api_key", "secret=", "password="].iter().any(|marker| lowered.contains(marker));
    if value.starts_with('/') || windows_absolute || sensitive || value.contains('@') {
        Err(DecisionMemoryError::invalid_structure(format!(
            "{field} contains a local path, secret, or personal identifier"
        )))
    } else {
        Ok(())
    }
}
