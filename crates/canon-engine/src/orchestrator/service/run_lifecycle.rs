use super::EngineService;
use super::*;
use crate::domain::run::RunLineageLink;

impl EngineService {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn apply_refinement_mode_correction(
        &self,
        store: &WorkspaceStore,
        run_id: &str,
        target_mode: Mode,
        mode_change_reason: &str,
    ) -> Result<String, EngineError> {
        store.init_runtime_state(None)?;
        let manifest = store.load_run_manifest(run_id)?;
        if manifest.mode == target_mode {
            return Ok(run_id.to_string());
        }

        let context = store.load_run_context(run_id)?;
        let state = store.load_run_state(run_id)?;
        let policy_set = store.load_policy_set(None)?;
        let mut artifact_contract = contract_for_mode(target_mode);
        classifier::apply_verification_layers(&policy_set, manifest.risk, &mut artifact_contract);

        if state.state == RunState::Draft {
            let updated_context =
                self.retarget_refinement_context(&context, run_id, target_mode)?;
            let updated_manifest = RunManifest { mode: target_mode, lineage: None, ..manifest };
            store.persist_run_bundle(&PersistedRunBundle {
                run: updated_manifest,
                context: updated_context,
                state,
                artifact_contract,
                artifacts: Vec::new(),
                links: LinkManifest {
                    artifacts: Vec::new(),
                    decisions: Vec::new(),
                    traces: Vec::new(),
                    invocations: Vec::new(),
                    evidence: None,
                },
                gates: Vec::new(),
                approvals: Vec::new(),
                verification_records: Vec::new(),
                evidence: None,
                invocations: Vec::new(),
            })?;
            return Ok(run_id.to_string());
        }

        let successor_identity = self.next_unique_run_identity(store)?;
        let successor_context =
            self.retarget_refinement_context(&context, &successor_identity.run_id, target_mode)?;
        let mut successor_manifest = RunManifest::from_identity(
            &successor_identity,
            target_mode,
            manifest.risk,
            manifest.zone,
            manifest.system_context,
            manifest.classification.clone(),
            manifest.owner.clone(),
        );
        successor_manifest.lineage = Some(RunLineageLink {
            carried_from: run_id.to_string(),
            supersedes: run_id.to_string(),
            mode_change_reason: mode_change_reason.to_string(),
        });

        store.persist_run_bundle(&PersistedRunBundle {
            run: successor_manifest,
            context: successor_context,
            state: RunStateManifest {
                state: RunState::Draft,
                updated_at: successor_identity.created_at,
            },
            artifact_contract,
            artifacts: Vec::new(),
            links: LinkManifest {
                artifacts: Vec::new(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: None,
            },
            gates: Vec::new(),
            approvals: Vec::new(),
            verification_records: Vec::new(),
            evidence: None,
            invocations: Vec::new(),
        })?;

        Ok(successor_identity.run_id)
    }

    pub(super) fn refresh_run_state(
        &self,
        store: &WorkspaceStore,
        manifest: &RunManifest,
        context: &RunContext,
        contract: &crate::domain::artifact::ArtifactContract,
        artifacts: &[PersistedArtifact],
        approvals: &[ApprovalRecord],
    ) -> Result<RunState, EngineError> {
        let evidence_bundle = store.load_evidence_bundle(&manifest.run_id)?;
        let evidence_complete = evidence_bundle.is_some();
        let validation_independence_satisfied = evidence_bundle
            .as_ref()
            .map(|bundle| bundle.validation_paths.iter().all(|path| path.independence.sufficient))
            .unwrap_or(true);

        let artifact_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();

        let gates = match manifest.mode {
            Mode::Discovery => gatekeeper::evaluate_discovery_gates(
                contract,
                &artifact_inputs,
                gatekeeper::DiscoveryGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::SystemShaping => gatekeeper::evaluate_system_shaping_gates(
                contract,
                &artifact_inputs,
                gatekeeper::SystemShapingGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    evidence_complete,
                },
            ),
            Mode::Change => gatekeeper::evaluate_change_gates(
                contract,
                &artifact_inputs,
                gatekeeper::ChangeGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    system_context: manifest.system_context,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::Incident => gatekeeper::evaluate_incident_gates(
                contract,
                &artifact_inputs,
                gatekeeper::IncidentGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::SystemAssessment => gatekeeper::evaluate_system_assessment_gates(
                contract,
                &artifact_inputs,
                gatekeeper::SystemAssessmentGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::SecurityAssessment => gatekeeper::evaluate_security_assessment_gates(
                contract,
                &artifact_inputs,
                gatekeeper::SecurityAssessmentGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::DomainLanguage => gatekeeper::evaluate_domain_language_gates(
                contract,
                &artifact_inputs,
                gatekeeper::DomainLanguageGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::DomainModel => gatekeeper::evaluate_domain_model_gates(
                contract,
                &artifact_inputs,
                gatekeeper::DomainModelGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::SupplyChainAnalysis => gatekeeper::evaluate_supply_chain_analysis_gates(
                contract,
                &artifact_inputs,
                gatekeeper::SupplyChainAnalysisGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::Implementation => gatekeeper::evaluate_implementation_gates(
                contract,
                &artifact_inputs,
                gatekeeper::ImplementationGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    system_context: manifest.system_context,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::Migration => gatekeeper::evaluate_migration_gates(
                contract,
                &artifact_inputs,
                gatekeeper::MigrationGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::Refactor => gatekeeper::evaluate_refactor_gates(
                contract,
                &artifact_inputs,
                gatekeeper::RefactorGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    system_context: manifest.system_context,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::Review => gatekeeper::evaluate_review_gates(
                contract,
                &artifact_inputs,
                gatekeeper::ReviewGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    evidence_complete,
                },
            ),
            Mode::Verification => gatekeeper::evaluate_verification_gates(
                contract,
                &artifact_inputs,
                gatekeeper::VerificationGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::Architecture => gatekeeper::evaluate_architecture_gates(
                contract,
                &artifact_inputs,
                gatekeeper::ArchitectureGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    evidence_complete,
                },
            ),
            Mode::PrReview => gatekeeper::evaluate_pr_review_gates(
                contract,
                &artifact_inputs,
                gatekeeper::PrReviewGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    denied_invocations: evidence_bundle
                        .as_ref()
                        .map(|bundle| bundle.denied_invocations.as_slice())
                        .unwrap_or(&[]),
                    evidence_complete,
                },
            ),
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };
        let mut state = run_state_from_gates(&gates);
        if matches!(manifest.mode, Mode::Implementation | Mode::Refactor)
            && execution_continuation_pending(context, approvals)
            && !matches!(state, RunState::Blocked)
        {
            state = RunState::AwaitingApproval;
        }
        let state_manifest = RunStateManifest { state, updated_at: OffsetDateTime::now_utc() };
        store.persist_gate_evaluations(&manifest.run_id, &gates)?;
        store.persist_run_state(&manifest.run_id, &state_manifest)?;
        Ok(state)
    }

    pub(super) fn requirements_request(
        &self,
        spec: RequirementsRequestSpec<'_>,
    ) -> InvocationRequest {
        let adapter = match spec.capability {
            CapabilityKind::GenerateContent
            | CapabilityKind::CritiqueContent
            | CapabilityKind::ProposeWorkspaceEdit => canon_adapters::AdapterKind::CopilotCli,
            _ => canon_adapters::AdapterKind::Filesystem,
        };

        self.governed_request(GovernedRequestSpec {
            run_id: spec.run_id,
            mode: Mode::Requirements,
            risk: spec.risk,
            zone: spec.zone,
            system_context: spec.system_context,
            owner: spec.owner,
            adapter,
            capability: spec.capability,
            summary: spec.summary,
            scope: spec.scope,
        })
    }

    pub(super) fn governed_request(&self, spec: GovernedRequestSpec<'_>) -> InvocationRequest {
        let capability_profile = classify_capability(spec.adapter, spec.capability);

        InvocationRequest {
            request_id: format!("{}-{}", spec.run_id, capability_tag(spec.capability)),
            run_id: spec.run_id.to_string(),
            mode: spec.mode.as_str().to_string(),
            system_context: spec.system_context,
            risk: spec.risk,
            zone: spec.zone,
            adapter: spec.adapter,
            capability: spec.capability,
            orientation: capability_profile.orientation,
            mutability: capability_profile.mutability,
            trust_boundary: capability_profile.trust_boundary,
            lineage: capability_profile.lineage,
            requested_scope: spec.scope,
            owner: Some(spec.owner.to_string()),
            summary: spec.summary.to_string(),
            requested_at: OffsetDateTime::now_utc(),
        }
    }

    pub(super) fn read_requirements_context(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<String, EngineError> {
        let filesystem = FilesystemAdapter;
        let mut fragments = Vec::new();
        let include_input_labels = inputs.len() + inline_inputs.len() > 1;

        self.append_file_input_fragments(
            &filesystem,
            inputs,
            include_input_labels,
            &mut fragments,
        )?;
        self.append_inline_input_fragments(
            inputs,
            inline_inputs,
            include_input_labels,
            &mut fragments,
        );

        let normalized = preserve_multiline_summary(&fragments.join("\n"));
        if normalized.is_empty() {
            Err(EngineError::Validation(
                "authored input contained no usable content after normalization".to_string(),
            ))
        } else {
            Ok(normalized)
        }
    }

    fn append_file_input_fragments(
        &self,
        filesystem: &FilesystemAdapter,
        inputs: &[String],
        include_input_labels: bool,
        fragments: &mut Vec<String>,
    ) -> Result<(), EngineError> {
        for input in inputs {
            let resolved = self.resolve_input_path(input);
            let files = self.collect_content_input_files(input)?;
            if files.is_empty() {
                fragments.push(input.clone());
                continue;
            }

            let include_labels = resolved.is_dir() || files.len() > 1 || include_input_labels;
            for path in files {
                let (contents, _) = filesystem
                    .read_to_string_traced(&path, "capture requirements context")
                    .map_err(|error| EngineError::Validation(error.to_string()))?;
                if include_labels {
                    fragments.push(format!(
                        "## Input: {}\n\n{}",
                        self.persisted_input_path(&path),
                        contents
                    ));
                } else {
                    fragments.push(contents);
                }
            }
        }

        Ok(())
    }

    fn append_inline_input_fragments(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
        include_input_labels: bool,
        fragments: &mut Vec<String>,
    ) {
        for (index, inline_input) in inline_inputs.iter().enumerate() {
            if include_input_labels || !inputs.is_empty() {
                fragments.push(format!(
                    "## Input: {}\n\n{}",
                    inline_input_label(index),
                    inline_input
                ));
            } else {
                fragments.push(inline_input.clone());
            }
        }
    }

    pub(super) fn change_validation_attempt(
        &self,
        request: &InvocationRequest,
    ) -> Result<(String, InvocationAttempt), EngineError> {
        let shell = ShellAdapter;
        let adapter_request = shell.validation_request(&request.summary);
        let command =
            shell.run(&adapter_request, "git", &["ls-files"], Some(&self.repo_root), false);

        let (summary, outcome, executor) = match command {
            Ok(output) if output.status_code == 0 => {
                let files = output
                    .stdout
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .take(8)
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                let summary = if files.is_empty() {
                    "Validation tool confirmed the repository is empty but reachable.".to_string()
                } else {
                    format!(
                        "Validation tool reviewed tracked repository surfaces: {}",
                        files.join(", ")
                    )
                };
                (
                    summary.clone(),
                    ToolOutcome {
                        kind: ToolOutcomeKind::Succeeded,
                        summary,
                        exit_code: Some(0),
                        payload_refs: Vec::new(),
                        candidate_artifacts: Vec::new(),
                        recorded_at: OffsetDateTime::now_utc(),
                    },
                    "shell:git-ls-files".to_string(),
                )
            }
            Ok(output) => {
                let fallback = self.scan_workspace_surface()?;
                let summary = format!(
                    "Validation fell back to local workspace scan after git returned {}: {} | Fallback surfaces: {}",
                    output.status_code,
                    output.stderr.trim(),
                    fallback.join(", ")
                );
                (
                    summary.clone(),
                    ToolOutcome {
                        kind: ToolOutcomeKind::PartiallySucceeded,
                        summary,
                        exit_code: output.status_code.into(),
                        payload_refs: Vec::new(),
                        candidate_artifacts: Vec::new(),
                        recorded_at: OffsetDateTime::now_utc(),
                    },
                    "validation-fallback".to_string(),
                )
            }
            Err(_) => {
                let fallback = self.scan_workspace_surface()?;
                let summary = format!(
                    "Validation used a bounded workspace scan because git repository inspection was unavailable: {}",
                    fallback.join(", ")
                );
                (
                    summary.clone(),
                    ToolOutcome {
                        kind: ToolOutcomeKind::Succeeded,
                        summary,
                        exit_code: Some(0),
                        payload_refs: Vec::new(),
                        candidate_artifacts: Vec::new(),
                        recorded_at: OffsetDateTime::now_utc(),
                    },
                    "validation-fallback".to_string(),
                )
            }
        };

        Ok((summary, self.completed_attempt(request, 1, &executor, outcome)))
    }

    pub(super) fn locate_authored_mutation_patch(
        &self,
        inputs: &[String],
        allowed_paths: &[String],
    ) -> Result<Option<AuthoredMutationPatch>, EngineError> {
        let mut discovered = Vec::new();

        for input in inputs {
            for candidate in mutation_payload_candidates_for(&self.resolve_input_path(input)) {
                if !candidate.is_file() {
                    continue;
                }

                let canonical = candidate.canonicalize()?;
                if !discovered.iter().any(|existing: &PathBuf| existing == &canonical) {
                    discovered.push(canonical);
                }
            }
        }

        if discovered.is_empty() {
            return Ok(None);
        }

        if discovered.len() > 1 {
            let paths = discovered
                .iter()
                .map(|path| self.persisted_input_path(path))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(EngineError::Validation(format!(
                "multiple bounded mutation payloads were found; keep exactly one patch payload in the packet: {paths}"
            )));
        }

        let absolute_path = discovered.pop().ok_or_else(|| {
            EngineError::Validation(
                "expected exactly one bounded mutation payload after preflight selection"
                    .to_string(),
            )
        })?;
        let relative_path = self.persisted_input_path(&absolute_path);
        let patch = std::fs::read_to_string(&absolute_path)?;
        let changed_paths = parse_unified_diff_paths(&patch)?;
        let out_of_bounds = changed_paths
            .iter()
            .filter(|path| !path_within_allowed_scope(path, allowed_paths))
            .cloned()
            .collect::<Vec<_>>();
        if !out_of_bounds.is_empty() {
            return Err(EngineError::Validation(format!(
                "bounded mutation payload `{relative_path}` touches paths outside Allowed Paths: {}; declared allowed paths: {}",
                out_of_bounds.join(", "),
                allowed_paths.join(", ")
            )));
        }

        Ok(Some(AuthoredMutationPatch { absolute_path, relative_path, changed_paths }))
    }

    pub(super) fn apply_authored_mutation_patch(
        &self,
        request: &InvocationRequest,
        patch: &AuthoredMutationPatch,
    ) -> Result<InvocationAttempt, EngineError> {
        let shell = ShellAdapter;
        let adapter_request = shell.mutating_request(&request.summary);
        let patch_arg = patch.absolute_path.to_string_lossy().into_owned();

        let check_args = ["apply", "--check", "--whitespace=nowarn", patch_arg.as_str()];
        let check_output = shell
            .run(&adapter_request, "git", &check_args, Some(&self.repo_root), true)
            .map_err(|error| {
                EngineError::Validation(format!(
                    "failed to preflight bounded mutation payload `{}`: {error}",
                    patch.relative_path
                ))
            })?;
        if check_output.status_code != 0 {
            return Err(EngineError::Validation(format!(
                "bounded mutation payload `{}` failed git apply --check with exit code {}: {}",
                patch.relative_path,
                check_output.status_code,
                process_failure_excerpt(&check_output.stdout, &check_output.stderr)
            )));
        }

        let apply_args = ["apply", "--whitespace=nowarn", patch_arg.as_str()];
        let apply_output = shell
            .run(&adapter_request, "git", &apply_args, Some(&self.repo_root), true)
            .map_err(|error| {
                EngineError::Validation(format!(
                    "failed to apply bounded mutation payload `{}`: {error}",
                    patch.relative_path
                ))
            })?;
        if apply_output.status_code != 0 {
            return Err(EngineError::Validation(format!(
                "bounded mutation payload `{}` failed git apply with exit code {}: {}",
                patch.relative_path,
                apply_output.status_code,
                process_failure_excerpt(&apply_output.stdout, &apply_output.stderr)
            )));
        }

        let summary = format!(
            "Applied authored bounded patch {} within allowed paths: {}",
            patch.relative_path,
            patch.changed_paths.join(", ")
        );

        Ok(self.completed_attempt(
            request,
            1,
            "shell:git-apply",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary,
                exit_code: Some(0),
                payload_refs: vec![crate::domain::execution::PayloadReference {
                    path: patch.relative_path.clone(),
                    digest: None,
                }],
                candidate_artifacts: patch.changed_paths.clone(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        ))
    }

    pub(super) fn scan_workspace_surface(&self) -> Result<Vec<String>, EngineError> {
        let mut collected = Vec::new();
        let mut stack = vec![self.repo_root.clone()];

        while let Some(path) = stack.pop() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();

                if is_special_repository_directory(&name) {
                    continue;
                }

                if entry_path.is_dir() {
                    stack.push(entry_path);
                    continue;
                }

                if let Ok(relative) = entry_path.strip_prefix(&self.repo_root) {
                    collected.push(relative.display().to_string());
                }
            }
        }

        collected.sort();
        collected.truncate(8);
        if collected.is_empty() {
            collected.push("no-repository-surfaces-detected".to_string());
        }

        Ok(collected)
    }

    pub(super) fn completed_attempt(
        &self,
        request: &InvocationRequest,
        attempt_number: u32,
        executor: &str,
        outcome: ToolOutcome,
    ) -> InvocationAttempt {
        InvocationAttempt {
            request_id: request.request_id.clone(),
            attempt_number,
            started_at: OffsetDateTime::now_utc(),
            finished_at: OffsetDateTime::now_utc(),
            executor: executor.to_string(),
            outcome,
        }
    }

    pub(super) fn policy_decision_attempt(
        &self,
        request: &InvocationRequest,
        decision: &crate::domain::execution::InvocationPolicyDecision,
    ) -> InvocationAttempt {
        let outcome_kind = if decision.constraints.recommendation_only {
            ToolOutcomeKind::RecommendationOnly
        } else {
            match decision.kind {
                PolicyDecisionKind::NeedsApproval => ToolOutcomeKind::AwaitingApproval,
                PolicyDecisionKind::Deny => ToolOutcomeKind::Denied,
                PolicyDecisionKind::Allow | PolicyDecisionKind::AllowConstrained => {
                    ToolOutcomeKind::PartiallySucceeded
                }
            }
        };

        InvocationAttempt {
            request_id: request.request_id.clone(),
            attempt_number: 1,
            started_at: decision.decided_at,
            finished_at: decision.decided_at,
            executor: "policy".to_string(),
            outcome: ToolOutcome {
                kind: outcome_kind,
                summary: decision.rationale.clone(),
                exit_code: None,
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: decision.decided_at,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use canon_adapters::AdapterKind;
    use std::process::Command;
    use tempfile::tempdir;
    use time::OffsetDateTime;

    use super::*;
    use crate::domain::execution::{
        InvocationConstraintSet, InvocationPolicyDecision, PolicyDecisionKind, ToolOutcome,
        ToolOutcomeKind,
    };

    fn sample_request(service: &EngineService) -> InvocationRequest {
        service.governed_request(GovernedRequestSpec {
            run_id: "R-20260529-test",
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            owner: "Owner <owner@example.com>",
            adapter: AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "Inspect repo",
            scope: vec!["src".to_string()],
        })
    }

    fn sample_patch_body(path: &str) -> String {
        format!(
            "diff --git a/{path} b/{path}\n--- a/{path}\n+++ b/{path}\n@@ -1 +1 @@\n-old\n+new\n"
        )
    }

    fn requirements_run_request() -> RunRequest {
        RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: Vec::new(),
            inline_inputs: vec![
                "# Requirements Brief\n\n## Problem\nBound the problem.\n\n## Outcome\nShip a bounded packet.\n"
                    .to_string(),
            ],
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    #[test]
    fn requirements_request_maps_generation_to_copilot_and_reads_to_filesystem() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());

        let generation = service.requirements_request(RequirementsRequestSpec {
            run_id: "R-20260529-req",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            owner: "Owner <owner@example.com>",
            capability: CapabilityKind::GenerateContent,
            summary: "Generate requirements",
            scope: vec!["requirements".to_string()],
        });
        assert_eq!(generation.adapter, AdapterKind::CopilotCli);

        let context = service.requirements_request(RequirementsRequestSpec {
            run_id: "R-20260529-req",
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            owner: "Owner <owner@example.com>",
            capability: CapabilityKind::ReadRepository,
            summary: "Read requirements context",
            scope: vec!["canon-input".to_string()],
        });
        assert_eq!(context.adapter, AdapterKind::Filesystem);
        assert_eq!(context.mode, "requirements");
    }

    #[test]
    fn apply_refinement_mode_correction_returns_same_run_id_when_mode_is_unchanged() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let store = WorkspaceStore::new(workspace.path());
        let summary = service.run(requirements_run_request()).expect("requirements run");

        let corrected = service
            .apply_refinement_mode_correction(
                &store,
                &summary.run_id,
                Mode::Requirements,
                "mode already aligned",
            )
            .expect("same mode correction should succeed");

        assert_eq!(corrected, summary.run_id);
    }

    #[test]
    fn read_requirements_context_labels_multiple_sources_and_rejects_empty_content() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let packet_dir = workspace.path().join("canon-input");
        std::fs::create_dir_all(&packet_dir).expect("create canon-input");
        std::fs::write(packet_dir.join("brief.md"), "# Brief\nProblem\n").expect("write brief");

        let combined = service
            .read_requirements_context(
                &["canon-input/brief.md".to_string()],
                &["Inline support".to_string()],
            )
            .expect("combined context");
        assert!(combined.contains("## Input: canon-input/brief.md"));
        assert!(combined.contains("## Input: inline-input-01.md"));

        let empty = service
            .read_requirements_context(&[], &["   \n\n".to_string()])
            .expect_err("blank inline content should fail");
        assert!(
            empty
                .to_string()
                .contains("authored input contained no usable content after normalization")
        );
    }

    #[test]
    fn locate_authored_mutation_patch_handles_none_multiple_out_of_bounds_and_success() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let packet_dir = workspace.path().join("packet");
        std::fs::create_dir_all(&packet_dir).expect("create packet dir");
        std::fs::write(packet_dir.join("brief.md"), "# Brief\n").expect("write brief");

        assert!(
            service
                .locate_authored_mutation_patch(
                    &["packet".to_string()],
                    &["src/lib.rs".to_string()]
                )
                .expect("no patch should succeed")
                .is_none()
        );

        std::fs::write(packet_dir.join("bounded.diff"), sample_patch_body("src/lib.rs"))
            .expect("write bounded diff");
        std::fs::write(packet_dir.join("patch.diff"), sample_patch_body("src/main.rs"))
            .expect("write second diff");
        let multiple = service
            .locate_authored_mutation_patch(&["packet".to_string()], &["src/lib.rs".to_string()])
            .expect_err("multiple payloads should fail");
        assert!(multiple.to_string().contains("multiple bounded mutation payloads were found"));

        std::fs::remove_file(packet_dir.join("patch.diff")).expect("remove second diff");
        let out_of_bounds = service
            .locate_authored_mutation_patch(
                &["packet".to_string()],
                &["src/allowed.rs".to_string()],
            )
            .expect_err("out of bounds patch should fail");
        assert!(out_of_bounds.to_string().contains("touches paths outside Allowed Paths"));

        let patch = service
            .locate_authored_mutation_patch(&["packet".to_string()], &["src/lib.rs".to_string()])
            .expect("bounded patch should parse")
            .expect("expected patch payload");
        assert!(patch.relative_path.ends_with("packet/bounded.diff"));
        assert_eq!(patch.changed_paths, vec!["src/lib.rs".to_string()]);
    }

    #[test]
    fn scan_workspace_surface_ignores_special_dirs_and_reports_empty_repos() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());

        assert_eq!(
            service.scan_workspace_surface().expect("empty surface scan"),
            vec!["no-repository-surfaces-detected".to_string()]
        );

        std::fs::create_dir_all(workspace.path().join("src")).expect("create src dir");
        std::fs::create_dir_all(workspace.path().join(".git")).expect("create git dir");
        std::fs::write(workspace.path().join("src/lib.rs"), "pub fn x() {}\n")
            .expect("write lib.rs");
        std::fs::write(workspace.path().join(".git/HEAD"), "ref: refs/heads/main\n")
            .expect("write git head");

        let surfaces = service.scan_workspace_surface().expect("surface scan");
        assert!(surfaces.contains(&"src/lib.rs".to_string()));
        assert!(!surfaces.iter().any(|path| path.starts_with(".git/")));
    }

    #[test]
    fn change_validation_attempt_reports_empty_but_reachable_repository() {
        let workspace = tempdir().expect("tempdir");
        let init_status = Command::new("git")
            .arg("init")
            .current_dir(workspace.path())
            .status()
            .expect("git init status");
        assert!(init_status.success());

        let service = EngineService::new(workspace.path());
        let request = sample_request(&service);
        let (summary, attempt) =
            service.change_validation_attempt(&request).expect("validation attempt should succeed");

        assert!(summary.contains("repository is empty but reachable"));
        assert_eq!(attempt.executor, "shell:git-ls-files");
        assert_eq!(attempt.outcome.kind, ToolOutcomeKind::Succeeded);
        assert_eq!(attempt.outcome.exit_code, Some(0));
    }

    #[test]
    fn completed_and_policy_attempts_preserve_request_identity_and_outcome_kind() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let request = sample_request(&service);

        let completed = service.completed_attempt(
            &request,
            2,
            "shell:test",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: "done".to_string(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );
        assert_eq!(completed.request_id, request.request_id);
        assert_eq!(completed.attempt_number, 2);
        assert_eq!(completed.executor, "shell:test");

        let recommendation = InvocationPolicyDecision {
            kind: PolicyDecisionKind::AllowConstrained,
            constraints: InvocationConstraintSet {
                recommendation_only: true,
                ..InvocationConstraintSet::default()
            },
            requires_approval: false,
            rationale: "bounded".to_string(),
            policy_refs: vec!["policy-1".to_string()],
            decided_at: OffsetDateTime::now_utc(),
        };
        let recommendation_attempt = service.policy_decision_attempt(&request, &recommendation);
        assert_eq!(recommendation_attempt.outcome.kind, ToolOutcomeKind::RecommendationOnly);

        let denied = InvocationPolicyDecision {
            kind: PolicyDecisionKind::Deny,
            constraints: InvocationConstraintSet::default(),
            requires_approval: false,
            rationale: "denied".to_string(),
            policy_refs: vec!["policy-2".to_string()],
            decided_at: OffsetDateTime::now_utc(),
        };
        let denied_attempt = service.policy_decision_attempt(&request, &denied);
        assert_eq!(denied_attempt.outcome.kind, ToolOutcomeKind::Denied);

        let approval = InvocationPolicyDecision {
            kind: PolicyDecisionKind::NeedsApproval,
            constraints: InvocationConstraintSet::default(),
            requires_approval: true,
            rationale: "needs approval".to_string(),
            policy_refs: vec!["policy-3".to_string()],
            decided_at: OffsetDateTime::now_utc(),
        };
        let approval_attempt = service.policy_decision_attempt(&request, &approval);
        assert_eq!(approval_attempt.outcome.kind, ToolOutcomeKind::AwaitingApproval);
    }
}
