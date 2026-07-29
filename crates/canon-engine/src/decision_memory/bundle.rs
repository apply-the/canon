//! Typed governance-bundle construction without expanding the public contract.

use std::collections::{BTreeMap, BTreeSet};

use canon_contracts::{
    AcceptanceCriterion, AuthorityRequirement, BundleDigest, BundleId, CanonContractVersion,
    ChallengeTier, Claim, EvidenceReference, ExternalVerificationEvidence, GovernanceBundle,
    GovernancePacket, PacketId, Profile, Revision, ScopeItem, VerificationKind,
    VerificationRequirement,
};
use serde::{Deserialize, Serialize};

use super::{
    AlternativeContent, ArtifactContent, AssumptionContent, ClaimContent, ContentDigest,
    DecisionContent, DecisionMemoryError, DecisionMemoryGraph, DecisionMemoryNode, DependencyEdge,
    DependencyKind, GovernanceDecision, NodeEnvelope, NodeId, PacketContent, TriggerContent,
};

const SUPPORTED_AUTHORITY_ZONE: &str = "governance-release";
const SUPPORTED_CHANGE_CLASS: &str = "governance-kernel";
const SUPPORTED_CONTEXT_CLASS: &str = "repository-local";
const MAX_RISK_TIER: u8 = 3;

/// Authoring input for one public governance packet.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernancePacketDraft {
    /// Stable packet identity.
    pub packet_id: String,
    /// Stable governance profile.
    pub profile: Profile,
    /// Exact packet revision.
    pub revision: u64,
    /// Governed change intent.
    pub change_intent: String,
    /// Admitted scope.
    pub scope: Vec<String>,
    /// Declared risks.
    pub risks: Vec<String>,
    /// Invariants to preserve.
    pub invariants: Vec<String>,
    /// Deterministic acceptance criteria.
    pub acceptance_criteria: Vec<String>,
    /// Exact referenced packet identities.
    pub cross_packet_references: Vec<String>,
}

/// Internal exact-binding metadata for required verification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationRequirementContent {
    /// Stable requirement identity.
    pub requirement_id: NodeId,
    /// Exact owning packet identity.
    pub packet_id: NodeId,
    /// Exact required claim identities.
    pub claim_ids: Vec<NodeId>,
    /// Exact required subject-artifact identities.
    pub artifact_ids: Vec<NodeId>,
    /// Required deterministic or external evidence category.
    pub kind: VerificationKind,
    /// Minimum challenge tier.
    pub minimum_challenge_tier: ChallengeTier,
    /// Immutable evidence references admitted by the requirement.
    pub accepted_evidence_references: Vec<String>,
}

/// Internal exact-binding metadata for supplied evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceContent {
    /// Stable evidence identity.
    pub evidence_id: NodeId,
    /// Exact owning packet identity.
    pub packet_id: NodeId,
    /// Exact claim identities covered by this evidence.
    pub claim_ids: Vec<NodeId>,
    /// Exact subject-artifact identities covered by this evidence.
    pub artifact_ids: Vec<NodeId>,
    /// Exact verification requirements this evidence satisfies.
    pub requirement_ids: Vec<NodeId>,
    /// Immutable evidence references.
    pub references: Vec<String>,
    /// External producer lineage.
    pub lineage: String,
    /// Independently constructed context identity.
    pub independent_context_identity: String,
    /// Satisfied challenge tier.
    pub challenge_tier: ChallengeTier,
    /// Whether this is external semantic evidence.
    pub external_semantic: bool,
    /// Exact-binding freshness supplied by the binding layer.
    pub fresh: bool,
    /// Named Tier 2 degradation record, when policy admits one.
    pub named_override: Option<String>,
}

/// Internal approval binding absent from the immutable public DTO.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalContent {
    /// Stable approval identity.
    pub approval_id: NodeId,
    /// Exact owning packet identity.
    pub packet_id: NodeId,
    /// Exact approved claim identities.
    pub claim_ids: Vec<NodeId>,
    /// Exact approved subject-artifact identities.
    pub artifact_ids: Vec<NodeId>,
    /// Exact evidence identities reviewed by this approval.
    pub evidence_ids: Vec<NodeId>,
    /// Exact verification requirements reviewed by this approval.
    pub requirement_ids: Vec<NodeId>,
    /// Named approver identity.
    pub approver: String,
    /// Exact authority zone.
    pub authority_zone: String,
    /// Whether authority approved rather than rejected.
    pub approved: bool,
    /// Exact decision-memory revision reviewed.
    pub decision_memory_revision: u64,
    /// Last revision for which this approval remains valid.
    pub valid_through_revision: Option<u64>,
    /// Exact-binding freshness supplied by the binding layer.
    pub fresh: bool,
}

/// Named and justified risk acceptance.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskAcceptanceContent {
    /// Stable risk-acceptance identity.
    pub acceptance_id: NodeId,
    /// Exact owning packet identity.
    pub packet_id: NodeId,
    /// Named risk owner.
    pub owner: String,
    /// Accepted bounded risk.
    pub risk: String,
    /// Required justification.
    pub justification: String,
    /// Challenge tier at which the risk was accepted.
    pub challenge_tier: ChallengeTier,
    /// Exact forbidden lineage whose degradation is accepted.
    pub lineage: String,
    /// Exact approval authorizing this acceptance.
    pub approval_id: NodeId,
    /// Exact-binding freshness.
    pub fresh: bool,
}

/// Exact subject-artifact node and owning packet binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectArtifactBinding {
    /// Stable artifact node identity.
    pub artifact_id: NodeId,
    /// Exact owning packet identity.
    pub packet_id: NodeId,
    /// Exact artifact revision and content digest.
    pub content: ArtifactContent,
}

/// Complete authoring input used to construct a frozen public bundle plus internal metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceBundleDraft {
    /// Stable bundle identity.
    pub bundle_id: String,
    /// Primary stable profile.
    pub profile: Profile,
    /// Exact decision-memory revision governed by this bundle.
    pub decision_memory_revision: u64,
    /// Typed packet drafts.
    pub packets: Vec<GovernancePacketDraft>,
    /// Exact subject artifacts governed by the packets.
    pub subject_artifacts: Vec<SubjectArtifactBinding>,
    /// Named required approvers.
    pub required_approvers: Vec<String>,
    /// Closed authority-zone value.
    pub authority_zone: String,
    /// Closed risk tier from zero through three.
    pub risk_tier: u8,
    /// Closed change class.
    pub change_class: String,
    /// Closed context class.
    pub context_class: String,
    /// Named decision owners.
    pub owners: Vec<String>,
    /// Stable governed claim text.
    pub claims: Vec<String>,
    /// Exact verification requirements.
    pub required_evidence: Vec<VerificationRequirementContent>,
    /// Supplied evidence records.
    pub provided_evidence: Vec<EvidenceContent>,
    /// Implementer or shared lineages that cannot independently verify work.
    pub forbidden_lineages: Vec<String>,
    /// Named approvals.
    pub approvals: Vec<ApprovalContent>,
    /// Recorded assumptions.
    pub assumptions: Vec<String>,
    /// Considered alternatives.
    pub alternatives: Vec<String>,
    /// Immutable decision rationale.
    pub rationale: String,
    /// Named risk acceptances.
    pub risk_acceptances: Vec<RiskAcceptanceContent>,
    /// Reconsideration triggers.
    pub triggers: Vec<String>,
    /// Whether the governed observation produced no change.
    pub no_change: bool,
}

/// Internal metadata bound to the immutable public governance bundle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceMetadata {
    /// Canonical packet digest by exact packet identity.
    pub packet_digests: BTreeMap<NodeId, ContentDigest>,
    /// Exact packet revision by stable packet identity.
    pub packet_revisions: BTreeMap<NodeId, u64>,
    /// Exact subject artifacts.
    pub subject_artifacts: Vec<SubjectArtifactBinding>,
    /// Closed authority zone.
    pub authority_zone: String,
    /// Frozen risk tier.
    pub risk_tier: u8,
    /// Frozen change class.
    pub change_class: String,
    /// Frozen context class.
    pub context_class: String,
    /// Exact decision-memory revision governed by this bundle.
    pub decision_memory_revision: u64,
    /// Named owners.
    pub owners: Vec<String>,
    /// Exact internal claim records.
    pub claims: BTreeMap<NodeId, String>,
    /// Exact verification requirements.
    pub required_evidence: Vec<VerificationRequirementContent>,
    /// Supplied evidence.
    pub provided_evidence: Vec<EvidenceContent>,
    /// Lineages excluded from independent verification.
    pub forbidden_lineages: Vec<String>,
    /// Supplied approvals.
    pub approvals: Vec<ApprovalContent>,
    /// Assumptions preserved with the decision.
    pub assumptions: Vec<String>,
    /// Alternatives preserved with the decision.
    pub alternatives: Vec<String>,
    /// Immutable rationale.
    pub rationale: String,
    /// Named risk acceptances.
    pub risk_acceptances: Vec<RiskAcceptanceContent>,
    /// Reconsideration triggers.
    pub triggers: Vec<String>,
    /// Explicit no-change event.
    pub no_change: bool,
}

/// Public frozen governance DTO paired with Canon-owned exact-binding metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundGovernanceBundle {
    /// Immutable public contract shape.
    pub contract: GovernanceBundle,
    /// Internal deterministic validation metadata.
    pub metadata: GovernanceMetadata,
}

#[derive(Serialize)]
struct BundleDigestInput<'a> {
    contract_version: CanonContractVersion,
    bundle_id: &'a str,
    profile: Profile,
    packets: &'a [GovernancePacket],
    authority: &'a AuthorityRequirement,
    required_evidence: &'a [VerificationRequirement],
    decision_memory_revision: Revision,
    metadata: &'a GovernanceMetadata,
}

/// Builds a normalized governance bundle using only the frozen public DTOs.
pub fn build_governance_bundle(
    mut draft: GovernanceBundleDraft,
) -> Result<BoundGovernanceBundle, DecisionMemoryError> {
    validate_draft(&draft)?;
    normalize_draft(&mut draft);
    let packets = build_packets(&draft);
    validate_packet_references(&packets)?;
    let claims = build_claims(&draft.bundle_id, &draft.claims);
    validate_internal_bindings(&draft, &claims)?;
    let metadata = build_metadata(&draft, &packets, claims)?;
    let authority = AuthorityRequirement { required_approvers: draft.required_approvers.clone() };
    let required_evidence = build_public_requirements(&metadata)?;
    let digest_input = BundleDigestInput {
        contract_version: CanonContractVersion::V1,
        bundle_id: &draft.bundle_id,
        profile: draft.profile,
        packets: &packets,
        authority: &authority,
        required_evidence: &required_evidence,
        decision_memory_revision: Revision::new(draft.decision_memory_revision),
        metadata: &metadata,
    };
    let digest = ContentDigest::compute("governance-bundle", &digest_input)?;
    let contract = GovernanceBundle {
        contract_version: CanonContractVersion::V1,
        bundle_id: BundleId::new(draft.bundle_id),
        bundle_digest: BundleDigest::new(digest.sha256),
        profile: draft.profile,
        packets,
        authority,
        required_evidence,
        decision_memory_revision: Revision::new(draft.decision_memory_revision),
    };
    Ok(BoundGovernanceBundle { contract, metadata })
}

impl DecisionMemoryGraph {
    /// Constructs a deterministic graph from a bound governance bundle.
    pub fn from_bundle(bundle: &BoundGovernanceBundle) -> Result<Self, DecisionMemoryError> {
        let mut graph = Self::new();
        graph.admit_bundle(bundle)?;
        Ok(graph)
    }

    /// Admits another exactly bound bundle into repository-local decision memory.
    pub fn admit_bundle(
        &mut self,
        bundle: &BoundGovernanceBundle,
    ) -> Result<(), DecisionMemoryError> {
        let mut staged = self.clone();
        insert_packets(&mut staged, bundle)?;
        insert_artifacts(&mut staged, bundle)?;
        insert_claims(&mut staged, bundle)?;
        insert_requirements(&mut staged, bundle)?;
        insert_evidence(&mut staged, bundle)?;
        insert_approvals(&mut staged, bundle)?;
        insert_history(&mut staged, bundle)?;
        insert_risk_acceptances(&mut staged, bundle)?;
        add_bundle_edges(&mut staged, bundle)?;
        staged.record_bundle_admission_event(
            typed_string(&bundle.contract.bundle_id)?,
            bundle.metadata.decision_memory_revision,
        );
        *self = staged;
        Ok(())
    }
}

fn validate_draft(draft: &GovernanceBundleDraft) -> Result<(), DecisionMemoryError> {
    require_nonblank(&draft.bundle_id, "bundle identity")?;
    require_nonblank(&draft.authority_zone, "authority zone")?;
    require_nonblank(&draft.change_class, "change class")?;
    require_nonblank(&draft.context_class, "context class")?;
    require_nonblank(&draft.rationale, "decision rationale")?;
    require_nonempty(&draft.packets, "packets")?;
    require_nonempty(&draft.subject_artifacts, "subject artifacts")?;
    require_nonempty(&draft.required_approvers, "required approvers")?;
    require_nonempty(&draft.owners, "owners")?;
    require_nonempty(&draft.claims, "claims")?;
    require_nonempty(&draft.required_evidence, "verification requirements")?;
    if draft.decision_memory_revision == 0 {
        return Err(DecisionMemoryError::invalid_structure(
            "decision-memory revision must be positive",
        ));
    }
    if draft.authority_zone != SUPPORTED_AUTHORITY_ZONE
        || draft.change_class != SUPPORTED_CHANGE_CLASS
        || draft.context_class != SUPPORTED_CONTEXT_CLASS
        || draft.risk_tier > MAX_RISK_TIER
    {
        return Err(DecisionMemoryError::invalid_structure(
            "unsupported closed governance classification",
        ));
    }
    require_unique_strings(&draft.claims, "claim")?;
    require_unique_strings(&draft.required_approvers, "approver")?;
    require_unique_strings(&draft.owners, "owner")?;
    require_unique_strings(&draft.forbidden_lineages, "forbidden lineage")?;
    require_unique_ids(
        draft.required_evidence.iter().map(|value| &value.requirement_id),
        "verification requirement",
    )?;
    require_unique_ids(
        draft.subject_artifacts.iter().map(|value| &value.artifact_id),
        "subject artifact",
    )?;
    require_unique_ids(draft.provided_evidence.iter().map(|value| &value.evidence_id), "evidence")?;
    require_unique_ids(draft.approvals.iter().map(|value| &value.approval_id), "approval")?;
    require_unique_ids(
        draft.risk_acceptances.iter().map(|value| &value.acceptance_id),
        "risk acceptance",
    )?;
    for requirement in &draft.required_evidence {
        require_nonempty(&requirement.claim_ids, "required evidence claims")?;
        require_nonempty(&requirement.artifact_ids, "required evidence artifacts")?;
        require_nonempty(
            &requirement.accepted_evidence_references,
            "accepted evidence references",
        )?;
        require_unique_strings(
            &requirement.accepted_evidence_references,
            "accepted evidence reference",
        )?;
    }
    for acceptance in &draft.risk_acceptances {
        require_nonblank(&acceptance.lineage, "risk-acceptance lineage")?;
        require_nonblank(&acceptance.risk, "accepted risk")?;
        require_nonblank(&acceptance.justification, "risk-acceptance justification")?;
    }
    if draft.packets.first().is_some_and(|packet| packet.profile != draft.profile) {
        return Err(DecisionMemoryError::invalid_structure(
            "primary packet profile does not match bundle profile",
        ));
    }
    for packet in &draft.packets {
        require_nonblank(&packet.packet_id, "packet identity")?;
        require_nonblank(&packet.change_intent, "packet change intent")?;
        if packet.revision == 0 {
            return Err(DecisionMemoryError::invalid_structure("packet revision must be positive"));
        }
        require_nonempty(&packet.scope, "packet scope")?;
        require_nonempty(&packet.risks, "packet risks")?;
        require_nonempty(&packet.invariants, "packet invariants")?;
        require_nonempty(&packet.acceptance_criteria, "packet acceptance criteria")?;
    }
    require_unique_strings(
        &draft.packets.iter().map(|value| value.packet_id.clone()).collect::<Vec<_>>(),
        "packet",
    )
}

fn normalize_draft(draft: &mut GovernanceBundleDraft) {
    for packet in &mut draft.packets {
        packet.scope.sort();
        packet.risks.sort();
        packet.invariants.sort();
        packet.acceptance_criteria.sort();
        packet.cross_packet_references.sort();
    }
    draft.required_approvers.sort();
    draft.subject_artifacts.sort_by(|left, right| left.artifact_id.cmp(&right.artifact_id));
    draft.owners.sort();
    draft.claims.sort();
    draft.required_evidence.sort_by(|left, right| left.requirement_id.cmp(&right.requirement_id));
    draft.provided_evidence.sort_by(|left, right| left.evidence_id.cmp(&right.evidence_id));
    draft.forbidden_lineages.sort();
    draft.approvals.sort_by(|left, right| left.approval_id.cmp(&right.approval_id));
    draft.risk_acceptances.sort_by(|left, right| left.acceptance_id.cmp(&right.acceptance_id));
    draft.assumptions.sort();
    draft.alternatives.sort();
    draft.triggers.sort();
    for requirement in &mut draft.required_evidence {
        requirement.claim_ids.sort();
        requirement.artifact_ids.sort();
        requirement.accepted_evidence_references.sort();
    }
    for evidence in &mut draft.provided_evidence {
        evidence.claim_ids.sort();
        evidence.artifact_ids.sort();
        evidence.requirement_ids.sort();
        evidence.references.sort();
    }
    for approval in &mut draft.approvals {
        approval.claim_ids.sort();
        approval.artifact_ids.sort();
        approval.evidence_ids.sort();
        approval.requirement_ids.sort();
    }
}

fn build_packets(draft: &GovernanceBundleDraft) -> Vec<GovernancePacket> {
    draft
        .packets
        .iter()
        .map(|packet| GovernancePacket {
            packet_id: PacketId::new(&packet.packet_id),
            profile: packet.profile,
            change_intent: packet.change_intent.clone(),
            scope: packet.scope.iter().map(ScopeItem::new).collect(),
            risks: packet.risks.clone(),
            invariants: packet.invariants.clone(),
            acceptance_criteria: packet
                .acceptance_criteria
                .iter()
                .map(AcceptanceCriterion::new)
                .collect(),
            cross_packet_references: packet
                .cross_packet_references
                .iter()
                .map(PacketId::new)
                .collect(),
        })
        .collect()
}

fn build_claims(bundle_id: &str, claims: &[String]) -> BTreeMap<NodeId, String> {
    claims
        .iter()
        .enumerate()
        .map(|(index, claim)| {
            let id = if claims.len() == 1 {
                NodeId::new(format!("claim-{bundle_id}-main"))
            } else {
                NodeId::new(format!("claim-{bundle_id}-{:04}", index + 1))
            };
            (id, claim.clone())
        })
        .collect()
}

fn build_metadata(
    draft: &GovernanceBundleDraft,
    packets: &[GovernancePacket],
    claims: BTreeMap<NodeId, String>,
) -> Result<GovernanceMetadata, DecisionMemoryError> {
    let packet_digests = draft
        .packets
        .iter()
        .zip(packets)
        .map(|(draft, packet)| {
            ContentDigest::compute("governance-packet", packet)
                .map(|digest| (NodeId::new(&draft.packet_id), digest))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let packet_revisions = draft
        .packets
        .iter()
        .map(|packet| (NodeId::new(&packet.packet_id), packet.revision))
        .collect();
    Ok(GovernanceMetadata {
        packet_digests,
        packet_revisions,
        subject_artifacts: draft.subject_artifacts.clone(),
        authority_zone: draft.authority_zone.clone(),
        risk_tier: draft.risk_tier,
        change_class: draft.change_class.clone(),
        context_class: draft.context_class.clone(),
        decision_memory_revision: draft.decision_memory_revision,
        owners: draft.owners.clone(),
        claims,
        required_evidence: draft.required_evidence.clone(),
        provided_evidence: draft.provided_evidence.clone(),
        forbidden_lineages: draft.forbidden_lineages.clone(),
        approvals: draft.approvals.clone(),
        assumptions: draft.assumptions.clone(),
        alternatives: draft.alternatives.clone(),
        rationale: draft.rationale.clone(),
        risk_acceptances: draft.risk_acceptances.clone(),
        triggers: draft.triggers.clone(),
        no_change: draft.no_change,
    })
}

fn build_public_requirements(
    metadata: &GovernanceMetadata,
) -> Result<Vec<VerificationRequirement>, DecisionMemoryError> {
    metadata
        .required_evidence
        .iter()
        .map(|requirement| {
            Ok(VerificationRequirement {
                kind: requirement.kind,
                claims: claim_values(&requirement.claim_ids, &metadata.claims)?
                    .into_iter()
                    .map(Claim::new)
                    .collect(),
                minimum_challenge_tier: requirement.minimum_challenge_tier,
            })
        })
        .collect()
}

fn validate_packet_references(packets: &[GovernancePacket]) -> Result<(), DecisionMemoryError> {
    let identities = packets
        .iter()
        .map(|packet| typed_string(&packet.packet_id).map(|id| (id, packet.profile)))
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    for packet in packets {
        let source = NodeId::new(typed_string(&packet.packet_id)?);
        for target in &packet.cross_packet_references {
            let target = NodeId::new(typed_string(target)?);
            let Some(target_profile) = identities.get(target.as_str()) else {
                return Err(DecisionMemoryError::DanglingReference {
                    reference_source: source,
                    target,
                });
            };
            if *target_profile != packet.profile {
                return Err(DecisionMemoryError::invalid_structure(
                    "cross-profile packet reference is not exactly compatible",
                ));
            }
        }
    }
    Ok(())
}

fn validate_internal_bindings(
    draft: &GovernanceBundleDraft,
    claims: &BTreeMap<NodeId, String>,
) -> Result<(), DecisionMemoryError> {
    let packets =
        draft.packets.iter().map(|value| NodeId::new(&value.packet_id)).collect::<BTreeSet<_>>();
    let primary_packet =
        draft.packets.first().map(|value| NodeId::new(&value.packet_id)).ok_or_else(|| {
            DecisionMemoryError::invalid_structure("bundle has no primary packet")
        })?;
    let artifact_packets = draft
        .subject_artifacts
        .iter()
        .map(|value| (value.artifact_id.clone(), value.packet_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let artifacts = artifact_packets.keys().cloned().collect::<BTreeSet<_>>();
    let requirement_packets = draft
        .required_evidence
        .iter()
        .map(|value| (value.requirement_id.clone(), value.packet_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let requirements = requirement_packets.keys().cloned().collect::<BTreeSet<_>>();
    let evidence_packets = draft
        .provided_evidence
        .iter()
        .map(|value| (value.evidence_id.clone(), value.packet_id.clone()))
        .collect::<BTreeMap<_, _>>();
    let evidence_ids = evidence_packets.keys().cloned().collect::<BTreeSet<_>>();
    for artifact in &draft.subject_artifacts {
        validate_binding(&artifact.artifact_id, &artifact.packet_id, &[], &packets, claims)?;
        ContentDigest::from_sha256(
            artifact.artifact_id.to_string(),
            &artifact.content.content_digest,
        )?;
    }
    for (source, packet, claim_ids) in draft
        .required_evidence
        .iter()
        .map(|value| (&value.requirement_id, &value.packet_id, &value.claim_ids))
        .chain(
            draft
                .provided_evidence
                .iter()
                .map(|value| (&value.evidence_id, &value.packet_id, &value.claim_ids)),
        )
        .chain(
            draft
                .approvals
                .iter()
                .map(|value| (&value.approval_id, &value.packet_id, &value.claim_ids)),
        )
    {
        validate_binding(source, packet, claim_ids, &packets, claims)?;
        if !claim_ids.is_empty() && packet != &primary_packet {
            return Err(DecisionMemoryError::invalid_structure(
                "claim binding cannot be borrowed across packets",
            ));
        }
    }
    for requirement in &draft.required_evidence {
        validate_ids(&requirement.requirement_id, &requirement.artifact_ids, &artifacts)?;
        validate_scoped_ids(
            &requirement.requirement_id,
            &requirement.packet_id,
            &requirement.artifact_ids,
            &artifact_packets,
        )?;
    }
    for evidence in &draft.provided_evidence {
        validate_ids(&evidence.evidence_id, &evidence.artifact_ids, &artifacts)?;
        validate_scoped_ids(
            &evidence.evidence_id,
            &evidence.packet_id,
            &evidence.artifact_ids,
            &artifact_packets,
        )?;
        validate_ids(&evidence.evidence_id, &evidence.requirement_ids, &requirements)?;
        validate_scoped_ids(
            &evidence.evidence_id,
            &evidence.packet_id,
            &evidence.requirement_ids,
            &requirement_packets,
        )?;
    }
    for approval in &draft.approvals {
        validate_ids(&approval.approval_id, &approval.artifact_ids, &artifacts)?;
        validate_scoped_ids(
            &approval.approval_id,
            &approval.packet_id,
            &approval.artifact_ids,
            &artifact_packets,
        )?;
        validate_ids(&approval.approval_id, &approval.evidence_ids, &evidence_ids)?;
        validate_scoped_ids(
            &approval.approval_id,
            &approval.packet_id,
            &approval.evidence_ids,
            &evidence_packets,
        )?;
        validate_ids(&approval.approval_id, &approval.requirement_ids, &requirements)?;
        validate_scoped_ids(
            &approval.approval_id,
            &approval.packet_id,
            &approval.requirement_ids,
            &requirement_packets,
        )?;
    }
    for risk in &draft.risk_acceptances {
        validate_binding(&risk.acceptance_id, &risk.packet_id, &[], &packets, claims)?;
        if !draft.approvals.iter().any(|approval| {
            approval.approval_id == risk.approval_id && approval.packet_id == risk.packet_id
        }) {
            return Err(DecisionMemoryError::DanglingReference {
                reference_source: risk.acceptance_id.clone(),
                target: risk.approval_id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_scoped_ids(
    source: &NodeId,
    packet: &NodeId,
    references: &[NodeId],
    admitted: &BTreeMap<NodeId, NodeId>,
) -> Result<(), DecisionMemoryError> {
    for target in references {
        if admitted.get(target) != Some(packet) {
            return Err(DecisionMemoryError::invalid_structure(format!(
                "{source} cannot borrow {target} across packet boundaries"
            )));
        }
    }
    Ok(())
}

fn validate_ids(
    source: &NodeId,
    references: &[NodeId],
    admitted: &BTreeSet<NodeId>,
) -> Result<(), DecisionMemoryError> {
    for target in references {
        if !admitted.contains(target) {
            return Err(DecisionMemoryError::DanglingReference {
                reference_source: source.clone(),
                target: target.clone(),
            });
        }
    }
    Ok(())
}

fn validate_binding(
    source: &NodeId,
    packet: &NodeId,
    claim_ids: &[NodeId],
    packets: &BTreeSet<NodeId>,
    claims: &BTreeMap<NodeId, String>,
) -> Result<(), DecisionMemoryError> {
    if !packets.contains(packet) {
        return Err(DecisionMemoryError::DanglingReference {
            reference_source: source.clone(),
            target: packet.clone(),
        });
    }
    for claim_id in claim_ids {
        if !claims.contains_key(claim_id) {
            return Err(DecisionMemoryError::DanglingReference {
                reference_source: source.clone(),
                target: claim_id.clone(),
            });
        }
    }
    Ok(())
}

fn insert_packets(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for packet in &bundle.contract.packets {
        let packet_id = NodeId::new(typed_string(&packet.packet_id)?);
        let digest = bundle
            .metadata
            .packet_digests
            .get(&packet_id)
            .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: packet_id.clone() })?;
        let owner = bundle.metadata.owners.first().cloned().unwrap_or_default();
        let revision = bundle
            .metadata
            .packet_revisions
            .get(&packet_id)
            .copied()
            .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: packet_id.clone() })?;
        let node = NodeEnvelope::new(
            packet_id,
            "governance-packet",
            PacketContent {
                profile: packet.profile,
                packet_digest: digest.sha256.clone(),
                revision,
                owner,
            },
        )?;
        graph.insert_or_replay(DecisionMemoryNode::Packet(node))?;
    }
    Ok(())
}

fn insert_artifacts(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for artifact in &bundle.metadata.subject_artifacts {
        let node = NodeEnvelope::new(
            artifact.artifact_id.clone(),
            "governance-subject-artifact",
            artifact.content.clone(),
        )?;
        graph.insert_or_replay(DecisionMemoryNode::Artifact(node))?;
    }
    Ok(())
}

fn insert_claims(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    let packet_id = primary_packet_id(bundle)?;
    for (claim_id, statement) in &bundle.metadata.claims {
        let node = NodeEnvelope::new(
            claim_id.clone(),
            "governance-claim",
            ClaimContent { packet_id: packet_id.clone(), statement: statement.clone() },
        )?;
        graph.insert_or_replay(DecisionMemoryNode::Claim(node))?;
    }
    Ok(())
}

fn insert_requirements(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for requirement in &bundle.metadata.required_evidence {
        let node = NodeEnvelope::new(
            requirement.requirement_id.clone(),
            "verification-policy",
            requirement.clone(),
        )?;
        graph.insert_or_replay(DecisionMemoryNode::VerificationRequirement(node))?;
    }
    Ok(())
}

fn insert_evidence(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for evidence in &bundle.metadata.provided_evidence {
        let mut node =
            NodeEnvelope::new(evidence.evidence_id.clone(), "external-evidence", evidence.clone())?;
        if !evidence.fresh {
            node.freshness = super::FreshnessState::Stale {
                observed_digest: node.content_digest().sha256.clone(),
                reason: "evidence binding was stale at admission".to_string(),
                reason_chain: vec![evidence.evidence_id.clone()],
            };
        }
        graph.insert_or_replay(DecisionMemoryNode::Evidence(node))?;
    }
    Ok(())
}

fn insert_approvals(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for approval in &bundle.metadata.approvals {
        let mut node =
            NodeEnvelope::new(approval.approval_id.clone(), "named-authority", approval.clone())?;
        if !approval.fresh {
            node.freshness = super::FreshnessState::Stale {
                observed_digest: node.content_digest().sha256.clone(),
                reason: "approval binding was stale at admission".to_string(),
                reason_chain: vec![approval.approval_id.clone()],
            };
        }
        graph.insert_or_replay(DecisionMemoryNode::Approval(node))?;
    }
    Ok(())
}

fn insert_history(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    let owner = bundle.metadata.owners.first().cloned().unwrap_or_default();
    for (index, statement) in bundle.metadata.assumptions.iter().enumerate() {
        graph.insert_or_replay(DecisionMemoryNode::Assumption(NodeEnvelope::new(
            NodeId::new(format!(
                "assumption-{}-{:04}",
                typed_string(&bundle.contract.bundle_id)?,
                index + 1
            )),
            "governance-bundle",
            AssumptionContent { statement: statement.clone(), owner: owner.clone() },
        )?))?;
    }
    for (index, description) in bundle.metadata.alternatives.iter().enumerate() {
        graph.insert_or_replay(DecisionMemoryNode::Alternative(NodeEnvelope::new(
            NodeId::new(format!(
                "alternative-{}-{:04}",
                typed_string(&bundle.contract.bundle_id)?,
                index + 1
            )),
            "governance-bundle",
            AlternativeContent {
                description: description.clone(),
                disposition: "not-selected".to_string(),
            },
        )?))?;
    }
    for (index, condition) in bundle.metadata.triggers.iter().enumerate() {
        graph.insert_or_replay(DecisionMemoryNode::Trigger(NodeEnvelope::new(
            NodeId::new(format!(
                "trigger-{}-{:04}",
                typed_string(&bundle.contract.bundle_id)?,
                index + 1
            )),
            "governance-bundle",
            TriggerContent { condition: condition.clone(), owner: owner.clone() },
        )?))?;
    }
    Ok(())
}

fn insert_risk_acceptances(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for acceptance in &bundle.metadata.risk_acceptances {
        let mut node = NodeEnvelope::new(
            acceptance.acceptance_id.clone(),
            "named-risk-acceptance",
            acceptance.clone(),
        )?;
        if !acceptance.fresh {
            node.freshness = super::FreshnessState::Stale {
                observed_digest: node.content_digest().sha256.clone(),
                reason: "risk acceptance binding was stale at admission".to_string(),
                reason_chain: vec![acceptance.acceptance_id.clone()],
            };
        }
        graph.insert_or_replay(DecisionMemoryNode::RiskAcceptance(node))?;
    }
    Ok(())
}

pub(crate) fn record_terminal_decision(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
    decision: GovernanceDecision,
) -> Result<NodeId, DecisionMemoryError> {
    let bundle_id = typed_string(&bundle.contract.bundle_id)?;
    let decision_id = NodeId::new(format!(
        "decision-{bundle_id}-r{}-{}",
        bundle.metadata.decision_memory_revision,
        decision.as_str()
    ));
    let node = NodeEnvelope::new(
        decision_id.clone(),
        "deterministic-governance",
        DecisionContent {
            bundle_id: bundle_id.clone(),
            rationale: bundle.metadata.rationale.clone(),
            alternatives: bundle.metadata.alternatives.clone(),
            assumptions: bundle.metadata.assumptions.clone(),
            triggers: bundle.metadata.triggers.clone(),
            owners: bundle.metadata.owners.clone(),
            decision,
        },
    )?;
    graph.insert_or_replay(DecisionMemoryNode::Decision(node))?;
    add_decision_edges(graph, &decision_id, &active_node_ids(bundle)?)?;
    Ok(decision_id)
}

fn add_bundle_edges(
    graph: &mut DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
) -> Result<(), DecisionMemoryError> {
    for packet in &bundle.contract.packets {
        let packet_id = typed_string(&packet.packet_id)?;
        for target in &packet.cross_packet_references {
            let target_id = NodeId::new(typed_string(target)?);
            graph.add_edge(DependencyEdge::new(
                NodeId::new(&packet_id),
                target_id.clone(),
                DependencyKind::Binding,
            ))?;
            graph.add_edge(DependencyEdge::new(
                target_id,
                NodeId::new(&packet_id),
                DependencyKind::FreshnessImpact,
            ))?;
        }
    }
    for artifact in &bundle.metadata.subject_artifacts {
        graph.add_edge(DependencyEdge::new(
            artifact.packet_id.clone(),
            artifact.artifact_id.clone(),
            DependencyKind::FreshnessImpact,
        ))?;
        for requirement in &bundle.metadata.required_evidence {
            if requirement.artifact_ids.contains(&artifact.artifact_id) {
                graph.add_edge(DependencyEdge::new(
                    artifact.artifact_id.clone(),
                    requirement.requirement_id.clone(),
                    DependencyKind::FreshnessImpact,
                ))?;
            }
        }
        for evidence in &bundle.metadata.provided_evidence {
            if evidence.artifact_ids.contains(&artifact.artifact_id) {
                graph.add_edge(DependencyEdge::new(
                    artifact.artifact_id.clone(),
                    evidence.evidence_id.clone(),
                    DependencyKind::Evidence,
                ))?;
            }
        }
        for approval in &bundle.metadata.approvals {
            if approval.artifact_ids.contains(&artifact.artifact_id) {
                graph.add_edge(DependencyEdge::new(
                    artifact.artifact_id.clone(),
                    approval.approval_id.clone(),
                    DependencyKind::Authority,
                ))?;
            }
        }
    }
    for claim_id in bundle.metadata.claims.keys() {
        let packet_id = primary_packet_id(bundle)?;
        graph.add_edge(DependencyEdge::new(
            packet_id.clone(),
            claim_id.clone(),
            DependencyKind::Binding,
        ))?;
        graph.add_edge(DependencyEdge::new(
            packet_id,
            claim_id.clone(),
            DependencyKind::FreshnessImpact,
        ))?;
    }
    for requirement in &bundle.metadata.required_evidence {
        for claim_id in &requirement.claim_ids {
            graph.add_edge(DependencyEdge::new(
                claim_id.clone(),
                requirement.requirement_id.clone(),
                DependencyKind::FreshnessImpact,
            ))?;
        }
    }
    for evidence in &bundle.metadata.provided_evidence {
        for claim_id in &evidence.claim_ids {
            graph.add_edge(DependencyEdge::new(
                claim_id.clone(),
                evidence.evidence_id.clone(),
                DependencyKind::Evidence,
            ))?;
        }
        for requirement_id in &evidence.requirement_ids {
            graph.add_edge(DependencyEdge::new(
                requirement_id.clone(),
                evidence.evidence_id.clone(),
                DependencyKind::FreshnessImpact,
            ))?;
        }
    }
    for approval in &bundle.metadata.approvals {
        for claim_id in &approval.claim_ids {
            graph.add_edge(DependencyEdge::new(
                claim_id.clone(),
                approval.approval_id.clone(),
                DependencyKind::Authority,
            ))?;
        }
        for evidence_id in &approval.evidence_ids {
            graph.add_edge(DependencyEdge::new(
                evidence_id.clone(),
                approval.approval_id.clone(),
                DependencyKind::Authority,
            ))?;
        }
        for requirement_id in &approval.requirement_ids {
            graph.add_edge(DependencyEdge::new(
                requirement_id.clone(),
                approval.approval_id.clone(),
                DependencyKind::Authority,
            ))?;
        }
    }
    for acceptance in &bundle.metadata.risk_acceptances {
        graph.add_edge(DependencyEdge::new(
            acceptance.approval_id.clone(),
            acceptance.acceptance_id.clone(),
            DependencyKind::Authority,
        ))?;
    }
    Ok(())
}

fn add_decision_edges(
    graph: &mut DecisionMemoryGraph,
    decision_id: &NodeId,
    active_nodes: &BTreeSet<NodeId>,
) -> Result<(), DecisionMemoryError> {
    let dependencies = graph
        .nodes()
        .filter(|node| active_nodes.contains(node.id()))
        .map(|node| {
            let kind = match node {
                DecisionMemoryNode::Approval(_) => DependencyKind::Authority,
                DecisionMemoryNode::Evidence(_) => DependencyKind::Evidence,
                _ => DependencyKind::FreshnessImpact,
            };
            (node.id().clone(), kind)
        })
        .collect::<Vec<_>>();
    for (source, kind) in dependencies {
        graph.add_edge(DependencyEdge::new(source, decision_id.clone(), kind))?;
    }
    Ok(())
}

pub(crate) fn active_node_ids(
    bundle: &BoundGovernanceBundle,
) -> Result<BTreeSet<NodeId>, DecisionMemoryError> {
    let bundle_id = typed_string(&bundle.contract.bundle_id)?;
    let mut active = BTreeSet::new();
    for packet in &bundle.contract.packets {
        let packet_id = typed_string(&packet.packet_id)?;
        active.insert(NodeId::new(&packet_id));
    }
    active.extend(
        bundle.metadata.subject_artifacts.iter().map(|artifact| artifact.artifact_id.clone()),
    );
    active.extend(bundle.metadata.claims.keys().cloned());
    active
        .extend(bundle.metadata.required_evidence.iter().map(|value| value.requirement_id.clone()));
    active.extend(bundle.metadata.provided_evidence.iter().map(|value| value.evidence_id.clone()));
    active.extend(bundle.metadata.approvals.iter().map(|value| value.approval_id.clone()));
    active.extend(bundle.metadata.risk_acceptances.iter().map(|value| value.acceptance_id.clone()));
    active.extend(
        bundle
            .metadata
            .assumptions
            .iter()
            .enumerate()
            .map(|(index, _)| NodeId::new(format!("assumption-{bundle_id}-{:04}", index + 1))),
    );
    active.extend(
        bundle
            .metadata
            .alternatives
            .iter()
            .enumerate()
            .map(|(index, _)| NodeId::new(format!("alternative-{bundle_id}-{:04}", index + 1))),
    );
    active.extend(
        bundle
            .metadata
            .triggers
            .iter()
            .enumerate()
            .map(|(index, _)| NodeId::new(format!("trigger-{bundle_id}-{:04}", index + 1))),
    );
    Ok(active)
}

pub(crate) fn external_evidence(
    evidence: &EvidenceContent,
    claims: &BTreeMap<NodeId, String>,
) -> Result<ExternalVerificationEvidence, DecisionMemoryError> {
    Ok(ExternalVerificationEvidence {
        reviewer_identity: evidence.lineage.split('/').next().unwrap_or_default().to_string(),
        lineage: evidence.lineage.clone(),
        independent_context_identity: evidence.independent_context_identity.clone(),
        claims: claim_values(&evidence.claim_ids, claims)?.into_iter().map(Claim::new).collect(),
        findings: Vec::new(),
        evidence_references: evidence.references.iter().map(EvidenceReference::new).collect(),
        challenge_tier: evidence.challenge_tier,
        named_override: evidence.named_override.clone(),
    })
}

pub(crate) fn claim_values(
    ids: &[NodeId],
    claims: &BTreeMap<NodeId, String>,
) -> Result<Vec<String>, DecisionMemoryError> {
    ids.iter()
        .map(|id| {
            claims
                .get(id)
                .cloned()
                .ok_or_else(|| DecisionMemoryError::NodeNotFound { node_id: id.clone() })
        })
        .collect()
}

pub(crate) fn typed_string<T: Serialize>(value: &T) -> Result<String, DecisionMemoryError> {
    serde_json::to_value(value)
        .map_err(DecisionMemoryError::serialization)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| DecisionMemoryError::invalid_structure("typed identity was not a string"))
}

pub(crate) fn recompute_bundle_digest(
    bundle: &BoundGovernanceBundle,
) -> Result<ContentDigest, DecisionMemoryError> {
    let bundle_id = typed_string(&bundle.contract.bundle_id)?;
    ContentDigest::compute(
        "governance-bundle",
        &BundleDigestInput {
            contract_version: bundle.contract.contract_version,
            bundle_id: &bundle_id,
            profile: bundle.contract.profile,
            packets: &bundle.contract.packets,
            authority: &bundle.contract.authority,
            required_evidence: &bundle.contract.required_evidence,
            decision_memory_revision: bundle.contract.decision_memory_revision,
            metadata: &bundle.metadata,
        },
    )
}

fn primary_packet_id(bundle: &BoundGovernanceBundle) -> Result<NodeId, DecisionMemoryError> {
    bundle
        .contract
        .packets
        .first()
        .ok_or_else(|| DecisionMemoryError::invalid_structure("bundle has no primary packet"))
        .and_then(|packet| typed_string(&packet.packet_id))
        .map(NodeId::new)
}

fn require_nonblank(value: &str, field: &str) -> Result<(), DecisionMemoryError> {
    if value.trim().is_empty() {
        Err(DecisionMemoryError::invalid_structure(format!("{field} must not be blank")))
    } else {
        Ok(())
    }
}

fn require_nonempty<T>(values: &[T], field: &str) -> Result<(), DecisionMemoryError> {
    if values.is_empty() {
        Err(DecisionMemoryError::invalid_structure(format!("{field} must not be empty")))
    } else {
        Ok(())
    }
}

fn require_unique_strings(
    values: &[String],
    kind: &'static str,
) -> Result<(), DecisionMemoryError> {
    let mut seen = BTreeSet::new();
    for value in values {
        require_nonblank(value, kind)?;
        if !seen.insert(value) {
            return Err(DecisionMemoryError::DuplicateIdentity { kind, identity: value.clone() });
        }
    }
    Ok(())
}

fn require_unique_ids<'a>(
    values: impl Iterator<Item = &'a NodeId>,
    kind: &'static str,
) -> Result<(), DecisionMemoryError> {
    let mut seen = BTreeSet::new();
    for value in values {
        require_nonblank(value.as_str(), kind)?;
        if !seen.insert(value) {
            return Err(DecisionMemoryError::DuplicateIdentity {
                kind,
                identity: value.to_string(),
            });
        }
    }
    Ok(())
}
