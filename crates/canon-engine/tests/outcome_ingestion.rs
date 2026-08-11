//! Transactional terminal-outcome ingestion contract.

use canon_contracts::{
    ApprovalDecision, AuthoritativeTimestamp, ChallengeTier, Claim, CommitIdentity, Deviation,
    EvidenceReference, FinalFingerprint, OutcomeApprovalBinding, OutcomeAuthorityBinding,
    OutcomeChallengeBinding, OutcomeEventDigest, OutcomeEventId, OutcomeLineage, OutcomeSessionId,
    OutcomeSourceProduct, RecordOutcomeDisposition, RecordOutcomeRejectionReason,
    RecordOutcomeRequest, RepositoryIdentity, Revision, TerminalOutcomeStatus, VerificationKind,
};
use canon_engine::decision_memory::{
    ApprovalContent, ArtifactContent, DecisionContent, DecisionMemoryGraph, DecisionMemoryNode,
    DecisionMemoryStore, DecisionMemoryStoreSnapshot, DependencyEdge, DependencyKind,
    EvidenceContent, GovernanceBundleDraft, GovernanceDecision, GovernancePacketDraft,
    NodeEnvelope, NodeId, OutcomeFaultPoint, SubjectArtifactBinding, ValidationCode,
    VerificationRequirementContent, build_governance_bundle, record_outcome,
    record_outcome_with_fault, validate_governance_bundle,
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
fn contract_and_bundle_rejections_preserve_exact_actions_without_mutation() -> TestResult {
    let root = tempfile::tempdir()?;
    let store = admitted_store(root.path(), "bundle-outcome")?;
    let before = std::fs::read(store.snapshot_path())?;
    let original = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;

    let mut invalid_digest = original.clone();
    invalid_digest.event_digest = OutcomeEventDigest::new("sha256:invalid");
    require_rejection(
        &store,
        invalid_digest,
        RecordOutcomeRejectionReason::InvalidOutcome,
        canon_contracts::OutcomeNextAction::InspectDecisionMemory,
    )?;

    let mut bundle_conflict = original.clone();
    bundle_conflict.governance_bundle_digest =
        canon_contracts::BundleDigest::new("sha256:different-bundle");
    bundle_conflict.recompute_event_digest()?;
    require_rejection(
        &store,
        bundle_conflict,
        RecordOutcomeRejectionReason::DecisionMemoryConflict,
        canon_contracts::OutcomeNextAction::InspectDecisionMemory,
    )?;

    let mut authority = original.clone();
    authority.authority_binding.claims.clear();
    authority.recompute_event_digest()?;
    require_rejection(
        &store,
        authority,
        RecordOutcomeRejectionReason::AuthorityBindingInvalid,
        canon_contracts::OutcomeNextAction::ObtainAuthority,
    )?;

    let mut approval = original.clone();
    approval.approval_binding = None;
    approval.recompute_event_digest()?;
    require_rejection(
        &store,
        approval,
        RecordOutcomeRejectionReason::ApprovalBindingInvalid,
        canon_contracts::OutcomeNextAction::ObtainAuthority,
    )?;

    let mut evidence = original.clone();
    evidence.challenge_binding.evidence_references.clear();
    evidence.recompute_event_digest()?;
    require_rejection(
        &store,
        evidence,
        RecordOutcomeRejectionReason::EvidenceBindingInvalid,
        canon_contracts::OutcomeNextAction::RepairEvidence,
    )?;

    let mut lineage = original;
    lineage.lineage.verifier_identity = Some(lineage.lineage.producer_identity.clone());
    lineage.lineage.verifier_invocation_id = Some(lineage.lineage.producer_invocation_id.clone());
    lineage.recompute_event_digest()?;
    require_rejection(
        &store,
        lineage,
        RecordOutcomeRejectionReason::LineageInvalid,
        canon_contracts::OutcomeNextAction::RepairEvidence,
    )?;

    require(
        std::fs::read(store.snapshot_path())? == before,
        "rejected outcomes mutated decision memory",
    )
}

#[test]
fn frozen_outcome_contract_rejects_partial_lineage_and_duplicate_bindings() -> TestResult {
    let original = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;

    let mut empty_time = original.clone();
    empty_time.occurred_at = Some(AuthoritativeTimestamp::new(""));
    require_contract_reason(&empty_time, RecordOutcomeRejectionReason::InvalidOutcome)?;

    let mut authority_duplicate = original.clone();
    authority_duplicate.authority_binding.claims.push(Claim::new("claim:outcome"));
    require_contract_reason(
        &authority_duplicate,
        RecordOutcomeRejectionReason::AuthorityBindingInvalid,
    )?;

    let mut challenge_claim_duplicate = original.clone();
    challenge_claim_duplicate.challenge_binding.claims.push(Claim::new("claim:outcome"));
    require_contract_reason(
        &challenge_claim_duplicate,
        RecordOutcomeRejectionReason::EvidenceBindingInvalid,
    )?;

    let mut challenge_evidence_duplicate = original.clone();
    challenge_evidence_duplicate
        .challenge_binding
        .evidence_references
        .push(EvidenceReference::new("proof:challenge"));
    require_contract_reason(
        &challenge_evidence_duplicate,
        RecordOutcomeRejectionReason::EvidenceBindingInvalid,
    )?;

    let mut partial_verifier = original.clone();
    partial_verifier.lineage.verifier_invocation_id = None;
    require_contract_reason(&partial_verifier, RecordOutcomeRejectionReason::LineageInvalid)?;

    let mut tier_zero = original.clone();
    tier_zero.approval_binding = None;
    tier_zero.challenge_binding.tier = ChallengeTier::Tier0;
    tier_zero.challenge_binding.challenger_identity = None;
    tier_zero.challenge_binding.challenger_invocation_id = None;
    tier_zero.challenge_binding.independent_context_identity = None;
    tier_zero.challenge_binding.evidence_references.clear();
    tier_zero.lineage.verifier_identity = None;
    tier_zero.lineage.verifier_invocation_id = None;
    tier_zero.recompute_event_digest()?;
    require(tier_zero.validate().is_ok(), "valid Tier 0 contract was rejected")?;

    let mut missing_verifier = original.clone();
    missing_verifier.challenge_binding.tier = ChallengeTier::Tier1;
    missing_verifier.lineage.verifier_identity = None;
    missing_verifier.lineage.verifier_invocation_id = None;
    require_contract_reason(&missing_verifier, RecordOutcomeRejectionReason::LineageInvalid)?;

    let mut tier_zero_with_reviewer = original.clone();
    tier_zero_with_reviewer.challenge_binding.tier = ChallengeTier::Tier0;
    require_contract_reason(
        &tier_zero_with_reviewer,
        RecordOutcomeRejectionReason::LineageInvalid,
    )?;

    let mut mismatched_reviewer = original.clone();
    mismatched_reviewer.lineage.verifier_identity = Some("different-reviewer".to_string());
    require_contract_reason(&mismatched_reviewer, RecordOutcomeRejectionReason::LineageInvalid)?;

    for missing in ["identity", "invocation", "context"] {
        let mut request = original.clone();
        request.challenge_binding.tier = ChallengeTier::Tier1;
        match missing {
            "identity" => request.challenge_binding.challenger_identity = None,
            "invocation" => request.challenge_binding.challenger_invocation_id = None,
            "context" => request.challenge_binding.independent_context_identity = None,
            _ => return Err("unknown challenge fixture".into()),
        }
        require_contract_reason(&request, RecordOutcomeRejectionReason::EvidenceBindingInvalid)?;
    }
    Ok(())
}

#[test]
fn outcome_authorization_rejects_conflicting_or_unreconstructable_governance() -> TestResult {
    let bundle = build_governance_bundle(governance_draft("bundle-outcome"))?;

    let mut conflict_graph = DecisionMemoryGraph::from_bundle(&bundle)?;
    let conflicting_decision = NodeEnvelope::new(
        NodeId::new("decision-bundle-outcome-r1-accepted"),
        "deterministic-governance",
        DecisionContent {
            bundle_id: "bundle-outcome".to_string(),
            rationale: "conflicting terminal decision".to_string(),
            alternatives: bundle.metadata.alternatives.clone(),
            assumptions: bundle.metadata.assumptions.clone(),
            triggers: bundle.metadata.triggers.clone(),
            owners: bundle.metadata.owners.clone(),
            decision: GovernanceDecision::Accepted,
        },
    )?;
    conflict_graph.insert_or_replay(DecisionMemoryNode::Decision(conflicting_decision))?;
    let conflict = validate_governance_bundle(&bundle, &mut conflict_graph);
    require(conflict.decision == GovernanceDecision::Conflict, "decision conflict was accepted")?;
    require(
        conflict.findings.iter().any(|finding| finding.code == ValidationCode::Conflict),
        "decision conflict lacked a deterministic finding",
    )?;

    let mut changed_draft = governance_draft("bundle-outcome");
    changed_draft.subject_artifacts[0].content.content_digest = "b".repeat(64);
    let changed_bundle = build_governance_bundle(changed_draft)?;
    let mut original_graph = DecisionMemoryGraph::from_bundle(&bundle)?;
    let mismatch = validate_governance_bundle(&changed_bundle, &mut original_graph);
    require(
        mismatch.findings.iter().any(|finding| {
            finding.code == ValidationCode::Conflict
                && finding.message == "graph node does not match the bound bundle"
        }),
        "bundle-to-graph content mismatch was not detected",
    )?;

    let mut unreconstructable = bundle.clone();
    unreconstructable.metadata.subject_artifacts[0].packet_id = NodeId::new("missing-packet");
    let mut admitted_graph = DecisionMemoryGraph::from_bundle(&bundle)?;
    let rejected = validate_governance_bundle(&unreconstructable, &mut admitted_graph);
    require(
        rejected.findings.iter().any(|finding| {
            finding.code == ValidationCode::Conflict
                && finding.message == "bound graph could not be reconstructed"
        }),
        "unreconstructable governance did not fail closed",
    )
}

#[test]
fn outcome_authorization_rejects_stale_missing_and_extra_governance_state() -> TestResult {
    let mut stale_draft = governance_draft("bundle-outcome");
    stale_draft.provided_evidence[0].fresh = false;
    let stale_bundle = build_governance_bundle(stale_draft)?;
    let stale_graph = DecisionMemoryGraph::from_bundle(&stale_bundle)?;
    let mut stale_value = serde_json::to_value(stale_graph)?;
    stale_value["nodes"]["evidence-bundle-outcome"]["node"]["freshness"] =
        serde_json::json!({"state": "fresh"});
    let mut invalid_freshness = serde_json::from_value::<DecisionMemoryGraph>(stale_value)?;
    let freshness_result = validate_governance_bundle(&stale_bundle, &mut invalid_freshness);
    require(
        freshness_result.findings.iter().any(|finding| {
            finding.code == ValidationCode::Conflict
                && finding.message == "declared stale content is represented as fresh"
        }),
        "invalid declared freshness was accepted",
    )?;

    let bundle = build_governance_bundle(governance_draft("bundle-outcome"))?;
    let artifact_id = NodeId::new("artifact-bundle-outcome");
    let mut missing_graph = DecisionMemoryGraph::from_bundle(&bundle)?;
    missing_graph.remove_node(&artifact_id)?;
    let missing_result = validate_governance_bundle(&bundle, &mut missing_graph);
    require(
        missing_result.findings.iter().any(|finding| {
            finding.code == ValidationCode::DanglingReference
                && finding.node_id.as_ref() == Some(&artifact_id)
        }),
        "missing active artifact was accepted",
    )?;

    let mut extra_edge_graph = DecisionMemoryGraph::from_bundle(&bundle)?;
    extra_edge_graph.add_edge(DependencyEdge::new(
        NodeId::new("claim-bundle-outcome-main"),
        NodeId::new("evidence-bundle-outcome"),
        DependencyKind::Binding,
    ))?;
    let extra_edge_result = validate_governance_bundle(&bundle, &mut extra_edge_graph);
    require(
        extra_edge_result.findings.iter().any(|finding| {
            finding.code == ValidationCode::Conflict
                && finding.message == "graph edges do not match the bound bundle"
        }),
        "extra governance edge was accepted",
    )?;

    let mut internal_review_draft = governance_draft("bundle-internal-review");
    internal_review_draft.provided_evidence[0].external_semantic = false;
    let internal_review_bundle = build_governance_bundle(internal_review_draft)?;
    let mut internal_review_graph = DecisionMemoryGraph::from_bundle(&internal_review_bundle)?;
    let internal_review =
        validate_governance_bundle(&internal_review_bundle, &mut internal_review_graph);
    require(
        internal_review.decision == GovernanceDecision::RequiredMissing,
        "internal semantic review satisfied external evidence policy",
    )
}

#[test]
fn fault_before_commit_is_retryable_and_fault_after_commit_replays() -> TestResult {
    let root = tempfile::tempdir()?;
    let store = admitted_store(root.path(), "bundle-outcome")?;
    let request = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;
    let before = std::fs::read(store.snapshot_path())?;
    for fault in [
        OutcomeFaultPoint::BeforeMutation,
        OutcomeFaultPoint::AfterJournalBeforePersist,
        OutcomeFaultPoint::BeforePersist,
    ] {
        let rejected = record_outcome_with_fault(&store, "event-outcome", request.clone(), fault)?;
        require(
            rejected.reason_code == Some(RecordOutcomeRejectionReason::PersistenceFailed),
            "pre-commit fault was not typed",
        )?;
        require(
            std::fs::read(store.snapshot_path())? == before,
            "pre-commit fault persisted event",
        )?;
    }

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

#[test]
fn persisted_outcome_audit_and_revision_tampering_fail_closed() -> TestResult {
    let root = tempfile::tempdir()?;
    let store = admitted_store(root.path(), "bundle-outcome")?;
    let request = outcome_request("event-outcome", TerminalOutcomeStatus::Published)?;
    record_outcome(&store, "event-outcome", request)?;
    let snapshot = store.load()?;

    let mut audit_value = serde_json::to_value(&snapshot)?;
    let audit_outcome = outcome_event_value(&mut audit_value)?;
    audit_outcome["execution_audit"]["process_invocations"] = serde_json::json!(1);
    let mut audit_snapshot = serde_json::from_value::<DecisionMemoryStoreSnapshot>(audit_value)?;
    audit_snapshot.terminal_result.graph_digest = audit_snapshot.graph.digest()?.sha256;
    require(
        store.persist(&audit_snapshot).is_err(),
        "nonzero outcome execution audit was persisted",
    )?;

    let mut revision_value = serde_json::to_value(snapshot)?;
    let revision_outcome = outcome_event_value(&mut revision_value)?;
    let revision =
        revision_outcome["decision_memory_revision"].as_u64().ok_or("outcome revision missing")?;
    revision_outcome["decision_memory_revision"] = serde_json::json!(revision.saturating_add(1));
    let mut revision_snapshot =
        serde_json::from_value::<DecisionMemoryStoreSnapshot>(revision_value)?;
    revision_snapshot.terminal_result.graph_digest = revision_snapshot.graph.digest()?.sha256;
    require(store.persist(&revision_snapshot).is_err(), "divergent outcome revision was persisted")
}

fn outcome_event_value(value: &mut serde_json::Value) -> Result<&mut serde_json::Value, String> {
    value["graph"]["events"]
        .as_array_mut()
        .and_then(|events| events.iter_mut().find(|event| event["event"] == "outcome_recorded"))
        .and_then(|event| event.get_mut("outcome"))
        .ok_or_else(|| "serialized outcome event missing".to_string())
}

fn require_rejection(
    store: &DecisionMemoryStore,
    request: RecordOutcomeRequest,
    reason: RecordOutcomeRejectionReason,
    action: canon_contracts::OutcomeNextAction,
) -> TestResult {
    let event_id = request.event_id.as_str().to_owned();
    let response = record_outcome(store, &event_id, request)?;
    require(response.reason_code == Some(reason), "outcome rejection reason drifted")?;
    require(response.next_actions == vec![action], "outcome rejection action drifted")
}

fn require_contract_reason(
    request: &RecordOutcomeRequest,
    reason: RecordOutcomeRejectionReason,
) -> TestResult {
    let error = request.validate().err().ok_or("invalid contract fixture was accepted")?;
    require(error.reason_code() == reason, "contract rejection reason drifted")
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
