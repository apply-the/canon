//! External evidence and approval contracts keep semantic review outside Canon.

use serde::{Deserialize, Serialize};

use crate::{Claim, EvidenceReference, Finding, Revision};

/// Independence tier declared for external verification evidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChallengeTier {
    /// Deterministic checks only.
    #[serde(rename = "tier_0")]
    Tier0,
    /// Separate verification invocation with fresh claim-matched evidence.
    #[serde(rename = "tier_1")]
    Tier1,
    /// Distinct lineage and independently constructed verification context.
    #[serde(rename = "tier_2")]
    Tier2,
    /// Different provider family or qualified human challenge with approval.
    #[serde(rename = "tier_3")]
    Tier3,
}

/// Verification categories Canon can deterministically require.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationKind {
    /// A deterministic rule or command result.
    Deterministic,
    /// Evidence produced by a semantic reviewer external to Canon.
    ExternalSemanticReview,
}

/// Recorded outcome of a deterministic verification check.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterministicVerificationStatus {
    /// The declared deterministic check passed.
    Passed,
    /// The declared deterministic check failed.
    Failed,
}

/// Evidence produced by a named deterministic check or policy rule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicVerificationEvidence {
    /// Stable identity of the command, check, or policy rule.
    pub check_identity: String,
    /// Claims evaluated by the deterministic check.
    pub claims: Vec<Claim>,
    /// Recorded check outcome.
    pub status: DeterministicVerificationStatus,
    /// Immutable evidence references supporting the outcome.
    pub evidence_references: Vec<EvidenceReference>,
}

/// Deterministic policy requirement for evidence supplied by another actor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationRequirement {
    /// Required verification category.
    pub kind: VerificationKind,
    /// Claims the submitted evidence must address.
    pub claims: Vec<Claim>,
    /// Minimum independence tier required by policy.
    pub minimum_challenge_tier: ChallengeTier,
}

/// External semantic-review evidence that Canon may validate but never produce.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalVerificationEvidence {
    /// Executor or human reviewer identity.
    pub reviewer_identity: String,
    /// Provider, executor, and invocation lineage.
    pub lineage: String,
    /// Identity of the independently constructed review context.
    pub independent_context_identity: String,
    /// Claims reviewed.
    pub claims: Vec<Claim>,
    /// Findings produced by the external reviewer.
    pub findings: Vec<Finding>,
    /// Immutable evidence references supporting the findings.
    pub evidence_references: Vec<EvidenceReference>,
    /// Independence tier satisfied by the review.
    pub challenge_tier: ChallengeTier,
    /// Named, justified override when policy explicitly permits degradation.
    pub named_override: Option<String>,
}

/// Stable approval outcome; unknown values fail deserialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    /// Named authority approved the governed claims.
    Approved,
    /// Named authority rejected the governed claims.
    Rejected,
}

/// Approval projection bound to a decision-memory revision and claim set.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Approval {
    /// Identity of the approving or rejecting authority.
    pub approver_identity: String,
    /// Recorded authority decision.
    pub decision: ApprovalDecision,
    /// Exact decision-memory revision reviewed.
    pub decision_memory_revision: Revision,
    /// Claims covered by the decision.
    pub claims: Vec<Claim>,
}
