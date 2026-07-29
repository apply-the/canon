//! Typed failures for deterministic governance and decision memory.

use std::path::PathBuf;

use thiserror::Error;

use super::NodeId;

/// Fail-closed errors raised before a governance result can be recorded.
#[derive(Debug, Error)]
pub enum DecisionMemoryError {
    /// A required field or invariant was absent.
    #[error("invalid governance structure: {message}")]
    InvalidStructure {
        /// Stable diagnostic without volatile local context.
        message: String,
    },
    /// One stable identity appeared more than once.
    #[error("duplicate {kind} identity: {identity}")]
    DuplicateIdentity {
        /// Kind of identity that was duplicated.
        kind: &'static str,
        /// Duplicated stable value.
        identity: String,
    },
    /// A digest did not use the frozen SHA-256 representation.
    #[error("malformed SHA-256 digest for {identity}")]
    MalformedDigest {
        /// Identity whose digest was malformed.
        identity: String,
    },
    /// A graph relationship referenced a missing node.
    #[error("dangling reference from {reference_source} to {target}")]
    DanglingReference {
        /// Source node.
        reference_source: NodeId,
        /// Missing target node.
        target: NodeId,
    },
    /// The same node identity was replayed with different content.
    #[error("decision-memory content conflict for {node_id}")]
    ContentConflict {
        /// Conflicting stable node identity.
        node_id: NodeId,
    },
    /// A freshness or authority relationship introduced a cycle.
    #[error("decision-memory dependency cycle through {node_id}")]
    DependencyCycle {
        /// Node at which cycle admission was rejected.
        node_id: NodeId,
    },
    /// A requested node did not exist.
    #[error("decision-memory node not found: {node_id}")]
    NodeNotFound {
        /// Missing stable node identity.
        node_id: NodeId,
    },
    /// Typed deterministic serialization failed.
    #[error("decision-memory serialization failed: {message}")]
    Serialization {
        /// Serialization diagnostic.
        message: String,
    },
    /// The persisted snapshot failed validation or decoding.
    #[error("invalid persisted decision-memory snapshot: {message}")]
    InvalidSnapshot {
        /// Stable snapshot diagnostic.
        message: String,
    },
    /// Repository-local persistence failed.
    #[error("decision-memory persistence failed at {path}: {source}")]
    Persistence {
        /// Affected repository-local path.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
}

impl DecisionMemoryError {
    pub(crate) fn invalid_structure(message: impl Into<String>) -> Self {
        Self::InvalidStructure { message: message.into() }
    }

    pub(crate) fn serialization(error: impl std::fmt::Display) -> Self {
        Self::Serialization { message: error.to_string() }
    }
}
