use super::EngineService;
use super::*;

impl EngineService {
    pub(super) fn summarize_run(
        &self,
        store: &WorkspaceStore,
        spec: RunSummarySpec<'_>,
    ) -> Result<RunSummary, EngineError> {
        let details =
            self.collect_run_runtime_details(store, spec.run_id, spec.mode, spec.state)?;
        let manifest = store.load_run_manifest(spec.run_id)?;

        Ok(RunSummary {
            run_id: spec.run_id.to_string(),
            uuid: manifest.uuid.clone(),
            owner: manifest.owner,
            mode: spec.mode.as_str().to_string(),
            risk: spec.risk.as_str().to_string(),
            zone: spec.zone.as_str().to_string(),
            system_context: details.system_context.map(|context| context.as_str().to_string()),
            state: format!("{:?}", spec.state),
            artifact_count: spec.artifact_count,
            invocations_total: details.invocations_total,
            invocations_denied: details.invocations_denied,
            invocations_pending_approval: details.pending_invocation_approvals,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            closure_status: details.closure_status,
            decomposition_scope: details.decomposition_scope,
            closure_findings: details.closure_findings,
            closure_notes: details.closure_notes,
            possible_actions: details.possible_actions,
            mode_result: details.mode_result,
            recommended_next_action: details.recommended_next_action,
        })
    }

    pub(super) fn collect_run_runtime_details(
        &self,
        store: &WorkspaceStore,
        run_id: &str,
        mode: Mode,
        state: RunState,
    ) -> Result<RunRuntimeDetails, EngineError> {
        let invocations = store.load_persisted_invocations(run_id).unwrap_or_default();
        let evidence_bundle = store.load_evidence_bundle(run_id)?;
        let gates = store.load_gate_evaluations(run_id).unwrap_or_default();
        let context = store.load_run_context(run_id).ok();
        let system_context = context.as_ref().and_then(|context| context.system_context);
        let backlog_planning =
            context.as_ref().and_then(|context| context.backlog_planning.as_ref());

        let persisted_artifacts = store
            .load_artifact_contract(run_id)
            .ok()
            .and_then(|contract| store.load_persisted_artifacts(run_id, mode, &contract).ok());

        let artifact_paths = persisted_artifacts
            .as_ref()
            .map(|artifacts| {
                artifacts
                    .iter()
                    .map(|artifact| format!(".canon/{}", artifact.record.relative_path))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mode_result = persisted_artifacts
            .as_ref()
            .and_then(|artifacts| summarize_mode_result(mode, artifacts));
        let approvals = store.load_approval_records(run_id).unwrap_or_default();
        let mode_result =
            apply_execution_posture_summary(mode_result, context.as_ref(), &approvals);

        let pending_invocation_targets = invocations
            .iter()
            .filter(|invocation| {
                invocation.decision.requires_approval
                    && !invocation
                        .approvals
                        .iter()
                        .any(|approval| matches!(approval.decision, ApprovalDecision::Approve))
            })
            .map(|invocation| format!("invocation:{}", invocation.request.request_id))
            .collect::<Vec<_>>();

        let pending_gate_targets = gates
            .iter()
            .filter(|gate| matches!(gate.status, GateStatus::NeedsApproval))
            .map(|gate| format!("gate:{}", gate.gate.as_str()))
            .collect::<Vec<_>>();

        let blocked_gates = gates
            .iter()
            .filter(|gate| matches!(gate.status, GateStatus::Blocked))
            .map(|gate| GateInspectSummary {
                gate: gate.gate.as_str().to_string(),
                status: format!("{:?}", gate.status),
                blockers: gate.blockers.clone(),
            })
            .collect::<Vec<_>>();

        let mut approval_targets = pending_gate_targets;
        approval_targets.extend(pending_invocation_targets);

        let mode_result = mode_result.map(|mut summary| {
            summary.action_chips = build_action_chips_for(
                state,
                &approval_targets,
                &summary.primary_artifact_path,
                run_id,
            );
            summary
        });

        let blocking_classification =
            if !approval_targets.is_empty() || matches!(state, RunState::AwaitingApproval) {
                Some("approval-gated".to_string())
            } else if !blocked_gates.is_empty() || matches!(state, RunState::Blocked) {
                Some("artifact-blocked".to_string())
            } else {
                None
            };

        let validation_independence_satisfied = evidence_bundle
            .as_ref()
            .map(|bundle| bundle.validation_paths.iter().all(|path| path.independence.sufficient))
            .unwrap_or(true);

        let closure_status = backlog_planning
            .map(|planning| planning.closure_assessment.status.as_str().to_string());
        let decomposition_scope = backlog_planning
            .map(|planning| planning.closure_assessment.decomposition_scope.as_str().to_string());
        let closure_findings = backlog_planning
            .map(|planning| {
                planning
                    .closure_assessment
                    .findings
                    .iter()
                    .map(|finding| ClosureFindingInspectSummary {
                        category: finding.category.clone(),
                        severity: finding.severity.as_str().to_string(),
                        affected_scope: finding.affected_scope.clone(),
                        recommended_followup: finding.recommended_followup.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let closure_notes =
            backlog_planning.and_then(|planning| planning.closure_assessment.notes.clone());

        let recommended_next_action = recommend_next_action(
            state,
            mode_result.as_ref(),
            &artifact_paths,
            evidence_bundle.is_some(),
            &blocked_gates,
            &approval_targets,
        );
        let possible_actions = build_possible_actions(
            state,
            mode_result.as_ref(),
            &artifact_paths,
            evidence_bundle.is_some(),
            &blocked_gates,
            &approval_targets,
            run_id,
        );

        Ok(RunRuntimeDetails {
            system_context,
            invocations_total: invocations.len(),
            invocations_denied: evidence_bundle
                .as_ref()
                .map(|bundle| bundle.denied_invocations.len())
                .unwrap_or(0),
            pending_invocation_approvals: approval_targets
                .iter()
                .filter(|target| target.starts_with("invocation:"))
                .count(),
            validation_independence_satisfied,
            blocking_classification,
            blocked_gates,
            approval_targets,
            artifact_paths,
            closure_status,
            decomposition_scope,
            closure_findings,
            closure_notes,
            possible_actions,
            mode_result,
            recommended_next_action,
        })
    }
}
