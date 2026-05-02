//! Run-state derivation and next-action recommendation.

use crate::domain::gate::{GateEvaluation, GateStatus};
use crate::domain::run::RunState;

use super::{
    GateInspectSummary, ModeResultSummary, PossibleActionSummary, RecommendedActionSummary,
};

// ── Gate → RunState ───────────────────────────────────────────────────────────

pub(crate) fn run_state_from_gates(gates: &[GateEvaluation]) -> RunState {
    if gates.iter().any(|gate| matches!(gate.status, GateStatus::NeedsApproval)) {
        RunState::AwaitingApproval
    } else if gates.iter().any(|gate| matches!(gate.status, GateStatus::Blocked)) {
        RunState::Blocked
    } else {
        RunState::Completed
    }
}

// ── Next-action recommendation ────────────────────────────────────────────────

pub(crate) fn recommend_next_action(
    state: RunState,
    mode_result: Option<&ModeResultSummary>,
    artifact_paths: &[String],
    has_evidence_bundle: bool,
    blocked_gates: &[GateInspectSummary],
    approval_targets: &[String],
) -> Option<RecommendedActionSummary> {
    if !approval_targets.is_empty() {
        if !artifact_paths.is_empty() {
            return Some(RecommendedActionSummary {
                action: "inspect-artifacts".to_string(),
                rationale: "Review the emitted packet before recording approval.".to_string(),
                target: None,
            });
        }

        if has_evidence_bundle {
            return Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "Approval is required; inspect the evidence lineage before deciding."
                    .to_string(),
                target: None,
            });
        }

        return Some(RecommendedActionSummary {
            action: "approve".to_string(),
            rationale: "Canon is explicitly waiting for approval on a real target.".to_string(),
            target: approval_targets.first().cloned(),
        });
    }

    if matches!(state, RunState::AwaitingApproval) {
        return Some(RecommendedActionSummary {
            action: "resume".to_string(),
            rationale: "Approval is already recorded; resume the run to execute the post-approval continuation."
                .to_string(),
            target: None,
        });
    }

    if !blocked_gates.is_empty() || matches!(state, RunState::Blocked) {
        if !artifact_paths.is_empty() {
            return Some(RecommendedActionSummary {
                action: "inspect-artifacts".to_string(),
                rationale: "The run is blocked by gate blockers in the emitted packet, not by a pending approval."
                    .to_string(),
                target: None,
            });
        }

        if has_evidence_bundle {
            return Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "The run is blocked but no readable artifact packet was found; inspect the evidence bundle next."
                    .to_string(),
                target: None,
            });
        }
    }

    if matches!(state, RunState::Completed) {
        if mode_result.is_some() {
            return None;
        }

        if !artifact_paths.is_empty() {
            return Some(RecommendedActionSummary {
                action: "inspect-artifacts".to_string(),
                rationale: "The run completed and emitted readable artifacts worth reviewing."
                    .to_string(),
                target: None,
            });
        }

        if has_evidence_bundle {
            return Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "The run completed; inspect the evidence bundle for execution lineage."
                    .to_string(),
                target: None,
            });
        }
    }

    None
}

pub(crate) fn build_possible_actions(
    state: RunState,
    mode_result: Option<&ModeResultSummary>,
    artifact_paths: &[String],
    has_evidence_bundle: bool,
    blocked_gates: &[GateInspectSummary],
    approval_targets: &[String],
    run_id: &str,
) -> Vec<PossibleActionSummary> {
    let mut actions = Vec::new();

    if let Some(mode_result) = mode_result
        && !mode_result.primary_artifact_path.is_empty()
    {
        actions.push(PossibleActionSummary {
            action: "open-primary-artifact".to_string(),
            text: mode_result.primary_artifact_action.text_fallback.clone(),
            target: Some(mode_result.primary_artifact_path.clone()),
        });
    }

    if !artifact_paths.is_empty() && !run_id.is_empty() {
        actions.push(PossibleActionSummary {
            action: "inspect-artifacts".to_string(),
            text: format!(
                "Use $canon-inspect-artifacts for the full emitted packet on run {run_id}."
            ),
            target: None,
        });
    }

    if has_evidence_bundle && !run_id.is_empty() {
        actions.push(PossibleActionSummary {
            action: "inspect-evidence".to_string(),
            text: format!(
                "Use $canon-inspect-evidence only if you need lineage or policy rationale for run {run_id}."
            ),
            target: None,
        });
    }

    if !run_id.is_empty() {
        for target in approval_targets {
            actions.push(PossibleActionSummary {
                action: "approve".to_string(),
                text: format!(
                    "Use $canon-approve for target {target} on run {run_id} after review."
                ),
                target: Some(target.clone()),
            });
        }
    }

    if matches!(state, RunState::AwaitingApproval)
        && approval_targets.is_empty()
        && !run_id.is_empty()
    {
        actions.push(PossibleActionSummary {
            action: "resume".to_string(),
            text: format!(
                "Use $canon-resume for run {run_id} only if Canon still requires continuation."
            ),
            target: None,
        });
        actions.push(PossibleActionSummary {
            action: "status".to_string(),
            text: format!(
                "Use $canon-status to confirm the post-approval run state for run {run_id}."
            ),
            target: None,
        });
    }

    if (!blocked_gates.is_empty() || matches!(state, RunState::Blocked))
        && artifact_paths.is_empty()
        && has_evidence_bundle
        && !run_id.is_empty()
        && !actions.iter().any(|action| action.action == "inspect-evidence")
    {
        actions.push(PossibleActionSummary {
            action: "inspect-evidence".to_string(),
            text: format!(
                "Use $canon-inspect-evidence only if you need lineage or policy rationale for run {run_id}."
            ),
            target: None,
        });
    }

    actions
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::gate::{GateKind, GateStatus};
    use time::OffsetDateTime;

    fn make_gate(kind: GateKind, status: GateStatus) -> GateEvaluation {
        GateEvaluation {
            gate: kind,
            status,
            blockers: Vec::new(),
            evaluated_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    fn make_blocked_gate(name: &str) -> GateInspectSummary {
        GateInspectSummary {
            gate: name.to_string(),
            status: "Blocked".to_string(),
            blockers: vec!["blocker".to_string()],
        }
    }

    #[test]
    fn run_state_from_gates_awaiting_approval_takes_priority() {
        let gates = vec![
            make_gate(GateKind::Execution, GateStatus::NeedsApproval),
            make_gate(GateKind::Risk, GateStatus::Blocked),
        ];
        assert_eq!(run_state_from_gates(&gates), RunState::AwaitingApproval);
    }

    #[test]
    fn run_state_from_gates_blocked_takes_priority_over_completed() {
        let gates = vec![
            make_gate(GateKind::Risk, GateStatus::Blocked),
            make_gate(GateKind::ReleaseReadiness, GateStatus::Passed),
        ];
        assert_eq!(run_state_from_gates(&gates), RunState::Blocked);
    }

    #[test]
    fn run_state_from_gates_completed_when_all_passed() {
        let gates = vec![make_gate(GateKind::ReleaseReadiness, GateStatus::Passed)];
        assert_eq!(run_state_from_gates(&gates), RunState::Completed);
    }

    #[test]
    fn run_state_from_gates_completed_for_empty_gates() {
        assert_eq!(run_state_from_gates(&[]), RunState::Completed);
    }

    #[test]
    fn recommend_next_action_returns_inspect_artifacts_when_approval_needed_and_artifacts_exist() {
        let action = recommend_next_action(
            RunState::AwaitingApproval,
            None,
            &[".canon/artifacts/run-1/task-mapping.md".to_string()],
            false,
            &[],
            &["gate:execution".to_string()],
        );
        assert_eq!(action.unwrap().action, "inspect-artifacts");
    }

    #[test]
    fn recommend_next_action_returns_resume_when_no_targets_and_awaiting() {
        let action = recommend_next_action(RunState::AwaitingApproval, None, &[], false, &[], &[]);
        assert_eq!(action.unwrap().action, "resume");
    }

    #[test]
    fn recommend_next_action_returns_none_for_completed_run_with_mode_result() {
        use super::super::ResultActionSummary;
        let mode_result = ModeResultSummary {
            headline: "done".to_string(),
            artifact_packet_summary: "packet".to_string(),
            execution_posture: None,
            primary_artifact_title: "Task Mapping".to_string(),
            primary_artifact_path: ".canon/artifacts/task-mapping.md".to_string(),
            primary_artifact_action: ResultActionSummary {
                id: "open-primary-artifact".to_string(),
                label: "Open".to_string(),
                host_action: "open-file".to_string(),
                target: ".canon/artifacts/task-mapping.md".to_string(),
                text_fallback: "Open.".to_string(),
            },
            result_excerpt: "excerpt".to_string(),
            action_chips: Vec::new(),
        };
        let action =
            recommend_next_action(RunState::Completed, Some(&mode_result), &[], false, &[], &[]);
        assert!(action.is_none());
    }

    #[test]
    fn recommend_next_action_returns_evidence_for_blocked_without_artifacts() {
        let blocked = vec![make_blocked_gate("artifact-completeness")];
        let action = recommend_next_action(RunState::Blocked, None, &[], true, &blocked, &[]);
        assert_eq!(action.unwrap().action, "inspect-evidence");
    }

    #[test]
    fn recommend_next_action_returns_approve_with_target_when_no_artifacts_or_evidence() {
        let action = recommend_next_action(
            RunState::AwaitingApproval,
            None,
            &[],
            false,
            &[],
            &["gate:execution".to_string()],
        );
        let action = action.unwrap();
        assert_eq!(action.action, "approve");
        assert_eq!(action.target, Some("gate:execution".to_string()));
    }

    #[test]
    fn build_possible_actions_completed_result_prefers_open_then_packet_review() {
        use super::super::ResultActionSummary;

        let mode_result = ModeResultSummary {
            headline: "done".to_string(),
            artifact_packet_summary: "packet".to_string(),
            execution_posture: None,
            primary_artifact_title: "Task Mapping".to_string(),
            primary_artifact_path: ".canon/artifacts/task-mapping.md".to_string(),
            primary_artifact_action: ResultActionSummary {
                id: "open-primary-artifact".to_string(),
                label: "Open".to_string(),
                host_action: "open-file".to_string(),
                target: ".canon/artifacts/task-mapping.md".to_string(),
                text_fallback: "Open the primary artifact at .canon/artifacts/task-mapping.md."
                    .to_string(),
            },
            result_excerpt: "excerpt".to_string(),
            action_chips: Vec::new(),
        };

        let actions = build_possible_actions(
            RunState::Completed,
            Some(&mode_result),
            &[".canon/artifacts/task-mapping.md".to_string()],
            true,
            &[],
            &[],
            "run-1",
        );

        let ids: Vec<&str> = actions.iter().map(|action| action.action.as_str()).collect();
        assert_eq!(ids, vec!["open-primary-artifact", "inspect-artifacts", "inspect-evidence"]);
    }

    #[test]
    fn build_possible_actions_resumable_state_prefers_resume_then_status() {
        let actions =
            build_possible_actions(RunState::AwaitingApproval, None, &[], false, &[], &[], "run-2");

        let ids: Vec<&str> = actions.iter().map(|action| action.action.as_str()).collect();
        assert_eq!(ids, vec!["resume", "status"]);
    }
}
