//! Execution-posture, approval-gate, and run-context mutation helpers.

use crate::domain::approval::ApprovalRecord;
use crate::domain::execution::ExecutionPosture;
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
use crate::domain::run::RunContext;

use super::ModeResultSummary;

// ── Execution posture resolution ─────────────────────────────────────────────

pub(crate) fn apply_execution_posture_summary(
    mode_result: Option<ModeResultSummary>,
    context: Option<&RunContext>,
    approvals: &[ApprovalRecord],
) -> Option<ModeResultSummary> {
    let mut mode_result = mode_result?;
    if let Some(execution_posture) = resolved_execution_posture_label(context, approvals) {
        mode_result.execution_posture = Some(execution_posture);
    }
    Some(mode_result)
}

pub(crate) fn resolved_execution_posture_label(
    context: Option<&RunContext>,
    approvals: &[ApprovalRecord],
) -> Option<String> {
    let base = context.and_then(execution_posture_label);
    let execution_approved = execution_gate_is_approved(approvals);
    let continuation_consumed = context.is_some_and(post_approval_execution_consumed);
    match (base, execution_approved, continuation_consumed) {
        (Some("recommendation-only"), true, true) => Some("approved-recommendation".to_string()),
        (other, _, _) => other.map(str::to_string),
    }
}

pub(crate) fn resolved_execution_posture_label_for_mode(
    mode: Mode,
    context: Option<&RunContext>,
    approvals: &[ApprovalRecord],
) -> Option<String> {
    resolved_execution_posture_label(context, approvals).or_else(|| match mode {
        Mode::Incident | Mode::Migration | Mode::SecurityAssessment | Mode::SupplyChainAnalysis => {
            Some("recommendation-only".to_string())
        }
        _ => None,
    })
}

pub(crate) fn execution_posture_label(context: &RunContext) -> Option<&'static str> {
    context
        .implementation_execution
        .as_ref()
        .map(|execution| execution.execution_posture)
        .or_else(|| {
            context.refactor_execution.as_ref().map(|execution| execution.execution_posture)
        })
        .map(|posture| match posture {
            ExecutionPosture::Mutating => "mutating",
            ExecutionPosture::RecommendationOnly => "recommendation-only",
        })
}

pub(crate) fn set_execution_posture(context: &mut RunContext, posture: ExecutionPosture) {
    if let Some(execution) = context.implementation_execution.as_mut() {
        execution.execution_posture = posture;
    }

    if let Some(execution) = context.refactor_execution.as_mut() {
        execution.execution_posture = posture;
    }
}

pub(crate) fn post_approval_execution_consumed(context: &RunContext) -> bool {
    context
        .implementation_execution
        .as_ref()
        .map(|execution| execution.post_approval_execution_consumed)
        .or_else(|| {
            context
                .refactor_execution
                .as_ref()
                .map(|execution| execution.post_approval_execution_consumed)
        })
        .unwrap_or(false)
}

pub(crate) fn set_post_approval_execution_consumed(context: &mut RunContext, consumed: bool) {
    if let Some(execution) = context.implementation_execution.as_mut() {
        execution.post_approval_execution_consumed = consumed;
    }

    if let Some(execution) = context.refactor_execution.as_mut() {
        execution.post_approval_execution_consumed = consumed;
    }
}

pub(crate) fn execution_continuation_pending(
    context: &RunContext,
    approvals: &[ApprovalRecord],
) -> bool {
    execution_gate_is_approved(approvals) && !post_approval_execution_consumed(context)
}

pub(crate) fn execution_gate_is_approved(approvals: &[ApprovalRecord]) -> bool {
    approvals
        .iter()
        .any(|approval| approval.matches_gate(GateKind::Execution) && approval.is_approved())
}

// ── Approval record helpers ───────────────────────────────────────────────────

pub(crate) fn approval_record_refs(run_id: &str, approvals: &[ApprovalRecord]) -> Vec<String> {
    approvals
        .iter()
        .enumerate()
        .map(|(index, _)| format!("runs/{run_id}/approvals/approval-{index:02}.toml"))
        .collect()
}

pub(crate) fn approved_execution_mutation_rationale(
    mode: Mode,
    declared_scope: &[String],
    patch_path: &str,
) -> String {
    match mode {
        Mode::Implementation => format!(
            "implementation mutation is approved for bounded execution within the declared mutation bounds using authored patch payload {patch_path}: {}",
            declared_scope.join(", ")
        ),
        Mode::Refactor => format!(
            "refactor mutation is approved for bounded execution within the declared refactor scope using authored patch payload {patch_path}: {}",
            declared_scope.join(", ")
        ),
        Mode::Change => format!(
            "change mutation is approved for bounded execution using authored patch payload {patch_path}: {}",
            declared_scope.join(", ")
        ),
        _ => format!(
            "bounded mutation is approved for execution using authored patch payload {patch_path}: {}",
            declared_scope.join(", ")
        ),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::approval::ApprovalDecision;
    use crate::domain::execution::{MutationBounds, MutationExpansionPolicy};
    use crate::domain::run::ImplementationExecutionContext;
    use time::OffsetDateTime;

    fn make_impl_context(posture: ExecutionPosture, consumed: bool) -> RunContext {
        RunContext {
            repo_root: ".".to_string(),
            owner: None,
            inputs: Vec::new(),
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
            system_context: None,
            upstream_context: None,
            implementation_execution: Some(ImplementationExecutionContext {
                plan_sources: vec!["canon-input/implementation.md".to_string()],
                execution_posture: posture,
                post_approval_execution_consumed: consumed,
                mutation_bounds: MutationBounds {
                    declared_paths: Vec::new(),
                    owners: Vec::new(),
                    source_refs: Vec::new(),
                    expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                },
                task_targets: Vec::new(),
                safety_net: Vec::new(),
                rollback_expectations: Vec::new(),
            }),
            refactor_execution: None,
            backlog_planning: None,
        }
    }

    fn make_execution_approval() -> ApprovalRecord {
        ApprovalRecord {
            gate: Some(GateKind::Execution),
            invocation_request_id: None,
            by: "tester".to_string(),
            decision: ApprovalDecision::Approve,
            rationale: "ok".to_string(),
            recorded_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn execution_posture_label_returns_recommendation_only_by_default() {
        let ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        assert_eq!(execution_posture_label(&ctx), Some("recommendation-only"));
    }

    #[test]
    fn execution_posture_label_returns_mutating_when_set() {
        let ctx = make_impl_context(ExecutionPosture::Mutating, false);
        assert_eq!(execution_posture_label(&ctx), Some("mutating"));
    }

    #[test]
    fn set_execution_posture_updates_implementation_context() {
        let mut ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        set_execution_posture(&mut ctx, ExecutionPosture::Mutating);
        assert_eq!(
            ctx.implementation_execution.unwrap().execution_posture,
            ExecutionPosture::Mutating
        );
    }

    #[test]
    fn post_approval_execution_consumed_is_false_by_default() {
        let ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        assert!(!post_approval_execution_consumed(&ctx));
    }

    #[test]
    fn set_post_approval_execution_consumed_sets_flag() {
        let mut ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        set_post_approval_execution_consumed(&mut ctx, true);
        assert!(ctx.implementation_execution.unwrap().post_approval_execution_consumed);
    }

    #[test]
    fn execution_continuation_pending_true_when_approved_not_consumed() {
        let ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        let approvals = vec![make_execution_approval()];
        assert!(execution_continuation_pending(&ctx, &approvals));
    }

    #[test]
    fn execution_continuation_pending_false_when_not_approved() {
        let ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        assert!(!execution_continuation_pending(&ctx, &[]));
    }

    #[test]
    fn execution_continuation_pending_false_when_consumed() {
        let ctx = make_impl_context(ExecutionPosture::RecommendationOnly, true);
        let approvals = vec![make_execution_approval()];
        assert!(!execution_continuation_pending(&ctx, &approvals));
    }

    #[test]
    fn resolved_execution_posture_label_promotes_approved_recommendation() {
        let mut ctx = make_impl_context(ExecutionPosture::RecommendationOnly, true);
        ctx.implementation_execution.as_mut().unwrap().post_approval_execution_consumed = true;
        let approvals = vec![make_execution_approval()];
        let label = resolved_execution_posture_label(Some(&ctx), &approvals);
        assert_eq!(label, Some("approved-recommendation".to_string()));
    }

    #[test]
    fn resolved_execution_posture_label_stays_recommendation_only_before_resume() {
        let ctx = make_impl_context(ExecutionPosture::RecommendationOnly, false);
        let approvals = vec![make_execution_approval()];
        let label = resolved_execution_posture_label(Some(&ctx), &approvals);
        assert_eq!(label, Some("recommendation-only".to_string()));
    }

    #[test]
    fn approval_record_refs_generates_paths() {
        let approval = make_execution_approval();
        let refs = approval_record_refs("run-xyz", &[approval]);
        assert_eq!(refs, vec!["runs/run-xyz/approvals/approval-00.toml"]);
    }

    #[test]
    fn approved_execution_mutation_rationale_covers_implementation() {
        let rationale = approved_execution_mutation_rationale(
            Mode::Implementation,
            &["src/**".to_string()],
            "canon-input/implementation/mutation.diff",
        );
        assert!(rationale.contains("implementation mutation is approved"));
        assert!(rationale.contains("src/**"));
    }

    #[test]
    fn approved_execution_mutation_rationale_covers_refactor() {
        let rationale = approved_execution_mutation_rationale(
            Mode::Refactor,
            &["crates/**".to_string()],
            "patch.diff",
        );
        assert!(rationale.contains("refactor mutation is approved"));
    }
}
