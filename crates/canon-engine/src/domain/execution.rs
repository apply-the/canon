use canon_adapters::{
    AdapterKind, CapabilityKind, InvocationOrientation, LineageClass, MutabilityClass,
    TrustBoundaryKind,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::SystemContext;

/// The outcome of applying governance policy to an invocation request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecisionKind {
    /// The invocation is permitted without constraints.
    Allow,
    /// The invocation is permitted but subject to explicit constraints.
    AllowConstrained,
    /// The invocation requires a human approval record before proceeding.
    NeedsApproval,
    /// The invocation is not permitted by current policy.
    Deny,
}

/// Controls how much of an invocation payload is retained in the run record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayloadRetentionLevel {
    /// Only a human-readable summary is retained.
    SummaryOnly,
    /// A summary plus a content digest is retained.
    SummaryWithDigest,
    /// The full payload is retained alongside the summary.
    SummaryWithRetainedPayload,
}

/// The outcome classification of a completed tool invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolOutcomeKind {
    /// The invocation completed successfully.
    Succeeded,
    /// The invocation completed but produced partial results.
    PartiallySucceeded,
    /// The invocation failed.
    Failed,
    /// The invocation was blocked by policy and not executed.
    Denied,
    /// The invocation is waiting for a human approval before it can run.
    AwaitingApproval,
    /// The invocation produced a recommendation but did not mutate state.
    RecommendationOnly,
}

/// Whether a run is executing mutations or generating recommendations only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionPosture {
    /// The run is permitted to make real mutations to the workspace.
    Mutating,
    /// The run produces recommendations only; no mutations are applied.
    RecommendationOnly,
}

/// Policy applied when an invocation requests paths outside its declared mutation bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutationExpansionPolicy {
    /// Mutation outside declared bounds is blocked unless an explicit approval is recorded.
    DenyWithoutApproval,
}

/// Declares the paths and expansion policy for a bounded mutation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationBounds {
    /// The set of paths the run is explicitly permitted to mutate.
    pub declared_paths: Vec<String>,
    /// Identifiers of the humans who own the declared scope.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<String>,
    /// Source references (e.g. task IDs, spec refs) that authorize this scope.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_refs: Vec<String>,
    /// What happens when the run tries to mutate paths outside `declared_paths`.
    pub expansion_policy: MutationExpansionPolicy,
}

/// Whether a safety-net evidence item was authored before the run or during it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SafetyNetEvidenceProvenance {
    /// The evidence existed before the run began.
    PreExisting,
    /// The evidence was produced during this run.
    AuthoredInRun,
}

/// Whether a safety-net evidence requirement is satisfied or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SafetyNetEvidenceStatus {
    /// The safety-net requirement is satisfied by available evidence.
    Satisfied,
    /// Required evidence is missing and no exception has been approved.
    Missing,
    /// Required evidence is missing but an explicit exception approval was recorded.
    ExceptionApproved,
}

/// A single piece of safety-net evidence required for a bounded mutation run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafetyNetEvidence {
    /// The surface (file or directory) this evidence covers.
    pub target_surface: String,
    /// The kind of evidence (e.g. `"test"`, `"contract"`, `"snapshot"`).
    pub evidence_kind: String,
    /// Whether the evidence existed before the run or was authored during it.
    pub provenance: SafetyNetEvidenceProvenance,
    /// Paths or identifiers pointing to the supporting evidence artifacts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// Whether the requirement is currently satisfied.
    pub status: SafetyNetEvidenceStatus,
}

/// Whether an evidence item supports, blocks, or requires explicit disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceDisposition {
    /// The evidence is consistent with the claimed outcome.
    Supporting,
    /// The evidence contradicts or undermines the claimed outcome.
    Blocking,
    /// The evidence status is unclear and requires an explicit human disposition.
    NeedsDisposition,
}

/// Describes a concrete adapter and its trust and capability surface for a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAdapterDescriptor {
    /// The adapter being described.
    pub adapter: AdapterKind,
    /// The trust boundary this adapter operates within.
    pub trust_boundary: TrustBoundaryKind,
    /// Whether this adapter is available in the current runtime environment.
    pub available: bool,
    /// The capability kinds this adapter is configured to provide.
    pub supported_capabilities: Vec<CapabilityKind>,
}

/// The set of runtime constraints applied to a policy-approved invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InvocationConstraintSet {
    /// Paths the invocation is permitted to read or mutate.
    pub allowed_paths: Vec<String>,
    /// Optional identifier for the constraint profile that was applied.
    pub command_profile: Option<String>,
    /// Maximum payload bytes this invocation may produce or consume.
    pub max_payload_bytes: Option<u64>,
    /// Whether the invocation must produce recommendations only.
    pub recommendation_only: bool,
    /// Whether workspace patch application is disabled for this invocation.
    pub patch_disabled: bool,
    /// The retention level applied to this invocation's payload.
    pub payload_retention: Option<PayloadRetentionLevel>,
}

/// A single governance-gated invocation request produced during run execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationRequest {
    /// Unique identifier for this request (UUIDv7).
    pub request_id: String,
    /// The run this request belongs to.
    pub run_id: String,
    /// The mode the run is executing under.
    pub mode: String,
    /// Whether the run targets a new or existing system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    /// Risk class of the run at the time of this request.
    pub risk: RiskClass,
    /// Usage zone of the run at the time of this request.
    pub zone: UsageZone,
    /// The adapter this request targets.
    pub adapter: AdapterKind,
    /// The specific capability being requested.
    pub capability: CapabilityKind,
    /// The orientation of this invocation (context, generation, or validation).
    pub orientation: InvocationOrientation,
    /// The mutability class of the requested capability.
    pub mutability: MutabilityClass,
    /// The trust boundary the adapter operates within.
    pub trust_boundary: TrustBoundaryKind,
    /// The lineage class of the generated content, if any.
    pub lineage: LineageClass,
    /// The paths or scopes being accessed or mutated.
    pub requested_scope: Vec<String>,
    /// The human or agent owner of this request, if named.
    pub owner: Option<String>,
    /// One-line human-readable summary of the invocation purpose.
    pub summary: String,
    /// When this request was created.
    pub requested_at: OffsetDateTime,
}

/// The outcome of applying governance policy to a specific invocation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationPolicyDecision {
    /// The policy decision kind (allow, constrained, needs-approval, deny).
    pub kind: PolicyDecisionKind,
    /// The constraint set applied if the decision is Allow or AllowConstrained.
    pub constraints: InvocationConstraintSet,
    /// Whether a human approval record is required before execution.
    pub requires_approval: bool,
    /// Human-readable explanation of the policy decision.
    pub rationale: String,
    /// Identifiers of the policy rules that produced this decision.
    pub policy_refs: Vec<String>,
    /// When this decision was computed.
    pub decided_at: OffsetDateTime,
}

/// A reference to an artifact payload produced or consumed during an invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadReference {
    /// Path to the referenced artifact or payload file.
    pub path: String,
    /// Optional content digest used for integrity verification.
    pub digest: Option<String>,
}

/// The result of executing a governance-gated tool invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolOutcome {
    /// Outcome classification of the completed invocation.
    pub kind: ToolOutcomeKind,
    /// Human-readable summary of what the tool produced or why it failed.
    pub summary: String,
    /// Process exit code, if the invocation spawned a subprocess.
    pub exit_code: Option<i32>,
    /// References to any payload files produced by the tool.
    pub payload_refs: Vec<PayloadReference>,
    /// Candidate artifact paths the orchestrator may promote.
    pub candidate_artifacts: Vec<String>,
    /// When the outcome was recorded.
    pub recorded_at: OffsetDateTime,
}

/// A single execution attempt for a specific invocation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationAttempt {
    /// The request ID this attempt is for.
    pub request_id: String,
    /// 1-based attempt count (1 = first try).
    pub attempt_number: u32,
    /// When the attempt started.
    pub started_at: OffsetDateTime,
    /// When the attempt finished.
    pub finished_at: OffsetDateTime,
    /// Identifier of the executor (adapter + runtime instance).
    pub executor: String,
    /// The outcome produced by this attempt.
    pub outcome: ToolOutcome,
}

/// Records an invocation that was blocked by policy and never executed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedInvocation {
    /// The request ID of the denied invocation.
    pub request_id: String,
    /// Human-readable explanation of why the invocation was denied.
    pub rationale: String,
    /// The policy rule references that produced the denial.
    pub policy_refs: Vec<String>,
    /// When this denial was recorded.
    pub recorded_at: OffsetDateTime,
}

/// A compact audit record of a single invocation's lifecycle within a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationTrace {
    /// The run this trace belongs to.
    pub run_id: String,
    /// The request ID that this trace covers.
    pub request_id: String,
    /// The adapter that was invoked.
    pub adapter: AdapterKind,
    /// The capability that was exercised.
    pub capability: CapabilityKind,
    /// The orientation of the invocation.
    pub orientation: InvocationOrientation,
    /// The policy decision that governed the invocation.
    pub policy_decision: PolicyDecisionKind,
    /// The outcome kind, if the invocation completed.
    pub outcome: Option<ToolOutcomeKind>,
    /// Paths of artifacts that this invocation produced or consumed.
    pub linked_artifacts: Vec<String>,
    /// Decision record IDs linked to this invocation.
    pub linked_decisions: Vec<String>,
    /// Approval record IDs linked to this invocation.
    pub linked_approvals: Vec<String>,
    /// When this trace was recorded.
    pub recorded_at: OffsetDateTime,
}

/// Assessment of whether a validation path is independent from the generation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIndependenceAssessment {
    /// The validation path ID being assessed.
    pub target_id: String,
    /// Whether the validation is considered sufficiently independent.
    pub sufficient: bool,
    /// Human-readable explanation of the independence assessment.
    pub rationale: String,
    /// Supporting evidence or reference IDs.
    pub supporting_refs: Vec<String>,
}

/// A group of request IDs that share a common AI generation lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationPath {
    /// Identifier for this generation path.
    pub path_id: String,
    /// Ordered request IDs that contributed to this generation path.
    pub request_ids: Vec<String>,
    /// Lineage classes represented in this path.
    pub lineage_classes: Vec<LineageClass>,
    /// Artifact files that were produced by this generation path.
    pub derived_artifacts: Vec<String>,
}

/// A group of request IDs that collectively validate a generation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationPath {
    /// Identifier for this validation path.
    pub path_id: String,
    /// Request IDs that contributed to this validation path.
    pub request_ids: Vec<String>,
    /// Lineage classes represented in this path.
    pub lineage_classes: Vec<LineageClass>,
    /// References to verification records linked to this path.
    pub verification_refs: Vec<String>,
    /// Independence assessment for this path relative to its generation path.
    pub independence: ValidationIndependenceAssessment,
}

/// The complete evidence bundle for a Canon run.
///
/// Contains generation paths, validation paths, denied invocations, and
/// cross-references to all artifact, decision, and approval records produced
/// during the run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    /// The run this bundle belongs to.
    pub run_id: String,
    /// All generation paths recorded during the run.
    pub generation_paths: Vec<GenerationPath>,
    /// All validation paths recorded during the run.
    pub validation_paths: Vec<ValidationPath>,
    /// Invocations that were blocked by policy and never executed.
    pub denied_invocations: Vec<DeniedInvocation>,
    /// References to invocation trace records.
    pub trace_refs: Vec<String>,
    /// References to artifact files produced during the run.
    pub artifact_refs: Vec<String>,
    /// References to decision records.
    pub decision_refs: Vec<String>,
    /// References to approval records.
    pub approval_refs: Vec<String>,
}
