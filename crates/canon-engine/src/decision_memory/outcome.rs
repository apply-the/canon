//! Transactional admission of Boundline terminal outcomes into decision memory.
//!
//! Outcome recording extends the existing graph journal and atomic snapshot;
//! it does not create a parallel persistence authority.

use canon_contracts::{
    DecisionMemoryDigest, OutcomeNextAction, RecordOutcomeDisposition,
    RecordOutcomeRejectionReason, RecordOutcomeRequest, RecordOutcomeResponse, Revision,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{
    ContentDigest, DecisionMemoryError, DecisionMemoryStore, GovernanceDecision,
    PublicationOutcomeEvent,
};

const OUTCOME_REVISION_DIGEST_DOMAIN: &str = "decision-memory-outcome-revision";
const SHA256_PREFIX: &str = "sha256:";

/// Deterministic validation and commit phases for one terminal outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeValidationPhase {
    /// The typed public contract line is supported.
    ValidateContractVersion,
    /// The typed operation is `record_outcome`.
    ValidateOperation,
    /// Envelope and event identities are equal.
    ValidateRequestIdentity,
    /// The canonical event digest matches every authoritative field.
    ValidateCanonicalDigest,
    /// The outcome belongs to the closed terminal matrix.
    ValidateTerminalStatus,
    /// The clone-local source repository binding is portable and present.
    ValidateRepositoryBinding,
    /// The referenced governance bundle is the exact admitted bundle.
    ValidateBundleBinding,
    /// Revision bindings are internally coherent.
    ValidateTransactionRevision,
    /// Named authority covers the terminal claims.
    ValidateAuthority,
    /// Required approval is exact and current.
    ValidateApproval,
    /// Challenge policy is structurally satisfied.
    ValidateChallenge,
    /// Proof references and claim bindings are exact.
    ValidateEvidence,
    /// Producer and verifier lineages are independent where required.
    ValidateLineage,
    /// The admitted governance state remains current.
    ValidateFreshness,
    /// Existing identity is checked before revision allocation.
    CheckIdempotency,
    /// One typed journal event is added in memory.
    ApplyDecisionMemoryEvent,
    /// Graph, journal, response identity, and snapshot commit atomically.
    PersistAtomically,
    /// The typed public response is projected.
    ProjectResponse,
}

impl OutcomeValidationPhase {
    fn complete_trace() -> Vec<Self> {
        vec![
            Self::ValidateContractVersion,
            Self::ValidateOperation,
            Self::ValidateRequestIdentity,
            Self::ValidateCanonicalDigest,
            Self::ValidateTerminalStatus,
            Self::ValidateRepositoryBinding,
            Self::ValidateBundleBinding,
            Self::ValidateTransactionRevision,
            Self::ValidateAuthority,
            Self::ValidateApproval,
            Self::ValidateChallenge,
            Self::ValidateEvidence,
            Self::ValidateLineage,
            Self::ValidateFreshness,
            Self::CheckIdempotency,
            Self::ApplyDecisionMemoryEvent,
            Self::PersistAtomically,
            Self::ProjectResponse,
        ]
    }

    pub(crate) fn is_complete_trace(phases: &[Self]) -> bool {
        phases == Self::complete_trace()
    }
}

/// Declared fault boundaries used to qualify crash semantics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutcomeFaultPoint {
    /// Normal production execution.
    #[default]
    None,
    /// Stop before any in-memory mutation.
    BeforeMutation,
    /// Stop after graph/journal mutation but before snapshot persistence.
    AfterJournalBeforePersist,
    /// Simulate failure at the atomic persistence boundary.
    BeforePersist,
    /// Lose the response after the durable commit.
    AfterDurableCommit,
}

/// Failures for which no trustworthy response could be delivered.
#[derive(Debug, Error)]
pub enum OutcomeIngestionError {
    /// Existing decision memory could not be loaded or validated.
    #[error(transparent)]
    DecisionMemory(#[from] DecisionMemoryError),
    /// The event committed, but the response was deliberately lost.
    #[error("outcome committed but the response was unavailable")]
    ResponseUnavailableAfterCommit,
}

/// Records one event through the production no-fault path.
pub fn record_outcome(
    store: &DecisionMemoryStore,
    request_id: &str,
    request: RecordOutcomeRequest,
) -> Result<RecordOutcomeResponse, OutcomeIngestionError> {
    record_outcome_with_fault(store, request_id, request, OutcomeFaultPoint::None)
}

/// Records one event with an optional deterministic fault boundary.
pub fn record_outcome_with_fault(
    store: &DecisionMemoryStore,
    request_id: &str,
    request: RecordOutcomeRequest,
    fault: OutcomeFaultPoint,
) -> Result<RecordOutcomeResponse, OutcomeIngestionError> {
    let mut snapshot = store.load()?;
    if let Some(recorded) = snapshot
        .graph
        .publication_outcomes()
        .find(|outcome| outcome.request.event_id == request.event_id)
    {
        return replay_or_conflict(recorded, &request).map_err(OutcomeIngestionError::from);
    }
    if request_id != request.event_id.as_str() {
        return Ok(rejected(&request, RecordOutcomeRejectionReason::IdentityDigestConflict));
    }
    if let Err(error) = request.validate() {
        return Ok(rejected(&request, error.reason_code()));
    }
    if let Some(reason) = validate_against_snapshot(&snapshot, &request)? {
        return Ok(rejected(&request, reason));
    }
    if fault == OutcomeFaultPoint::BeforeMutation {
        return Ok(rejected(&request, RecordOutcomeRejectionReason::PersistenceFailed));
    }

    let outcome = snapshot.graph.record_publication_outcome(
        request.clone(),
        OutcomeValidationPhase::complete_trace(),
        super::ExecutionAuditCounters::default(),
    );
    snapshot.terminal_result.graph_digest = snapshot.graph.digest()?.sha256;
    let response = successful_response(&outcome, RecordOutcomeDisposition::Recorded)?;
    if matches!(
        fault,
        OutcomeFaultPoint::AfterJournalBeforePersist | OutcomeFaultPoint::BeforePersist
    ) {
        return Ok(rejected(&request, RecordOutcomeRejectionReason::PersistenceFailed));
    }
    store.persist(&snapshot)?;
    if fault == OutcomeFaultPoint::AfterDurableCommit {
        return Err(OutcomeIngestionError::ResponseUnavailableAfterCommit);
    }
    Ok(response)
}

fn validate_against_snapshot(
    snapshot: &super::DecisionMemoryStoreSnapshot,
    request: &RecordOutcomeRequest,
) -> Result<Option<RecordOutcomeRejectionReason>, DecisionMemoryError> {
    let Some(bundle) = snapshot
        .admitted_bundles
        .iter()
        .find(|bundle| bundle.contract.bundle_id == request.governance_bundle_id)
    else {
        return Ok(Some(RecordOutcomeRejectionReason::DecisionMemoryConflict));
    };
    if bundle.contract.bundle_digest != request.governance_bundle_digest {
        return Ok(Some(RecordOutcomeRejectionReason::DecisionMemoryConflict));
    }
    if snapshot.terminal_result.decision == GovernanceDecision::Stale {
        return Ok(Some(RecordOutcomeRejectionReason::StaleOutcome));
    }
    if snapshot.terminal_result.decision != GovernanceDecision::Accepted {
        return Ok(Some(RecordOutcomeRejectionReason::DecisionMemoryConflict));
    }
    if bundle.metadata.decision_memory_revision == 0 {
        return Ok(Some(RecordOutcomeRejectionReason::DecisionMemoryConflict));
    }
    Ok(None)
}

fn replay_or_conflict(
    recorded: &PublicationOutcomeEvent,
    request: &RecordOutcomeRequest,
) -> Result<RecordOutcomeResponse, DecisionMemoryError> {
    if recorded.request.event_digest == request.event_digest {
        successful_response(recorded, RecordOutcomeDisposition::Replayed)
    } else {
        Ok(rejected(request, RecordOutcomeRejectionReason::IdentityDigestConflict))
    }
}

fn successful_response(
    outcome: &PublicationOutcomeEvent,
    disposition: RecordOutcomeDisposition,
) -> Result<RecordOutcomeResponse, DecisionMemoryError> {
    let digest = ContentDigest::compute(
        OUTCOME_REVISION_DIGEST_DOMAIN,
        &(&outcome.request, outcome.decision_memory_revision),
    )?;
    Ok(RecordOutcomeResponse {
        event_id: outcome.request.event_id.clone(),
        event_digest: outcome.request.event_digest.clone(),
        disposition,
        decision_memory_revision: Some(Revision::new(outcome.decision_memory_revision)),
        decision_memory_digest: Some(DecisionMemoryDigest::new(format!(
            "{SHA256_PREFIX}{}",
            digest.sha256
        ))),
        reason_code: None,
        next_actions: Vec::new(),
    })
}

fn rejected(
    request: &RecordOutcomeRequest,
    reason: RecordOutcomeRejectionReason,
) -> RecordOutcomeResponse {
    RecordOutcomeResponse {
        event_id: request.event_id.clone(),
        event_digest: request.event_digest.clone(),
        disposition: RecordOutcomeDisposition::Rejected,
        decision_memory_revision: None,
        decision_memory_digest: None,
        reason_code: Some(reason),
        next_actions: next_actions(reason),
    }
}

fn next_actions(reason: RecordOutcomeRejectionReason) -> Vec<OutcomeNextAction> {
    match reason {
        RecordOutcomeRejectionReason::PersistenceFailed => {
            vec![OutcomeNextAction::RestorePersistence, OutcomeNextAction::Retry]
        }
        RecordOutcomeRejectionReason::IdentityDigestConflict => {
            vec![OutcomeNextAction::ResolveIdentityConflict]
        }
        RecordOutcomeRejectionReason::UnsupportedContractLine
        | RecordOutcomeRejectionReason::UnsupportedOperation => {
            vec![OutcomeNextAction::UpgradeContract]
        }
        RecordOutcomeRejectionReason::AuthorityBindingInvalid
        | RecordOutcomeRejectionReason::ApprovalBindingInvalid => {
            vec![OutcomeNextAction::ObtainAuthority]
        }
        RecordOutcomeRejectionReason::EvidenceBindingInvalid
        | RecordOutcomeRejectionReason::LineageInvalid => {
            vec![OutcomeNextAction::RepairEvidence]
        }
        RecordOutcomeRejectionReason::StaleOutcome
        | RecordOutcomeRejectionReason::NonterminalOutcome => {
            vec![OutcomeNextAction::ReverifyOutcome]
        }
        RecordOutcomeRejectionReason::InvalidOutcome
        | RecordOutcomeRejectionReason::DecisionMemoryConflict => {
            vec![OutcomeNextAction::InspectDecisionMemory]
        }
    }
}
