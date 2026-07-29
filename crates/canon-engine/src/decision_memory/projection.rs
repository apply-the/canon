//! Deterministic, non-secret public projection of decision memory.

use canon_contracts::{DecisionMemoryNode as PublicNode, DecisionMemoryProjection, Revision};

use super::graph::require_safe_projection_text;
use super::{DecisionMemoryError, DecisionMemoryGraph, FreshnessState};

/// Projects the internal graph into the immutable public decision-memory DTO.
pub fn project_decision_memory(
    graph: &DecisionMemoryGraph,
) -> Result<DecisionMemoryProjection, DecisionMemoryError> {
    let nodes = graph
        .nodes()
        .map(|node| -> Result<PublicNode, DecisionMemoryError> {
            require_safe_projection_text(node.id().as_str(), "node identity")?;
            let stale_reason = match node.freshness() {
                FreshnessState::Fresh => None,
                FreshnessState::Stale { reason, reason_chain, .. } => {
                    require_safe_projection_text(reason, "stale reason")?;
                    for node_id in reason_chain {
                        require_safe_projection_text(node_id.as_str(), "stale reason identity")?;
                    }
                    Some(format!(
                        "{} [{}]",
                        reason,
                        reason_chain
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" -> ")
                    ))
                }
            };
            let relationships = graph
                .edges()
                .filter(|edge| edge.source == *node.id())
                .map(|edge| {
                    require_safe_projection_text(edge.target.as_str(), "relationship identity")?;
                    Ok(format!("{}:{}", edge.kind.as_str(), edge.target))
                })
                .collect::<Result<Vec<_>, DecisionMemoryError>>()?;
            for superseded in node.supersedes() {
                require_safe_projection_text(superseded.as_str(), "superseded identity")?;
            }
            Ok(PublicNode {
                node_id: node.id().to_string(),
                kind: node.kind().to_string(),
                source_reference: node.provenance().to_string(),
                revision_introduced: Revision::new(node.revision_introduced()),
                relationships,
                supersedes: node.supersedes().iter().map(ToString::to_string).collect(),
                fresh: !node.freshness().is_stale(),
                stale_reason,
            })
        })
        .collect::<Result<Vec<_>, DecisionMemoryError>>()?;
    Ok(DecisionMemoryProjection { revision: Revision::new(graph.revision), nodes })
}
