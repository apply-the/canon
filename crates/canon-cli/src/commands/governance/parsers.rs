//! Request field parsers for the governance adapter.
//!
//! Each `parse_*` function converts a raw string field from the incoming JSON
//! request into a typed engine value, returning a blocked [`GovernanceResponse`]
//! on failure rather than propagating a generic parse error.

use super::paths::{canonical_repo_root, non_empty, path_to_slash_string, resolve_request_path};
use super::*;

/// Parses a `mode` string into a [`Mode`].
///
/// Returns a [`GovernanceReasonCode::UnsupportedMode`] blocked response when
/// the value does not correspond to any known Canon mode.
pub(super) fn parse_mode(value: &str) -> Result<Mode, GovernanceFailure> {
    value.parse::<Mode>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::UnsupportedMode,
            format!("mode `{value}` is not supported by Canon governance"),
            vec!["mode".to_string()],
        ))
    })
}

/// Parses a `system_context` string into a [`SystemContext`].
///
/// Returns a [`GovernanceReasonCode::UnsupportedSystemContext`] blocked
/// response when the value is not `new` or `existing`.
pub(super) fn parse_system_context(value: &str) -> Result<SystemContext, GovernanceFailure> {
    value.parse::<SystemContext>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::UnsupportedSystemContext,
            format!("system_context `{value}` is not supported by Canon governance"),
            vec!["system_context".to_string()],
        ))
    })
}

/// Parses a `risk` string into a [`RiskClass`].
///
/// Returns a [`GovernanceReasonCode::UnsupportedRisk`] blocked response when
/// the value does not match any recognized risk classification.
pub(super) fn parse_risk(value: &str) -> Result<RiskClass, GovernanceFailure> {
    value.parse::<RiskClass>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::UnsupportedRisk,
            format!("risk `{value}` is not supported by Canon governance"),
            vec!["risk".to_string()],
        ))
    })
}

/// Parses a `zone` string into a [`UsageZone`].
///
/// Returns a [`GovernanceReasonCode::UnsupportedZone`] blocked response when
/// the value does not match any recognized usage zone.
pub(super) fn parse_zone(value: &str) -> Result<UsageZone, GovernanceFailure> {
    value.parse::<UsageZone>().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::UnsupportedZone,
            format!("zone `{value}` is not supported by Canon governance"),
            vec!["zone".to_string()],
        ))
    })
}

/// Collects all unique, workspace-relative input document references from a
/// [`GovernanceRequest`].
///
/// Processes `bounded_context.stage_brief_ref` first, then each entry in
/// `input_documents`, normalizing every path to a forward-slash string
/// relative to the repository root. Duplicate references are silently
/// de-duplicated. Returns a blocked response for any inaccessible or
/// out-of-workspace path.
pub(super) fn collect_input_references(
    repo_root: &Path,
    request: &GovernanceRequest,
) -> Result<Vec<String>, GovernanceFailure> {
    let mut inputs = Vec::new();

    if let Some(stage_brief_ref) = non_empty(request.bounded_context.stage_brief_ref.as_deref()) {
        push_unique_input(repo_root, stage_brief_ref, &mut inputs)?;
    }

    for document in &request.input_documents {
        if let Some(reference) = document.reference() {
            push_unique_input(repo_root, reference, &mut inputs)?;
        }
    }

    Ok(inputs)
}

/// Adds `reference` to `inputs` if it is not already present.
///
/// Normalizes the path via [`normalize_workspace_relative_ref`] before the
/// duplicate check, so paths that differ only in representation (e.g.
/// absolute vs. relative) are correctly de-duplicated.
pub(super) fn push_unique_input(
    repo_root: &Path,
    reference: &str,
    inputs: &mut Vec<String>,
) -> Result<(), GovernanceFailure> {
    let normalized = normalize_workspace_relative_ref(repo_root, reference)?;
    if !inputs.contains(&normalized) {
        inputs.push(normalized);
    }
    Ok(())
}

/// Canonicalizes `reference` and returns a workspace-relative forward-slash
/// string.
///
/// Verifies that the resolved path is contained within `repo_root` before
/// stripping the prefix, preventing path traversal outside the declared
/// workspace boundary.
pub(super) fn normalize_workspace_relative_ref(
    repo_root: &Path,
    reference: &str,
) -> Result<String, GovernanceFailure> {
    let canonical_repo = canonical_repo_root(repo_root).map_err(|error| {
        Box::new(GovernanceResponse::failed(
            GovernanceReasonCode::WorkspaceUnavailable,
            format!("workspace is not accessible: {error}"),
            None,
        ))
    })?;
    let candidate = resolve_request_path(repo_root, reference);
    let canonical_candidate = candidate.canonicalize().map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::InputDocumentMissing,
            format!("document `{reference}` is not accessible inside the workspace"),
            vec!["input_documents".to_string()],
        ))
    })?;

    if !canonical_candidate.starts_with(&canonical_repo) {
        return Err(Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::PathOutsideWorkspace,
            format!("document `{reference}` escapes the declared workspace boundary"),
            vec!["input_documents".to_string()],
        )));
    }

    let relative = canonical_candidate.strip_prefix(&canonical_repo).map_err(|_| {
        Box::new(GovernanceResponse::blocked(
            GovernanceReasonCode::PathOutsideWorkspace,
            format!("document `{reference}` escapes the declared workspace boundary"),
            vec!["input_documents".to_string()],
        ))
    })?;

    Ok(path_to_slash_string(relative))
}
