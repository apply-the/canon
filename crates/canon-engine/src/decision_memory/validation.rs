//! Ordered fail-closed validation phases for governance bundles.

use std::collections::BTreeSet;

use canon_adapters::reviewer::{
    EvidenceFreshness, ExternalEvidenceValidationContext, TerminalEvidenceState,
    validate_external_evidence,
};
use canon_contracts::{ChallengeTier, Claim, EvidenceReference, Profile, VerificationKind};
use serde::{Deserialize, Serialize};

use super::bundle::{
    active_node_ids, claim_values, external_evidence, recompute_bundle_digest,
    record_terminal_decision, typed_string,
};
use super::digest::is_sha256;
use super::graph::{DECISION_MEMORY_CONTRACT_LINE, DECISION_MEMORY_SCHEMA_VERSION};
use super::{BoundGovernanceBundle, DecisionMemoryGraph, FreshnessState, NodeId};

const VALIDATOR_IDENTITY: &str = "canon-deterministic-governance";
const INVOCATION_IDENTITY: &str = "canon-validation-invocation";

/// Explicit deterministic validation lifecycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationPhase {
    /// Decode typed input.
    Parse,
    /// Normalize set-like collections.
    Normalize,
    /// Validate required fields and closed values.
    ValidateStructure,
    /// Validate exact graph references.
    ValidateCrossReferences,
    /// Validate named authority and approvals.
    ValidateAuthority,
    /// Validate exact evidence requirements.
    ValidateEvidenceRequirements,
    /// Validate exact-binding freshness.
    ValidateFreshness,
    /// Record a terminal deterministic result.
    RecordDecision,
    /// Produce a deterministic projection.
    Project,
}

/// Stable internal reason classifications for deterministic findings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationCode {
    /// A graph edge references an absent node.
    DanglingReference,
    /// A required approval was absent.
    ApprovalMissing,
    /// An approval did not exactly match owner, packet, claims, zone, or freshness.
    ApprovalBindingMismatch,
    /// A required evidence record was absent.
    EvidenceMissing,
    /// Evidence did not exactly match packet and claims.
    EvidenceBindingMismatch,
    /// External evidence lineage or independent context was invalid.
    EvidenceLineageInvalid,
    /// Evidence challenge tier did not satisfy policy.
    ChallengeTierInsufficient,
    /// A required node was stale.
    StaleBinding,
    /// A closed value or schema identity was unsupported.
    Unsupported,
    /// A required structural invariant was malformed.
    InvalidStructure,
    /// A typed content conflict prevented admission.
    Conflict,
}

/// One deterministic, stably ordered validation finding.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Phase that produced the finding.
    pub phase: ValidationPhase,
    /// Stable reason code.
    pub code: ValidationCode,
    /// Exact affected identity, when available.
    pub node_id: Option<NodeId>,
    /// Stable diagnostic without volatile environment details.
    pub message: String,
}

/// Terminal deterministic governance status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    /// All deterministic bindings and required external evidence structure passed.
    Accepted,
    /// A deterministic rule rejected the governed request.
    Rejected,
    /// Named authority was absent or insufficient.
    Blocked,
    /// Required evidence was absent or insufficient.
    RequiredMissing,
    /// An exact dependency changed after admission.
    Stale,
    /// A contract line, schema, or closed value is unsupported.
    Unsupported,
    /// The same identity carried incompatible content.
    Conflict,
}

impl GovernanceDecision {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Blocked => "blocked",
            Self::RequiredMissing => "required-missing",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
            Self::Conflict => "conflict",
        }
    }
}

/// Auditable proof that deterministic governance invoked no external capability.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAuditCounters {
    /// Child-process invocations.
    pub process_invocations: u64,
    /// Network invocations.
    pub network_invocations: u64,
    /// Provider credential reads.
    pub provider_credential_reads: u64,
    /// Model calls.
    pub model_calls: u64,
    /// Semantic evidence records created by Canon.
    pub semantic_evidence_created: u64,
}

/// Complete terminal result suitable for atomic persistence with the graph.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceValidationResult {
    /// Terminal deterministic status.
    pub decision: GovernanceDecision,
    /// Ordered phases actually completed.
    pub phases: Vec<ValidationPhase>,
    /// Stably ordered deterministic findings.
    pub findings: Vec<ValidationFinding>,
    /// Graph digest validated by this result.
    pub graph_digest: String,
    /// Canon never asserts semantic truth for external findings.
    pub semantic_truth_asserted: bool,
    /// Exact terminal decision node recorded for this result.
    pub decision_node_id: Option<NodeId>,
    /// Capability counters for the deterministic kernel.
    pub execution_audit: ExecutionAuditCounters,
}

/// Validates one bundle and graph without invoking any external actor.
pub fn validate_governance_bundle(
    bundle: &BoundGovernanceBundle,
    graph: &mut DecisionMemoryGraph,
) -> GovernanceValidationResult {
    let mut result = {
        let mut validator = Validator::new(graph);
        validator.complete(ValidationPhase::Parse);
        validator.complete(ValidationPhase::Normalize);
        if !validator.validate_structure(bundle) {
            validator.finish(GovernanceDecision::Unsupported)
        } else if !validator.validate_cross_references(bundle) {
            validator.finish(GovernanceDecision::RequiredMissing)
        } else if !validator.validate_authority(bundle) {
            validator.finish(GovernanceDecision::Blocked)
        } else if !validator.validate_evidence(bundle) {
            validator.finish(GovernanceDecision::RequiredMissing)
        } else if !validator.validate_freshness(bundle) {
            validator.finish(GovernanceDecision::Stale)
        } else {
            validator.finish(GovernanceDecision::Accepted)
        }
    };
    let graph_is_recordable = !graph.has_freshness_cycle() && graph.dangling_edges().is_empty();
    if graph_is_recordable {
        result.phases.push(ValidationPhase::RecordDecision);
        match record_terminal_decision(graph, bundle, result.decision) {
            Ok(node_id) => result.decision_node_id = Some(node_id),
            Err(error) => {
                result.decision = GovernanceDecision::Conflict;
                result.findings.push(ValidationFinding {
                    phase: ValidationPhase::RecordDecision,
                    code: ValidationCode::Conflict,
                    node_id: None,
                    message: error.to_string(),
                });
            }
        }
        result.phases.push(ValidationPhase::Project);
    }
    result.findings.sort();
    if result.decision_node_id.is_some()
        && let Ok(bundle_id) = typed_string(&bundle.contract.bundle_id)
    {
        graph.record_validation_event(bundle_id, bundle.metadata.decision_memory_revision);
    }
    result.graph_digest = graph
        .digest()
        .map(|digest| digest.sha256)
        .unwrap_or_else(|_| "digest-unavailable".to_string());
    result
}

struct Validator<'a> {
    graph: &'a DecisionMemoryGraph,
    phases: Vec<ValidationPhase>,
    findings: BTreeSet<ValidationFinding>,
}

impl<'a> Validator<'a> {
    fn new(graph: &'a DecisionMemoryGraph) -> Self {
        Self { graph, phases: Vec::new(), findings: BTreeSet::new() }
    }

    fn complete(&mut self, phase: ValidationPhase) {
        self.phases.push(phase);
    }

    fn validate_structure(&mut self, bundle: &BoundGovernanceBundle) -> bool {
        self.complete(ValidationPhase::ValidateStructure);
        if self.graph.schema_version != DECISION_MEMORY_SCHEMA_VERSION
            || self.graph.contract_line != DECISION_MEMORY_CONTRACT_LINE
        {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateStructure,
                code: ValidationCode::Unsupported,
                node_id: None,
                message: "unsupported graph schema or contract line".to_string(),
            });
        }
        let bundle_digest = typed_string(&bundle.contract.bundle_digest).unwrap_or_default();
        let digest_matches =
            recompute_bundle_digest(bundle).is_ok_and(|expected| expected.sha256 == bundle_digest);
        if !is_sha256(&bundle_digest) || !digest_matches {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateStructure,
                code: ValidationCode::InvalidStructure,
                node_id: None,
                message: "governance bundle digest is malformed or mismatched".to_string(),
            });
        }
        if self.graph.has_freshness_cycle() {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateStructure,
                code: ValidationCode::Conflict,
                node_id: None,
                message: "freshness or authority dependencies contain a cycle".to_string(),
            });
        }
        if self.graph.has_invalid_node_digest() {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateStructure,
                code: ValidationCode::Conflict,
                node_id: None,
                message: "recorded node digest does not match typed node content".to_string(),
            });
        }
        if self.graph.has_invalid_declared_freshness() {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateStructure,
                code: ValidationCode::Conflict,
                node_id: None,
                message: "declared stale content is represented as fresh".to_string(),
            });
        }
        match (DecisionMemoryGraph::from_bundle(bundle), active_node_ids(bundle)) {
            (Ok(expected), Ok(active)) => {
                for node_id in &active {
                    let mismatch =
                        expected.node(node_id).ok().zip(self.graph.node(node_id).ok()).is_some_and(
                            |(expected, actual)| {
                                expected.content_digest() != actual.content_digest()
                            },
                        );
                    if mismatch {
                        self.findings.insert(ValidationFinding {
                            phase: ValidationPhase::ValidateStructure,
                            code: ValidationCode::Conflict,
                            node_id: Some(node_id.clone()),
                            message: "graph node does not match the bound bundle".to_string(),
                        });
                    }
                }
                let expected_edges = expected
                    .edges()
                    .filter(|edge| active.contains(&edge.source) && active.contains(&edge.target))
                    .cloned()
                    .collect::<BTreeSet<_>>();
                let actual_edges = self
                    .graph
                    .edges()
                    .filter(|edge| active.contains(&edge.source) && active.contains(&edge.target))
                    .cloned()
                    .collect::<BTreeSet<_>>();
                if expected_edges != actual_edges {
                    self.findings.insert(ValidationFinding {
                        phase: ValidationPhase::ValidateStructure,
                        code: ValidationCode::Conflict,
                        node_id: None,
                        message: "graph edges do not match the bound bundle".to_string(),
                    });
                }
            }
            _ => {
                self.findings.insert(ValidationFinding {
                    phase: ValidationPhase::ValidateStructure,
                    code: ValidationCode::Conflict,
                    node_id: None,
                    message: "bound graph could not be reconstructed".to_string(),
                });
            }
        }
        !self.findings.iter().any(|finding| finding.phase == ValidationPhase::ValidateStructure)
    }

    fn validate_cross_references(&mut self, bundle: &BoundGovernanceBundle) -> bool {
        self.complete(ValidationPhase::ValidateCrossReferences);
        for edge in self.graph.dangling_edges() {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateCrossReferences,
                code: ValidationCode::DanglingReference,
                node_id: Some(edge.target.clone()),
                message: format!("{} references missing {}", edge.source, edge.target),
            });
        }
        if let Ok(active) = active_node_ids(bundle) {
            for node_id in active {
                if self.graph.node(&node_id).is_err() {
                    self.findings.insert(ValidationFinding {
                        phase: ValidationPhase::ValidateCrossReferences,
                        code: ValidationCode::DanglingReference,
                        node_id: Some(node_id.clone()),
                        message: format!("active governance node {node_id} is missing"),
                    });
                }
            }
        }
        self.findings.is_empty()
    }

    fn validate_authority(&mut self, bundle: &BoundGovernanceBundle) -> bool {
        self.complete(ValidationPhase::ValidateAuthority);
        let required = bundle.contract.authority.required_approvers.iter().collect::<BTreeSet<_>>();
        let expected_claims = bundle.metadata.claims.keys().cloned().collect::<Vec<_>>();
        let expected_artifacts = bundle
            .metadata
            .subject_artifacts
            .iter()
            .map(|artifact| artifact.artifact_id.clone())
            .collect::<Vec<_>>();
        let expected_evidence = bundle
            .metadata
            .provided_evidence
            .iter()
            .map(|evidence| evidence.evidence_id.clone())
            .collect::<Vec<_>>();
        let expected_requirements = bundle
            .metadata
            .required_evidence
            .iter()
            .map(|requirement| requirement.requirement_id.clone())
            .collect::<Vec<_>>();
        let expected_packet = bundle
            .metadata
            .required_evidence
            .first()
            .map(|requirement| requirement.packet_id.clone());
        for approver in required {
            let exact = bundle.metadata.approvals.iter().any(|approval| {
                approval.approver == *approver
                    && approval.authority_zone == bundle.metadata.authority_zone
                    && Some(&approval.packet_id) == expected_packet.as_ref()
                    && approval.claim_ids == expected_claims
                    && approval.artifact_ids == expected_artifacts
                    && approval.evidence_ids == expected_evidence
                    && approval.requirement_ids == expected_requirements
                    && approval.approved
                    && approval.decision_memory_revision == bundle.metadata.decision_memory_revision
                    && approval
                        .valid_through_revision
                        .is_none_or(|revision| revision >= bundle.metadata.decision_memory_revision)
            });
            if !exact {
                self.findings.insert(ValidationFinding {
                    phase: ValidationPhase::ValidateAuthority,
                    code: ValidationCode::ApprovalMissing,
                    node_id: None,
                    message: format!("required approver {approver} has no exact fresh approval"),
                });
            }
        }
        !self.findings.iter().any(|finding| finding.phase == ValidationPhase::ValidateAuthority)
    }

    fn validate_evidence(&mut self, bundle: &BoundGovernanceBundle) -> bool {
        self.complete(ValidationPhase::ValidateEvidenceRequirements);
        for requirement in &bundle.metadata.required_evidence {
            if !requirement_matches_profile_policy(bundle, requirement) {
                self.findings.insert(ValidationFinding {
                    phase: ValidationPhase::ValidateEvidenceRequirements,
                    code: ValidationCode::EvidenceBindingMismatch,
                    node_id: Some(requirement.requirement_id.clone()),
                    message: "caller requirement is weaker than the Canon profile policy"
                        .to_string(),
                });
                continue;
            }
            let exact_candidates = bundle.metadata.provided_evidence.iter().filter(|evidence| {
                evidence.packet_id == requirement.packet_id
                    && evidence.claim_ids == requirement.claim_ids
                    && evidence.artifact_ids == requirement.artifact_ids
                    && evidence.requirement_ids.contains(&requirement.requirement_id)
            });
            let mut accepted = false;
            for evidence in exact_candidates {
                if self.validate_evidence_candidate(bundle, requirement, evidence) {
                    accepted = true;
                }
            }
            if !accepted {
                self.findings.insert(ValidationFinding {
                    phase: ValidationPhase::ValidateEvidenceRequirements,
                    code: ValidationCode::EvidenceMissing,
                    node_id: Some(requirement.requirement_id.clone()),
                    message: "no exact supplied evidence satisfies the requirement".to_string(),
                });
            }
        }
        !self
            .findings
            .iter()
            .any(|finding| finding.phase == ValidationPhase::ValidateEvidenceRequirements)
    }

    fn validate_evidence_candidate(
        &mut self,
        bundle: &BoundGovernanceBundle,
        requirement: &super::VerificationRequirementContent,
        evidence: &super::EvidenceContent,
    ) -> bool {
        if self.graph.node(&evidence.evidence_id).is_err() {
            return false;
        }
        if evidence.references.is_empty()
            || evidence.references.iter().any(|reference| !is_immutable_reference(reference))
            || evidence.references != requirement.accepted_evidence_references
        {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateEvidenceRequirements,
                code: ValidationCode::EvidenceBindingMismatch,
                node_id: Some(evidence.evidence_id.clone()),
                message: "evidence reference is not an immutable digest binding".to_string(),
            });
            return false;
        }
        if tier_rank(evidence.challenge_tier) < tier_rank(requirement.minimum_challenge_tier) {
            self.findings.insert(ValidationFinding {
                phase: ValidationPhase::ValidateEvidenceRequirements,
                code: ValidationCode::ChallengeTierInsufficient,
                node_id: Some(evidence.evidence_id.clone()),
                message: "supplied challenge tier is below the exact requirement".to_string(),
            });
            return false;
        }
        match requirement.kind {
            VerificationKind::Deterministic => !evidence.external_semantic,
            VerificationKind::ExternalSemanticReview => {
                self.validate_external_candidate(bundle, requirement, evidence)
            }
        }
    }

    fn validate_external_candidate(
        &mut self,
        bundle: &BoundGovernanceBundle,
        requirement: &super::VerificationRequirementContent,
        evidence: &super::EvidenceContent,
    ) -> bool {
        if !evidence.external_semantic {
            return false;
        }
        let external = match external_evidence(evidence, &bundle.metadata.claims) {
            Ok(value) => value,
            Err(_) => return false,
        };
        let required_claims = match claim_values(&requirement.claim_ids, &bundle.metadata.claims) {
            Ok(values) => values.into_iter().map(Claim::new).collect(),
            Err(_) => return false,
        };
        let admitted_lineage = admitted_override_lineage(self.graph, bundle, requirement, evidence);
        let forbidden_lineages = bundle
            .metadata
            .forbidden_lineages
            .iter()
            .filter(|lineage| Some(lineage.as_str()) != admitted_lineage.as_deref())
            .cloned()
            .collect();
        let context = ExternalEvidenceValidationContext {
            canon_validator_identity: VALIDATOR_IDENTITY.to_string(),
            current_invocation_identity: INVOCATION_IDENTITY.to_string(),
            required_claims,
            accepted_evidence_references: requirement
                .accepted_evidence_references
                .iter()
                .map(EvidenceReference::new)
                .collect(),
            minimum_challenge_tier: requirement.minimum_challenge_tier,
            freshness: EvidenceFreshness::Fresh,
            terminal_state: TerminalEvidenceState::Terminal,
            forbidden_lineages,
        };
        match validate_external_evidence(&external, &context) {
            Ok(acceptance) => acceptance.structure_valid && !acceptance.semantic_judgment_asserted,
            Err(_) => {
                self.findings.insert(ValidationFinding {
                    phase: ValidationPhase::ValidateEvidenceRequirements,
                    code: ValidationCode::EvidenceLineageInvalid,
                    node_id: Some(evidence.evidence_id.clone()),
                    message: "external evidence failed deterministic lineage validation"
                        .to_string(),
                });
                false
            }
        }
    }

    fn validate_freshness(&mut self, bundle: &BoundGovernanceBundle) -> bool {
        self.complete(ValidationPhase::ValidateFreshness);
        let active = active_node_ids(bundle).unwrap_or_default();
        for node in self.graph.nodes().filter(|node| active.contains(node.id())) {
            if let FreshnessState::Stale { .. } = node.freshness() {
                self.findings.insert(ValidationFinding {
                    phase: ValidationPhase::ValidateFreshness,
                    code: ValidationCode::StaleBinding,
                    node_id: Some(node.id().clone()),
                    message: "exact graph binding is stale".to_string(),
                });
            }
        }
        !self.findings.iter().any(|finding| finding.phase == ValidationPhase::ValidateFreshness)
    }

    fn finish(self, decision: GovernanceDecision) -> GovernanceValidationResult {
        GovernanceValidationResult {
            decision,
            phases: self.phases,
            findings: self.findings.into_iter().collect(),
            graph_digest: String::new(),
            semantic_truth_asserted: false,
            decision_node_id: None,
            execution_audit: ExecutionAuditCounters::default(),
        }
    }
}

const fn tier_rank(tier: ChallengeTier) -> u8 {
    match tier {
        ChallengeTier::Tier0 => 0,
        ChallengeTier::Tier1 => 1,
        ChallengeTier::Tier2 => 2,
        ChallengeTier::Tier3 => 3,
    }
}

fn admitted_override_lineage(
    graph: &DecisionMemoryGraph,
    bundle: &BoundGovernanceBundle,
    requirement: &super::VerificationRequirementContent,
    evidence: &super::EvidenceContent,
) -> Option<String> {
    if requirement.minimum_challenge_tier != ChallengeTier::Tier2
        || evidence.challenge_tier != ChallengeTier::Tier2
    {
        return None;
    }
    let override_id = evidence.named_override.as_deref()?;
    bundle.metadata.risk_acceptances.iter().find_map(|acceptance| {
        let approval_matches = bundle.metadata.approvals.iter().any(|approval| {
            approval.approval_id == acceptance.approval_id
                && approval.approver == acceptance.owner
                && approval.packet_id == requirement.packet_id
                && approval.claim_ids == requirement.claim_ids
                && approval.artifact_ids == requirement.artifact_ids
                && approval.evidence_ids.contains(&evidence.evidence_id)
                && approval.requirement_ids.contains(&requirement.requirement_id)
                && approval.authority_zone == bundle.metadata.authority_zone
                && approval.approved
                && approval.decision_memory_revision == bundle.metadata.decision_memory_revision
                && approval
                    .valid_through_revision
                    .is_none_or(|revision| revision >= bundle.metadata.decision_memory_revision)
                && approval.fresh
        });
        let graph_fresh = graph
            .freshness(&acceptance.acceptance_id)
            .is_ok_and(|freshness| freshness == &FreshnessState::Fresh)
            && graph
                .freshness(&acceptance.approval_id)
                .is_ok_and(|freshness| freshness == &FreshnessState::Fresh);
        let admitted = acceptance.acceptance_id.as_str() == override_id
            && acceptance.packet_id == requirement.packet_id
            && acceptance.challenge_tier == ChallengeTier::Tier2
            && acceptance.fresh
            && !acceptance.justification.trim().is_empty()
            && bundle.contract.authority.required_approvers.contains(&acceptance.owner)
            && bundle.metadata.forbidden_lineages.contains(&acceptance.lineage)
            && evidence.lineage.contains(&acceptance.lineage)
            && acceptance.risk == format!("same-lineage:{}", acceptance.lineage)
            && approval_matches
            && graph_fresh;
        admitted.then(|| acceptance.lineage.clone())
    })
}

fn is_immutable_reference(reference: &str) -> bool {
    reference.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn requirement_matches_profile_policy(
    bundle: &BoundGovernanceBundle,
    requirement: &super::VerificationRequirementContent,
) -> bool {
    match requirement.kind {
        VerificationKind::Deterministic => {
            let deterministic_permitted = matches!(bundle.contract.profile, Profile::Verification)
                || (bundle.metadata.no_change
                    && matches!(bundle.contract.profile, Profile::Discovery | Profile::Backlog));
            deterministic_permitted
                && bundle.metadata.risk_tier == 0
                && requirement.minimum_challenge_tier == ChallengeTier::Tier0
        }
        VerificationKind::ExternalSemanticReview => {
            tier_rank(requirement.minimum_challenge_tier)
                >= tier_rank(profile_minimum_tier(
                    bundle.contract.profile,
                    bundle.metadata.risk_tier,
                ))
        }
    }
}

const fn profile_minimum_tier(profile: Profile, risk_tier: u8) -> ChallengeTier {
    if risk_tier >= 3 || matches!(profile, Profile::Incident) {
        return ChallengeTier::Tier3;
    }
    match profile {
        Profile::Discovery | Profile::Requirements | Profile::Backlog => ChallengeTier::Tier1,
        Profile::Architecture
        | Profile::Change
        | Profile::Refactor
        | Profile::Verification
        | Profile::PrReview
        | Profile::Incident => ChallengeTier::Tier2,
    }
}
