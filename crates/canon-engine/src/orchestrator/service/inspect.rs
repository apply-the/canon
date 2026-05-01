use super::EngineService;
use super::*;

impl EngineService {
    pub fn inspect(&self, target: InspectTarget) -> Result<InspectResponse, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let (name, system_context, entries) = match target {
            InspectTarget::Modes => (
                "modes".to_string(),
                None,
                Mode::all()
                    .iter()
                    .map(|mode| InspectEntry::Name(mode.as_str().to_string()))
                    .collect::<Vec<_>>(),
            ),
            InspectTarget::Methods => (
                "methods".to_string(),
                None,
                store.list_method_files()?.into_iter().map(InspectEntry::Name).collect::<Vec<_>>(),
            ),
            InspectTarget::Policies => (
                "policies".to_string(),
                None,
                store.list_policy_files()?.into_iter().map(InspectEntry::Name).collect::<Vec<_>>(),
            ),
            InspectTarget::RiskZone { mode, risk, zone, inputs, inline_inputs } => (
                "risk-zone".to_string(),
                None,
                vec![InspectEntry::RiskZone(self.inspect_risk_zone(
                    mode,
                    risk,
                    zone,
                    &inputs,
                    &inline_inputs,
                )?)],
            ),
            InspectTarget::Clarity { mode, inputs } => (
                "clarity".to_string(),
                None,
                vec![InspectEntry::Clarity(self.inspect_clarity(mode, &inputs)?)],
            ),
            InspectTarget::Artifacts { run_id } => {
                let run_id = self.resolve_run(&run_id)?;
                let system_context =
                    store.load_run_context(&run_id).ok().and_then(|context| context.system_context);
                (
                    "artifacts".to_string(),
                    system_context,
                    store
                        .list_artifact_files(&run_id)?
                        .into_iter()
                        .map(InspectEntry::Name)
                        .collect::<Vec<_>>(),
                )
            }
            InspectTarget::Invocations { run_id } => {
                let run_id = self.resolve_run(&run_id)?;
                let system_context =
                    store.load_run_context(&run_id).ok().and_then(|context| context.system_context);
                let artifacts = store
                    .load_run_manifest(&run_id)
                    .ok()
                    .and_then(|manifest| {
                        store
                            .load_artifact_contract(&run_id)
                            .ok()
                            .map(|contract| (manifest, contract))
                    })
                    .and_then(|(manifest, contract)| {
                        store.load_persisted_artifacts(&run_id, manifest.mode, &contract).ok()
                    })
                    .unwrap_or_default();
                let entries = store
                    .load_persisted_invocations(&run_id)?
                    .into_iter()
                    .map(|invocation| {
                        let linked_artifacts = artifacts
                            .iter()
                            .filter(|artifact| {
                                artifact.record.provenance.as_ref().is_some_and(|provenance| {
                                    provenance.request_ids.contains(&invocation.request.request_id)
                                })
                            })
                            .map(|artifact| artifact.record.relative_path.clone())
                            .collect::<Vec<_>>();
                        let approval_state = if invocation.decision.requires_approval {
                            if invocation.approvals.iter().any(|approval| {
                                matches!(approval.decision, ApprovalDecision::Approve)
                            }) {
                                "approved"
                            } else {
                                "pending"
                            }
                        } else {
                            "not-required"
                        };
                        InspectEntry::Invocation(InvocationInspectSummary {
                            request_id: invocation.request.request_id.clone(),
                            adapter: format!("{:?}", invocation.request.adapter),
                            capability: format!("{:?}", invocation.request.capability),
                            orientation: format!("{:?}", invocation.request.orientation),
                            policy_decision: format!("{:?}", invocation.decision.kind),
                            recommendation_only: invocation
                                .decision
                                .constraints
                                .recommendation_only,
                            approval_state: approval_state.to_string(),
                            latest_outcome: invocation
                                .attempts
                                .last()
                                .map(|attempt| format!("{:?}", attempt.outcome.kind)),
                            linked_artifacts,
                        })
                    })
                    .collect::<Vec<_>>();
                ("invocations".to_string(), system_context, entries)
            }
            InspectTarget::Evidence { run_id } => {
                let run_id = self.resolve_run(&run_id)?;
                let mode = store.load_run_manifest(&run_id)?.mode;
                let run_context = store.load_run_context(&run_id).ok();
                let approvals = store.load_approval_records(&run_id).unwrap_or_default();
                let system_context =
                    run_context.as_ref().and_then(|context| context.system_context);
                let upstream_context =
                    run_context.as_ref().and_then(|context| context.upstream_context.as_ref());
                let backlog_planning =
                    run_context.as_ref().and_then(|context| context.backlog_planning.as_ref());
                let entries = store
                    .load_evidence_bundle(&run_id)?
                    .map(|evidence| {
                        vec![InspectEntry::Evidence(EvidenceInspectSummary {
                            execution_posture: resolved_execution_posture_label_for_mode(
                                mode,
                                run_context.as_ref(),
                                &approvals,
                            ),
                            upstream_feature_slice: upstream_context
                                .and_then(|context| context.feature_slice.clone()),
                            primary_upstream_mode: upstream_context
                                .and_then(|context| context.primary_upstream_mode.clone()),
                            upstream_source_refs: upstream_context
                                .map(|context| context.source_refs.clone())
                                .unwrap_or_default(),
                            carried_forward_items: upstream_context
                                .map(|context| context.carried_forward_items.clone())
                                .unwrap_or_default(),
                            excluded_upstream_scope: upstream_context
                                .and_then(|context| context.excluded_upstream_scope.clone()),
                            closure_status: backlog_planning.map(|planning| {
                                planning.closure_assessment.status.as_str().to_string()
                            }),
                            decomposition_scope: backlog_planning.map(|planning| {
                                planning.closure_assessment.decomposition_scope.as_str().to_string()
                            }),
                            closure_findings: backlog_planning
                                .map(|planning| {
                                    planning
                                        .closure_assessment
                                        .findings
                                        .iter()
                                        .map(|finding| ClosureFindingInspectSummary {
                                            category: finding.category.clone(),
                                            severity: finding.severity.as_str().to_string(),
                                            affected_scope: finding.affected_scope.clone(),
                                            recommended_followup: finding
                                                .recommended_followup
                                                .clone(),
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default(),
                            closure_notes: backlog_planning
                                .and_then(|planning| planning.closure_assessment.notes.clone()),
                            generation_paths: evidence
                                .generation_paths
                                .into_iter()
                                .map(|path| path.path_id)
                                .collect(),
                            validation_paths: evidence
                                .validation_paths
                                .into_iter()
                                .map(|path| path.path_id)
                                .collect(),
                            denied_invocations: evidence
                                .denied_invocations
                                .into_iter()
                                .map(|denied| denied.request_id)
                                .collect(),
                            artifact_provenance_links: evidence.artifact_refs,
                        })]
                    })
                    .unwrap_or_default();
                ("evidence".to_string(), system_context, entries)
            }
        };

        Ok(InspectResponse {
            target: name,
            system_context: system_context.map(|context| context.as_str().to_string()),
            entries,
        })
    }

    pub(super) fn inspect_risk_zone(
        &self,
        mode: Mode,
        risk: Option<RiskClass>,
        zone: Option<UsageZone>,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<ClassificationInspectSummary, EngineError> {
        if inputs.is_empty() && inline_inputs.is_empty() {
            return Err(EngineError::Validation(format!(
                "risk-zone inspection requires at least one input for {}",
                mode.as_str()
            )));
        }

        if matches!(mode, Mode::PrReview) {
            if !inline_inputs.is_empty() {
                return Err(EngineError::Validation(
                    "risk-zone inspection for pr-review does not support --input-text".to_string(),
                ));
            }
            if inputs.len() < 2 {
                return Err(EngineError::Validation(
                    "risk-zone inspection for pr-review requires two refs or inputs".to_string(),
                ));
            }
        } else {
            self.validate_authored_inputs(mode, inputs, inline_inputs)?;
        }

        let intake_summary = if matches!(mode, Mode::PrReview) {
            self.load_input_summary(inputs, &[])?
        } else {
            self.read_requirements_context(inputs, inline_inputs)?
        };
        let repo_surfaces = self.scan_workspace_surface().unwrap_or_default();
        let inferred =
            classifier::infer_risk_zone(mode, risk, zone, &intake_summary, inputs, &repo_surfaces);

        Ok(ClassificationInspectSummary {
            mode: mode.as_str().to_string(),
            risk: inferred.risk.as_str().to_string(),
            zone: inferred.zone.as_str().to_string(),
            risk_was_supplied: inferred.risk_was_supplied,
            zone_was_supplied: inferred.zone_was_supplied,
            confidence: inferred.confidence.as_str().to_string(),
            requires_confirmation: inferred.requires_confirmation,
            headline: inferred.headline,
            rationale: inferred.rationale,
            risk_rationale: inferred.risk_rationale,
            zone_rationale: inferred.zone_rationale,
            signals: inferred.signals,
            risk_signals: inferred.risk_signals,
            zone_signals: inferred.zone_signals,
        })
    }

    pub(super) fn inspect_clarity(
        &self,
        mode: Mode,
        inputs: &[String],
    ) -> Result<ClarityInspectSummary, EngineError> {
        if inputs.is_empty() {
            return Err(EngineError::Validation(format!(
                "clarity inspection requires at least one input for {}",
                mode.as_str()
            )));
        }

        match mode {
            Mode::Requirements => self.inspect_requirements_clarity(inputs),
            Mode::Discovery => self.inspect_discovery_clarity(inputs),
            Mode::SupplyChainAnalysis => self.inspect_supply_chain_analysis_clarity(inputs),
            Mode::PrReview => Err(EngineError::UnsupportedInspectTarget(
                "clarity inspection is not available for pr-review because it uses diff-backed inputs rather than authored packet files".to_string(),
            )),
            other => self.inspect_authored_mode_clarity(other, inputs),
        }
    }

    pub(super) fn inspect_authored_mode_clarity(
        &self,
        mode: Mode,
        inputs: &[String],
    ) -> Result<ClarityInspectSummary, EngineError> {
        self.validate_authored_input_paths(mode, inputs)?;
        for input in inputs {
            let resolved = self.resolve_input_path(input);
            if !resolved.exists() {
                return Err(EngineError::Validation(format!(
                    "input `{input}` was not found from {}",
                    self.repo_root.display()
                )));
            }
        }

        let source_inputs = self.clarity_source_inputs(inputs)?;
        let context_summary = self.read_requirements_context(inputs, &[])?;
        let brief = AuthoredModeBrief::from_context(mode, context_summary, &source_inputs);
        let missing_context = authored_mode_missing_context(&brief);
        let clarification_questions = prioritized_authored_mode_clarification_questions(&brief);
        let reasoning_signals = authored_mode_reasoning_signals(&source_inputs, &brief);
        let requires_clarification =
            !missing_context.is_empty() || !clarification_questions.is_empty();
        let recommended_focus =
            authored_mode_recommended_focus(&brief, &missing_context, &clarification_questions);

        Ok(ClarityInspectSummary {
            mode: mode.as_str().to_string(),
            summary: brief.summary(),
            source_inputs,
            requires_clarification,
            missing_context,
            clarification_questions,
            reasoning_signals,
            recommended_focus,
        })
    }

    pub(super) fn inspect_requirements_clarity(
        &self,
        inputs: &[String],
    ) -> Result<ClarityInspectSummary, EngineError> {
        self.validate_authored_input_paths(Mode::Requirements, inputs)?;
        for input in inputs {
            let resolved = self.resolve_input_path(input);
            if !resolved.exists() {
                return Err(EngineError::Validation(format!(
                    "input `{input}` was not found from {}",
                    self.repo_root.display()
                )));
            }
        }

        let source_inputs = self.clarity_source_inputs(inputs)?;
        let context_summary = self.read_requirements_context(inputs, &[])?;
        let brief = RequirementsBrief::from_context(context_summary.clone(), &source_inputs);
        let missing_context = requirements_missing_context(&brief);
        let clarification_questions =
            prioritized_requirements_clarification_questions(&brief, &context_summary);
        let reasoning_signals = requirements_reasoning_signals(&source_inputs, &brief);
        let requires_clarification =
            !missing_context.is_empty() || !clarification_questions.is_empty();
        let recommended_focus = if !missing_context.is_empty() {
            "Resolve the missing context items before starting a requirements run or handing the packet to downstream design work.".to_string()
        } else if !clarification_questions.is_empty() {
            "Review the authored open questions with the named owner before selecting the next governed mode.".to_string()
        } else {
            "No critical clarification questions detected; the authored brief is bounded enough for requirements mode.".to_string()
        };

        Ok(ClarityInspectSummary {
            mode: Mode::Requirements.as_str().to_string(),
            summary: brief.summary(),
            source_inputs,
            requires_clarification,
            missing_context,
            clarification_questions,
            reasoning_signals,
            recommended_focus,
        })
    }

    pub(super) fn inspect_discovery_clarity(
        &self,
        inputs: &[String],
    ) -> Result<ClarityInspectSummary, EngineError> {
        self.validate_authored_input_paths(Mode::Discovery, inputs)?;
        for input in inputs {
            let resolved = self.resolve_input_path(input);
            if !resolved.exists() {
                return Err(EngineError::Validation(format!(
                    "input `{input}` was not found from {}",
                    self.repo_root.display()
                )));
            }
        }

        let source_inputs = self.clarity_source_inputs(inputs)?;
        let context_summary = self.read_requirements_context(inputs, &[])?;
        let repo_surfaces = self.scan_workspace_surface()?;
        let brief = DiscoveryBrief::from_context(context_summary, &repo_surfaces);
        let missing_context = discovery_missing_context(&brief);
        let clarification_questions = prioritized_discovery_clarification_questions(&brief);
        let reasoning_signals = discovery_reasoning_signals(&source_inputs, &repo_surfaces, &brief);
        let requires_clarification =
            !missing_context.is_empty() || !clarification_questions.is_empty();
        let recommended_focus = if !missing_context.is_empty() {
            "Resolve the missing discovery boundaries before translating this packet into requirements, architecture, or change planning.".to_string()
        } else if !clarification_questions.is_empty() {
            "Review the open discovery questions with the named owner before choosing the downstream handoff mode.".to_string()
        } else {
            "No critical clarification questions detected; discovery has enough explicit structure for downstream translation.".to_string()
        };

        Ok(ClarityInspectSummary {
            mode: Mode::Discovery.as_str().to_string(),
            summary: discovery_summary(&brief),
            source_inputs,
            requires_clarification,
            missing_context,
            clarification_questions,
            reasoning_signals,
            recommended_focus,
        })
    }

    pub(super) fn inspect_supply_chain_analysis_clarity(
        &self,
        inputs: &[String],
    ) -> Result<ClarityInspectSummary, EngineError> {
        self.validate_authored_input_paths(Mode::SupplyChainAnalysis, inputs)?;
        for input in inputs {
            let resolved = self.resolve_input_path(input);
            if !resolved.exists() {
                return Err(EngineError::Validation(format!(
                    "input `{input}` was not found from {}",
                    self.repo_root.display()
                )));
            }
        }

        let source_inputs = self.clarity_source_inputs(inputs)?;
        let context_summary = self.read_requirements_context(inputs, &[])?;
        let brief = SupplyChainAnalysisBrief::from_context(context_summary, &source_inputs);
        let missing_context = supply_chain_analysis_missing_context(&brief);
        let clarification_questions =
            prioritized_supply_chain_analysis_clarification_questions(&brief);
        let reasoning_signals = supply_chain_analysis_reasoning_signals(&source_inputs, &brief);
        let requires_clarification =
            !missing_context.is_empty() || !clarification_questions.is_empty();
        let recommended_focus = if !missing_context.is_empty() {
            "Resolve the missing licensing, distribution, ecosystem, and tool-policy decisions before treating the supply-chain packet as review-ready.".to_string()
        } else if !clarification_questions.is_empty() {
            "Review the missing posture and scanner-policy questions with the named owner before starting the governed run.".to_string()
        } else {
            "No critical clarification questions detected; the authored supply-chain brief is bounded enough for the governed run.".to_string()
        };

        Ok(ClarityInspectSummary {
            mode: Mode::SupplyChainAnalysis.as_str().to_string(),
            summary: brief.summary(),
            source_inputs,
            requires_clarification,
            missing_context,
            clarification_questions,
            reasoning_signals,
            recommended_focus,
        })
    }
}
