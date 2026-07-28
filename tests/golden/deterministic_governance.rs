//! Golden corpus for deterministic governance and repository-local decision memory.

use std::collections::BTreeSet;

use canon_contracts::{ChallengeTier, Profile, VerificationKind};
use canon_engine::decision_memory::{
    ApprovalContent, ArtifactContent, BoundGovernanceBundle, ClaimContent, DecisionContent,
    DecisionMemoryError, DecisionMemoryGraph, DecisionMemoryNode, DecisionMemoryStore,
    DecisionMemoryStoreSnapshot, DependencyEdge, DependencyKind, EvidenceContent, FreshnessState,
    GovernanceBundleDraft, GovernanceDecision, GovernancePacketDraft, InsertOutcome, NodeEnvelope,
    NodeId, PacketContent, RiskAcceptanceContent, ValidationCode, ValidationPhase,
    VerificationRequirementContent, build_governance_bundle, project_decision_memory,
    propagate_stale, validate_governance_bundle,
};
use tempfile::tempdir;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn require(condition: bool, message: &str) -> TestResult {
    if condition { Ok(()) } else { Err(message.to_string().into()) }
}

fn packet(profile: Profile, id: &str) -> GovernancePacketDraft {
    GovernancePacketDraft {
        packet_id: id.to_string(),
        profile,
        change_intent: "preserve governed behavior".to_string(),
        scope: vec!["workspace".to_string()],
        risks: vec!["incorrect authorization".to_string()],
        invariants: vec!["Canon never executes semantic review".to_string()],
        acceptance_criteria: vec!["all exact bindings validate".to_string()],
        cross_packet_references: Vec::new(),
    }
}

fn draft(profile: Profile) -> GovernanceBundleDraft {
    GovernanceBundleDraft {
        bundle_id: "bundle-083".to_string(),
        profile,
        packets: vec![packet(profile, "packet-main")],
        required_approvers: vec!["release-owner".to_string()],
        authority_zone: "governance-release".to_string(),
        risk_tier: 2,
        change_class: "governance-kernel".to_string(),
        context_class: "repository-local".to_string(),
        owners: vec!["release-owner".to_string()],
        claims: vec!["claim-exact-binding".to_string()],
        required_evidence: vec![VerificationRequirementContent {
            requirement_id: NodeId::new("requirement-main"),
            packet_id: NodeId::new("packet-main"),
            claim_ids: vec![NodeId::new("claim-main")],
            kind: VerificationKind::ExternalSemanticReview,
            minimum_challenge_tier: ChallengeTier::Tier2,
        }],
        provided_evidence: vec![EvidenceContent {
            evidence_id: NodeId::new("evidence-main"),
            packet_id: NodeId::new("packet-main"),
            claim_ids: vec![NodeId::new("claim-main")],
            references: vec!["evidence://immutable/main".to_string()],
            lineage: "provider:challenger/executor:review/invocation:083".to_string(),
            independent_context_identity: "context-independent-083".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            external_semantic: true,
            fresh: true,
        }],
        approvals: vec![ApprovalContent {
            approval_id: NodeId::new("approval-main"),
            packet_id: NodeId::new("packet-main"),
            claim_ids: vec![NodeId::new("claim-main")],
            approver: "release-owner".to_string(),
            authority_zone: "governance-release".to_string(),
            approved: true,
            fresh: true,
        }],
        assumptions: vec!["published contract remains immutable".to_string()],
        alternatives: vec!["defer decision memory".to_string()],
        rationale: "exact graph bindings fail closed".to_string(),
        risk_acceptances: vec![RiskAcceptanceContent {
            acceptance_id: NodeId::new("risk-main"),
            packet_id: NodeId::new("packet-main"),
            owner: "release-owner".to_string(),
            risk: "bounded same-lineage degradation".to_string(),
            justification: "fixture exercises named acceptance".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            fresh: true,
        }],
        triggers: vec!["packet content changes".to_string()],
        no_change: false,
    }
}

fn graph_for(bundle: &BoundGovernanceBundle) -> Result<DecisionMemoryGraph, DecisionMemoryError> {
    DecisionMemoryGraph::from_bundle(bundle)
}

#[test]
fn structural_validation_fails_closed_and_is_deterministic() -> TestResult {
    let valid = build_governance_bundle(draft(Profile::Change))?;
    let graph = graph_for(&valid)?;
    let accepted = validate_governance_bundle(&valid, &graph);
    require(accepted.decision == GovernanceDecision::Accepted, "valid bundle was not accepted")?;
    require(
        accepted.phases
            == [
                ValidationPhase::Parse,
                ValidationPhase::Normalize,
                ValidationPhase::ValidateStructure,
                ValidationPhase::ValidateCrossReferences,
                ValidationPhase::ValidateAuthority,
                ValidationPhase::ValidateEvidenceRequirements,
                ValidationPhase::ValidateFreshness,
                ValidationPhase::RecordDecision,
                ValidationPhase::Project,
            ],
        "validation phases changed",
    )?;

    let mut malformed = draft(Profile::Change);
    malformed.bundle_id.clear();
    require(
        matches!(
            build_governance_bundle(malformed),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "missing identity did not fail closed",
    )?;

    let mut duplicates = draft(Profile::Change);
    duplicates.claims.push("claim-exact-binding".to_string());
    require(
        matches!(
            build_governance_bundle(duplicates),
            Err(DecisionMemoryError::DuplicateIdentity { .. })
        ),
        "duplicate claim did not fail closed",
    )?;

    let mut reordered = draft(Profile::Change);
    reordered.owners.reverse();
    reordered.required_approvers.reverse();
    let reordered = build_governance_bundle(reordered)?;
    require(
        valid.contract.bundle_digest == reordered.contract.bundle_digest,
        "set-like input order changed the canonical digest",
    )
}

#[test]
fn cross_packet_bindings_do_not_borrow_labels_evidence_or_authority() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    graph.remove_node(&NodeId::new("evidence-main"))?;
    let rejected = validate_governance_bundle(&bundle, &graph);
    require(
        rejected.decision == GovernanceDecision::RequiredMissing,
        "missing exact evidence binding was not rejected",
    )?;
    require(
        rejected.findings.iter().any(|finding| {
            finding.phase == ValidationPhase::ValidateCrossReferences
                && finding.code == ValidationCode::DanglingReference
        }),
        "missing evidence did not produce a dangling-reference finding",
    )?;

    let mut borrowed_draft = draft(Profile::Change);
    borrowed_draft.provided_evidence[0].packet_id = NodeId::new("same-display-name");
    let borrowed = build_governance_bundle(borrowed_draft)?;
    let borrowed_graph = graph_for(&borrowed)?;
    let rejected = validate_governance_bundle(&borrowed, &borrowed_graph);
    require(
        rejected.decision != GovernanceDecision::Accepted,
        "display-name equality replaced exact packet identity",
    )
}

#[test]
fn authority_is_exact_and_evidence_never_grants_it() -> TestResult {
    let mut insufficient = draft(Profile::Change);
    insufficient.approvals.clear();
    let bundle = build_governance_bundle(insufficient)?;
    let graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &graph);
    require(
        result.decision == GovernanceDecision::Blocked,
        "evidence incorrectly granted missing authority",
    )?;
    require(
        result.findings.iter().any(|finding| {
            finding.phase == ValidationPhase::ValidateAuthority
                && finding.code == ValidationCode::ApprovalMissing
        }),
        "missing exact approval finding was absent",
    )?;

    let mut automatic_tier_three = draft(Profile::Incident);
    automatic_tier_three.required_evidence[0].minimum_challenge_tier = ChallengeTier::Tier3;
    automatic_tier_three.provided_evidence[0].challenge_tier = ChallengeTier::Tier2;
    let bundle = build_governance_bundle(automatic_tier_three)?;
    let graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &graph);
    require(
        result.decision == GovernanceDecision::RequiredMissing,
        "Tier 3 was automatically overridden",
    )
}

#[test]
fn all_profiles_apply_exact_required_evidence_bindings() -> TestResult {
    let profiles = [
        Profile::Discovery,
        Profile::Requirements,
        Profile::Architecture,
        Profile::Backlog,
        Profile::Change,
        Profile::Refactor,
        Profile::Verification,
        Profile::PrReview,
        Profile::Incident,
    ];
    for profile in profiles {
        let bundle = build_governance_bundle(draft(profile))?;
        let graph = graph_for(&bundle)?;
        require(
            validate_governance_bundle(&bundle, &graph).decision == GovernanceDecision::Accepted,
            "profile rejected an exact evidence binding",
        )?;

        let mut wrong_tier = draft(profile);
        wrong_tier.provided_evidence[0].challenge_tier = ChallengeTier::Tier1;
        let bundle = build_governance_bundle(wrong_tier)?;
        let graph = graph_for(&bundle)?;
        require(
            validate_governance_bundle(&bundle, &graph).decision
                == GovernanceDecision::RequiredMissing,
            "profile accepted insufficient challenge tier",
        )?;
    }
    Ok(())
}

#[test]
fn stale_propagation_is_transitive_idempotent_and_branch_local() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let unrelated = NodeEnvelope::new(
        NodeId::new("artifact-unrelated"),
        "fixture",
        ArtifactContent {
            artifact_identity: "unrelated".to_string(),
            revision: "rev-1".to_string(),
            content_digest: "a".repeat(64),
        },
    )?;
    graph.insert_or_replay(DecisionMemoryNode::Artifact(unrelated))?;
    graph.add_edge(DependencyEdge::new(
        NodeId::new("packet-main"),
        NodeId::new("claim-main"),
        DependencyKind::FreshnessImpact,
    ))?;
    graph.add_edge(DependencyEdge::new(
        NodeId::new("claim-main"),
        NodeId::new("approval-main"),
        DependencyKind::FreshnessImpact,
    ))?;

    let first = propagate_stale(
        &mut graph,
        &NodeId::new("packet-main"),
        "f".repeat(64),
        "packet revision changed",
    )?;
    require(first.changed_nodes == 3, "direct/transitive stale count changed")?;
    require(
        graph.freshness(&NodeId::new("approval-main"))?.is_stale(),
        "transitive dependent remained fresh",
    )?;
    require(
        graph.freshness(&NodeId::new("artifact-unrelated"))? == &FreshnessState::Fresh,
        "unrelated branch became stale",
    )?;

    let repeated = propagate_stale(
        &mut graph,
        &NodeId::new("packet-main"),
        "f".repeat(64),
        "packet revision changed",
    )?;
    require(repeated.changed_nodes == 0, "repeated propagation was not idempotent")
}

#[test]
fn cycles_conflicts_and_replays_are_typed() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let packet = graph.node(&NodeId::new("packet-main"))?.clone();
    require(
        graph.insert_or_replay(packet.clone())? == InsertOutcome::Replayed,
        "same ID and digest was not idempotent",
    )?;

    let conflicting = DecisionMemoryNode::Packet(NodeEnvelope::new(
        NodeId::new("packet-main"),
        "fixture",
        PacketContent {
            profile: Profile::Change,
            packet_digest: "b".repeat(64),
            revision: 2,
            owner: "release-owner".to_string(),
        },
    )?);
    require(
        matches!(
            graph.insert_or_replay(conflicting),
            Err(DecisionMemoryError::ContentConflict { .. })
        ),
        "same ID with a different digest did not conflict",
    )?;

    graph.add_edge(DependencyEdge::new(
        NodeId::new("packet-main"),
        NodeId::new("claim-main"),
        DependencyKind::Authority,
    ))?;
    require(
        matches!(
            graph.add_edge(DependencyEdge::new(
                NodeId::new("claim-main"),
                NodeId::new("packet-main"),
                DependencyKind::Authority,
            )),
            Err(DecisionMemoryError::DependencyCycle { .. })
        ),
        "authority cycle was admitted",
    )
}

#[test]
fn history_round_trip_and_projection_preserve_governance_without_local_paths() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Architecture))?;
    let mut graph = graph_for(&bundle)?;
    let old_decision = graph.node(&NodeId::new("decision-bundle-083"))?.clone();
    let successor = DecisionMemoryNode::Decision(NodeEnvelope::new(
        NodeId::new("decision-bundle-083-v2"),
        "fixture",
        DecisionContent {
            bundle_id: "bundle-083".to_string(),
            rationale: "superseding rationale".to_string(),
            alternatives: vec!["retain prior decision".to_string()],
            assumptions: vec!["contract remains frozen".to_string()],
            triggers: vec!["new verified input".to_string()],
            owners: vec!["release-owner".to_string()],
            decision: GovernanceDecision::Accepted,
        },
    )?);
    graph.insert_or_replay(successor)?;
    graph.supersede(
        &old_decision.id().clone(),
        &NodeId::new("decision-bundle-083-v2"),
        "new verified input",
    )?;

    let result = validate_governance_bundle(&bundle, &graph);
    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));
    store
        .persist(&DecisionMemoryStoreSnapshot { graph: graph.clone(), terminal_result: result })?;
    let restored = store.load()?;
    require(restored.graph == graph, "persisted graph round-trip changed semantics")?;

    let projection = project_decision_memory(&restored.graph)?;
    let serialized = serde_json::to_string(&projection)?;
    require(
        !serialized.contains(workspace.path().to_string_lossy().as_ref()),
        "local path leaked",
    )?;
    require(
        projection.nodes.iter().any(|node| !node.fresh && node.supersedes.is_empty()),
        "historical stale decision was not inspectable",
    )
}

#[test]
fn seeded_input_order_has_one_projection_and_graph_digest() -> TestResult {
    let first_bundle = build_governance_bundle(draft(Profile::Verification))?;
    let mut reordered = draft(Profile::Verification);
    reordered.assumptions.reverse();
    reordered.alternatives.reverse();
    reordered.triggers.reverse();
    reordered.owners.reverse();
    reordered.claims.reverse();
    let second_bundle = build_governance_bundle(reordered)?;
    let first = graph_for(&first_bundle)?;
    let second = graph_for(&second_bundle)?;
    require(first.digest()? == second.digest()?, "equivalent graphs had different digests")?;
    require(
        project_decision_memory(&first)? == project_decision_memory(&second)?,
        "equivalent graphs had different projections",
    )
}

#[test]
fn governance_kernel_has_no_execution_capability_or_synthetic_evidence() -> TestResult {
    let source = include_str!("../../crates/canon-engine/src/decision_memory/mod.rs");
    for forbidden in [
        "std::process",
        "Command::",
        "reqwest",
        "ureq",
        "API_KEY",
        "TOKEN",
        "execute_semantic",
        "create_semantic_evidence",
    ] {
        require(!source.contains(forbidden), "decision-memory module gained execution capability")?;
    }

    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let graph = graph_for(&bundle)?;
    let evidence_ids = graph
        .nodes()
        .filter_map(|node| match node {
            DecisionMemoryNode::Evidence(value) => Some(value.id().clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    require(
        evidence_ids == BTreeSet::from([NodeId::new("evidence-main")]),
        "validation synthesized semantic evidence",
    )
}
