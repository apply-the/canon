use super::EngineService;
use super::*;
use crate::domain::run::{
    ClarificationRefinementStatus, ContinuationCandidateSummary, RefinementWorkflowFamily,
};
use uuid::{ContextV7, Timestamp, Uuid};

const REFINEMENT_WORKING_BRIEF_FILE_NAME: &str = "working-brief.md";
const REFINEMENT_TEMPLATE_ROOT: &str = "defaults/templates/canon-input";
const REFINEMENT_MATCH_REASON_SAME_INPUTS: &str = "same authoritative input fingerprint";
const RUN_ID_COLLISION_RETRY_LIMIT: usize = 8;
const RUN_ID_COLLISION_WINDOW_MILLIS: i64 = 65_536;

struct RefinementSeedState {
    source_inputs: Vec<String>,
    missing_context: Vec<String>,
    clarification_questions: Vec<ClarificationQuestionSummary>,
}

impl EngineService {
    pub(super) fn map_init_summary(summary: StoreInitSummary) -> InitSummary {
        InitSummary {
            repo_root: summary.repo_root,
            canon_root: summary.canon_root,
            methods_materialized: summary.methods_materialized,
            policies_materialized: summary.policies_materialized,
            skills_materialized: summary.skills_materialized,
            claude_md_created: summary.claude_md_created,
        }
    }

    pub(super) fn map_skills_summary(summary: StoreSkillsSummary) -> SkillsSummary {
        SkillsSummary {
            skills_dir: summary.skills_dir,
            skills_materialized: summary.skills_materialized,
            skills_skipped: summary.skills_skipped,
            claude_md_created: summary.claude_md_created,
        }
    }
    pub(super) fn authored_input_name(path: &str) -> Option<&str> {
        Path::new(path).file_name().and_then(|name| name.to_str())
    }

    pub(super) fn resolve_approver(&self, explicit_approver: &str) -> String {
        self.resolve_identity(explicit_approver)
    }

    pub(super) fn resolve_owner(&self, explicit_owner: &str) -> String {
        self.resolve_identity(explicit_owner)
    }

    pub(super) fn resolve_identity(&self, explicit_identity: &str) -> String {
        let explicit_identity = explicit_identity.trim();
        if !explicit_identity.is_empty() {
            return explicit_identity.to_string();
        }

        self.resolve_git_owner(GitConfigScope::Local)
            .or_else(|| self.resolve_git_owner(GitConfigScope::Global))
            .unwrap_or_default()
    }

    pub(super) fn resolve_git_owner(&self, scope: GitConfigScope) -> Option<String> {
        let name = self.git_config_value(scope, "user.name");
        let email = self.git_config_value(scope, "user.email");

        match (name, email) {
            (Some(name), Some(email)) => Some(format!("{name} <{email}>")),
            (Some(name), None) => Some(name),
            (None, Some(email)) => Some(email),
            (None, None) => None,
        }
    }

    pub(super) fn git_config_value(&self, scope: GitConfigScope, key: &str) -> Option<String> {
        let shell = ShellAdapter;
        let request = shell.read_only_request("resolve owner identity from git config");
        let scope_arg = match scope {
            GitConfigScope::Local => "--local",
            GitConfigScope::Global => "--global",
        };

        let output = shell
            .run(
                &request,
                "git",
                &["config", scope_arg, "--get", key],
                Some(&self.repo_root),
                false,
            )
            .ok()?;
        let value = output.stdout.trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    }

    pub(super) fn is_targeted_refinement_mode(mode: Mode) -> bool {
        matches!(
            mode,
            Mode::Requirements
                | Mode::Discovery
                | Mode::Brainstorming
                | Mode::SystemShaping
                | Mode::Architecture
                | Mode::Change
                | Mode::PolicyShaping
        )
    }

    pub(super) fn start_refinement_draft(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: &crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = self.next_unique_run_identity(store)?;
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(policy_set, request.risk, &mut artifact_contract);
        let working_brief_seed =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let suggested_candidate =
            self.find_refinement_candidate(store, request.mode, &input_fingerprints)?;
        let mut context = self.build_run_context(&request, input_fingerprints, identity.created_at);
        context.clarification_refinement = Some(self.build_refinement_context(
            &identity.run_id,
            request.mode,
            &request,
            suggested_candidate,
            identity.created_at,
        )?);

        let bundle = PersistedRunBundle {
            run: RunManifest::from_identity(
                &identity,
                request.mode,
                request.risk,
                request.zone,
                request.system_context,
                request.classification.clone(),
                request.owner.clone(),
            ),
            context,
            state: RunStateManifest { state: RunState::Draft, updated_at: identity.created_at },
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
        };
        store.persist_run_bundle(&bundle)?;

        let Some(refinement) = bundle.context.clarification_refinement.as_ref() else {
            return Err(EngineError::Validation(
                "targeted refinement draft is missing refinement context".to_string(),
            ));
        };
        let working_brief =
            crate::artifacts::render_refinement_working_brief(&working_brief_seed, refinement);
        store.persist_refinement_working_brief(&identity.run_id, request.mode, &working_brief)?;

        self.summarize_run(
            store,
            RunSummarySpec {
                run_id: &identity.run_id,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                state: RunState::Draft,
                artifact_count: 0,
            },
        )
    }

    pub(super) fn build_refinement_context(
        &self,
        run_id: &str,
        mode: Mode,
        request: &RunRequest,
        suggested_candidate: Option<ContinuationCandidateSummary>,
        recorded_at: OffsetDateTime,
    ) -> Result<ClarificationRefinementContext, EngineError> {
        let refinement_seed = self.collect_refinement_seed_state(mode, request)?;
        let authoring_lifecycle = self.build_authoring_lifecycle_summary(
            &request.inputs,
            &refinement_seed.source_inputs,
            &refinement_seed.missing_context,
            &refinement_seed.clarification_questions,
            false,
        );

        Ok(ClarificationRefinementContext {
            workflow_family: self.refinement_workflow_family(mode),
            current_mode: mode,
            working_brief_path: self.refinement_working_brief_path(run_id, mode),
            template_ref: Self::refinement_template_ref(mode),
            status: ClarificationRefinementStatus::Active,
            explicit_continuation_required: true,
            authoritative_input_refs: authoring_lifecycle.authoritative_inputs.clone(),
            supporting_input_refs: authoring_lifecycle.supporting_inputs.clone(),
            suggested_candidate,
            records: Self::build_refinement_clarification_records(
                &refinement_seed.clarification_questions,
                recorded_at,
            ),
            readiness_delta: Self::build_structured_refinement_readiness_items(
                &authoring_lifecycle,
                &refinement_seed.missing_context,
                &refinement_seed.clarification_questions,
            ),
        })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn retarget_refinement_context(
        &self,
        context: &RunContext,
        run_id: &str,
        target_mode: Mode,
    ) -> Result<RunContext, EngineError> {
        let mut updated = context.clone();
        let Some(refinement) = updated.clarification_refinement.as_mut() else {
            return Err(EngineError::Validation(format!(
                "run `{run_id}` has no refinement context; cannot change mode"
            )));
        };

        refinement.current_mode = target_mode;
        refinement.working_brief_path = self.refinement_working_brief_path(run_id, target_mode);
        refinement.template_ref = Self::refinement_template_ref(target_mode);
        refinement.status = ClarificationRefinementStatus::Active;
        refinement.explicit_continuation_required = true;
        refinement.suggested_candidate = None;

        Ok(updated)
    }

    fn refinement_workflow_family(&self, mode: Mode) -> RefinementWorkflowFamily {
        match mode {
            Mode::Requirements
            | Mode::Discovery
            | Mode::Brainstorming
            | Mode::SystemShaping
            | Mode::Architecture
            | Mode::Change
            | Mode::Backlog
            | Mode::DomainLanguage
            | Mode::PolicyShaping
            | Mode::DomainModel => RefinementWorkflowFamily::Planning,
            Mode::Implementation | Mode::Refactor | Mode::Migration | Mode::Debugging => {
                RefinementWorkflowFamily::Execution
            }
            Mode::Incident
            | Mode::Review
            | Mode::Verification
            | Mode::PrReview
            | Mode::SystemAssessment
            | Mode::SecurityAssessment
            | Mode::SupplyChainAnalysis => RefinementWorkflowFamily::Assessment,
        }
    }

    fn collect_refinement_seed_state(
        &self,
        mode: Mode,
        request: &RunRequest,
    ) -> Result<RefinementSeedState, EngineError> {
        let source_inputs =
            self.refinement_source_inputs(&request.inputs, &request.inline_inputs)?;
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;

        match mode {
            Mode::Requirements => {
                let brief =
                    RequirementsBrief::from_context(context_summary.clone(), &source_inputs);
                Ok(RefinementSeedState {
                    source_inputs,
                    missing_context: requirements_missing_context(&brief),
                    clarification_questions: prioritized_requirements_clarification_questions(
                        &brief,
                        &context_summary,
                    ),
                })
            }
            Mode::Discovery => {
                let repo_surfaces = self.scan_workspace_surface()?;
                let brief = DiscoveryBrief::from_context(context_summary, &repo_surfaces);
                Ok(RefinementSeedState {
                    source_inputs,
                    missing_context: discovery_missing_context(&brief),
                    clarification_questions: prioritized_discovery_clarification_questions(&brief),
                })
            }
            Mode::SystemShaping
            | Mode::Brainstorming
            | Mode::Architecture
            | Mode::Change
            | Mode::PolicyShaping => {
                let brief = AuthoredModeBrief::from_context(mode, context_summary, &source_inputs);
                Ok(RefinementSeedState {
                    source_inputs,
                    missing_context: authored_mode_missing_context(&brief),
                    clarification_questions: prioritized_authored_mode_clarification_questions(
                        &brief,
                    ),
                })
            }
            Mode::Backlog
            | Mode::DomainLanguage
            | Mode::DomainModel
            | Mode::Implementation
            | Mode::Refactor
            | Mode::Migration
            | Mode::Debugging
            | Mode::Incident
            | Mode::Review
            | Mode::Verification
            | Mode::PrReview
            | Mode::SystemAssessment
            | Mode::SecurityAssessment
            | Mode::SupplyChainAnalysis => Ok(RefinementSeedState {
                source_inputs,
                missing_context: Vec::new(),
                clarification_questions: Vec::new(),
            }),
        }
    }

    pub(super) fn find_refinement_candidate(
        &self,
        store: &WorkspaceStore,
        mode: Mode,
        input_fingerprints: &[InputFingerprint],
    ) -> Result<Option<ContinuationCandidateSummary>, EngineError> {
        let runs_root = store.layout.runs_dir();
        if !runs_root.exists() {
            return Ok(None);
        }

        let request_keys = Self::refinement_fingerprint_keys(input_fingerprints);
        if request_keys.is_empty() {
            return Ok(None);
        }

        let mut files = Vec::new();
        collect_files_recursively(&runs_root, &mut files)?;

        let mut matches = Vec::new();
        for run_toml in files
            .into_iter()
            .filter(|path| path.file_name().and_then(|name| name.to_str()) == Some("run.toml"))
        {
            let run_manifest = Self::load_toml::<RunManifest>(&run_toml)?.canonicalize();
            if run_manifest.mode != mode {
                continue;
            }

            let Some(run_dir) = run_toml.parent() else {
                continue;
            };
            let run_state = Self::load_toml::<RunStateManifest>(&run_dir.join("state.toml"))?;
            if matches!(
                run_state.state,
                RunState::Superseded | RunState::Aborted | RunState::Failed
            ) {
                continue;
            }

            let run_context = Self::load_toml::<RunContext>(&run_dir.join("context.toml"))?;
            if Self::refinement_fingerprint_keys(&run_context.input_fingerprints) != request_keys {
                continue;
            }

            matches.push(ContinuationCandidateSummary {
                run_id: run_manifest.run_id,
                mode: run_manifest.mode,
                state: run_state.state,
                match_reason: REFINEMENT_MATCH_REASON_SAME_INPUTS.to_string(),
                advisory: true,
            });
        }

        if matches.len() == 1 { Ok(matches.pop()) } else { Ok(None) }
    }

    pub(super) fn next_unique_run_identity(
        &self,
        store: &WorkspaceStore,
    ) -> Result<RunIdentity, EngineError> {
        let base_created_at = OffsetDateTime::now_utc();
        for attempt in 0..RUN_ID_COLLISION_RETRY_LIMIT {
            let identity = if attempt == 0 {
                RunIdentity::new_now_v7()
            } else {
                let shifted_created_at = base_created_at
                    + time::Duration::milliseconds(RUN_ID_COLLISION_WINDOW_MILLIS * attempt as i64);
                let timestamp = Timestamp::from_unix(
                    ContextV7::new(),
                    shifted_created_at.unix_timestamp() as u64,
                    shifted_created_at.nanosecond(),
                );
                RunIdentity::from_parts(Uuid::new_v7(timestamp), shifted_created_at)
            };
            if !store.layout.run_dir(&identity.run_id).exists() {
                return Ok(identity);
            }
        }

        Err(EngineError::Validation(
            "failed to generate a unique run identity after repeated collisions".to_string(),
        ))
    }

    fn refinement_working_brief_path(&self, run_id: &str, mode: Mode) -> String {
        let run_dir = self.project_layout().run_dir(run_id);
        let relative_run_dir = run_dir
            .strip_prefix(self.canon_workspace_root())
            .map(Path::to_path_buf)
            .unwrap_or_else(|_| run_dir.clone());

        relative_run_dir
            .join("artifacts")
            .join(mode.as_str())
            .join(REFINEMENT_WORKING_BRIEF_FILE_NAME)
            .to_string_lossy()
            .into_owned()
    }

    fn refinement_template_ref(mode: Mode) -> String {
        format!("{}/{}.md", REFINEMENT_TEMPLATE_ROOT, mode.as_str())
    }

    fn refinement_fingerprint_keys(
        input_fingerprints: &[InputFingerprint],
    ) -> Vec<(String, &'static str, Option<String>, u64)> {
        let mut keys = input_fingerprints
            .iter()
            .map(|fingerprint| {
                (
                    fingerprint.path.clone(),
                    Self::input_source_kind_label(fingerprint.source_kind),
                    fingerprint.content_digest_sha256.clone(),
                    fingerprint.size_bytes,
                )
            })
            .collect::<Vec<_>>();
        keys.sort();
        keys
    }

    fn input_source_kind_label(kind: InputSourceKind) -> &'static str {
        match kind {
            InputSourceKind::Path => "path",
            InputSourceKind::Inline => "inline",
        }
    }

    fn load_toml<T>(path: &Path) -> Result<T, EngineError>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let contents = std::fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(|error| {
            EngineError::Validation(format!(
                "failed to parse persisted runtime file `{}`: {error}",
                path.display()
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use tempfile::tempdir;
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;
    use crate::domain::run::{
        ClarificationRefinementContext, ClassificationProvenance, InputFingerprint,
        InputSourceKind, RunContext, RunIdentity,
    };
    use crate::persistence::manifests::{RunManifest, RunStateManifest};
    use crate::persistence::store::WorkspaceStore;

    fn requirements_request() -> RunRequest {
        RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: Vec::new(),
            inline_inputs: vec![
                "# Requirements Brief\n\n## Problem\nBound the problem.\n\n## Outcome\nShip a bounded packet.\n\n## Constraints\n- Keep scope explicit.\n\n## Tradeoffs\n- Favor clarity over breadth.\n\n## Out of Scope\n- No unrelated rollout work.\n".to_string(),
            ],
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    fn sample_fingerprint(path: &str, digest: &str) -> InputFingerprint {
        InputFingerprint {
            path: path.to_string(),
            source_kind: InputSourceKind::Path,
            size_bytes: 42,
            modified_unix_seconds: 1_700_000_000,
            content_digest_sha256: Some(digest.to_string()),
            snapshot_ref: None,
        }
    }

    fn persist_run_with_fingerprints(
        store: &WorkspaceStore,
        identity: &RunIdentity,
        mode: Mode,
        state: RunState,
        input_fingerprints: Vec<InputFingerprint>,
    ) {
        let run_dir = store.layout.run_dir(&identity.run_id);
        std::fs::create_dir_all(&run_dir).expect("create run directory");

        let run_manifest = RunManifest::from_identity(
            identity,
            mode,
            RiskClass::LowImpact,
            UsageZone::Green,
            None,
            ClassificationProvenance::explicit(),
            "Owner <owner@example.com>".to_string(),
        );
        let run_context = RunContext {
            repo_root: store.layout.repo_root.display().to_string(),
            workspace_identity: crate::domain::run::WorkspaceIdentity::same_root(
                store.layout.repo_root.display().to_string(),
            ),
            owner: Some("Owner <owner@example.com>".to_string()),
            inputs: vec!["idea.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints,
            system_context: None,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: identity.created_at,
        };
        let run_state = RunStateManifest { state, updated_at: identity.created_at };

        std::fs::write(
            run_dir.join("run.toml"),
            toml::to_string(&run_manifest).expect("serialize run manifest"),
        )
        .expect("write run manifest");
        std::fs::write(
            run_dir.join("context.toml"),
            toml::to_string(&run_context).expect("serialize run context"),
        )
        .expect("write run context");
        std::fs::write(
            run_dir.join("state.toml"),
            toml::to_string(&run_state).expect("serialize run state"),
        )
        .expect("write run state");
    }

    #[test]
    fn run_starts_targeted_refinement_draft_and_persists_working_brief() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let summary = service.run(requirements_request()).expect("requirements draft run");

        assert_eq!(summary.state, "Draft");
        let refinement = summary.refinement_state.expect("refinement state");
        assert_eq!(refinement.current_mode, "requirements");
        assert!(refinement.explicit_continuation_required);
        assert!(workspace.path().join(&refinement.working_brief_path).exists());
    }

    #[test]
    fn retarget_refinement_context_requires_existing_refinement_and_resets_candidate() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());

        let missing = RunContext {
            repo_root: workspace.path().display().to_string(),
            workspace_identity: crate::domain::run::WorkspaceIdentity::same_root(
                workspace.path().display().to_string(),
            ),
            owner: None,
            inputs: Vec::new(),
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: None,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            clarification_refinement: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::from_unix_timestamp(1_700_000_000).expect("timestamp"),
        };
        let error = service
            .retarget_refinement_context(&missing, "R-20260529-missing", Mode::Architecture)
            .expect_err("missing refinement should fail");
        assert!(error.to_string().contains("has no refinement context"));

        let mut with_refinement = missing.clone();
        with_refinement.clarification_refinement = Some(ClarificationRefinementContext {
            workflow_family: RefinementWorkflowFamily::Planning,
            current_mode: Mode::Requirements,
            working_brief_path:
                ".canon/runs/R-20260529-test/artifacts/requirements/working-brief.md".to_string(),
            template_ref: EngineService::refinement_template_ref(Mode::Requirements),
            status: ClarificationRefinementStatus::Ready,
            explicit_continuation_required: false,
            authoritative_input_refs: vec!["idea.md".to_string()],
            supporting_input_refs: vec!["context.md".to_string()],
            suggested_candidate: Some(ContinuationCandidateSummary {
                run_id: "R-20260529-prev".to_string(),
                mode: Mode::Requirements,
                state: RunState::Draft,
                match_reason: "same authoritative input fingerprint".to_string(),
                advisory: true,
            }),
            records: Vec::new(),
            readiness_delta: Vec::new(),
        });

        let updated = service
            .retarget_refinement_context(&with_refinement, "R-20260529-test", Mode::Architecture)
            .expect("retarget refinement context");
        let refinement = updated.clarification_refinement.expect("refinement");
        assert_eq!(refinement.current_mode, Mode::Architecture);
        assert_eq!(refinement.status, ClarificationRefinementStatus::Active);
        assert!(refinement.explicit_continuation_required);
        assert!(refinement.suggested_candidate.is_none());
    }

    #[test]
    fn find_refinement_candidate_handles_missing_empty_and_matching_runtime_state() {
        let workspace = tempdir().expect("tempdir");
        let service = EngineService::new(workspace.path());
        let store = WorkspaceStore::new(workspace.path());

        assert!(
            service
                .find_refinement_candidate(
                    &store,
                    Mode::Requirements,
                    &[sample_fingerprint("idea.md", "abc")]
                )
                .expect("missing runs root")
                .is_none()
        );
        store.init_runtime_state(None).expect("init runtime state");
        assert!(
            service
                .find_refinement_candidate(&store, Mode::Requirements, &[])
                .expect("empty request keys")
                .is_none()
        );

        let created_at = OffsetDateTime::from_unix_timestamp(1_700_000_100).expect("timestamp");
        let mismatch_mode = RunIdentity::from_parts(Uuid::now_v7(), created_at);
        let terminal =
            RunIdentity::from_parts(Uuid::now_v7(), created_at + time::Duration::seconds(1));
        let mismatch_fp =
            RunIdentity::from_parts(Uuid::now_v7(), created_at + time::Duration::seconds(2));
        let matching =
            RunIdentity::from_parts(Uuid::now_v7(), created_at + time::Duration::seconds(3));

        persist_run_with_fingerprints(
            &store,
            &mismatch_mode,
            Mode::Architecture,
            RunState::Draft,
            vec![sample_fingerprint("idea.md", "abc")],
        );
        persist_run_with_fingerprints(
            &store,
            &terminal,
            Mode::Requirements,
            RunState::Failed,
            vec![sample_fingerprint("idea.md", "abc")],
        );
        persist_run_with_fingerprints(
            &store,
            &mismatch_fp,
            Mode::Requirements,
            RunState::Draft,
            vec![sample_fingerprint("idea.md", "different")],
        );
        persist_run_with_fingerprints(
            &store,
            &matching,
            Mode::Requirements,
            RunState::AwaitingApproval,
            vec![sample_fingerprint("idea.md", "abc")],
        );

        let candidate = service
            .find_refinement_candidate(
                &store,
                Mode::Requirements,
                &[sample_fingerprint("idea.md", "abc")],
            )
            .expect("find matching candidate")
            .expect("single matching candidate");

        assert_eq!(candidate.run_id, matching.run_id);
        assert_eq!(candidate.mode, Mode::Requirements);
        assert_eq!(candidate.state, RunState::AwaitingApproval);
        assert!(candidate.advisory);
    }

    #[test]
    fn load_toml_reports_parse_errors_with_path_context() {
        let workspace = tempdir().expect("tempdir");
        let invalid = workspace.path().join("broken.toml");
        std::fs::write(&invalid, "not = [valid").expect("write invalid toml");

        let error = EngineService::load_toml::<RunManifest>(&invalid).expect_err("invalid toml");

        assert!(error.to_string().contains("failed to parse persisted runtime file"));
        assert!(error.to_string().contains("broken.toml"));
    }

    #[test]
    fn resolve_git_owner_handles_partial_local_identity() {
        let workspace = tempdir().expect("tempdir");
        let init_status = Command::new("git")
            .arg("init")
            .current_dir(workspace.path())
            .status()
            .expect("git init status");
        assert!(init_status.success());

        let name_status = Command::new("git")
            .args(["config", "user.name", "Canon Owner"])
            .current_dir(workspace.path())
            .status()
            .expect("set local name");
        assert!(name_status.success());

        let service = EngineService::new(workspace.path());
        assert_eq!(
            service.resolve_git_owner(GitConfigScope::Local),
            Some("Canon Owner".to_string())
        );

        let unset_name = Command::new("git")
            .args(["config", "--unset", "user.name"])
            .current_dir(workspace.path())
            .status()
            .expect("unset local name");
        assert!(unset_name.success());

        let email_status = Command::new("git")
            .args(["config", "user.email", "owner@example.com"])
            .current_dir(workspace.path())
            .status()
            .expect("set local email");
        assert!(email_status.success());

        assert_eq!(
            service.resolve_git_owner(GitConfigScope::Local),
            Some("owner@example.com".to_string())
        );
    }

    #[test]
    fn targeted_refinement_mode_predicate_matches_supported_modes() {
        assert!(EngineService::is_targeted_refinement_mode(Mode::Requirements));
        assert!(EngineService::is_targeted_refinement_mode(Mode::Discovery));
        assert!(EngineService::is_targeted_refinement_mode(Mode::SystemShaping));
        assert!(EngineService::is_targeted_refinement_mode(Mode::Architecture));
        assert!(EngineService::is_targeted_refinement_mode(Mode::Change));
        assert!(!EngineService::is_targeted_refinement_mode(Mode::Review));
        assert!(!EngineService::is_targeted_refinement_mode(Mode::Implementation));
    }
}
