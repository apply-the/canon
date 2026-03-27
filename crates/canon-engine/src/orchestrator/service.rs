use std::path::{Path, PathBuf};

use canon_adapters::shell::ShellAdapter;
use serde::Serialize;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::artifacts::contract::contract_for_mode;
use crate::artifacts::markdown::{
    render_brownfield_artifact, render_pr_review_artifact, render_requirements_artifact,
};
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::artifact::ArtifactRecord;
use crate::domain::gate::{GateKind, GateStatus};
use crate::domain::mode::{Mode, all_mode_profiles};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{InputFingerprint, RunContext, RunState};
use crate::orchestrator::{classifier, gatekeeper, resume, verification_runner};
use crate::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use crate::persistence::store::{
    InitSummary as StoreInitSummary, PersistedArtifact, PersistedRunBundle, WorkspaceStore,
};
use crate::review::findings::ReviewPacket;
use crate::review::summary::ReviewSummary;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported inspect target: {0}")]
    UnsupportedInspectTarget(String),
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("mode `{0}` is not implemented yet")]
    UnsupportedMode(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectTarget {
    Modes,
    Methods,
    Policies,
    Artifacts { run_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    pub mode: Mode,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub owner: String,
    pub inputs: Vec<String>,
    pub excluded_paths: Vec<String>,
    pub policy_root: Option<String>,
    pub method_root: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InitSummary {
    pub repo_root: String,
    pub canon_root: String,
    pub methods_materialized: usize,
    pub policies_materialized: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InspectResponse {
    pub target: String,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSummary {
    pub run_id: String,
    pub mode: String,
    pub risk: String,
    pub zone: String,
    pub state: String,
    pub artifact_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusSummary {
    pub run: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalSummary {
    pub run_id: String,
    pub gate: String,
    pub decision: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct EngineService {
    repo_root: PathBuf,
}

impl EngineService {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self { repo_root: repo_root.as_ref().to_path_buf() }
    }

    pub fn init(&self) -> Result<InitSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.init_runtime_state()?;
        Ok(Self::map_init_summary(summary))
    }

    pub fn run(&self, request: RunRequest) -> Result<RunSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        store.init_runtime_state()?;

        let policy_root = request.policy_root.as_deref().map(|root| {
            let root = PathBuf::from(root);
            if root.is_absolute() { root } else { self.repo_root.join(root) }
        });
        let policy_set = store.load_policy_set(policy_root.as_deref())?;
        classifier::classify_owner_requirement(&policy_set, request.risk, &request.owner)
            .map_err(EngineError::Validation)?;

        match request.mode {
            Mode::Requirements => self.run_requirements(&store, request, policy_set),
            Mode::BrownfieldChange => self.run_brownfield_change(&store, request, policy_set),
            Mode::PrReview => self.run_pr_review(&store, request, policy_set),
            other => Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        }
    }

    pub fn approve(
        &self,
        run_id: &str,
        gate: GateKind,
        by: &str,
        decision: ApprovalDecision,
        rationale: &str,
    ) -> Result<ApprovalSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let manifest = store.load_run_manifest(run_id)?;
        let contract = store.load_artifact_contract(run_id)?;
        let context = store.load_run_context(run_id)?;
        let mut approvals = store.load_approval_records(run_id)?;
        let artifacts = store.load_persisted_artifacts(run_id, manifest.mode, &contract)?;

        let approval = ApprovalRecord {
            gate,
            by: by.to_string(),
            decision,
            rationale: rationale.to_string(),
            recorded_at: OffsetDateTime::now_utc(),
        };
        store.persist_approval_record(run_id, &approval)?;
        approvals.push(approval.clone());

        let state =
            self.refresh_run_state(&store, &manifest, &context, &contract, &artifacts, &approvals)?;

        Ok(ApprovalSummary {
            run_id: run_id.to_string(),
            gate: gate.as_str().to_string(),
            decision: decision.as_str().to_string(),
            state: format!("{state:?}"),
        })
    }

    pub fn resume(&self, run_id: &str) -> Result<RunSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let manifest = store.load_run_manifest(run_id)?;
        let context = store.load_run_context(run_id)?;

        if !resume::input_fingerprints_match(&self.repo_root, &context.input_fingerprints)? {
            return Err(EngineError::Validation(format!(
                "stale run `{run_id}`: input context changed; fork or rerun instead"
            )));
        }

        let contract = store.load_artifact_contract(run_id)?;
        let artifacts = store.load_persisted_artifacts(run_id, manifest.mode, &contract)?;
        let approvals = store.load_approval_records(run_id)?;
        let state =
            self.refresh_run_state(&store, &manifest, &context, &contract, &artifacts, &approvals)?;

        Ok(RunSummary {
            run_id: run_id.to_string(),
            mode: manifest.mode.as_str().to_string(),
            risk: manifest.risk.as_str().to_string(),
            zone: manifest.zone.as_str().to_string(),
            state: format!("{state:?}"),
            artifact_count: artifacts.len(),
        })
    }

    pub fn inspect(&self, target: InspectTarget) -> Result<InspectResponse, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let (name, entries) = match target {
            InspectTarget::Modes => (
                "modes".to_string(),
                Mode::all().iter().map(|mode| mode.as_str().to_string()).collect::<Vec<_>>(),
            ),
            InspectTarget::Methods => ("methods".to_string(), store.list_method_files()?),
            InspectTarget::Policies => ("policies".to_string(), store.list_policy_files()?),
            InspectTarget::Artifacts { run_id } => {
                ("artifacts".to_string(), store.list_artifact_files(&run_id)?)
            }
        };

        Ok(InspectResponse { target: name, entries })
    }

    pub fn status(&self, run: &str) -> Result<StatusSummary, EngineError> {
        let _ = all_mode_profiles();
        let store = WorkspaceStore::new(&self.repo_root);
        let state = store.load_run_state(run)?;
        Ok(StatusSummary { run: run.to_string(), state: format!("{:?}", state.state) })
    }

    fn run_requirements(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let now = OffsetDateTime::now_utc();
        let run_id = Uuid::now_v7().to_string();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let idea_summary = self.load_input_summary(&request.inputs)?;
        let input_fingerprints = self.capture_input_fingerprints(&request.inputs)?;
        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| PersistedArtifact {
                record: ArtifactRecord {
                    file_name: requirement.file_name.clone(),
                    relative_path: format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    ),
                    format: requirement.format,
                },
                contents: render_requirements_artifact(&requirement.file_name, &idea_summary),
            })
            .collect::<Vec<_>>();

        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_requirements_gates(
            &artifact_contract,
            &gate_inputs,
            &request.owner,
        );
        let state = run_state_from_gates(&gates);

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                owner: request.owner.clone(),
                created_at: now,
            },
            context: RunContext {
                repo_root: self.repo_root.display().to_string(),
                owner: Some(request.owner),
                inputs: request.inputs,
                excluded_paths: request.excluded_paths,
                input_fingerprints,
                captured_at: now,
            },
            state: RunStateManifest { state, updated_at: now },
            artifact_contract,
            links: LinkManifest {
                artifacts: artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect(),
                decisions: Vec::new(),
                traces: Vec::new(),
            },
            verification_records: verification_runner::requirements_verification_records(
                &artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect::<Vec<_>>(),
            ),
            artifacts,
            gates,
            approvals: Vec::new(),
        };

        store.persist_run_bundle(&bundle)?;

        Ok(RunSummary {
            run_id,
            mode: bundle.run.mode.as_str().to_string(),
            risk: bundle.run.risk.as_str().to_string(),
            zone: bundle.run.zone.as_str().to_string(),
            state: format!("{:?}", bundle.state.state),
            artifact_count: bundle.artifacts.len(),
        })
    }

    fn run_brownfield_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let now = OffsetDateTime::now_utc();
        let run_id = Uuid::now_v7().to_string();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let brief_summary = self.load_input_summary(&request.inputs)?;
        let input_fingerprints = self.capture_input_fingerprints(&request.inputs)?;
        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| PersistedArtifact {
                record: ArtifactRecord {
                    file_name: requirement.file_name.clone(),
                    relative_path: format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    ),
                    format: requirement.format,
                },
                contents: render_brownfield_artifact(&requirement.file_name, &brief_summary),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_brownfield_gates(
            &artifact_contract,
            &gate_inputs,
            &request.owner,
            request.risk,
            request.zone,
            &approvals,
        );
        let state = run_state_from_gates(&gates);

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                owner: request.owner.clone(),
                created_at: now,
            },
            context: RunContext {
                repo_root: self.repo_root.display().to_string(),
                owner: Some(request.owner),
                inputs: request.inputs,
                excluded_paths: request.excluded_paths,
                input_fingerprints,
                captured_at: now,
            },
            state: RunStateManifest { state, updated_at: now },
            artifact_contract: artifact_contract.clone(),
            links: LinkManifest {
                artifacts: artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect(),
                decisions: Vec::new(),
                traces: Vec::new(),
            },
            verification_records: verification_runner::brownfield_verification_records(
                &artifact_contract.required_verification_layers,
                &artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect::<Vec<_>>(),
            ),
            artifacts,
            gates,
            approvals,
        };

        store.persist_run_bundle(&bundle)?;

        Ok(RunSummary {
            run_id,
            mode: bundle.run.mode.as_str().to_string(),
            risk: bundle.run.risk.as_str().to_string(),
            zone: bundle.run.zone.as_str().to_string(),
            state: format!("{:?}", bundle.state.state),
            artifact_count: bundle.artifacts.len(),
        })
    }

    fn run_pr_review(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let now = OffsetDateTime::now_utc();
        let run_id = Uuid::now_v7().to_string();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let (base_ref, head_ref) = self.load_pr_review_refs(&request.inputs)?;
        let input_fingerprints = self.capture_input_fingerprints(&request.inputs)?;

        let shell = ShellAdapter;
        let diff = shell.git_diff(&base_ref, &head_ref, &self.repo_root).map_err(|error| {
            EngineError::Validation(format!("unable to collect pr-review diff: {error}"))
        })?;
        let review_packet = ReviewPacket::from_diff(
            &diff.base_ref,
            &diff.head_ref,
            diff.changed_files,
            &diff.patch,
        );
        let review_summary = ReviewSummary::from_packet(&review_packet, false);

        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| PersistedArtifact {
                record: ArtifactRecord {
                    file_name: requirement.file_name.clone(),
                    relative_path: format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    ),
                    format: requirement.format,
                },
                contents: render_pr_review_artifact(
                    &requirement.file_name,
                    &review_packet,
                    &review_summary,
                ),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_pr_review_gates(
            &artifact_contract,
            &gate_inputs,
            &request.owner,
            request.risk,
            request.zone,
            &approvals,
        );
        let state = run_state_from_gates(&gates);

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                owner: request.owner.clone(),
                created_at: now,
            },
            context: RunContext {
                repo_root: self.repo_root.display().to_string(),
                owner: Some(request.owner),
                inputs: request.inputs,
                excluded_paths: request.excluded_paths,
                input_fingerprints,
                captured_at: now,
            },
            state: RunStateManifest { state, updated_at: now },
            artifact_contract: artifact_contract.clone(),
            links: LinkManifest {
                artifacts: artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect(),
                decisions: Vec::new(),
                traces: Vec::new(),
            },
            verification_records: verification_runner::pr_review_verification_records(
                &artifact_contract.required_verification_layers,
                &artifacts
                    .iter()
                    .map(|artifact| artifact.record.relative_path.clone())
                    .collect::<Vec<_>>(),
            ),
            artifacts,
            gates,
            approvals,
        };

        store.persist_run_bundle(&bundle)?;
        store.persist_adapter_invocations(&run_id, &diff.invocations)?;

        Ok(RunSummary {
            run_id,
            mode: bundle.run.mode.as_str().to_string(),
            risk: bundle.run.risk.as_str().to_string(),
            zone: bundle.run.zone.as_str().to_string(),
            state: format!("{:?}", bundle.state.state),
            artifact_count: bundle.artifacts.len(),
        })
    }

    fn refresh_run_state(
        &self,
        store: &WorkspaceStore,
        manifest: &RunManifest,
        _context: &RunContext,
        contract: &crate::domain::artifact::ArtifactContract,
        artifacts: &[PersistedArtifact],
        approvals: &[ApprovalRecord],
    ) -> Result<RunState, EngineError> {
        let gates = match manifest.mode {
            Mode::BrownfieldChange => gatekeeper::evaluate_brownfield_gates(
                contract,
                &artifacts
                    .iter()
                    .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
                    .collect::<Vec<_>>(),
                &manifest.owner,
                manifest.risk,
                manifest.zone,
                approvals,
            ),
            Mode::PrReview => gatekeeper::evaluate_pr_review_gates(
                contract,
                &artifacts
                    .iter()
                    .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
                    .collect::<Vec<_>>(),
                &manifest.owner,
                manifest.risk,
                manifest.zone,
                approvals,
            ),
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };
        let state = run_state_from_gates(&gates);
        let state_manifest = RunStateManifest { state, updated_at: OffsetDateTime::now_utc() };
        store.persist_gate_evaluations(&manifest.run_id, &gates)?;
        store.persist_run_state(&manifest.run_id, &state_manifest)?;
        Ok(state)
    }

    fn load_input_summary(&self, inputs: &[String]) -> Result<String, EngineError> {
        let mut fragments = Vec::new();

        for input in inputs {
            let resolved = self.resolve_input_path(input);

            if resolved.is_file() {
                let contents = std::fs::read_to_string(&resolved)?;
                fragments.push(contents);
            } else {
                fragments.push(input.clone());
            }
        }

        let combined = fragments.join("\n");
        let normalized = combined.split_whitespace().collect::<Vec<_>>().join(" ");
        if normalized.is_empty() {
            Ok("Capture the bounded engineering need before implementation accelerates drift."
                .to_string())
        } else {
            Ok(normalized)
        }
    }

    fn capture_input_fingerprints(
        &self,
        inputs: &[String],
    ) -> Result<Vec<InputFingerprint>, EngineError> {
        let mut fingerprints = Vec::new();

        for input in inputs {
            let resolved = self.resolve_input_path(input);
            if !resolved.is_file() {
                continue;
            }

            let metadata = std::fs::metadata(&resolved)?;
            let modified = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs() as i64)
                .unwrap_or_default();

            fingerprints.push(InputFingerprint {
                path: input.clone(),
                size_bytes: metadata.len(),
                modified_unix_seconds: modified,
            });
        }

        Ok(fingerprints)
    }

    fn resolve_input_path(&self, input: &str) -> PathBuf {
        let path = PathBuf::from(input);
        if path.is_absolute() { path } else { self.repo_root.join(path) }
    }

    fn load_pr_review_refs(&self, inputs: &[String]) -> Result<(String, String), EngineError> {
        if inputs.len() < 2 {
            return Err(EngineError::Validation(
                "pr-review requires two inputs: <base-ref> <head-ref>".to_string(),
            ));
        }

        Ok((inputs[0].clone(), inputs[1].clone()))
    }

    fn map_init_summary(summary: StoreInitSummary) -> InitSummary {
        InitSummary {
            repo_root: summary.repo_root,
            canon_root: summary.canon_root,
            methods_materialized: summary.methods_materialized,
            policies_materialized: summary.policies_materialized,
        }
    }
}

fn run_state_from_gates(gates: &[crate::domain::gate::GateEvaluation]) -> RunState {
    if gates.iter().any(|gate| matches!(gate.status, GateStatus::NeedsApproval)) {
        RunState::AwaitingApproval
    } else if gates.iter().any(|gate| matches!(gate.status, GateStatus::Blocked)) {
        RunState::Blocked
    } else {
        RunState::Completed
    }
}
