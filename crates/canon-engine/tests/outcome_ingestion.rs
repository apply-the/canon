//! Transactional terminal-outcome ingestion contract.

use canon_contracts::{
    ApprovalDecision, AuthoritativeTimestamp, ChallengeTier, Claim, CommitIdentity, Deviation,
    EvidenceReference, FinalFingerprint, OutcomeApprovalBinding, OutcomeAuthorityBinding,
    OutcomeChallengeBinding, OutcomeEventDigest, OutcomeEventId, OutcomeLineage, OutcomeSessionId,
    OutcomeSourceProduct, RecordOutcomeDisposition, RecordOutcomeRejectionReason,
    RecordOutcomeRequest, RepositoryIdentity, Revision, TerminalOutcomeStatus, VerificationKind,
};
use canon_engine::decision_memory::{
    ApprovalContent, ArtifactContent, DecisionMemoryGraph, DecisionMemoryStore,
    DecisionMemoryStoreSnapshot, EvidenceContent, GovernanceBundleDraft, GovernancePacketDraft,
    NodeId, OutcomeFaultPoint, SubjectArtifactBinding, VerificationRequirementContent,
    build_governance_bundle, record_outcome, record_outcome_with_fault, validate_governance_bundle,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn require(condition: bool, message: &str) -> TestResult {
    if condition { Ok(()) } else { Err(message.to_string().into()) }
}

#[test]
fn first_record_replay_and_conflict_are_atomic() -> TestResult {
    let root = tempfile::tempdir()?;
    let store = admitted_store(root.path(), "bundle-outcome")?;
    let request = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;
    let first = record_outcome(&store, "event-outcome", request.clone())?;
    require(first.disposition == RecordOutcomeDisposition::Recorded, "first event not recorded")?;
    let recorded_snapshot = store.load()?;
    let event =
        recorded_snapshot.graph.publication_outcomes().next().ok_or("outcome event missing")?;
    require(event.validation_phases.len() == 18, "validation trace is incomplete")?;
    require(
        event.execution_audit == canon_engine::decision_memory::ExecutionAuditCounters::default(),
        "outcome ingestion executed an external capability",
    )?;
    let bytes = std::fs::read(store.snapshot_path())?;
    let replay = record_outcome(&store, "event-outcome", request.clone())?;
    require(replay.disposition == RecordOutcomeDisposition::Replayed, "event not replayed")?;
    require(
        replay.decision_memory_revision == first.decision_memory_revision
            && replay.decision_memory_digest == first.decision_memory_digest,
        "replay changed durable identity",
    )?;
    require(std::fs::read(store.snapshot_path())? == bytes, "replay rewrote the snapshot")?;

    let mut changed = request;
    changed.deviations.push(Deviation::new("deviation:changed"));
    changed.recompute_event_digest()?;
    let conflict = record_outcome(&store, "event-outcome", changed)?;
    require(
        conflict.reason_code == Some(RecordOutcomeRejectionReason::IdentityDigestConflict),
        "digest conflict was not rejected",
    )?;
    require(std::fs::read(store.snapshot_path())? == bytes, "conflict mutated decision memory")
}

#[test]
fn negative_terminal_values_record_without_inferred_success() -> TestResult {
    for status in [
        TerminalOutcomeStatus::NoChange,
        TerminalOutcomeStatus::Failed,
        TerminalOutcomeStatus::Cancelled,
        TerminalOutcomeStatus::Rejected,
    ] {
        let root = tempfile::tempdir()?;
        let store = admitted_store(root.path(), "bundle-outcome")?;
        let request = outcome_request("event-outcome", status)?;
        let response = record_outcome(&store, "event-outcome", request)?;
        require(response.disposition == RecordOutcomeDisposition::Recorded, "terminal rejected")?;
        let snapshot = store.load()?;
        let outcomes = snapshot.graph.publication_outcomes().collect::<Vec<_>>();
        require(outcomes.len() == 1, "outcome event count drifted")?;
        require(outcomes[0].request.terminal_status == status, "negative terminal status changed")?;
    }
    Ok(())
}

#[test]
fn identity_nonterminal_and_binding_failures_do_not_mutate() -> TestResult {
    let root = tempfile::tempdir()?;
    let store = admitted_store(root.path(), "bundle-outcome")?;
    let before = std::fs::read(store.snapshot_path())?;
    let request = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;
    let mismatch = record_outcome(&store, "different-event", request)?;
    require(
        mismatch.reason_code == Some(RecordOutcomeRejectionReason::IdentityDigestConflict),
        "request identity mismatch was accepted",
    )?;
    require(std::fs::read(store.snapshot_path())? == before, "identity mismatch mutated state")
}

#[test]
fn fault_before_commit_is_retryable_and_fault_after_commit_replays() -> TestResult {
    let root = tempfile::tempdir()?;
    let store = admitted_store(root.path(), "bundle-outcome")?;
    let request = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;
    let before = std::fs::read(store.snapshot_path())?;
    let rejected = record_outcome_with_fault(
        &store,
        "event-outcome",
        request.clone(),
        OutcomeFaultPoint::BeforePersist,
    )?;
    require(
        rejected.reason_code == Some(RecordOutcomeRejectionReason::PersistenceFailed),
        "pre-commit fault was not typed",
    )?;
    require(std::fs::read(store.snapshot_path())? == before, "pre-commit fault persisted event")?;

    let lost = record_outcome_with_fault(
        &store,
        "event-outcome",
        request.clone(),
        OutcomeFaultPoint::AfterDurableCommit,
    );
    require(lost.is_err(), "post-commit response fault returned success")?;
    let replay = record_outcome(&store, "event-outcome", request)?;
    require(replay.disposition == RecordOutcomeDisposition::Replayed, "commit was duplicated")?;
    require(
        store.load()?.graph.publication_outcomes().count() == 1,
        "post-commit retry added an event",
    )
}

fn admitted_store(
    root: &std::path::Path,
    bundle_id: &str,
) -> Result<DecisionMemoryStore, Box<dyn std::error::Error>> {
    let bundle = build_governance_bundle(governance_draft(bundle_id))?;
    let mut graph = DecisionMemoryGraph::from_bundle(&bundle)?;
    let terminal_result = validate_governance_bundle(&bundle, &mut graph);
    let store = DecisionMemoryStore::new(root);
    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles: vec![bundle],
        graph,
        terminal_result,
    })?;
    Ok(store)
}

fn outcome_request(
    event_id: &str,
    status: TerminalOutcomeStatus,
) -> Result<RecordOutcomeRequest, Box<dyn std::error::Error>> {
    let bundle = build_governance_bundle(governance_draft("bundle-outcome"))?;
    let final_revision = Revision::new(42);
    let (published_commit, final_fingerprint) = match status {
        TerminalOutcomeStatus::Published => (
            Some(CommitIdentity::new("6b4d8ac1d1644cbd88f78d57f66aeb78550d3f42")),
            Some(FinalFingerprint::new("sha256:published-fingerprint")),
        ),
        TerminalOutcomeStatus::NoChange => {
            (None, Some(FinalFingerprint::new("sha256:unchanged-fingerprint")))
        }
        _ => (None, None),
    };
    let mut request = RecordOutcomeRequest {
        event_id: OutcomeEventId::new(event_id),
        event_digest: OutcomeEventDigest::placeholder(),
        source_product: OutcomeSourceProduct::Boundline,
        source_repository_identity: RepositoryIdentity::new("git-common-dir:canon-test"),
        governance_bundle_id: bundle.contract.bundle_id,
        governance_bundle_digest: bundle.contract.bundle_digest,
        session_id: OutcomeSessionId::new("session-outcome"),
        final_transaction_revision: final_revision,
        terminal_status: status,
        published_commit,
        final_fingerprint,
        proof_references: vec![EvidenceReference::new("proof:outcome")],
        deviations: vec![Deviation::new("deviation:none")],
        terminal_claims: vec![Claim::new("claim:outcome")],
        authority_binding: OutcomeAuthorityBinding {
            authority_identity: "release-owner".to_string(),
            final_transaction_revision: final_revision,
            claims: vec![Claim::new("claim:outcome")],
        },
        approval_binding: Some(OutcomeApprovalBinding {
            approver_identity: "release-owner".to_string(),
            decision: ApprovalDecision::Approved,
            final_transaction_revision: final_revision,
            claims: vec![Claim::new("claim:outcome")],
        }),
        challenge_binding: OutcomeChallengeBinding {
            tier: ChallengeTier::Tier2,
            challenger_identity: Some("independent-reviewer".to_string()),
            challenger_invocation_id: Some("review-invocation".to_string()),
            independent_context_identity: Some("context:independent".to_string()),
            claims: vec![Claim::new("claim:outcome")],
            evidence_references: vec![EvidenceReference::new("proof:challenge")],
            named_override: None,
        },
        lineage: OutcomeLineage {
            producer_identity: "boundline-executor".to_string(),
            producer_invocation_id: "boundline-invocation".to_string(),
            verifier_identity: Some("independent-reviewer".to_string()),
            verifier_invocation_id: Some("review-invocation".to_string()),
        },
        occurred_at: Some(AuthoritativeTimestamp::new("2026-08-05T10:15:30Z")),
    };
    request.recompute_event_digest()?;
    Ok(request)
}

fn governance_draft(bundle_id: &str) -> GovernanceBundleDraft {
    let packet_id = NodeId::new(format!("packet-{bundle_id}"));
    let artifact_id = NodeId::new(format!("artifact-{bundle_id}"));
    let claim_id = NodeId::new(format!("claim-{bundle_id}-main"));
    let requirement_id = NodeId::new(format!("requirement-{bundle_id}"));
    let evidence_id = NodeId::new(format!("evidence-{bundle_id}"));
    GovernanceBundleDraft {
        bundle_id: bundle_id.to_string(),
        profile: canon_contracts::Profile::Discovery,
        decision_memory_revision: 1,
        packets: vec![GovernancePacketDraft {
            packet_id: packet_id.to_string(),
            profile: canon_contracts::Profile::Discovery,
            revision: 1,
            change_intent: "record terminal outcome".to_string(),
            scope: vec!["workspace".to_string()],
            risks: vec!["incorrect outcome".to_string()],
            invariants: vec!["no semantic execution".to_string()],
            acceptance_criteria: vec!["exact outcome binding".to_string()],
            cross_packet_references: Vec::new(),
        }],
        subject_artifacts: vec![SubjectArtifactBinding {
            artifact_id: artifact_id.clone(),
            packet_id: packet_id.clone(),
            content: ArtifactContent {
                artifact_identity: "workspace".to_string(),
                revision: "git:fixture".to_string(),
                content_digest: "a".repeat(64),
            },
        }],
        required_approvers: vec!["release-owner".to_string()],
        authority_zone: "governance-release".to_string(),
        risk_tier: 2,
        change_class: "governance-kernel".to_string(),
        context_class: "repository-local".to_string(),
        owners: vec!["release-owner".to_string()],
        claims: vec!["claim-outcome".to_string()],
        required_evidence: vec![VerificationRequirementContent {
            requirement_id: requirement_id.clone(),
            packet_id: packet_id.clone(),
            claim_ids: vec![claim_id.clone()],
            artifact_ids: vec![artifact_id.clone()],
            kind: VerificationKind::ExternalSemanticReview,
            minimum_challenge_tier: ChallengeTier::Tier2,
            accepted_evidence_references: vec![format!("sha256:{}", "e".repeat(64))],
        }],
        provided_evidence: vec![EvidenceContent {
            evidence_id: evidence_id.clone(),
            packet_id: packet_id.clone(),
            claim_ids: vec![claim_id.clone()],
            artifact_ids: vec![artifact_id.clone()],
            requirement_ids: vec![requirement_id.clone()],
            references: vec![format!("sha256:{}", "e".repeat(64))],
            lineage: "provider:challenger/executor:review/invocation:test".to_string(),
            independent_context_identity: "context-independent".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            external_semantic: true,
            fresh: true,
            named_override: None,
        }],
        forbidden_lineages: Vec::new(),
        approvals: vec![ApprovalContent {
            approval_id: NodeId::new(format!("approval-{bundle_id}")),
            packet_id,
            claim_ids: vec![claim_id],
            artifact_ids: vec![artifact_id],
            evidence_ids: vec![evidence_id],
            requirement_ids: vec![requirement_id],
            approver: "release-owner".to_string(),
            authority_zone: "governance-release".to_string(),
            approved: true,
            decision_memory_revision: 1,
            valid_through_revision: Some(1),
            fresh: true,
        }],
        assumptions: vec!["contract immutable".to_string()],
        alternatives: vec!["defer".to_string()],
        rationale: "bind actual outcome".to_string(),
        risk_acceptances: Vec::new(),
        triggers: vec!["governed state changes".to_string()],
        no_change: false,
    }
}
