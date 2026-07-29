//! Terminal outcome DTOs provide a public, deterministic Boundline-to-Canon boundary.

use std::fmt::{Display, Formatter};

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};

mod digest;
mod validation;

use crate::{
    ApprovalDecision, BundleDigest, BundleId, ChallengeTier, Claim, EvidenceReference, Revision,
};

/// Domain separator for Canon 0.91 terminal outcome digests.
pub const OUTCOME_CANONICALIZATION_DOMAIN: &str = "canon-boundline-outcome-c14n-v1";

const SHA256_PREFIX: &str = "sha256:";
const SHA256_HEX_LENGTH: usize = 64;
const DOMAIN_SEPARATOR: u8 = 0;
const PLACEHOLDER_DIGEST: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
const PRIVATE_VALUE_MARKERS: [&str; 6] =
    ["provider_token:", "api_key:", "secret:", "password:", "raw_prompt:", "private_conversation:"];

macro_rules! outcome_string_identifier {
    ($(#[$metadata:meta])* $name:ident) => {
        $(#[$metadata])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a typed value from its stable wire representation.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the stable wire representation.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

outcome_string_identifier!(
    /// Idempotency identity of one Boundline terminal outcome.
    OutcomeEventId
);
outcome_string_identifier!(
    /// Clone-local source repository identity supplied by Boundline.
    RepositoryIdentity
);
outcome_string_identifier!(
    /// Boundline governed-session identity.
    OutcomeSessionId
);
outcome_string_identifier!(
    /// Verified Git commit produced by publication.
    CommitIdentity
);
outcome_string_identifier!(
    /// Final authoritative workspace fingerprint.
    FinalFingerprint
);
outcome_string_identifier!(
    /// Declared deviation retained with the terminal outcome.
    Deviation
);
outcome_string_identifier!(
    /// Canon decision-memory digest returned after recording.
    DecisionMemoryDigest
);
outcome_string_identifier!(
    /// Authoritative event time supplied by the source transaction.
    AuthoritativeTimestamp
);

/// Versioned SHA-256 digest of one terminal outcome request.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OutcomeEventDigest(String);

impl OutcomeEventDigest {
    /// Creates a digest from its stable wire representation.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Provides a construction sentinel that must be recomputed before exchange.
    pub fn placeholder() -> Self {
        Self(PLACEHOLDER_DIGEST.to_owned())
    }

    /// Returns the stable wire representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Only Boundline may originate the 0.91 terminal outcome request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeSourceProduct {
    /// Boundline delivery control plane.
    Boundline,
}

/// Candidate terminal statuses recognized by the outcome contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalOutcomeStatus {
    /// Publication created and admitted a verified Git commit.
    Published,
    /// Verification established that no authoritative change was needed.
    NoChange,
    /// Governed execution reached a terminal failure.
    Failed,
    /// Named authority explicitly cancelled the governed session.
    Cancelled,
    /// Work is blocked and remains nonterminal.
    Blocked,
    /// A later revision invalidated the candidate, which remains nonterminal.
    Stale,
    /// Named authority or deterministic policy terminally rejected the outcome.
    Rejected,
}

impl TerminalOutcomeStatus {
    const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Published | Self::NoChange | Self::Failed | Self::Cancelled | Self::Rejected
        )
    }
}

/// Authority binding supplied by Boundline without granting authority.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeAuthorityBinding {
    /// Stable identity of the authority responsible for the outcome.
    pub authority_identity: String,
    /// Exact Boundline transaction revision governed by the authority.
    pub final_transaction_revision: Revision,
    /// Exact claims covered by the authority binding.
    pub claims: Vec<Claim>,
}

/// Approval binding supplied when policy requires explicit approval.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeApprovalBinding {
    /// Stable identity of the named approver.
    pub approver_identity: String,
    /// Recorded approval decision.
    pub decision: ApprovalDecision,
    /// Exact Boundline transaction revision reviewed.
    pub final_transaction_revision: Revision,
    /// Exact claims covered by the approval.
    pub claims: Vec<Claim>,
}

/// Challenge evidence binding supplied by an external verifier.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeChallengeBinding {
    /// Independence tier satisfied by the challenge.
    pub tier: ChallengeTier,
    /// Stable challenger identity when the tier requires a separate verifier.
    pub challenger_identity: Option<String>,
    /// Stable challenger invocation identity.
    pub challenger_invocation_id: Option<String>,
    /// Identity of the independently constructed verification context.
    pub independent_context_identity: Option<String>,
    /// Claims evaluated by the challenge.
    pub claims: Vec<Claim>,
    /// Immutable evidence supporting the challenge.
    pub evidence_references: Vec<EvidenceReference>,
    /// Named degradation override when policy explicitly permits one.
    pub named_override: Option<String>,
}

/// Producer and verifier lineage prevents self-attested independence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeLineage {
    /// Stable identity of the outcome producer.
    pub producer_identity: String,
    /// Stable invocation identity of the outcome producer.
    pub producer_invocation_id: String,
    /// Stable identity of a separate verifier when one is required.
    pub verifier_identity: Option<String>,
    /// Stable invocation identity of the separate verifier.
    pub verifier_invocation_id: Option<String>,
}

/// Canon disposition for a terminal outcome request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordOutcomeDisposition {
    /// Canon durably created exactly one outcome event.
    Recorded,
    /// Canon returned the durable response for an exact prior event.
    Replayed,
    /// Canon created no outcome event.
    Rejected,
}

/// Stable rejection reasons for the outcome recording boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordOutcomeRejectionReason {
    /// The request used an unsupported contract line.
    UnsupportedContractLine,
    /// The request violated a terminal outcome invariant.
    InvalidOutcome,
    /// The supplied status is not terminal.
    NonterminalOutcome,
    /// An existing event identity has different canonical content.
    IdentityDigestConflict,
    /// The authority binding is invalid or incomplete.
    AuthorityBindingInvalid,
    /// The required approval binding is invalid or incomplete.
    ApprovalBindingInvalid,
    /// Required proof or challenge evidence is invalid.
    EvidenceBindingInvalid,
    /// Producer and verifier lineage is invalid.
    LineageInvalid,
    /// A later state invalidated the supplied outcome.
    StaleOutcome,
    /// The decision-memory base cannot admit the event.
    DecisionMemoryConflict,
    /// Canon could not durably persist the event.
    PersistenceFailed,
    /// The runtime has not installed the requested operation.
    UnsupportedOperation,
}

impl RecordOutcomeRejectionReason {
    /// Returns the exact stable wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedContractLine => "unsupported_contract_line",
            Self::InvalidOutcome => "invalid_outcome",
            Self::NonterminalOutcome => "nonterminal_outcome",
            Self::IdentityDigestConflict => "identity_digest_conflict",
            Self::AuthorityBindingInvalid => "authority_binding_invalid",
            Self::ApprovalBindingInvalid => "approval_binding_invalid",
            Self::EvidenceBindingInvalid => "evidence_binding_invalid",
            Self::LineageInvalid => "lineage_invalid",
            Self::StaleOutcome => "stale_outcome",
            Self::DecisionMemoryConflict => "decision_memory_conflict",
            Self::PersistenceFailed => "persistence_failed",
            Self::UnsupportedOperation => "unsupported_operation",
        }
    }
}

/// Typed next actions returned after an outcome rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeNextAction {
    /// Retry the same canonical request when the transient condition clears.
    Retry,
    /// Upgrade to a contract line supporting outcome recording.
    UpgradeContract,
    /// Reverify the outcome against current authoritative state.
    ReverifyOutcome,
    /// Resolve an identity/digest conflict without overwriting the event.
    ResolveIdentityConflict,
    /// Obtain the named authority required by policy.
    ObtainAuthority,
    /// Repair missing or invalid evidence bindings.
    RepairEvidence,
    /// Refresh governance before re-admission.
    RefreshGovernance,
    /// Restore durable persistence before retry.
    RestorePersistence,
    /// Inspect the current decision-memory projection.
    InspectDecisionMemory,
}

/// Public request for recording one terminal Boundline outcome.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RecordOutcomeRequest {
    /// Idempotency identity of the outcome.
    pub event_id: OutcomeEventId,
    /// Domain-separated canonical digest of every authoritative field.
    pub event_digest: OutcomeEventDigest,
    /// Explicit source product.
    pub source_product: OutcomeSourceProduct,
    /// Clone-local source repository identity.
    pub source_repository_identity: RepositoryIdentity,
    /// Governance bundle authorizing the governed session.
    pub governance_bundle_id: BundleId,
    /// Canonical digest of the governance bundle.
    pub governance_bundle_digest: BundleDigest,
    /// Boundline governed-session identity.
    pub session_id: OutcomeSessionId,
    /// Final Boundline transaction revision.
    pub final_transaction_revision: Revision,
    /// Closed candidate terminal status.
    pub terminal_status: TerminalOutcomeStatus,
    /// Verified published commit, only for `published`.
    pub published_commit: Option<CommitIdentity>,
    /// Final authoritative fingerprint when the outcome has one.
    pub final_fingerprint: Option<FinalFingerprint>,
    /// Immutable fresh proof references.
    pub proof_references: Vec<EvidenceReference>,
    /// Declared deviations retained with the outcome.
    pub deviations: Vec<Deviation>,
    /// Exact terminal claims.
    pub terminal_claims: Vec<Claim>,
    /// Named authority binding.
    pub authority_binding: OutcomeAuthorityBinding,
    /// Explicit approval binding when required by challenge policy.
    pub approval_binding: Option<OutcomeApprovalBinding>,
    /// Challenge tier, identity, claims, and evidence binding.
    pub challenge_binding: OutcomeChallengeBinding,
    /// Producer and verifier lineage.
    pub lineage: OutcomeLineage,
    /// Authoritative source timestamp when Boundline records one.
    pub occurred_at: Option<AuthoritativeTimestamp>,
}

#[derive(Deserialize)]
struct RecordOutcomeRequestWire {
    event_id: OutcomeEventId,
    event_digest: OutcomeEventDigest,
    source_product: OutcomeSourceProduct,
    source_repository_identity: RepositoryIdentity,
    governance_bundle_id: BundleId,
    governance_bundle_digest: BundleDigest,
    session_id: OutcomeSessionId,
    final_transaction_revision: Revision,
    terminal_status: TerminalOutcomeStatus,
    published_commit: Option<CommitIdentity>,
    final_fingerprint: Option<FinalFingerprint>,
    proof_references: Vec<EvidenceReference>,
    deviations: Vec<Deviation>,
    terminal_claims: Vec<Claim>,
    authority_binding: OutcomeAuthorityBinding,
    approval_binding: Option<OutcomeApprovalBinding>,
    challenge_binding: OutcomeChallengeBinding,
    lineage: OutcomeLineage,
    occurred_at: Option<AuthoritativeTimestamp>,
}

impl From<RecordOutcomeRequestWire> for RecordOutcomeRequest {
    fn from(wire: RecordOutcomeRequestWire) -> Self {
        Self {
            event_id: wire.event_id,
            event_digest: wire.event_digest,
            source_product: wire.source_product,
            source_repository_identity: wire.source_repository_identity,
            governance_bundle_id: wire.governance_bundle_id,
            governance_bundle_digest: wire.governance_bundle_digest,
            session_id: wire.session_id,
            final_transaction_revision: wire.final_transaction_revision,
            terminal_status: wire.terminal_status,
            published_commit: wire.published_commit,
            final_fingerprint: wire.final_fingerprint,
            proof_references: wire.proof_references,
            deviations: wire.deviations,
            terminal_claims: wire.terminal_claims,
            authority_binding: wire.authority_binding,
            approval_binding: wire.approval_binding,
            challenge_binding: wire.challenge_binding,
            lineage: wire.lineage,
            occurred_at: wire.occurred_at,
        }
    }
}

impl<'de> Deserialize<'de> for RecordOutcomeRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let request = Self::from(RecordOutcomeRequestWire::deserialize(deserializer)?);
        request.validate().map_err(D::Error::custom)?;
        Ok(request)
    }
}

impl RecordOutcomeRequest {
    /// Recomputes the canonical digest after authored fields change.
    pub fn recompute_event_digest(&mut self) -> Result<(), RecordOutcomeContractError> {
        self.event_digest = self.canonical_event_digest()?;
        Ok(())
    }

    /// Computes the domain-separated digest without trusting the supplied digest.
    pub fn canonical_event_digest(&self) -> Result<OutcomeEventDigest, RecordOutcomeContractError> {
        let canonical = digest::canonical_digest_input(self)?;
        let mut hasher = Sha256::new();
        hasher.update(OUTCOME_CANONICALIZATION_DOMAIN.as_bytes());
        hasher.update([DOMAIN_SEPARATOR]);
        hasher.update(canonical.as_bytes());
        let bytes = hasher.finalize();
        let mut encoded = String::with_capacity(SHA256_PREFIX.len() + SHA256_HEX_LENGTH);
        encoded.push_str(SHA256_PREFIX);
        for byte in bytes {
            use std::fmt::Write as _;
            write!(&mut encoded, "{byte:02x}").map_err(|_| {
                RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome)
            })?;
        }
        Ok(OutcomeEventDigest::new(encoded))
    }

    /// Validates all closed request invariants and the supplied digest.
    pub fn validate(&self) -> Result<(), RecordOutcomeContractError> {
        validation::validate_identity(self)?;
        validation::validate_terminal_status(self)?;
        validation::validate_set_bindings(self)?;
        validation::validate_authority(self)?;
        validation::validate_challenge_and_lineage(self)?;
        validation::validate_portability(self)?;
        validation::validate_digest_shape(&self.event_digest)?;
        if self.canonical_event_digest()? != self.event_digest {
            return Err(RecordOutcomeContractError::new(
                RecordOutcomeRejectionReason::InvalidOutcome,
            ));
        }
        Ok(())
    }
}

/// Public response for one terminal outcome recording attempt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RecordOutcomeResponse {
    /// Outcome event identity copied from the request.
    pub event_id: OutcomeEventId,
    /// Canonical outcome digest copied from the request.
    pub event_digest: OutcomeEventDigest,
    /// Whether Canon recorded, replayed, or rejected the event.
    pub disposition: RecordOutcomeDisposition,
    /// Durable decision-memory revision for recorded and replayed events.
    pub decision_memory_revision: Option<Revision>,
    /// Durable decision-memory digest for recorded and replayed events.
    pub decision_memory_digest: Option<DecisionMemoryDigest>,
    /// Typed rejection reason when no event was created.
    pub reason_code: Option<RecordOutcomeRejectionReason>,
    /// Typed actions that may resolve a rejection.
    pub next_actions: Vec<OutcomeNextAction>,
}

#[derive(Deserialize)]
struct RecordOutcomeResponseWire {
    event_id: OutcomeEventId,
    event_digest: OutcomeEventDigest,
    disposition: RecordOutcomeDisposition,
    decision_memory_revision: Option<Revision>,
    decision_memory_digest: Option<DecisionMemoryDigest>,
    reason_code: Option<RecordOutcomeRejectionReason>,
    next_actions: Vec<OutcomeNextAction>,
}

impl From<RecordOutcomeResponseWire> for RecordOutcomeResponse {
    fn from(wire: RecordOutcomeResponseWire) -> Self {
        Self {
            event_id: wire.event_id,
            event_digest: wire.event_digest,
            disposition: wire.disposition,
            decision_memory_revision: wire.decision_memory_revision,
            decision_memory_digest: wire.decision_memory_digest,
            reason_code: wire.reason_code,
            next_actions: wire.next_actions,
        }
    }
}

impl<'de> Deserialize<'de> for RecordOutcomeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let response = Self::from(RecordOutcomeResponseWire::deserialize(deserializer)?);
        response.validate().map_err(D::Error::custom)?;
        Ok(response)
    }
}

impl RecordOutcomeResponse {
    /// Validates disposition-specific revision and rejection fields.
    pub fn validate(&self) -> Result<(), RecordOutcomeContractError> {
        if self.event_id.as_str().trim().is_empty() {
            return Err(RecordOutcomeContractError::new(
                RecordOutcomeRejectionReason::InvalidOutcome,
            ));
        }
        validation::validate_digest_shape(&self.event_digest)?;
        if let Some(digest) = &self.decision_memory_digest {
            validation::validate_sha256_wire(digest.as_str())?;
        }
        validation::ensure_unique(
            &self.next_actions,
            RecordOutcomeRejectionReason::InvalidOutcome,
        )?;
        match self.disposition {
            RecordOutcomeDisposition::Recorded | RecordOutcomeDisposition::Replayed
                if self.decision_memory_revision.is_some()
                    && self.decision_memory_digest.is_some()
                    && self.reason_code.is_none() =>
            {
                Ok(())
            }
            RecordOutcomeDisposition::Rejected
                if self.decision_memory_revision.is_none()
                    && self.decision_memory_digest.is_none()
                    && self.reason_code.is_some() =>
            {
                Ok(())
            }
            _ => Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome)),
        }
    }
}

/// Validation error exposing only a stable contract reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordOutcomeContractError {
    reason: RecordOutcomeRejectionReason,
}

impl RecordOutcomeContractError {
    const fn new(reason: RecordOutcomeRejectionReason) -> Self {
        Self { reason }
    }

    /// Returns the stable reason associated with this validation failure.
    pub const fn reason_code(self) -> RecordOutcomeRejectionReason {
        self.reason
    }
}

impl Display for RecordOutcomeContractError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.reason.as_str())
    }
}

impl std::error::Error for RecordOutcomeContractError {}
