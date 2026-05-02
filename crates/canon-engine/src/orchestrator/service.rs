use std::path::{Path, PathBuf};

use canon_adapters::classify_capability;
use canon_adapters::copilot_cli::{CopilotCliAdapter, RequirementsGenerationInput};
use canon_adapters::filesystem::FilesystemAdapter;
use canon_adapters::shell::ShellAdapter;
use canon_adapters::{CapabilityKind, LineageClass};
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;
use time::OffsetDateTime;

use crate::artifacts::contract::contract_for_mode;
use crate::artifacts::markdown::{
    render_architecture_artifact, render_change_artifact, render_discovery_artifact,
    render_implementation_artifact, render_incident_artifact, render_migration_artifact,
    render_pr_review_artifact, render_refactor_artifact,
    render_requirements_artifact_from_evidence, render_review_artifact,
    render_security_assessment_artifact, render_supply_chain_analysis_artifact,
    render_system_assessment_artifact, render_system_shaping_artifact,
    render_verification_artifact,
};
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::artifact::ArtifactRecord;
use crate::domain::execution::{
    DeniedInvocation, EvidenceBundle, ExecutionPosture, GenerationPath, InvocationAttempt,
    InvocationRequest, MutationBounds, MutationExpansionPolicy, PolicyDecisionKind, ToolOutcome,
    ToolOutcomeKind, ValidationPath,
};
use crate::domain::gate::{GateKind, GateStatus};
use crate::domain::mode::{Mode, all_mode_profiles};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::{
    ClassificationProvenance, ImplementationExecutionContext, InlineInput, InputFingerprint,
    InputSourceKind, RefactorExecutionContext, RunContext, RunIdentity, RunState, SystemContext,
    UpstreamContext,
};
use crate::orchestrator::publish::{PublishSummary, publish_run};
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

pub(crate) mod clarity;
pub(crate) mod context_parse;
pub(crate) mod execution;
pub(crate) mod next_action;
pub(crate) mod patch;
pub(crate) mod summarizers;

mod inspect;
mod mode_backlog;
mod mode_change;
mod mode_discovery;
mod mode_incident;
mod mode_migration;
mod mode_pr_review;
mod mode_requirements;
mod mode_review;
mod mode_security_assessment;
mod mode_shaping;
mod mode_supply_chain_analysis;
mod mode_system_assessment;

use clarity::*;
use context_parse::*;
use execution::*;
use next_action::*;
use patch::*;
use summarizers::*;

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
    RiskZone {
        mode: Mode,
        risk: Option<RiskClass>,
        zone: Option<UsageZone>,
        inputs: Vec<String>,
        inline_inputs: Vec<String>,
    },
    Clarity {
        mode: Mode,
        inputs: Vec<String>,
    },
    Artifacts {
        run_id: String,
    },
    Invocations {
        run_id: String,
    },
    Evidence {
        run_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    pub mode: Mode,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub system_context: Option<SystemContext>,
    pub classification: ClassificationProvenance,
    pub owner: String,
    pub inputs: Vec<String>,
    pub inline_inputs: Vec<String>,
    pub excluded_paths: Vec<String>,
    pub policy_root: Option<String>,
    pub method_root: Option<String>,
}

impl RunRequest {
    fn authored_input_count(&self) -> usize {
        self.inputs.len() + self.inline_inputs.len()
    }

    fn merged_input_sources(&self) -> Vec<String> {
        let mut sources = self.inputs.clone();
        sources.extend(
            self.inline_inputs.iter().enumerate().map(|(index, _)| inline_input_label(index)),
        );
        sources
    }

    fn transient_inline_inputs(&self) -> Vec<InlineInput> {
        self.inline_inputs
            .iter()
            .enumerate()
            .map(|(index, contents)| InlineInput {
                label: inline_input_label(index),
                contents: contents.clone(),
            })
            .collect()
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_context: Option<String>,
    pub entries: Vec<InspectEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum InspectEntry {
    Name(String),
    RiskZone(ClassificationInspectSummary),
    Clarity(ClarityInspectSummary),
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
    pub recommendation_only: bool,
    pub approval_state: String,
    pub latest_outcome: Option<String>,
    pub linked_artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClosureFindingInspectSummary {
    pub category: String,
    pub severity: String,
    pub affected_scope: String,
    pub recommended_followup: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EvidenceInspectSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_posture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_feature_slice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_upstream_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub upstream_source_refs: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub carried_forward_items: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_upstream_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition_scope: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub closure_findings: Vec<ClosureFindingInspectSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_notes: Option<String>,
    pub generation_paths: Vec<String>,
    pub validation_paths: Vec<String>,
    pub denied_invocations: Vec<String>,
    pub artifact_provenance_links: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationInspectSummary {
    pub mode: String,
    pub risk: String,
    pub zone: String,
    pub risk_was_supplied: bool,
    pub zone_was_supplied: bool,
    pub confidence: String,
    pub requires_confirmation: bool,
    pub headline: String,
    pub rationale: String,
    pub risk_rationale: String,
    pub zone_rationale: String,
    pub signals: Vec<String>,
    pub risk_signals: Vec<String>,
    pub zone_signals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClarificationQuestionSummary {
    pub id: String,
    pub prompt: String,
    pub rationale: String,
    pub evidence: String,
    pub affects: String,
    pub default_if_skipped: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClarityInspectSummary {
    pub mode: String,
    pub summary: String,
    pub source_inputs: Vec<String>,
    pub requires_clarification: bool,
    pub missing_context: Vec<String>,
    pub clarification_questions: Vec<ClarificationQuestionSummary>,
    pub reasoning_signals: Vec<String>,
    pub output_quality: OutputQualitySummary,
    pub recommended_focus: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutputQualitySummary {
    pub posture: String,
    pub materially_closed: bool,
    pub evidence_signals: Vec<String>,
    pub downgrade_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResultActionSummary {
    pub id: String,
    pub label: String,
    pub host_action: String,
    pub target: String,
    pub text_fallback: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionChip {
    pub id: String,
    pub label: String,
    pub skill: String,
    pub intent: String,
    pub prefilled_args: std::collections::BTreeMap<String, String>,
    pub required_user_inputs: Vec<String>,
    pub visibility_condition: String,
    pub recommended: bool,
    pub text_fallback: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModeResultSummary {
    pub headline: String,
    pub artifact_packet_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_posture: Option<String>,
    pub primary_artifact_title: String,
    pub primary_artifact_path: String,
    pub primary_artifact_action: ResultActionSummary,
    pub result_excerpt: String,
    #[serde(default)]
    pub action_chips: Vec<ActionChip>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSummary {
    pub run_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    pub owner: String,
    pub mode: String,
    pub risk: String,
    pub zone: String,
    pub system_context: Option<String>,
    pub state: String,
    pub artifact_count: usize,
    pub invocations_total: usize,
    pub invocations_denied: usize,
    pub invocations_pending_approval: usize,
    pub blocking_classification: Option<String>,
    pub blocked_gates: Vec<GateInspectSummary>,
    pub approval_targets: Vec<String>,
    pub artifact_paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition_scope: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub closure_findings: Vec<ClosureFindingInspectSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_notes: Option<String>,
    pub mode_result: Option<ModeResultSummary>,
    pub recommended_next_action: Option<RecommendedActionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusSummary {
    pub run: String,
    pub owner: String,
    pub state: String,
    pub system_context: Option<String>,
    pub invocations_total: usize,
    pub pending_invocation_approvals: usize,
    pub validation_independence_satisfied: bool,
    pub blocking_classification: Option<String>,
    pub blocked_gates: Vec<GateInspectSummary>,
    pub approval_targets: Vec<String>,
    pub artifact_paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition_scope: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub closure_findings: Vec<ClosureFindingInspectSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_notes: Option<String>,
    pub mode_result: Option<ModeResultSummary>,
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
pub(super) struct RequirementsRequestSpec<'a> {
    run_id: &'a str,
    risk: RiskClass,
    zone: UsageZone,
    system_context: Option<SystemContext>,
    owner: &'a str,
    capability: CapabilityKind,
    summary: &'a str,
    scope: Vec<String>,
}

#[derive(Debug, Clone)]
pub(super) struct GovernedRequestSpec<'a> {
    run_id: &'a str,
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    system_context: Option<SystemContext>,
    owner: &'a str,
    adapter: canon_adapters::AdapterKind,
    capability: CapabilityKind,
    summary: &'a str,
    scope: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct RunSummarySpec<'a> {
    run_id: &'a str,
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    state: RunState,
    artifact_count: usize,
}

#[derive(Debug, Clone)]
pub(super) struct RunRuntimeDetails {
    system_context: Option<SystemContext>,
    invocations_total: usize,
    invocations_denied: usize,
    pending_invocation_approvals: usize,
    validation_independence_satisfied: bool,
    blocking_classification: Option<String>,
    blocked_gates: Vec<GateInspectSummary>,
    approval_targets: Vec<String>,
    artifact_paths: Vec<String>,
    closure_status: Option<String>,
    decomposition_scope: Option<String>,
    closure_findings: Vec<ClosureFindingInspectSummary>,
    closure_notes: Option<String>,
    mode_result: Option<ModeResultSummary>,
    recommended_next_action: Option<RecommendedActionSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GitConfigScope {
    Local,
    Global,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AuthoredMutationPatch {
    absolute_path: PathBuf,
    relative_path: String,
    changed_paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EngineService {
    repo_root: PathBuf,
}

impl EngineService {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self { repo_root: repo_root.as_ref().to_path_buf() }
    }

    /// Repository root this service operates against.
    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    /// Resolve a user-supplied run reference (`run_id`, full UUID, prefix
    /// `short_id`, or `@last`) to its canonical filesystem key.
    ///
    /// This is the canonical entry point for CLI commands that accept a
    /// run reference; it surfaces ambiguity, missing-history, and not-found
    /// errors as `EngineError::Validation`.
    pub fn resolve_run(&self, query: &str) -> Result<String, EngineError> {
        use crate::persistence::layout::ProjectLayout;
        use crate::persistence::lookup::{LookupQuery, resolve};
        let layout = ProjectLayout::new(&self.repo_root);
        let parsed = LookupQuery::parse(query);
        let handle =
            resolve(&layout, &parsed).map_err(|e| EngineError::Validation(e.to_string()))?;
        Ok(handle.run_id)
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
        classifier::validate_system_context(request.mode, request.system_context)
            .map_err(EngineError::Validation)?;
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
        request.inputs = self.auto_bind_canonical_mode_inputs(
            request.mode,
            &request.inputs,
            &request.inline_inputs,
        );
        self.validate_authored_inputs(request.mode, &request.inputs, &request.inline_inputs)?;

        match request.mode {
            Mode::Requirements => self.run_requirements(&store, request, policy_set),
            Mode::Discovery => self.run_discovery(&store, request, policy_set),
            Mode::SystemShaping => self.run_system_shaping(&store, request, policy_set),
            Mode::Change => self.run_change(&store, request, policy_set),
            Mode::Backlog => self.run_backlog(&store, request, policy_set),
            Mode::Incident => self.run_incident(&store, request, policy_set),
            Mode::SystemAssessment => self.run_system_assessment(&store, request, policy_set),
            Mode::SecurityAssessment => self.run_security_assessment(&store, request, policy_set),
            Mode::SupplyChainAnalysis => {
                self.run_supply_chain_analysis(&store, request, policy_set)
            }
            Mode::Implementation => self.run_implementation(&store, request, policy_set),
            Mode::Migration => self.run_migration(&store, request, policy_set),
            Mode::Refactor => self.run_refactor(&store, request, policy_set),
            Mode::Architecture => self.run_architecture(&store, request, policy_set),
            Mode::Review => self.run_review(&store, request, policy_set),
            Mode::Verification => self.run_verification(&store, request, policy_set),
            Mode::PrReview => self.run_pr_review(&store, request, policy_set),
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
        let canonical = self.resolve_run(run_id)?;
        let run_id = canonical.as_str();
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
        let canonical = self.resolve_run(run_id)?;
        let run_id = canonical.as_str();
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
                system_context: context.system_context,
                classification: manifest.classification.clone(),
                owner: manifest.owner.clone(),
                inputs: self.resume_inputs(&context),
                inline_inputs: Vec::new(),
                excluded_paths: context.excluded_paths.clone(),
                policy_root: None,
                method_root: None,
            };
            let input_scope = request.merged_input_sources();
            let now = OffsetDateTime::now_utc();
            let evidence_path = format!("runs/{run_id}/evidence.toml");
            let context_request = self.requirements_request(RequirementsRequestSpec {
                run_id,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                owner: &request.owner,
                capability: CapabilityKind::ReadRepository,
                summary: "capture repository and idea context",
                scope: input_scope.clone(),
            });
            let context_decision =
                invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
            let context_summary =
                self.read_requirements_context(&request.inputs, &request.inline_inputs)?;
            let context_attempt = self.completed_attempt(
                &context_request,
                1,
                "filesystem",
                ToolOutcome {
                    kind: ToolOutcomeKind::Succeeded,
                    summary: format!(
                        "Captured requirements context from {} input(s).",
                        request.authored_input_count()
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
                system_context: request.system_context,
                owner: &request.owner,
                capability: CapabilityKind::GenerateContent,
                summary: "generate bounded requirements framing",
                scope: input_scope.clone(),
            });
            let generation_decision =
                invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
            let denied_edit_request = self.requirements_request(RequirementsRequestSpec {
                run_id,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                owner: &request.owner,
                capability: CapabilityKind::ProposeWorkspaceEdit,
                summary: "attempt workspace mutation from requirements mode",
                scope: input_scope.clone(),
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
                system_context: request.system_context,
                owner: &request.owner,
                capability: CapabilityKind::CritiqueContent,
                summary: "critique generated requirements framing",
                scope: input_scope.clone(),
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

        let should_resume_change_execution = match manifest.mode {
            Mode::Change => artifacts.is_empty(),
            Mode::Implementation | Mode::Refactor => {
                execution_continuation_pending(&context, &approvals)
            }
            _ => false,
        };

        if should_resume_change_execution {
            let policy_set = store.load_policy_set(None)?;
            let request = RunRequest {
                mode: manifest.mode,
                risk: manifest.risk,
                zone: manifest.zone,
                system_context: context.system_context,
                classification: manifest.classification.clone(),
                owner: manifest.owner.clone(),
                inputs: self.resume_inputs(&context),
                inline_inputs: Vec::new(),
                excluded_paths: context.excluded_paths.clone(),
                policy_root: None,
                method_root: None,
            };
            return self.execute_change(
                &store,
                request,
                policy_set,
                manifest.to_identity().ok_or_else(|| {
                    EngineError::Validation(format!(
                        "run `{run_id}` is missing identity metadata; cannot resume"
                    ))
                })?,
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

    pub fn status(&self, run: &str) -> Result<StatusSummary, EngineError> {
        let _ = all_mode_profiles();
        let store = WorkspaceStore::new(&self.repo_root);
        let canonical = self.resolve_run(run)?;
        let run = canonical.as_str();
        let manifest = store.load_run_manifest(run)?;
        let state = store.load_run_state(run)?;
        let details = self.collect_run_runtime_details(&store, run, manifest.mode, state.state)?;

        Ok(StatusSummary {
            run: run.to_string(),
            owner: manifest.owner,
            state: format!("{:?}", state.state),
            system_context: details.system_context.map(|context| context.as_str().to_string()),
            invocations_total: details.invocations_total,
            pending_invocation_approvals: details.pending_invocation_approvals,
            validation_independence_satisfied: details.validation_independence_satisfied,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            closure_status: details.closure_status,
            decomposition_scope: details.decomposition_scope,
            closure_findings: details.closure_findings,
            closure_notes: details.closure_notes,
            mode_result: details.mode_result,
            recommended_next_action: details.recommended_next_action,
        })
    }

    pub fn publish(&self, run: &str, to: Option<PathBuf>) -> Result<PublishSummary, EngineError> {
        let canonical = self.resolve_run(run)?;
        publish_run(&self.repo_root, &canonical, to.as_deref())
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

        let normalized = preserve_multiline_summary(&fragments.join("\n"));
        if normalized.is_empty() {
            Err(EngineError::Validation(
                "authored input contained no usable content after normalization".to_string(),
            ))
        } else {
            Ok(normalized)
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

        let absolute_path = discovered.pop().expect("checked for a discovered patch");
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
            mode_result,
            recommended_next_action,
        })
    }

    pub(super) fn load_input_summary(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<String, EngineError> {
        let mut fragments = Vec::new();

        for input in inputs {
            let files = self.collect_content_input_files(input)?;
            if !files.is_empty() {
                for resolved in files {
                    let contents = std::fs::read_to_string(&resolved)?;
                    fragments.push(contents);
                }
            } else {
                fragments.push(input.clone());
            }
        }

        fragments.extend(inline_inputs.iter().cloned());

        let combined = fragments.join("\n");
        let normalized = preserve_multiline_summary(&combined);
        if normalized.is_empty() {
            Ok("Capture the bounded engineering need before implementation accelerates drift."
                .to_string())
        } else {
            Ok(normalized)
        }
    }

    pub(super) fn build_run_context(
        &self,
        request: &RunRequest,
        input_fingerprints: Vec<InputFingerprint>,
        captured_at: OffsetDateTime,
    ) -> RunContext {
        let (implementation_execution, refactor_execution) =
            self.scaffold_mode_execution_context(request);
        let upstream_context = self.scaffold_upstream_context(request);

        RunContext {
            repo_root: self.repo_root.display().to_string(),
            owner: Some(request.owner.clone()),
            inputs: request.merged_input_sources(),
            excluded_paths: request.excluded_paths.clone(),
            input_fingerprints,
            system_context: request.system_context,
            upstream_context,
            implementation_execution,
            refactor_execution,
            backlog_planning: None,
            inline_inputs: request.transient_inline_inputs(),
            captured_at,
        }
    }

    pub(super) fn scaffold_upstream_context(
        &self,
        request: &RunRequest,
    ) -> Option<UpstreamContext> {
        if !matches!(request.mode, Mode::Implementation | Mode::Refactor) {
            return None;
        }

        let summary = self.load_input_summary(&request.inputs, &request.inline_inputs).ok()?;
        let normalized = summary.to_lowercase();
        let feature_slice = extract_marker(&summary, &normalized, "feature slice");
        let primary_upstream_mode = extract_marker(&summary, &normalized, "primary upstream mode");
        let source_refs = extract_first_marker_entries(&summary, &["upstream sources"]);
        let carried_forward_items = extract_first_marker_entries(
            &summary,
            &["carried-forward decisions", "carried-forward invariants"],
        );
        let excluded_upstream_scope =
            extract_marker(&summary, &normalized, "excluded upstream scope");

        if feature_slice.is_none()
            && primary_upstream_mode.is_none()
            && source_refs.is_empty()
            && carried_forward_items.is_empty()
            && excluded_upstream_scope.is_none()
        {
            None
        } else {
            Some(UpstreamContext {
                feature_slice,
                primary_upstream_mode,
                source_refs,
                carried_forward_items,
                excluded_upstream_scope,
            })
        }
    }

    pub(super) fn scaffold_mode_execution_context(
        &self,
        request: &RunRequest,
    ) -> (Option<ImplementationExecutionContext>, Option<RefactorExecutionContext>) {
        let source_refs = request.merged_input_sources();
        let owners =
            if !request.owner.trim().is_empty() { vec![request.owner.clone()] } else { Vec::new() };

        match request.mode {
            Mode::Implementation => (
                Some(ImplementationExecutionContext {
                    plan_sources: source_refs.clone(),
                    mutation_bounds: MutationBounds {
                        declared_paths: Vec::new(),
                        owners,
                        source_refs,
                        expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                    },
                    task_targets: Vec::new(),
                    safety_net: Vec::new(),
                    execution_posture: ExecutionPosture::RecommendationOnly,
                    rollback_expectations: Vec::new(),
                    post_approval_execution_consumed: false,
                }),
                None,
            ),
            Mode::Refactor => (
                None,
                Some(RefactorExecutionContext {
                    preserved_behavior: Vec::new(),
                    structural_rationale: None,
                    refactor_scope: MutationBounds {
                        declared_paths: Vec::new(),
                        owners,
                        source_refs,
                        expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                    },
                    safety_net: Vec::new(),
                    no_feature_addition_target: None,
                    allowed_exceptions: Vec::new(),
                    execution_posture: ExecutionPosture::RecommendationOnly,
                    post_approval_execution_consumed: false,
                }),
            ),
            _ => (None, None),
        }
    }

    pub(super) fn resume_inputs(&self, context: &RunContext) -> Vec<String> {
        context
            .inputs
            .iter()
            .map(|input| {
                context
                    .input_fingerprints
                    .iter()
                    .find(|fingerprint| {
                        fingerprint.source_kind == InputSourceKind::Inline
                            && fingerprint.path == *input
                    })
                    .and_then(|fingerprint| fingerprint.snapshot_ref.as_ref())
                    .map(|snapshot_ref| format!(".canon/{snapshot_ref}"))
                    .unwrap_or_else(|| input.clone())
            })
            .collect()
    }

    pub(super) fn capture_input_fingerprints(
        &self,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<Vec<InputFingerprint>, EngineError> {
        let mut fingerprints = Vec::new();

        for input in inputs {
            for resolved in self.collect_input_files(input)? {
                let metadata = std::fs::metadata(&resolved)?;
                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|duration| duration.as_secs() as i64)
                    .unwrap_or_default();

                fingerprints.push(InputFingerprint {
                    path: self.persisted_input_path(&resolved),
                    source_kind: InputSourceKind::Path,
                    size_bytes: metadata.len(),
                    modified_unix_seconds: modified,
                    content_digest_sha256: Some(sha256_hex(&std::fs::read(&resolved)?)),
                    snapshot_ref: None,
                });
            }
        }

        let captured_at = OffsetDateTime::now_utc().unix_timestamp();
        for (index, inline_input) in inline_inputs.iter().enumerate() {
            fingerprints.push(InputFingerprint {
                path: inline_input_label(index),
                source_kind: InputSourceKind::Inline,
                size_bytes: inline_input.len() as u64,
                modified_unix_seconds: captured_at,
                content_digest_sha256: Some(sha256_hex(inline_input.as_bytes())),
                snapshot_ref: None,
            });
        }

        Ok(fingerprints)
    }

    pub(super) fn clarity_source_inputs(
        &self,
        inputs: &[String],
    ) -> Result<Vec<String>, EngineError> {
        let mut source_inputs = Vec::new();

        for input in inputs {
            let files = self.collect_input_files(input)?;
            if files.is_empty() {
                if !source_inputs.iter().any(|existing| existing == input) {
                    source_inputs.push(input.clone());
                }
                continue;
            }

            for path in files {
                let persisted = self.persisted_input_path(&path);
                if !source_inputs.iter().any(|existing| existing == &persisted) {
                    source_inputs.push(persisted);
                }
            }
        }

        Ok(source_inputs)
    }

    pub(super) fn collect_input_files(&self, input: &str) -> Result<Vec<PathBuf>, EngineError> {
        let resolved = self.resolve_input_path(input);
        if resolved.is_file() {
            return Ok(vec![resolved]);
        }
        if resolved.is_dir() {
            let mut files = Vec::new();
            collect_files_recursively(&resolved, &mut files)?;
            files.sort();
            return Ok(files);
        }

        Ok(Vec::new())
    }

    pub(super) fn collect_content_input_files(
        &self,
        input: &str,
    ) -> Result<Vec<PathBuf>, EngineError> {
        Ok(self
            .collect_input_files(input)?
            .into_iter()
            .filter(|path| !is_known_mutation_payload_file(path))
            .collect())
    }

    pub(super) fn validate_review_authored_input_path(
        &self,
        inputs: &[String],
    ) -> Result<(), EngineError> {
        const REVIEW_INPUT_HINT: &str = "canon-input/review.md or canon-input/review/";

        if inputs.len() != 1 {
            return Err(EngineError::Validation(format!(
                "review requires exactly one authored input at {REVIEW_INPUT_HINT}"
            )));
        }

        let input = &inputs[0];
        let resolved = self.resolve_input_path(input);
        if !resolved.exists() {
            return Err(EngineError::Validation(format!(
                "review input `{input}` was not found from {}; expected {REVIEW_INPUT_HINT}",
                self.repo_root.display()
            )));
        }

        let resolved_canonical = resolved.canonicalize()?;
        let canonical_review_file = self.repo_root.join("canon-input").join("review.md");
        let canonical_review_dir = self.repo_root.join("canon-input").join("review");
        let mut allowed_paths = Vec::new();

        if canonical_review_file.exists() {
            allowed_paths.push(canonical_review_file.canonicalize()?);
        }
        if canonical_review_dir.exists() {
            allowed_paths.push(canonical_review_dir.canonicalize()?);
        }

        if !allowed_paths.iter().any(|path| path == &resolved_canonical) {
            return Err(EngineError::Validation(format!(
                "review accepts only {REVIEW_INPUT_HINT}, not `{input}`"
            )));
        }

        Ok(())
    }

    pub(super) fn validate_authored_input_paths(
        &self,
        mode: Mode,
        inputs: &[String],
    ) -> Result<(), EngineError> {
        if matches!(mode, Mode::PrReview) {
            return Ok(());
        }

        if matches!(mode, Mode::Review) && !inputs.is_empty() {
            self.validate_review_authored_input_path(inputs)?;
        }

        let canon_root = self
            .repo_root
            .join(".canon")
            .exists()
            .then(|| self.repo_root.join(".canon").canonicalize())
            .transpose()?;

        for input in inputs {
            let resolved = self.resolve_input_path(input);
            if !resolved.exists() {
                return Err(EngineError::Validation(format!(
                    "input `{input}` was not found from {}",
                    self.repo_root.display()
                )));
            }

            let canonical = resolved.canonicalize()?;
            if canon_root.as_ref().is_some_and(|root| canonical.starts_with(root)) {
                return Err(EngineError::Validation(format!(
                    "input `{input}` points inside .canon/ and cannot be used as authored input for {}",
                    mode.as_str()
                )));
            }

            let files = self.collect_content_input_files(input)?;
            if resolved.is_dir() && files.is_empty() {
                return Err(EngineError::Validation(format!(
                    "input `{input}` is an empty directory and does not contain authored content"
                )));
            }

            let mut has_usable_content = false;
            for file in files {
                let contents = std::fs::read_to_string(&file)?;
                if !contents.trim().is_empty() {
                    has_usable_content = true;
                    break;
                }
            }

            if !has_usable_content {
                let message = if resolved.is_dir() {
                    format!("input `{input}` expands to files with no usable authored content")
                } else {
                    format!("input `{input}` is empty or whitespace-only")
                };
                return Err(EngineError::Validation(message));
            }
        }

        Ok(())
    }

    pub(super) fn validate_authored_inputs(
        &self,
        mode: Mode,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Result<(), EngineError> {
        if matches!(mode, Mode::PrReview) {
            if !inline_inputs.is_empty() {
                return Err(EngineError::Validation(
                    "pr-review does not support --input-text; pass two refs via --input"
                        .to_string(),
                ));
            }
            return Ok(());
        }

        let source_count = inputs.len() + inline_inputs.len();
        if matches!(mode, Mode::Review) {
            if source_count != 1 {
                return Err(EngineError::Validation(
                    "review requires exactly one authored input at canon-input/review.md or canon-input/review/, or exactly one --input-text value"
                        .to_string(),
                ));
            }
        } else if source_count == 0 {
            return Err(EngineError::Validation(format!(
                "{} requires at least one authored input via --input or --input-text",
                mode.as_str()
            )));
        }

        self.validate_authored_input_paths(mode, inputs)?;
        for (index, inline_input) in inline_inputs.iter().enumerate() {
            if inline_input.trim().is_empty() {
                return Err(EngineError::Validation(format!(
                    "inline input {} for {} is empty or whitespace-only",
                    index + 1,
                    mode.as_str()
                )));
            }
        }

        Ok(())
    }

    pub(super) fn persisted_input_path(&self, resolved: &Path) -> String {
        resolved
            .strip_prefix(&self.repo_root)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| resolved.display().to_string())
    }

    pub(super) fn resolve_input_path(&self, input: &str) -> PathBuf {
        let path = PathBuf::from(input);
        if path.is_absolute() { path } else { self.repo_root.join(path) }
    }

    pub(super) fn auto_bind_canonical_mode_inputs(
        &self,
        mode: Mode,
        inputs: &[String],
        inline_inputs: &[String],
    ) -> Vec<String> {
        if !inputs.is_empty() || !inline_inputs.is_empty() {
            return inputs.to_vec();
        }

        let Some((file_name, dir_name)) = canonical_mode_input_binding(mode) else {
            return Vec::new();
        };

        let canonical_root = self.repo_root.join("canon-input");
        let canonical_dir = canonical_root.join(dir_name);
        if canonical_dir.exists() {
            return vec![format!("canon-input/{dir_name}")];
        }

        let canonical_file = canonical_root.join(file_name);
        if canonical_file.exists() {
            return vec![format!("canon-input/{file_name}")];
        }

        Vec::new()
    }

    pub(super) fn load_pr_review_refs(
        &self,
        inputs: &[String],
    ) -> Result<(String, String), EngineError> {
        if inputs.len() < 2 {
            return Err(EngineError::Validation(
                "pr-review requires two inputs: <base-ref> <head-ref>".to_string(),
            ));
        }

        Ok((inputs[0].clone(), inputs[1].clone()))
    }

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
}

fn collect_files_recursively(directory: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursively(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }

    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut encoded = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    encoded
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

fn inline_input_label(index: usize) -> String {
    format!("inline-input-{:02}.md", index + 1)
}

fn process_failure_excerpt(stdout: &str, stderr: &str) -> String {
    let stderr = stderr.trim();
    if !stderr.is_empty() {
        return stderr.to_string();
    }

    let stdout = stdout.trim();
    if !stdout.is_empty() {
        return stdout.to_string();
    }

    "no process output captured".to_string()
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

fn canonical_mode_input_binding(mode: Mode) -> Option<(&'static str, &'static str)> {
    match mode {
        Mode::Backlog => Some(("backlog.md", "backlog")),
        Mode::Incident => Some(("incident.md", "incident")),
        Mode::Implementation => Some(("implementation.md", "implementation")),
        Mode::Migration => Some(("migration.md", "migration")),
        Mode::SystemAssessment => Some(("system-assessment.md", "system-assessment")),
        Mode::SecurityAssessment => Some(("security-assessment.md", "security-assessment")),
        Mode::SupplyChainAnalysis => Some(("supply-chain-analysis.md", "supply-chain-analysis")),
        Mode::Refactor => Some(("refactor.md", "refactor")),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
