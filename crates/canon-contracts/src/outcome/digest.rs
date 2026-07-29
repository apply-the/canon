//! Versioned canonical JSON and digest-input construction.

use serde_json::Value;

use super::*;

#[derive(Serialize)]
struct CanonicalOutcomeRequest<'a> {
    source_product: OutcomeSourceProduct,
    event_id: &'a OutcomeEventId,
    source_repository_identity: &'a RepositoryIdentity,
    governance_bundle_id: &'a BundleId,
    governance_bundle_digest: &'a BundleDigest,
    session_id: &'a OutcomeSessionId,
    final_transaction_revision: Revision,
    terminal_status: TerminalOutcomeStatus,
    published_commit: &'a Option<CommitIdentity>,
    final_fingerprint: &'a Option<FinalFingerprint>,
    proof_references: Vec<&'a EvidenceReference>,
    deviations: Vec<&'a Deviation>,
    terminal_claims: Vec<&'a Claim>,
    authority_binding: CanonicalAuthorityBinding<'a>,
    approval_binding: Option<CanonicalApprovalBinding<'a>>,
    challenge_binding: CanonicalChallengeBinding<'a>,
    lineage: &'a OutcomeLineage,
    occurred_at: &'a Option<AuthoritativeTimestamp>,
}

#[derive(Serialize)]
struct CanonicalAuthorityBinding<'a> {
    authority_identity: &'a str,
    final_transaction_revision: Revision,
    claims: Vec<&'a Claim>,
}

#[derive(Serialize)]
struct CanonicalApprovalBinding<'a> {
    approver_identity: &'a str,
    decision: ApprovalDecision,
    final_transaction_revision: Revision,
    claims: Vec<&'a Claim>,
}

#[derive(Serialize)]
struct CanonicalChallengeBinding<'a> {
    tier: ChallengeTier,
    challenger_identity: &'a Option<String>,
    challenger_invocation_id: &'a Option<String>,
    independent_context_identity: &'a Option<String>,
    claims: Vec<&'a Claim>,
    evidence_references: Vec<&'a EvidenceReference>,
    named_override: &'a Option<String>,
}

pub(super) fn canonical_digest_input(
    request: &RecordOutcomeRequest,
) -> Result<String, RecordOutcomeContractError> {
    let input = CanonicalOutcomeRequest {
        source_product: request.source_product,
        event_id: &request.event_id,
        source_repository_identity: &request.source_repository_identity,
        governance_bundle_id: &request.governance_bundle_id,
        governance_bundle_digest: &request.governance_bundle_digest,
        session_id: &request.session_id,
        final_transaction_revision: request.final_transaction_revision,
        terminal_status: request.terminal_status,
        published_commit: &request.published_commit,
        final_fingerprint: &request.final_fingerprint,
        proof_references: sorted(&request.proof_references),
        deviations: sorted(&request.deviations),
        terminal_claims: sorted(&request.terminal_claims),
        authority_binding: CanonicalAuthorityBinding {
            authority_identity: request.authority_binding.authority_identity.as_str(),
            final_transaction_revision: request.authority_binding.final_transaction_revision,
            claims: sorted(&request.authority_binding.claims),
        },
        approval_binding: request.approval_binding.as_ref().map(|approval| {
            CanonicalApprovalBinding {
                approver_identity: approval.approver_identity.as_str(),
                decision: approval.decision,
                final_transaction_revision: approval.final_transaction_revision,
                claims: sorted(&approval.claims),
            }
        }),
        challenge_binding: CanonicalChallengeBinding {
            tier: request.challenge_binding.tier,
            challenger_identity: &request.challenge_binding.challenger_identity,
            challenger_invocation_id: &request.challenge_binding.challenger_invocation_id,
            independent_context_identity: &request.challenge_binding.independent_context_identity,
            claims: sorted(&request.challenge_binding.claims),
            evidence_references: sorted(&request.challenge_binding.evidence_references),
            named_override: &request.challenge_binding.named_override,
        },
        lineage: &request.lineage,
        occurred_at: &request.occurred_at,
    };
    let value = serde_json::to_value(input).map_err(|_| {
        RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome)
    })?;
    canonical_json(&value)
}

fn canonical_json(value: &Value) -> Result<String, RecordOutcomeContractError> {
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) => {
            serde_json::to_string(value).map_err(|_| {
                RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome)
            })
        }
        Value::Number(number) if number.is_i64() || number.is_u64() => Ok(number.to_string()),
        Value::Number(_) => {
            Err(RecordOutcomeContractError::new(RecordOutcomeRejectionReason::InvalidOutcome))
        }
        Value::Array(values) => {
            let items = values.iter().map(canonical_json).collect::<Result<Vec<_>, _>>()?;
            Ok(format!("[{}]", items.join(",")))
        }
        Value::Object(entries) => {
            let mut keys = entries.keys().collect::<Vec<_>>();
            keys.sort_unstable();
            let fields = keys
                .into_iter()
                .map(|key| {
                    let encoded_key = serde_json::to_string(key).map_err(|_| {
                        RecordOutcomeContractError::new(
                            RecordOutcomeRejectionReason::InvalidOutcome,
                        )
                    })?;
                    let field = entries.get(key).ok_or_else(|| {
                        RecordOutcomeContractError::new(
                            RecordOutcomeRejectionReason::InvalidOutcome,
                        )
                    })?;
                    Ok(format!("{encoded_key}:{}", canonical_json(field)?))
                })
                .collect::<Result<Vec<_>, RecordOutcomeContractError>>()?;
            Ok(format!("{{{}}}", fields.join(",")))
        }
    }
}

fn sorted<T: Ord>(values: &[T]) -> Vec<&T> {
    let mut sorted = values.iter().collect::<Vec<_>>();
    sorted.sort_unstable();
    sorted
}
