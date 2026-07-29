//! Stable deterministic governance contracts for Canon.
//!
//! The crate defines governance exchange data only. It contains no model
//! routing, semantic-review executor, code execution, workspace mutation, or
//! persistence implementation.

mod decision_memory;
mod evidence;
mod governance;
mod identifiers;
mod outcome;
mod profile;
mod rpc;

pub use decision_memory::{
    DecisionMemoryNode, DecisionMemoryProjection, GovernancePublication, PublicationProjectionKind,
};
pub use evidence::{
    Approval, ApprovalDecision, ChallengeTier, DeterministicVerificationEvidence,
    DeterministicVerificationStatus, ExternalVerificationEvidence, VerificationKind,
    VerificationRequirement,
};
pub use governance::{AuthorityRequirement, GovernanceBundle, GovernancePacket};
pub use identifiers::{
    AcceptanceCriterion, BundleDigest, BundleId, Claim, EvidenceReference, Finding, PacketId,
    Revision, ScopeItem,
};
pub use outcome::{
    AuthoritativeTimestamp, CommitIdentity, DecisionMemoryDigest, Deviation, FinalFingerprint,
    OUTCOME_CANONICALIZATION_DOMAIN, OutcomeApprovalBinding, OutcomeAuthorityBinding,
    OutcomeChallengeBinding, OutcomeEventDigest, OutcomeEventId, OutcomeLineage, OutcomeNextAction,
    OutcomeSessionId, OutcomeSourceProduct, RecordOutcomeContractError, RecordOutcomeDisposition,
    RecordOutcomeRejectionReason, RecordOutcomeRequest, RecordOutcomeResponse, RepositoryIdentity,
    TerminalOutcomeStatus,
};
pub use profile::{Profile, StableProfileRegistry};
pub use rpc::{CanonContractVersion, OneShotOperation, OneShotRequest, OneShotResponse};
