//! Ordered replay verification for complete repository decision memory.

use std::collections::BTreeMap;

use super::bundle::{recompute_bundle_digest, typed_string};
use super::graph::DecisionMemoryEvent;
use super::{
    BoundGovernanceBundle, DecisionMemoryError, DecisionMemoryGraph, GovernanceValidationResult,
    NodeId, propagate_stale, validate_governance_bundle,
};

type BundleIdentity = (String, u64);

/// Reconstructs graph and terminal result from admitted roots and one ordered operation journal.
pub(crate) fn reconstruct_repository(
    bundles: &[BoundGovernanceBundle],
    recorded_graph: &DecisionMemoryGraph,
) -> Result<(DecisionMemoryGraph, GovernanceValidationResult), DecisionMemoryError> {
    let roots = validated_roots(bundles)?;
    let mut expected = DecisionMemoryGraph::new();
    let mut admitted = Vec::new();
    let mut last_result = None;
    for event in recorded_graph.events().to_vec() {
        match event {
            DecisionMemoryEvent::BundleAdmission { bundle_id, decision_memory_revision } => {
                let identity = (bundle_id, decision_memory_revision);
                let bundle = roots.get(&identity).ok_or_else(|| {
                    DecisionMemoryError::invalid_structure(
                        "admission event has no exact governance-bundle root",
                    )
                })?;
                if admitted.contains(&identity) {
                    return Err(DecisionMemoryError::invalid_structure(
                        "governance bundle was admitted more than once",
                    ));
                }
                expected.admit_bundle(bundle)?;
                admitted.push(identity);
                last_result = None;
            }
            DecisionMemoryEvent::Validation { bundle_id, decision_memory_revision } => {
                let identity = (bundle_id, decision_memory_revision);
                if !admitted.contains(&identity) {
                    return Err(DecisionMemoryError::invalid_structure(
                        "validation event precedes exact bundle admission",
                    ));
                }
                let bundle = roots.get(&identity).ok_or_else(|| {
                    DecisionMemoryError::invalid_structure(
                        "validation event has no exact governance-bundle root",
                    )
                })?;
                last_result = Some(validate_governance_bundle(bundle, &mut expected));
            }
            DecisionMemoryEvent::StalePropagation { changed_node, observed_digest, reason } => {
                let propagation =
                    propagate_stale(&mut expected, &changed_node, observed_digest, reason)?;
                if propagation.changed_nodes == 0 {
                    return Err(DecisionMemoryError::invalid_structure(
                        "persisted stale event does not change reconstructed state",
                    ));
                }
                last_result = None;
            }
            DecisionMemoryEvent::Supersession { previous, successor, reason } => {
                validate_supersession_direction(&expected, &previous, &successor)?;
                expected.supersede(&previous, &successor, reason)?;
                last_result = None;
            }
            DecisionMemoryEvent::OutcomeRecorded { outcome } => {
                let replayed = expected.record_publication_outcome(
                    outcome.request,
                    outcome.validation_phases,
                    outcome.execution_audit,
                );
                if replayed.decision_memory_revision != outcome.decision_memory_revision {
                    return Err(DecisionMemoryError::invalid_structure(
                        "outcome event revision diverges during journal replay",
                    ));
                }
            }
        }
    }
    let expected_order = bundles.iter().map(bundle_identity).collect::<Result<Vec<_>, _>>()?;
    if admitted != expected_order {
        return Err(DecisionMemoryError::invalid_structure(
            "bundle roots do not match ordered admission events",
        ));
    }
    let mut terminal = last_result.ok_or_else(|| {
        DecisionMemoryError::invalid_structure(
            "operation journal does not end in deterministic validation",
        )
    })?;
    terminal.graph_digest = expected.digest()?.sha256;
    Ok((expected, terminal))
}

fn validated_roots(
    bundles: &[BoundGovernanceBundle],
) -> Result<BTreeMap<BundleIdentity, &BoundGovernanceBundle>, DecisionMemoryError> {
    if bundles.is_empty() {
        return Err(DecisionMemoryError::invalid_structure(
            "persisted decision memory has no admitted governance bundle",
        ));
    }
    let mut roots = BTreeMap::new();
    for bundle in bundles {
        validate_bundle_root(bundle)?;
        let identity = bundle_identity(bundle)?;
        if roots.insert(identity, bundle).is_some() {
            return Err(DecisionMemoryError::invalid_structure(
                "admitted governance bundle identity and revision are duplicated",
            ));
        }
    }
    Ok(roots)
}

fn validate_bundle_root(bundle: &BoundGovernanceBundle) -> Result<(), DecisionMemoryError> {
    let recorded = typed_string(&bundle.contract.bundle_digest)?;
    let recomputed = recompute_bundle_digest(bundle)?;
    if recorded == recomputed.sha256 {
        Ok(())
    } else {
        Err(DecisionMemoryError::invalid_structure(
            "admitted governance bundle digest does not match its content",
        ))
    }
}

fn bundle_identity(bundle: &BoundGovernanceBundle) -> Result<BundleIdentity, DecisionMemoryError> {
    Ok((typed_string(&bundle.contract.bundle_id)?, bundle.metadata.decision_memory_revision))
}

fn validate_supersession_direction(
    graph: &DecisionMemoryGraph,
    previous: &NodeId,
    successor: &NodeId,
) -> Result<(), DecisionMemoryError> {
    let (previous_bundle, previous_revision) = decision_identity(graph, previous)?;
    let (successor_bundle, successor_revision) = decision_identity(graph, successor)?;
    if previous_bundle == successor_bundle && previous_revision < successor_revision {
        Ok(())
    } else {
        Err(DecisionMemoryError::invalid_structure(
            "decision supersession must advance one bundle to a later admitted revision",
        ))
    }
}

fn decision_identity(
    graph: &DecisionMemoryGraph,
    node_id: &NodeId,
) -> Result<(String, u64), DecisionMemoryError> {
    let decision = match graph.node(node_id)? {
        super::DecisionMemoryNode::Decision(value) => value,
        _ => {
            return Err(DecisionMemoryError::invalid_structure(
                "supersession endpoint is not a decision",
            ));
        }
    };
    Ok((
        decision.content.bundle_id.clone(),
        parse_decision_revision(node_id, decision.content.decision.as_str())?,
    ))
}

fn parse_decision_revision(
    node_id: &NodeId,
    decision_status: &str,
) -> Result<u64, DecisionMemoryError> {
    let status_suffix = format!("-{decision_status}");
    node_id
        .as_str()
        .strip_suffix(&status_suffix)
        .and_then(|without_status| without_status.rsplit_once("-r"))
        .and_then(|(_, revision)| revision.parse::<u64>().ok())
        .ok_or_else(|| DecisionMemoryError::invalid_structure("decision revision is malformed"))
}
