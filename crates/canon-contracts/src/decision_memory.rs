//! Decision-memory publication exposes projections, never workspace mutation.

use serde::{Deserialize, Serialize};

use crate::{Approval, CanonContractVersion, EvidenceReference, GovernanceBundle, Revision};

/// Public decision-memory node with explicit freshness and supersession.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionMemoryNode {
    /// Stable node identity.
    pub node_id: String,
    /// Stable node kind.
    pub kind: String,
    /// Source packet, evidence, approval, or outcome reference.
    pub source_reference: String,
    /// Revision at which this node was introduced.
    pub revision_introduced: Revision,
    /// Related decision-memory node or governance identities.
    pub relationships: Vec<String>,
    /// Nodes explicitly superseded by this node.
    pub supersedes: Vec<String>,
    /// Whether the node remains fresh for its source state.
    pub fresh: bool,
    /// Stable explanation when the node is stale.
    pub stale_reason: Option<String>,
}

/// Read-only decision-memory graph projection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionMemoryProjection {
    /// Exact projected decision-memory revision.
    pub revision: Revision,
    /// Nodes visible at the projected revision.
    pub nodes: Vec<DecisionMemoryNode>,
}

/// Projection families Canon may publish through its stable surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationProjectionKind {
    /// Typed governance bundle projection.
    GovernanceBundle,
    /// Read-only decision-memory projection.
    DecisionMemory,
    /// Evidence and verification projection.
    Evidence,
}

/// Canon publication containing governance projections only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernancePublication {
    /// Schema version used to encode the publication.
    pub contract_version: CanonContractVersion,
    /// Typed governance bundle being published.
    pub governance_bundle: GovernanceBundle,
    /// Projection families included in the publication.
    pub projections: Vec<PublicationProjectionKind>,
    /// Read-only decision-memory projection.
    pub decision_memory: DecisionMemoryProjection,
    /// Immutable evidence references projected with the bundle.
    pub evidence: Vec<EvidenceReference>,
    /// Authority decisions projected with the bundle.
    pub approvals: Vec<Approval>,
}
