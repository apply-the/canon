//! Governance command handlers.
//!
//! Contains the high-level handlers for `governance start` and
//! `governance refresh` commands, plus the shared request validation logic
//! used by both.

use super::error::map_engine_error;
use super::parsers::{
    collect_input_references, parse_mode, parse_risk, parse_system_context, parse_zone,
};
use super::paths::{canonical_repo_root, non_empty, resolve_request_path};
use super::projection::project_run_response;
use super::status::explicit_classification;
use super::*;

/// Handles a `governance start` request.
///
/// Validates the request, builds a [`RunRequest`], delegates to the engine,
/// and projects the resulting run state into a [`GovernanceResponse`].
/// Returns a blocked or failed response without panicking on any error path.
pub(super) fn handle_start(
    service: &EngineService,
    repo_root: &Path,
    request: GovernanceRequest,
) -> GovernanceResponse {
    let validated = match validate_request(repo_root, &request, GovernanceOperation::Start) {
        Ok(validated) => validated,
        Err(response) => return *response,
    };

    let run_request = RunRequest {
        mode: validated.mode,
        risk: validated.risk,
        zone: validated.zone,
        system_context: Some(validated.system_context),
        classification: explicit_classification(),
        owner: validated.owner,
        inputs: validated.inputs,
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    };

    match service.run(run_request) {
        Ok(summary) => project_run_response(
            repo_root,
            &summary.run_id,
            summary.mode_result.map(|result| result.headline),
        ),
        Err(error) => map_engine_error(error, None),
    }
}

/// Handles a `governance refresh` request.
///
/// Re-projects the current run state for an existing `run_ref` without
/// re-executing the run. Used by adapters to poll for updated approval or
/// readiness status after a human has acted on a run outside the adapter.
pub(super) fn handle_refresh(repo_root: &Path, request: GovernanceRequest) -> GovernanceResponse {
    if let Err(response) = validate_request(repo_root, &request, GovernanceOperation::Refresh) {
        return *response;
    }

    let Some(run_ref) = non_empty(request.run_ref.as_deref()) else {
        return GovernanceResponse::blocked(
            GovernanceReasonCode::MissingRequiredField,
            "request is missing required fields for domain execution",
            vec!["run_ref".to_string()],
        );
    };

    project_run_response(repo_root, run_ref, None)
}

/// Validates a [`GovernanceRequest`] for the given [`GovernanceOperation`].
///
/// Checks for missing required fields, verifies `request_kind` matches the
/// operation, confirms the workspace binding, and parses all typed fields
/// (mode, system context, risk, zone, inputs). Returns a [`ValidatedStartRequest`]
/// on success or a boxed [`GovernanceResponse`] error on any validation failure.
pub(super) fn validate_request(
    repo_root: &Path,
    request: &GovernanceRequest,
    operation: GovernanceOperation,
) -> Result<ValidatedStartRequest, GovernanceFailure> {
    let missing = missing_domain_fields(request, &operation);
    if !missing.is_empty() {
        return Err(Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::MissingRequiredField,
            "request is missing required fields for domain execution",
            missing,
        )));
    }

    let expected_kind = match operation {
        GovernanceOperation::Start => "start",
        GovernanceOperation::Refresh => "refresh",
    };
    let request_kind = non_empty(request.request_kind.as_deref()).unwrap_or_default();
    if request_kind != expected_kind {
        return Err(Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::RequestKindMismatch,
            format!(
                "request_kind `{request_kind}` does not match the invoked `{expected_kind}` operation"
            ),
            vec!["request_kind".to_string()],
        )));
    }

    validate_workspace_binding(repo_root, request.workspace_ref.as_deref().unwrap_or_default())?;

    let mode = parse_mode(request.mode.as_deref().unwrap_or_default())?;
    let system_context =
        parse_system_context(request.system_context.as_deref().unwrap_or_default())?;
    let risk = parse_risk(request.risk.as_deref().unwrap_or_default())?;
    let zone = parse_zone(request.zone.as_deref().unwrap_or_default())?;
    let owner = request.owner.as_deref().unwrap_or_default().trim().to_string();
    let inputs = collect_input_references(repo_root, request)?;

    Ok(ValidatedStartRequest { mode, system_context, risk, zone, owner, inputs })
}

/// Returns the names of all required fields that are absent or blank.
///
/// For `Refresh` operations, `run_ref` is additionally required. The returned
/// list is attached to [`GovernanceResponse::missing_fields`] so adapter
/// clients can surface precise field-level errors to their callers.
pub(super) fn missing_domain_fields(
    request: &GovernanceRequest,
    operation: &GovernanceOperation,
) -> Vec<String> {
    let mut missing = Vec::new();
    let required = [
        ("request_kind", request.request_kind.as_deref()),
        ("governance_attempt_id", request.governance_attempt_id.as_deref()),
        ("stage_key", request.stage_key.as_deref()),
        ("goal", request.goal.as_deref()),
        ("workspace_ref", request.workspace_ref.as_deref()),
        ("mode", request.mode.as_deref()),
        ("system_context", request.system_context.as_deref()),
        ("risk", request.risk.as_deref()),
        ("zone", request.zone.as_deref()),
        ("owner", request.owner.as_deref()),
    ];

    for (field, value) in required {
        if non_empty(value).is_none() {
            missing.push(field.to_string());
        }
    }

    if matches!(operation, GovernanceOperation::Refresh)
        && non_empty(request.run_ref.as_deref()).is_none()
    {
        missing.push("run_ref".to_string());
    }

    missing
}

/// Verifies that `workspace_ref` resolves to the same directory as `repo_root`.
///
/// Canonicalizes both paths before comparing so that symlinks and relative
/// components do not produce false mismatches. Returns a blocked response
/// when the resolved paths differ, preventing cross-workspace contamination.
pub(super) fn validate_workspace_binding(
    repo_root: &Path,
    workspace_ref: &str,
) -> Result<(), GovernanceFailure> {
    let canonical_repo = canonical_repo_root(repo_root).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::WorkspaceUnavailable,
            format!("workspace is not accessible: {error}"),
            None,
        ))
    })?;
    let requested = resolve_request_path(repo_root, workspace_ref);
    let canonical_requested = requested.canonicalize().map_err(|_| {
        Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::WorkspaceUnavailable,
            format!("workspace `{workspace_ref}` is not accessible"),
            None,
        ))
    })?;

    if canonical_requested != canonical_repo {
        return Err(Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::WorkspaceMismatch,
            "workspace_ref must bind to the current Canon workspace",
            vec!["workspace_ref".to_string()],
        )));
    }

    Ok(())
}
