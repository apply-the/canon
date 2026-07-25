//! Contract tests for Canon's deterministic public governance data shapes.

use canon_contracts::{
    AcceptanceCriterion, Approval, ApprovalDecision, AuthorityRequirement, BundleDigest, BundleId,
    CanonContractVersion, ChallengeTier, Claim, DecisionMemoryNode, DecisionMemoryProjection,
    DeterministicVerificationEvidence, DeterministicVerificationStatus, EvidenceReference,
    ExternalVerificationEvidence, Finding, GovernanceBundle, GovernancePacket,
    GovernancePublication, OneShotOperation, OneShotRequest, PacketId, Profile,
    PublicationProjectionKind, Revision, ScopeItem, StableProfileRegistry, VerificationKind,
    VerificationRequirement,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fmt::Debug;

const BUNDLE_ID: &str = "bundle-001";
const BUNDLE_DIGEST: &str = "sha256:bundle";
const PACKET_ID: &str = "packet-001";
const DECISION_REVISION: u64 = 12;

fn check(condition: bool, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    if condition { Ok(()) } else { Err(std::io::Error::other(message).into()) }
}

fn check_eq<T: Debug + PartialEq>(
    actual: T,
    expected: T,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if actual == expected {
        Ok(())
    } else {
        Err(std::io::Error::other(format!("{message}: actual {actual:?}, expected {expected:?}"))
            .into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct StartPayload {
    bundle_id: BundleId,
}

fn governance_bundle() -> GovernanceBundle {
    let packet = GovernancePacket {
        packet_id: PacketId::new(PACKET_ID),
        profile: Profile::Change,
        change_intent: "Make publication atomic".to_owned(),
        scope: vec![ScopeItem::new("publication")],
        risks: vec!["partial authoritative state".to_owned()],
        invariants: vec!["Canon never modifies the workspace".to_owned()],
        acceptance_criteria: vec![AcceptanceCriterion::new("publication is crash consistent")],
        cross_packet_references: vec![PacketId::new("packet-000")],
    };
    GovernanceBundle {
        contract_version: CanonContractVersion::V1,
        bundle_id: BundleId::new(BUNDLE_ID),
        bundle_digest: BundleDigest::new(BUNDLE_DIGEST),
        profile: Profile::Change,
        packets: vec![packet],
        authority: AuthorityRequirement { required_approvers: vec!["release-owner".to_owned()] },
        required_evidence: vec![VerificationRequirement {
            kind: VerificationKind::ExternalSemanticReview,
            claims: vec![Claim::new("publication-safety")],
            minimum_challenge_tier: ChallengeTier::Tier2,
        }],
        decision_memory_revision: Revision::new(DECISION_REVISION),
    }
}

#[test]
fn registry_contains_exactly_nine_stable_profiles_without_implementation()
-> Result<(), Box<dyn std::error::Error>> {
    let profiles = StableProfileRegistry::profiles();
    let encoded = serde_json::to_value(profiles)?;

    check_eq(
        encoded,
        json!([
            "discovery",
            "requirements",
            "architecture",
            "backlog",
            "change",
            "refactor",
            "verification",
            "pr-review",
            "incident"
        ]),
        "stable profile registry",
    )?;
    check(
        !serde_json::to_value(profiles)?
            .as_array()
            .is_some_and(|values| values.iter().any(|value| value == "implementation")),
        "implementation appeared in the stable registry",
    )?;
    check(
        serde_json::from_value::<Profile>(json!("implementation")).is_err(),
        "implementation profile was accepted",
    )?;
    Ok(())
}

#[test]
fn governance_packets_and_bundles_round_trip_with_stable_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let bundle = governance_bundle();
    let encoded = serde_json::to_value(&bundle)?;

    check_eq(encoded["contract_version"].clone(), json!("1.0"), "contract version")?;
    check_eq(encoded["bundle_id"].clone(), json!(BUNDLE_ID), "bundle ID")?;
    check_eq(encoded["profile"].clone(), json!("change"), "profile")?;
    check_eq(
        encoded["required_evidence"][0]["kind"].clone(),
        json!("external_semantic_review"),
        "external review requirement",
    )?;
    check_eq(serde_json::from_value::<GovernanceBundle>(encoded)?, bundle, "bundle round trip")?;
    Ok(())
}

#[test]
fn deterministic_verification_evidence_is_typed_and_fail_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let evidence = DeterministicVerificationEvidence {
        check_identity: "cargo-test-workspace".to_owned(),
        claims: vec![Claim::new("workspace-tests-pass")],
        status: DeterministicVerificationStatus::Passed,
        evidence_references: vec![EvidenceReference::new("evidence-test-log")],
    };
    let encoded = serde_json::to_value(&evidence)?;

    check_eq(
        encoded["check_identity"].clone(),
        json!("cargo-test-workspace"),
        "deterministic check identity",
    )?;
    check_eq(encoded["status"].clone(), json!("passed"), "deterministic check status")?;
    check_eq(
        serde_json::from_value::<DeterministicVerificationEvidence>(encoded)?,
        evidence,
        "deterministic evidence round trip",
    )?;
    check(
        serde_json::from_value::<DeterministicVerificationStatus>(json!("self_attested")).is_err(),
        "unknown deterministic verification status was accepted",
    )
}

#[test]
fn external_semantic_review_is_evidence_not_an_internal_executor()
-> Result<(), Box<dyn std::error::Error>> {
    let evidence = ExternalVerificationEvidence {
        reviewer_identity: "reviewer-001".to_owned(),
        lineage: "provider-family/model-pin/invocation-001".to_owned(),
        independent_context_identity: "context-sha256:001".to_owned(),
        claims: vec![Claim::new("publication-safety")],
        findings: vec![Finding::new("No unexplained state is overwritten")],
        evidence_references: vec![EvidenceReference::new("evidence-001")],
        challenge_tier: ChallengeTier::Tier2,
        named_override: None,
    };
    let encoded = serde_json::to_value(&evidence)?;

    check_eq(encoded["reviewer_identity"].clone(), json!("reviewer-001"), "reviewer identity")?;
    check_eq(encoded["challenge_tier"].clone(), json!("tier_2"), "challenge tier")?;
    check_eq(
        serde_json::from_value::<ExternalVerificationEvidence>(encoded)?,
        evidence,
        "external evidence round trip",
    )?;
    check(
        serde_json::from_value::<VerificationKind>(json!("internal_semantic_reviewer")).is_err(),
        "internal semantic reviewer was accepted",
    )?;
    Ok(())
}

#[test]
fn unknown_authority_and_approval_values_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
    check(
        serde_json::from_value::<ApprovalDecision>(json!("self_approved")).is_err(),
        "unknown approval authority was accepted",
    )?;
    check(
        serde_json::from_value::<ChallengeTier>(json!("automatic_override")).is_err(),
        "unknown challenge authority was accepted",
    )
}

#[test]
fn unsupported_versions_are_rejected_and_additive_fields_are_accepted()
-> Result<(), Box<dyn std::error::Error>> {
    let request = OneShotRequest {
        contract_version: CanonContractVersion::V1,
        request_id: "req-001".to_owned(),
        operation: OneShotOperation::Start,
        payload: StartPayload { bundle_id: BundleId::new(BUNDLE_ID) },
    };
    let mut unsupported = serde_json::to_value(&request)?;
    unsupported["contract_version"] = json!("2.0");
    check(
        serde_json::from_value::<OneShotRequest<StartPayload>>(unsupported).is_err(),
        "unsupported contract version was accepted",
    )?;

    let mut additive = serde_json::to_value(&request)?;
    additive["future_optional_field"] = json!({"introduced_in": "1.1"});
    check_eq(
        serde_json::from_value::<OneShotRequest<StartPayload>>(additive)?,
        request,
        "additive one-shot request",
    )?;
    Ok(())
}

#[test]
fn one_shot_operation_set_is_frozen() -> Result<(), Box<dyn std::error::Error>> {
    let operations = [
        (OneShotOperation::Capabilities, "capabilities"),
        (OneShotOperation::Start, "start"),
        (OneShotOperation::Refresh, "refresh"),
        (OneShotOperation::Approve, "approve"),
        (OneShotOperation::Inspect, "inspect"),
        (OneShotOperation::Publish, "publish"),
    ];

    for (operation, wire_value) in operations {
        check_eq(serde_json::to_value(operation)?, json!(wire_value), "one-shot operation")?;
    }
    check(
        serde_json::from_value::<OneShotOperation>(json!("execute")).is_err(),
        "unsupported one-shot operation was accepted",
    )?;
    Ok(())
}

#[test]
fn decision_memory_and_publication_contain_projections_only()
-> Result<(), Box<dyn std::error::Error>> {
    let approval = Approval {
        approver_identity: "release-owner".to_owned(),
        decision: ApprovalDecision::Approved,
        decision_memory_revision: Revision::new(DECISION_REVISION),
        claims: vec![Claim::new("publication-safety")],
    };
    let node = DecisionMemoryNode {
        node_id: "decision-001".to_owned(),
        kind: "approval".to_owned(),
        source_reference: "approval-001".to_owned(),
        revision_introduced: Revision::new(DECISION_REVISION),
        relationships: vec!["bundle-001".to_owned()],
        supersedes: vec![],
        fresh: true,
        stale_reason: None,
    };
    let publication = GovernancePublication {
        contract_version: CanonContractVersion::V1,
        governance_bundle: governance_bundle(),
        projections: vec![
            PublicationProjectionKind::GovernanceBundle,
            PublicationProjectionKind::DecisionMemory,
            PublicationProjectionKind::Evidence,
        ],
        decision_memory: DecisionMemoryProjection {
            revision: Revision::new(DECISION_REVISION),
            nodes: vec![node],
        },
        evidence: vec![EvidenceReference::new("evidence-001")],
        approvals: vec![approval],
    };
    let encoded = serde_json::to_value(&publication)?;

    check_eq(
        encoded["projections"].clone(),
        json!(["governance_bundle", "decision_memory", "evidence"]),
        "publication projection kinds",
    )?;
    check_eq(
        encoded["governance_bundle"]["packets"][0]["change_intent"].clone(),
        json!("Make publication atomic"),
        "typed governance packet projection",
    )?;
    check_eq(
        serde_json::from_value::<GovernancePublication>(encoded)?,
        publication,
        "governance publication round trip",
    )?;

    let unknown_projection: Value = json!("workspace_mutation");
    check(
        serde_json::from_value::<PublicationProjectionKind>(unknown_projection).is_err(),
        "workspace mutation projection was accepted",
    )?;
    Ok(())
}
