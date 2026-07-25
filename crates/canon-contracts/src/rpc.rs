//! One-shot machine-interface DTOs freeze operations without implementing a server.

use serde::{Deserialize, Serialize};

/// Frozen Canon contract schema version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonContractVersion {
    /// Canon public contract version 1.0.
    #[serde(rename = "1.0")]
    V1,
}

/// Stable operations accepted by the one-shot Canon machine interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OneShotOperation {
    /// Describe supported contract capabilities.
    Capabilities,
    /// Start a governed run from typed inputs.
    Start,
    /// Refresh deterministic projections.
    Refresh,
    /// Record a named authority decision.
    Approve,
    /// Inspect authoritative governance projections.
    Inspect,
    /// Publish governance, decision-memory, and evidence projections.
    Publish,
}

/// Typed one-shot request envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneShotRequest<T> {
    /// Schema version used to decode the request.
    pub contract_version: CanonContractVersion,
    /// Caller-provided request identity.
    pub request_id: String,
    /// Stable operation to perform.
    pub operation: OneShotOperation,
    /// Operation-specific typed payload.
    pub payload: T,
}

/// Typed one-shot response envelope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OneShotResponse<T> {
    /// Schema version used to encode the response.
    pub contract_version: CanonContractVersion,
    /// Request identity copied from the input.
    pub request_id: String,
    /// Operation-specific typed projection.
    pub result: T,
}
