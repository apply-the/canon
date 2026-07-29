//! Exact-digest stale propagation over typed freshness-impact edges.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::digest::is_sha256;
use super::graph::require_safe_projection_text;
use super::{DecisionMemoryError, DecisionMemoryGraph, FreshnessState, NodeId};

/// Result of one deterministic stale-propagation transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StalePropagation {
    /// Number of nodes whose freshness changed.
    pub changed_nodes: usize,
    /// Stable ordered identities that became stale.
    pub stale_nodes: Vec<NodeId>,
}

/// Marks an exact changed node and its typed transitive dependents stale.
pub fn propagate_stale(
    graph: &mut DecisionMemoryGraph,
    changed_node: &NodeId,
    observed_digest: impl Into<String>,
    reason: impl Into<String>,
) -> Result<StalePropagation, DecisionMemoryError> {
    let observed_digest = observed_digest.into();
    let reason = reason.into();
    require_safe_projection_text(&reason, "stale reason")?;
    if !is_sha256(&observed_digest) {
        return Err(DecisionMemoryError::MalformedDigest { identity: changed_node.to_string() });
    }
    let current = graph.node(changed_node)?;
    if current.content_digest().sha256 == observed_digest
        || matches!(
            current.freshness(),
            FreshnessState::Stale { observed_digest: recorded, .. } if recorded == &observed_digest
        )
    {
        return Ok(StalePropagation { changed_nodes: 0, stale_nodes: Vec::new() });
    }
    if graph.has_freshness_cycle() {
        return Err(DecisionMemoryError::DependencyCycle { node_id: changed_node.clone() });
    }

    let mut pending = VecDeque::from([changed_node.clone()]);
    let mut paths = BTreeMap::from([(changed_node.clone(), vec![changed_node.clone()])]);
    let mut visited = BTreeSet::new();
    while let Some(current) = pending.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }
        let path = paths.get(&current).cloned().unwrap_or_else(|| vec![current.clone()]);
        for dependent in graph.dependents(&current) {
            paths.entry(dependent.clone()).or_insert_with(|| {
                let mut dependent_path = path.clone();
                dependent_path.push(dependent.clone());
                dependent_path
            });
            pending.push_back(dependent);
        }
    }

    let stale_nodes = visited.into_iter().collect::<Vec<_>>();
    for node_id in &stale_nodes {
        let chain = paths.get(node_id).cloned().unwrap_or_else(|| vec![node_id.clone()]);
        *graph.node_mut(node_id)?.freshness_mut() = FreshnessState::Stale {
            observed_digest: observed_digest.clone(),
            reason: reason.clone(),
            reason_chain: chain,
        };
    }
    graph.revision = graph.revision.saturating_add(1);
    graph.record_stale_event(changed_node.clone(), observed_digest, reason);
    Ok(StalePropagation { changed_nodes: stale_nodes.len(), stale_nodes })
}
