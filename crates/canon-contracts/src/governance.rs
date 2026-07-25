//! Typed packets and bundles carry deterministic governance intent and policy.

use serde::{Deserialize, Serialize};

use crate::{
    AcceptanceCriterion, BundleDigest, BundleId, CanonContractVersion, PacketId, Profile, Revision,
    ScopeItem, VerificationRequirement,
};

/// Named authority required before a governance bundle can be admitted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRequirement {
    /// Stable identities of required approvers.
    pub required_approvers: Vec<String>,
}

/// Typed governance packet for one stable profile.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernancePacket {
    /// Stable packet identity.
    pub packet_id: PacketId,
    /// Stable governance profile.
    pub profile: Profile,
    /// Change intent governed by the packet.
    pub change_intent: String,
    /// Admitted scope.
    pub scope: Vec<ScopeItem>,
    /// Declared risks.
    pub risks: Vec<String>,
    /// Invariants that admitted work must preserve.
    pub invariants: Vec<String>,
    /// Deterministically inspectable acceptance criteria.
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    /// References to other packets needed for cross-packet validation.
    pub cross_packet_references: Vec<PacketId>,
}

/// Deterministic authorization bundle published for Boundline consumption.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceBundle {
    /// Schema version used to encode the bundle.
    pub contract_version: CanonContractVersion,
    /// Stable bundle identity.
    pub bundle_id: BundleId,
    /// Canonical bundle digest.
    pub bundle_digest: BundleDigest,
    /// Primary stable governance profile.
    pub profile: Profile,
    /// Authored packets admitted into the bundle.
    pub packets: Vec<GovernancePacket>,
    /// Required named authority.
    pub authority: AuthorityRequirement,
    /// Deterministic and external-evidence requirements.
    pub required_evidence: Vec<VerificationRequirement>,
    /// Decision-memory revision from which the bundle was projected.
    pub decision_memory_revision: Revision,
}
