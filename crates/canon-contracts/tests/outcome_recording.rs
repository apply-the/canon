//! Contract tests for the additive Canon 0.91 Boundline outcome exchange.

use std::fmt::Debug;

use canon_contracts::{
    ApprovalDecision, AuthoritativeTimestamp, BundleDigest, BundleId, CanonContractVersion,
    ChallengeTier, Claim, CommitIdentity, DecisionMemoryDigest, Deviation, EvidenceReference,
    FinalFingerprint, OneShotOperation, OneShotRequest, OutcomeApprovalBinding,
    OutcomeAuthorityBinding, OutcomeChallengeBinding, OutcomeEventDigest, OutcomeEventId,
    OutcomeLineage, OutcomeNextAction, OutcomeSessionId, OutcomeSourceProduct,
    RecordOutcomeDisposition, RecordOutcomeRejectionReason, RecordOutcomeRequest,
    RecordOutcomeResponse, RepositoryIdentity, Revision, TerminalOutcomeStatus,
};
use serde_json::{Value, json};

const EVENT_ID: &str = "outcome-event-001";
const SESSION_ID: &str = "session-083-001";
const BUNDLE_ID: &str = "bundle-083-001";
const FINAL_REVISION: u64 = 42;
const CANON_REVISION: u64 = 19;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn require(condition: bool, message: impl Into<String>) -> TestResult {
    if condition { Ok(()) } else { Err(message.into().into()) }
}

fn require_eq<T: Debug + PartialEq>(actual: T, expected: T, message: &str) -> TestResult {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{message}: actual {actual:?}, expected {expected:?}").into())
    }
}

fn base_request(
    status: TerminalOutcomeStatus,
) -> Result<RecordOutcomeRequest, Box<dyn std::error::Error>> {
    let (published_commit, final_fingerprint) = match status {
        TerminalOutcomeStatus::Published => (
            Some(CommitIdentity::new("6b4d8ac1d1644cbd88f78d57f66aeb78550d3f42")),
            Some(FinalFingerprint::new("sha256:published-fingerprint")),
        ),
        TerminalOutcomeStatus::NoChange => {
            (None, Some(FinalFingerprint::new("sha256:unchanged-fingerprint")))
        }
        TerminalOutcomeStatus::Failed
        | TerminalOutcomeStatus::Cancelled
        | TerminalOutcomeStatus::Blocked
        | TerminalOutcomeStatus::Stale
        | TerminalOutcomeStatus::Rejected => (None, None),
    };
    let mut request = RecordOutcomeRequest {
        event_id: OutcomeEventId::new(EVENT_ID),
        event_digest: OutcomeEventDigest::placeholder(),
        source_product: OutcomeSourceProduct::Boundline,
        source_repository_identity: RepositoryIdentity::new("git-common-dir:repo-083"),
        governance_bundle_id: BundleId::new(BUNDLE_ID),
        governance_bundle_digest: BundleDigest::new("sha256:bundle-083"),
        session_id: OutcomeSessionId::new(SESSION_ID),
        final_transaction_revision: Revision::new(FINAL_REVISION),
        terminal_status: status,
        published_commit,
        final_fingerprint,
        proof_references: vec![
            EvidenceReference::new("proof:cargo-test"),
            EvidenceReference::new("proof:nextest"),
        ],
        deviations: vec![Deviation::new("deviation:none")],
        terminal_claims: vec![Claim::new("claim:verified"), Claim::new("claim:published")],
        authority_binding: OutcomeAuthorityBinding {
            authority_identity: "release-owner".to_owned(),
            final_transaction_revision: Revision::new(FINAL_REVISION),
            claims: vec![Claim::new("claim:published")],
        },
        approval_binding: Some(OutcomeApprovalBinding {
            approver_identity: "release-owner".to_owned(),
            decision: ApprovalDecision::Approved,
            final_transaction_revision: Revision::new(FINAL_REVISION),
            claims: vec![Claim::new("claim:published")],
        }),
        challenge_binding: OutcomeChallengeBinding {
            tier: ChallengeTier::Tier2,
            challenger_identity: Some("independent-reviewer".to_owned()),
            challenger_invocation_id: Some("review-invocation-001".to_owned()),
            independent_context_identity: Some("context:independent-001".to_owned()),
            claims: vec![Claim::new("claim:published")],
            evidence_references: vec![EvidenceReference::new("proof:independent-review")],
            named_override: None,
        },
        lineage: OutcomeLineage {
            producer_identity: "boundline-executor".to_owned(),
            producer_invocation_id: "boundline-invocation-001".to_owned(),
            verifier_identity: Some("independent-reviewer".to_owned()),
            verifier_invocation_id: Some("review-invocation-001".to_owned()),
        },
        occurred_at: Some(AuthoritativeTimestamp::new("2026-07-29T10:15:30Z")),
    };
    request.recompute_event_digest()?;
    Ok(request)
}

fn decode_request(
    request: &RecordOutcomeRequest,
) -> Result<RecordOutcomeRequest, serde_json::Error> {
    serde_json::from_value(serde_json::to_value(request)?)
}

#[test]
fn canon_091_inventory_adds_only_record_outcome() -> TestResult {
    let operations = [
        OneShotOperation::Capabilities,
        OneShotOperation::Start,
        OneShotOperation::Refresh,
        OneShotOperation::Approve,
        OneShotOperation::Inspect,
        OneShotOperation::Publish,
        OneShotOperation::RecordOutcome,
    ];
    let actual = operations.into_iter().map(serde_json::to_value).collect::<Result<Vec<_>, _>>()?;
    require_eq(
        actual,
        serde_json::from_str::<Vec<Value>>(
            r#"["capabilities","start","refresh","approve","inspect","publish","record_outcome"]"#,
        )?,
        "Canon 0.91 operation inventory",
    )
}

#[test]
fn historical_six_operations_keep_their_090_wire_values() -> TestResult {
    let historical = [
        (OneShotOperation::Capabilities, "capabilities"),
        (OneShotOperation::Start, "start"),
        (OneShotOperation::Refresh, "refresh"),
        (OneShotOperation::Approve, "approve"),
        (OneShotOperation::Inspect, "inspect"),
        (OneShotOperation::Publish, "publish"),
    ];
    for (operation, wire_value) in historical {
        require_eq(
            serde_json::to_value(operation)?,
            json!(wire_value),
            "0.90 operation wire value",
        )?;
    }
    Ok(())
}

#[test]
fn request_and_all_response_dispositions_round_trip() -> TestResult {
    let request = base_request(TerminalOutcomeStatus::Published)?;
    let envelope = OneShotRequest {
        contract_version: CanonContractVersion::V1,
        request_id: EVENT_ID.to_owned(),
        operation: OneShotOperation::RecordOutcome,
        payload: request.clone(),
    };
    require_eq(
        serde_json::from_value::<OneShotRequest<RecordOutcomeRequest>>(serde_json::to_value(
            &envelope,
        )?)?,
        envelope,
        "record_outcome envelope round trip",
    )?;

    for disposition in [RecordOutcomeDisposition::Recorded, RecordOutcomeDisposition::Replayed] {
        let response = RecordOutcomeResponse {
            event_id: request.event_id.clone(),
            event_digest: request.event_digest.clone(),
            disposition,
            decision_memory_revision: Some(Revision::new(CANON_REVISION)),
            decision_memory_digest: Some(DecisionMemoryDigest::new(
                "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            )),
            reason_code: None,
            next_actions: Vec::new(),
        };
        require_eq(
            serde_json::from_value::<RecordOutcomeResponse>(serde_json::to_value(&response)?)?,
            response,
            "successful outcome response round trip",
        )?;
    }

    let rejected = RecordOutcomeResponse {
        event_id: request.event_id,
        event_digest: request.event_digest,
        disposition: RecordOutcomeDisposition::Rejected,
        decision_memory_revision: None,
        decision_memory_digest: None,
        reason_code: Some(RecordOutcomeRejectionReason::UnsupportedOperation),
        next_actions: vec![OutcomeNextAction::UpgradeContract],
    };
    require_eq(
        serde_json::from_value::<RecordOutcomeResponse>(serde_json::to_value(&rejected)?)?,
        rejected,
        "rejected outcome response round trip",
    )
}

#[test]
fn unknown_operation_status_disposition_reason_and_action_fail_closed() -> TestResult {
    require(
        serde_json::from_value::<OneShotOperation>(json!("record_terminal_result")).is_err(),
        "unknown operation was accepted",
    )?;
    require(
        serde_json::from_value::<TerminalOutcomeStatus>(json!("succeeded")).is_err(),
        "unknown terminal status was accepted",
    )?;
    require(
        serde_json::from_value::<RecordOutcomeDisposition>(json!("accepted")).is_err(),
        "unknown disposition was accepted",
    )?;
    require(
        serde_json::from_value::<RecordOutcomeRejectionReason>(json!("internal_error")).is_err(),
        "unknown rejection reason was accepted",
    )?;
    require(
        serde_json::from_value::<OutcomeNextAction>(json!("force_accept")).is_err(),
        "unknown next action was accepted",
    )
}

#[test]
fn recorded_replayed_and_rejected_response_invariants_fail_closed() -> TestResult {
    let request = base_request(TerminalOutcomeStatus::Published)?;
    let successful = RecordOutcomeResponse {
        event_id: request.event_id.clone(),
        event_digest: request.event_digest.clone(),
        disposition: RecordOutcomeDisposition::Recorded,
        decision_memory_revision: Some(Revision::new(CANON_REVISION)),
        decision_memory_digest: Some(DecisionMemoryDigest::new(
            "sha256:1111111111111111111111111111111111111111111111111111111111111111",
        )),
        reason_code: None,
        next_actions: Vec::new(),
    };

    for (label, mutation) in [
        ("recorded-without-revision", json!({"decision_memory_revision": null})),
        ("recorded-with-reason", json!({"reason_code": "persistence_failed"})),
        ("recorded-with-invalid-digest", json!({"decision_memory_digest": "not-a-digest"})),
    ] {
        let mut value = serde_json::to_value(&successful)?;
        let patch = mutation.as_object().ok_or("response mutation was not an object")?;
        for (key, replacement) in patch {
            value[key] = replacement.clone();
        }
        require(
            serde_json::from_value::<RecordOutcomeResponse>(value).is_err(),
            format!("{label} response was accepted"),
        )?;
    }

    let mut rejected = serde_json::to_value(&successful)?;
    rejected["disposition"] = json!("rejected");
    rejected["reason_code"] = json!("persistence_failed");
    require(
        serde_json::from_value::<RecordOutcomeResponse>(rejected).is_err(),
        "rejected response invented a decision-memory revision",
    )
}

#[test]
fn response_identity_actions_and_error_reason_fail_closed() -> TestResult {
    let request = base_request(TerminalOutcomeStatus::Published)?;
    let response = RecordOutcomeResponse {
        event_id: request.event_id,
        event_digest: request.event_digest,
        disposition: RecordOutcomeDisposition::Rejected,
        decision_memory_revision: None,
        decision_memory_digest: None,
        reason_code: Some(RecordOutcomeRejectionReason::InvalidOutcome),
        next_actions: vec![OutcomeNextAction::RepairEvidence],
    };

    let mut empty_identity = serde_json::to_value(&response)?;
    empty_identity["event_id"] = json!("");
    let error = match serde_json::from_value::<RecordOutcomeResponse>(empty_identity) {
        Ok(_) => return Err("empty response identity was accepted".into()),
        Err(error) => error,
    };
    require(
        error.to_string().contains("invalid_outcome"),
        "response rejection did not preserve the stable reason",
    )?;

    let mut duplicate_actions = serde_json::to_value(&response)?;
    duplicate_actions["next_actions"] = json!(["repair_evidence", "repair_evidence"]);
    require(
        serde_json::from_value::<RecordOutcomeResponse>(duplicate_actions).is_err(),
        "duplicate next actions were accepted",
    )?;

    let mut invalid_request = base_request(TerminalOutcomeStatus::Published)?;
    invalid_request.lineage.producer_identity.clear();
    invalid_request.recompute_event_digest()?;
    let contract_error = match invalid_request.validate() {
        Ok(()) => return Err("empty producer identity was accepted".into()),
        Err(error) => error,
    };
    require_eq(
        contract_error.reason_code(),
        RecordOutcomeRejectionReason::InvalidOutcome,
        "contract error reason accessor",
    )
}

#[test]
fn missing_identity_and_duplicate_sets_are_rejected() -> TestResult {
    let mut missing_identity = base_request(TerminalOutcomeStatus::Published)?;
    missing_identity.event_id = OutcomeEventId::new("");
    missing_identity.recompute_event_digest()?;
    require(
        decode_request(&missing_identity).is_err(),
        "empty outcome event identity was accepted",
    )?;

    for (label, mut request) in [
        ("claims", base_request(TerminalOutcomeStatus::Published)?),
        ("proof-references", base_request(TerminalOutcomeStatus::Published)?),
    ] {
        if label == "claims" {
            request.terminal_claims.push(Claim::new("claim:published"));
        } else {
            request.proof_references.push(EvidenceReference::new("proof:cargo-test"));
        }
        request.recompute_event_digest()?;
        require(decode_request(&request).is_err(), format!("duplicate {label} were accepted"))?;
    }
    Ok(())
}

#[test]
fn digest_and_terminal_commit_invariants_are_enforced() -> TestResult {
    let mut invalid_digest = base_request(TerminalOutcomeStatus::Published)?;
    invalid_digest.event_digest = OutcomeEventDigest::new(
        "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    );
    require(decode_request(&invalid_digest).is_err(), "invalid event digest was accepted")?;

    let mut published_without_commit = base_request(TerminalOutcomeStatus::Published)?;
    published_without_commit.published_commit = None;
    published_without_commit.recompute_event_digest()?;
    require(
        decode_request(&published_without_commit).is_err(),
        "published outcome without a commit was accepted",
    )?;

    let mut no_change_with_commit = base_request(TerminalOutcomeStatus::NoChange)?;
    no_change_with_commit.published_commit = Some(CommitIdentity::new("fabricated-commit"));
    no_change_with_commit.recompute_event_digest()?;
    require(
        decode_request(&no_change_with_commit).is_err(),
        "no_change outcome with a commit was accepted",
    )?;

    for status in [
        TerminalOutcomeStatus::Failed,
        TerminalOutcomeStatus::Cancelled,
        TerminalOutcomeStatus::Rejected,
    ] {
        let mut request = base_request(status)?;
        request.published_commit = Some(CommitIdentity::new("forbidden-commit"));
        request.recompute_event_digest()?;
        require(
            decode_request(&request).is_err(),
            format!("{status:?} outcome with a commit was accepted"),
        )?;
    }
    Ok(())
}

#[test]
fn blocked_and_stale_candidates_are_explicitly_nonterminal() -> TestResult {
    for status in [TerminalOutcomeStatus::Blocked, TerminalOutcomeStatus::Stale] {
        let request = base_request(status)?;
        let error = decode_request(&request).expect_err("nonterminal status was accepted");
        require(
            error.to_string().contains("nonterminal_outcome"),
            format!("{status:?} did not use the nonterminal outcome reason"),
        )?;
    }
    Ok(())
}

#[test]
fn lineage_cannot_self_attest_independence() -> TestResult {
    let mut request = base_request(TerminalOutcomeStatus::Published)?;
    request.lineage.verifier_identity = Some(request.lineage.producer_identity.clone());
    request.lineage.verifier_invocation_id = Some(request.lineage.producer_invocation_id.clone());
    request.recompute_event_digest()?;
    let error = decode_request(&request).expect_err("self-attested lineage was accepted");
    require(
        error.to_string().contains("lineage_invalid"),
        "self-attested lineage did not use the lineage reason",
    )
}

#[test]
fn tier_zero_cannot_carry_verifier_lineage_or_independence_claims() -> TestResult {
    let mut request = base_request(TerminalOutcomeStatus::Failed)?;
    request.challenge_binding.tier = ChallengeTier::Tier0;
    request.challenge_binding.challenger_identity = None;
    request.challenge_binding.challenger_invocation_id = None;
    request.challenge_binding.independent_context_identity = None;
    request.lineage.verifier_identity = Some(request.lineage.producer_identity.clone());
    request.lineage.verifier_invocation_id = Some(request.lineage.producer_invocation_id.clone());
    request.approval_binding = None;
    request.recompute_event_digest()?;
    let error = decode_request(&request).expect_err("Tier 0 verifier lineage was accepted");
    require(
        error.to_string().contains("lineage_invalid"),
        "Tier 0 verifier lineage did not use the lineage reason",
    )
}

#[test]
fn authority_approval_and_challenge_bind_only_terminal_claims() -> TestResult {
    for (label, mut request) in [
        ("authority", base_request(TerminalOutcomeStatus::Published)?),
        ("approval", base_request(TerminalOutcomeStatus::Published)?),
        ("challenge", base_request(TerminalOutcomeStatus::Published)?),
    ] {
        let unbound = Claim::new("claim:not-terminal");
        match label {
            "authority" => request.authority_binding.claims.push(unbound),
            "approval" => request
                .approval_binding
                .as_mut()
                .ok_or("approval fixture missing")?
                .claims
                .push(unbound),
            "challenge" => request.challenge_binding.claims.push(unbound),
            _ => return Err("unknown binding fixture".into()),
        }
        request.recompute_event_digest()?;
        require(
            decode_request(&request).is_err(),
            format!("{label} accepted a claim absent from terminal_claims"),
        )?;
    }
    Ok(())
}

#[test]
fn canonical_digest_is_invariant_to_set_insertion_order() -> TestResult {
    let first = base_request(TerminalOutcomeStatus::Published)?;
    require_eq(
        first.event_digest.clone(),
        OutcomeEventDigest::new(
            "sha256:eb492104900410462528226f5fca56a758ef13b940d8a0c5d962110d5de2bafa",
        ),
        "independently calculated canonical digest fixture",
    )?;
    let mut reordered = first.clone();
    reordered.proof_references.reverse();
    reordered.terminal_claims.reverse();
    reordered.authority_binding.claims.reverse();
    reordered.challenge_binding.claims.reverse();
    reordered.recompute_event_digest()?;
    require_eq(
        reordered.event_digest,
        first.event_digest,
        "set insertion order changed the canonical digest",
    )
}

#[test]
fn rejection_reason_inventory_has_exact_wire_values() -> TestResult {
    let reasons = [
        (RecordOutcomeRejectionReason::UnsupportedContractLine, "unsupported_contract_line"),
        (RecordOutcomeRejectionReason::InvalidOutcome, "invalid_outcome"),
        (RecordOutcomeRejectionReason::NonterminalOutcome, "nonterminal_outcome"),
        (RecordOutcomeRejectionReason::IdentityDigestConflict, "identity_digest_conflict"),
        (RecordOutcomeRejectionReason::AuthorityBindingInvalid, "authority_binding_invalid"),
        (RecordOutcomeRejectionReason::ApprovalBindingInvalid, "approval_binding_invalid"),
        (RecordOutcomeRejectionReason::EvidenceBindingInvalid, "evidence_binding_invalid"),
        (RecordOutcomeRejectionReason::LineageInvalid, "lineage_invalid"),
        (RecordOutcomeRejectionReason::StaleOutcome, "stale_outcome"),
        (RecordOutcomeRejectionReason::DecisionMemoryConflict, "decision_memory_conflict"),
        (RecordOutcomeRejectionReason::PersistenceFailed, "persistence_failed"),
        (RecordOutcomeRejectionReason::UnsupportedOperation, "unsupported_operation"),
    ];
    for (reason, wire_value) in reasons {
        require_eq(
            serde_json::to_value(reason)?,
            json!(wire_value),
            "rejection reason wire value",
        )?;
        require_eq(reason.as_str(), wire_value, "rejection reason display value")?;
    }
    Ok(())
}

#[test]
fn same_identity_with_different_authoritative_content_has_a_different_digest() -> TestResult {
    let first = base_request(TerminalOutcomeStatus::Published)?;
    let mut changed = first.clone();
    changed.final_transaction_revision = Revision::new(FINAL_REVISION + 1);
    changed.authority_binding.final_transaction_revision = Revision::new(FINAL_REVISION + 1);
    if let Some(approval) = &mut changed.approval_binding {
        approval.final_transaction_revision = Revision::new(FINAL_REVISION + 1);
    }
    changed.recompute_event_digest()?;
    require(
        first.event_id == changed.event_id && first.event_digest != changed.event_digest,
        "same identity with changed authoritative content shared a digest",
    )
}

#[test]
fn portable_request_rejects_paths_secrets_raw_prompts_and_private_conversations() -> TestResult {
    let absolute_path = ["artifact:", "/", "private", "/", "proof.json"].concat();
    for (label, deviation) in [
        ("absolute-path", absolute_path.as_str()),
        ("provider-token", "provider_token:credential-material"),
        ("raw-prompt", "raw_prompt:private-instruction"),
        ("private-conversation", "private_conversation:transcript"),
    ] {
        let mut request = base_request(TerminalOutcomeStatus::Published)?;
        request.deviations = vec![Deviation::new(deviation)];
        request.recompute_event_digest()?;
        require(
            decode_request(&request).is_err(),
            format!("{label} content was accepted into a portable outcome"),
        )?;
    }
    Ok(())
}

#[test]
fn duplicate_json_keys_and_floating_point_revisions_fail_closed() -> TestResult {
    let request = base_request(TerminalOutcomeStatus::Published)?;
    let encoded = serde_json::to_string(&request)?;
    let duplicate = encoded.replacen(
        r#""event_id":"outcome-event-001","#,
        r#""event_id":"outcome-event-001","event_id":"outcome-event-002","#,
        1,
    );
    require(
        serde_json::from_str::<RecordOutcomeRequest>(&duplicate).is_err(),
        "duplicate JSON key was accepted",
    )?;

    let mut floating_revision = serde_json::to_value(&request)?;
    floating_revision["final_transaction_revision"] = json!(42.5);
    require(
        serde_json::from_value::<RecordOutcomeRequest>(floating_revision).is_err(),
        "floating-point revision was accepted",
    )
}

#[test]
fn additive_fields_preserve_the_v1_envelope_and_outcome_schema() -> TestResult {
    let request = base_request(TerminalOutcomeStatus::NoChange)?;
    let envelope = OneShotRequest {
        contract_version: CanonContractVersion::V1,
        request_id: EVENT_ID.to_owned(),
        operation: OneShotOperation::RecordOutcome,
        payload: request.clone(),
    };
    let mut value = serde_json::to_value(&envelope)?;
    value["future_envelope_projection"] = json!({"introduced_in": "0.92"});
    value["payload"]["future_outcome_projection"] = json!("ignored-by-0.91");
    let decoded = serde_json::from_value::<OneShotRequest<RecordOutcomeRequest>>(value)?;
    require_eq(decoded, envelope, "additive 0.91 outcome envelope")
}
