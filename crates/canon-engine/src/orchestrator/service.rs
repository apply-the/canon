use std::collections::BTreeSet;
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
    render_domain_language_artifact, render_domain_model_artifact, render_implementation_artifact,
    render_incident_artifact, render_migration_artifact, render_pr_review_artifact,
    render_refactor_artifact, render_requirements_artifact_from_evidence, render_review_artifact,
    render_security_assessment_artifact, render_supply_chain_analysis_artifact,
    render_system_assessment_artifact, render_system_shaping_artifact,
    render_verification_artifact,
};
use crate::domain::approval::{ApprovalDecision, ApprovalRecord};
use crate::domain::artifact::{
    ArtifactRecord, ArtifactRequirement, RuntimePacketMetadata, artifact_slug, is_packet_sidecar,
    is_special_repository_directory,
};
use crate::domain::execution::{
    DeniedInvocation, EvidenceBundle, ExecutionPosture, GenerationPath, InvocationAttempt,
    InvocationRequest, MutationBounds, MutationExpansionPolicy, PolicyDecisionKind, ToolOutcome,
    ToolOutcomeKind, ValidationPath,
};
use crate::domain::gate::{GateKind, GateStatus};
use crate::domain::mode::{Mode, all_mode_profiles};
use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::publish_profile::{
    AdaptiveGovernanceV1Envelope, AdaptiveGovernanceV1RuntimeInputs, AuthorityApprovalState,
    AuthorityGovernanceV1Envelope, AuthorityGovernanceV1RuntimeInputs, AuthorityPacketReadiness,
};
use crate::domain::run::{
    ClarificationRefinementContext, ClassificationProvenance, ImplementationExecutionContext,
    InlineInput, InputFingerprint, InputSourceKind, ReadinessDeltaItem, RefactorExecutionContext,
    RunContext, RunIdentity, RunState, SystemContext, UpstreamContext,
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

mod context_builder;
mod identity;
mod input_handling;
mod inspect;
mod mode_backlog;
mod mode_change;
mod mode_discovery;
mod mode_domain_language;
mod mode_domain_model;
mod mode_incident;
mod mode_migration;
mod mode_pr_review;
mod mode_requirements;
mod mode_review;
mod mode_security_assessment;
mod mode_shaping;
mod mode_supply_chain_analysis;
mod mode_system_assessment;
mod run_lifecycle;
mod run_op;
mod run_summary;
mod support;

use clarity::*;
use context_parse::*;
use execution::*;
use next_action::*;
use patch::*;
use summarizers::*;
use support::*;

/// Errors returned by the Orchestrator service.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Standard I/O error during file or directory operations.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// The specified inspection target (e.g. invalid run ID or mode) is not supported.
    #[error("unsupported inspect target: {0}")]
    UnsupportedInspectTarget(String),
    /// A configuration or input validation check failed.
    #[error("validation failed: {0}")]
    Validation(String),
    /// The requested `Mode` execution logic is not yet implemented.
    #[error("mode `{0}` is not implemented yet")]
    UnsupportedMode(String),
}

/// The specific domain of run state or configuration being inspected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectTarget {
    /// List all supported governance modes.
    Modes,
    /// List available methods/capabilities.
    Methods,
    /// List active governance policies.
    Policies,
    /// Inspect the risk classification and usage zone for a given pattern.
    RiskZone {
        /// The mode to evaluate classification for.
        mode: Mode,
        /// An explicit risk class override, if provided.
        risk: Option<RiskClass>,
        /// An explicit usage zone override, if provided.
        zone: Option<UsageZone>,
        /// File path inputs to feed to the classifier.
        inputs: Vec<String>,
        /// Inline text inputs to feed to the classifier.
        inline_inputs: Vec<String>,
    },
    /// Inspect output-quality clarity for the named mode and inputs.
    Clarity {
        /// The mode to evaluate clarity for.
        mode: Mode,
        /// File path inputs to inspect.
        inputs: Vec<String>,
    },
    /// List artifact paths for the named run.
    Artifacts {
        /// The run ID to inspect.
        run_id: String,
    },
    /// List invocation records for the named run.
    Invocations {
        /// The run ID to inspect.
        run_id: String,
    },
    /// List evidence records for the named run.
    Evidence {
        /// The run ID to inspect.
        run_id: String,
    },
    /// Inspect run-scoped clarification refinement state for the named run.
    Refinement {
        /// The run ID to inspect.
        run_id: String,
    },
}

/// Request parameters for a new governed run, containing all governance and execution constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    /// The specific workflow mode (e.g., Change, Discovery).
    pub mode: Mode,
    /// The risk classification of the proposed work.
    pub risk: RiskClass,
    /// The operational zone (e.g., Red, Yellow, Green) for the run.
    pub zone: UsageZone,
    /// Whether the work target is a new system or an existing one.
    pub system_context: Option<SystemContext>,
    /// Metadata describing how the risk/zone was determined.
    pub classification: ClassificationProvenance,
    /// The human or entity responsible for the run.
    pub owner: String,
    /// Paths to input files containing requirements or specs.
    pub inputs: Vec<String>,
    /// Raw input content provided directly.
    pub inline_inputs: Vec<String>,
    /// Paths to exclude from analysis or mutation.
    pub excluded_paths: Vec<String>,
    /// Custom policy root directory.
    pub policy_root: Option<String>,
    /// Custom method root directory.
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

/// The AI tool frontend the user is running Canon from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTool {
    /// OpenAI Codex CLI or compatible Codex-family agent.
    Codex,
    /// GitHub Copilot in VS Code or Copilot CLI.
    Copilot,
    /// Anthropic Claude desktop or Claude Code.
    Claude,
    /// Cursor IDE or Cursor agent flows.
    Cursor,
    /// Antigravity assistant host integrations.
    Antigravity,
}

impl AiTool {
    fn materialization_target(self) -> SkillMaterializationTarget {
        match self {
            Self::Codex | Self::Copilot | Self::Cursor | Self::Antigravity => {
                SkillMaterializationTarget::Agents
            }
            Self::Claude => SkillMaterializationTarget::Claude,
        }
    }
}

/// Summary returned after a successful `canon init` operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InitSummary {
    /// Absolute path to the repository root.
    pub repo_root: String,
    /// Absolute path to the `.canon/` runtime directory.
    pub canon_root: String,
    /// Number of method files materialized.
    pub methods_materialized: usize,
    /// Number of policy files materialized.
    pub policies_materialized: usize,
    /// Number of skill files materialized.
    pub skills_materialized: usize,
    /// Whether a `CLAUDE.md` file was created or updated.
    pub claude_md_created: bool,
}

/// Summary returned after a `canon skills` materialization run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillsSummary {
    /// Path to the skills directory that was populated.
    pub skills_dir: String,
    /// Number of skill files successfully materialized.
    pub skills_materialized: usize,
    /// Number of skill files skipped (already up to date).
    pub skills_skipped: usize,
    /// Whether a `CLAUDE.md` file was created or updated.
    pub claude_md_created: bool,
}

/// A single skill entry in a skills listing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillEntry {
    /// The skill name (e.g. `canon-implementation`).
    pub name: String,
    /// The support state of the skill (e.g. `available`, `preview`).
    pub support_state: String,
}

/// The response returned by a `canon inspect` command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InspectResponse {
    /// The inspection target label (e.g. `"modes"`, `"risk-zone"`).
    pub target: String,
    /// System context of the inspected run, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_context: Option<String>,
    /// The ordered list of inspection result entries.
    pub entries: Vec<InspectEntry>,
}

/// A polymorphic entry in an [`InspectResponse`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum InspectEntry {
    /// A plain name string (e.g. a mode or policy name).
    Name(String),
    /// A risk/zone classification summary.
    RiskZone(ClassificationInspectSummary),
    /// A clarity inspection summary.
    Clarity(ClarityInspectSummary),
    /// An invocation inspection summary.
    Invocation(InvocationInspectSummary),
    /// An evidence inspection summary.
    Evidence(EvidenceInspectSummary),
    /// A refinement inspection summary.
    Refinement(RefinementInspectSummary),
}

/// Inspection summary for a single invocation within a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InvocationInspectSummary {
    /// The invocation request ID.
    pub request_id: String,
    /// The adapter that was (or would be) invoked.
    pub adapter: String,
    /// The capability kind.
    pub capability: String,
    /// The execution orientation.
    pub orientation: String,
    /// The policy decision applied to this invocation.
    pub policy_decision: String,
    /// Whether the invocation was constrained to recommendation-only.
    pub recommendation_only: bool,
    /// Whether approval was granted, denied, or pending.
    pub approval_state: String,
    /// The outcome kind of the latest attempt, if any.
    pub latest_outcome: Option<String>,
    /// Artifact paths linked to this invocation.
    pub linked_artifacts: Vec<String>,
}

/// A compact representation of a closure finding for inspection output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClosureFindingInspectSummary {
    /// Category label for the finding.
    pub category: String,
    /// Severity of the finding (`"warning"` or `"blocking"`).
    pub severity: String,
    /// The artifact or surface scope affected.
    pub affected_scope: String,
    /// Recommended action to resolve the finding.
    pub recommended_followup: String,
}

/// Full evidence inspection summary for a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EvidenceInspectSummary {
    /// Execution posture of the run (`"mutating"` or `"recommendation-only"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_posture: Option<String>,
    /// Feature slice from the upstream run, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_feature_slice: Option<String>,
    /// Primary mode of the upstream run, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_upstream_mode: Option<String>,
    /// Source references from the upstream context.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub upstream_source_refs: Vec<String>,
    /// Items carried forward from upstream.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub carried_forward_items: Vec<String>,
    /// Excluded upstream scope, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_upstream_scope: Option<String>,
    /// Closure status of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_status: Option<String>,
    /// Decomposition scope of the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition_scope: Option<String>,
    /// Closure findings for this run.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub closure_findings: Vec<ClosureFindingInspectSummary>,
    /// Optional notes about the closure assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_notes: Option<String>,
    /// Summary labels for each generation path.
    pub generation_paths: Vec<String>,
    /// Summary labels for each validation path.
    pub validation_paths: Vec<String>,
    /// Summary labels for each denied invocation.
    pub denied_invocations: Vec<String>,
    /// Artifact provenance links.
    pub artifact_provenance_links: Vec<String>,
}

/// Risk/zone classification inspection summary for a run or set of inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassificationInspectSummary {
    /// The mode being classified.
    pub mode: String,
    /// The inferred or supplied risk class.
    pub risk: String,
    /// The inferred or supplied usage zone.
    pub zone: String,
    /// Whether the risk was explicitly supplied by the user.
    pub risk_was_supplied: bool,
    /// Whether the zone was explicitly supplied by the user.
    pub zone_was_supplied: bool,
    /// Confidence level of the classification.
    pub confidence: String,
    /// Whether the user must confirm the classification before proceeding.
    pub requires_confirmation: bool,
    /// One-line headline summarizing the classification result.
    pub headline: String,
    /// Full rationale for the combined risk/zone classification.
    pub rationale: String,
    /// Rationale specific to the risk class.
    pub risk_rationale: String,
    /// Rationale specific to the usage zone.
    pub zone_rationale: String,
    /// Combined signals that influenced the classification.
    pub signals: Vec<String>,
    /// Signals that specifically influenced the risk class.
    pub risk_signals: Vec<String>,
    /// Signals that specifically influenced the usage zone.
    pub zone_signals: Vec<String>,
}

/// A single clarification question surfaced during a clarity inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClarificationQuestionSummary {
    /// Unique identifier for this question.
    pub id: String,
    /// The question text to present to the user.
    pub prompt: String,
    /// Why this question is being asked.
    pub rationale: String,
    /// The evidence gap or signal that triggered this question.
    pub evidence: String,
    /// Which downstream decisions or artifacts are affected if skipped.
    pub affects: String,
    /// The default assumption made if the user skips this question.
    pub default_if_skipped: String,
    /// The current status of the question (`"open"` or `"resolved"`).
    pub status: String,
}

/// Full clarity inspection summary for a set of mode inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClarityInspectSummary {
    /// The mode being assessed.
    pub mode: String,
    /// One-sentence summary of the clarity assessment.
    pub summary: String,
    /// The input paths that were analyzed.
    pub source_inputs: Vec<String>,
    /// Whether the inputs require clarification before a run can proceed.
    pub requires_clarification: bool,
    /// Specific context items that are missing or underspecified.
    pub missing_context: Vec<String>,
    /// Targeted questions to surface missing context.
    pub clarification_questions: Vec<ClarificationQuestionSummary>,
    /// Reasoning signals that informed the clarity assessment.
    pub reasoning_signals: Vec<String>,
    /// Output quality assessment for the current inputs.
    pub output_quality: OutputQualitySummary,
    /// Authoring lifecycle assessment for the current inputs.
    pub authoring_lifecycle: AuthoringLifecycleSummary,
    /// The most important next step for the author.
    pub recommended_focus: String,
}

/// Assessment of the output quality achievable from the current inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OutputQualitySummary {
    /// Quality posture label (e.g. `"high"`, `"medium"`, `"low"`).
    pub posture: String,
    /// Whether the inputs are materially closed (no critical gaps).
    pub materially_closed: bool,
    /// Evidence signals that support the quality assessment.
    pub evidence_signals: Vec<String>,
    /// Reasons why the quality was downgraded from ideal.
    pub downgrade_reasons: Vec<String>,
}

/// Assessment of where the inputs sit in the authoring lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthoringLifecycleSummary {
    /// The structural shape of the input packet.
    pub packet_shape: String,
    /// Whether the inputs are authoritative, derived, or ambiguous.
    pub authority_status: String,
    /// Input paths considered authoritative for this mode.
    pub authoritative_inputs: Vec<String>,
    /// Input paths that are supporting but not primary.
    pub supporting_inputs: Vec<String>,
    /// Delta steps needed to reach full authoring readiness.
    pub readiness_delta: Vec<String>,
    /// The single most impactful next authoring action.
    pub next_authoring_step: String,
}

/// A suggested follow-up action for the user after a run completes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResultActionSummary {
    /// Unique identifier for this action.
    pub id: String,
    /// Human-readable label for the action.
    pub label: String,
    /// The host action type (e.g. `"open-file"`, `"run-command"`).
    pub host_action: String,
    /// The target of the action (file path, URL, or command).
    pub target: String,
    /// Plain-text fallback description if the host cannot render the action.
    pub text_fallback: String,
}

/// An action chip rendered in the Canon UX for a suggested next step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionChip {
    /// Unique identifier for this chip.
    pub id: String,
    /// Short label shown in the chip button.
    pub label: String,
    /// The skill invoked when the chip is activated.
    pub skill: String,
    /// The user intent this chip represents.
    pub intent: String,
    /// Pre-filled argument values for the skill invocation.
    pub prefilled_args: std::collections::BTreeMap<String, String>,
    /// Argument keys the user must supply before the skill can run.
    pub required_user_inputs: Vec<String>,
    /// Condition under which this chip should be shown.
    pub visibility_condition: String,
    /// Whether this chip is the recommended primary next action.
    pub recommended: bool,
    /// Plain-text fallback shown if the host cannot render chips.
    pub text_fallback: String,
}

/// The mode-specific result excerpt and navigation surface for a completed run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModeResultSummary {
    /// One-line headline of the result.
    pub headline: String,
    /// One-line summary of the emitted artifact packet.
    pub artifact_packet_summary: String,
    /// Execution posture of the run, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_posture: Option<String>,
    /// Title of the primary artifact.
    pub primary_artifact_title: String,
    /// File path of the primary artifact.
    pub primary_artifact_path: String,
    /// Action to open or navigate to the primary artifact.
    pub primary_artifact_action: ResultActionSummary,
    /// A short excerpt from the primary artifact for inline display.
    pub result_excerpt: String,
    /// Suggested next-step action chips.
    #[serde(default)]
    pub action_chips: Vec<ActionChip>,
}

/// Advisory continuation candidate surfaced as part of refinement state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RefinementCandidateSummary {
    /// Candidate run identifier.
    pub run_id: String,
    /// Candidate mode label.
    pub mode: String,
    /// Candidate lifecycle state label.
    pub state: String,
    /// Human-readable explanation for the advisory match.
    pub match_reason: String,
    /// Candidate detection is always advisory.
    pub advisory: bool,
}

/// Advisory continuation candidate surfaced as a top-level inspect payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SuggestedContinuationSummary {
    /// Candidate run identifier.
    pub run_id: String,
    /// Candidate mode label.
    pub mode: String,
    /// Candidate lifecycle state label.
    pub state: String,
    /// Human-readable explanation for the advisory match.
    pub match_reason: String,
    /// Candidate detection is always advisory.
    pub advisory: bool,
    /// Mutation remains blocked until explicit continuation intent is captured.
    pub mutation_allowed: bool,
}

/// Run-scoped clarification record exposed through `inspect refinement`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RefinementClarificationRecordSummary {
    /// Stable clarification record identifier.
    pub id: String,
    /// Prompt shown to the operator.
    pub prompt: String,
    /// Answer text or applied default text.
    pub answer: String,
    /// Whether the answer was explicit, defaulted, or deferred.
    pub answer_kind: String,
    /// Working-brief sections or readiness surfaces affected by the answer.
    pub affected_sections: Vec<String>,
    /// Current resolution state for the record.
    pub resolution_state: String,
    /// When the answer or default was recorded.
    pub recorded_at: time::OffsetDateTime,
}

/// Successor lineage surfaced through `inspect refinement`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RefinementLineageSummary {
    /// Prior run whose context was carried forward.
    pub carried_from: String,
    /// Prior run replaced for forward work.
    pub supersedes: String,
    /// Explanation for the mode redirection.
    pub mode_change_reason: String,
}

/// Structured readiness item included in refinement summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RefinementReadinessItemSummary {
    /// Stable readiness item identifier.
    pub id: String,
    /// Section or lifecycle area affected.
    pub section: String,
    /// Human-readable readiness summary.
    pub summary: String,
    /// Whether the item blocks readiness.
    pub blocking: bool,
    /// Source category for the readiness item.
    pub source_kind: String,
    /// Whether a safe default is available.
    pub default_available: bool,
    /// Whether the item is already resolved.
    pub resolved: bool,
}

/// Shared refinement summary surfaced through status and inspect flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RefinementStateSummary {
    /// Workflow family label for the refinement session.
    pub workflow_family: String,
    /// Current target mode label.
    pub current_mode: String,
    /// Run-local working brief artifact path.
    pub working_brief_path: String,
    /// Template or method source used to seed the working brief.
    pub template_ref: String,
    /// Current refinement lifecycle status.
    pub status: String,
    /// Whether explicit continuation is still required.
    pub explicit_continuation_required: bool,
    /// Authoritative input references that seed the working brief.
    pub authoritative_input_refs: Vec<String>,
    /// Supporting input references retained as provenance.
    pub supporting_input_refs: Vec<String>,
    /// Total clarification records currently retained.
    pub records_total: usize,
    /// Number of clarification records that are not yet resolved.
    pub unresolved_records: usize,
    /// Flat readiness-delta strings derived from the structured items.
    pub readiness_delta: Vec<String>,
    /// Structured readiness items for detailed inspect flows.
    pub readiness_items: Vec<RefinementReadinessItemSummary>,
    /// Advisory continuation candidate, if available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_candidate: Option<RefinementCandidateSummary>,
}

/// Detailed run-scoped refinement inspect payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RefinementInspectSummary {
    /// Run identifier.
    pub run_id: String,
    /// Current governed mode label.
    pub mode: String,
    /// Current run lifecycle state label.
    pub state: String,
    /// Canonical run-local working brief path.
    pub working_brief_path: String,
    /// Immutable authoritative inputs that seed the working brief.
    pub authoritative_inputs: Vec<String>,
    /// Supporting inputs retained as provenance.
    pub supporting_inputs: Vec<String>,
    /// Clarification records retained for the working brief.
    pub clarification_records: Vec<RefinementClarificationRecordSummary>,
    /// Structured readiness items preserved for inspect consumers.
    pub readiness_delta: Vec<RefinementReadinessItemSummary>,
    /// Advisory continuation candidate when Canon detects one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_continuation: Option<SuggestedContinuationSummary>,
    /// Successor lineage for redirected started work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage: Option<RefinementLineageSummary>,
}

/// A summary of a completed or in-progress run, suitable for display or API consumption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunSummary {
    /// The human-readable run ID (e.g. `20240513-abcd`).
    pub run_id: String,
    /// The unique machine-readable UUID for the run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// The owner of the run.
    pub owner: String,
    /// The governance mode name.
    pub mode: String,
    /// The risk classification name.
    pub risk: String,
    /// The governance zone name.
    pub zone: String,
    /// The system context (New vs Existing) as a string.
    pub system_context: Option<String>,
    /// Current execution state (e.g. Completed, Blocked).
    pub state: String,
    /// Total number of artifacts emitted.
    pub artifact_count: usize,
    /// Total number of tool invocations attempted.
    pub invocations_total: usize,
    /// Number of invocations denied by policy.
    pub invocations_denied: usize,
    /// Number of invocations waiting for manual approval.
    pub invocations_pending_approval: usize,
    /// If blocked, the classification triggering the block.
    pub blocking_classification: Option<String>,
    /// List of specific gates that are currently blocked.
    pub blocked_gates: Vec<GateInspectSummary>,
    /// List of targets requiring approval (e.g. files, shell commands).
    pub approval_targets: Vec<String>,
    /// Paths to all emitted artifacts.
    pub artifact_paths: Vec<String>,
    /// High-level status of the run's closure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_status: Option<String>,
    /// Scope of the decomposition if this is a parent run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decomposition_scope: Option<String>,
    /// Specific findings from the run's closure.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub closure_findings: Vec<ClosureFindingInspectSummary>,
    /// Detailed notes regarding the closure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closure_notes: Option<String>,
    /// List of actions that can be taken next.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub possible_actions: Vec<PossibleActionSummary>,
    /// Additive refinement state for draft or successor-aware continuation flows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refinement_state: Option<RefinementStateSummary>,
    /// Detailed mode-specific result summary if available.
    pub mode_result: Option<ModeResultSummary>,
    /// The single most important next step for the user.
    pub recommended_next_action: Option<RecommendedActionSummary>,
}

/// A snapshot of the runtime status of a run, focused on operational blockers and progress.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusSummary {
    /// The run being tracked.
    pub run: String,
    /// The human owner.
    pub owner: String,
    /// Simplified state string.
    pub state: String,
    /// System context of the run.
    pub system_context: Option<String>,
    /// Total tool invocations attempted so far.
    pub invocations_total: usize,
    /// Number of invocations waiting for approval.
    pub pending_invocation_approvals: usize,
    /// Whether the 'Validation Independence' policy constraint is met.
    pub validation_independence_satisfied: bool,
    /// If blocked, the classification triggering the block.
    pub blocking_classification: Option<String>,
    /// List of gates currently preventing progress.
    pub blocked_gates: Vec<GateInspectSummary>,
    /// Identifiers of pending approvals the run is waiting for.
    pub approval_targets: Vec<String>,
    /// Relative paths of persisted artifacts for the run.
    pub artifact_paths: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Closure status label for backlog runs.
    pub closure_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Decomposition scope label for backlog runs.
    pub decomposition_scope: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    /// Closure findings for backlog runs.
    pub closure_findings: Vec<ClosureFindingInspectSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Human-readable closure notes for backlog runs.
    pub closure_notes: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// All possible next actions for the run.
    pub possible_actions: Vec<PossibleActionSummary>,
    /// Additive refinement state for same-run clarification workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refinement_state: Option<RefinementStateSummary>,
    /// The mode-specific result summary, if the run has completed.
    pub mode_result: Option<ModeResultSummary>,
    /// The recommended next action for the operator.
    pub recommended_next_action: Option<RecommendedActionSummary>,
}

/// A summary of a single gate evaluation included in status and inspect responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GateInspectSummary {
    /// The gate kind label.
    pub gate: String,
    /// The gate evaluation status label.
    pub status: String,
    /// Human-readable blockers preventing this gate from passing.
    pub blockers: Vec<String>,
}

/// The single recommended next action for an operator, with rationale and optional target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecommendedActionSummary {
    /// The action verb (e.g., `"approve"`, `"resume"`).
    pub action: String,
    /// Human-readable rationale for the recommendation.
    pub rationale: String,
    /// The optional target for the action (e.g., a gate label).
    pub target: Option<String>,
}

/// A possible next action surfaced in a status or inspect response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PossibleActionSummary {
    /// The action verb (e.g., `"approve"`, `"resume"`).
    pub action: String,
    /// Human-readable text describing the action.
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The optional target for the action.
    pub target: Option<String>,
}

/// Summary returned after recording an approval decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApprovalSummary {
    /// The run ID the approval was recorded against.
    pub run_id: String,
    /// The approval target (gate or invocation label).
    pub target: String,
    /// The identity of the reviewer who recorded the decision.
    pub approved_by: String,
    /// When the approval was recorded (ISO-8601).
    pub recorded_at: String,
    /// The approval decision (`"approve"` or `"reject"`).
    pub decision: String,
    /// The run state after the approval was applied.
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
    possible_actions: Vec<PossibleActionSummary>,
    refinement_state: Option<RefinementStateSummary>,
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

/// The core orchestrator service responsible for governed run lifecycle and policy evaluation.
#[derive(Debug, Clone)]
pub struct EngineService {
    repo_root: PathBuf,
}

impl EngineService {
    /// Creates a new `EngineService` anchored to a specific repository root.
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

    /// Initializes a repository for Canon usage, ensuring required governance structures exist.
    pub fn init(&self, ai_tool: Option<AiTool>) -> Result<InitSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.init_runtime_state(ai_tool.map(AiTool::materialization_target))?;
        Ok(Self::map_init_summary(summary))
    }

    /// Installs repository-local skills suitable for the specified AI tool.
    pub fn skills_install(&self, ai_tool: AiTool) -> Result<SkillsSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.install_skills(ai_tool.materialization_target())?;
        Ok(Self::map_skills_summary(summary))
    }

    /// Updates existing repository-local skills to the latest versions.
    pub fn skills_update(&self, ai_tool: AiTool) -> Result<SkillsSummary, EngineError> {
        let store = WorkspaceStore::new(&self.repo_root);
        let summary = store.update_skills(ai_tool.materialization_target())?;
        Ok(Self::map_skills_summary(summary))
    }

    /// Lists all discovered Canon skills and their support status.
    pub fn skills_list(&self) -> Vec<SkillEntry> {
        let store = WorkspaceStore::new(&self.repo_root);
        store
            .list_skills()
            .into_iter()
            .map(|entry| SkillEntry { name: entry.name, support_state: entry.support_state })
            .collect()
    }
}

/// Return the canonical delivery order for a packet's body artifacts.
///
/// Sidecar artifacts (e.g. `view-manifest.json`, `packet-metadata.json`) are excluded;
/// the returned slice contains only the files that form the readable body of the packet,
/// in the order defined by the mode's [`ArtifactContract`].
pub(super) fn packet_body_artifact_order(
    artifact_requirements: &[ArtifactRequirement],
) -> Vec<String> {
    artifact_requirements
        .iter()
        .filter(|requirement| !is_packet_sidecar(&requirement.file_name))
        .map(|requirement| requirement.file_name.clone())
        .collect()
}

/// Serialize a `packet-metadata.json` sidecar for the given run.
///
/// The returned JSON string encodes:
/// - `run_id` and `mode` for provenance;
/// - `primary_artifact`: the first non-sidecar artifact in the contract order;
/// - `artifact_order`: the full body-artifact sequence;
/// - `legacy_aliases` (omitted when empty): a map from bare slug to prefixed filename
///   enabling consumers to resolve unprefixed references.
///
/// Build the machine-facing packet metadata sidecar for runtime-authored modes.
pub(super) fn build_runtime_packet_metadata(
    run_id: &str,
    request: &RunRequest,
    approvals: &[ApprovalRecord],
    artifact_requirements: &[ArtifactRequirement],
) -> Result<String, EngineError> {
    let artifact_order = packet_body_artifact_order(artifact_requirements);
    let primary_artifact = artifact_order.first().cloned().unwrap_or_default();
    let legacy_aliases = artifact_requirements
        .iter()
        .filter_map(|requirement| {
            let slug = artifact_slug(&requirement.file_name);
            (slug != requirement.file_name)
                .then(|| (slug.to_string(), requirement.file_name.clone()))
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let authority_governance = Some(AuthorityGovernanceV1Envelope::from_runtime_inputs(
        AuthorityGovernanceV1RuntimeInputs {
            mode: request.mode,
            risk: request.risk,
            zone: request.zone,
            approval_state: authority_approval_state(approvals),
            packet_readiness: AuthorityPacketReadiness::Reusable,
            primary_artifact: (!primary_artifact.is_empty()).then_some(primary_artifact.clone()),
            artifact_order: artifact_order.clone(),
            promotion_refs: Vec::new(),
        },
    ));
    let adaptive_governance = Some(AdaptiveGovernanceV1Envelope::from_runtime_inputs(
        AdaptiveGovernanceV1RuntimeInputs {
            risk: request.risk,
            zone: request.zone,
            approval_state: authority_approval_state(approvals),
            packet_readiness: AuthorityPacketReadiness::Reusable,
        },
    ));

    #[derive(Serialize)]
    struct RuntimePacketMetadataEnvelope<'a> {
        run_id: &'a str,
        mode: &'a str,
        #[serde(flatten)]
        metadata: RuntimePacketMetadata,
    }

    let payload = RuntimePacketMetadataEnvelope {
        run_id,
        mode: request.mode.as_str(),
        metadata: RuntimePacketMetadata {
            primary_artifact,
            artifact_order,
            publish_order: None,
            legacy_aliases: (!legacy_aliases.is_empty()).then_some(legacy_aliases),
            expertise_input: None,
            publication_target_class: None,
            artifact_indexing: None,
            semantic_descriptor: None,
            authority_governance,
            adaptive_governance,
        },
    };

    serde_json::to_string_pretty(&payload).map_err(|error| {
        EngineError::Validation(format!("packet metadata serialization failed: {error}"))
    })
}

#[cfg(test)]
mod tests;
