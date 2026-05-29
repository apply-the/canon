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
            refinement_state: details.refinement_state,
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
        let refinement_state = context.as_ref().and_then(Self::summarize_refinement_state);

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
            refinement_state,
            mode_result,
            recommended_next_action,
        })
    }

    pub(super) fn summarize_refinement_state(
        context: &RunContext,
    ) -> Option<RefinementStateSummary> {
        let refinement: &ClarificationRefinementContext =
            context.clarification_refinement.as_ref()?;
        let readiness_items = refinement
            .readiness_delta
            .iter()
            .map(|item| RefinementReadinessItemSummary {
                id: item.id.clone(),
                section: item.section.clone(),
                summary: item.summary.clone(),
                blocking: item.blocking,
                source_kind: readiness_source_kind_label(item).to_string(),
                default_available: item.default_available,
                resolved: item.resolved,
            })
            .collect::<Vec<_>>();
        let unresolved_records = refinement
            .records
            .iter()
            .filter(|record| {
                !matches!(
                    record.resolution_state,
                    crate::domain::run::ClarificationResolutionState::Resolved
                )
            })
            .count();

        Some(RefinementStateSummary {
            workflow_family: refinement_workflow_family_label(refinement).to_string(),
            current_mode: refinement.current_mode.as_str().to_string(),
            working_brief_path: refinement.working_brief_path.clone(),
            template_ref: refinement.template_ref.clone(),
            status: refinement_status_label(refinement).to_string(),
            explicit_continuation_required: refinement.explicit_continuation_required,
            authoritative_input_refs: refinement.authoritative_input_refs.clone(),
            supporting_input_refs: refinement.supporting_input_refs.clone(),
            records_total: refinement.records.len(),
            unresolved_records,
            readiness_delta: Self::build_refinement_readiness_delta(&refinement.readiness_delta),
            readiness_items,
            suggested_candidate: refinement.suggested_candidate.as_ref().map(|candidate| {
                RefinementCandidateSummary {
                    run_id: candidate.run_id.clone(),
                    mode: candidate.mode.as_str().to_string(),
                    state: format!("{:?}", candidate.state),
                    match_reason: candidate.match_reason.clone(),
                    advisory: candidate.advisory,
                }
            }),
        })
    }
}

fn refinement_workflow_family_label(refinement: &ClarificationRefinementContext) -> &'static str {
    match refinement.workflow_family {
        crate::domain::run::RefinementWorkflowFamily::Planning => "planning",
        crate::domain::run::RefinementWorkflowFamily::Execution => "execution",
        crate::domain::run::RefinementWorkflowFamily::Assessment => "assessment",
    }
}

fn refinement_status_label(refinement: &ClarificationRefinementContext) -> &'static str {
    match refinement.status {
        crate::domain::run::ClarificationRefinementStatus::Active => "active",
        crate::domain::run::ClarificationRefinementStatus::Ready => "ready",
        crate::domain::run::ClarificationRefinementStatus::Superseded => "superseded",
    }
}

fn readiness_source_kind_label(item: &ReadinessDeltaItem) -> &'static str {
    match item.source_kind {
        crate::domain::run::ReadinessDeltaSourceKind::AuthorityGap => "authority-gap",
        crate::domain::run::ReadinessDeltaSourceKind::MissingContext => "missing-context",
        crate::domain::run::ReadinessDeltaSourceKind::ClarificationGap => "clarification-gap",
        crate::domain::run::ReadinessDeltaSourceKind::SupportingInputWarning => {
            "supporting-input-warning"
        }
    }
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::EngineService;
    use crate::domain::mode::Mode;
    use crate::domain::run::{
        ClarificationAnswerKind, ClarificationRecord, ClarificationRefinementContext,
        ClarificationRefinementStatus, ClarificationResolutionState, ContinuationCandidateSummary,
        ReadinessDeltaItem, ReadinessDeltaSourceKind, RefinementWorkflowFamily, RunContext,
        RunState,
    };

    fn sample_context() -> RunContext {
        RunContext {
            repo_root: "/repo".to_string(),
            owner: Some("owner@example.com".to_string()),
            inputs: vec!["canon-input/requirements/brief.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: None,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: Some(ClarificationRefinementContext {
                workflow_family: RefinementWorkflowFamily::Planning,
                current_mode: Mode::Requirements,
                working_brief_path:
                    ".canon/runs/R-20260529-ab12cd34/artifacts/requirements/working-brief.md"
                        .to_string(),
                template_ref: "docs/templates/canon-input/requirements.md".to_string(),
                status: ClarificationRefinementStatus::Active,
                explicit_continuation_required: true,
                authoritative_input_refs: vec!["canon-input/requirements/brief.md".to_string()],
                supporting_input_refs: vec![
                    "canon-input/requirements/context-links.md".to_string(),
                ],
                suggested_candidate: Some(ContinuationCandidateSummary {
                    run_id: "R-20260529-ab12cd34".to_string(),
                    mode: Mode::Requirements,
                    state: RunState::Draft,
                    match_reason: "same authoritative input fingerprint".to_string(),
                    advisory: true,
                }),
                records: vec![
                    ClarificationRecord {
                        id: "cq-001".to_string(),
                        prompt: "Which actor owns the problem?".to_string(),
                        answer: "platform operators".to_string(),
                        answer_kind: ClarificationAnswerKind::Explicit,
                        affected_sections: vec!["Actors".to_string()],
                        resolution_state: ClarificationResolutionState::Resolved,
                        recorded_at: OffsetDateTime::UNIX_EPOCH,
                    },
                    ClarificationRecord {
                        id: "cq-002".to_string(),
                        prompt: "Who owns independent validation?".to_string(),
                        answer: "deferred".to_string(),
                        answer_kind: ClarificationAnswerKind::Deferred,
                        affected_sections: vec!["Validation Strategy".to_string()],
                        resolution_state: ClarificationResolutionState::Deferred,
                        recorded_at: OffsetDateTime::UNIX_EPOCH,
                    },
                ],
                readiness_delta: vec![
                    ReadinessDeltaItem {
                        id: "rd-001".to_string(),
                        section: "Validation Strategy".to_string(),
                        summary: "Independent validation owner is not yet named.".to_string(),
                        blocking: true,
                        source_kind: ReadinessDeltaSourceKind::MissingContext,
                        default_available: false,
                        resolved: false,
                    },
                    ReadinessDeltaItem {
                        id: "rd-002".to_string(),
                        section: "Supporting Inputs".to_string(),
                        summary: "Supporting inputs cannot replace the authoritative brief."
                            .to_string(),
                        blocking: false,
                        source_kind: ReadinessDeltaSourceKind::SupportingInputWarning,
                        default_available: false,
                        resolved: true,
                    },
                ],
            }),
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn summarize_refinement_state_flattens_unresolved_readiness_items() {
        let summary = EngineService::summarize_refinement_state(&sample_context())
            .expect("refinement summary should be present");

        assert_eq!(summary.workflow_family, "planning");
        assert_eq!(summary.current_mode, "requirements");
        assert_eq!(summary.status, "active");
        assert_eq!(summary.records_total, 2);
        assert_eq!(summary.unresolved_records, 1);
        assert_eq!(
            summary.readiness_delta,
            vec!["Independent validation owner is not yet named.".to_string()]
        );
        assert_eq!(summary.readiness_items.len(), 2);
        assert_eq!(
            summary.suggested_candidate.expect("candidate summary should be present").run_id,
            "R-20260529-ab12cd34"
        );
    }
}
