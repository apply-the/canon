//! Run-state derivation and next-action recommendation.

use crate::domain::gate::{GateEvaluation, GateStatus};
use crate::domain::run::RunState;

use super::{GateInspectSummary, ModeResultSummary, RecommendedActionSummary};

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
}
