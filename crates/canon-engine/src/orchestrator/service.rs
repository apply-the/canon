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
    render_implementation_artifact, render_pr_review_artifact, render_refactor_artifact,
    render_requirements_artifact_from_evidence, render_review_artifact,
    render_system_shaping_artifact, render_verification_artifact,
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
    pub recommended_focus: String,
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
struct RequirementsRequestSpec<'a> {
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
struct GovernedRequestSpec<'a> {
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
    system_context: Option<SystemContext>,
    invocations_total: usize,
    invocations_denied: usize,
    pending_invocation_approvals: usize,
    validation_independence_satisfied: bool,
    blocking_classification: Option<String>,
    blocked_gates: Vec<GateInspectSummary>,
    approval_targets: Vec<String>,
    artifact_paths: Vec<String>,
    mode_result: Option<ModeResultSummary>,
    recommended_next_action: Option<RecommendedActionSummary>,
}

#[derive(Debug, Clone)]
struct RequirementsBrief {
    problem: String,
    outcome: String,
    constraints: Vec<String>,
    tradeoffs: Vec<String>,
    out_of_scope: Vec<String>,
    open_questions: Vec<String>,
    source_refs: Vec<String>,
}

impl RequirementsBrief {
    fn from_context(context_summary: String, source_refs: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let problem =
            extract_context_marker(&context_summary, &normalized, &["problem", "intent", "goal"])
                .or_else(|| extract_context_marker(&context_summary, &normalized, &["abstract"]))
                .or_else(|| extract_context_marker(&context_summary, &normalized, &["subject"]))
                .map(|value| condense_context_block(&value, 420))
                .unwrap_or_else(|| {
                    "NOT CAPTURED - Provide a `## Problem` section in the requirements input."
                        .to_string()
                });
        let outcome = extract_context_marker(
            &context_summary,
            &normalized,
            &["outcome", "desired outcome", "success signal", "objective"],
        )
        .map(|value| condense_context_block(&value, 320))
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Outcome` section in the requirements input.".to_string()
        });

        let constraints = extract_context_list(
            &context_summary,
            &normalized,
            &["constraints", "constraint", "non-negotiables"],
        );
        let tradeoffs =
            extract_context_list(&context_summary, &normalized, &["tradeoffs", "tradeoff"]);
        let out_of_scope = extract_context_list(
            &context_summary,
            &normalized,
            &["out of scope", "out-of-scope", "scope cuts", "excluded"],
        );
        let open_questions = extract_context_list(
            &context_summary,
            &normalized,
            &["open questions", "questions", "unknowns", "risks"],
        );

        let mut brief = Self {
            problem,
            outcome,
            constraints: default_list(
                constraints,
                "NOT CAPTURED - Provide a `## Constraints` section in the requirements input.",
            ),
            tradeoffs: default_list(
                tradeoffs,
                "NOT CAPTURED - Provide a `## Tradeoffs` section in the requirements input.",
            ),
            out_of_scope: default_list(
                out_of_scope,
                "NOT CAPTURED - Provide a `## Out of Scope` or `## Scope Cuts` section in the requirements input.",
            ),
            open_questions,
            source_refs: source_refs.iter().map(ToString::to_string).collect(),
        };

        if brief.open_questions.is_empty() {
            brief.open_questions.extend(
                prioritized_requirements_clarification_questions(&brief, &context_summary)
                    .into_iter()
                    .take(4)
                    .map(|question| question.prompt),
            );
        }

        if brief.open_questions.is_empty() {
            brief
                .open_questions
                .push("Which downstream mode should consume this packet first?".to_string());
        }

        brief
    }

    fn summary(&self) -> String {
        let mut lines = vec![
            format!("Problem framing: {}", truncate_context_excerpt(&self.problem, 180)),
            format!("Desired outcome: {}", truncate_context_excerpt(&self.outcome, 180)),
        ];

        if !self.source_refs.is_empty() {
            lines.push(format!("Source inputs: {}", self.source_refs.join(", ")));
        }

        lines.join("\n")
    }
}

#[derive(Debug, Clone)]
struct DiscoveryBrief {
    context_summary: String,
    problem: String,
    constraints: String,
    repo_focus: String,
    unknowns: String,
    next_phase: String,
}

impl DiscoveryBrief {
    fn from_context(context_summary: String, repo_surfaces: &[String]) -> Self {
        let normalized = context_summary.to_lowercase();
        let problem =
            extract_context_marker(&context_summary, &normalized, &["problem", "problem domain"])
                .unwrap_or_else(|| {
                    let line = first_meaningful_line(&context_summary);
                    if line.contains("Bound the problem to") {
                        "NOT CAPTURED - Provide a `## Problem` section in the discovery brief."
                            .to_string()
                    } else {
                        line
                    }
                });
        let constraints = extract_context_marker(
            &context_summary,
            &normalized,
            &["constraints", "constraint", "boundary"],
        )
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide a `## Constraints` section in the discovery brief.".to_string()
        });
        let repo_focus = extract_context_marker(
            &context_summary,
            &normalized,
            &["repo focus", "repository focus", "current state", "system slice"],
        )
        .unwrap_or_else(|| {
            if repo_surfaces.is_empty() {
                "NOT CAPTURED - Provide a `## Repo Focus` section in the discovery brief."
                    .to_string()
            } else {
                format!(
                    "Ground discovery in these repository surfaces: {}",
                    repo_surfaces.join(", ")
                )
            }
        });
        let unknowns = extract_context_marker(
            &context_summary,
            &normalized,
            &["unknowns", "open questions", "risks"],
        )
        .unwrap_or_else(|| {
            "NOT CAPTURED - Provide an `## Unknowns` section in the discovery brief.".to_string()
        });
        let next_phase = extract_context_marker(
            &context_summary,
            &normalized,
            &["next phase", "handoff", "translation trigger"],
        )
        .unwrap_or_else(|| {
            let inferred = infer_discovery_next_phase(&context_summary);
            if inferred.contains("Translate this packet") {
                "NOT CAPTURED - Provide a `## Next Phase` section in the discovery brief."
                    .to_string()
            } else {
                inferred
            }
        });

        Self { context_summary, problem, constraints, repo_focus, unknowns, next_phase }
    }

    fn generation_prompt(&self, repo_surfaces: &[String]) -> String {
        format!(
            "# Discovery Brief\n\n## Problem\n{}\n\n## Constraints\n{}\n\n## Repo Focus\n{}\n\n## Repo Surface\n{}\n\n## Unknowns\n{}\n\n## Next Phase\n{}",
            self.problem,
            self.constraints,
            self.repo_focus,
            render_repo_surface_block(repo_surfaces),
            self.unknowns,
            self.next_phase,
        )
    }

    fn critique_prompt(&self, generation_summary: &str, repo_surfaces: &[String]) -> String {
        format!(
            "# Discovery Critique Target\n\n## Context Summary\n{}\n\n## Repo Surface\n{}\n\n## Generated Framing\n{}\n\n## Challenge\nCheck whether the generated framing stays anchored to the repository surfaces, preserves the stated constraints, and points to a concrete next governed mode.",
            self.context_summary,
            render_repo_surface_block(repo_surfaces),
            generation_summary,
        )
    }

    fn evidence_backed_summary(
        &self,
        repo_surfaces: &[String],
        generation_summary: &str,
        critique_summary: &str,
        validation_summary: &str,
    ) -> String {
        format!(
            "# Discovery Brief\n\n## Problem\n{}\n\n## Constraints\n{}\n\n## Repo Focus\n{}\n\n## Repo Surface\n{}\n\n## Unknowns\n{}\n\n## Next Phase\n{}\n\nGenerated framing: {}\n\nCritique evidence: {}\n\nValidation evidence: {}",
            self.problem,
            self.constraints,
            self.repo_focus,
            render_repo_surface_block(repo_surfaces),
            self.unknowns,
            self.next_phase,
            generation_summary,
            critique_summary,
            validation_summary,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GitConfigScope {
    Local,
    Global,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthoredMutationPatch {
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
            Mode::Implementation => self.run_implementation(&store, request, policy_set),
            Mode::Refactor => self.run_refactor(&store, request, policy_set),
            Mode::Architecture => self.run_architecture(&store, request, policy_set),
            Mode::Review => self.run_review(&store, request, policy_set),
            Mode::Verification => self.run_verification(&store, request, policy_set),
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
                let run_context = store.load_run_context(&run_id).ok();
                let approvals = store.load_approval_records(&run_id).unwrap_or_default();
                let system_context =
                    run_context.as_ref().and_then(|context| context.system_context);
                let upstream_context =
                    run_context.as_ref().and_then(|context| context.upstream_context.as_ref());
                let entries = store
                    .load_evidence_bundle(&run_id)?
                    .map(|evidence| {
                        vec![InspectEntry::Evidence(EvidenceInspectSummary {
                            execution_posture: resolved_execution_posture_label(
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

    fn inspect_risk_zone(
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

    fn inspect_clarity(
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
            other => Err(EngineError::UnsupportedInspectTarget(format!(
                "clarity inspection is currently available only for requirements and discovery, not {}",
                other.as_str()
            ))),
        }
    }

    fn inspect_requirements_clarity(
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

    fn inspect_discovery_clarity(
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
            mode_result: details.mode_result,
            recommended_next_action: details.recommended_next_action,
        })
    }

    pub fn publish(&self, run: &str, to: Option<PathBuf>) -> Result<PublishSummary, EngineError> {
        let canonical = self.resolve_run(run)?;
        publish_run(&self.repo_root, &canonical, to.as_deref())
    }

    fn run_requirements(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.requirements_request(RequirementsRequestSpec {
            run_id: &run_id,
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
            run_id: &run_id,
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
            run_id: &run_id,
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
                    uuid: Some(run_uuid.clone()),
                    short_id: Some(run_short_id.clone()),
                    slug: None,
                    title: None,
                    mode: request.mode,
                    risk: request.risk,
                    zone: request.zone,
                    system_context: request.system_context,
                    classification: request.classification.clone(),
                    owner: request.owner.clone(),
                    created_at: now,
                },
                context: self.build_run_context(&request, input_fingerprints, now),
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
                uuid: Some(run_uuid.clone()),
                owner: bundle.run.owner.clone(),
                mode: bundle.run.mode.as_str().to_string(),
                risk: bundle.run.risk.as_str().to_string(),
                zone: bundle.run.zone.as_str().to_string(),
                system_context: bundle
                    .run
                    .system_context
                    .map(|context| context.as_str().to_string()),
                state: format!("{:?}", bundle.state.state),
                artifact_count: 0,
                invocations_total: details.invocations_total,
                invocations_denied: details.invocations_denied,
                invocations_pending_approval: details.pending_invocation_approvals,
                blocking_classification: details.blocking_classification,
                blocked_gates: details.blocked_gates,
                approval_targets: details.approval_targets,
                artifact_paths: details.artifact_paths,
                mode_result: details.mode_result,
                recommended_next_action: details.recommended_next_action,
            });
        }

        let requirements_brief = RequirementsBrief::from_context(context_summary, &input_scope);
        let brief_summary = requirements_brief.summary();
        let copilot = CopilotCliAdapter;
        let generation_output = copilot.generate_requirements(RequirementsGenerationInput {
            problem: &requirements_brief.problem,
            outcome: &requirements_brief.outcome,
            constraints: &requirements_brief.constraints,
            tradeoffs: &requirements_brief.tradeoffs,
            out_of_scope: &requirements_brief.out_of_scope,
            open_questions: &requirements_brief.open_questions,
            source_refs: &requirements_brief.source_refs,
        });
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
            system_context: request.system_context,
            owner: &request.owner,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique generated requirements framing",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot.critique_requirements(
            &requirements_brief.problem,
            &requirements_brief.outcome,
            &requirements_brief.constraints,
            &requirements_brief.out_of_scope,
            &requirements_brief.open_questions,
            &generation_output.summary,
        );
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
                    &brief_summary,
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
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
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
            uuid: Some(run_uuid.clone()),
            owner: bundle.run.owner.clone(),
            mode: bundle.run.mode.as_str().to_string(),
            risk: bundle.run.risk.as_str().to_string(),
            zone: bundle.run.zone.as_str().to_string(),
            system_context: bundle.run.system_context.map(|context| context.as_str().to_string()),
            state: format!("{:?}", bundle.state.state),
            artifact_count: bundle.artifacts.len(),
            invocations_total: details.invocations_total,
            invocations_denied: details.invocations_denied,
            invocations_pending_approval: details.pending_invocation_approvals,
            blocking_classification: details.blocking_classification,
            blocked_gates: details.blocked_gates,
            approval_targets: details.approval_targets,
            artifact_paths: details.artifact_paths,
            mode_result: details.mode_result,
            recommended_next_action: details.recommended_next_action,
        })
    }

    fn run_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        self.execute_change(store, request, policy_set, identity, Vec::new())
    }

    fn run_implementation(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        self.execute_change(store, request, policy_set, identity, Vec::new())
    }

    fn run_refactor(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        self.execute_change(store, request, policy_set, identity, Vec::new())
    }

    fn run_discovery(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Discovery,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture discovery context and problem framing",
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;
        let repo_surfaces = self.scan_workspace_surface()?;
        let discovery_brief = DiscoveryBrief::from_context(context_summary, &repo_surfaces);
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured discovery context from {} input(s) and {} repository surface(s).",
                    request.authored_input_count(),
                    repo_surfaces.len()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Discovery,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded discovery analysis",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let copilot = CopilotCliAdapter;
        let generation_output =
            copilot.generate(&discovery_brief.generation_prompt(&repo_surfaces));
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

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Discovery,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique discovery framing against repository evidence",
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = copilot
            .critique(&discovery_brief.critique_prompt(&generation_output.summary, &repo_surfaces));
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

        let validation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Discovery,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: "validate discovery framing against repository context",
            scope: input_scope.clone(),
        });
        let validation_decision =
            invocation_runtime::evaluate_request_policy(&validation_request, &policy_set);
        let (validation_summary, validation_attempt) =
            self.change_validation_attempt(&validation_request)?;

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
        let evidence_backed_summary = discovery_brief.evidence_backed_summary(
            &repo_surfaces,
            &generation_output.summary,
            &critique_output.summary,
            &validation_summary,
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
                            critique_request.request_id.clone(),
                            validation_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_discovery_artifact(
                    &requirement.file_name,
                    &evidence_backed_summary,
                ),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_discovery_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::DiscoveryGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                validation_independence_satisfied: validation_path.independence.sufficient,
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "discovery",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        for record in &mut verification_records {
            record.request_ids = vec![
                generation_request.request_id.clone(),
                critique_request.request_id.clone(),
                validation_request.request_id.clone(),
            ];
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
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    validation_request.request_id
                ),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
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
                    request: validation_request,
                    decision: validation_decision,
                    attempts: vec![validation_attempt],
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

    fn run_system_shaping(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::SystemShaping,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture system-shaping context and bounded intent",
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
                    "Captured system-shaping context from {} input(s).",
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::SystemShaping,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded system-shaping analysis",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
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

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::SystemShaping,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique bounded system-shaping analysis",
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
            path_id: format!("validation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                critique_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", critique_request.request_id),
                    request_ids: vec![critique_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::AiVendorFamily],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        critique_request.request_id
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
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_system_shaping_artifact(
                    &requirement.file_name,
                    &context_summary,
                    &generation_output.summary,
                    &critique_output.summary,
                ),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_system_shaping_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::SystemShapingGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "system-shaping",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
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
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
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
                    attempts: vec![generation_attempt],
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

    fn run_architecture(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Architecture,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: "capture architecture context and structural dilemma",
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
                    "Captured architecture context from {} input(s).",
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Architecture,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: "generate bounded architecture analysis",
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
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

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::Architecture,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique bounded architecture decisions and invariants",
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
            path_id: format!("validation:{}", critique_request.request_id),
            request_ids: vec![critique_request.request_id.clone()],
            lineage_classes: vec![LineageClass::AiVendorFamily],
            verification_refs: vec![format!(
                "runs/{run_id}/invocations/{}/attempt-01.toml",
                critique_request.request_id
            )],
            independence: evidence_builder::assess_validation_independence(
                &generation_path,
                &ValidationPath {
                    path_id: format!("validation:{}", critique_request.request_id),
                    request_ids: vec![critique_request.request_id.clone()],
                    lineage_classes: vec![LineageClass::AiVendorFamily],
                    verification_refs: vec![format!(
                        "runs/{run_id}/invocations/{}/attempt-01.toml",
                        critique_request.request_id
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
                            context_request.request_id.clone(),
                            generation_request.request_id.clone(),
                            critique_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: crate::domain::execution::EvidenceDisposition::Supporting,
                    }),
                },
                contents: render_architecture_artifact(
                    &requirement.file_name,
                    &context_summary,
                    &generation_output.summary,
                    &critique_output.summary,
                ),
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = gatekeeper::evaluate_architecture_gates(
            &artifact_contract,
            &gate_inputs,
            gatekeeper::ArchitectureGateContext {
                owner: &request.owner,
                risk: request.risk,
                zone: request.zone,
                approvals: &approvals,
                evidence_complete: true,
            },
        );
        let state = run_state_from_gates(&gates);

        let mut verification_records = verification_runner::analysis_verification_records(
            "architecture",
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
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
            denied_invocations: Vec::new(),
            trace_refs: vec![format!("traces/{run_id}.jsonl")],
            artifact_refs: artifact_paths.clone(),
            decision_refs: vec![
                format!("runs/{run_id}/invocations/{}/decision.toml", context_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    generation_request.request_id
                ),
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
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
                    attempts: vec![generation_attempt],
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

    fn run_review(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        self.run_review_like_mode(store, request, policy_set)
    }

    fn run_verification(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        self.run_review_like_mode(store, request, policy_set)
    }

    fn run_review_like_mode(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
    ) -> Result<RunSummary, EngineError> {
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");
        let context_summary =
            self.read_requirements_context(&request.inputs, &request.inline_inputs)?;

        let (
            context_request_summary,
            generation_request_summary,
            critique_request_summary,
            validation_request_summary,
        ) = match request.mode {
            Mode::Review => (
                "capture review packet context and authored evidence target",
                "generate bounded review packet for a non-PR change package",
                "critique review findings, evidence gaps, and disposition posture",
                "validate review evidence against the repository surface",
            ),
            Mode::Verification => (
                "capture verification target context and authored evidence basis",
                "generate bounded verification challenge packet",
                "critique supported claims, contradictions, and unresolved findings",
                "validate verification evidence against the repository surface",
            ),
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: context_request_summary,
            scope: input_scope.clone(),
        });
        let context_decision =
            invocation_runtime::evaluate_request_policy(&context_request, &policy_set);
        let context_attempt = self.completed_attempt(
            &context_request,
            1,
            "filesystem",
            ToolOutcome {
                kind: ToolOutcomeKind::Succeeded,
                summary: format!(
                    "Captured {} context from {} input(s).",
                    request.mode.as_str(),
                    request.authored_input_count()
                ),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: generation_request_summary,
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let copilot = CopilotCliAdapter;
        let generation_output = match request.mode {
            Mode::Review => copilot.generate_review(&context_summary),
            Mode::Verification => copilot.generate_verification(&context_summary),
            _ => unreachable!("unsupported review-like mode"),
        };
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

        let critique_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: critique_request_summary,
            scope: input_scope.clone(),
        });
        let critique_decision =
            invocation_runtime::evaluate_request_policy(&critique_request, &policy_set);
        let critique_output = match request.mode {
            Mode::Review => copilot.critique_review(&generation_output.summary),
            Mode::Verification => copilot.critique_verification(&generation_output.summary),
            _ => unreachable!("unsupported review-like mode"),
        };
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

        let validation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: validation_request_summary,
            scope: input_scope.clone(),
        });
        let validation_decision =
            invocation_runtime::evaluate_request_policy(&validation_request, &policy_set);
        let (validation_summary, validation_attempt) =
            self.change_validation_attempt(&validation_request)?;

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

        let artifact_disposition = match request.mode {
            Mode::Review => crate::domain::execution::EvidenceDisposition::NeedsDisposition,
            Mode::Verification => crate::domain::execution::EvidenceDisposition::Supporting,
            _ => unreachable!("unsupported review-like mode"),
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
                            validation_request.request_id.clone(),
                        ],
                        evidence_bundle: Some(evidence_path.clone()),
                        disposition: artifact_disposition,
                    }),
                },
                contents: match request.mode {
                    Mode::Review => render_review_artifact(
                        &requirement.file_name,
                        &context_summary,
                        &generation_output.summary,
                        &critique_output.summary,
                        &validation_summary,
                    ),
                    Mode::Verification => render_verification_artifact(
                        &requirement.file_name,
                        &context_summary,
                        &generation_output.summary,
                        &critique_output.summary,
                        &validation_summary,
                    ),
                    _ => unreachable!("unsupported review-like mode"),
                },
            })
            .collect::<Vec<_>>();

        let approvals = Vec::new();
        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let state = match request.mode {
            Mode::Review => {
                let gates = gatekeeper::evaluate_review_gates(
                    &artifact_contract,
                    &gate_inputs,
                    gatekeeper::ReviewGateContext {
                        owner: &request.owner,
                        risk: request.risk,
                        zone: request.zone,
                        approvals: &approvals,
                        evidence_complete: true,
                    },
                );
                let state = run_state_from_gates(&gates);
                (state, gates)
            }
            Mode::Verification => {
                let gates = gatekeeper::evaluate_verification_gates(
                    &artifact_contract,
                    &gate_inputs,
                    gatekeeper::VerificationGateContext {
                        owner: &request.owner,
                        risk: request.risk,
                        zone: request.zone,
                        approvals: &approvals,
                        validation_independence_satisfied: validation_path.independence.sufficient,
                        evidence_complete: true,
                    },
                );
                let state = run_state_from_gates(&gates);
                (state, gates)
            }
            _ => unreachable!("unsupported review-like mode"),
        };
        let (state, gates) = state;

        let mut verification_records = verification_runner::analysis_verification_records(
            request.mode.as_str(),
            &artifact_contract.required_verification_layers,
            &artifact_paths,
        );
        verification_runner::attach_runtime_lineage(
            &mut verification_records,
            &[
                generation_request.request_id.clone(),
                critique_request.request_id.clone(),
                validation_request.request_id.clone(),
            ],
            &validation_path.path_id,
            &evidence_path,
        );

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
                format!("runs/{run_id}/invocations/{}/decision.toml", critique_request.request_id),
                format!(
                    "runs/{run_id}/invocations/{}/decision.toml",
                    validation_request.request_id
                ),
            ],
            approval_refs: Vec::new(),
        };

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
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
                    request: validation_request,
                    decision: validation_decision,
                    attempts: vec![validation_attempt],
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

    fn execute_change(
        &self,
        store: &WorkspaceStore,
        request: RunRequest,
        policy_set: crate::domain::policy::PolicySet,
        identity: RunIdentity,
        approvals: Vec<ApprovalRecord>,
    ) -> Result<RunSummary, EngineError> {
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let brief_summary = self.load_input_summary(&request.inputs, &request.inline_inputs)?;
        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let (
            context_request_summary,
            context_attempt_summary,
            generation_request_summary,
            validation_request_summary,
            declared_execution_scope,
            mutation_summary,
        ) = match request.mode {
            Mode::Change => {
                let declared_change_surface = extract_change_surface_entries(&brief_summary);
                let mutation_summary = if declared_change_surface.is_empty() {
                    "propose bounded legacy transformation without mutating the workspace"
                        .to_string()
                } else {
                    format!(
                        "propose bounded legacy transformation within declared change surface: {}",
                        declared_change_surface.join(", ")
                    )
                };
                (
                    "capture change brief and repository context",
                    "Captured change brief and bounded repository context.",
                    "generate bounded change framing",
                    "validate change framing against repository context",
                    declared_change_surface,
                    mutation_summary,
                )
            }
            Mode::Implementation => {
                let declared_mutation_bounds = extract_execution_scope_entries(
                    &brief_summary,
                    &["allowed paths", "mutation bounds"],
                );
                let mutation_summary = if declared_mutation_bounds.is_empty() {
                    "propose bounded implementation guidance without mutating the workspace"
                        .to_string()
                } else {
                    format!(
                        "propose bounded implementation guidance within declared mutation bounds: {}",
                        declared_mutation_bounds.join(", ")
                    )
                };
                (
                    "capture implementation brief and bounded repository context",
                    "Captured implementation brief and bounded repository context.",
                    "generate bounded implementation packet",
                    "validate implementation safety-net evidence against repository context",
                    declared_mutation_bounds,
                    mutation_summary,
                )
            }
            Mode::Refactor => {
                let declared_refactor_scope = extract_execution_scope_entries(
                    &brief_summary,
                    &["allowed paths", "refactor scope"],
                );
                let mutation_summary = if declared_refactor_scope.is_empty() {
                    "propose bounded structural refactor guidance without mutating the workspace"
                        .to_string()
                } else {
                    format!(
                        "propose bounded structural refactor guidance within declared refactor scope: {}",
                        declared_refactor_scope.join(", ")
                    )
                };
                (
                    "capture refactor brief and bounded repository context",
                    "Captured refactor brief and bounded repository context.",
                    "generate bounded refactor packet",
                    "validate refactor preservation evidence against repository context",
                    declared_refactor_scope,
                    mutation_summary,
                )
            }
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };

        let context_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Filesystem,
            capability: CapabilityKind::ReadRepository,
            summary: context_request_summary,
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
                summary: context_attempt_summary.to_string(),
                exit_code: Some(0),
                payload_refs: Vec::new(),
                candidate_artifacts: Vec::new(),
                recorded_at: OffsetDateTime::now_utc(),
            },
        );

        let generation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::GenerateContent,
            summary: generation_request_summary,
            scope: input_scope.clone(),
        });
        let generation_decision =
            invocation_runtime::evaluate_request_policy(&generation_request, &policy_set);
        let generation_policy_attempt =
            self.policy_decision_attempt(&generation_request, &generation_decision);

        let mutation_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ExecuteBoundedTransformation,
            summary: &mutation_summary,
            scope: declared_execution_scope.clone(),
        });
        let mut mutation_decision =
            invocation_runtime::evaluate_request_policy(&mutation_request, &policy_set);

        let approved_generation = approvals.iter().any(|approval| {
            approval.matches_invocation(&generation_request.request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });
        let approved_mutation = approvals.iter().any(|approval| {
            approval.matches_invocation(&mutation_request.request_id)
                && matches!(approval.decision, ApprovalDecision::Approve)
        });
        let execution_gate_approved = execution_gate_is_approved(&approvals);
        let mutation_patch = if matches!(request.mode, Mode::Implementation | Mode::Refactor) {
            self.locate_authored_mutation_patch(&request.inputs, &declared_execution_scope)?
        } else {
            None
        };

        if execution_gate_approved
            && mutation_patch.is_some()
            && matches!(
                mutation_decision.kind,
                PolicyDecisionKind::Allow | PolicyDecisionKind::AllowConstrained
            )
            && !mutation_decision.constraints.patch_disabled
        {
            mutation_decision.constraints.recommendation_only = false;
            mutation_decision.requires_approval = false;
            mutation_decision.rationale = approved_execution_mutation_rationale(
                request.mode,
                &declared_execution_scope,
                mutation_patch
                    .as_ref()
                    .map(|patch| patch.relative_path.as_str())
                    .unwrap_or_default(),
            );
        }

        let mutation_attempt = if matches!(
            mutation_decision.kind,
            PolicyDecisionKind::Allow | PolicyDecisionKind::AllowConstrained
        ) && !mutation_decision.constraints.recommendation_only
            && !mutation_decision.constraints.patch_disabled
        {
            match mutation_patch.as_ref() {
                Some(patch) => self.apply_authored_mutation_patch(&mutation_request, patch)?,
                None => self.policy_decision_attempt(&mutation_request, &mutation_decision),
            }
        } else {
            self.policy_decision_attempt(&mutation_request, &mutation_decision)
        };

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
                approval_refs: approval_record_refs(&run_id, &approvals),
            };

            let bundle = PersistedRunBundle {
                run: RunManifest {
                    run_id: run_id.clone(),
                    uuid: Some(run_uuid.clone()),
                    short_id: Some(run_short_id.clone()),
                    slug: None,
                    title: None,
                    mode: request.mode,
                    risk: request.risk,
                    zone: request.zone,
                    system_context: request.system_context,
                    classification: request.classification.clone(),
                    owner: request.owner.clone(),
                    created_at: now,
                },
                context: self.build_run_context(&request, input_fingerprints, now),
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
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::ValidateWithTool,
            summary: validation_request_summary,
            scope: input_scope.clone(),
        });
        let validation_decision =
            invocation_runtime::evaluate_request_policy(&validation_request, &policy_set);
        let (validation_summary, validation_attempt) =
            self.change_validation_attempt(&validation_request)?;

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
        let default_owner = self.resolve_owner("");
        let artifacts = artifact_contract
            .artifact_requirements
            .iter()
            .map(|requirement| {
                let contents = match request.mode {
                    Mode::Change => render_change_artifact(
                        &requirement.file_name,
                        &evidence_backed_summary,
                        &default_owner,
                    ),
                    Mode::Implementation => render_implementation_artifact(
                        &requirement.file_name,
                        &evidence_backed_summary,
                        &default_owner,
                    ),
                    Mode::Refactor => render_refactor_artifact(
                        &requirement.file_name,
                        &evidence_backed_summary,
                        &default_owner,
                    ),
                    other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
                };

                Ok(PersistedArtifact {
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
                    contents,
                })
            })
            .collect::<Result<Vec<_>, EngineError>>()?;

        let gate_inputs = artifacts
            .iter()
            .map(|artifact| (artifact.record.file_name.clone(), artifact.contents.clone()))
            .collect::<Vec<_>>();
        let gates = match request.mode {
            Mode::Change => gatekeeper::evaluate_change_gates(
                &artifact_contract,
                &gate_inputs,
                gatekeeper::ChangeGateContext {
                    owner: &request.owner,
                    risk: request.risk,
                    zone: request.zone,
                    approvals: &approvals,
                    system_context: request.system_context,
                    validation_independence_satisfied: validation_path.independence.sufficient,
                    evidence_complete: true,
                },
            ),
            Mode::Implementation => gatekeeper::evaluate_implementation_gates(
                &artifact_contract,
                &gate_inputs,
                gatekeeper::ImplementationGateContext {
                    owner: &request.owner,
                    risk: request.risk,
                    zone: request.zone,
                    approvals: &approvals,
                    system_context: request.system_context,
                    validation_independence_satisfied: validation_path.independence.sufficient,
                    evidence_complete: true,
                },
            ),
            Mode::Refactor => gatekeeper::evaluate_refactor_gates(
                &artifact_contract,
                &gate_inputs,
                gatekeeper::RefactorGateContext {
                    owner: &request.owner,
                    risk: request.risk,
                    zone: request.zone,
                    approvals: &approvals,
                    system_context: request.system_context,
                    validation_independence_satisfied: validation_path.independence.sufficient,
                    evidence_complete: true,
                },
            ),
            other => return Err(EngineError::UnsupportedMode(other.as_str().to_string())),
        };
        let mut state = run_state_from_gates(&gates);
        if mutation_decision.requires_approval
            && !approved_mutation
            && !matches!(state, RunState::Blocked)
        {
            state = RunState::AwaitingApproval;
        }

        let mut verification_records = verification_runner::change_verification_records(
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
            approval_refs: approval_record_refs(&run_id, &approvals),
        };

        let generation_attempts =
            if matches!(generation_decision.kind, PolicyDecisionKind::NeedsApproval) {
                vec![generation_policy_attempt, generation_attempt]
            } else {
                vec![generation_attempt]
            };

        let mut run_context = self.build_run_context(&request, input_fingerprints, now);
        if matches!(mutation_attempt.outcome.kind, ToolOutcomeKind::Succeeded) {
            set_execution_posture(&mut run_context, ExecutionPosture::Mutating);
        }
        if execution_gate_approved {
            set_post_approval_execution_consumed(&mut run_context, true);
        }

        let bundle = PersistedRunBundle {
            run: RunManifest {
                run_id: run_id.clone(),
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: run_context,
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
        let identity = RunIdentity::new_now_v7();
        let now = identity.created_at;
        let run_id = identity.run_id.clone();
        let run_uuid = identity.uuid.as_simple().to_string();
        let run_short_id = identity.short_id.clone();
        let mut artifact_contract = contract_for_mode(request.mode);
        classifier::apply_verification_layers(&policy_set, request.risk, &mut artifact_contract);

        let (base_ref, head_ref) = self.load_pr_review_refs(&request.inputs)?;
        let is_worktree = head_ref == "WORKTREE";
        let input_fingerprints =
            self.capture_input_fingerprints(&request.inputs, &request.inline_inputs)?;
        let input_scope = request.merged_input_sources();
        let evidence_path = format!("runs/{run_id}/evidence.toml");

        let diff_request = self.governed_request(GovernedRequestSpec {
            run_id: &run_id,
            mode: Mode::PrReview,
            risk: request.risk,
            zone: request.zone,
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::Shell,
            capability: CapabilityKind::InspectDiff,
            summary: "inspect diff surfaces and bounded patch context",
            scope: input_scope.clone(),
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
            system_context: request.system_context,
            owner: &request.owner,
            adapter: canon_adapters::AdapterKind::CopilotCli,
            capability: CapabilityKind::CritiqueContent,
            summary: "critique the reviewed diff and preserve reviewer-facing evidence",
            scope: input_scope.clone(),
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
                uuid: Some(run_uuid.clone()),
                short_id: Some(run_short_id.clone()),
                slug: None,
                title: None,
                mode: request.mode,
                risk: request.risk,
                zone: request.zone,
                system_context: request.system_context,
                classification: request.classification.clone(),
                owner: request.owner.clone(),
                created_at: now,
            },
            context: self.build_run_context(&request, input_fingerprints, now),
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
            system_context: spec.system_context,
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

    fn read_requirements_context(
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

    fn change_validation_attempt(
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

    fn locate_authored_mutation_patch(
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

    fn apply_authored_mutation_patch(
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
            mode_result: details.mode_result,
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
        let context = store.load_run_context(run_id).ok();
        let system_context = context.as_ref().and_then(|context| context.system_context);

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
            mode_result,
            recommended_next_action,
        })
    }

    fn load_input_summary(
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

    fn build_run_context(
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
            inline_inputs: request.transient_inline_inputs(),
            captured_at,
        }
    }

    fn scaffold_upstream_context(&self, request: &RunRequest) -> Option<UpstreamContext> {
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

    fn scaffold_mode_execution_context(
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

    fn resume_inputs(&self, context: &RunContext) -> Vec<String> {
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

    fn capture_input_fingerprints(
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

    fn clarity_source_inputs(&self, inputs: &[String]) -> Result<Vec<String>, EngineError> {
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

    fn collect_input_files(&self, input: &str) -> Result<Vec<PathBuf>, EngineError> {
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

    fn collect_content_input_files(&self, input: &str) -> Result<Vec<PathBuf>, EngineError> {
        Ok(self
            .collect_input_files(input)?
            .into_iter()
            .filter(|path| !is_known_mutation_payload_file(path))
            .collect())
    }

    fn validate_review_authored_input_path(&self, inputs: &[String]) -> Result<(), EngineError> {
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

    fn validate_authored_input_paths(
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

    fn validate_authored_inputs(
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

    fn persisted_input_path(&self, resolved: &Path) -> String {
        resolved
            .strip_prefix(&self.repo_root)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| resolved.display().to_string())
    }

    fn resolve_input_path(&self, input: &str) -> PathBuf {
        let path = PathBuf::from(input);
        if path.is_absolute() { path } else { self.repo_root.join(path) }
    }

    fn auto_bind_canonical_mode_inputs(
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

fn default_list(values: Vec<String>, fallback: &str) -> Vec<String> {
    if values.is_empty() { vec![fallback.to_string()] } else { values }
}

fn list_contains_missing_markers(values: &[String]) -> bool {
    values.iter().any(|value| value.contains("NOT CAPTURED"))
}

fn count_captured_list_items(values: &[String]) -> usize {
    values.iter().filter(|value| !value.contains("NOT CAPTURED")).count()
}

fn push_clarification_question(
    questions: &mut Vec<ClarificationQuestionSummary>,
    id: &str,
    prompt: &str,
    rationale: &str,
    evidence: &str,
) {
    if questions.iter().any(|question| question.prompt.eq_ignore_ascii_case(prompt)) {
        return;
    }

    questions.push(ClarificationQuestionSummary {
        id: id.to_string(),
        prompt: prompt.to_string(),
        rationale: rationale.to_string(),
        evidence: evidence.to_string(),
    });
}

fn is_default_requirements_open_question(question: &str) -> bool {
    question.eq_ignore_ascii_case("Which downstream mode should consume this packet first?")
}

fn question_prompt(question: &str) -> String {
    let trimmed = question.trim().trim_end_matches('.');
    if trimmed.ends_with('?') { trimmed.to_string() } else { format!("{trimmed}?") }
}

fn requirements_missing_context(brief: &RequirementsBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        missing.push(
            "Problem framing is missing explicit authored intent or operator goal.".to_string(),
        );
    }
    if brief.outcome.contains("NOT CAPTURED") {
        missing.push(
            "Outcome framing is missing an explicit success signal or bounded result.".to_string(),
        );
    }
    if list_contains_missing_markers(&brief.constraints) {
        missing.push(
            "Constraints are incomplete; downstream shaping would lack explicit non-negotiables."
                .to_string(),
        );
    }
    if list_contains_missing_markers(&brief.tradeoffs) {
        missing.push(
            "Tradeoffs are incomplete; option evaluation would drift toward generic guidance."
                .to_string(),
        );
    }
    if list_contains_missing_markers(&brief.out_of_scope) {
        missing.push(
            "Scope cuts are incomplete; the packet does not yet name explicit exclusions."
                .to_string(),
        );
    }

    missing
}

fn prioritized_requirements_clarification_questions(
    brief: &RequirementsBrief,
    context_summary: &str,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();
    let first_line = first_meaningful_line(context_summary);

    if brief.problem.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-problem",
            "What bounded operator or engineering problem should this requirements packet frame?",
            "Without an explicit problem statement, later modes will optimize for the wrong boundary.",
            &format!("Current intake starts with: {first_line}"),
        );
    }
    if brief.outcome.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-outcome",
            "What explicit outcome or success signal should this packet preserve?",
            "A requirements packet needs a bounded success condition before tradeoffs or exclusions make sense.",
            "No authored `## Outcome` or equivalent success section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.constraints) {
        push_clarification_question(
            &mut questions,
            "clarify-constraints",
            "Which constraints are non-negotiable for this work?",
            "Constraints determine whether downstream shaping stays repo-specific instead of becoming generic planning advice.",
            "No authored `## Constraints`, `## Constraint`, or `## Non-Negotiables` section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.tradeoffs) {
        push_clarification_question(
            &mut questions,
            "clarify-tradeoffs",
            "Which tradeoffs are acceptable, and which ones are explicitly rejected?",
            "Tradeoffs anchor option evaluation and keep the packet honest about what the team is willing to sacrifice.",
            "No authored `## Tradeoffs` section was detected in the supplied inputs.",
        );
    }
    if list_contains_missing_markers(&brief.out_of_scope) {
        push_clarification_question(
            &mut questions,
            "clarify-scope-cuts",
            "What is explicitly out of scope or deferred for this packet?",
            "Scope cuts keep the packet bounded and prevent later modes from inventing extra work.",
            "No authored `## Out of Scope`, `## Scope Cuts`, or equivalent exclusions section was detected in the supplied inputs.",
        );
    }

    for (index, question) in brief.open_questions.iter().enumerate() {
        if question.contains("NOT CAPTURED") || is_default_requirements_open_question(question) {
            continue;
        }

        let prompt = question_prompt(question);
        push_clarification_question(
            &mut questions,
            &format!("authored-open-question-{}", index + 1),
            &prompt,
            "This question is already explicit in the supplied brief and should be resolved before the packet is treated as stable downstream input.",
            "Captured from the authored open-questions or unknowns section.",
        );
    }

    questions.truncate(5);
    questions
}

fn requirements_reasoning_signals(
    source_inputs: &[String],
    brief: &RequirementsBrief,
) -> Vec<String> {
    vec![
        format!(
            "Detected {} authored input surface(s): {}.",
            source_inputs.len(),
            if source_inputs.is_empty() {
                "no-authored-source-inputs-recorded".to_string()
            } else {
                source_inputs.join(", ")
            }
        ),
        format!(
            "Captured {} constraint point(s), {} tradeoff point(s), {} scope cut(s), and {} open question(s).",
            count_captured_list_items(&brief.constraints),
            count_captured_list_items(&brief.tradeoffs),
            count_captured_list_items(&brief.out_of_scope),
            count_captured_list_items(&brief.open_questions)
        ),
        if brief.problem.contains("NOT CAPTURED") || brief.outcome.contains("NOT CAPTURED") {
            "The problem/outcome pair is still incomplete, so downstream design would rely on interpretation instead of authored intent.".to_string()
        } else {
            "The problem/outcome pair is explicit enough to bound a requirements packet before downstream mode selection.".to_string()
        },
    ]
}

fn discovery_summary(brief: &DiscoveryBrief) -> String {
    format!(
        "Problem framing: {}\nConstraints: {}\nRepo focus: {}\nNext phase: {}",
        truncate_context_excerpt(&brief.problem, 180),
        truncate_context_excerpt(&brief.constraints, 180),
        truncate_context_excerpt(&brief.repo_focus, 180),
        truncate_context_excerpt(&brief.next_phase, 180),
    )
}

fn discovery_missing_context(brief: &DiscoveryBrief) -> Vec<String> {
    let mut missing = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        missing.push(
            "Problem framing is missing; discovery still needs an explicit problem domain."
                .to_string(),
        );
    }
    if brief.constraints.contains("NOT CAPTURED") {
        missing.push(
            "Constraints are missing; discovery does not yet name the boundary it must preserve."
                .to_string(),
        );
    }
    if brief.repo_focus.contains("NOT CAPTURED") {
        missing.push(
            "Repository focus is missing; discovery is not yet anchored to concrete repo surfaces."
                .to_string(),
        );
    }
    if brief.unknowns.contains("NOT CAPTURED") {
        missing.push(
            "Unknowns are missing; discovery does not yet name the unresolved decision pressure points."
                .to_string(),
        );
    }
    if brief.next_phase.contains("NOT CAPTURED") {
        missing.push(
            "Next-phase handoff is missing; discovery does not yet say which downstream mode should consume the packet."
                .to_string(),
        );
    }

    missing
}

fn prioritized_discovery_clarification_questions(
    brief: &DiscoveryBrief,
) -> Vec<ClarificationQuestionSummary> {
    let mut questions = Vec::new();

    if brief.problem.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-problem",
            "What exact problem domain should discovery bound before handoff?",
            "Discovery needs a named problem domain so later packets do not drift across unrelated repository surfaces.",
            "No authored `## Problem` or `## Problem Domain` section was detected in the supplied discovery brief.",
        );
    }
    if brief.constraints.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-constraints",
            "Which explicit constraints or boundary rules must discovery preserve?",
            "Constraints keep the discovery packet honest about what later modes are allowed to change or assume.",
            "No authored `## Constraints` or equivalent boundary section was detected in the supplied discovery brief.",
        );
    }
    if brief.repo_focus.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-repo-focus",
            "Which repository surfaces, modules, or files should discovery stay anchored to?",
            "Repo focus determines whether discovery remains grounded in the actual workspace instead of generic planning language.",
            "No authored `## Repo Focus`, `## Repository Focus`, or `## System Slice` section was detected in the supplied discovery brief.",
        );
    }
    if brief.unknowns.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-unknowns",
            "Which unknowns or decision risks should discovery make explicit before handoff?",
            "Discovery becomes weaker when it names a boundary but not the unresolved questions that still matter.",
            "No authored `## Unknowns` or `## Open Questions` section was detected in the supplied discovery brief.",
        );
    }
    if brief.next_phase.contains("NOT CAPTURED") {
        push_clarification_question(
            &mut questions,
            "clarify-discovery-next-phase",
            "Which downstream governed mode should consume this packet next, and under what trigger?",
            "A discovery packet needs a concrete translation path so the result remains actionable.",
            "No authored `## Next Phase`, `## Handoff`, or `## Translation Trigger` section was detected in the supplied discovery brief.",
        );
    }

    if !brief.unknowns.contains("NOT CAPTURED") {
        for (index, question) in split_context_items(&brief.unknowns).into_iter().enumerate() {
            if question.contains("NOT CAPTURED") {
                continue;
            }

            let prompt = question_prompt(&question);
            push_clarification_question(
                &mut questions,
                &format!("authored-discovery-question-{}", index + 1),
                &prompt,
                "This unknown is already explicit in the discovery brief and should stay visible before downstream translation.",
                "Captured from the authored unknowns or open-questions section.",
            );
        }
    }

    questions.truncate(5);
    questions
}

fn discovery_reasoning_signals(
    source_inputs: &[String],
    repo_surfaces: &[String],
    brief: &DiscoveryBrief,
) -> Vec<String> {
    vec![
        format!(
            "Detected {} authored input surface(s): {}.",
            source_inputs.len(),
            if source_inputs.is_empty() {
                "no-authored-source-inputs-recorded".to_string()
            } else {
                source_inputs.join(", ")
            }
        ),
        format!(
            "Mapped {} repository surface hint(s) for discovery anchoring.",
            repo_surfaces.len()
        ),
        format!(
            "Captured {} unknown or open-question item(s) and inferred next phase `{}`.",
            if brief.unknowns.contains("NOT CAPTURED") {
                0
            } else {
                count_markdown_entries(&brief.unknowns)
            },
            truncate_context_excerpt(&brief.next_phase, 96)
        ),
    ]
}

fn extract_context_list(source: &str, normalized: &str, markers: &[&str]) -> Vec<String> {
    extract_context_marker(source, normalized, markers)
        .map(|value| split_context_items(&value))
        .unwrap_or_default()
}

fn split_context_items(block: &str) -> Vec<String> {
    let items = block
        .lines()
        .filter_map(|line| trim_list_item(line.trim()))
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();

    if !items.is_empty() {
        return items;
    }

    let condensed = condense_context_block(block, 220);
    if condensed.is_empty() { Vec::new() } else { vec![condensed] }
}

fn trim_list_item(line: &str) -> Option<String> {
    if !is_meaningful_context_line(line) {
        return None;
    }

    if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        return Some(item.trim().to_string());
    }

    let mut digits = 0usize;
    for character in line.chars() {
        if character.is_ascii_digit() {
            digits += 1;
            continue;
        }

        if digits > 0 && (character == '.' || character == ')') {
            return Some(line[digits + 1..].trim().to_string());
        }

        break;
    }

    None
}

fn condense_context_block(value: &str, max_chars: usize) -> String {
    let filtered = value
        .lines()
        .map(str::trim)
        .filter(|line| is_meaningful_context_line(line))
        .collect::<Vec<_>>();
    let candidate =
        if filtered.is_empty() { trim_context_block(value) } else { filtered.join(" ") };

    truncate_context_excerpt(&candidate, max_chars)
}

fn is_meaningful_context_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty()
        && !trimmed.starts_with('#')
        && !trimmed.starts_with("## Input:")
        && !trimmed.starts_with("![](")
        && !trimmed.starts_with('|')
        && !trimmed.starts_with("Page ")
        && !trimmed.eq_ignore_ascii_case("internal")
        && !trimmed.starts_with("Version ")
        && !trimmed.starts_with("Author:")
        && !trimmed.starts_with("Prepared by:")
        && !trimmed.starts_with("Checked by:")
        && !trimmed.starts_with("Approved by:")
        && !trimmed.starts_with("Revision")
}

fn truncate_context_excerpt(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    let char_count = trimmed.chars().count();
    if char_count <= max_chars {
        return trimmed.to_string();
    }

    let truncated = trimmed.chars().take(max_chars).collect::<String>();
    let safe = truncated.rfind(char::is_whitespace).map(|index| truncated[..index].trim());
    match safe {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}..."),
        _ => format!("{}...", truncated.trim()),
    }
}

fn summarize_mode_result(mode: Mode, artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    match mode {
        Mode::Requirements => summarize_requirements_mode_result(artifacts),
        Mode::Discovery => summarize_discovery_mode_result(artifacts),
        Mode::SystemShaping => summarize_system_shaping_mode_result(artifacts),
        Mode::Architecture => summarize_architecture_mode_result(artifacts),
        Mode::Change => summarize_change_mode_result(artifacts),
        Mode::Implementation => summarize_implementation_mode_result(artifacts),
        Mode::Refactor => summarize_refactor_mode_result(artifacts),
        Mode::Review => summarize_review_mode_result(artifacts),
        Mode::Verification => summarize_verification_mode_result(artifacts),
        Mode::PrReview => summarize_pr_review_mode_result(artifacts),
        _ => None,
    }
}

fn apply_execution_posture_summary(
    mode_result: Option<ModeResultSummary>,
    context: Option<&RunContext>,
    approvals: &[ApprovalRecord],
) -> Option<ModeResultSummary> {
    let mut mode_result = mode_result?;
    mode_result.execution_posture = resolved_execution_posture_label(context, approvals);
    Some(mode_result)
}

fn resolved_execution_posture_label(
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

fn execution_posture_label(context: &RunContext) -> Option<&'static str> {
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

fn set_execution_posture(context: &mut RunContext, posture: ExecutionPosture) {
    if let Some(execution) = context.implementation_execution.as_mut() {
        execution.execution_posture = posture;
    }

    if let Some(execution) = context.refactor_execution.as_mut() {
        execution.execution_posture = posture;
    }
}

fn post_approval_execution_consumed(context: &RunContext) -> bool {
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

fn set_post_approval_execution_consumed(context: &mut RunContext, consumed: bool) {
    if let Some(execution) = context.implementation_execution.as_mut() {
        execution.post_approval_execution_consumed = consumed;
    }

    if let Some(execution) = context.refactor_execution.as_mut() {
        execution.post_approval_execution_consumed = consumed;
    }
}

fn execution_continuation_pending(context: &RunContext, approvals: &[ApprovalRecord]) -> bool {
    execution_gate_is_approved(approvals) && !post_approval_execution_consumed(context)
}

fn execution_gate_is_approved(approvals: &[ApprovalRecord]) -> bool {
    approvals
        .iter()
        .any(|approval| approval.matches_gate(GateKind::Execution) && approval.is_approved())
}

fn approval_record_refs(run_id: &str, approvals: &[ApprovalRecord]) -> Vec<String> {
    approvals
        .iter()
        .enumerate()
        .map(|(index, _)| format!("runs/{run_id}/approvals/approval-{index:02}.toml"))
        .collect()
}

fn approved_execution_mutation_rationale(
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

fn mutation_payload_candidates_for(resolved: &Path) -> Vec<PathBuf> {
    if resolved.is_dir() {
        return known_mutation_payload_names().iter().map(|name| resolved.join(name)).collect();
    }

    let Some(parent) = resolved.parent() else {
        return Vec::new();
    };

    let mut candidates =
        known_mutation_payload_names().iter().map(|name| parent.join(name)).collect::<Vec<_>>();
    if let Some(stem) = resolved.file_stem().and_then(|stem| stem.to_str()) {
        candidates.push(parent.join(format!("{stem}.diff")));
        candidates.push(parent.join(format!("{stem}.patch")));
    }
    candidates
}

fn is_known_mutation_payload_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            let lower = name.to_ascii_lowercase();
            known_mutation_payload_names().iter().any(|candidate| *candidate == lower)
        })
        .unwrap_or(false)
}

fn known_mutation_payload_names() -> &'static [&'static str] {
    &[
        "patch.diff",
        "mutation.diff",
        "mutation.patch",
        "execution.diff",
        "execution.patch",
        "bounded.diff",
        "bounded.patch",
    ]
}

fn parse_unified_diff_paths(patch: &str) -> Result<Vec<String>, EngineError> {
    let mut changed_paths = Vec::new();

    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let mut parts = rest.split_whitespace();
            for raw in parts.by_ref().take(2) {
                if let Some(path) = normalize_diff_path(raw) {
                    push_unique_path(&mut changed_paths, path);
                }
            }
            continue;
        }

        if let Some(raw) = line.strip_prefix("--- ").or_else(|| line.strip_prefix("+++ "))
            && let Some(path) = normalize_diff_path(raw.trim())
        {
            push_unique_path(&mut changed_paths, path);
        }
    }

    if changed_paths.is_empty() {
        return Err(EngineError::Validation(
            "bounded mutation payload does not declare any changed file paths".to_string(),
        ));
    }

    Ok(changed_paths)
}

fn normalize_diff_path(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches('"');
    if trimmed == "/dev/null" {
        return None;
    }

    let stripped =
        trimmed.strip_prefix("a/").or_else(|| trimmed.strip_prefix("b/")).unwrap_or(trimmed);
    let normalized = normalize_repo_relative_path(stripped);
    (!normalized.is_empty()).then_some(normalized)
}

fn push_unique_path(paths: &mut Vec<String>, candidate: String) {
    if !paths.iter().any(|existing| existing == &candidate) {
        paths.push(candidate);
    }
}

fn path_within_allowed_scope(path: &str, allowed_paths: &[String]) -> bool {
    let normalized_path = normalize_repo_relative_path(path);
    if normalized_path.is_empty() {
        return false;
    }

    allowed_paths.iter().any(|entry| {
        normalized_scope_prefix(entry).is_some_and(|allowed| {
            normalized_path == allowed || normalized_path.starts_with(&format!("{allowed}/"))
        })
    })
}

fn normalized_scope_prefix(entry: &str) -> Option<String> {
    let normalized = normalize_repo_relative_path(entry);
    if normalized.is_empty() {
        return None;
    }

    let trimmed = normalized
        .strip_suffix("/**")
        .or_else(|| normalized.strip_suffix("/*"))
        .unwrap_or(normalized.as_str())
        .trim_end_matches('/');
    (!trimmed.is_empty()).then_some(trimmed.to_string())
}

fn normalize_repo_relative_path(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_matches('/')
        .to_string()
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

fn build_action_chips_for(
    state: RunState,
    approval_targets: &[String],
    primary_artifact_path: &str,
    run_id: &str,
) -> Vec<ActionChip> {
    use std::collections::BTreeMap;

    let mut chips: Vec<ActionChip> = Vec::new();

    if !primary_artifact_path.is_empty() {
        let mut args = BTreeMap::new();
        args.insert("PATH".to_string(), primary_artifact_path.to_string());
        chips.push(ActionChip {
            id: "open-primary-artifact".to_string(),
            label: "Open primary artifact".to_string(),
            skill: "host:open-file".to_string(),
            intent: "Inspect".to_string(),
            prefilled_args: args,
            required_user_inputs: Vec::new(),
            visibility_condition: "primary_artifact_path is non-empty".to_string(),
            recommended: false,
            text_fallback: format!("Open the primary artifact at {primary_artifact_path}."),
        });
    }

    if matches!(state, RunState::AwaitingApproval | RunState::Completed) && !run_id.is_empty() {
        let mut args = BTreeMap::new();
        args.insert("RUN_ID".to_string(), run_id.to_string());
        chips.push(ActionChip {
            id: "inspect-evidence".to_string(),
            label: "Inspect evidence".to_string(),
            skill: "canon-inspect-evidence".to_string(),
            intent: "Inspect".to_string(),
            prefilled_args: args,
            required_user_inputs: Vec::new(),
            visibility_condition: "state is AwaitingApproval or Completed".to_string(),
            recommended: matches!(state, RunState::AwaitingApproval)
                && !approval_targets.is_empty(),
            text_fallback: format!("Use $canon-inspect-evidence for run {run_id}."),
        });
    }

    if matches!(state, RunState::AwaitingApproval) && !run_id.is_empty() {
        for target in approval_targets {
            let mut args = BTreeMap::new();
            args.insert("RUN_ID".to_string(), run_id.to_string());
            args.insert("TARGET".to_string(), target.clone());
            chips.push(ActionChip {
                id: format!("approve-{}", target.replace(':', "-")),
                label: "Approve generation...".to_string(),
                skill: "canon-approve".to_string(),
                intent: "GovernedAction".to_string(),
                prefilled_args: args,
                required_user_inputs: vec![
                    "BY".to_string(),
                    "DECISION".to_string(),
                    "RATIONALE".to_string(),
                ],
                visibility_condition:
                    "state is AwaitingApproval and Canon issued an approval target".to_string(),
                recommended: false,
                text_fallback: format!(
                    "Review the packet for run {run_id}, then approve using $canon-approve."
                ),
            });
        }

        if approval_targets.is_empty() {
            let mut args = BTreeMap::new();
            args.insert("RUN_ID".to_string(), run_id.to_string());
            chips.push(ActionChip {
                id: "resume-run".to_string(),
                label: "Resume run".to_string(),
                skill: "canon-resume".to_string(),
                intent: "GovernedAction".to_string(),
                prefilled_args: args,
                required_user_inputs: Vec::new(),
                visibility_condition:
                    "state is AwaitingApproval and Canon has no remaining approval targets"
                        .to_string(),
                recommended: true,
                text_fallback: format!(
                    "Use $canon-resume for run {run_id} to continue post-approval execution."
                ),
            });
        }
    }

    chips
}

fn summarize_discovery_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "problem-map.md")?;
    let unknowns_artifact = artifacts
        .iter()
        .find(|artifact| artifact.record.file_name == "unknowns-and-assumptions.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "context-boundary.md");

    let problem_domain = extract_context_section(&primary.contents, "Problem Domain")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem domain summary is missing.".to_string());
    let repo_signals = extract_context_section(&primary.contents, "Repo Signals")
        .unwrap_or_else(|| "NOT CAPTURED - Repository signals are missing.".to_string());
    let next_phase = extract_context_section(&primary.contents, "Downstream Handoff")
        .or_else(|| {
            boundary_artifact.and_then(|artifact| {
                extract_context_section(&artifact.contents, "Translation Trigger")
            })
        })
        .unwrap_or_else(|| "NOT CAPTURED - Next-phase handoff is missing.".to_string());
    let unknowns = unknowns_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Unknowns"))
        .unwrap_or_else(|| "NOT CAPTURED - Unknowns section is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&problem_domain, &repo_signals, &next_phase, &unknowns]);
    let repo_signal_count = count_markdown_entries(&repo_signals);
    let unknown_count = count_markdown_entries(&unknowns);

    let headline = if missing_context_markers == 0 {
        "Discovery packet ready for downstream translation.".to_string()
    } else {
        format!(
            "Discovery packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = format!(
        "Primary artifact maps {repo_signal_count} repository signal(s) and {unknown_count} unknown or assumption set(s). Next phase: {}.",
        truncate_context_excerpt(&next_phase, 120)
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Map".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem_domain, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_system_shaping_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "system-shape.md")?;
    let capability_map =
        artifacts.iter().find(|artifact| artifact.record.file_name == "capability-map.md");
    let delivery_options =
        artifacts.iter().find(|artifact| artifact.record.file_name == "delivery-options.md");
    let risk_hotspots =
        artifacts.iter().find(|artifact| artifact.record.file_name == "risk-hotspots.md");

    let system_shape = extract_context_section(&primary.contents, "System Shape")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - System shape summary is missing.".to_string());
    let boundary_decisions = extract_context_section(&primary.contents, "Boundary Decisions")
        .unwrap_or_else(|| "NOT CAPTURED - Boundary decisions are missing.".to_string());
    let capabilities = capability_map
        .and_then(|artifact| extract_context_section(&artifact.contents, "Capabilities"))
        .unwrap_or_else(|| "NOT CAPTURED - Capability map is missing.".to_string());
    let delivery_phases = delivery_options
        .and_then(|artifact| extract_context_section(&artifact.contents, "Delivery Phases"))
        .unwrap_or_else(|| "NOT CAPTURED - Delivery phases are missing.".to_string());
    let hotspots = risk_hotspots
        .and_then(|artifact| extract_context_section(&artifact.contents, "Hotspots"))
        .unwrap_or_else(|| "NOT CAPTURED - Risk hotspots are missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &system_shape,
        &boundary_decisions,
        &capabilities,
        &delivery_phases,
        &hotspots,
    ]);
    let capability_count = count_markdown_entries(&capabilities);
    let delivery_count = count_markdown_entries(&delivery_phases);
    let hotspot_count = count_markdown_entries(&hotspots);

    let headline = if missing_context_markers == 0 {
        "System-shaping packet ready for downstream architecture or delivery planning.".to_string()
    } else {
        format!(
            "System-shaping packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = format!(
        "Primary artifact names {capability_count} capability slice(s), {delivery_count} delivery phase set(s), and {hotspot_count} risk hotspot set(s)."
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "System Shape".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&system_shape, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_architecture_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts
        .iter()
        .find(|artifact| artifact.record.file_name == "architecture-decisions.md")?;
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "invariants.md");
    let tradeoff_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "tradeoff-matrix.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "boundary-map.md");

    let decisions = extract_context_section(&primary.contents, "Decisions")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Architecture decisions are missing.".to_string());
    let tradeoffs = extract_context_section(&primary.contents, "Tradeoffs")
        .or_else(|| {
            tradeoff_artifact
                .and_then(|artifact| extract_context_section(&artifact.contents, "Scores"))
        })
        .unwrap_or_else(|| "NOT CAPTURED - Architecture tradeoffs are missing.".to_string());
    let invariants = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Invariants are missing.".to_string());
    let boundaries = boundary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Boundaries"))
        .unwrap_or_else(|| "NOT CAPTURED - Boundary map is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&decisions, &tradeoffs, &invariants, &boundaries]);
    let decision_count = count_markdown_entries(&decisions);
    let tradeoff_count = count_markdown_entries(&tradeoffs);
    let invariant_count = count_markdown_entries(&invariants);
    let boundary_count = count_markdown_entries(&boundaries);

    let headline = if missing_context_markers == 0 {
        "Architecture packet ready for downstream implementation or review.".to_string()
    } else {
        format!(
            "Architecture packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = format!(
        "Primary artifact records {decision_count} decision set(s), {tradeoff_count} tradeoff set(s), {invariant_count} invariant set(s), and {boundary_count} boundary set(s)."
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Architecture Decisions".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&decisions, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_change_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "change-surface.md")?;
    let legacy_invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "legacy-invariants.md");
    let validation_strategy_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "validation-strategy.md");
    let system_slice_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "system-slice.md");

    let (change_surface, change_surface_missing) = extract_result_section(
        &primary.contents,
        "Change Surface",
        "Missing Context",
        "NOT CAPTURED - Change surface section is missing.",
    );
    let (legacy_invariants, legacy_missing) = legacy_invariants_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Legacy Invariants",
                "Missing Context",
                "NOT CAPTURED - Legacy invariants section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Legacy invariants artifact is missing.".to_string(), true)
        });
    let (validation_strategy, validation_missing) = validation_strategy_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Validation Strategy",
                "Missing Context",
                "NOT CAPTURED - Validation strategy section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Validation strategy artifact is missing.".to_string(), true)
        });
    let (system_slice, system_slice_missing) = system_slice_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "System Slice",
                "Missing Context",
                "NOT CAPTURED - System slice section is missing.",
            )
        })
        .unwrap_or_else(|| ("NOT CAPTURED - System slice artifact is missing.".to_string(), true));

    let missing_context_markers =
        [change_surface_missing, legacy_missing, validation_missing, system_slice_missing]
            .into_iter()
            .filter(|missing| *missing)
            .count();
    let change_surface_count = count_markdown_entries(&change_surface);
    let legacy_invariant_count = count_markdown_entries(&legacy_invariants);
    let validation_count = count_markdown_entries(&validation_strategy);

    let headline = if missing_context_markers == 0 {
        "Change packet ready for bounded change review.".to_string()
    } else {
        format!(
            "Change packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact names {change_surface_count} change-surface point(s). Packet also captures {legacy_invariant_count} legacy invariant(s) and {validation_count} validation check set(s) for the bounded slice {}.",
            truncate_context_excerpt(&system_slice, 90)
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Change surface: {change_surface_count}; legacy invariants: {legacy_invariant_count}; validation checks: {validation_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Change Surface".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&change_surface, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_implementation_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "task-mapping.md")?;
    let mutation_bounds_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "mutation-bounds.md");
    let validation_hooks_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "validation-hooks.md");

    let (task_mapping, task_mapping_missing) = extract_result_section(
        &primary.contents,
        "Task Mapping",
        "Missing Context",
        "NOT CAPTURED - Task mapping section is missing.",
    );
    let (bounded_changes, bounded_changes_missing) = extract_result_section(
        &primary.contents,
        "Bounded Changes",
        "Missing Context",
        "NOT CAPTURED - Bounded changes section is missing.",
    );
    let (allowed_paths, allowed_paths_missing) = mutation_bounds_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Allowed Paths",
                "Missing Context",
                "NOT CAPTURED - Allowed paths section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Mutation bounds artifact is missing.".to_string(), true)
        });
    let (safety_net_evidence, safety_net_missing) = validation_hooks_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Safety-Net Evidence",
                "Missing Context",
                "NOT CAPTURED - Safety-net evidence section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Validation hooks artifact is missing.".to_string(), true)
        });

    let missing_context_markers =
        [task_mapping_missing, bounded_changes_missing, allowed_paths_missing, safety_net_missing]
            .into_iter()
            .filter(|missing| *missing)
            .count();
    let task_count = count_markdown_entries(&task_mapping);
    let allowed_path_count = count_markdown_entries(&allowed_paths);
    let safety_net_count = count_markdown_entries(&safety_net_evidence);

    let headline = if missing_context_markers == 0 {
        "Implementation packet ready for bounded execution review.".to_string()
    } else {
        format!(
            "Implementation packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact maps {task_count} task set(s) across {allowed_path_count} allowed path set(s) with {safety_net_count} safety-net evidence set(s). Bounded changes: {}.",
            truncate_context_excerpt(&bounded_changes, 90)
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Tasks: {task_count}; allowed paths: {allowed_path_count}; safety-net evidence: {safety_net_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Task Mapping".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&task_mapping, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_refactor_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "preserved-behavior.md")?;
    let scope_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "refactor-scope.md");
    let contract_drift_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "contract-drift-check.md");
    let no_feature_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "no-feature-addition.md");

    let (preserved_behavior, preserved_missing) = extract_result_section(
        &primary.contents,
        "Preserved Behavior",
        "Missing Context",
        "NOT CAPTURED - Preserved behavior section is missing.",
    );
    let (_refactor_scope, scope_missing) = scope_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Refactor Scope",
                "Missing Context",
                "NOT CAPTURED - Refactor scope section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Refactor scope artifact is missing.".to_string(), true)
        });
    let (allowed_paths, allowed_paths_missing) = scope_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Allowed Paths",
                "Missing Context",
                "NOT CAPTURED - Allowed paths section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Refactor scope artifact is missing.".to_string(), true)
        });
    let (contract_drift, drift_missing) = contract_drift_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Contract Drift",
                "Missing Context",
                "NOT CAPTURED - Contract drift section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - Contract drift artifact is missing.".to_string(), true)
        });
    let (feature_audit, feature_audit_missing) = no_feature_artifact
        .map(|artifact| {
            extract_result_section(
                &artifact.contents,
                "Feature Audit",
                "Missing Context",
                "NOT CAPTURED - Feature audit section is missing.",
            )
        })
        .unwrap_or_else(|| {
            ("NOT CAPTURED - No-feature-addition artifact is missing.".to_string(), true)
        });

    let missing_context_markers = [
        preserved_missing,
        scope_missing,
        allowed_paths_missing,
        drift_missing,
        feature_audit_missing,
    ]
    .into_iter()
    .filter(|missing| *missing)
    .count();
    let preserved_count = count_markdown_entries(&preserved_behavior);
    let allowed_path_count = count_markdown_entries(&allowed_paths);
    let feature_audit_count = count_markdown_entries(&feature_audit);

    let headline = if missing_context_markers == 0 {
        "Refactor packet ready for preservation review.".to_string()
    } else {
        format!(
            "Refactor packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact names {preserved_count} preserved-behavior set(s) across {allowed_path_count} allowed path set(s). Contract drift note: {}. Feature audit sets: {feature_audit_count}.",
            truncate_context_excerpt(&contract_drift, 90)
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Preserved behavior: {preserved_count}; allowed paths: {allowed_path_count}; feature audit: {feature_audit_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Preserved Behavior".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&preserved_behavior, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_review_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "review-disposition.md")?;
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "boundary-assessment.md");
    let missing_evidence_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "missing-evidence.md");

    let final_disposition = extract_context_section(&primary.contents, "Final Disposition")
        .unwrap_or_else(|| "NOT CAPTURED - Final disposition section is missing.".to_string());
    let accepted_risks = extract_context_section(&primary.contents, "Accepted Risks")
        .unwrap_or_else(|| "NOT CAPTURED - Accepted risks section is missing.".to_string());
    let boundary_findings = boundary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Boundary Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Boundary findings section is missing.".to_string());
    let missing_evidence = missing_evidence_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Missing Evidence"))
        .unwrap_or_else(|| "NOT CAPTURED - Missing evidence section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &final_disposition,
        &accepted_risks,
        &boundary_findings,
        &missing_evidence,
    ]);
    let disposition_status = extract_labeled_context_value(&final_disposition, "Status")
        .unwrap_or_else(|| "unknown-disposition".to_string());
    let rationale = extract_labeled_context_value(&final_disposition, "Rationale")
        .unwrap_or_else(|| truncate_context_excerpt(&final_disposition, 320));
    let boundary_count = count_context_items_without_placeholders(
        &boundary_findings,
        &["No boundary expansion beyond the authored review target was detected."],
    );
    let accepted_risk_count = count_context_items_without_placeholders(
        &accepted_risks,
        &[
            "No accepted risks recorded while disposition is still pending.",
            "Residual review notes remain bounded to the current package and can be inspected through the emitted artifacts.",
        ],
    );

    let headline = if missing_context_markers == 0 {
        match disposition_status.as_str() {
            "awaiting-disposition" => {
                "Review packet requires explicit disposition before release-readiness can pass."
                    .to_string()
            }
            "accepted-with-approval" => {
                "Review packet completed with explicit approval for the remaining concerns."
                    .to_string()
            }
            _ => "Review packet ready for downstream inspection and bounded follow-up.".to_string(),
        }
    } else {
        format!(
            "Review packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact records `{disposition_status}` disposition with {boundary_count} boundary finding set(s) and {accepted_risk_count} accepted-risk or review-note set(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Boundary findings: {boundary_count}; accepted risks: {accepted_risk_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Review Disposition".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&rationale, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_verification_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "verification-report.md")?;
    let unresolved_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "unresolved-findings.md");
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "invariants-checklist.md");

    let verified_claims = extract_context_section(&primary.contents, "Verified Claims")
        .unwrap_or_else(|| "NOT CAPTURED - Verified claims section is missing.".to_string());
    let rejected_claims = extract_context_section(&primary.contents, "Rejected Claims")
        .unwrap_or_else(|| "NOT CAPTURED - Rejected claims section is missing.".to_string());
    let overall_verdict = extract_context_section(&primary.contents, "Overall Verdict")
        .unwrap_or_else(|| "NOT CAPTURED - Overall verdict section is missing.".to_string());
    let open_findings = unresolved_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Open findings section is missing.".to_string());
    let claims_under_test = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Claims Under Test"))
        .unwrap_or_else(|| "NOT CAPTURED - Claims under test section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &verified_claims,
        &rejected_claims,
        &overall_verdict,
        &open_findings,
        &claims_under_test,
    ]);
    let verdict_status = extract_labeled_context_value(&overall_verdict, "Status")
        .unwrap_or_else(|| "unknown-verdict".to_string());
    let open_findings_status = extract_labeled_context_value(&open_findings, "Status")
        .unwrap_or_else(|| "unknown-open-findings".to_string());
    let claim_count = count_context_items_without_placeholders(
        &claims_under_test,
        &["The current invariants are bounded enough for recorded verification."],
    );
    let open_finding_count = count_context_items_without_placeholders(
        &open_findings,
        &["No unresolved findings remain from the current verification target."],
    );

    let headline = if missing_context_markers == 0 {
        if open_findings_status == "unresolved-findings-open" {
            format!(
                "Verification found {open_finding_count} unresolved finding(s) and blocked release readiness."
            )
        } else {
            format!(
                "Verification packet completed with `{verdict_status}` verdict across {claim_count} claim set(s)."
            )
        }
    } else {
        format!(
            "Verification packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact records `{verdict_status}` verdict with {claim_count} claim set(s) under test and {open_finding_count} unresolved finding set(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Claim sets: {claim_count}; open findings: {open_finding_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Verification Report".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&overall_verdict, 320),
        action_chips: Vec::new(),
    })
}

fn summarize_pr_review_mode_result(artifacts: &[PersistedArtifact]) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "review-summary.md")?;
    let pr_analysis_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "pr-analysis.md");

    let final_disposition = extract_context_section(&primary.contents, "Final Disposition")
        .unwrap_or_else(|| "NOT CAPTURED - Final disposition section is missing.".to_string());
    let severity = extract_context_section(&primary.contents, "Severity")
        .unwrap_or_else(|| "NOT CAPTURED - Severity section is missing.".to_string());
    let must_fix_findings = extract_context_section(&primary.contents, "Must-Fix Findings")
        .unwrap_or_else(|| "NOT CAPTURED - Must-fix findings section is missing.".to_string());
    let accepted_risks = extract_context_section(&primary.contents, "Accepted Risks")
        .unwrap_or_else(|| "NOT CAPTURED - Accepted risks section is missing.".to_string());
    let changed_modules = pr_analysis_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Changed Modules"))
        .unwrap_or_else(|| "NOT CAPTURED - Changed modules section is missing.".to_string());
    let inferred_intent = pr_analysis_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Inferred Intent"))
        .unwrap_or_else(|| "NOT CAPTURED - Inferred intent section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &final_disposition,
        &severity,
        &must_fix_findings,
        &accepted_risks,
        &changed_modules,
        &inferred_intent,
    ]);
    let disposition_status = extract_labeled_context_value(&final_disposition, "Status")
        .unwrap_or_else(|| "unknown-disposition".to_string());
    let rationale = extract_labeled_context_value(&final_disposition, "Rationale")
        .unwrap_or_else(|| truncate_context_excerpt(&final_disposition, 320));
    let overall_severity = extract_labeled_context_value(&severity, "Overall severity")
        .unwrap_or_else(|| {
            if must_fix_findings.contains("No must-fix findings remain.") {
                "review-notes".to_string()
            } else {
                "must-fix".to_string()
            }
        });
    let must_fix_count =
        extract_labeled_usize(&severity, "Must-fix findings").unwrap_or_else(|| {
            count_context_items_without_placeholders(
                &must_fix_findings,
                &["No must-fix findings remain."],
            )
        });
    let review_note_count = extract_labeled_usize(&severity, "Review notes").unwrap_or_else(|| {
        count_context_items_without_placeholders(&accepted_risks, &["No accepted risks recorded."])
    });
    let changed_surface_count = count_context_items_without_placeholders(
        &changed_modules,
        &["No changed surfaces detected."],
    );

    let headline = if missing_context_markers == 0 {
        match disposition_status.as_str() {
            "ready-with-review-notes" => format!(
                "PR review completed with {review_note_count} review note(s) and no unresolved must-fix findings."
            ),
            "awaiting-disposition" => format!(
                "PR review found {must_fix_count} must-fix finding(s) and is waiting for explicit disposition."
            ),
            "accepted-with-approval" => {
                "PR review completed with explicit approval for the remaining must-fix findings."
                    .to_string()
            }
            _ => format!("PR review completed with disposition `{disposition_status}`."),
        }
    } else {
        format!(
            "PR review packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact records `{disposition_status}` disposition with `{overall_severity}` severity across {changed_surface_count} changed surface(s), {must_fix_count} must-fix finding(s), and {review_note_count} review note(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Changed surfaces: {changed_surface_count}; must-fix findings: {must_fix_count}; review notes: {review_note_count}."
        )
    };
    let result_excerpt = if rationale.contains("NOT CAPTURED") {
        truncate_context_excerpt(&inferred_intent, 320)
    } else {
        truncate_context_excerpt(&rationale, 320)
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Review Summary".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt,
        action_chips: Vec::new(),
    })
}

fn summarize_requirements_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.file_name == "problem-statement.md")?;
    let constraints_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "constraints.md");
    let scope_cuts_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "scope-cuts.md");
    let decision_artifact =
        artifacts.iter().find(|artifact| artifact.record.file_name == "decision-checklist.md");

    let problem = extract_context_section(&primary.contents, "Problem")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem statement summary is missing.".to_string());
    let constraints = constraints_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Constraints"))
        .unwrap_or_else(|| "NOT CAPTURED - Constraints section is missing.".to_string());
    let scope_cuts = scope_cuts_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Scope Cuts"))
        .unwrap_or_else(|| "NOT CAPTURED - Scope cuts section is missing.".to_string());
    let open_questions = decision_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Questions"))
        .unwrap_or_else(|| "NOT CAPTURED - Open questions section is missing.".to_string());

    let missing_context_markers = [&problem, &constraints, &scope_cuts, &open_questions]
        .into_iter()
        .filter(|section| section.contains("NOT CAPTURED"))
        .count();
    let constraint_count = count_markdown_entries(&constraints);
    let scope_cut_count = count_markdown_entries(&scope_cuts);
    let open_question_count = count_markdown_entries(&open_questions);

    let headline = if missing_context_markers == 0 {
        "Requirements packet ready for downstream review.".to_string()
    } else {
        format!(
            "Requirements packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "Primary artifact is ready. Packet captures {constraint_count} constraint point(s), {scope_cut_count} scope cut(s), and {open_question_count} open question(s)."
        )
    } else {
        format!(
            "Primary artifact is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Constraints: {constraint_count}; scope cuts: {scope_cut_count}; open questions: {open_question_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Statement".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem, 320),
        action_chips: Vec::new(),
    })
}

fn primary_artifact_action_for(path: &str) -> ResultActionSummary {
    ResultActionSummary {
        id: "open-primary-artifact".to_string(),
        label: "Open primary artifact".to_string(),
        host_action: "open-file".to_string(),
        target: path.to_string(),
        text_fallback: format!("Open the primary artifact at {path}."),
    }
}

fn count_markdown_entries(block: &str) -> usize {
    let count = block.lines().filter(|line| trim_list_item(line.trim()).is_some()).count();
    if count == 0 && !block.trim().is_empty() && !block.contains("NOT CAPTURED") {
        1
    } else {
        count
    }
}

fn count_context_items_without_placeholders(block: &str, placeholders: &[&str]) -> usize {
    if block.contains("NOT CAPTURED") {
        return 0;
    }

    split_context_items(block)
        .into_iter()
        .filter(|item| {
            !placeholders.iter().any(|placeholder| item.eq_ignore_ascii_case(placeholder))
        })
        .count()
}

fn extract_labeled_usize(block: &str, label: &str) -> Option<usize> {
    extract_labeled_context_value(block, label)?.parse().ok()
}

fn extract_labeled_context_value(block: &str, label: &str) -> Option<String> {
    let prefix = format!("{}:", label.to_ascii_lowercase());

    block.lines().find_map(|line| {
        let normalized = trim_list_item(line.trim()).unwrap_or_else(|| line.trim().to_string());
        if !normalized.to_ascii_lowercase().starts_with(&prefix) {
            return None;
        }

        let value = normalized[normalized.find(':')? + 1..].trim();
        if value.is_empty() { None } else { Some(value.to_string()) }
    })
}

fn extract_result_section(
    contents: &str,
    section: &str,
    missing_section: &str,
    fallback: &str,
) -> (String, bool) {
    if let Some(value) = extract_context_section(contents, section) {
        return (value, false);
    }

    if let Some(value) = extract_context_section(contents, missing_section) {
        return (format!("NOT CAPTURED - {}", trim_context_block(&value)), true);
    }

    (fallback.to_string(), true)
}

fn count_missing_context_markers<T>(sections: impl IntoIterator<Item = T>) -> usize
where
    T: AsRef<str>,
{
    sections.into_iter().filter(|section| section.as_ref().contains("NOT CAPTURED")).count()
}

fn render_repo_surface_block(repo_surfaces: &[String]) -> String {
    if repo_surfaces.is_empty() {
        "- no-repository-surfaces-detected".to_string()
    } else {
        repo_surfaces.iter().map(|surface| format!("- {surface}")).collect::<Vec<_>>().join("\n")
    }
}

fn extract_context_marker(source: &str, normalized: &str, markers: &[&str]) -> Option<String> {
    markers.iter().find_map(|marker| {
        extract_context_section(source, marker)
            .or_else(|| extract_context_inline_marker(source, normalized, marker))
    })
}

fn extract_context_inline_marker(source: &str, normalized: &str, marker: &str) -> Option<String> {
    let marker_with_colon = format!("{marker}:");
    let start = normalized.find(&marker_with_colon)?;
    let remainder = &source[start + marker_with_colon.len()..];
    let line = remainder.lines().next()?.trim();
    if line.is_empty() { None } else { Some(line.to_string()) }
}

fn extract_context_section(source: &str, marker: &str) -> Option<String> {
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if !is_matching_context_heading(line, marker) {
            continue;
        }

        let mut section_lines = Vec::new();
        while let Some(next_line) = lines.peek() {
            if next_line.trim().starts_with('#') {
                break;
            }

            section_lines.push(lines.next().unwrap_or_default());
        }

        let section = trim_context_block(&section_lines.join("\n"));
        if !section.is_empty() {
            return Some(section);
        }
    }

    None
}

fn is_matching_context_heading(line: &str, marker: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    trimmed.trim_start_matches('#').trim().eq_ignore_ascii_case(marker)
}

fn trim_context_block(value: &str) -> String {
    let lines = value.lines().collect::<Vec<_>>();
    let start = lines.iter().position(|line| !line.trim().is_empty());
    let end = lines.iter().rposition(|line| !line.trim().is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => String::new(),
    }
}

fn first_meaningful_line(source: &str) -> String {
    source
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToString::to_string)
        .unwrap_or_else(|| {
            "Bound the problem to the current repository before moving into a planning or execution mode."
                .to_string()
        })
}

fn infer_discovery_next_phase(source: &str) -> String {
    let normalized = source.to_lowercase();
    if normalized.contains("architecture") || normalized.contains("boundary") {
        "Translate this discovery packet into architecture mode with named boundaries, invariants, and explicit tradeoffs."
            .to_string()
    } else if normalized.contains("legacy")
        || normalized.contains("existing")
        || normalized.contains("change")
    {
        "Translate this discovery packet into change mode with preserved invariants and a bounded change surface."
            .to_string()
    } else if normalized.contains("new capability")
        || normalized.contains("system-shaping")
        || normalized.contains("system shaping")
        || normalized.contains("new system")
    {
        "Translate this discovery packet into system-shaping mode with explicit capability boundaries and phased delivery options."
            .to_string()
    } else {
        "Translate this discovery packet into requirements mode with a bounded problem statement, constraints, options, and scope cuts."
            .to_string()
    }
}

fn extract_change_surface_entries(source: &str) -> Vec<String> {
    extract_execution_scope_entries(source, &["change surface"])
}

fn extract_first_marker_entries(source: &str, markers: &[&str]) -> Vec<String> {
    markers
        .iter()
        .find_map(|marker| {
            let entries = extract_marker_entries(source, marker);
            (!entries.is_empty()).then_some(entries)
        })
        .unwrap_or_default()
}

fn extract_execution_scope_entries(source: &str, markers: &[&str]) -> Vec<String> {
    extract_first_marker_entries(source, markers)
}

fn extract_marker_entries(source: &str, marker: &str) -> Vec<String> {
    let normalized = source.to_lowercase();
    let Some(raw_surface) = extract_marker(source, &normalized, marker) else {
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
    let mut lines = remainder.lines();
    let line = lines.next()?.trim();
    if !line.is_empty() {
        return Some(line.to_string());
    }

    let mut section_lines = Vec::new();
    for next_line in lines {
        let trimmed = next_line.trim_end();
        let normalized_line = trimmed.trim();

        if normalized_line.is_empty() {
            if !section_lines.is_empty() {
                break;
            }
            continue;
        }

        if looks_like_inline_marker(normalized_line) || normalized_line.starts_with('#') {
            break;
        }

        section_lines.push(trimmed);
    }

    let section = trim_context_block(&section_lines.join("\n"));
    if section.is_empty() { None } else { Some(section) }
}

fn looks_like_inline_marker(line: &str) -> bool {
    if line.starts_with(['-', '*', '+']) {
        return false;
    }

    let Some((prefix, _)) = line.split_once(':') else {
        return false;
    };
    let prefix = prefix.trim();
    !prefix.is_empty()
        && prefix.chars().all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, ' ' | '-' | '_'))
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
            rationale:
                "Approval is already recorded; resume the run to execute the post-approval continuation."
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
        Mode::Implementation => Some(("implementation.md", "implementation")),
        Mode::Refactor => Some(("refactor.md", "refactor")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EngineService, GateInspectSummary, ModeResultSummary, RecommendedActionSummary,
        ResultActionSummary, RunRequest, apply_execution_posture_summary,
        approved_execution_mutation_rationale, build_action_chips_for,
        canonical_mode_input_binding, capability_tag, execution_continuation_pending,
        extract_change_surface_entries, preserve_multiline_summary, recommend_next_action,
        resolved_execution_posture_label, run_state_from_gates, set_execution_posture,
        set_post_approval_execution_consumed,
    };
    use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
    use crate::domain::execution::ExecutionPosture;
    use crate::domain::gate::{GateEvaluation, GateKind, GateStatus};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::RunState;
    use crate::persistence::store::{
        InitSummary as StoreInitSummary, SkillsSummary as StoreSkillsSummary,
    };
    use canon_adapters::CapabilityKind;
    use tempfile::TempDir;
    use time::OffsetDateTime;

    #[test]
    fn change_surface_entries_prefer_markdown_section_over_inline_summary_mentions() {
        let source = "# Change Brief\n\n## Change Surface\n- session repository\n- auth service\n\nMutation posture: propose bounded legacy transformation within declared change surface: entire repository, adjacent modules";

        let entries = extract_change_surface_entries(source);

        assert_eq!(entries, vec!["session repository".to_string(), "auth service".to_string()]);
    }

    #[test]
    fn change_surface_entries_fall_back_to_inline_marker_and_dedupe_segments() {
        let source = "Summary\n\nChange Surface: auth service, session repository; auth service; token cleanup job";

        let entries = extract_change_surface_entries(source);

        assert_eq!(
            entries,
            vec![
                "auth service".to_string(),
                "session repository".to_string(),
                "token cleanup job".to_string()
            ]
        );
    }

    #[test]
    fn preserve_multiline_summary_keeps_bullets_on_separate_lines() {
        let summary = "## Change Surface\n- first bullet\n- second bullet\n\n## Validation Strategy\n- independent check";

        let normalized = preserve_multiline_summary(summary);

        assert!(normalized.contains("- first bullet\n- second bullet"));
        assert!(normalized.contains("\n\n## Validation Strategy\n- independent check"));
    }

    #[test]
    fn run_state_from_gates_prioritizes_approval_then_blocked_then_completed() {
        let approval_gate = GateEvaluation {
            gate: GateKind::Risk,
            status: GateStatus::NeedsApproval,
            blockers: vec!["approval required".to_string()],
            evaluated_at: OffsetDateTime::UNIX_EPOCH,
        };
        let blocked_gate = GateEvaluation {
            gate: GateKind::Architecture,
            status: GateStatus::Blocked,
            blockers: vec!["missing artifact".to_string()],
            evaluated_at: OffsetDateTime::UNIX_EPOCH,
        };
        let passed_gate = GateEvaluation {
            gate: GateKind::Exploration,
            status: GateStatus::Passed,
            blockers: Vec::new(),
            evaluated_at: OffsetDateTime::UNIX_EPOCH,
        };

        assert_eq!(
            run_state_from_gates(&[passed_gate.clone(), approval_gate]),
            RunState::AwaitingApproval
        );
        assert_eq!(run_state_from_gates(&[passed_gate.clone(), blocked_gate]), RunState::Blocked);
        assert_eq!(run_state_from_gates(&[passed_gate]), RunState::Completed);
    }

    #[test]
    fn recommend_next_action_prefers_evidence_for_completed_runs_without_artifacts() {
        let action = recommend_next_action(RunState::Completed, None, &[], true, &[], &[]);

        assert_eq!(
            action,
            Some(RecommendedActionSummary {
                action: "inspect-evidence".to_string(),
                rationale: "The run completed; inspect the evidence bundle for execution lineage."
                    .to_string(),
                target: None,
            })
        );
    }

    #[test]
    fn recommend_next_action_is_absent_for_completed_runs_with_mode_result() {
        let mode_result = ModeResultSummary {
            headline: "Requirements packet ready for downstream review.".to_string(),
            artifact_packet_summary: "Primary artifact is ready.".to_string(),
            execution_posture: None,
            primary_artifact_title: "Problem Statement".to_string(),
            primary_artifact_path: ".canon/artifacts/run-123/requirements/problem-statement.md"
                .to_string(),
            primary_artifact_action: ResultActionSummary {
                id: "open-primary-artifact".to_string(),
                label: "Open primary artifact".to_string(),
                host_action: "open-file".to_string(),
                target: ".canon/artifacts/run-123/requirements/problem-statement.md"
                    .to_string(),
                text_fallback:
                    "Open the primary artifact at .canon/artifacts/run-123/requirements/problem-statement.md."
                        .to_string(),
            },
            result_excerpt: "Build a bounded USB flashing CLI.".to_string(),
            action_chips: Vec::new(),
        };

        let action = recommend_next_action(
            RunState::Completed,
            Some(&mode_result),
            std::slice::from_ref(&mode_result.primary_artifact_path),
            true,
            &[],
            &[],
        );

        assert_eq!(action, None);
    }

    #[test]
    fn build_action_chips_for_emits_full_frontend_contract_fields() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            ".canon/artifacts/run-123/refactor/preserved-behavior.md",
            "run-123",
        );

        assert_eq!(chips.len(), 3);

        let open_chip = &chips[0];
        assert_eq!(open_chip.id, "open-primary-artifact");
        assert_eq!(open_chip.intent, "Inspect");
        assert_eq!(
            open_chip.text_fallback,
            "Open the primary artifact at .canon/artifacts/run-123/refactor/preserved-behavior.md."
        );

        let inspect_chip = &chips[1];
        assert_eq!(inspect_chip.id, "inspect-evidence");
        assert_eq!(inspect_chip.intent, "Inspect");
        assert!(inspect_chip.recommended);
        assert_eq!(inspect_chip.text_fallback, "Use $canon-inspect-evidence for run run-123.");

        let approve_chip = &chips[2];
        assert_eq!(approve_chip.id, "approve-gate-execution");
        assert_eq!(approve_chip.intent, "GovernedAction");
        assert_eq!(approve_chip.prefilled_args.get("RUN_ID"), Some(&"run-123".to_string()));
        assert_eq!(approve_chip.prefilled_args.get("TARGET"), Some(&"gate:execution".to_string()));
        assert_eq!(
            approve_chip.required_user_inputs,
            vec!["BY".to_string(), "DECISION".to_string(), "RATIONALE".to_string()]
        );
        assert_eq!(
            approve_chip.text_fallback,
            "Review the packet for run run-123, then approve using $canon-approve."
        );
    }

    #[test]
    fn capability_tag_covers_supported_capabilities() {
        let cases = [
            (CapabilityKind::ReadRepository, "context"),
            (CapabilityKind::GenerateContent, "generate"),
            (CapabilityKind::CritiqueContent, "critique"),
            (CapabilityKind::ProposeWorkspaceEdit, "edit"),
            (CapabilityKind::InspectDiff, "inspect-diff"),
            (CapabilityKind::ReadArtifact, "read-artifact"),
            (CapabilityKind::EmitArtifact, "emit-artifact"),
            (CapabilityKind::RunCommand, "run-command"),
            (CapabilityKind::ValidateWithTool, "validate"),
            (CapabilityKind::InvokeStructuredTool, "structured-tool"),
            (CapabilityKind::ExecuteBoundedTransformation, "transform"),
        ];

        for (capability, expected) in cases {
            assert_eq!(capability_tag(capability), expected);
        }
    }

    #[test]
    fn engine_service_helpers_map_store_summaries_and_pr_review_inputs() {
        let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

        let init = EngineService::map_init_summary(StoreInitSummary {
            repo_root: "/repo".to_string(),
            canon_root: "/repo/.canon".to_string(),
            methods_materialized: 12,
            policies_materialized: 5,
            skills_materialized: 19,
            claude_md_created: false,
        });
        assert_eq!(init.repo_root, "/repo");
        assert_eq!(init.methods_materialized, 12);

        let skills = EngineService::map_skills_summary(StoreSkillsSummary {
            skills_dir: "/repo/.agents/skills".to_string(),
            skills_materialized: 19,
            skills_skipped: 2,
            claude_md_created: true,
        });
        assert_eq!(skills.skills_dir, "/repo/.agents/skills");
        assert_eq!(skills.skills_skipped, 2);
        assert!(skills.claude_md_created);

        let refs = service
            .load_pr_review_refs(&["origin/main".to_string(), "HEAD".to_string()])
            .expect("two refs should parse");
        assert_eq!(refs, ("origin/main".to_string(), "HEAD".to_string()));

        let error = service
            .load_pr_review_refs(&["origin/main".to_string()])
            .expect_err("missing head ref should fail");
        assert!(error.to_string().contains("pr-review requires two inputs"));
    }

    #[test]
    fn engine_service_resolves_relative_and_absolute_input_paths() {
        let service = EngineService::new("/tmp/canon-root");

        assert_eq!(
            service.resolve_input_path("idea.md"),
            std::path::PathBuf::from("/tmp/canon-root").join("idea.md")
        );
        assert_eq!(
            service.resolve_input_path("/tmp/elsewhere/input.md"),
            std::path::PathBuf::from("/tmp/elsewhere/input.md")
        );
    }

    #[test]
    fn canonical_mode_input_binding_is_defined_for_promoted_execution_modes() {
        assert_eq!(
            canonical_mode_input_binding(Mode::Implementation),
            Some(("implementation.md", "implementation"))
        );
        assert_eq!(canonical_mode_input_binding(Mode::Refactor), Some(("refactor.md", "refactor")));
        assert_eq!(canonical_mode_input_binding(Mode::Requirements), None);
    }

    #[test]
    fn auto_bind_canonical_mode_inputs_prefers_directory_over_single_file() {
        let workspace = TempDir::new().expect("temp dir");
        let canon_input = workspace.path().join("canon-input");
        std::fs::create_dir_all(canon_input.join("implementation")).expect("implementation dir");
        std::fs::write(
            canon_input.join("implementation").join("brief.md"),
            "# Implementation Brief\n\nMutation Bounds: src/auth/**\n",
        )
        .expect("implementation brief");
        std::fs::write(
            canon_input.join("implementation.md"),
            "# Implementation Brief\n\nMutation Bounds: src/auth/**\n",
        )
        .expect("implementation file");

        let service = EngineService::new(workspace.path());

        assert_eq!(
            service.auto_bind_canonical_mode_inputs(Mode::Implementation, &[], &[]),
            vec!["canon-input/implementation".to_string()]
        );
    }

    #[test]
    fn auto_bind_canonical_mode_inputs_uses_single_file_when_directory_is_absent() {
        let workspace = TempDir::new().expect("temp dir");
        let canon_input = workspace.path().join("canon-input");
        std::fs::create_dir_all(&canon_input).expect("canon-input dir");
        std::fs::write(
            canon_input.join("refactor.md"),
            "# Refactor Brief\n\nPreserved Behavior: public API remains stable.\n",
        )
        .expect("refactor file");

        let service = EngineService::new(workspace.path());

        assert_eq!(
            service.auto_bind_canonical_mode_inputs(Mode::Refactor, &[], &[]),
            vec!["canon-input/refactor.md".to_string()]
        );
    }

    #[test]
    fn build_run_context_scaffolds_implementation_execution_metadata() {
        let service = EngineService::new("/tmp/canon-root");
        let request = RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            inputs: vec!["canon-input/implementation.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        };

        let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
        let implementation =
            context.implementation_execution.expect("implementation execution scaffold");

        assert!(context.refactor_execution.is_none());
        assert!(context.upstream_context.is_none());
        assert_eq!(implementation.plan_sources, vec!["canon-input/implementation.md"]);
        assert_eq!(
            implementation.mutation_bounds.source_refs,
            vec!["canon-input/implementation.md"]
        );
        assert_eq!(implementation.mutation_bounds.owners, vec!["staff-engineer"]);
        assert_eq!(implementation.execution_posture, ExecutionPosture::RecommendationOnly);
        assert!(!implementation.post_approval_execution_consumed);
    }

    #[test]
    fn build_run_context_scaffolds_refactor_execution_metadata() {
        let service = EngineService::new("/tmp/canon-root");
        let request = RunRequest {
            mode: Mode::Refactor,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            inputs: vec!["canon-input/refactor.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        };

        let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
        let refactor = context.refactor_execution.expect("refactor execution scaffold");

        assert!(context.implementation_execution.is_none());
        assert!(context.upstream_context.is_none());
        assert_eq!(refactor.refactor_scope.source_refs, vec!["canon-input/refactor.md"]);
        assert_eq!(refactor.refactor_scope.owners, vec!["staff-engineer"]);
        assert_eq!(refactor.execution_posture, ExecutionPosture::RecommendationOnly);
        assert!(!refactor.post_approval_execution_consumed);
    }

    #[test]
    fn build_run_context_extracts_upstream_context_from_folder_packet() {
        let workspace = TempDir::new().expect("temp dir");
        let packet_root = workspace.path().join("canon-input").join("implementation");
        std::fs::create_dir_all(&packet_root).expect("packet root");
        std::fs::write(
            packet_root.join("brief.md"),
            "# Implementation Brief\n\nFeature Slice: auth session revocation\nPrimary Upstream Mode: change\nTask Mapping: 1. Thread the helper through the revocation service.\nMutation Bounds: src/auth/session.rs\nAllowed Paths:\n- src/auth/session.rs\nSafety-Net Evidence: session contract coverage exists.\nIndependent Checks:\n- cargo test --test session_contract\nRollback Triggers: formatting drift\nRollback Steps: revert the bounded patch\n",
        )
        .expect("brief");
        std::fs::write(
            packet_root.join("source-map.md"),
            "# Source Map\n\n## Upstream Sources\n\n- docs/changes/R-20260422-AUTHREVOC/change-surface.md\n- docs/changes/R-20260422-AUTHREVOC/implementation-plan.md\n\n## Carried-Forward Decisions\n\n- Revocation output formatting stays stable.\n- Contract coverage must pass before and after mutation.\n\n## Excluded Upstream Scope\n\nLogin UI flow and token issuance remain out of scope.\n",
        )
        .expect("source map");

        let service = EngineService::new(workspace.path());
        let request = RunRequest {
            mode: Mode::Implementation,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(crate::domain::run::SystemContext::Existing),
            classification: crate::domain::run::ClassificationProvenance::explicit(),
            owner: "staff-engineer".to_string(),
            inputs: vec!["canon-input/implementation".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        };

        let context = service.build_run_context(&request, Vec::new(), OffsetDateTime::UNIX_EPOCH);
        let upstream = context.upstream_context.expect("upstream context");

        assert_eq!(upstream.feature_slice.as_deref(), Some("auth session revocation"));
        assert_eq!(upstream.primary_upstream_mode.as_deref(), Some("change"));
        assert_eq!(
            upstream.source_refs,
            vec![
                "docs/changes/R-20260422-AUTHREVOC/change-surface.md".to_string(),
                "docs/changes/R-20260422-AUTHREVOC/implementation-plan.md".to_string(),
            ]
        );
        assert_eq!(
            upstream.carried_forward_items,
            vec![
                "Revocation output formatting stays stable.".to_string(),
                "Contract coverage must pass before and after mutation.".to_string(),
            ]
        );
        assert_eq!(
            upstream.excluded_upstream_scope.as_deref(),
            Some("Login UI flow and token issuance remain out of scope.")
        );
    }

    #[test]
    fn apply_execution_posture_summary_reads_recommendation_only_from_run_context() {
        let mode_result = ModeResultSummary {
            headline: "Implementation packet ready.".to_string(),
            artifact_packet_summary: "Primary artifact is ready.".to_string(),
            execution_posture: None,
            primary_artifact_title: "Task Mapping".to_string(),
            primary_artifact_path: ".canon/artifacts/run-123/implementation/task-mapping.md"
                .to_string(),
            primary_artifact_action: ResultActionSummary {
                id: "open-primary-artifact".to_string(),
                label: "Open primary artifact".to_string(),
                host_action: "open-file".to_string(),
                target: ".canon/artifacts/run-123/implementation/task-mapping.md"
                    .to_string(),
                text_fallback:
                    "Open the primary artifact at .canon/artifacts/run-123/implementation/task-mapping.md."
                        .to_string(),
            },
            result_excerpt: "Bounded implementation summary".to_string(),
            action_chips: Vec::new(),
        };
        let context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "staff-engineer".to_string(),
                inputs: vec!["canon-input/implementation.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );

        let summarized = apply_execution_posture_summary(Some(mode_result), Some(&context), &[])
            .expect("summarized mode result");

        assert_eq!(summarized.execution_posture.as_deref(), Some("recommendation-only"));
    }

    #[test]
    fn resolved_execution_posture_label_promotes_approved_execution_runs() {
        let mut context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Refactor,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "staff-engineer".to_string(),
                inputs: vec!["canon-input/refactor.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        set_post_approval_execution_consumed(&mut context, true);
        let approvals = vec![ApprovalRecord::for_gate(
            GateKind::Execution,
            "maintainer".to_string(),
            ApprovalDecision::Approve,
            "approved bounded execution".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )];

        assert_eq!(
            resolved_execution_posture_label(Some(&context), &approvals).as_deref(),
            Some("approved-recommendation")
        );
    }

    #[test]
    fn resolved_execution_posture_label_keeps_recommendation_only_until_resume_runs() {
        let context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "staff-engineer".to_string(),
                inputs: vec!["canon-input/implementation.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        let approvals = vec![ApprovalRecord::for_gate(
            GateKind::Execution,
            "maintainer".to_string(),
            ApprovalDecision::Approve,
            "approved bounded execution".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )];

        assert_eq!(
            resolved_execution_posture_label(Some(&context), &approvals).as_deref(),
            Some("recommendation-only")
        );
    }

    #[test]
    fn resolve_identity_prefers_explicit_values() {
        let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

        assert_eq!(
            service.resolve_owner("  Owner <owner@example.com>  "),
            "Owner <owner@example.com>".to_string()
        );
        assert_eq!(
            service.resolve_approver("Reviewer <reviewer@example.com>"),
            "Reviewer <reviewer@example.com>".to_string()
        );
    }

    #[test]
    fn recommend_next_action_prefers_artifact_review_for_approval_gated_runs() {
        let action = recommend_next_action(
            RunState::AwaitingApproval,
            None,
            &[".canon/artifacts/run-123/change/system-slice.md".to_string()],
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
            None,
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
    fn recommend_next_action_points_to_resume_when_post_approval_continuation_is_pending() {
        let action = recommend_next_action(RunState::AwaitingApproval, None, &[], true, &[], &[]);

        assert_eq!(
            action,
            Some(RecommendedActionSummary {
                action: "resume".to_string(),
                rationale: "Approval is already recorded; resume the run to execute the post-approval continuation.".to_string(),
                target: None,
            })
        );
    }

    #[test]
    fn build_action_chips_for_emits_resume_when_awaiting_continuation_without_targets() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &[],
            ".canon/artifacts/run-123/implementation/task-mapping.md",
            "run-123",
        );

        assert_eq!(chips.len(), 3);
        let inspect_chip = &chips[1];
        assert!(!inspect_chip.recommended);

        let resume_chip = &chips[2];
        assert_eq!(resume_chip.id, "resume-run");
        assert_eq!(resume_chip.label, "Resume run");
        assert_eq!(resume_chip.skill, "canon-resume");
        assert_eq!(resume_chip.prefilled_args.get("RUN_ID"), Some(&"run-123".to_string()));
        assert_eq!(
            resume_chip.text_fallback,
            "Use $canon-resume for run run-123 to continue post-approval execution."
        );
        assert!(resume_chip.recommended);
    }

    #[test]
    fn recommend_next_action_uses_evidence_for_blocked_runs_without_artifacts() {
        let action = recommend_next_action(
            RunState::Blocked,
            None,
            &[],
            true,
            &[GateInspectSummary {
                gate: "change-preservation".to_string(),
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

    #[test]
    fn set_execution_posture_updates_implementation_context() {
        let mut context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "maintainer".to_string(),
                inputs: vec!["canon-input/implementation.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        assert_eq!(
            context.implementation_execution.as_ref().unwrap().execution_posture,
            ExecutionPosture::RecommendationOnly
        );
        set_execution_posture(&mut context, ExecutionPosture::Mutating);
        assert_eq!(
            context.implementation_execution.as_ref().unwrap().execution_posture,
            ExecutionPosture::Mutating
        );
    }

    #[test]
    fn set_execution_posture_updates_refactor_context() {
        let mut context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Refactor,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "maintainer".to_string(),
                inputs: vec!["canon-input/refactor.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        set_execution_posture(&mut context, ExecutionPosture::Mutating);
        assert_eq!(
            context.refactor_execution.as_ref().unwrap().execution_posture,
            ExecutionPosture::Mutating
        );
    }

    #[test]
    fn set_post_approval_execution_consumed_updates_implementation_context() {
        let mut context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "maintainer".to_string(),
                inputs: vec!["canon-input/implementation.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        assert!(
            !context.implementation_execution.as_ref().unwrap().post_approval_execution_consumed
        );
        set_post_approval_execution_consumed(&mut context, true);
        assert!(
            context.implementation_execution.as_ref().unwrap().post_approval_execution_consumed
        );
    }

    #[test]
    fn execution_continuation_pending_is_true_when_gate_approved_and_not_consumed() {
        let context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "maintainer".to_string(),
                inputs: vec!["canon-input/implementation.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        let approvals = vec![ApprovalRecord::for_gate(
            GateKind::Execution,
            "maintainer".to_string(),
            ApprovalDecision::Approve,
            "approved".to_string(),
            OffsetDateTime::UNIX_EPOCH,
        )];
        assert!(execution_continuation_pending(&context, &approvals));
    }

    #[test]
    fn execution_continuation_pending_is_false_when_gate_not_approved() {
        let context = EngineService::new("/tmp/canon-root").build_run_context(
            &RunRequest {
                mode: Mode::Refactor,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(crate::domain::run::SystemContext::Existing),
                classification: crate::domain::run::ClassificationProvenance::explicit(),
                owner: "maintainer".to_string(),
                inputs: vec!["canon-input/refactor.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            },
            Vec::new(),
            OffsetDateTime::UNIX_EPOCH,
        );
        assert!(!execution_continuation_pending(&context, &[]));
    }

    #[test]
    fn approved_execution_mutation_rationale_covers_mode_variants() {
        let scope = vec!["src/auth/session.rs".to_string()];
        let impl_label =
            approved_execution_mutation_rationale(Mode::Implementation, &scope, "patch.diff");
        assert!(impl_label.contains("implementation mutation"));
        assert!(impl_label.contains("patch.diff"));

        let refactor_label =
            approved_execution_mutation_rationale(Mode::Refactor, &scope, "patch.diff");
        assert!(refactor_label.contains("refactor mutation"));

        let change_label =
            approved_execution_mutation_rationale(Mode::Change, &scope, "patch.diff");
        assert!(change_label.contains("change mutation"));

        let other_label =
            approved_execution_mutation_rationale(Mode::Requirements, &scope, "patch.diff");
        assert!(other_label.contains("bounded mutation"));
    }
}
