//! Run-state and packet-readiness helpers for the governance adapter.
//!
//! Translates raw engine [`RunState`] and [`PacketReadiness`] values into
//! the adapter-facing [`GovernanceStatus`], [`ApprovalState`], and
//! [`GovernanceReasonCode`] types, and builds the human-readable headline and
//! message strings included in every [`GovernanceResponse`].

use super::paths::artifact_contains_missing_authored_body;
use super::paths::packet_leaf;
use super::*;

/// Maps an adapter [`ApprovalState`] to the authority-layer
/// [`AuthorityApprovalState`] used inside the `authority_governance` block.
pub(super) fn authority_approval_state(approval_state: ApprovalState) -> AuthorityApprovalState {
    match approval_state {
        ApprovalState::NotNeeded => AuthorityApprovalState::NotNeeded,
        ApprovalState::Requested => AuthorityApprovalState::Requested,
        ApprovalState::Granted => AuthorityApprovalState::Granted,
        ApprovalState::Rejected => AuthorityApprovalState::Rejected,
        ApprovalState::Expired => AuthorityApprovalState::Expired,
    }
}

/// Maps an optional [`PacketReadiness`] (defaulting to `Pending`) to the
/// authority-layer [`AuthorityPacketReadiness`] used inside the
/// `authority_governance` block.
pub(super) fn authority_packet_readiness(
    packet_readiness: Option<PacketReadiness>,
) -> AuthorityPacketReadiness {
    match packet_readiness.unwrap_or(PacketReadiness::Pending) {
        PacketReadiness::Pending => AuthorityPacketReadiness::Pending,
        PacketReadiness::Incomplete => AuthorityPacketReadiness::Incomplete,
        PacketReadiness::Reusable => AuthorityPacketReadiness::Reusable,
        PacketReadiness::Rejected => AuthorityPacketReadiness::Rejected,
    }
}

/// Returns the artifact paths for a run that fall within the mode's path
/// segment.
///
/// When `expected_document_refs` is empty, all available artifact paths under
/// the mode segment are returned. When it is non-empty, only the paths that
/// are already present in the artifact store are returned.
pub(super) fn load_document_refs(
    store: &WorkspaceStore,
    run_ref: &str,
    mode: Mode,
    expected_document_refs: &[String],
) -> Result<Vec<String>, std::io::Error> {
    let mode_segment = format!("/{}/", mode.as_str());
    let available = store
        .list_artifact_files(run_ref)?
        .into_iter()
        .filter(|path| path.contains(&mode_segment))
        .collect::<HashSet<_>>();

    if expected_document_refs.is_empty() {
        let mut refs = available.into_iter().collect::<Vec<_>>();
        refs.sort();
        return Ok(refs);
    }

    Ok(expected_document_refs
        .iter()
        .filter(|path| available.contains(path.as_str()))
        .cloned()
        .collect())
}

/// Returns the expected document refs that are absent from the run's artifact
/// store.
///
/// An empty result means all expected artifacts are present. Non-empty results
/// are surfaced as `missing_sections` in the [`GovernanceResponse`].
pub(super) fn missing_document_refs(
    expected_document_refs: &[String],
    document_refs: &[String],
) -> Vec<String> {
    let present = document_refs.iter().map(String::as_str).collect::<HashSet<_>>();
    expected_document_refs
        .iter()
        .filter(|reference| !present.contains(reference.as_str()))
        .cloned()
        .collect()
}

/// Returns the document refs whose artifact files still contain the
/// missing-authored-body placeholder marker.
///
/// These are treated as rejected because the file exists but the operator
/// has not yet replaced the template placeholder with real content.
pub(super) fn rejected_document_refs(repo_root: &Path, document_refs: &[String]) -> Vec<String> {
    document_refs
        .iter()
        .filter(|reference| artifact_contains_missing_authored_body(repo_root, reference))
        .cloned()
        .collect()
}

/// Derives the [`PacketReadiness`] for a run from its document ref sets.
///
/// Precedence (highest to lowest):
/// 1. `Rejected` when any artifact still contains the placeholder marker.
/// 2. `Incomplete` when any expected artifacts are missing.
/// 3. `Reusable` when all expected artifacts are present.
/// 4. `Pending` when no artifacts exist yet or neither list has entries.
pub(super) fn packet_readiness_value(
    expected_document_refs: &[String],
    document_refs: &[String],
    missing_refs: &[String],
    rejected_refs: &[String],
) -> Option<PacketReadiness> {
    if expected_document_refs.is_empty() && document_refs.is_empty() {
        return Some(PacketReadiness::Pending);
    }
    if !rejected_refs.is_empty() {
        return Some(PacketReadiness::Rejected);
    }
    if !missing_refs.is_empty() {
        return Some(PacketReadiness::Incomplete);
    }
    if !document_refs.is_empty() {
        return Some(PacketReadiness::Reusable);
    }

    Some(PacketReadiness::Pending)
}

/// Builds the `missing_sections` list for a [`GovernanceResponse`].
///
/// Returns the leaf file names (not full paths) of all missing and rejected
/// artifact refs, de-duplicated, so adapters can display concise section names
/// to operators.
pub(super) fn packet_missing_sections(
    missing_refs: &[String],
    rejected_refs: &[String],
) -> Vec<String> {
    let mut sections =
        missing_refs.iter().map(|reference| packet_leaf(reference)).collect::<Vec<_>>();

    for rejected in rejected_refs {
        let leaf = packet_leaf(rejected);
        if !sections.contains(&leaf) {
            sections.push(leaf);
        }
    }

    sections
}

/// Derives the [`ApprovalState`] from the run state and approval flags.
///
/// `AwaitingApproval` run state takes precedence over the approval flags;
/// otherwise `Rejected` takes precedence over `Granted`, and `Granted`
/// over `NotNeeded`.
pub(super) fn approval_state_value(
    run_state: RunState,
    any_approved: bool,
    any_rejected: bool,
) -> ApprovalState {
    if matches!(run_state, RunState::AwaitingApproval) {
        return ApprovalState::Requested;
    }
    if any_rejected {
        return ApprovalState::Rejected;
    }
    if any_approved {
        return ApprovalState::Granted;
    }
    ApprovalState::NotNeeded
}

/// Maps a [`RunState`] and optional [`PacketReadiness`] to the adapter-facing
/// [`GovernanceStatus`].
///
/// The mapping coalesces the many internal engine states into the smaller set
/// of status values the adapter contract exposes:
/// `PendingSelection`, `Running`, `AwaitingApproval`, `GovernedReady`,
/// `Blocked`, `Completed`, and `Failed`.
pub(super) fn normalized_status(
    run_state: RunState,
    packet_readiness: Option<PacketReadiness>,
) -> GovernanceStatus {
    match run_state {
        RunState::Draft
        | RunState::ContextCaptured
        | RunState::Classified
        | RunState::Contracted
        | RunState::Gated => GovernanceStatus::PendingSelection,
        RunState::Executing | RunState::Verifying => GovernanceStatus::Running,
        RunState::AwaitingApproval => GovernanceStatus::AwaitingApproval,
        RunState::Blocked => GovernanceStatus::Blocked,
        RunState::Completed => match packet_readiness {
            Some(PacketReadiness::Reusable) => GovernanceStatus::GovernedReady,
            Some(PacketReadiness::Incomplete) | Some(PacketReadiness::Rejected) => {
                GovernanceStatus::Blocked
            }
            _ => GovernanceStatus::Completed,
        },
        RunState::Failed | RunState::Aborted | RunState::Superseded => GovernanceStatus::Failed,
        RunState::AwaitingReviewerOutput => GovernanceStatus::PendingSelection,
    }
}

/// Returns a [`GovernanceReasonCode`] for statuses that require one, or
/// `None` for success/in-progress statuses.
///
/// `Blocked` reason codes are further differentiated by packet readiness
/// so adapter clients can distinguish incomplete from rejected packets.
pub(super) fn response_reason_code(
    status: GovernanceStatus,
    packet_readiness: Option<PacketReadiness>,
) -> Option<GovernanceReasonCode> {
    match status {
        GovernanceStatus::AwaitingApproval => Some(GovernanceReasonCode::ApprovalRequired),
        GovernanceStatus::Blocked => match packet_readiness {
            Some(PacketReadiness::Incomplete) => Some(GovernanceReasonCode::IncompletePacket),
            Some(PacketReadiness::Rejected) => Some(GovernanceReasonCode::RejectedPacket),
            _ => Some(GovernanceReasonCode::BlockedByGovernance),
        },
        GovernanceStatus::Failed => Some(GovernanceReasonCode::RunFailed),
        _ => None,
    }
}

/// Produces a short headline string for the given status and packet
/// readiness, or `None` for states that have no default headline.
///
/// Callers may override this with a `headline_hint` from the engine result;
/// this function provides the fallback when no hint is available.
pub(super) fn default_headline(
    status: GovernanceStatus,
    packet_readiness: Option<PacketReadiness>,
) -> Option<String> {
    let headline = match status {
        GovernanceStatus::GovernedReady => "Governed packet is reusable",
        GovernanceStatus::AwaitingApproval => "Governed packet is waiting on approval",
        GovernanceStatus::Blocked => match packet_readiness {
            Some(PacketReadiness::Incomplete) => "Governed packet is incomplete",
            Some(PacketReadiness::Rejected) => "Governed packet was rejected for downstream reuse",
            _ => "Governance execution is blocked",
        },
        GovernanceStatus::Completed => "Governance execution completed",
        GovernanceStatus::Running => "Governance execution is still running",
        GovernanceStatus::PendingSelection => {
            "Governance execution is still selecting the next action"
        }
        GovernanceStatus::Failed => "Governance execution failed",
    };

    Some(headline.to_string())
}

/// Produces a human-readable message string for a [`GovernanceResponse`],
/// incorporating the `run_ref` and packet readiness where relevant.
pub(super) fn default_message(
    status: GovernanceStatus,
    run_ref: &str,
    packet_readiness: Option<PacketReadiness>,
) -> String {
    match status {
        GovernanceStatus::GovernedReady => {
            format!("run `{run_ref}` produced a reusable governed packet")
        }
        GovernanceStatus::AwaitingApproval => {
            format!("run `{run_ref}` is awaiting approval before downstream reuse")
        }
        GovernanceStatus::Blocked => match packet_readiness {
            Some(PacketReadiness::Incomplete) => {
                format!("run `{run_ref}` is blocked because the governed packet is incomplete")
            }
            Some(PacketReadiness::Rejected) => {
                format!("run `{run_ref}` is blocked because the governed packet is not reusable")
            }
            _ => format!("run `{run_ref}` is blocked by Canon governance"),
        },
        GovernanceStatus::Completed => {
            format!("run `{run_ref}` completed without a reusable packet projection")
        }
        GovernanceStatus::Running => format!("run `{run_ref}` is still running"),
        GovernanceStatus::PendingSelection => {
            format!("run `{run_ref}` is still selecting the next governed step")
        }
        GovernanceStatus::Failed => format!("run `{run_ref}` failed"),
    }
}

/// Builds a [`ClassificationProvenance`] indicating that risk and zone were
/// supplied explicitly by the governance adapter request.
///
/// Used during `handle_start` to mark adapter-provided classifications as
/// having an explicit, non-inferred source so downstream lineage is accurate.
pub(super) fn explicit_classification() -> ClassificationProvenance {
    ClassificationProvenance {
        risk: ClassificationFieldProvenance::new(
            ClassificationSource::Explicit,
            "Risk class was supplied by the governance adapter request.".to_string(),
            Vec::new(),
        ),
        zone: ClassificationFieldProvenance::new(
            ClassificationSource::Explicit,
            "Usage zone was supplied by the governance adapter request.".to_string(),
            Vec::new(),
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use canon_engine::domain::run::RunState;

    // ── authority_approval_state ─────────────────────────────────────────

    #[test]
    fn authority_approval_state_maps_all_variants() {
        assert_eq!(
            authority_approval_state(ApprovalState::NotNeeded),
            AuthorityApprovalState::NotNeeded
        );
        assert_eq!(
            authority_approval_state(ApprovalState::Requested),
            AuthorityApprovalState::Requested
        );
        assert_eq!(
            authority_approval_state(ApprovalState::Granted),
            AuthorityApprovalState::Granted
        );
        assert_eq!(
            authority_approval_state(ApprovalState::Rejected),
            AuthorityApprovalState::Rejected
        );
        assert_eq!(
            authority_approval_state(ApprovalState::Expired),
            AuthorityApprovalState::Expired
        );
    }

    // ── authority_packet_readiness ───────────────────────────────────────

    #[test]
    fn authority_packet_readiness_maps_all_variants() {
        assert_eq!(
            authority_packet_readiness(Some(PacketReadiness::Pending)),
            AuthorityPacketReadiness::Pending
        );
        assert_eq!(
            authority_packet_readiness(Some(PacketReadiness::Incomplete)),
            AuthorityPacketReadiness::Incomplete
        );
        assert_eq!(
            authority_packet_readiness(Some(PacketReadiness::Reusable)),
            AuthorityPacketReadiness::Reusable
        );
        assert_eq!(
            authority_packet_readiness(Some(PacketReadiness::Rejected)),
            AuthorityPacketReadiness::Rejected
        );
        // None defaults to Pending
        assert_eq!(authority_packet_readiness(None), AuthorityPacketReadiness::Pending);
    }

    // ── normalized_status ───────────────────────────────────────────────—

    #[test]
    fn completed_with_incomplete_readiness_maps_to_blocked() {
        let status = normalized_status(RunState::Completed, Some(PacketReadiness::Incomplete));
        assert_eq!(status, GovernanceStatus::Blocked);
    }

    #[test]
    fn completed_with_rejected_readiness_maps_to_blocked() {
        let status = normalized_status(RunState::Completed, Some(PacketReadiness::Rejected));
        assert_eq!(status, GovernanceStatus::Blocked);
    }

    #[test]
    fn completed_without_readiness_maps_to_completed() {
        let status = normalized_status(RunState::Completed, None);
        assert_eq!(status, GovernanceStatus::Completed);
    }

    #[test]
    fn failed_maps_to_failed() {
        let status = normalized_status(RunState::Failed, None);
        assert_eq!(status, GovernanceStatus::Failed);
    }

    #[test]
    fn aborted_maps_to_failed() {
        let status = normalized_status(RunState::Aborted, None);
        assert_eq!(status, GovernanceStatus::Failed);
    }

    #[test]
    fn executing_maps_to_running() {
        let status = normalized_status(RunState::Executing, None);
        assert_eq!(status, GovernanceStatus::Running);
    }

    #[test]
    fn awaiting_approval_maps_to_awaiting_approval() {
        let status = normalized_status(RunState::AwaitingApproval, None);
        assert_eq!(status, GovernanceStatus::AwaitingApproval);
    }

    // ── response_reason_code ─────────────────────────────────────────────

    #[test]
    fn response_reason_code_for_awaiting_approval() {
        let code = response_reason_code(GovernanceStatus::AwaitingApproval, None);
        assert_eq!(code, Some(GovernanceReasonCode::ApprovalRequired));
    }

    #[test]
    fn response_reason_code_for_blocked_incomplete() {
        let code =
            response_reason_code(GovernanceStatus::Blocked, Some(PacketReadiness::Incomplete));
        assert_eq!(code, Some(GovernanceReasonCode::IncompletePacket));
    }

    #[test]
    fn response_reason_code_for_blocked_rejected() {
        let code = response_reason_code(GovernanceStatus::Blocked, Some(PacketReadiness::Rejected));
        assert_eq!(code, Some(GovernanceReasonCode::RejectedPacket));
    }

    #[test]
    fn response_reason_code_for_blocked_default() {
        let code = response_reason_code(GovernanceStatus::Blocked, None);
        assert_eq!(code, Some(GovernanceReasonCode::BlockedByGovernance));
    }

    #[test]
    fn response_reason_code_for_failed() {
        let code = response_reason_code(GovernanceStatus::Failed, None);
        assert_eq!(code, Some(GovernanceReasonCode::RunFailed));
    }

    #[test]
    fn response_reason_code_for_completed_is_none() {
        let code = response_reason_code(GovernanceStatus::Completed, None);
        assert_eq!(code, None);
    }

    // ── default_headline ─────────────────────────────────────────────────

    #[test]
    fn default_headline_for_completed() {
        let h = default_headline(GovernanceStatus::Completed, None);
        assert_eq!(h, Some("Governance execution completed".to_string()));
    }

    #[test]
    fn default_headline_for_running() {
        let h = default_headline(GovernanceStatus::Running, None);
        assert_eq!(h, Some("Governance execution is still running".to_string()));
    }

    #[test]
    fn default_headline_for_failed() {
        let h = default_headline(GovernanceStatus::Failed, None);
        assert_eq!(h, Some("Governance execution failed".to_string()));
    }

    #[test]
    fn default_headline_for_pending_selection() {
        let h = default_headline(GovernanceStatus::PendingSelection, None);
        assert_eq!(h, Some("Governance execution is still selecting the next action".to_string()));
    }

    // ── default_message ──────────────────────────────────────────────────

    #[test]
    fn default_message_for_completed() {
        let msg = default_message(GovernanceStatus::Completed, "r1", None);
        assert!(msg.contains("r1"));
        assert!(msg.contains("completed"));
    }

    #[test]
    fn default_message_for_running() {
        let msg = default_message(GovernanceStatus::Running, "r1", None);
        assert!(msg.contains("r1"));
        assert!(msg.contains("still running"));
    }

    #[test]
    fn default_message_for_failed() {
        let msg = default_message(GovernanceStatus::Failed, "r1", None);
        assert!(msg.contains("r1"));
        assert!(msg.contains("failed"));
    }

    #[test]
    fn default_message_for_pending_selection() {
        let msg = default_message(GovernanceStatus::PendingSelection, "r1", None);
        assert!(msg.contains("r1"));
        assert!(msg.contains("selecting"));
    }

    #[test]
    fn default_message_for_blocked_incomplete() {
        let msg =
            default_message(GovernanceStatus::Blocked, "r1", Some(PacketReadiness::Incomplete));
        assert!(msg.contains("incomplete"));
    }

    #[test]
    fn default_message_for_blocked_rejected() {
        let msg = default_message(GovernanceStatus::Blocked, "r1", Some(PacketReadiness::Rejected));
        assert!(msg.contains("not reusable"));
    }

    #[test]
    fn default_message_for_governed_ready() {
        let msg = default_message(GovernanceStatus::GovernedReady, "r1", None);
        assert!(msg.contains("reusable"));
    }

    // ── existing tests ───────────────────────────────────────────────────

    #[test]
    fn awaiting_reviewer_output_maps_to_pending_selection() {
        let status = normalized_status(RunState::AwaitingReviewerOutput, None);
        assert_eq!(status, GovernanceStatus::PendingSelection);
    }

    #[test]
    fn draft_maps_to_pending_selection() {
        let status = normalized_status(RunState::Draft, None);
        assert_eq!(status, GovernanceStatus::PendingSelection);
    }

    #[test]
    fn completed_with_reusable_readiness_maps_to_governed_ready() {
        let status = normalized_status(RunState::Completed, Some(PacketReadiness::Reusable));
        assert_eq!(status, GovernanceStatus::GovernedReady);
    }

    #[test]
    fn blocked_maps_to_blocked() {
        let status = normalized_status(RunState::Blocked, None);
        assert_eq!(status, GovernanceStatus::Blocked);
    }
}
