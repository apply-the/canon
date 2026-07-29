//! Shared deterministic governance service for stable CLI and RPC surfaces.
//!
//! The service is deliberately repository-local and capability-free. It owns
//! admission, exact replay, projection, and durable decision-memory storage so
//! transports cannot substitute synthetic success for a kernel result.

use canon_contracts::DecisionMemoryProjection;
use canon_engine::EngineService;
use canon_engine::decision_memory::{
    DecisionMemoryError, DecisionMemoryGraph, DecisionMemoryStore, DecisionMemoryStoreSnapshot,
    ExecutionAuditCounters, GovernanceBundleDraft, GovernanceDecision, build_governance_bundle,
    project_decision_memory, validate_governance_bundle,
};
use serde::Serialize;

use crate::error::{CliError, CliResult};

const DECISION_MEMORY_DIRECTORY: &str = "decision-memory";
const DECISION_MEMORY_STATE_FILE: &str = "state.json";

/// Stable machine projection returned by every deterministic governance operation.
#[derive(Clone, Debug, Serialize)]
pub struct StableGovernanceResult {
    /// Terminal kernel status.
    pub terminal_status: GovernanceDecision,
    /// Canonical digest of the resulting graph.
    pub graph_digest: String,
    /// Whether a mutation was an exact replay of an admitted bundle.
    pub replayed: bool,
    /// Proof that the kernel invoked no external capability.
    pub execution_audit: ExecutionAuditCounters,
    /// Read-only decision-memory projection.
    pub decision_memory: DecisionMemoryProjection,
}

/// Admits a typed governance draft or returns its exact durable replay.
pub fn mutate(
    service: &EngineService,
    request_id: &str,
    draft: GovernanceBundleDraft,
) -> CliResult<StableGovernanceResult> {
    if request_id != draft.bundle_id {
        return Err(CliError::IdentityDigestConflict(
            "request_id must equal bundle_id for mutation".to_string(),
        ));
    }
    let candidate = build_governance_bundle(draft).map_err(invalid_governance)?;
    let store = DecisionMemoryStore::new(service.canon_runtime_dir());
    if snapshot_path(service).exists() {
        let snapshot = store.load().map_err(invalid_governance)?;
        if let Some(recorded) = snapshot
            .admitted_bundles
            .iter()
            .find(|bundle| bundle.contract.bundle_id == candidate.contract.bundle_id)
        {
            if recorded != &candidate {
                return Err(CliError::IdentityDigestConflict(
                    "bundle identity has different canonical content".to_string(),
                ));
            }
            return project_snapshot(snapshot, true);
        }
        return Err(CliError::InvalidInput(
            "stale_state: this stable line admits one terminal bundle per decision-memory store"
                .to_string(),
        ));
    }

    let mut graph = DecisionMemoryGraph::from_bundle(&candidate).map_err(invalid_governance)?;
    let terminal_result = validate_governance_bundle(&candidate, &mut graph);
    let snapshot =
        DecisionMemoryStoreSnapshot { admitted_bundles: vec![candidate], graph, terminal_result };
    store.persist(&snapshot).map_err(invalid_governance)?;
    project_snapshot(snapshot, false)
}

/// Loads the current durable state without rewriting it.
pub fn inspect(service: &EngineService) -> CliResult<StableGovernanceResult> {
    let snapshot =
        DecisionMemoryStore::new(service.canon_runtime_dir()).load().map_err(invalid_governance)?;
    project_snapshot(snapshot, false)
}

fn snapshot_path(service: &EngineService) -> std::path::PathBuf {
    service.canon_runtime_dir().join(DECISION_MEMORY_DIRECTORY).join(DECISION_MEMORY_STATE_FILE)
}

fn project_snapshot(
    snapshot: DecisionMemoryStoreSnapshot,
    replayed: bool,
) -> CliResult<StableGovernanceResult> {
    let decision_memory = project_decision_memory(&snapshot.graph).map_err(invalid_governance)?;
    Ok(StableGovernanceResult {
        terminal_status: snapshot.terminal_result.decision,
        graph_digest: snapshot.terminal_result.graph_digest,
        replayed,
        execution_audit: snapshot.terminal_result.execution_audit,
        decision_memory,
    })
}

fn invalid_governance(error: DecisionMemoryError) -> CliError {
    match error {
        DecisionMemoryError::Persistence { source, .. } => CliError::Io(source),
        other => CliError::InvalidInput(other.to_string()),
    }
}

/// Maps terminal deterministic governance states to the frozen CLI exit matrix.
pub const fn exit_code(decision: GovernanceDecision) -> i32 {
    match decision {
        GovernanceDecision::Accepted => 0,
        GovernanceDecision::Stale => 2,
        GovernanceDecision::Blocked => 3,
        GovernanceDecision::Rejected | GovernanceDecision::RequiredMissing => 5,
        GovernanceDecision::Conflict => 7,
        GovernanceDecision::Unsupported => 8,
    }
}

#[cfg(test)]
pub(super) mod tests {
    use canon_contracts::{ChallengeTier, Profile, VerificationKind};
    use canon_engine::EngineService;
    use canon_engine::decision_memory::{
        ApprovalContent, ArtifactContent, EvidenceContent, GovernanceBundleDraft,
        GovernanceDecision, GovernancePacketDraft, NodeId, SubjectArtifactBinding,
        VerificationRequirementContent,
    };

    use super::{exit_code, inspect, mutate, snapshot_path};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn require(condition: bool, message: &str) -> TestResult {
        if condition { Ok(()) } else { Err(message.to_string().into()) }
    }

    #[test]
    fn mutation_replay_conflict_inspection_and_persistence_failure_are_directly_covered()
    -> TestResult {
        let workspace = tempfile::tempdir()?;
        let service = EngineService::new(workspace.path());
        let initial_draft = draft("bundle-direct", 1);
        let first = mutate(&service, "bundle-direct", initial_draft.clone())?;
        require(!first.replayed, "first mutation was marked replayed")?;
        let replay = mutate(&service, "bundle-direct", initial_draft)?;
        require(replay.replayed, "exact mutation was not replayed")?;
        require(inspect(&service)?.graph_digest == first.graph_digest, "read projection drifted")?;
        require(
            mutate(&service, "different", draft("different-bundle", 1)).is_err(),
            "identity mismatch was accepted",
        )?;
        require(
            mutate(&service, "bundle-direct", draft("bundle-direct", 2)).is_err(),
            "digest conflict was accepted",
        )?;

        let obstructed = tempfile::tempdir()?;
        std::fs::write(obstructed.path().join(".canon"), b"not-a-directory")?;
        require(
            mutate(
                &EngineService::new(obstructed.path()),
                "bundle-obstructed",
                draft("bundle-obstructed", 1),
            )
            .is_err(),
            "persistence obstruction was accepted",
        )?;

        require(
            mutate(&service, "second-bundle", draft("second-bundle", 1)).is_err(),
            "second terminal identity was accepted",
        )?;
        for (decision, expected) in [
            (GovernanceDecision::Accepted, 0),
            (GovernanceDecision::Stale, 2),
            (GovernanceDecision::Blocked, 3),
            (GovernanceDecision::Rejected, 5),
            (GovernanceDecision::RequiredMissing, 5),
            (GovernanceDecision::Conflict, 7),
            (GovernanceDecision::Unsupported, 8),
        ] {
            require(exit_code(decision) == expected, "terminal exit-code mapping drifted")?;
        }

        let invalid_workspace = tempfile::tempdir()?;
        let invalid_service = EngineService::new(invalid_workspace.path());
        let mut invalid_draft = draft("bundle-invalid", 1);
        invalid_draft.bundle_id.clear();
        require(
            mutate(&invalid_service, "", invalid_draft).is_err(),
            "structurally invalid admission was accepted",
        )?;

        std::fs::write(snapshot_path(&service), b"{}")?;
        require(inspect(&service).is_err(), "corrupt decision-memory snapshot was projected")?;
        Ok(())
    }

    pub(crate) fn draft(bundle_id: &str, revision: u64) -> GovernanceBundleDraft {
        let packet_id = NodeId::new(format!("packet-{bundle_id}"));
        let artifact_id = NodeId::new(format!("artifact-{bundle_id}"));
        let claim_id = NodeId::new(format!("claim-{bundle_id}-main"));
        let requirement_id = NodeId::new(format!("requirement-{bundle_id}"));
        let evidence_id = NodeId::new(format!("evidence-{bundle_id}"));
        GovernanceBundleDraft {
            bundle_id: bundle_id.to_string(),
            profile: Profile::Discovery,
            decision_memory_revision: revision,
            packets: vec![GovernancePacketDraft {
                packet_id: packet_id.to_string(),
                profile: Profile::Discovery,
                revision,
                change_intent: "preserve deterministic governance".to_string(),
                scope: vec!["workspace".to_string()],
                risks: vec!["incorrect authorization".to_string()],
                invariants: vec!["no semantic execution".to_string()],
                acceptance_criteria: vec!["exact bindings validate".to_string()],
                cross_packet_references: Vec::new(),
            }],
            subject_artifacts: vec![SubjectArtifactBinding {
                artifact_id: artifact_id.clone(),
                packet_id: packet_id.clone(),
                content: ArtifactContent {
                    artifact_identity: "workspace".to_string(),
                    revision: format!("git:fixture-{revision}"),
                    content_digest: if revision == 1 { "a".repeat(64) } else { "b".repeat(64) },
                },
            }],
            required_approvers: vec!["release-owner".to_string()],
            authority_zone: "governance-release".to_string(),
            risk_tier: 1,
            change_class: "governance-kernel".to_string(),
            context_class: "repository-local".to_string(),
            owners: vec!["release-owner".to_string()],
            claims: vec!["claim-exact-binding".to_string()],
            required_evidence: vec![VerificationRequirementContent {
                requirement_id: requirement_id.clone(),
                packet_id: packet_id.clone(),
                claim_ids: vec![claim_id.clone()],
                artifact_ids: vec![artifact_id.clone()],
                kind: VerificationKind::ExternalSemanticReview,
                minimum_challenge_tier: ChallengeTier::Tier1,
                accepted_evidence_references: vec![format!("sha256:{}", "e".repeat(64))],
            }],
            provided_evidence: vec![EvidenceContent {
                evidence_id: evidence_id.clone(),
                packet_id: packet_id.clone(),
                claim_ids: vec![claim_id.clone()],
                artifact_ids: vec![artifact_id.clone()],
                requirement_ids: vec![requirement_id.clone()],
                references: vec![format!("sha256:{}", "e".repeat(64))],
                lineage: "provider:challenger/executor:review/invocation:direct".to_string(),
                independent_context_identity: "context-independent-direct".to_string(),
                challenge_tier: ChallengeTier::Tier1,
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
                decision_memory_revision: revision,
                valid_through_revision: Some(revision),
                fresh: true,
            }],
            assumptions: vec!["contract remains immutable".to_string()],
            alternatives: vec!["defer".to_string()],
            rationale: "exact bindings fail closed".to_string(),
            risk_acceptances: Vec::new(),
            triggers: vec!["packet content changes".to_string()],
            no_change: false,
        }
    }
}
