//! Closed validation rules for terminal outcome exchange.

use std::collections::BTreeSet;

use serde_json::Value;

use super::*;

pub(super) fn validate_identity(
    request: &RecordOutcomeRequest,
) -> Result<(), RecordOutcomeContractError> {
    for identity in [
        request.event_id.as_str(),
        request.source_repository_identity.as_str(),
        request.session_id.as_str(),
    ] {
        if identity.trim().is_empty() || contains_forbidden_path(identity) {
            return Err(RecordOutcomeContractError::new(
                RecordOutcomeRejectionReason::InvalidOutcome,
            ));
        }
    }
    for identity in [
        request.authority_binding.authority_identity.as_str(),
        request.lineage.producer_identity.as_str(),
        request.lineage.producer_invocation_id.as_str(),
    ] {
        if identity.trim().is_empty() {
            return Err(RecordOutcomeContractError::new(
                RecordOutcomeRejectionReason::InvalidOutcome,
            ));
        }
    }
    if request.occurred_at.as_ref().is_some_and(|value| value.as_str().trim().is_empty()) {
        return Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome));
    }
    Ok(())
}

pub(super) fn validate_portability(
    request: &RecordOutcomeRequest,
) -> Result<(), RecordOutcomeContractError> {
    let value = serde_json::to_value(request).map_err(|_| {
        RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome)
    })?;
    if contains_private_value(&value) {
        Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome))
    } else {
        Ok(())
    }
}

fn contains_private_value(value: &Value) -> bool {
    match value {
        Value::String(candidate) => is_forbidden_portable_value(candidate),
        Value::Array(values) => values.iter().any(contains_private_value),
        Value::Object(entries) => entries.values().any(contains_private_value),
        Value::Null | Value::Bool(_) | Value::Number(_) => false,
    }
}

fn is_forbidden_portable_value(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    contains_forbidden_path(value)
        || value.starts_with("file://")
        || PRIVATE_VALUE_MARKERS.iter().any(|marker| normalized.contains(marker))
}

pub(super) fn validate_terminal_status(
    request: &RecordOutcomeRequest,
) -> Result<(), RecordOutcomeContractError> {
    if !request.terminal_status.is_terminal() {
        return Err(RecordOutcomeContractError::new(
            RecordOutcomeRejectionReason::NonterminalOutcome,
        ));
    }
    match request.terminal_status {
        TerminalOutcomeStatus::Published
            if request.published_commit.is_some() && request.final_fingerprint.is_some() =>
        {
            Ok(())
        }
        TerminalOutcomeStatus::NoChange
            if request.published_commit.is_none() && request.final_fingerprint.is_some() =>
        {
            Ok(())
        }
        TerminalOutcomeStatus::Failed
        | TerminalOutcomeStatus::Cancelled
        | TerminalOutcomeStatus::Rejected
            if request.published_commit.is_none() =>
        {
            Ok(())
        }
        _ => Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome)),
    }
}

pub(super) fn validate_set_bindings(
    request: &RecordOutcomeRequest,
) -> Result<(), RecordOutcomeContractError> {
    ensure_unique(&request.proof_references, RecordOutcomeRejectionReason::EvidenceBindingInvalid)?;
    ensure_unique(&request.deviations, RecordOutcomeRejectionReason::InvalidOutcome)?;
    ensure_unique(&request.terminal_claims, RecordOutcomeRejectionReason::EvidenceBindingInvalid)?;
    ensure_unique(
        &request.authority_binding.claims,
        RecordOutcomeRejectionReason::AuthorityBindingInvalid,
    )?;
    ensure_unique(
        &request.challenge_binding.claims,
        RecordOutcomeRejectionReason::EvidenceBindingInvalid,
    )?;
    ensure_unique(
        &request.challenge_binding.evidence_references,
        RecordOutcomeRejectionReason::EvidenceBindingInvalid,
    )?;
    if let Some(approval) = &request.approval_binding {
        ensure_unique(&approval.claims, RecordOutcomeRejectionReason::ApprovalBindingInvalid)?;
        ensure_subset(
            &approval.claims,
            &request.terminal_claims,
            RecordOutcomeRejectionReason::ApprovalBindingInvalid,
        )?;
    }
    ensure_subset(
        &request.authority_binding.claims,
        &request.terminal_claims,
        RecordOutcomeRejectionReason::AuthorityBindingInvalid,
    )?;
    ensure_subset(
        &request.challenge_binding.claims,
        &request.terminal_claims,
        RecordOutcomeRejectionReason::EvidenceBindingInvalid,
    )?;
    Ok(())
}

pub(super) fn validate_authority(
    request: &RecordOutcomeRequest,
) -> Result<(), RecordOutcomeContractError> {
    if request.authority_binding.authority_identity.trim().is_empty()
        || request.authority_binding.final_transaction_revision
            != request.final_transaction_revision
        || request.authority_binding.claims.is_empty()
    {
        return Err(RecordOutcomeContractError::new(
            RecordOutcomeRejectionReason::AuthorityBindingInvalid,
        ));
    }
    let approval_required =
        matches!(request.challenge_binding.tier, ChallengeTier::Tier2 | ChallengeTier::Tier3);
    match (&request.approval_binding, approval_required) {
        (Some(approval), _)
            if !approval.approver_identity.trim().is_empty()
                && approval.decision == ApprovalDecision::Approved
                && approval.final_transaction_revision == request.final_transaction_revision
                && !approval.claims.is_empty() =>
        {
            Ok(())
        }
        (None, false) => Ok(()),
        _ => Err(RecordOutcomeContractError::new(
            RecordOutcomeRejectionReason::ApprovalBindingInvalid,
        )),
    }
}

pub(super) fn validate_challenge_and_lineage(
    request: &RecordOutcomeRequest,
) -> Result<(), RecordOutcomeContractError> {
    let producer = &request.lineage;
    let verifier_pair = match (&producer.verifier_identity, &producer.verifier_invocation_id) {
        (Some(identity), Some(invocation))
            if !identity.trim().is_empty() && !invocation.trim().is_empty() =>
        {
            Some((identity, invocation))
        }
        (None, None) => None,
        _ => {
            return Err(RecordOutcomeContractError::new(
                RecordOutcomeRejectionReason::LineageInvalid,
            ));
        }
    };
    let independent_challenge = match request.challenge_binding.tier {
        ChallengeTier::Tier0
            if verifier_pair.is_none()
                && request.challenge_binding.challenger_identity.is_none()
                && request.challenge_binding.challenger_invocation_id.is_none()
                && request.challenge_binding.independent_context_identity.is_none()
                && request.challenge_binding.named_override.is_none() =>
        {
            return Ok(());
        }
        ChallengeTier::Tier0 => {
            return Err(RecordOutcomeContractError::new(
                RecordOutcomeRejectionReason::LineageInvalid,
            ));
        }
        ChallengeTier::Tier1 | ChallengeTier::Tier2 | ChallengeTier::Tier3 => {
            challenge_identity(&request.challenge_binding)?
        }
    };
    let (verifier_identity, verifier_invocation) = verifier_pair.ok_or_else(|| {
        RecordOutcomeContractError::new(RecordOutcomeRejectionReason::LineageInvalid)
    })?;
    if verifier_identity != independent_challenge.0
        || verifier_invocation != independent_challenge.1
        || (verifier_identity == &producer.producer_identity
            && verifier_invocation == &producer.producer_invocation_id)
    {
        return Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::LineageInvalid));
    }
    Ok(())
}

fn challenge_identity(
    challenge: &OutcomeChallengeBinding,
) -> Result<(&String, &String), RecordOutcomeContractError> {
    let identity = challenge.challenger_identity.as_ref().ok_or_else(|| {
        RecordOutcomeContractError::new(RecordOutcomeRejectionReason::EvidenceBindingInvalid)
    })?;
    let invocation = challenge.challenger_invocation_id.as_ref().ok_or_else(|| {
        RecordOutcomeContractError::new(RecordOutcomeRejectionReason::EvidenceBindingInvalid)
    })?;
    let context = challenge.independent_context_identity.as_ref().ok_or_else(|| {
        RecordOutcomeContractError::new(RecordOutcomeRejectionReason::EvidenceBindingInvalid)
    })?;
    if identity.trim().is_empty()
        || invocation.trim().is_empty()
        || context.trim().is_empty()
        || challenge.claims.is_empty()
        || challenge.evidence_references.is_empty()
        || challenge.named_override.as_ref().is_some_and(|value| value.trim().is_empty())
    {
        return Err(RecordOutcomeContractError::new(
            RecordOutcomeRejectionReason::EvidenceBindingInvalid,
        ));
    }
    Ok((identity, invocation))
}

pub(super) fn validate_digest_shape(
    digest: &OutcomeEventDigest,
) -> Result<(), RecordOutcomeContractError> {
    validate_sha256_wire(digest.as_str())
}

pub(super) fn validate_sha256_wire(digest: &str) -> Result<(), RecordOutcomeContractError> {
    let Some(hex) = digest.strip_prefix(SHA256_PREFIX) else {
        return Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome));
    };
    if hex.len() != SHA256_HEX_LENGTH
        || !hex.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome));
    }
    Ok(())
}

pub(super) fn ensure_unique<T: Ord>(
    values: &[T],
    reason: RecordOutcomeRejectionReason,
) -> Result<(), RecordOutcomeContractError> {
    let unique = values.iter().collect::<BTreeSet<_>>();
    if unique.len() == values.len() { Ok(()) } else { Err(RecordOutcomeContractError::new(reason)) }
}

fn ensure_subset<T: Ord>(
    values: &[T],
    allowed: &[T],
    reason: RecordOutcomeRejectionReason,
) -> Result<(), RecordOutcomeContractError> {
    let allowed = allowed.iter().collect::<BTreeSet<_>>();
    if values.iter().all(|value| allowed.contains(value)) {
        Ok(())
    } else {
        Err(RecordOutcomeContractError::new(reason))
    }
}

fn contains_forbidden_path(value: &str) -> bool {
    value.starts_with('/') || value.contains(":/") || value.contains(r":\")
}
