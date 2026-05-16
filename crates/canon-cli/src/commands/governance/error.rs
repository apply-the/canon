//! Engine error mapping for the governance adapter.
//!
//! Translates [`EngineError`] variants into [`GovernanceResponse`] values so
//! every upstream failure surfaces as a structured, machine-readable response
//! rather than a raw Rust error.

use super::*;

/// Converts an [`EngineError`] into a [`GovernanceResponse`].
///
/// Each engine error variant maps to a specific [`GovernanceReasonCode`] and
/// human-readable message. The optional `run_ref` is attached to the response
/// when the run was already created before the failure occurred.
pub(super) fn map_engine_error(error: EngineError, run_ref: Option<String>) -> GovernanceResponse {
    match error {
        EngineError::Validation(message) => GovernanceResponse::blocked(
            GovernanceReasonCode::DomainValidationFailed,
            message,
            Vec::new(),
        )
        .with_run_ref(run_ref),
        EngineError::UnsupportedMode(mode) => GovernanceResponse::blocked(
            GovernanceReasonCode::UnsupportedMode,
            format!("mode `{mode}` is not supported by Canon governance"),
            vec!["mode".to_string()],
        )
        .with_run_ref(run_ref),
        EngineError::Io(error) => GovernanceResponse::failed(
            GovernanceReasonCode::WorkspaceUnavailable,
            format!("workspace or runtime state is not accessible: {error}"),
            run_ref,
        ),
        EngineError::UnsupportedInspectTarget(target) => GovernanceResponse::failed(
            GovernanceReasonCode::RuntimeError,
            format!("unexpected engine target surfaced from governance execution: {target}"),
            run_ref,
        ),
    }
}

impl GovernanceResponse {
    /// Attaches a `run_ref` to a response after the run has been created.
    ///
    /// Used by error paths that fire after the engine has already allocated a
    /// run identifier, so callers can still locate the partial run record.
    fn with_run_ref(mut self, run_ref: Option<String>) -> Self {
        self.run_ref = run_ref;
        self
    }
}
