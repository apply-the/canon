use canon_adapters::{
    AdapterKind, CapabilityKind, InvocationOrientation, LineageClass, MutabilityClass,
    TrustBoundaryKind,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::domain::policy::{RiskClass, UsageZone};
use crate::domain::run::SystemContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecisionKind {
    Allow,
    AllowConstrained,
    NeedsApproval,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PayloadRetentionLevel {
    SummaryOnly,
    SummaryWithDigest,
    SummaryWithRetainedPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolOutcomeKind {
    Succeeded,
    PartiallySucceeded,
    Failed,
    Denied,
    AwaitingApproval,
    RecommendationOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionPosture {
    Mutating,
    RecommendationOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutationExpansionPolicy {
    DenyWithoutApproval,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationBounds {
    pub declared_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_refs: Vec<String>,
    pub expansion_policy: MutationExpansionPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SafetyNetEvidenceProvenance {
    PreExisting,
    AuthoredInRun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SafetyNetEvidenceStatus {
    Satisfied,
    Missing,
    ExceptionApproved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafetyNetEvidence {
    pub target_surface: String,
    pub evidence_kind: String,
    pub provenance: SafetyNetEvidenceProvenance,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    pub status: SafetyNetEvidenceStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceDisposition {
    Supporting,
    Blocking,
    NeedsDisposition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAdapterDescriptor {
    pub adapter: AdapterKind,
    pub trust_boundary: TrustBoundaryKind,
    pub available: bool,
    pub supported_capabilities: Vec<CapabilityKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InvocationConstraintSet {
    pub allowed_paths: Vec<String>,
    pub command_profile: Option<String>,
    pub max_payload_bytes: Option<u64>,
    pub recommendation_only: bool,
    pub patch_disabled: bool,
    pub payload_retention: Option<PayloadRetentionLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationRequest {
    pub request_id: String,
    pub run_id: String,
    pub mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    pub risk: RiskClass,
    pub zone: UsageZone,
    pub adapter: AdapterKind,
    pub capability: CapabilityKind,
    pub orientation: InvocationOrientation,
    pub mutability: MutabilityClass,
    pub trust_boundary: TrustBoundaryKind,
    pub lineage: LineageClass,
    pub requested_scope: Vec<String>,
    pub owner: Option<String>,
    pub summary: String,
    pub requested_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationPolicyDecision {
    pub kind: PolicyDecisionKind,
    pub constraints: InvocationConstraintSet,
    pub requires_approval: bool,
    pub rationale: String,
    pub policy_refs: Vec<String>,
    pub decided_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadReference {
    pub path: String,
    pub digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolOutcome {
    pub kind: ToolOutcomeKind,
    pub summary: String,
    pub exit_code: Option<i32>,
    pub payload_refs: Vec<PayloadReference>,
    pub candidate_artifacts: Vec<String>,
    pub recorded_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationAttempt {
    pub request_id: String,
    pub attempt_number: u32,
    pub started_at: OffsetDateTime,
    pub finished_at: OffsetDateTime,
    pub executor: String,
    pub outcome: ToolOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedInvocation {
    pub request_id: String,
    pub rationale: String,
    pub policy_refs: Vec<String>,
    pub recorded_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationTrace {
    pub run_id: String,
    pub request_id: String,
    pub adapter: AdapterKind,
    pub capability: CapabilityKind,
    pub orientation: InvocationOrientation,
    pub policy_decision: PolicyDecisionKind,
    pub outcome: Option<ToolOutcomeKind>,
    pub linked_artifacts: Vec<String>,
    pub linked_decisions: Vec<String>,
    pub linked_approvals: Vec<String>,
    pub recorded_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIndependenceAssessment {
    pub target_id: String,
    pub sufficient: bool,
    pub rationale: String,
    pub supporting_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationPath {
    pub path_id: String,
    pub request_ids: Vec<String>,
    pub lineage_classes: Vec<LineageClass>,
    pub derived_artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationPath {
    pub path_id: String,
    pub request_ids: Vec<String>,
    pub lineage_classes: Vec<LineageClass>,
    pub verification_refs: Vec<String>,
    pub independence: ValidationIndependenceAssessment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub run_id: String,
    pub generation_paths: Vec<GenerationPath>,
    pub validation_paths: Vec<ValidationPath>,
    pub denied_invocations: Vec<DeniedInvocation>,
    pub trace_refs: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub decision_refs: Vec<String>,
    pub approval_refs: Vec<String>,
}
