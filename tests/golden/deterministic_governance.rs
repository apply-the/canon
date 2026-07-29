//! Golden corpus for deterministic governance and repository-local decision memory.

use std::collections::BTreeSet;

use canon_contracts::{BundleDigest, ChallengeTier, Profile, VerificationKind};
use canon_engine::decision_memory::{
    ApprovalContent, ArtifactContent, BoundGovernanceBundle, ContentDigest, DecisionContent,
    DecisionMemoryError, DecisionMemoryGraph, DecisionMemoryNode, DecisionMemoryStore,
    DecisionMemoryStoreSnapshot, DependencyEdge, DependencyKind, EvidenceContent, FreshnessState,
    GovernanceBundleDraft, GovernanceDecision, GovernancePacketDraft, GovernanceValidationResult,
    InsertOutcome, NodeEnvelope, NodeId, PacketContent, RiskAcceptanceContent,
    SubjectArtifactBinding, ValidationCode, ValidationPhase, VerificationRequirementContent,
    build_governance_bundle, project_decision_memory, propagate_stale, validate_governance_bundle,
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
        revision: 1,
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
        decision_memory_revision: 1,
        packets: vec![packet(profile, "packet-main")],
        subject_artifacts: vec![SubjectArtifactBinding {
            artifact_id: NodeId::new("artifact-bundle-083-main"),
            packet_id: NodeId::new("packet-main"),
            content: ArtifactContent {
                artifact_identity: "workspace".to_string(),
                revision: "git:fixture-revision".to_string(),
                content_digest: "a".repeat(64),
            },
        }],
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
            claim_ids: vec![NodeId::new("claim-bundle-083-main")],
            artifact_ids: vec![NodeId::new("artifact-bundle-083-main")],
            kind: VerificationKind::ExternalSemanticReview,
            minimum_challenge_tier: ChallengeTier::Tier2,
            accepted_evidence_references: vec![format!("sha256:{}", "e".repeat(64))],
        }],
        provided_evidence: vec![EvidenceContent {
            evidence_id: NodeId::new("evidence-main"),
            packet_id: NodeId::new("packet-main"),
            claim_ids: vec![NodeId::new("claim-bundle-083-main")],
            artifact_ids: vec![NodeId::new("artifact-bundle-083-main")],
            requirement_ids: vec![NodeId::new("requirement-main")],
            references: vec![format!("sha256:{}", "e".repeat(64))],
            lineage: "provider:challenger/executor:review/invocation:083".to_string(),
            independent_context_identity: "context-independent-083".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            external_semantic: true,
            fresh: true,
            named_override: None,
        }],
        forbidden_lineages: vec!["implementer-lineage".to_string()],
        approvals: vec![ApprovalContent {
            approval_id: NodeId::new("approval-main"),
            packet_id: NodeId::new("packet-main"),
            claim_ids: vec![NodeId::new("claim-bundle-083-main")],
            artifact_ids: vec![NodeId::new("artifact-bundle-083-main")],
            evidence_ids: vec![NodeId::new("evidence-main")],
            requirement_ids: vec![NodeId::new("requirement-main")],
            approver: "release-owner".to_string(),
            authority_zone: "governance-release".to_string(),
            approved: true,
            decision_memory_revision: 1,
            valid_through_revision: Some(1),
            fresh: true,
        }],
        assumptions: vec!["published contract remains immutable".to_string()],
        alternatives: vec!["defer decision memory".to_string()],
        rationale: "exact graph bindings fail closed".to_string(),
        risk_acceptances: vec![RiskAcceptanceContent {
            acceptance_id: NodeId::new("risk-main"),
            packet_id: NodeId::new("packet-main"),
            owner: "release-owner".to_string(),
            risk: "same-lineage:implementer-lineage".to_string(),
            justification: "fixture exercises named acceptance".to_string(),
            challenge_tier: ChallengeTier::Tier2,
            lineage: "implementer-lineage".to_string(),
            approval_id: NodeId::new("approval-main"),
            fresh: true,
        }],
        triggers: vec!["packet content changes".to_string()],
        no_change: false,
    }
}

fn graph_for(bundle: &BoundGovernanceBundle) -> Result<DecisionMemoryGraph, DecisionMemoryError> {
    DecisionMemoryGraph::from_bundle(bundle)
}

fn decision_for(draft: GovernanceBundleDraft) -> Result<GovernanceDecision, DecisionMemoryError> {
    let bundle = build_governance_bundle(draft)?;
    let mut graph = graph_for(&bundle)?;
    Ok(validate_governance_bundle(&bundle, &mut graph).decision)
}

fn require_draft_rejected(draft: GovernanceBundleDraft, message: &str) -> TestResult {
    require(build_governance_bundle(draft).is_err(), message)
}

fn require_snapshot_rejected(
    store: &DecisionMemoryStore,
    admitted_bundles: Vec<BoundGovernanceBundle>,
    graph: DecisionMemoryGraph,
    mut terminal_result: GovernanceValidationResult,
    message: &str,
) -> TestResult {
    terminal_result.graph_digest = graph.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles,
                graph,
                terminal_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        message,
    )
}

fn exact_profile_draft(profile: Profile) -> GovernanceBundleDraft {
    let mut value = draft(profile);
    if profile == Profile::Incident {
        value.required_evidence[0].minimum_challenge_tier = ChallengeTier::Tier3;
        value.provided_evidence[0].challenge_tier = ChallengeTier::Tier3;
    }
    value
}

fn retarget_draft(mut value: GovernanceBundleDraft, suffix: &str) -> GovernanceBundleDraft {
    let packet_id = NodeId::new(format!("packet-{suffix}"));
    let artifact_id = NodeId::new(format!("artifact-bundle-{suffix}-main"));
    let claim_id = NodeId::new(format!("claim-bundle-{suffix}-main"));
    let requirement_id = NodeId::new(format!("requirement-{suffix}"));
    let evidence_id = NodeId::new(format!("evidence-{suffix}"));
    let approval_id = NodeId::new(format!("approval-{suffix}"));
    value.bundle_id = format!("bundle-{suffix}");
    value.packets[0].packet_id = packet_id.to_string();
    value.subject_artifacts[0].artifact_id = artifact_id.clone();
    value.subject_artifacts[0].packet_id = packet_id.clone();
    value.subject_artifacts[0].content.content_digest = "b".repeat(64);
    value.required_evidence[0].requirement_id = requirement_id.clone();
    value.required_evidence[0].packet_id = packet_id.clone();
    value.required_evidence[0].claim_ids = vec![claim_id.clone()];
    value.required_evidence[0].artifact_ids = vec![artifact_id.clone()];
    value.provided_evidence[0].evidence_id = evidence_id.clone();
    value.provided_evidence[0].packet_id = packet_id.clone();
    value.provided_evidence[0].claim_ids = vec![claim_id.clone()];
    value.provided_evidence[0].artifact_ids = vec![artifact_id.clone()];
    value.provided_evidence[0].requirement_ids = vec![requirement_id.clone()];
    value.approvals[0].approval_id = approval_id.clone();
    value.approvals[0].packet_id = packet_id.clone();
    value.approvals[0].claim_ids = vec![claim_id];
    value.approvals[0].artifact_ids = vec![artifact_id];
    value.approvals[0].evidence_ids = vec![evidence_id];
    value.approvals[0].requirement_ids = vec![requirement_id];
    value.risk_acceptances[0].acceptance_id = NodeId::new(format!("risk-{suffix}"));
    value.risk_acceptances[0].packet_id = packet_id;
    value.risk_acceptances[0].approval_id = approval_id;
    value
}

fn changed_digest(node: &DecisionMemoryNode) -> Result<ContentDigest, DecisionMemoryError> {
    match node {
        DecisionMemoryNode::Packet(value) => {
            let mut content = value.content.clone();
            content.revision = content.revision.saturating_add(1);
            Ok(NodeEnvelope::new(value.id().clone(), "governance-packet", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Artifact(value) => {
            let mut content = value.content.clone();
            content.content_digest = "b".repeat(64);
            Ok(NodeEnvelope::new(value.id().clone(), "governance-subject-artifact", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Claim(value) => {
            let mut content = value.content.clone();
            content.statement.push_str("-changed");
            Ok(NodeEnvelope::new(value.id().clone(), "governance-claim", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Evidence(value) => {
            let mut content = value.content.clone();
            content.lineage.push_str("/changed");
            content.challenge_tier = ChallengeTier::Tier3;
            Ok(NodeEnvelope::new(value.id().clone(), "external-evidence", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Approval(value) => {
            let mut content = value.content.clone();
            content.approver = "replacement-owner".to_string();
            Ok(NodeEnvelope::new(value.id().clone(), "named-authority", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::VerificationRequirement(value) => {
            let mut content = value.content.clone();
            content.minimum_challenge_tier = ChallengeTier::Tier3;
            Ok(NodeEnvelope::new(value.id().clone(), "verification-policy", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Assumption(value) => {
            let mut content = value.content.clone();
            content.owner = "replacement-owner".to_string();
            Ok(NodeEnvelope::new(value.id().clone(), "governance-bundle", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Alternative(value) => {
            let mut content = value.content.clone();
            content.disposition = "selected".to_string();
            Ok(NodeEnvelope::new(value.id().clone(), "governance-bundle", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Trigger(value) => {
            let mut content = value.content.clone();
            content.owner = "replacement-owner".to_string();
            Ok(NodeEnvelope::new(value.id().clone(), "governance-bundle", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::RiskAcceptance(value) => {
            let mut content = value.content.clone();
            content.owner = "replacement-owner".to_string();
            Ok(NodeEnvelope::new(value.id().clone(), "named-risk-acceptance", content)?
                .content_digest()
                .clone())
        }
        DecisionMemoryNode::Decision(value) => {
            let mut content = value.content.clone();
            content.rationale.push_str("-changed");
            content.owners = vec!["replacement-owner".to_string()];
            Ok(NodeEnvelope::new(value.id().clone(), "deterministic-governance", content)?
                .content_digest()
                .clone())
        }
    }
}

#[test]
fn structural_validation_fails_closed_and_is_deterministic() -> TestResult {
    let valid = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&valid)?;
    let accepted = validate_governance_bundle(&valid, &mut graph);
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

    let mut missing_packet_id = draft(Profile::Change);
    missing_packet_id.packets[0].packet_id.clear();
    require(
        matches!(
            build_governance_bundle(missing_packet_id),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "missing packet identity did not fail closed",
    )?;

    let mut empty_requirement_scope = draft(Profile::Change);
    empty_requirement_scope.required_evidence[0].artifact_ids.clear();
    require(
        matches!(
            build_governance_bundle(empty_requirement_scope),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "empty required artifact set did not fail closed",
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

    let mut duplicate_evidence = draft(Profile::Change);
    duplicate_evidence.provided_evidence.push(duplicate_evidence.provided_evidence[0].clone());
    require(
        matches!(
            build_governance_bundle(duplicate_evidence),
            Err(DecisionMemoryError::DuplicateIdentity { .. })
        ),
        "duplicate evidence did not fail closed",
    )?;

    let mut duplicate_approval = draft(Profile::Change);
    duplicate_approval.approvals.push(duplicate_approval.approvals[0].clone());
    require(
        matches!(
            build_governance_bundle(duplicate_approval),
            Err(DecisionMemoryError::DuplicateIdentity { .. })
        ),
        "duplicate approval did not fail closed",
    )?;

    let mut unsupported = graph_for(&valid)?;
    unsupported.contract_line = "canon-contracts/unknown".to_string();
    require(
        validate_governance_bundle(&valid, &mut unsupported).decision
            == GovernanceDecision::Unsupported,
        "unknown contract line did not fail closed",
    )?;

    let mut malformed_bundle = valid.clone();
    malformed_bundle.contract.bundle_digest = BundleDigest::new("not-a-digest");
    let mut graph = graph_for(&valid)?;
    require(
        validate_governance_bundle(&malformed_bundle, &mut graph).decision
            == GovernanceDecision::Unsupported,
        "malformed bundle digest did not fail closed",
    )?;

    let mut forged_bundle = valid.clone();
    forged_bundle.contract.packets[0].change_intent = "forged after digest".to_string();
    let mut graph = graph_for(&valid)?;
    require(
        validate_governance_bundle(&forged_bundle, &mut graph).decision
            == GovernanceDecision::Unsupported,
        "valid-looking stale bundle digest was not recomputed",
    )?;

    let mut unknown_authority = draft(Profile::Change);
    unknown_authority.authority_zone = "authority-expanding-unknown".to_string();
    require(
        matches!(
            build_governance_bundle(unknown_authority),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "unknown authority-expanding value was admitted",
    )?;

    let mut zero_revision = draft(Profile::Change);
    zero_revision.decision_memory_revision = 0;
    require_draft_rejected(zero_revision, "zero decision-memory revision was admitted")?;

    let mut unknown_change_class = draft(Profile::Change);
    unknown_change_class.change_class = "unknown-change-class".to_string();
    require_draft_rejected(unknown_change_class, "unknown change class was admitted")?;

    let mut unknown_context_class = draft(Profile::Change);
    unknown_context_class.context_class = "unknown-context-class".to_string();
    require_draft_rejected(unknown_context_class, "unknown context class was admitted")?;

    let mut excessive_risk = draft(Profile::Change);
    excessive_risk.risk_tier = 4;
    require_draft_rejected(excessive_risk, "out-of-range risk tier was admitted")?;

    let mut profile_mismatch = draft(Profile::Change);
    profile_mismatch.packets[0].profile = Profile::Architecture;
    require_draft_rejected(profile_mismatch, "primary packet profile mismatch was admitted")?;

    let mut zero_packet_revision = draft(Profile::Change);
    zero_packet_revision.packets[0].revision = 0;
    require_draft_rejected(zero_packet_revision, "zero packet revision was admitted")?;

    for (message, field) in [
        ("blank packet intent was admitted", "intent"),
        ("empty packet scope was admitted", "scope"),
        ("empty packet risks were admitted", "risks"),
        ("empty packet invariants were admitted", "invariants"),
        ("empty packet acceptance criteria were admitted", "acceptance"),
    ] {
        let mut invalid = draft(Profile::Change);
        match field {
            "intent" => invalid.packets[0].change_intent.clear(),
            "scope" => invalid.packets[0].scope.clear(),
            "risks" => invalid.packets[0].risks.clear(),
            "invariants" => invalid.packets[0].invariants.clear(),
            "acceptance" => invalid.packets[0].acceptance_criteria.clear(),
            _ => return Err("unknown structural fixture field".into()),
        }
        require_draft_rejected(invalid, message)?;
    }

    let mut empty_required_claims = draft(Profile::Change);
    empty_required_claims.required_evidence[0].claim_ids.clear();
    require_draft_rejected(empty_required_claims, "empty required claim set was admitted")?;

    let mut empty_accepted_references = draft(Profile::Change);
    empty_accepted_references.required_evidence[0].accepted_evidence_references.clear();
    require_draft_rejected(
        empty_accepted_references,
        "empty accepted-evidence reference set was admitted",
    )?;

    let mut duplicate_reference = draft(Profile::Change);
    let repeated_reference =
        duplicate_reference.required_evidence[0].accepted_evidence_references[0].clone();
    duplicate_reference.required_evidence[0].accepted_evidence_references.push(repeated_reference);
    require_draft_rejected(duplicate_reference, "duplicate evidence reference was admitted")?;

    let mut duplicate_approver = draft(Profile::Change);
    duplicate_approver.required_approvers.push("release-owner".to_string());
    require_draft_rejected(duplicate_approver, "duplicate required approver was admitted")?;

    let mut duplicate_owner = draft(Profile::Change);
    duplicate_owner.owners.push("release-owner".to_string());
    require_draft_rejected(duplicate_owner, "duplicate decision owner was admitted")?;

    let mut duplicate_lineage = draft(Profile::Change);
    duplicate_lineage.forbidden_lineages.push("implementer-lineage".to_string());
    require_draft_rejected(duplicate_lineage, "duplicate forbidden lineage was admitted")?;

    let mut duplicate_requirement = draft(Profile::Change);
    duplicate_requirement
        .required_evidence
        .push(duplicate_requirement.required_evidence[0].clone());
    require_draft_rejected(duplicate_requirement, "duplicate requirement identity was admitted")?;

    let mut duplicate_artifact = draft(Profile::Change);
    duplicate_artifact.subject_artifacts.push(duplicate_artifact.subject_artifacts[0].clone());
    require_draft_rejected(duplicate_artifact, "duplicate artifact identity was admitted")?;

    let mut duplicate_risk = draft(Profile::Change);
    duplicate_risk.risk_acceptances.push(duplicate_risk.risk_acceptances[0].clone());
    require_draft_rejected(duplicate_risk, "duplicate risk acceptance was admitted")?;

    for (message, field) in [
        ("blank risk lineage was admitted", "lineage"),
        ("blank accepted risk was admitted", "risk"),
        ("blank risk justification was admitted", "justification"),
    ] {
        let mut invalid = draft(Profile::Change);
        match field {
            "lineage" => invalid.risk_acceptances[0].lineage.clear(),
            "risk" => invalid.risk_acceptances[0].risk.clear(),
            "justification" => invalid.risk_acceptances[0].justification.clear(),
            _ => return Err("unknown risk fixture field".into()),
        }
        require_draft_rejected(invalid, message)?;
    }

    let mut unknown_packet_reference = draft(Profile::Change);
    unknown_packet_reference.packets[0].cross_packet_references.push("packet-unknown".to_string());
    require_draft_rejected(unknown_packet_reference, "unknown packet reference was admitted")?;

    let mut unknown_claim_binding = draft(Profile::Change);
    unknown_claim_binding.required_evidence[0].claim_ids = vec![NodeId::new("claim-unknown")];
    require_draft_rejected(unknown_claim_binding, "unknown claim binding was admitted")?;

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
    let mut two_packet = draft(Profile::Change);
    two_packet.packets.push(packet(Profile::Change, "packet-secondary"));
    two_packet.packets[0].cross_packet_references.push("packet-secondary".to_string());
    let exact = build_governance_bundle(two_packet)?;
    let mut exact_graph = graph_for(&exact)?;
    require(
        validate_governance_bundle(&exact, &mut exact_graph).decision
            == GovernanceDecision::Accepted,
        "exact compatible cross-packet binding was rejected",
    )?;

    let mut incompatible = draft(Profile::Change);
    incompatible.packets.push(packet(Profile::Architecture, "packet-secondary"));
    incompatible.packets[0].cross_packet_references.push("packet-secondary".to_string());
    require(
        matches!(
            build_governance_bundle(incompatible),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "incompatible cross-profile packet reuse was admitted",
    )?;

    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    graph.remove_node(&NodeId::new("evidence-main"))?;
    let rejected = validate_governance_bundle(&bundle, &mut graph);
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
    require(
        !rejected.phases.contains(&ValidationPhase::ValidateAuthority),
        "validation continued into authority after cross-reference failure",
    )?;

    let mut borrowed_draft = draft(Profile::Change);
    borrowed_draft.provided_evidence[0].packet_id = NodeId::new("same-display-name");
    require(
        matches!(
            build_governance_bundle(borrowed_draft),
            Err(DecisionMemoryError::DanglingReference { .. })
        ),
        "display-name equality replaced exact packet identity",
    )?;

    let mut borrowed_artifact = draft(Profile::Change);
    borrowed_artifact.packets.push(packet(Profile::Change, "packet-secondary"));
    borrowed_artifact.subject_artifacts.push(SubjectArtifactBinding {
        artifact_id: NodeId::new("artifact-secondary"),
        packet_id: NodeId::new("packet-secondary"),
        content: ArtifactContent {
            artifact_identity: "secondary".to_string(),
            revision: "git:secondary".to_string(),
            content_digest: "b".repeat(64),
        },
    });
    borrowed_artifact.provided_evidence[0].artifact_ids = vec![NodeId::new("artifact-secondary")];
    require(
        matches!(
            build_governance_bundle(borrowed_artifact),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "evidence borrowed an artifact across packet boundaries",
    )
}

#[test]
fn authority_is_exact_and_evidence_never_grants_it() -> TestResult {
    let mut insufficient = draft(Profile::Change);
    insufficient.approvals.clear();
    insufficient.risk_acceptances.clear();
    let bundle = build_governance_bundle(insufficient)?;
    let mut graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &mut graph);
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
    let mut graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &mut graph);
    require(
        result.decision == GovernanceDecision::RequiredMissing,
        "Tier 3 was automatically overridden",
    )?;

    let mut wrong_scope = draft(Profile::Change);
    wrong_scope.approvals[0].claim_ids.clear();
    let bundle = build_governance_bundle(wrong_scope)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision == GovernanceDecision::Blocked,
        "approval with the wrong claim scope granted authority",
    )?;

    let mut wrong_artifact = draft(Profile::Change);
    wrong_artifact.subject_artifacts.push(SubjectArtifactBinding {
        artifact_id: NodeId::new("artifact-unreviewed"),
        packet_id: NodeId::new("packet-main"),
        content: ArtifactContent {
            artifact_identity: "unreviewed".to_string(),
            revision: "git:unreviewed".to_string(),
            content_digest: "b".repeat(64),
        },
    });
    wrong_artifact.approvals[0].artifact_ids = vec![NodeId::new("artifact-unreviewed")];
    let bundle = build_governance_bundle(wrong_artifact)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision == GovernanceDecision::Blocked,
        "approval for a different artifact granted authority",
    )?;

    let mut stale = draft(Profile::Change);
    stale.approvals[0].fresh = false;
    let bundle = build_governance_bundle(stale)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision == GovernanceDecision::Stale,
        "stale approval was not classified as stale",
    )?;

    let mut expired = draft(Profile::Change);
    expired.approvals[0].valid_through_revision = Some(0);
    let bundle = build_governance_bundle(expired)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision == GovernanceDecision::Blocked,
        "expired approval granted authority",
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
        let exact = exact_profile_draft(profile);
        let bundle = build_governance_bundle(exact)?;
        let mut graph = graph_for(&bundle)?;
        require(
            validate_governance_bundle(&bundle, &mut graph).decision
                == GovernanceDecision::Accepted,
            "profile rejected an exact evidence binding",
        )?;

        let mut wrong_tier = exact_profile_draft(profile);
        wrong_tier.provided_evidence[0].challenge_tier = ChallengeTier::Tier1;
        let bundle = build_governance_bundle(wrong_tier)?;
        let mut graph = graph_for(&bundle)?;
        require(
            validate_governance_bundle(&bundle, &mut graph).decision
                == GovernanceDecision::RequiredMissing,
            "profile accepted insufficient challenge tier",
        )?;

        let mut missing = exact_profile_draft(profile);
        missing.provided_evidence.clear();
        missing.approvals[0].evidence_ids.clear();
        require(
            decision_for(missing)? == GovernanceDecision::RequiredMissing,
            "profile accepted missing evidence",
        )?;

        let mut wrong_reference = exact_profile_draft(profile);
        wrong_reference.provided_evidence[0].references =
            vec![format!("sha256:{}", "f".repeat(64))];
        require(
            decision_for(wrong_reference)? == GovernanceDecision::RequiredMissing,
            "profile accepted evidence bound to the wrong digest",
        )?;

        let mut stale = exact_profile_draft(profile);
        stale.provided_evidence[0].fresh = false;
        require(
            decision_for(stale)? == GovernanceDecision::Stale,
            "profile accepted stale evidence",
        )?;

        let mut label_only = exact_profile_draft(profile);
        label_only.provided_evidence[0].references = vec!["review-passed".to_string()];
        require(
            decision_for(label_only)? == GovernanceDecision::RequiredMissing,
            "profile accepted label-only evidence",
        )?;

        let mut claim_subset = exact_profile_draft(profile);
        claim_subset.provided_evidence[0].claim_ids.clear();
        require(
            decision_for(claim_subset)? == GovernanceDecision::RequiredMissing,
            "profile accepted a claim subset as an exact match",
        )?;

        let mut forbidden_lineage = exact_profile_draft(profile);
        forbidden_lineage.provided_evidence[0].lineage =
            "provider:implementer-lineage/executor:review/invocation:083".to_string();
        require(
            decision_for(forbidden_lineage)? == GovernanceDecision::RequiredMissing,
            "profile accepted forbidden shared lineage",
        )?;
    }
    Ok(())
}

#[test]
fn evidence_rejects_labels_shared_lineage_and_unscoped_overrides() -> TestResult {
    let mut wrong_artifact = draft(Profile::Verification);
    wrong_artifact.subject_artifacts.push(SubjectArtifactBinding {
        artifact_id: NodeId::new("artifact-unreviewed"),
        packet_id: NodeId::new("packet-main"),
        content: ArtifactContent {
            artifact_identity: "unreviewed".to_string(),
            revision: "git:unreviewed".to_string(),
            content_digest: "b".repeat(64),
        },
    });
    wrong_artifact.provided_evidence[0].artifact_ids = vec![NodeId::new("artifact-unreviewed")];
    wrong_artifact.approvals[0].artifact_ids =
        vec![NodeId::new("artifact-bundle-083-main"), NodeId::new("artifact-unreviewed")];
    let bundle = build_governance_bundle(wrong_artifact)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "evidence for a different exact artifact satisfied the requirement",
    )?;

    let mut label_only = draft(Profile::Verification);
    label_only.provided_evidence[0].references = vec!["passed-review".to_string()];
    let bundle = build_governance_bundle(label_only)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "label-only evidence satisfied a requirement",
    )?;

    let mut wrong_digest = draft(Profile::Verification);
    wrong_digest.provided_evidence[0].references = vec![format!("sha256:{}", "f".repeat(64))];
    let bundle = build_governance_bundle(wrong_digest)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "wrong digest-shaped evidence borrowed the admitted reference",
    )?;

    let mut downgraded = draft(Profile::Change);
    downgraded.required_evidence[0].kind = VerificationKind::Deterministic;
    downgraded.required_evidence[0].minimum_challenge_tier = ChallengeTier::Tier0;
    downgraded.provided_evidence[0].external_semantic = false;
    downgraded.provided_evidence[0].challenge_tier = ChallengeTier::Tier0;
    let bundle = build_governance_bundle(downgraded)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "caller downgraded Change policy to deterministic Tier 0",
    )?;

    let shared_lineage = "provider:implementer-lineage/executor:review/invocation:083";
    let mut shared = draft(Profile::Verification);
    shared.provided_evidence[0].lineage = shared_lineage.to_string();
    let bundle = build_governance_bundle(shared)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "shared lineage was treated as independent",
    )?;

    let mut unrelated_override = draft(Profile::Verification);
    unrelated_override.provided_evidence[0].lineage = shared_lineage.to_string();
    unrelated_override.provided_evidence[0].named_override = Some("risk-main".to_string());
    unrelated_override.risk_acceptances[0].lineage = "other-lineage".to_string();
    unrelated_override.risk_acceptances[0].risk = "same-lineage:other-lineage".to_string();
    let bundle = build_governance_bundle(unrelated_override)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "unrelated risk acceptance waived a forbidden lineage",
    )?;

    let mut wrong_scope_override = draft(Profile::Verification);
    wrong_scope_override.provided_evidence[0].lineage = shared_lineage.to_string();
    wrong_scope_override.provided_evidence[0].named_override = Some("risk-main".to_string());
    let mut override_approval = wrong_scope_override.approvals[0].clone();
    override_approval.approval_id = NodeId::new("approval-override-wrong-scope");
    override_approval.artifact_ids.clear();
    override_approval.evidence_ids.clear();
    override_approval.requirement_ids.clear();
    wrong_scope_override.approvals.push(override_approval);
    wrong_scope_override.risk_acceptances[0].approval_id =
        NodeId::new("approval-override-wrong-scope");
    let bundle = build_governance_bundle(wrong_scope_override)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "wrong-scope approval authorized a same-lineage override",
    )?;

    let mut named_tier_two = draft(Profile::Verification);
    named_tier_two.provided_evidence[0].lineage = shared_lineage.to_string();
    named_tier_two.provided_evidence[0].named_override = Some("risk-main".to_string());
    let bundle = build_governance_bundle(named_tier_two)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision == GovernanceDecision::Accepted,
        "named Tier 2 risk acceptance was not honored",
    )?;

    let mut tier_three = draft(Profile::Incident);
    tier_three.required_evidence[0].minimum_challenge_tier = ChallengeTier::Tier3;
    tier_three.provided_evidence[0].challenge_tier = ChallengeTier::Tier3;
    tier_three.provided_evidence[0].lineage = shared_lineage.to_string();
    tier_three.provided_evidence[0].named_override = Some("risk-main".to_string());
    let bundle = build_governance_bundle(tier_three)?;
    let mut graph = graph_for(&bundle)?;
    require(
        validate_governance_bundle(&bundle, &mut graph).decision
            == GovernanceDecision::RequiredMissing,
        "Tier 3 shared-lineage evidence received an automatic override",
    )
}

#[test]
fn deterministic_only_and_no_change_remain_explicit_governance_cases() -> TestResult {
    for profile in [Profile::Discovery, Profile::Backlog, Profile::Verification] {
        let mut deterministic = draft(profile);
        deterministic.no_change = true;
        deterministic.risk_tier = 0;
        deterministic.required_evidence[0].kind = VerificationKind::Deterministic;
        deterministic.required_evidence[0].minimum_challenge_tier = ChallengeTier::Tier0;
        deterministic.provided_evidence[0].external_semantic = false;
        deterministic.provided_evidence[0].challenge_tier = ChallengeTier::Tier0;
        deterministic.provided_evidence[0].lineage.clear();
        deterministic.provided_evidence[0].independent_context_identity.clear();
        let bundle = build_governance_bundle(deterministic)?;
        let mut graph = graph_for(&bundle)?;
        require(bundle.metadata.no_change, "no-change event was not preserved")?;
        require(
            validate_governance_bundle(&bundle, &mut graph).decision
                == GovernanceDecision::Accepted,
            "permitted deterministic-only no-change case was rejected",
        )?;
    }
    for profile in [
        Profile::Requirements,
        Profile::Architecture,
        Profile::Change,
        Profile::Refactor,
        Profile::PrReview,
        Profile::Incident,
    ] {
        let mut forbidden = draft(profile);
        forbidden.no_change = true;
        forbidden.risk_tier = 0;
        forbidden.required_evidence[0].kind = VerificationKind::Deterministic;
        forbidden.required_evidence[0].minimum_challenge_tier = ChallengeTier::Tier0;
        forbidden.provided_evidence[0].external_semantic = false;
        forbidden.provided_evidence[0].challenge_tier = ChallengeTier::Tier0;
        forbidden.provided_evidence[0].lineage.clear();
        forbidden.provided_evidence[0].independent_context_identity.clear();
        require(
            decision_for(forbidden)? == GovernanceDecision::RequiredMissing,
            "profile admitted deterministic-only no-change without policy permission",
        )?;
    }
    Ok(())
}

#[test]
fn stale_propagation_is_transitive_idempotent_and_branch_local() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let accepted = validate_governance_bundle(&bundle, &mut graph);
    require(
        accepted.decision == GovernanceDecision::Accepted,
        "stale fixture did not begin authorized",
    )?;
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
    let first = propagate_stale(
        &mut graph,
        &NodeId::new("packet-main"),
        "f".repeat(64),
        "packet revision changed",
    )?;
    require(first.changed_nodes == 8, "direct/transitive stale count changed")?;
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
    require(repeated.changed_nodes == 0, "repeated propagation was not idempotent")?;

    let mut persisted_graph = graph_for(&bundle)?;
    validate_governance_bundle(&bundle, &mut persisted_graph);
    propagate_stale(
        &mut persisted_graph,
        &NodeId::new("packet-main"),
        "f".repeat(64),
        "packet revision changed",
    )?;
    let stale_result = validate_governance_bundle(&bundle, &mut persisted_graph);
    require(
        stale_result.decision == GovernanceDecision::Stale,
        &format!(
            "stale dependencies did not produce a persistable stale decision: {stale_result:?}"
        ),
    )?;
    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));
    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles: vec![bundle],
        graph: persisted_graph,
        terminal_result: stale_result,
    })?;
    require(
        store.load()?.terminal_result.decision == GovernanceDecision::Stale,
        "stale terminal result did not survive persistence",
    )
}

#[test]
fn every_normative_dependency_change_invalidates_the_current_decision() -> TestResult {
    let changed_nodes = [
        "packet-main",
        "artifact-bundle-083-main",
        "claim-bundle-083-main",
        "evidence-main",
        "requirement-main",
        "approval-main",
        "risk-main",
        "assumption-bundle-083-0001",
        "alternative-bundle-083-0001",
        "trigger-bundle-083-0001",
        "decision-bundle-083-r1-accepted",
    ];
    for changed_node in changed_nodes {
        let bundle = build_governance_bundle(draft(Profile::Change))?;
        let mut graph = graph_for(&bundle)?;
        let result = validate_governance_bundle(&bundle, &mut graph);
        require(
            result.decision == GovernanceDecision::Accepted,
            "dependency fixture did not begin authorized",
        )?;
        let node_id = NodeId::new(changed_node);
        let node = graph.node(&node_id)?;
        let recorded_digest = node.content_digest().sha256.clone();
        let observed_digest = changed_digest(node)?.sha256;
        let unchanged =
            propagate_stale(&mut graph, &node_id, recorded_digest.clone(), "same content")?;
        require(unchanged.changed_nodes == 0, "same digest created false staleness")?;

        propagate_stale(&mut graph, &node_id, observed_digest, "exact binding changed")?;
        require(
            graph.freshness(&NodeId::new("decision-bundle-083-r1-accepted"))?.is_stale(),
            "normative dependency change left the current decision fresh",
        )?;
        let restored_label =
            propagate_stale(&mut graph, &node_id, recorded_digest, "display text restored")?;
        require(
            restored_label.changed_nodes == 0
                && graph.freshness(&NodeId::new("decision-bundle-083-r1-accepted"))?.is_stale(),
            "display-text restoration revived stale authority",
        )?;
    }

    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    validate_governance_bundle(&bundle, &mut graph);
    propagate_stale(
        &mut graph,
        &NodeId::new("approval-main"),
        "c".repeat(64),
        "approval input changed",
    )?;
    let replacement = bundle.metadata.provided_evidence[0].clone();
    let mut replacement = replacement;
    replacement.evidence_id = NodeId::new("evidence-reissued");
    graph.insert_or_replay(DecisionMemoryNode::Evidence(NodeEnvelope::new(
        replacement.evidence_id.clone(),
        "external-evidence",
        replacement,
    )?))?;
    require(
        graph.freshness(&NodeId::new("approval-main"))?.is_stale(),
        "new evidence revived an old approval without reissue",
    )
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

    let malformed = DecisionMemoryNode::Packet(NodeEnvelope::new(
        NodeId::new("packet-malformed"),
        "fixture",
        PacketContent {
            profile: Profile::Change,
            packet_digest: "not-a-digest".to_string(),
            revision: 1,
            owner: "release-owner".to_string(),
        },
    )?);
    require(
        matches!(
            graph.insert_or_replay(malformed),
            Err(DecisionMemoryError::MalformedDigest { .. })
        ),
        "malformed packet digest was admitted",
    )?;

    require(
        matches!(
            graph.add_edge(DependencyEdge::new(
                NodeId::new("claim-bundle-083-main"),
                NodeId::new("packet-main"),
                DependencyKind::Authority,
            )),
            Err(DecisionMemoryError::DependencyCycle { .. })
        ),
        "authority cycle was admitted",
    )?;

    let mut serialized = serde_json::to_value(&graph)?;
    let edges = serialized
        .get_mut("edges")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "serialized graph did not contain edges".to_string())?;
    edges.push(serde_json::to_value(DependencyEdge::new(
        NodeId::new("claim-bundle-083-main"),
        NodeId::new("packet-main"),
        DependencyKind::Authority,
    ))?);
    let mut cyclic: DecisionMemoryGraph = serde_json::from_value(serialized)?;
    let result = validate_governance_bundle(&bundle, &mut cyclic);
    require(
        result.decision == GovernanceDecision::Unsupported
            && !result.phases.contains(&ValidationPhase::RecordDecision),
        "deserialized dependency cycle reached a terminal success phase",
    )?;

    let mut missing_edge_value = serde_json::to_value(&graph)?;
    let missing_edge_list = missing_edge_value
        .get_mut("edges")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "serialized graph did not contain an edge list".to_string())?;
    require(missing_edge_list.pop().is_some(), "fixture graph had no edge to remove")?;
    let mut missing_edge: DecisionMemoryGraph = serde_json::from_value(missing_edge_value)?;
    require(
        validate_governance_bundle(&bundle, &mut missing_edge).decision
            == GovernanceDecision::Unsupported,
        "graph with a missing normative edge was admitted",
    )?;

    let mut tampered_value = serde_json::to_value(&graph)?;
    let owner = tampered_value
        .get_mut("nodes")
        .and_then(|nodes| nodes.get_mut("packet-main"))
        .and_then(|packet| packet.get_mut("node"))
        .and_then(|node| node.get_mut("content"))
        .and_then(|content| content.get_mut("owner"))
        .ok_or_else(|| "serialized packet owner was absent".to_string())?;
    *owner = serde_json::Value::String("tampered-owner".to_string());
    let mut tampered: DecisionMemoryGraph = serde_json::from_value(tampered_value)?;
    require(
        validate_governance_bundle(&bundle, &mut tampered).decision
            == GovernanceDecision::Unsupported,
        "node content changed without a new digest was admitted",
    )?;

    require(
        matches!(
            ContentDigest::from_sha256("deterministic-governance", "not-a-digest"),
            Err(DecisionMemoryError::MalformedDigest { .. })
        ),
        "malformed typed content digest was admitted",
    )?;
    require(
        matches!(
            propagate_stale(
                &mut graph,
                &NodeId::new("packet-main"),
                "not-a-digest",
                "invalid digest fixture",
            ),
            Err(DecisionMemoryError::MalformedDigest { .. })
        ),
        "stale propagation admitted a malformed observed digest",
    )?;
    require(
        DecisionMemoryGraph::default() == DecisionMemoryGraph::new(),
        "default decision-memory graph changed its schema identity",
    )?;
    let mut deletion_fixture = graph.clone();
    deletion_fixture.remove_node(&NodeId::new("packet-main"))?;
    require(
        validate_governance_bundle(&bundle, &mut deletion_fixture).decision
            == GovernanceDecision::RequiredMissing,
        "referenced-node deletion did not remain detectably invalid",
    )?;
    require(
        matches!(
            graph.add_edge(DependencyEdge::new(
                NodeId::new("node-missing"),
                NodeId::new("packet-main"),
                DependencyKind::Evidence,
            )),
            Err(DecisionMemoryError::NodeNotFound { .. })
        ),
        "edge with a missing endpoint was admitted",
    )?;
    require(
        matches!(
            NodeEnvelope::new(
                NodeId::new(" "),
                "fixture",
                PacketContent {
                    profile: Profile::Change,
                    packet_digest: "a".repeat(64),
                    revision: 1,
                    owner: "release-owner".to_string(),
                },
            ),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "blank node identity was admitted",
    )
}

#[test]
fn public_projection_inputs_reject_paths_secrets_and_personal_identifiers() -> TestResult {
    let separator = std::path::MAIN_SEPARATOR;
    let absolute = format!("{separator}private{separator}state");
    require(
        matches!(
            NodeEnvelope::new(
                NodeId::new("unsafe-provenance"),
                absolute,
                ArtifactContent {
                    artifact_identity: "fixture".to_string(),
                    revision: "one".to_string(),
                    content_digest: "a".repeat(64),
                },
            ),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "absolute provenance entered a public projection",
    )?;
    require(
        matches!(
            NodeEnvelope::new(
                NodeId::new(format!("{separator}private{separator}node")),
                "fixture",
                ArtifactContent {
                    artifact_identity: "fixture".to_string(),
                    revision: "one".to_string(),
                    content_digest: "a".repeat(64),
                },
            ),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "absolute node identity entered a public projection",
    )?;

    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let mut unsafe_graph_value = serde_json::to_value(&graph)?;
    let unsafe_identity = format!("{separator}private{separator}node");
    let nodes = unsafe_graph_value
        .get_mut("nodes")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| "serialized graph did not contain nodes".to_string())?;
    let mut unsafe_packet =
        nodes.remove("packet-main").ok_or_else(|| "serialized packet was absent".to_string())?;
    let serialized_id = unsafe_packet
        .get_mut("node")
        .and_then(|node| node.get_mut("id"))
        .ok_or_else(|| "serialized packet identity was absent".to_string())?;
    *serialized_id = serde_json::Value::String(unsafe_identity.clone());
    nodes.insert(unsafe_identity, unsafe_packet);
    let unsafe_graph: DecisionMemoryGraph = serde_json::from_value(unsafe_graph_value)?;
    require(
        matches!(
            project_decision_memory(&unsafe_graph),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "deserialized unsafe node identity entered a public projection",
    )?;

    let sensitive_reason = format!("{}{}", "token", "=credential");
    require(
        matches!(
            propagate_stale(
                &mut graph,
                &NodeId::new("packet-main"),
                "f".repeat(64),
                sensitive_reason,
            ),
            Err(DecisionMemoryError::InvalidStructure { .. })
        ),
        "secret-bearing stale reason entered a public projection",
    )
}

#[test]
fn history_round_trip_and_projection_preserve_governance_without_local_paths() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Architecture))?;
    let mut graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &mut graph);
    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));
    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles: vec![bundle.clone()],
        graph: graph.clone(),
        terminal_result: result,
    })?;
    let mut restored = store.load()?;
    require(restored.graph == graph, "persisted graph round-trip changed semantics")?;

    let old_decision = graph.node(&NodeId::new("decision-bundle-083-r1-accepted"))?.clone();
    let mut successor_draft = draft(Profile::Architecture);
    successor_draft.decision_memory_revision = 2;
    successor_draft.rationale = "superseding rationale".to_string();
    successor_draft.approvals[0].approval_id = NodeId::new("approval-r2");
    successor_draft.approvals[0].decision_memory_revision = 2;
    successor_draft.approvals[0].valid_through_revision = Some(2);
    successor_draft.risk_acceptances[0].acceptance_id = NodeId::new("risk-r2");
    successor_draft.risk_acceptances[0].approval_id = NodeId::new("approval-r2");
    let successor_bundle = build_governance_bundle(successor_draft)?;
    restored.graph.admit_bundle(&successor_bundle)?;
    validate_governance_bundle(&successor_bundle, &mut restored.graph);
    let successor_id = NodeId::new("decision-bundle-083-r2-accepted");
    restored.graph.supersede(&old_decision.id().clone(), &successor_id, "new verified input")?;
    let historical_result = validate_governance_bundle(&successor_bundle, &mut restored.graph);

    let conflicting_successor = DecisionMemoryNode::Decision(NodeEnvelope::new(
        successor_id.clone(),
        "deterministic-governance",
        DecisionContent {
            bundle_id: "bundle-083".to_string(),
            rationale: "conflicting rationale".to_string(),
            alternatives: Vec::new(),
            assumptions: Vec::new(),
            triggers: Vec::new(),
            owners: vec!["release-owner".to_string()],
            decision: GovernanceDecision::Accepted,
        },
    )?);
    require(
        matches!(
            restored.graph.insert_or_replay(conflicting_successor),
            Err(DecisionMemoryError::ContentConflict { .. })
        ),
        "same decision identity with different content did not conflict",
    )?;

    let mut admitted_bundles = restored.admitted_bundles.clone();
    admitted_bundles.push(successor_bundle);
    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles,
        graph: restored.graph.clone(),
        terminal_result: historical_result,
    })?;
    restored = store.load()?;

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
fn required_missing_history_can_be_superseded_by_a_later_verified_revision() -> TestResult {
    let mut incomplete_draft = draft(Profile::Architecture);
    incomplete_draft.provided_evidence[0].evidence_id = NodeId::new("evidence-incomplete");
    incomplete_draft.provided_evidence[0].references = vec![format!("sha256:{}", "f".repeat(64))];
    incomplete_draft.approvals[0].evidence_ids = vec![NodeId::new("evidence-incomplete")];
    let incomplete_bundle = build_governance_bundle(incomplete_draft)?;
    let mut graph = graph_for(&incomplete_bundle)?;
    let incomplete_result = validate_governance_bundle(&incomplete_bundle, &mut graph);
    require(
        incomplete_result.decision == GovernanceDecision::RequiredMissing,
        "invalid evidence did not produce the required-missing history fixture",
    )?;

    let mut verified_draft = draft(Profile::Architecture);
    verified_draft.decision_memory_revision = 2;
    verified_draft.approvals[0].approval_id = NodeId::new("approval-r2");
    verified_draft.approvals[0].decision_memory_revision = 2;
    verified_draft.approvals[0].valid_through_revision = Some(2);
    verified_draft.risk_acceptances[0].acceptance_id = NodeId::new("risk-r2");
    verified_draft.risk_acceptances[0].approval_id = NodeId::new("approval-r2");
    let verified_bundle = build_governance_bundle(verified_draft)?;
    graph.admit_bundle(&verified_bundle)?;
    validate_governance_bundle(&verified_bundle, &mut graph);
    graph.supersede(
        &NodeId::new("decision-bundle-083-r1-required-missing"),
        &NodeId::new("decision-bundle-083-r2-accepted"),
        "external evidence supplied at a later revision",
    )?;
    let terminal_result = validate_governance_bundle(&verified_bundle, &mut graph);

    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![incomplete_bundle, verified_bundle],
                graph,
                terminal_result,
            }),
            Ok(())
        ),
        "required-missing decision could not be preserved through valid supersession",
    )
}

#[test]
fn persistence_rejects_mismatched_or_torn_atomic_snapshots() -> TestResult {
    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &mut graph);
    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));

    let mut mismatched = result.clone();
    mismatched.graph_digest = "0".repeat(64);
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![bundle.clone()],
                graph: graph.clone(),
                terminal_result: mismatched,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "graph and terminal result were persisted with different digests",
    )?;

    let mut tampered_value = serde_json::to_value(&graph)?;
    let owner = tampered_value
        .get_mut("nodes")
        .and_then(|nodes| nodes.get_mut("packet-main"))
        .and_then(|packet| packet.get_mut("node"))
        .and_then(|node| node.get_mut("content"))
        .and_then(|content| content.get_mut("owner"))
        .ok_or_else(|| "serialized packet owner was absent".to_string())?;
    *owner = serde_json::Value::String("tampered-owner".to_string());
    let tampered_graph: DecisionMemoryGraph = serde_json::from_value(tampered_value)?;
    let mut tampered_result = result.clone();
    tampered_result.graph_digest = tampered_graph.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![bundle.clone()],
                graph: tampered_graph,
                terminal_result: tampered_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "snapshot with forged cached node digest was persisted",
    )?;

    let mut stale_draft = draft(Profile::Change);
    stale_draft.approvals[0].fresh = false;
    let stale_bundle = build_governance_bundle(stale_draft)?;
    let mut stale_graph = graph_for(&stale_bundle)?;
    let mut stale_result = validate_governance_bundle(&stale_bundle, &mut stale_graph);
    let mut resurrected_value = serde_json::to_value(&stale_graph)?;
    let freshness = resurrected_value
        .get_mut("nodes")
        .and_then(|nodes| nodes.get_mut("approval-main"))
        .and_then(|approval| approval.get_mut("node"))
        .and_then(|node| node.get_mut("freshness"))
        .ok_or_else(|| "serialized approval freshness was absent".to_string())?;
    *freshness = serde_json::json!({ "state": "fresh" });
    let resurrected_graph: DecisionMemoryGraph = serde_json::from_value(resurrected_value)?;
    stale_result.graph_digest = resurrected_graph.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![stale_bundle],
                graph: resurrected_graph,
                terminal_result: stale_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "serialized freshness tampering resurrected stale approval authority",
    )?;

    let mut propagated_graph = graph.clone();
    let changed_approval_digest =
        changed_digest(propagated_graph.node(&NodeId::new("approval-main"))?)?.sha256;
    propagate_stale(
        &mut propagated_graph,
        &NodeId::new("approval-main"),
        changed_approval_digest,
        "approval binding changed",
    )?;
    let mut propagated_result = validate_governance_bundle(&bundle, &mut propagated_graph);
    let mut propagated_value = serde_json::to_value(&propagated_graph)?;
    for node_id in ["approval-main", "risk-main", "decision-bundle-083-r1-accepted"] {
        let freshness = propagated_value
            .get_mut("nodes")
            .and_then(|nodes| nodes.get_mut(node_id))
            .and_then(|node| node.get_mut("node"))
            .and_then(|node| node.get_mut("freshness"))
            .ok_or_else(|| format!("serialized propagated freshness was absent for {node_id}"))?;
        *freshness = serde_json::json!({ "state": "fresh" });
    }
    let resurrected_propagation: DecisionMemoryGraph = serde_json::from_value(propagated_value)?;
    propagated_result.graph_digest = resurrected_propagation.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![bundle.clone()],
                graph: resurrected_propagation,
                terminal_result: propagated_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "journaled propagated staleness was resurrected by envelope tampering",
    )?;

    let mut missing_decision_edge_value = serde_json::to_value(&graph)?;
    let edges = missing_decision_edge_value
        .get_mut("edges")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "serialized graph did not contain edges".to_string())?;
    let edge_count = edges.len();
    let terminal_id = "decision-bundle-083-r1-accepted";
    edges.retain(|edge| {
        edge.get("target").and_then(serde_json::Value::as_str) != Some(terminal_id)
            || edge.get("source").and_then(serde_json::Value::as_str) != Some("approval-main")
    });
    require(edges.len() + 1 == edge_count, "terminal authority edge fixture was absent")?;
    let missing_decision_edge: DecisionMemoryGraph =
        serde_json::from_value(missing_decision_edge_value)?;
    let mut missing_edge_result = result.clone();
    missing_edge_result.graph_digest = missing_decision_edge.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![bundle.clone()],
                graph: missing_decision_edge,
                terminal_result: missing_edge_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "snapshot without a normative terminal authority edge was persisted",
    )?;

    let mut semantic_claim = result.clone();
    semantic_claim.semantic_truth_asserted = true;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![bundle.clone()],
                graph: graph.clone(),
                terminal_result: semantic_claim,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "snapshot persisted a deterministic result that asserted semantic truth",
    )?;

    let mut execution_claim = result.clone();
    execution_claim.execution_audit.model_calls = 1;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![bundle.clone()],
                graph: graph.clone(),
                terminal_result: execution_claim,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "snapshot persisted a deterministic result that claimed model execution",
    )?;

    let mut missing_authority_draft = draft(Profile::Change);
    missing_authority_draft.approvals.clear();
    missing_authority_draft.risk_acceptances.clear();
    let missing_authority_bundle = build_governance_bundle(missing_authority_draft)?;
    let mut fabricated_graph = graph_for(&missing_authority_bundle)?;
    let fabricated_dependencies = fabricated_graph
        .nodes()
        .map(|node| {
            let kind = match node {
                DecisionMemoryNode::Approval(_) => DependencyKind::Authority,
                DecisionMemoryNode::Evidence(_) => DependencyKind::Evidence,
                _ => DependencyKind::FreshnessImpact,
            };
            (node.id().clone(), kind)
        })
        .collect::<Vec<_>>();
    let fabricated_decision_id = NodeId::new("decision-bundle-083-r1-accepted");
    fabricated_graph.insert_or_replay(DecisionMemoryNode::Decision(NodeEnvelope::new(
        fabricated_decision_id.clone(),
        "deterministic-governance",
        DecisionContent {
            bundle_id: "bundle-083".to_string(),
            rationale: missing_authority_bundle.metadata.rationale.clone(),
            alternatives: missing_authority_bundle.metadata.alternatives.clone(),
            assumptions: missing_authority_bundle.metadata.assumptions.clone(),
            triggers: missing_authority_bundle.metadata.triggers.clone(),
            owners: missing_authority_bundle.metadata.owners.clone(),
            decision: GovernanceDecision::Accepted,
        },
    )?))?;
    for (source, kind) in fabricated_dependencies {
        fabricated_graph.add_edge(DependencyEdge::new(
            source,
            fabricated_decision_id.clone(),
            kind,
        ))?;
    }
    let mut fabricated_result = result.clone();
    fabricated_result.graph_digest = fabricated_graph.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![missing_authority_bundle],
                graph: fabricated_graph,
                terminal_result: fabricated_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "fabricated accepted decision bypassed deterministic authority recomputation",
    )?;

    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles: vec![bundle],
        graph,
        terminal_result: result,
    })?;
    let state_path = workspace.path().join(".canon/decision-memory/state.json");
    std::fs::write(&state_path, b"{\"schema_version\":")?;
    require(
        matches!(store.load(), Err(DecisionMemoryError::InvalidSnapshot { .. })),
        "torn snapshot was treated as complete",
    )
}

#[test]
fn persistence_replays_terminal_states_and_rejects_journal_topology_tampering() -> TestResult {
    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));
    let absent_store = DecisionMemoryStore::new(workspace.path().join(".canon-absent"));
    require(
        matches!(absent_store.load(), Err(DecisionMemoryError::Persistence { .. })),
        "missing decision-memory snapshot did not return a persistence error",
    )?;

    let mut blocked_draft = draft(Profile::Change);
    blocked_draft.approvals.clear();
    blocked_draft.risk_acceptances.clear();
    let mut required_missing_draft = draft(Profile::Architecture);
    required_missing_draft.provided_evidence[0].references =
        vec![format!("sha256:{}", "f".repeat(64))];
    let mut stale_draft = draft(Profile::Verification);
    stale_draft.approvals[0].fresh = false;
    for (draft, expected) in [
        (blocked_draft, GovernanceDecision::Blocked),
        (required_missing_draft, GovernanceDecision::RequiredMissing),
        (stale_draft, GovernanceDecision::Stale),
    ] {
        let bundle = build_governance_bundle(draft)?;
        let mut graph = graph_for(&bundle)?;
        let terminal_result = validate_governance_bundle(&bundle, &mut graph);
        require(
            terminal_result.decision == expected,
            "terminal-state persistence fixture produced the wrong decision",
        )?;
        store.persist(&DecisionMemoryStoreSnapshot {
            admitted_bundles: vec![bundle],
            graph,
            terminal_result,
        })?;
        require(
            store.load()?.terminal_result.decision == expected,
            "terminal-state snapshot did not replay exactly",
        )?;
    }

    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &mut graph);
    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles: vec![bundle.clone()],
        graph: graph.clone(),
        terminal_result: result.clone(),
    })?;
    let state_path = workspace.path().join(".canon/decision-memory/state.json");
    let mut persisted: serde_json::Value = serde_json::from_slice(&std::fs::read(&state_path)?)?;
    persisted["schema_version"] = serde_json::Value::String("unknown-snapshot-v0".to_string());
    std::fs::write(&state_path, serde_json::to_vec(&persisted)?)?;
    require(
        matches!(store.load(), Err(DecisionMemoryError::InvalidSnapshot { .. })),
        "unknown snapshot schema was loaded",
    )?;

    let mut wrong_contract = graph.clone();
    wrong_contract.contract_line = "canon-contracts/unknown".to_string();
    require_snapshot_rejected(
        &store,
        vec![bundle.clone()],
        wrong_contract,
        result.clone(),
        "unknown graph contract line was persisted",
    )?;

    let mut cycle_value = serde_json::to_value(&graph)?;
    cycle_value
        .get_mut("edges")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "serialized graph edges were absent".to_string())?
        .push(serde_json::json!({
            "source": "decision-bundle-083-r1-accepted",
            "target": "packet-main",
            "kind": "freshness_impact"
        }));
    require_snapshot_rejected(
        &store,
        vec![bundle.clone()],
        serde_json::from_value(cycle_value)?,
        result.clone(),
        "cyclic snapshot graph was persisted",
    )?;

    let mut dangling_value = serde_json::to_value(&graph)?;
    dangling_value
        .get_mut("edges")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "serialized graph edges were absent".to_string())?
        .push(serde_json::json!({
            "source": "node-missing",
            "target": "packet-main",
            "kind": "freshness_impact"
        }));
    require_snapshot_rejected(
        &store,
        vec![bundle.clone()],
        serde_json::from_value(dangling_value)?,
        result.clone(),
        "dangling snapshot graph was persisted",
    )?;

    require_snapshot_rejected(
        &store,
        Vec::new(),
        graph.clone(),
        result.clone(),
        "snapshot without admitted bundle roots was persisted",
    )?;
    require_snapshot_rejected(
        &store,
        vec![bundle.clone(), bundle.clone()],
        graph.clone(),
        result.clone(),
        "duplicate admitted bundle roots were persisted",
    )?;
    let mut tampered_bundle = bundle.clone();
    tampered_bundle.metadata.rationale = "tampered rationale".to_string();
    require_snapshot_rejected(
        &store,
        vec![tampered_bundle],
        graph.clone(),
        result.clone(),
        "bundle root with stale digest was persisted",
    )?;

    for (mutation, message) in [
        ("duplicate_admission", "duplicate admission journal event was persisted"),
        ("validation_first", "validation preceding admission was persisted"),
        ("unknown_root", "journal admission without an exact root was persisted"),
    ] {
        let mut value = serde_json::to_value(&graph)?;
        let events = value
            .get_mut("events")
            .and_then(serde_json::Value::as_array_mut)
            .ok_or_else(|| "serialized graph journal was absent".to_string())?;
        match mutation {
            "duplicate_admission" => events.insert(1, events[0].clone()),
            "validation_first" => events.swap(0, 1),
            "unknown_root" => {
                events[0]["bundle_id"] = serde_json::Value::String("bundle-unknown".to_string());
            }
            _ => return Err("unknown journal mutation fixture".into()),
        }
        require_snapshot_rejected(
            &store,
            vec![bundle.clone()],
            serde_json::from_value(value)?,
            result.clone(),
            message,
        )?;
    }

    let second = build_governance_bundle(retarget_draft(draft(Profile::Architecture), "084"))?;
    let mut repository_graph = graph;
    repository_graph.admit_bundle(&second)?;
    let repository_result = validate_governance_bundle(&second, &mut repository_graph);
    require_snapshot_rejected(
        &store,
        vec![second, bundle],
        repository_graph,
        repository_result,
        "bundle roots were persisted out of journal admission order",
    )
}

#[test]
fn seeded_input_order_has_one_projection_and_graph_digest() -> TestResult {
    let mut ordered = draft(Profile::Verification);
    ordered.owners.push("backup-owner".to_string());
    ordered.claims.push("claim-second-binding".to_string());
    let first_claim = NodeId::new("claim-bundle-083-0001");
    let second_claim = NodeId::new("claim-bundle-083-0002");
    ordered.required_evidence[0].claim_ids = vec![first_claim.clone(), second_claim.clone()];
    ordered.provided_evidence[0].claim_ids = vec![first_claim.clone(), second_claim.clone()];
    ordered.approvals[0].claim_ids = vec![first_claim, second_claim];
    ordered.assumptions.push("second assumption".to_string());
    ordered.alternatives.push("second alternative".to_string());
    ordered.triggers.push("second trigger".to_string());
    ordered.forbidden_lineages.push("second-forbidden-lineage".to_string());
    let second_reference = format!("sha256:{}", "f".repeat(64));
    ordered.required_evidence[0].accepted_evidence_references.push(second_reference.clone());
    ordered.provided_evidence[0].references.push(second_reference);
    ordered.subject_artifacts.push(SubjectArtifactBinding {
        artifact_id: NodeId::new("artifact-bundle-083-second"),
        packet_id: NodeId::new("packet-main"),
        content: ArtifactContent {
            artifact_identity: "workspace-second".to_string(),
            revision: "git:fixture-second".to_string(),
            content_digest: "b".repeat(64),
        },
    });
    ordered.approvals[0].artifact_ids.push(NodeId::new("artifact-bundle-083-second"));
    let first_bundle = build_governance_bundle(ordered.clone())?;
    let mut reordered = ordered;
    reordered.assumptions.reverse();
    reordered.alternatives.reverse();
    reordered.triggers.reverse();
    reordered.owners.reverse();
    reordered.claims.reverse();
    reordered.forbidden_lineages.reverse();
    reordered.subject_artifacts.reverse();
    reordered.required_evidence[0].claim_ids.reverse();
    reordered.required_evidence[0].accepted_evidence_references.reverse();
    reordered.provided_evidence[0].claim_ids.reverse();
    reordered.provided_evidence[0].references.reverse();
    reordered.approvals[0].claim_ids.reverse();
    reordered.approvals[0].artifact_ids.reverse();
    let second_bundle = build_governance_bundle(reordered)?;
    let mut first = graph_for(&first_bundle)?;
    let mut second = graph_for(&second_bundle)?;
    let first_result = validate_governance_bundle(&first_bundle, &mut first);
    let second_result = validate_governance_bundle(&second_bundle, &mut second);
    require(
        first_result == second_result,
        "equivalent graphs had different terminal deterministic results",
    )?;
    require(first.digest()? == second.digest()?, "equivalent graphs had different digests")?;
    require(
        project_decision_memory(&first)? == project_decision_memory(&second)?,
        "equivalent graphs had different projections",
    )
}

#[test]
fn repository_graph_admits_multiple_bundles_without_cross_branch_authority() -> TestResult {
    let first = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&first)?;
    validate_governance_bundle(&first, &mut graph);

    let second_draft = retarget_draft(draft(Profile::Architecture), "084");
    let second = build_governance_bundle(second_draft)?;
    graph.admit_bundle(&second)?;
    let second_result = validate_governance_bundle(&second, &mut graph);
    require(
        second_result.decision == GovernanceDecision::Accepted,
        "second exact bundle could not join repository decision memory",
    )?;

    let workspace = tempdir()?;
    let store = DecisionMemoryStore::new(workspace.path().join(".canon"));
    store.persist(&DecisionMemoryStoreSnapshot {
        admitted_bundles: vec![first.clone(), second.clone()],
        graph: graph.clone(),
        terminal_result: second_result.clone(),
    })?;
    let mut invalid_supersession = graph.clone();
    invalid_supersession.supersede(
        &NodeId::new("decision-bundle-083-r1-accepted"),
        &NodeId::new("decision-bundle-084-r1-accepted"),
        "unrelated bundle",
    )?;
    let invalid_supersession_result =
        validate_governance_bundle(&second, &mut invalid_supersession);
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![first.clone(), second.clone()],
                graph: invalid_supersession,
                terminal_result: invalid_supersession_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "cross-bundle decision supersession was persisted",
    )?;
    let mut contaminated = graph.clone();
    contaminated.add_edge(DependencyEdge::new(
        NodeId::new("packet-main"),
        NodeId::new("artifact-bundle-084-main"),
        DependencyKind::FreshnessImpact,
    ))?;
    let mut contaminated_result = second_result.clone();
    contaminated_result.graph_digest = contaminated.digest()?.sha256;
    require(
        matches!(
            store.persist(&DecisionMemoryStoreSnapshot {
                admitted_bundles: vec![first.clone(), second.clone()],
                graph: contaminated,
                terminal_result: contaminated_result,
            }),
            Err(DecisionMemoryError::InvalidSnapshot { .. })
        ),
        "cross-bundle freshness edge was persisted",
    )?;

    propagate_stale(
        &mut graph,
        &NodeId::new("packet-main"),
        "9".repeat(64),
        "first bundle changed",
    )?;
    require(
        graph.freshness(&NodeId::new("decision-bundle-084-r1-accepted"))? == &FreshnessState::Fresh,
        "unrelated historical branch invalidated the second decision",
    )?;

    let before_conflict = graph.clone();
    let mut conflicting_draft = retarget_draft(draft(Profile::Architecture), "085");
    conflicting_draft.approvals[0].approval_id = NodeId::new("approval-main");
    conflicting_draft.risk_acceptances[0].approval_id = NodeId::new("approval-main");
    let conflicting = build_governance_bundle(conflicting_draft)?;
    require(
        matches!(
            graph.admit_bundle(&conflicting),
            Err(DecisionMemoryError::ContentConflict { .. })
        ),
        "late admission conflict was not typed",
    )?;
    require(
        graph == before_conflict,
        "failed multi-bundle admission partially mutated repository decision memory",
    )
}

#[test]
fn governance_kernel_has_no_execution_capability_or_synthetic_evidence() -> TestResult {
    let sources = [
        include_str!("../../crates/canon-engine/src/decision_memory/mod.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/bundle.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/digest.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/error.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/freshness.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/graph.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/projection.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/store.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/topology.rs"),
        include_str!("../../crates/canon-engine/src/decision_memory/validation.rs"),
    ];
    for source in sources {
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
            require(
                !source.contains(forbidden),
                "decision-memory module gained execution capability",
            )?;
        }
    }

    let bundle = build_governance_bundle(draft(Profile::Change))?;
    let mut graph = graph_for(&bundle)?;
    let result = validate_governance_bundle(&bundle, &mut graph);
    require(
        result.execution_audit.process_invocations == 0
            && result.execution_audit.network_invocations == 0
            && result.execution_audit.provider_credential_reads == 0
            && result.execution_audit.model_calls == 0
            && result.execution_audit.semantic_evidence_created == 0,
        "deterministic validation recorded an external execution",
    )?;
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
