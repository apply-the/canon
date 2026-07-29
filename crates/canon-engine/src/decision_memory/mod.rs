//! Deterministic governance validation and repository-local decision memory.
//!
//! This module records typed governance relationships and validates exact
//! bindings. It deliberately contains no provider, model, network, subprocess,
//! or semantic-review execution capability.

mod bundle;
mod digest;
mod error;
mod freshness;
mod graph;
mod projection;
mod store;
mod topology;
mod validation;

pub use bundle::{
    ApprovalContent, BoundGovernanceBundle, EvidenceContent, GovernanceBundleDraft,
    GovernanceMetadata, GovernancePacketDraft, RiskAcceptanceContent, SubjectArtifactBinding,
    VerificationRequirementContent, build_governance_bundle,
};
pub use digest::{ContentDigest, GRAPH_DIGEST_SCHEMA_VERSION};
pub use error::DecisionMemoryError;
pub use freshness::{StalePropagation, propagate_stale};
pub use graph::{
    AlternativeContent, ArtifactContent, AssumptionContent, ClaimContent, DecisionContent,
    DecisionMemoryGraph, DecisionMemoryNode, DependencyEdge, DependencyKind, FreshnessState,
    InsertOutcome, NodeEnvelope, NodeId, PacketContent, TriggerContent,
};
pub use projection::project_decision_memory;
pub use store::{DecisionMemoryStore, DecisionMemoryStoreSnapshot};
pub use validation::{
    ExecutionAuditCounters, GovernanceDecision, GovernanceValidationResult, ValidationCode,
    ValidationFinding, ValidationPhase, validate_governance_bundle,
};
