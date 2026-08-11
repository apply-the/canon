//! Atomic repository-local persistence for graph and terminal result together.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::persistence::atomic::write_bytes_durable;

use super::graph::{DECISION_MEMORY_CONTRACT_LINE, DECISION_MEMORY_SCHEMA_VERSION};
use super::topology::reconstruct_repository;
use super::{
    BoundGovernanceBundle, DecisionMemoryNode, ExecutionAuditCounters, GovernanceDecision,
    ValidationPhase,
};
use super::{DecisionMemoryError, DecisionMemoryGraph, GovernanceValidationResult};

const SNAPSHOT_SCHEMA_VERSION: &str = "canon-decision-memory-snapshot-v1";
const DECISION_MEMORY_DIRECTORY: &str = "decision-memory";
const SNAPSHOT_FILE: &str = "state.json";

/// Atomic storage unit for graph state and its terminal deterministic result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionMemoryStoreSnapshot {
    /// Exact governance bundles that justify every persisted graph node and edge.
    pub admitted_bundles: Vec<BoundGovernanceBundle>,
    /// Exact graph state.
    pub graph: DecisionMemoryGraph,
    /// Terminal result bound to the graph digest.
    pub terminal_result: GovernanceValidationResult,
}

#[derive(Serialize, Deserialize)]
struct PersistedSnapshot {
    schema_version: String,
    snapshot: DecisionMemoryStoreSnapshot,
}

/// Repository-local decision-memory store rooted under the existing `.canon`.
#[derive(Clone, Debug)]
pub struct DecisionMemoryStore {
    snapshot_path: PathBuf,
}

impl DecisionMemoryStore {
    /// Creates a store under the caller's existing Canon state root.
    pub fn new(canon_root: impl AsRef<Path>) -> Self {
        Self {
            snapshot_path: canon_root.as_ref().join(DECISION_MEMORY_DIRECTORY).join(SNAPSHOT_FILE),
        }
    }

    /// Returns the durable snapshot path for diagnostics and qualification.
    pub fn snapshot_path(&self) -> &Path {
        &self.snapshot_path
    }

    /// Atomically persists graph and terminal result as one crash-consistent file.
    pub fn persist(
        &self,
        snapshot: &DecisionMemoryStoreSnapshot,
    ) -> Result<(), DecisionMemoryError> {
        validate_snapshot(snapshot)?;
        let persisted = PersistedSnapshot {
            schema_version: SNAPSHOT_SCHEMA_VERSION.to_string(),
            snapshot: snapshot.clone(),
        };
        let bytes =
            serde_json::to_vec_pretty(&persisted).map_err(DecisionMemoryError::serialization)?;
        write_bytes_durable(&self.snapshot_path, &bytes).map_err(|source| {
            DecisionMemoryError::Persistence { path: self.snapshot_path.clone(), source }
        })
    }

    /// Loads and validates the one complete atomic snapshot.
    pub fn load(&self) -> Result<DecisionMemoryStoreSnapshot, DecisionMemoryError> {
        let bytes = fs::read(&self.snapshot_path).map_err(|source| {
            DecisionMemoryError::Persistence { path: self.snapshot_path.clone(), source }
        })?;
        let persisted: PersistedSnapshot = serde_json::from_slice(&bytes)
            .map_err(|error| DecisionMemoryError::InvalidSnapshot { message: error.to_string() })?;
        if persisted.schema_version != SNAPSHOT_SCHEMA_VERSION {
            return Err(DecisionMemoryError::InvalidSnapshot {
                message: "unsupported decision-memory snapshot schema".to_string(),
            });
        }
        validate_snapshot(&persisted.snapshot)?;
        Ok(persisted.snapshot)
    }
}

fn validate_snapshot(snapshot: &DecisionMemoryStoreSnapshot) -> Result<(), DecisionMemoryError> {
    if snapshot.graph.schema_version != DECISION_MEMORY_SCHEMA_VERSION
        || snapshot.graph.contract_line != DECISION_MEMORY_CONTRACT_LINE
    {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "unsupported graph schema or contract line".to_string(),
        });
    }
    let digest = snapshot.graph.digest()?;
    if snapshot.terminal_result.graph_digest != digest.sha256 {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "terminal result digest does not match graph".to_string(),
        });
    }
    if snapshot.graph.has_freshness_cycle() {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "freshness or authority graph contains a cycle".to_string(),
        });
    }
    if snapshot.graph.has_invalid_node_digest() {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "recorded node digest does not match typed node content".to_string(),
        });
    }
    if snapshot.graph.has_invalid_declared_freshness() {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "declared stale content is represented as fresh".to_string(),
        });
    }
    if !snapshot.graph.dangling_edges().is_empty() {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "graph contains a dangling reference".to_string(),
        });
    }
    if snapshot.graph.publication_outcomes().any(|outcome| {
        !super::OutcomeValidationPhase::is_complete_trace(&outcome.validation_phases)
            || outcome.execution_audit != ExecutionAuditCounters::default()
    }) {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "outcome event violates deterministic validation audit".to_string(),
        });
    }
    validate_terminal_result(&snapshot.terminal_result)?;
    let (expected_graph, expected_result) =
        reconstruct_repository(&snapshot.admitted_bundles, &snapshot.graph)
            .map_err(|error| DecisionMemoryError::InvalidSnapshot { message: error.to_string() })?;
    if snapshot.graph != expected_graph || snapshot.terminal_result != expected_result {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "snapshot does not match deterministic bundle and event replay".to_string(),
        });
    }
    let terminal_matches = snapshot
        .terminal_result
        .decision_node_id
        .as_ref()
        .and_then(|node_id| snapshot.graph.node(node_id).ok())
        .is_some_and(|node| {
            matches!(
                node,
                DecisionMemoryNode::Decision(decision)
                    if decision.content.decision == snapshot.terminal_result.decision
                        && !decision.freshness.is_stale()
            )
        });
    if !terminal_matches {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "terminal result is not recorded in the graph".to_string(),
        });
    }
    Ok(())
}

fn validate_terminal_result(
    result: &GovernanceValidationResult,
) -> Result<(), DecisionMemoryError> {
    if result.semantic_truth_asserted || result.execution_audit != ExecutionAuditCounters::default()
    {
        return Err(DecisionMemoryError::InvalidSnapshot {
            message: "terminal result violates deterministic zero-execution invariants".to_string(),
        });
    }
    let phases_are_coherent = match result.decision {
        GovernanceDecision::Accepted => {
            result.phases
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
                ]
        }
        GovernanceDecision::Blocked => {
            result.phases
                == [
                    ValidationPhase::Parse,
                    ValidationPhase::Normalize,
                    ValidationPhase::ValidateStructure,
                    ValidationPhase::ValidateCrossReferences,
                    ValidationPhase::ValidateAuthority,
                    ValidationPhase::RecordDecision,
                    ValidationPhase::Project,
                ]
        }
        GovernanceDecision::RequiredMissing => {
            result.phases
                == [
                    ValidationPhase::Parse,
                    ValidationPhase::Normalize,
                    ValidationPhase::ValidateStructure,
                    ValidationPhase::ValidateCrossReferences,
                    ValidationPhase::RecordDecision,
                    ValidationPhase::Project,
                ]
                || result.phases
                    == [
                        ValidationPhase::Parse,
                        ValidationPhase::Normalize,
                        ValidationPhase::ValidateStructure,
                        ValidationPhase::ValidateCrossReferences,
                        ValidationPhase::ValidateAuthority,
                        ValidationPhase::ValidateEvidenceRequirements,
                        ValidationPhase::RecordDecision,
                        ValidationPhase::Project,
                    ]
        }
        GovernanceDecision::Stale => {
            result.phases
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
                ]
        }
        GovernanceDecision::Unsupported => {
            result.phases
                == [
                    ValidationPhase::Parse,
                    ValidationPhase::Normalize,
                    ValidationPhase::ValidateStructure,
                    ValidationPhase::RecordDecision,
                    ValidationPhase::Project,
                ]
        }
        GovernanceDecision::Rejected | GovernanceDecision::Conflict => false,
    };
    let findings_are_coherent = if result.decision == GovernanceDecision::Accepted {
        result.findings.is_empty()
    } else {
        !result.findings.is_empty()
            && result.findings.windows(2).all(|pair| pair[0] < pair[1])
            && result.findings.iter().all(|finding| result.phases.contains(&finding.phase))
    };
    if phases_are_coherent && findings_are_coherent {
        Ok(())
    } else {
        Err(DecisionMemoryError::InvalidSnapshot {
            message: "terminal result phases or findings are incoherent".to_string(),
        })
    }
}
