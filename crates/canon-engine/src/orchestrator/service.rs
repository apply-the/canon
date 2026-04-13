use std::path::{Path, PathBuf};

use canon_adapters::classify_capability;
use canon_adapters::copilot_cli::CopilotCliAdapter;
use canon_adapters::filesystem::FilesystemAdapter;
use canon_adapters::shell::ShellAdapter;
use canon_adapters::{CapabilityKind, LineageClass};
use serde::Serialize;
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::artifacts::contract::contract_for_mode;
use crate::artifacts::markdown::{
    render_brownfield_artifact, render_pr_review_artifact,
    render_requirements_artifact_from_evidence,
};
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::artifact::ArtifactRecord;
use crate::domain::execution::{
    DeniedInvocation, EvidenceBundle, GenerationPath, InvocationAttempt, InvocationRequest,
    PolicyDecisionKind, ToolOutcome, ToolOutcomeKind, ValidationPath,
};
use crate::domain::gate::{GateKind, GateStatus};
use crate::domain::mode::{Mode, all_mode_profiles};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{InputFingerprint, RunContext, RunState};
use crate::orchestrator::{classifier, gatekeeper, resume, verification_runner};
use crate::orchestrator::{evidence as evidence_builder, invocation as invocation_runtime};
use crate::persistence::invocations::PersistedInvocation;
use crate::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use crate::persistence::store::{
    InitSummary as StoreInitSummary, PersistedArtifact, PersistedRunBundle,
    SkillMaterializationTarget, SkillsSummary as StoreSkillsSummary, WorkspaceStore,
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
    Invocations { run_id: String },
    Evidence { run_id: String },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTool {
    Codex,
    Copilot,
    Claude,
}

impl AiTool {
    fn materialization_target(self) -> SkillMaterializationTarget {
        match self {
            Self::Codex | Self::Copilot => SkillMaterializationTarget::Agents,
            Self::Claude => SkillMaterializationTarget::Claude,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InitSummary {
    pub repo_root: String,
    pub canon_root: String,
    pub methods_materialized: usize,
    pub policies_materialized: usize,
    pub skills_materialized: usize,
    pub claude_md_created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillsSummary {
    pub skills_dir: String,
    pub skills_materialized: usize,
    pub skills_skipped: usize,
    pub claude_md_created: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillEntry {
    pub name: String,
    pub support_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InspectResponse {
    pub target: String,
    pub entries: Vec<InspectEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum InspectEntry {
    Name(String),
    Invocation(InvocationInspectSummary),
    Evidence(EvidenceInspectSummary),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvocationInspectSummary {
    pub request_id: String,
    pub adapter: String,
    pub capability: String,
    pub orientation: String,
    pub policy_decision: String,
    pub approval_state: String,
    pub latest_outcome: Option<String>,
    pub linked_artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EvidenceInspectSummary {
    pub generation_paths: Vec<String>,
    pub validation_paths: Vec<String>,
    pub denied_invocations: Vec<String>,
    pub artifact_provenance_links: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSummary {
    pub run_id: String,
    pub owner: String,
    pub mode: String,
    pub risk: String,
    pub zone: String,
    pub state: String,
    pub artifact_count: usize,
    pub invocations_total: usize,
    pub invocations_denied: usize,
    pub invocations_pending_approval: usize,
    pub blocking_classification: Option<String>,
    pub blocked_gates: Vec<GateInspectSummary>,
    pub approval_targets: Vec<String>,
    pub artifact_paths: Vec<String>,
    pub recommended_next_action: Option<RecommendedActionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusSummary {
    pub run: String,
    pub owner: String,
    pub state: String,
    pub invocations_total: usize,
    pub pending_invocation_approvals: usize,
    pub validation_independence_satisfied: bool,
    pub blocking_classification: Option<String>,
    pub blocked_gates: Vec<GateInspectSummary>,
    pub approval_targets: Vec<String>,
    pub artifact_paths: Vec<String>,
    pub recommended_next_action: Option<RecommendedActionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GateInspectSummary {
    pub gate: String,
    pub status: String,
    pub blockers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecommendedActionSummary {
    pub action: String,
    pub rationale: String,
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalSummary {
    pub run_id: String,
    pub target: String,
    pub approved_by: String,
    pub recorded_at: String,
    pub decision: String,
    pub state: String,
}

#[derive(Debug, Clone)]
struct RequirementsRequestSpec<'a> {
    run_id: &'a str,
    risk: RiskClass,
    zone: UsageZone,
    owner: &'a str,
    capability: CapabilityKind,
    summary: &'a str,
    scope: Vec<String>,
}

#[derive(Debug, Clone)]
struct GovernedRequestSpec<'a> {
    run_id: &'a str,
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    owner: &'a str,
    adapter: canon_adapters::AdapterKind,
    capability: CapabilityKind,
    summary: &'a str,
    scope: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct RunSummarySpec<'a> {
    run_id: &'a str,
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    state: RunState,
    artifact_count: usize,
}

#[derive(Debug, Clone)]
struct RunRuntimeDetails {
    invocations_total: usize,
    invocations_denied: usize,
    pending_invocation_approvals: usize,
    validation_independence_satisfied: bool,
    blocking_classification: Option<String>,
    blocked_gates: Vec<GateInspectSummary>,
    approval_targets: Vec<String>,
    artifact_paths: Vec<String>,
    recommended_next_action: Option<RecommendedActionSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GitConfigScope {
    Local,
    Global,
}

#[derive(Debug, Clone)]
pub struct EngineService {
    repo_root: PathBuf,
}

impl EngineService {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self { repo_root: repo_root.as_ref().to_path_buf() }
    }

    pub fn init(&self, ai_tool: Option<AiTool>) -> Result<InitSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.init_runtime_state(ai_tool.map(AiTool::materialization_target))?;
        Ok(Self::map_init_summary(summary))
    }

    pub fn skills_install(&self, ai_tool: AiTool) -> Result<SkillsSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.install_skills(ai_tool.materialization_target())?;
        Ok(Self::map_skills_summary(summary))
    }

    pub fn skills_update(&self, ai_tool: AiTool) -> Result<SkillsSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.update_skills(ai_tool.materialization_target())?;
        Ok(Self::map_skills_summary(summary))
    }

    pub fn skills_list(&self) -> Vec<SkillEntry> {
        let store = WorkspaceStore::new(&self.repo_root);
        store
            .list_skills()
            .into_iter()
            .map(|entry| SkillEntry { name: entry.name, support_state: entry.support_state })
            .collect()
    }

    pub fn run(&self, mut request: RunRequest) -> Result<RunSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        store.init_runtime_state(None)?;

        let policy_root = request.policy_root.as_deref().map(|root| {
            let root = PathBuf::from(root);
            if root.is_absolute() { root } else { self.repo_root.join(root) }
        });
        let policy_set = store.load_policy_set(policy_root.as_deref())?;
        let owner_supplied_explicitly = !request.owner.trim().is_empty();
        request.owner = self.resolve_owner(&request.owner);
        classifier::classify_owner_requirement(&policy_set, request.risk, &request.owner).map_err(
            |error| {
                if !owner_supplied_explicitly && request.owner.trim().is_empty() {
                    EngineError::Validation(format!(
                        "{error}; pass --owner or configure git user.name and user.email"
                    ))
                } else {
                    EngineError::Validation(error)
                }
            },
        )?;

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
        target: &str,
        by: &str,
        decision: ApprovalDecision,
        rationale: &str,
    ) -> Result<ApprovalSummary, EngineError> {
        let approver_supplied_explicitly = !by.trim().is_empty();
        let approver = self.resolve_approver(by);
        if !approver_supplied_explicitly && approver.trim().is_empty() {
            return Err(EngineError::Validation(
                "missing approver identity; pass --by or configure git user.name and user.email"
                    .to_string(),
            ));
        }

        let store = WorkspaceStore::new(&self.repo_root);
        let manifest = store.load_run_manifest(run_id)?;
        let contract = store.load_artifact_contract(run_id)?;
        let context = store.load_run_context(run_id)?;
        let mut approvals = store.load_approval_records(run_id)?;
        let artifacts =
            store.load_persisted_artifacts(run_id, manifest.mode, &contract).unwrap_or_default();

        let target_label = target.to_string();
        let approval = if let Some(gate) = target.strip_prefix("gate:") {
            ApprovalRecord::for_gate(
                gate.parse::<GateKind>()
                    .map_err(|error| EngineError::Validation(error.to_string()))?,
                approver.clone(),
                decision,
                rationale.to_string(),
                OffsetDateTime::now_utc(),
            )
        } else if let Some(request_id) = target.strip_prefix("invocation:") {
            ApprovalRecord::for_invocation(
                request_id.to_string(),
                approver,
                decision,
                rationale.to_string(),
                OffsetDateTime::now_utc(),
            )
        } else {
            return Err(EngineError::Validation(format!("unsupported approval target `{target}`")));
        };
        store.persist_approval_record(run_id, &approval)?;
        approvals.push(approval.clone());

        let state = if approval.gate.is_some() {
            self.refresh_run_state(&store, &manifest, &context, &contract, &artifacts, &approvals)?
        } else {
            store.load_run_state(run_id)?.state
        };

        Ok(ApprovalSummary {
            run_id: run_id.to_string(),
            target: target_label,
            approved_by: approval.by.clone(),
            recorded_at: approval.recorded_at.to_string(),
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
        let approvals = store.load_approval_records(run_id)?;
        let artifacts =
            store.load_persisted_artifacts(run_id, manifest.mode, &contract).unwrap_or_default();

        if matches!(manifest.mode, Mode::Requirements) && artifacts.is_empty() {
            let generation_request_id = format!("{run_id}-generate");
            let approved_generation = approvals.iter().any(|approval| {
                approval.matches_invocation(&generation_request_id)
                    && matches!(approval.decision, ApprovalDecision::Approve)
            });

            if !approved_generation {
                return self.summarize_run(
                    &store,
                    RunSummarySpec {
                        run_id,
                        mode: manifest.mode,
                        risk: manifest.risk,
                        zone: manifest.zone,
                        state: RunState::AwaitingApproval,
                        artifact_count: 0,
                    },
                );
            }

            let policy_set = store.load_policy_set(None)?;
            let request = RunRequest {
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                owner: manifest.owner.clone(),
                inputs: context.inputs.clone(),
                excluded_paths: context.excluded_paths.clone(),
                policy_root: None,
                method_root: None,
            };
            let now = OffsetDateTime::now_utc();
            let evidence_path = format!("runs/{run_id}/evidence.toml");
            let context_request = self.requirements_request(RequirementsRequestSpec {
                run_id,
                risk: request.risk,
                zone: request.zone,
                owner: &request.owner,
                capability: CapabilityKind::ReadRepository,
                summary: "capture repository and idea context",
                scope: request.inputs.clone(),
            });
            let context_decision =
                invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
            let context_summary = self.read_requirements_context(&request.inputs)?;
            let context_attempt = self.completed_attempt(
                &context_request,
                1,
                "filesystem",
                ToolOutcome {
                    kind: ToolOutcomeKind::Succeeded,
                    summary: format!(
                        "Captured requirements context from {} input(s).",
                        request.inputs.len()
                    ),
                    exit_code: Some(0),
                    payload_refs: Vec::new(),
                    candidate_artifacts: Vec::new(),
                    recorded_at: OffsetDateTime::now_utc(),
                },
            );
            let generation_request = self.requirements_request(RequirementsRequestSpec {
                run_id,
                risk: request.risk,
                zone: request.zone,
                owner: &request.owner,
                capability: CapabilityKind::GenerateContent,
                summary: "generate bounded requirements framing",
                scope: request.inputs.clone(),
            });
            let generation_decision =
                invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
            let denied_edit_request = self.requirements_request(RequirementsRequestSpec {
                run_id,
                risk: request.risk,
                zone: request.zone,
                owner: &request.owner,
                capability: CapabilityKind::ProposeWorkspaceEdit,
                summary: "attempt workspace mutation from requirements mode",
                scope: request.inputs.clone(),
            });
            let denied_edit_decision =
                invocation_runtime::evaluate_request_policy(&denied_edit_request, &policy_set);
            let denied_invocations =
                if matches!(denied_edit_decision.kind, PolicyDecisionKind::Deny) {
                    vec![DeniedInvocation {
                        request_id: denied_edit_request.request_id.clone(),
                        rationale: denied_edit_decision.rationale.clone(),
                        policy_refs: denied_edit_decision.policy_refs.clone(),
                        recorded_at: denied_edit_decision.decided_at,
                    }]
                } else {
                    Vec::new()
                };

            let copilot = CopilotCliAdapter;
            let generation_output = copilot.generate(&context_summary);
            let generation_attempt = self.completed_attempt(
                &generation_request,
                1,
                &generation_output.executor,
                ToolOutcome {
                    kind: ToolOutcomeKind::Succeeded,
                    summary: generation_output.summary.clone(),
                    exit_code: Some(0),
                    payload_refs: Vec::new(),
                    candidate_artifacts: contract
                        .artifact_requirements
                        .iter()
                        .map(|requirement| requirement.file_name.clone())
                        .collect(),
                    recorded_at: OffsetDateTime::now_utc(),
                },
            );
            let critique_request = self.requirements_request(RequirementsRequestSpec {
                run_id,
                risk: request.risk,
                zone: request.zone,
                owner: &request.owner,
                capability: CapabilityKind::CritiqueContent,
                summary: "critique generated requirements framing",
                scope: request.inputs.clone(),
            });
            let critique_decision =
                invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
            let critique_output = copilot.critique(&generation_output.summary);
            let critique_attempt = self.completed_attempt(
                &critique_request,
                1,
                &critique_output.executor,
                ToolOutcome {
                    kind: ToolOutcomeKind::Succeeded,
                    summary: critique_output.summary.clone(),
                    exit_code: Some(0),
                    payload_refs: Vec::new(),
                    candidate_artifacts: Vec::new(),
                    recorded_at: OffsetDateTime::now_utc(),
                },
            );

            let generation_path = GenerationPath {
                path_id: format!("generation:{}", generation_request.request_id),
                request_ids: vec![generation_request.request_id.clone()],
                lineage_classes: vec![LineageClass::AiVendorFamily],
                derived_artifacts: contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| {
                        format!(
                            "artifacts/{}/{}/{}",
                            run_id,
                            request.mode.as_str(),
                            requirement.file_name
                        )
                    })
                    .collect(),
            };
            let validation_path = ValidationPath {
                path_id: format!("validation:{}", critique_request.request_id),
                request_ids: vec![critique_request.request_id.clone()],
                lineage_classes: vec![LineageClass::AiVendorFamily],
                verification_refs: vec![format!(
                    "runs/{run_id}/invocations/{}/attempt-01.toml",
                    critique_request.request_id
                )],
                independence: evidence_builder::default_independence(&generation_path.path_id),
            };
            let validation_path = ValidationPath {
                independence: evidence_builder::assess_validation_independence(
                    &generation_path,
                    &validation_path,
                ),
                ..validation_path
            };
            let denied_summary = if denied_invocations.is_empty() {
                "No governed invocations were denied during requirements mode.".to_string()
            } else {
                denied_invocations
                    .iter()
                    .map(|denied| denied.rationale.clone())
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let artifacts = contract
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
                        provenance: Some(crate::domain::artifact::ArtifactProvenance {
                            request_ids: vec![
                                context_request.request_id.clone(),
                                generation_request.request_id.clone(),
                                critique_request.request_id.clone(),
                            ],
                            evidence_bundle: Some(evidence_path.clone()),
                            disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                        }),
                    },
                    contents: render_requirements_artifact_from_evidence(
                        &requirement.file_name,
                        &context_summary,
                        &generation_output.summary,
                        &critique_output.summary,
                        &denied_summary,
                    ),
                })
                .collect::<Vec<_>>();
            let gate_inputs = artifacts
                .iter()
                .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
                .collect::<Vec<_>>();
            let gates = gatekeeper::evaluate_requirements_gates(
                &contract,
                &gate_inputs,
                &request.owner,
                &denied_invocations,
                true,
            );
            let state = run_state_from_gates(&gates);
            let artifact_paths = artifacts
                .iter()
                .map(|artifact| artifact.record.relative_path.clone())
                .collect::<Vec<_>>();
            let mut verification_records =
                verification_runner::requirements_verification_records(&artifact_paths);
            for record in &mut verification_records {
                record.request_ids = vec![
                    generation_request.request_id.clone(),
                    critique_request.request_id.clone(),
                ];
                record.validation_path_id = Some(validation_path.path_id.clone());
                record.evidence_bundle = Some(evidence_path.clone());
            }
            let evidence = EvidenceBundle {
                run_id: run_id.to_string(),
                generation_paths: vec![generation_path],
                validation_paths: vec![validation_path],
                denied_invocations,
                trace_refs: vec![format!("traces/{run_id}.jsonl")],
                artifact_refs: artifact_paths,
                decision_refs: vec![
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        context_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        generation_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        critique_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        denied_edit_request.request_id
                    ),
                ],
                approval_refs: approvals
                    .iter()
                    .filter_map(|approval| {
                        approval
                            .invocation_request_id
                            .as_ref()
                            .map(|request_id| format!("runs/{run_id}/approvals/{}", request_id))
                    })
                    .collect(),
            };

            let bundle = PersistedRunBundle {
                run: manifest.clone(),
                context: context.clone(),
                state: RunStateManifest { state, updated_at: now },
                artifact_contract: contract.clone(),
                links: LinkManifest {
                    artifacts: artifacts
                        .iter()
                        .map(|artifact| artifact.record.relative_path.clone())
                        .collect(),
                    decisions: Vec::new(),
                    traces: Vec::new(),
                    invocations: Vec::new(),
                    evidence: Some(evidence_path.clone()),
                },
                verification_records,
                artifacts,
                gates,
                approvals: approvals.clone(),
                evidence: Some(evidence),
                invocations: vec![
                    PersistedInvocation {
                        request: context_request,
                        decision: context_decision,
                        attempts: vec![context_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: generation_request,
                        decision: generation_decision,
                        attempts: vec![generation_attempt],
                        approvals: approvals
                            .iter()
                            .filter(|approval| approval.matches_invocation(&generation_request_id))
                            .cloned()
                            .collect(),
                    },
                    PersistedInvocation {
                        request: critique_request,
                        decision: critique_decision,
                        attempts: vec![critique_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: denied_edit_request,
                        decision: denied_edit_decision,
                        attempts: Vec::new(),
                        approvals: Vec::new(),
                    },
                ],
            };
            store.persist_run_bundle(&bundle)?;
            return self.summarize_run(
                &store,
                RunSummarySpec {
                    run_id,
                    mode: manifest.mode,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    state,
                    artifact_count: bundle.artifacts.len(),
                },
            );
        }

        if matches!(manifest.mode, Mode::BrownfieldChange) && artifacts.is_empty() {
            let policy_set = store.load_policy_set(None)?;
            let request = RunRequest {
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                owner: manifest.owner.clone(),
                inputs: context.inputs.clone(),
                excluded_paths: context.excluded_paths.clone(),
                policy_root: None,
                method_root: None,
            };
            return self.execute_brownfield_change(
                &store,
                request,
                policy_set,
                run_id.to_string(),
                approvals.clone(),
            );
        }

        let state =
            self.refresh_run_state(&store, &manifest, &context, &contract, &artifacts, &approvals)?;

        self.summarize_run(
            &store,
            RunSummarySpec {
                run_id,
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                state,
                artifact_count: artifacts.len(),
            },
        )
    }

    pub fn inspect(&self, target: InspectTarget) -> Result<InspectResponse, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let (name, entries) = match target {
            InspectTarget::Modes => (
                "modes".to_string(),
                Mode::all()
                    .iter()
                    .map(|mode| InspectEntry::Name(mode.as_str().to_string()))
                    .collect::<Vec<_>>(),
            ),
            InspectTarget::Methods => (
                "methods".to_string(),
                store.list_method_files()?.into_iter().map(InspectEntry::Name).collect::<Vec<_>>(),
            ),
            InspectTarget::Policies => (
                "policies".to_string(),
                store.list_policy_files()?.into_iter().map(InspectEntry::Name).collect::<Vec<_>>(),
            ),
            InspectTarget::Artifacts { run_id } => (
                "artifacts".to_string(),
                store
                    .list_artifact_files(&run_id)?
                    .into_iter()
                    .map(InspectEntry::Name)
                    .collect::<Vec<_>>(),
            ),
            InspectTarget::Invocations { run_id } => {
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
                            approval_state: approval_state.to_string(),
                            latest_outcome: invocation
                                .attempts
                                .last()
                                .map(|attempt| format!("{:?}", attempt.outcome.kind)),
                            linked_artifacts,
                        })
                    })
                    .collect::<Vec<_>>();
                ("invocations".to_string(), entries)
            }
            InspectTarget::Evidence { run_id } => {
                let entries = store
                    .load_evidence_bundle(&run_id)?
                    .map(|evidence| {
                        vec![InspectEntry::Evidence(EvidenceInspectSummary {
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
                ("evidence".to_string(), entries)
            }
        };

        Ok(InspectResponse { target: name, entries })
    }

    pub fn status(&self, run: &str) -> Result<StatusSummary, EngineError> {
        let _ = all_mode_profiles();
        let store = WorkspaceStore::new(&self.repo_root);
        let manifest = store.load_run_manifest(run)?;
        let state = store.load_run_state(run)?;
        let details = self.collect_run_runtime_details(&store, run, manifest.mode, state.state)?;

        Ok(StatusSummary {
            run: run.to_string(),
            owner: manifest.owner,
            state: format!("{:?}", state.state),
            invocations_total: details.invocations_total,
            pending_invocation_approvals: details.pending_invocation_approvals,
            validation_independence_satisfied: details.validation_independence_satisfied,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            recommended_next_action: details.recommended_next_action,
        })
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

        let input_fingerprints = self.capture_input_fingerprints(&request.inputs)?;
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            capability: CapabilityKind::ReadRepository,
            summary: "capture repository and idea context",
            scope: request.inputs.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary = self.read_requirements_context(&request.inputs)?;
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured requirements context from {} input(s).",
                    request.inputs.len()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded requirements framing",
            scope: request.inputs.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);

        let denied_edit_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            capability: CapabilityKind::ProposeWorkspaceEdit,
            summary: "attempt workspace mutation from requirements mode",
            scope: request.inputs.clone(),
        });
        let denied_edit_decision =
            invocation_runtime::evaluate_request_policy(&denied_edit_request, &policy_set);
        let denied_invocations = if matches!(denied_edit_decision.kind, PolicyDecisionKind::Deny) {
            vec![DeniedInvocation {
                request_id: denied_edit_request.request_id.clone(),
                rationale: denied_edit_decision.rationale.clone(),
                policy_refs: denied_edit_decision.policy_refs.clone(),
                recorded_at: denied_edit_decision.decided_at,
            }]
        } else {
            Vec::new()
        };
        let generation_policy_attempt =
            self.policy_decision_attempt(&generation_request, &generation_decision);
        let denied_edit_policy_attempt =
            self.policy_decision_attempt(&denied_edit_request, &denied_edit_decision);

        if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
            let evidence = EvidenceBundle {
                run_id: run_id.clone(),
                generation_paths: Vec::new(),
                validation_paths: Vec::new(),
                denied_invocations,
                trace_refs: vec![format!("traces/{run_id}.jsonl")],
                artifact_refs: Vec::new(),
                decision_refs: vec![
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        context_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        generation_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        denied_edit_request.request_id
                    ),
                ],
                approval_refs: Vec::new(),
            };
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
                state: RunStateManifest { state: RunState::AwaitingApproval, updated_at: now },
                artifact_contract,
                links: LinkManifest {
                    artifacts: Vec::new(),
                    decisions: Vec::new(),
                    traces: Vec::new(),
                    invocations: Vec::new(),
                    evidence: Some(evidence_path),
                },
                verification_records: Vec::new(),
                artifacts: Vec::new(),
                gates: Vec::new(),
                approvals: Vec::new(),
                evidence: Some(evidence),
                invocations: vec![
                    PersistedInvocation {
                        request: context_request,
                        decision: context_decision,
                        attempts: vec![context_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: generation_request,
                        decision: generation_decision,
                        attempts: vec![generation_policy_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: denied_edit_request,
                        decision: denied_edit_decision,
                        attempts: vec![denied_edit_policy_attempt],
                        approvals: Vec::new(),
                    },
                ],
            };
            store.persist_run_bundle(&bundle)?;
            let details = self.collect_run_runtime_details(
                store,
                &bundle.run.run_id,
                bundle.run.mode,
                bundle.state.state,
            )?;
            return Ok(RunSummary {
                run_id,
                owner: bundle.run.owner.clone(),
                mode: bundle.run.mode.as_str().to_string(),
                risk: bundle.run.risk.as_str().to_string(),
                zone: bundle.run.zone.as_str().to_string(),
                state: format!("{:?}", bundle.state.state),
                artifact_count: 0,
                invocations_total: details.invocations_total,
                invocations_denied: details.invocations_denied,
                invocations_pending_approval: details.pending_invocation_approvals,
                blocking_classification: details.blocking_classification,
                blocked_gates: details.blocked_gates,
                approval_targets: details.approval_targets,
                artifact_paths: details.artifact_paths,
                recommended_next_action: details.recommended_next_action,
            });
        }

        let copilot = CopilotCliAdapter;
        let generation_output = copilot.generate(&context_summary);
        let generation_attempt = self.completed_attempt(
            &generation_request,
            1,
            &generation_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: generation_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let critique_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique generated requirements framing",
            scope: request.inputs.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot.critique(&generation_output.summary);
        let critique_attempt = self.completed_attempt(
            &critique_request,
            1,
            &critique_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: critique_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_path = GenerationPath {
            path_id: format!("generation:{}", generation_request.request_id),
            request_ids: vec![generation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_contract
                .artifact_requirements
                .iter()
                .map(|requirement| {
                    format!(
                        "artifacts/{}/{}/{}",
                        run_id,
                        request.mode.as_str(),
                        requirement.file_name
                    )
                })
                .collect(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                critique_request.request_id
            )],
            independence: evidence_builder::default_independence(&generation_path.path_id),
        };
        let validation_path = ValidationPath {
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &validation_path,
            ),
            ..validation_path
        };

        let denied_summary = if denied_invocations.is_empty() {
            "No governed invocations were denied during requirements mode.".to_string()
        } else {
            denied_invocations
                .iter()
                .map(|denied| denied.rationale.clone())
                .collect::<Vec<_>>()
                .join(" ")
        };
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
                    provenance: Some(crate::domain::artifact::ArtifactProvenance {
                        request_ids: vec![
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_requirements_artifact_from_evidence(
                    &requirement.file_name,
                    &context_summary,
                    &generation_output.summary,
                    &critique_output.summary,
                    &denied_summary,
                ),
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
            &denied_invocations,
            true,
        );
        let state = run_state_from_gates(&gates);
        let artifact_paths = artifacts
            .iter()
            .map(|artifact| artifact.record.relative_path.clone())
            .collect::<Vec<_>>();
        let mut verification_records =
            verification_runner::requirements_verification_records(&artifact_paths);
        for record in &mut verification_records {
            record.request_ids =
                vec![generation_request.request_id.clone(), critique_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }

        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations,
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    denied_edit_request.request_id
                ),
            ],
            approval_refs: Vec::new(),
        };

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
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals: Vec::new(),
            evidence: Some(evidence),
            invocations: vec![
                PersistedInvocation {
                    request: context_request,
                    decision: context_decision,
                    attempts: vec![context_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: generation_request,
                    decision: generation_decision,
                    attempts: vec![generation_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: denied_edit_request,
                    decision: denied_edit_decision,
                    attempts: vec![denied_edit_policy_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        let details = self.collect_run_runtime_details(
            store,
            &bundle.run.run_id,
            bundle.run.mode,
            bundle.state.state,
        )?;

        Ok(RunSummary {
            run_id,
            owner: bundle.run.owner.clone(),
            mode: bundle.run.mode.as_str().to_string(),
            risk: bundle.run.risk.as_str().to_string(),
            zone: bundle.run.zone.as_str().to_string(),
            state: format!("{:?}", bundle.state.state),
            artifact_count: bundle.artifacts.len(),
            invocations_total: details.invocations_total,
            invocations_denied: details.invocations_denied,
            invocations_pending_approval: details.pending_invocation_approvals,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            recommended_next_action: details.recommended_next_action,
        })
    }

    fn run_brownfield_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let run_id = Uuid::now_v7().to_string();
        self.execute_brownfield_change(store, request, policy_set, run_id, Vec::new())
    }

    fn execute_brownfield_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
        run_id: String,
        approvals: Vec<ApprovalRecord>,
    ) -> Result<RunSummary, EngineError> {
        let now = OffsetDateTime::now_utc();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let brief_summary = self.load_input_summary(&request.inputs)?;
        let input_fingerprints = self.capture_input_fingerprints(&request.inputs)?;
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::BrownfieldChange,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture brownfield brief and repository context",
            scope: request.inputs.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary = self.read_requirements_context(&request.inputs)?;
        let declared_change_surface = extract_brownfield_change_surface_entries(&context_summary);
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: "Captured brownfield brief and bounded repository context.".to_string(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::BrownfieldChange,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded brownfield change framing",
            scope: request.inputs.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let generation_policy_attempt =
            self.policy_decision_attempt(&generation_request, &generation_decision);

        let mutation_summary = if declared_change_surface.is_empty() {
            "propose bounded legacy transformation without mutating the workspace".to_string()
        } else {
            format!(
                "propose bounded legacy transformation within declared change surface: {}",
                declared_change_surface.join(", ")
            )
        };
        let mutation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::BrownfieldChange,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ExecuteBoundedTransformation,
            summary: &mutation_summary,
            scope: declared_change_surface.clone(),
        });
        let mutation_decision =
            invocation_runtime::evaluate_request_policy(&mutation_request, &policy_set);
        let mutation_attempt = self.policy_decision_attempt(&mutation_request, &mutation_decision);

        let approved_generation = approvals.iter().any(|approval| {
            approval.matches_invocation(&generation_request.request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });
        let approved_mutation = approvals.iter().any(|approval| {
            approval.matches_invocation(&mutation_request.request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });

        if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval)
            && !approved_generation
        {
            let evidence = EvidenceBundle {
                run_id: run_id.clone(),
                generation_paths: Vec::new(),
                validation_paths: Vec::new(),
                denied_invocations: Vec::new(),
                trace_refs: vec![format!("traces/{run_id}.jsonl")],
                artifact_refs: Vec::new(),
                decision_refs: vec![
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        context_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        generation_request.request_id
                    ),
                    format!(
                        "runs/{run_id}/invocations/{}/decision.toml",
                        mutation_request.request_id
                    ),
                ],
                approval_refs: approvals
                    .iter()
                    .filter_map(|approval| {
                        approval
                            .invocation_request_id
                            .as_ref()
                            .map(|request_id| format!("runs/{run_id}/approvals/{}", request_id))
                    })
                    .collect(),
            };

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
                state: RunStateManifest { state: RunState::AwaitingApproval, updated_at: now },
                artifact_contract,
                links: LinkManifest {
                    artifacts: Vec::new(),
                    decisions: Vec::new(),
                    traces: Vec::new(),
                    invocations: Vec::new(),
                    evidence: Some(evidence_path),
                },
                verification_records: Vec::new(),
                artifacts: Vec::new(),
                gates: Vec::new(),
                approvals,
                evidence: Some(evidence),
                invocations: vec![
                    PersistedInvocation {
                        request: context_request,
                        decision: context_decision,
                        attempts: vec![context_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: generation_request,
                        decision: generation_decision,
                        attempts: vec![generation_policy_attempt],
                        approvals: Vec::new(),
                    },
                    PersistedInvocation {
                        request: mutation_request,
                        decision: mutation_decision,
                        attempts: vec![mutation_attempt],
                        approvals: Vec::new(),
                    },
                ],
            };
            store.persist_run_bundle(&bundle)?;
            return self.summarize_run(
                store,
                RunSummarySpec {
                    run_id: &run_id,
                    mode: request.mode,
                    risk: request.risk,
                    zone: request.zone,
                    state: RunState::AwaitingApproval,
                    artifact_count: 0,
                },
            );
        }

        let copilot = CopilotCliAdapter;
        let generation_context = if context_summary == brief_summary {
            brief_summary.clone()
        } else {
            format!("{brief_summary}\n\n{context_summary}")
        };
        let generation_output = copilot.generate(&generation_context);
        let generation_attempt = self.completed_attempt(
            &generation_request,
            if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
                2
            } else {
                1
            },
            &generation_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: generation_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let validation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::BrownfieldChange,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: "validate brownfield change framing against repository context",
            scope: request.inputs.clone(),
        });
        let validation_decision =
            invocation_runtime::evaluate_request_policy(&validation_request, &policy_set);
        let (validation_summary, validation_attempt) =
            self.brownfield_validation_attempt(&validation_request)?;

        let artifact_paths = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                format!("artifacts/{}/{}/{}", run_id, request.mode.as_str(), requirement.file_name)
            })
            .collect::<Vec<_>>();
        let generation_path = GenerationPath {
            path_id: format!("generation:{}", generation_request.request_id),
            request_ids: vec![generation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_paths.clone(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", validation_request.request_id),
            request_ids: vec![validation_request.request_id.clone()],
            lineage_classes: vec![LineageClass::NonGenerative],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                validation_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", validation_request.request_id),
                    request_ids: vec![validation_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::NonGenerative],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        validation_request.request_id
                    )],
                    independence: evidence_builder::default_independence(&generation_path.path_id),
                },
            ),
        };

        let evidence_backed_summary = format!(
            "{brief_summary}\n\nGenerated framing: {}\n\nValidation evidence: {}\n\nMutation posture: {}",
            generation_output.summary, validation_summary, mutation_attempt.outcome.summary
        );
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
                    provenance: Some(crate::domain::artifact::ArtifactProvenance {
                        request_ids: vec![
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            validation_request.request_id.clone(),
                            mutation_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_brownfield_artifact(
                    &requirement.file_name,
                    &evidence_backed_summary,
                ),
            })
            .collect::<Vec<_>>();

        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_brownfield_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::BrownfieldGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                validation_independence_satisfied: validation_path.independence.sufficient,
                evidence_complete: true,
            },
        );
        let mut state = run_state_from_gates(&gates);
        if mutation_decision.requires_approval
            && !approved_mutation
            && !matches!(state, RunState::Blocked)
        {
            state = RunState::AwaitingApproval;
        }

        let mut verification_records = verification_runner::brownfield_verification_records(
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids =
                vec![generation_request.request_id.clone(), validation_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }

        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path.clone()],
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    validation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", mutation_request.request_id),
            ],
            approval_refs: approvals
                .iter()
                .filter_map(|approval| {
                    approval
                        .invocation_request_id
                        .as_ref()
                        .map(|request_id| format!("runs/{run_id}/approvals/{}", request_id))
                })
                .collect(),
        };

        let generation_attempts =
            if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
                vec![generation_policy_attempt, generation_attempt]
            } else {
                vec![generation_attempt]
            };

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
                artifacts: artifact_paths.clone(),
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals,
            evidence: Some(evidence),
            invocations: vec![
                PersistedInvocation {
                    request: context_request,
                    decision: context_decision,
                    attempts: vec![context_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: generation_request,
                    decision: generation_decision,
                    attempts: generation_attempts,
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: validation_request,
                    decision: validation_decision,
                    attempts: vec![validation_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: mutation_request,
                    decision: mutation_decision,
                    attempts: vec![mutation_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        self.summarize_run(
            store,
            RunSummarySpec {
                run_id: &run_id,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                state,
                artifact_count: bundle.artifacts.len(),
            },
        )
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
        let is_worktree = head_ref == "WORKTREE";
        let input_fingerprints = self.capture_input_fingerprints(&request.inputs)?;
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let diff_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::PrReview,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::InspectDiff,
            summary: "inspect diff surfaces and bounded patch context",
            scope: request.inputs.clone(),
        });
        let diff_decision = invocation_runtime::evaluate_request_policy(&diff_request, &policy_set);

        let shell = ShellAdapter;
        let diff = if is_worktree {
            shell.git_diff_worktree(&base_ref, &self.repo_root)
        } else {
            shell.git_diff(&base_ref, &head_ref, &self.repo_root)
        }
        .map_err(|error| {
            EngineError::Validation(format!("unable to collect pr-review diff: {error}"))
        })?;

        let changed_files_ref = store.persist_invocation_payload_text(
            &run_id,
            &diff_request.request_id,
            "changed-files.txt",
            &diff.changed_files_text,
        )?;
        let patch_ref = store.persist_invocation_payload_text(
            &run_id,
            &diff_request.request_id,
            "diff.patch",
            &diff.patch,
        )?;
        let diff_attempt = self.completed_attempt(
            &diff_request,
            1,
            "shell:git-diff",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Inspected {} changed surface(s) between {base_ref} and {head_ref}.",
                    diff.changed_files.len()
                ),
                exit_code: Some(0),
                payload_refs: vec![changed_files_ref, patch_ref],
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::PrReview,
            risk: request.risk,
            zone: request.zone,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique the reviewed diff and preserve reviewer-facing evidence",
            scope: request.inputs.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_input = format!(
            "Review {base_ref}..{head_ref} across {} changed surface(s): {}",
            diff.changed_files.len(),
            if diff.changed_files.is_empty() {
                "no changed surfaces detected".to_string()
            } else {
                diff.changed_files.join(", ")
            }
        );
        let copilot = CopilotCliAdapter;
        let critique_output = copilot.critique(&critique_input);
        let critique_attempt = self.completed_attempt(
            &critique_request,
            1,
            &critique_output.executor,
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: critique_output.summary.clone(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: artifact_contract
                    .artifact_requirements
                    .iter()
                    .map(|requirement| requirement.file_name.clone())
                    .collect(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let review_packet = ReviewPacket::from_evidence(
            &diff.base_ref,
            &diff.head_ref,
            diff.changed_files.clone(),
            &diff.patch,
            &critique_output.summary,
        );
        let review_summary = ReviewSummary::from_evidence(&review_packet, false);
        let artifact_paths = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                format!("artifacts/{}/{}/{}", run_id, request.mode.as_str(), requirement.file_name)
            })
            .collect::<Vec<_>>();
        let generation_path = GenerationPath {
            path_id: format!("generation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            derived_artifacts: artifact_paths.clone(),
        };
        let validation_path = ValidationPath {
            path_id: format!("validation:{}", diff_request.request_id),
            request_ids: vec![diff_request.request_id.clone()],
            lineage_classes: vec![LineageClass::NonGenerative],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                diff_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", diff_request.request_id),
                    request_ids: vec![diff_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::NonGenerative],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        diff_request.request_id
                    )],
                    independence: evidence_builder::default_independence(&generation_path.path_id),
                },
            ),
        };

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
                    provenance: Some(crate::domain::artifact::ArtifactProvenance {
                        request_ids: vec![
                            diff_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition:
                            crate::domain::execution::EvidenceDisposition::NeedsDisposition,
                    }),
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
            gatekeeper::PrReviewGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                denied_invocations: &[],
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);
        let mut verification_records = verification_runner::pr_review_verification_records(
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids =
                vec![diff_request.request_id.clone(), critique_request.request_id.clone()];
            record.validation_path_id = Some(validation_path.path_id.clone());
            record.evidence_bundle = Some(evidence_path.clone());
        }
        let evidence = EvidenceBundle {
            run_id: run_id.clone(),
            generation_paths: vec![generation_path],
            validation_paths: vec![validation_path],
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", diff_request.request_id),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
            ],
            approval_refs: Vec::new(),
        };

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
                artifacts: artifact_paths,
                decisions: Vec::new(),
                traces: Vec::new(),
                invocations: Vec::new(),
                evidence: Some(evidence_path.clone()),
            },
            verification_records,
            artifacts,
            gates,
            approvals,
            evidence: Some(evidence),
            invocations: vec![
                PersistedInvocation {
                    request: diff_request,
                    decision: diff_decision,
                    attempts: vec![diff_attempt],
                    approvals: Vec::new(),
                },
                PersistedInvocation {
                    request: critique_request,
                    decision: critique_decision,
                    attempts: vec![critique_attempt],
                    approvals: Vec::new(),
                },
            ],
        };

        store.persist_run_bundle(&bundle)?;
        store.persist_adapter_invocations(&run_id, &diff.invocations)?;
        self.summarize_run(
            store,
            RunSummarySpec {
                run_id: &run_id,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                state,
                artifact_count: bundle.artifacts.len(),
            },
        )
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
        let evidence_bundle = store.load_evidence_bundle(&manifest.run_id)?;
        let evidence_complete = evidence_bundle.is_some();
        let validation_independence_satisfied = evidence_bundle
            .as_ref()
            .map(|bundle| bundle.validation_paths.iter().all(|path| path.independence.sufficient))
            .unwrap_or(true);

        let gates = match manifest.mode {
            Mode::BrownfieldChange => gatekeeper::evaluate_brownfield_gates(
                contract,
                &artifacts
                    .iter()
                    .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
                    .collect::<Vec<_>>(),
                gatekeeper::BrownfieldGateContext {
                    owner: &manifest.owner,
                    risk: manifest.risk,
                    zone: manifest.zone,
                    approvals,
                    validation_independence_satisfied,
                    evidence_complete,
                },
            ),
            Mode::PrReview => gatekeeper::evaluate_pr_review_gates(
                contract,
                &artifacts
                    .iter()
                    .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
                    .collect::<Vec<_>>(),
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
        let state = run_state_from_gates(&gates);
        let state_manifest = RunStateManifest { state, updated_at: OffsetDateTime::now_utc() };
        store.persist_gate_evaluations(&manifest.run_id, &gates)?;
        store.persist_run_state(&manifest.run_id, &state_manifest)?;
        Ok(state)
    }

    fn requirements_request(&self, spec: RequirementsRequestSpec<'_>) -> InvocationRequest {
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
            owner: spec.owner,
            adapter,
            capability: spec.capability,
            summary: spec.summary,
            scope: spec.scope,
        })
    }

    fn governed_request(&self, spec: GovernedRequestSpec<'_>) -> InvocationRequest {
        let capability_profile = classify_capability(spec.adapter, spec.capability);

        InvocationRequest {
            request_id: format!("{}-{}", spec.run_id, capability_tag(spec.capability)),
            run_id: spec.run_id.to_string(),
            mode: spec.mode.as_str().to_string(),
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

    fn read_requirements_context(&self, inputs: &[String]) -> Result<String, EngineError> {
        let filesystem = FilesystemAdapter;
        let mut fragments = Vec::new();
        for input in inputs {
            let path = self.resolve_input_path(input);
            if path.is_file() {
                let (contents, _) = filesystem
                    .read_to_string_traced(&path, "capture requirements context")
                    .map_err(|error| EngineError::Validation(error.to_string()))?;
                fragments.push(contents);
            } else {
                fragments.push(input.clone());
            }
        }

        let normalized = preserve_multiline_summary(&fragments.join("\n"));
        Ok(if normalized.is_empty() {
            "Capture the bounded engineering need before implementation accelerates drift."
                .to_string()
        } else {
            normalized
        })
    }

    fn brownfield_validation_attempt(
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

    fn scan_workspace_surface(&self) -> Result<Vec<String>, EngineError> {
        let mut collected = Vec::new();
        let mut stack = vec![self.repo_root.clone()];

        while let Some(path) = stack.pop() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy();

                if name == ".git" || name == ".canon" || name == "target" {
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

    fn completed_attempt(
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

    fn policy_decision_attempt(
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

    fn summarize_run(
        &self,
        store: &WorkspaceStore,
        spec: RunSummarySpec<'_>,
    ) -> Result<RunSummary, EngineError> {
        let details =
            self.collect_run_runtime_details(store, spec.run_id, spec.mode, spec.state)?;
        let manifest = store.load_run_manifest(spec.run_id)?;

        Ok(RunSummary {
            run_id: spec.run_id.to_string(),
            owner: manifest.owner,
            mode: spec.mode.as_str().to_string(),
            risk: spec.risk.as_str().to_string(),
            zone: spec.zone.as_str().to_string(),
            state: format!("{:?}", spec.state),
            artifact_count: spec.artifact_count,
            invocations_total: details.invocations_total,
            invocations_denied: details.invocations_denied,
            invocations_pending_approval: details.pending_invocation_approvals,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            recommended_next_action: details.recommended_next_action,
        })
    }

    fn collect_run_runtime_details(
        &self,
        store: &WorkspaceStore,
        run_id: &str,
        mode: Mode,
        state: RunState,
    ) -> Result<RunRuntimeDetails, EngineError> {
        let invocations = store.load_persisted_invocations(run_id).unwrap_or_default();
        let evidence_bundle = store.load_evidence_bundle(run_id)?;
        let gates = store.load_gate_evaluations(run_id).unwrap_or_default();

        let artifact_paths = store
            .load_artifact_contract(run_id)
            .ok()
            .and_then(|contract| store.load_persisted_artifacts(run_id, mode, &contract).ok())
            .map(|artifacts| {
                artifacts
                    .into_iter()
                    .map(|artifact| format!(".canon/{}", artifact.record.relative_path))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

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

        let recommended_next_action = recommend_next_action(
            state,
            &artifact_paths,
            evidence_bundle.is_some(),
            &blocked_gates,
            &approval_targets,
        );

        Ok(RunRuntimeDetails {
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
            recommended_next_action,
        })
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
        let normalized = preserve_multiline_summary(&combined);
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
            skills_materialized: summary.skills_materialized,
            claude_md_created: summary.claude_md_created,
        }
    }

    fn map_skills_summary(summary: StoreSkillsSummary) -> SkillsSummary {
        SkillsSummary {
            skills_dir: summary.skills_dir,
            skills_materialized: summary.skills_materialized,
            skills_skipped: summary.skills_skipped,
            claude_md_created: summary.claude_md_created,
        }
    }

    fn resolve_approver(&self, explicit_approver: &str) -> String {
        self.resolve_identity(explicit_approver)
    }

    fn resolve_owner(&self, explicit_owner: &str) -> String {
        self.resolve_identity(explicit_owner)
    }

    fn resolve_identity(&self, explicit_identity: &str) -> String {
        let explicit_identity = explicit_identity.trim();
        if !explicit_identity.is_empty() {
            return explicit_identity.to_string();
        }

        self.resolve_git_owner(GitConfigScope::Local)
            .or_else(|| self.resolve_git_owner(GitConfigScope::Global))
            .unwrap_or_default()
    }

    fn resolve_git_owner(&self, scope: GitConfigScope) -> Option<String> {
        let name = self.git_config_value(scope, "user.name");
        let email = self.git_config_value(scope, "user.email");

        match (name, email) {
            (Some(name), Some(email)) => Some(format!("{name} <{email}>")),
            (Some(name), None) => Some(name),
            (None, Some(email)) => Some(email),
            (None, None) => None,
        }
    }

    fn git_config_value(&self, scope: GitConfigScope, key: &str) -> Option<String> {
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

        if output.status_code != 0 {
            return None;
        }

        let value = output.stdout.trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    }
}

fn preserve_multiline_summary(value: &str) -> String {
    let mut lines = Vec::new();
    let mut previous_blank = false;

    for raw_line in value.lines() {
        let line = raw_line.split_whitespace().collect::<Vec<_>>().join(" ");
        if line.is_empty() {
            if !previous_blank && !lines.is_empty() {
                lines.push(String::new());
            }
            previous_blank = true;
        } else {
            lines.push(line);
            previous_blank = false;
        }
    }

    lines.join("\n").trim().to_string()
}

fn extract_brownfield_change_surface_entries(source: &str) -> Vec<String> {
    let normalized = source.to_lowercase();
    let Some(raw_surface) = extract_marker(source, &normalized, "change surface") else {
        return Vec::new();
    };

    let mut entries = Vec::new();
    for line in raw_surface.lines() {
        let trimmed = line.trim().trim_start_matches(['-', '*', '+']).trim();
        if trimmed.is_empty() {
            continue;
        }

        for segment in trimmed.split(';').flat_map(|segment| segment.split(',')) {
            let value = segment.trim();
            if value.is_empty() {
                continue;
            }
            if !entries.iter().any(|existing: &String| existing.eq_ignore_ascii_case(value)) {
                entries.push(value.to_string());
            }
        }
    }

    entries
}

fn extract_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    extract_markdown_section(source, marker)
        .or_else(|| extract_inline_marker(source, normalized, marker))
}

fn extract_inline_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    let marker_with_colon = format!("{marker}:");
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let line = remainder.lines().next()?.trim();
    if line.is_empty() { None } else { Some(line.to_string()) }
}

fn extract_markdown_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if is_section_boundary(next_line) {
                break;
            }

            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_multiline_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
    }

    None
}

fn is_matching_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

fn is_section_boundary(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#')
        || trimmed.starts_with("Generated framing:")
        || trimmed.starts_with("Validation evidence:")
        || trimmed.starts_with("Mutation posture:")
}

fn trim_multiline_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
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

fn recommend_next_action(
    state: RunState,
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

fn capability_tag(capability: CapabilityKind) -> &'static str {
    match capability {
        CapabilityKind::ReadRepository => "context",
        CapabilityKind::GenerateContent => "generate",
        CapabilityKind::CritiqueContent => "critique",
        CapabilityKind::ProposeWorkspaceEdit => "edit",
        CapabilityKind::InspectDiff => "inspect-diff",
        CapabilityKind::ReadArtifact => "read-artifact",
        CapabilityKind::EmitArtifact => "emit-artifact",
        CapabilityKind::RunCommand => "run-command",
        CapabilityKind::ValidateWithTool => "validate",
        CapabilityKind::InvokeStructuredTool => "structured-tool",
        CapabilityKind::ExecuteBoundedTransformation => "transform",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GateInspectSummary, RecommendedActionSummary, extract_brownfield_change_surface_entries,
        preserve_multiline_summary, recommend_next_action,
    };
    use crate::domain::run::RunState;

    #[test]
    fn change_surface_entries_prefer_markdown_section_over_inline_summary_mentions() {
        let source = "# Brownfield Brief\n\n## Change Surface\n- session repository\n- auth service\n\nMutation posture: propose bounded legacy transformation within declared change surface: entire repository, adjacent modules";

        let entries = extract_brownfield_change_surface_entries(source);

        assert_eq!(entries, vec!["session repository".to_string(), "auth service".to_string()]);
    }

    #[test]
    fn preserve_multiline_summary_keeps_bullets_on_separate_lines() {
        let summary = "## Change Surface\n- first bullet\n- second bullet\n\n## Validation Strategy\n- independent check";

        let normalized = preserve_multiline_summary(summary);

        assert!(normalized.contains("- first bullet\n- second bullet"));
        assert!(normalized.contains("\n\n## Validation Strategy\n- independent check"));
    }

    #[test]
    fn recommend_next_action_prefers_artifact_review_for_approval_gated_runs() {
        let action = recommend_next_action(
            RunState::AwaitingApproval,
            &[".canon/artifacts/run-123/brownfield-change/system-slice.md".to_string()],
            true,
            &[],
            &["invocation:req-1".to_string()],
        );

        assert_eq!(
            action,
            Some(RecommendedActionSummary {
                action: "inspect-artifacts".to_string(),
                rationale: "Review the emitted packet before recording approval.".to_string(),
                target: None,
            })
        );
    }

    #[test]
    fn recommend_next_action_points_to_direct_approval_when_no_packet_exists() {
        let action = recommend_next_action(
            RunState::AwaitingApproval,
            &[],
            false,
            &[],
            &["gate:review-disposition".to_string()],
        );

        assert_eq!(
            action,
            Some(RecommendedActionSummary {
                action: "approve".to_string(),
                rationale: "Canon is explicitly waiting for approval on a real target.".to_string(),
                target: Some("gate:review-disposition".to_string()),
            })
        );
    }

    #[test]
    fn recommend_next_action_uses_evidence_for_blocked_runs_without_artifacts() {
        let action = recommend_next_action(
            RunState::Blocked,
            &[],
            true,
            &[GateInspectSummary {
                gate: "brownfield-preservation".to_string(),
                status: "Blocked".to_string(),
                blockers: vec!["legacy-invariants.md missing".to_string()],
            }],
            &[],
        );

        assert_eq!(
            action,
            Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "The run is blocked but no readable artifact packet was found; inspect the evidence bundle next.".to_string(),
                target: None,
            })
        );
    }
}
