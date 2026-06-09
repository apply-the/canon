//! Governance adapter protocol layer.
//!
//! This module is the entry point for all machine-facing governance requests.
//! It dispatches [`GovernanceCommand`] variants, enforces the JSON-only
//! contract, validates the adapter schema version, and delegates to the
//! appropriate handler in [`super::handlers`].

use super::handlers::{handle_refresh, handle_start};
use super::*;

/// Dispatches a [`GovernanceCommand`] and returns a JSON value.
///
/// Handles the three command variants:
/// - `Capabilities` — returns the static adapter capabilities manifest.
/// - `Start` — validates the request and delegates to [`handle_start`].
/// - `Refresh` — validates the request and delegates to [`handle_refresh`].
///
/// All variants enforce the `--json` flag before doing any work, and `Start`
/// and `Refresh` additionally validate the adapter schema version.
pub(super) fn command_response(
    service: &EngineService,
    command: GovernanceCommand,
    request: Option<GovernanceRequest>,
) -> CliResult<serde_json::Value> {
    match command {
        GovernanceCommand::Capabilities { json } => {
            require_json(json)?;
            Ok(serde_json::to_value(capabilities_response())?)
        }
        GovernanceCommand::Start { json } => {
            require_json(json)?;
            let request = request.ok_or_else(|| {
                CliError::InvalidInput("governance start requires a JSON request body".to_string())
            })?;
            enforce_supported_schema_version(request.adapter_schema_version.as_deref())?;
            Ok(serde_json::to_value(handle_start(service, request))?)
        }
        GovernanceCommand::Refresh { json } => {
            require_json(json)?;
            let request = request.ok_or_else(|| {
                CliError::InvalidInput(
                    "governance refresh requires a JSON request body".to_string(),
                )
            })?;
            enforce_supported_schema_version(request.adapter_schema_version.as_deref())?;
            Ok(serde_json::to_value(handle_refresh(service, request))?)
        }
    }
}

/// Builds the static [`GovernanceCapabilitiesResponse`] from compile-time constants.
///
/// The capabilities manifest is returned verbatim for every `Capabilities`
/// request; it declares supported modes, schema versions, status values, and
/// compatibility notes for adapter clients.
pub(super) fn capabilities_response() -> GovernanceCapabilitiesResponse {
    GovernanceCapabilitiesResponse {
        canon_version: env!("CARGO_PKG_VERSION"),
        supported_schema_versions: [ADAPTER_SCHEMA_VERSION],
        operations: OPERATIONS,
        supported_modes: Mode::all().iter().map(|mode| mode.as_str()).collect(),
        status_values: STATUS_VALUES,
        approval_state_values: APPROVAL_STATE_VALUES,
        packet_readiness_values: PACKET_READINESS_VALUES,
        compatibility_notes: [
            "The governance adapter is the machine-facing boundary around the same Canon runtime used by the human CLI.",
            "Canon is not the higher-level orchestrator; requests that omit adapter_schema_version are interpreted as v1 and unknown additive fields are ignored within supported schema versions.",
        ],
    }
}

/// Returns `Ok(())` when `json` is `true`, or a [`CliError::InvalidInput`]
/// when the caller omitted `--json`.
///
/// The governance adapter is machine-facing only; human-readable text output
/// is not supported and is rejected early via this guard.
pub(super) fn require_json(json: bool) -> CliResult<()> {
    if json {
        Ok(())
    } else {
        Err(CliError::InvalidInput(
            "governance commands require --json for the machine-facing contract".to_string(),
        ))
    }
}

/// Reads a [`GovernanceRequest`] from [`std::io::stdin`].
///
/// Thin wrapper around [`read_request_from`] for the normal CLI code path.
pub(super) fn read_request() -> CliResult<GovernanceRequest> {
    read_request_from(std::io::stdin())
}

/// Reads and deserializes a [`GovernanceRequest`] from an arbitrary reader.
///
/// Exposed separately from [`read_request`] so unit tests can supply an
/// in-memory cursor instead of stdin.
pub(super) fn read_request_from(mut reader: impl Read) -> CliResult<GovernanceRequest> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(serde_json::from_str(&buffer)?)
}

/// Validates the `adapter_schema_version` field in an incoming request.
///
/// `None` is treated as the implicit current version. Any version string other
/// than [`ADAPTER_SCHEMA_VERSION`] is rejected with a descriptive error so
/// adapter clients can detect version drift early.
pub(super) fn enforce_supported_schema_version(version: Option<&str>) -> CliResult<()> {
    match version {
        None | Some(ADAPTER_SCHEMA_VERSION) => Ok(()),
        Some(other) => {
            Err(CliError::InvalidInput(format!("unsupported adapter schema version: {other}")))
        }
    }
}
