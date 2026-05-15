use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::artifact::ArtifactContract;
use crate::domain::execution::{ExecutionPosture, MutationBounds, SafetyNetEvidence};
use crate::domain::mode::Mode;
use crate::domain::policy::{RiskClass, UsageZone};

/// Canonical identity for a Canon run.
///
/// `uuid` is the immutable machine identity (UUIDv7 today).
/// `run_id` is the human-facing display id `R-YYYYMMDD-SHORTID` derived
/// deterministically from `uuid` and the UTC date of `created_at`.
/// `short_id` is the first 8 hex characters of the lowercase canonical
/// UUID string. See `specs/009-run-id-display/contracts/run-identity-contract.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunIdentity {
    /// The UUID (v7) assigned to this run.
    pub uuid: Uuid,
    /// The human-readable run ID derived from the UUID.
    pub run_id: String,
    /// The first 8 hex characters of the canonical UUID string.
    pub short_id: String,
    /// The creation timestamp for this run.
    pub created_at: OffsetDateTime,
}

impl RunIdentity {
    /// Generate a fresh identity using `Uuid::now_v7()` and `OffsetDateTime::now_utc()`.
    pub fn new_now_v7() -> Self {
        Self::from_parts(Uuid::now_v7(), OffsetDateTime::now_utc())
    }

    /// Build a [`RunIdentity`] from a known UUID and timestamp.
    /// Used by the manifest read shim to reconstruct identity for legacy runs.
    pub fn from_parts(uuid: Uuid, created_at: OffsetDateTime) -> Self {
        let short_id = short_id_from_uuid(&uuid);
        let date = created_at.to_offset(time::UtcOffset::UTC).date();
        let run_id = format!(
            "R-{:04}{:02}{:02}-{}",
            date.year(),
            u8::from(date.month()),
            date.day(),
            short_id
        );
        Self { uuid, run_id, short_id, created_at }
    }
}

/// First 8 hex characters of the canonical lowercase UUID string.
pub fn short_id_from_uuid(uuid: &Uuid) -> String {
    let buf = uuid.as_simple().to_string();
    buf[..8].to_string()
}

/// Validate the canonical display-id shape `^R-\d{8}-[0-9a-f]{8}$`.
pub fn is_canonical_display_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 19 {
        return false;
    }
    if &bytes[..2] != b"R-" || bytes[10] != b'-' {
        return false;
    }
    bytes[2..10].iter().all(|b| b.is_ascii_digit())
        && bytes[11..].iter().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

// Mode names only describe the governed work type; system state stays explicit.
/// Represents whether a governance run targets a new or an existing system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemContext {
    /// Work on a system that does not exist or is being created from scratch.
    New,
    /// Work on a system with existing code, invariants, and users.
    Existing,
}

impl SystemContext {
    /// Returns the kebab-case string representation of this context.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Existing => "existing",
        }
    }
}

impl std::str::FromStr for SystemContext {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "new" => Ok(Self::New),
            "existing" => Ok(Self::Existing),
            other => Err(format!("unsupported system context: {other}")),
        }
    }
}

/// Lifecycle state of a Canon run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunState {
    /// The run manifest has been created but context has not been captured.
    Draft,
    /// Context has been captured and stored on the run.
    ContextCaptured,
    /// The run has been risk- and zone-classified.
    Classified,
    /// The artifact contract has been declared and accepted.
    Contracted,
    /// All mandatory pre-execution gates have been evaluated.
    Gated,
    /// The run is actively executing invocations.
    Executing,
    /// The run is waiting for a human approval before continuing.
    AwaitingApproval,
    /// The run is executing verification passes.
    Verifying,
    /// The run completed successfully.
    Completed,
    /// A gate or policy check is blocking the run from proceeding.
    Blocked,
    /// The run encountered a non-recoverable error.
    Failed,
    /// The run was explicitly stopped before completion.
    Aborted,
    /// The run has been replaced by a newer run.
    Superseded,
}

/// The captured runtime context for a Canon run.
///
/// Recorded once when the run transitions to `ContextCaptured` and immutable
/// thereafter. The gatekeeper and orchestrator read this to make all
/// subsequent decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContext {
    /// Absolute path to the repository root at the time of capture.
    pub repo_root: String,
    /// Named owner of this run, required for `SystemicImpact` risk class.
    pub owner: Option<String>,
    /// Paths to the input documents supplied at run start.
    pub inputs: Vec<String>,
    /// Paths explicitly excluded from context scanning.
    pub excluded_paths: Vec<String>,
    /// Content fingerprints of the input documents.
    pub input_fingerprints: Vec<InputFingerprint>,
    /// Whether the run targets a new or existing system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    /// Upstream run or artifact context, if this run continues prior work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_context: Option<UpstreamContext>,
    /// Implementation execution parameters, if this is an `Implementation` run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation_execution: Option<ImplementationExecutionContext>,
    /// Refactor execution parameters, if this is a `Refactor` run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refactor_execution: Option<RefactorExecutionContext>,
    /// Backlog planning parameters, if this is a `Backlog` run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backlog_planning: Option<BacklogPlanningContext>,
    /// In-memory inline input payloads; not persisted to TOML.
    #[serde(default, skip)]
    pub inline_inputs: Vec<InlineInput>,
    /// When the context was captured.
    pub captured_at: OffsetDateTime,
}

/// References to upstream runs or artifacts that this run continues or depends on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamContext {
    /// The specific feature slice or scope from the upstream run, if named.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_slice: Option<String>,
    /// The primary upstream mode (e.g. `"architecture"`, `"backlog"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_upstream_mode: Option<String>,
    /// Run IDs or artifact paths from the upstream run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_refs: Vec<String>,
    /// Items explicitly carried forward from the upstream run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub carried_forward_items: Vec<String>,
    /// Scope from the upstream run that this run explicitly excludes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded_upstream_scope: Option<String>,
}

/// Execution parameters for a bounded `Implementation` run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplementationExecutionContext {
    /// Source documents (plan files, task lists) that authorise the implementation scope.
    pub plan_sources: Vec<String>,
    /// The declared mutation bounds for this run.
    pub mutation_bounds: MutationBounds,
    /// Specific tasks or work items targeted by this run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub task_targets: Vec<String>,
    /// Safety-net evidence required before mutations are applied.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety_net: Vec<SafetyNetEvidence>,
    /// Whether the run is executing mutations or producing recommendations.
    pub execution_posture: ExecutionPosture,
    /// What the run is expected to do if it needs to be rolled back.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rollback_expectations: Vec<String>,
    /// Whether the post-approval execution token has been consumed.
    #[serde(default, skip_serializing_if = "is_false")]
    pub post_approval_execution_consumed: bool,
}

/// Execution parameters for a bounded `Refactor` run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorExecutionContext {
    /// Descriptions of the observable behaviors that must not change.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preserved_behavior: Vec<String>,
    /// Explanation of the structural change being made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structural_rationale: Option<String>,
    /// The mutation bounds declared for this refactor.
    pub refactor_scope: MutationBounds,
    /// Safety-net evidence required before mutations are applied.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety_net: Vec<SafetyNetEvidence>,
    /// Evidence or assertion that no new features were added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_feature_addition_target: Option<String>,
    /// Explicit exceptions to the no-feature-addition invariant.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_exceptions: Vec<String>,
    /// Whether the run is executing mutations or producing recommendations.
    pub execution_posture: ExecutionPosture,
    /// Whether the post-approval execution token has been consumed.
    #[serde(default, skip_serializing_if = "is_false")]
    pub post_approval_execution_consumed: bool,
}

/// Controls the decomposition granularity for a `Backlog` run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BacklogGranularity {
    /// Produce epics only.
    EpicOnly,
    /// Produce epics with delivery slices.
    EpicPlusSlice,
    /// Produce epics, delivery slices, and story candidates.
    EpicPlusSlicePlusStoryCandidate,
}

impl BacklogGranularity {
    /// Returns the kebab-case string representation of this granularity level.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EpicOnly => "epic-only",
            Self::EpicPlusSlice => "epic-plus-slice",
            Self::EpicPlusSlicePlusStoryCandidate => "epic-plus-slice-plus-story-candidate",
        }
    }

    /// Parses a kebab-case label into a `BacklogGranularity`, returning `None` for unknown labels.
    pub fn from_label(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "epic-only" => Some(Self::EpicOnly),
            "epic-plus-slice" => Some(Self::EpicPlusSlice),
            "epic-plus-slice-plus-story-candidate" => Some(Self::EpicPlusSlicePlusStoryCandidate),
            _ => None,
        }
    }
}

/// The overall closure status of a run: whether evidence is sufficient to close.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClosureStatus {
    /// Evidence is sufficient; the run can close.
    Sufficient,
    /// Evidence is weaker than expected but the run is still closable.
    Downgraded,
    /// Evidence is insufficient; the run cannot close yet.
    Blocked,
}

impl ClosureStatus {
    /// Returns the kebab-case string representation of this closure status.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sufficient => "sufficient",
            Self::Downgraded => "downgraded",
            Self::Blocked => "blocked",
        }
    }
}

/// Whether a closure assessment covers the full packet or only the risk surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClosureDecompositionScope {
    /// The assessment covers the complete artifact packet.
    FullPacket,
    /// The assessment covers only the risk-bearing surfaces.
    RiskOnlyPacket,
}

impl ClosureDecompositionScope {
    /// Returns the kebab-case string representation of this decomposition scope.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FullPacket => "full-packet",
            Self::RiskOnlyPacket => "risk-only-packet",
        }
    }
}

/// Urgency level of a closure finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClosureFindingSeverity {
    /// Informational; the run can still close.
    Warning,
    /// The run cannot close until this finding is resolved.
    Blocking,
}

impl ClosureFindingSeverity {
    /// Returns the kebab-case string representation of this severity.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }
}

/// A single finding produced during closure assessment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClosureFinding {
    /// Category label for the finding (e.g. `"missing-evidence"`).
    pub category: String,
    /// Whether this finding blocks closure or is informational.
    pub severity: ClosureFindingSeverity,
    /// The artifact or surface scope affected by this finding.
    pub affected_scope: String,
    /// Recommended action to resolve this finding.
    pub recommended_followup: String,
}

/// The result of evaluating whether a run has sufficient evidence to close.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClosureAssessment {
    /// The overall closure status.
    pub status: ClosureStatus,
    /// Individual findings that affect the status.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<ClosureFinding>,
    /// Whether the assessment was done on the full packet or only the risk surface.
    pub decomposition_scope: ClosureDecompositionScope,
    /// Optional human-readable notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ClosureAssessment {
    /// Returns a sufficient closure assessment with no findings.
    pub fn sufficient() -> Self {
        Self {
            status: ClosureStatus::Sufficient,
            findings: Vec::new(),
            decomposition_scope: ClosureDecompositionScope::FullPacket,
            notes: None,
        }
    }
}

/// Declares the granularity at which backlog items are decomposed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacklogPlanningContext {
    /// The governed mode (always `"backlog"` for this context).
    pub mode: String,
    /// Human-readable description of the delivery intent.
    pub delivery_intent: String,
    /// How granular the decomposition should be.
    pub desired_granularity: BacklogGranularity,
    /// The planning horizon (e.g. `"next sprint"`, `"next quarter"`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planning_horizon: Option<String>,
    /// Source references from upstream runs or documents.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_refs: Vec<String>,
    /// Priority signals that should influence decomposition ordering.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priority_inputs: Vec<String>,
    /// Delivery constraints that bound the scope of the backlog.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<String>,
    /// Items explicitly excluded from the backlog scope.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub out_of_scope: Vec<String>,
    /// Closure assessment for the backlog packet.
    pub closure_assessment: ClosureAssessment,
}

fn is_false(value: &bool) -> bool {
    !*value
}

/// An inline input payload provided directly at run start, not from a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineInput {
    /// Synthetic label used as a path reference in the run manifest.
    pub label: String,
    /// The raw text content of the inline input.
    pub contents: String,
}

/// Whether a run input came from a file path or was supplied inline.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputSourceKind {
    /// The input was loaded from a filesystem path.
    #[default]
    Path,
    /// The input was supplied as inline text.
    Inline,
}

/// A content fingerprint for a single run input, used for change detection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputFingerprint {
    /// The path or synthetic label of the input.
    pub path: String,
    /// Whether the input came from a file or inline text.
    #[serde(default)]
    pub source_kind: InputSourceKind,
    /// File size in bytes at the time of capture.
    pub size_bytes: u64,
    /// File modification time (Unix seconds) at the time of capture.
    pub modified_unix_seconds: i64,
    /// SHA-256 digest of the file contents, if computed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_digest_sha256: Option<String>,
    /// Reference to a content snapshot, if one was taken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_ref: Option<String>,
}

/// How a risk or zone classification was established for a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClassificationSource {
    /// The value was supplied explicitly by the operator.
    Explicit,
    /// The value was inferred and then confirmed by the operator.
    InferredConfirmed,
    /// The value was inferred but then overridden by the operator.
    InferredOverridden,
}

impl ClassificationSource {
    /// Returns the kebab-case string representation of this source.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Explicit => "explicit",
            Self::InferredConfirmed => "inferred-confirmed",
            Self::InferredOverridden => "inferred-overridden",
        }
    }
}

impl std::str::FromStr for ClassificationSource {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "explicit" => Ok(Self::Explicit),
            "inferred-confirmed" => Ok(Self::InferredConfirmed),
            "inferred-overridden" => Ok(Self::InferredOverridden),
            other => Err(format!("unsupported classification source: {other}")),
        }
    }
}

/// Provenance record for a single classification field (risk or zone).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationFieldProvenance {
    /// How the classification was established.
    pub source: ClassificationSource,
    /// Human-readable explanation of the classification decision.
    pub rationale: String,
    /// Input signals that influenced the classification.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signals: Vec<String>,
}

impl ClassificationFieldProvenance {
    /// Constructs a provenance record from source, rationale, and signals.
    pub fn new(
        source: ClassificationSource,
        rationale: impl Into<String>,
        signals: Vec<String>,
    ) -> Self {
        Self { source, rationale: rationale.into(), signals }
    }

    /// Returns a provenance record for an explicitly-supplied risk class.
    pub fn explicit_risk() -> Self {
        Self::new(
            ClassificationSource::Explicit,
            "Risk class was supplied explicitly at run start.",
            Vec::new(),
        )
    }

    /// Returns a provenance record for an explicitly-supplied usage zone.
    pub fn explicit_zone() -> Self {
        Self::new(
            ClassificationSource::Explicit,
            "Usage zone was supplied explicitly at run start.",
            Vec::new(),
        )
    }
}

/// Tracks how governance classifications (Risk, Zone) were derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationProvenance {
    /// Provenance for the risk rating.
    pub risk: ClassificationFieldProvenance,
    /// Provenance for the usage zone.
    pub zone: ClassificationFieldProvenance,
}

impl ClassificationProvenance {
    /// Returns a provenance record for a fully explicit (no inference) classification.
    pub fn explicit() -> Self {
        Self {
            risk: ClassificationFieldProvenance::explicit_risk(),
            zone: ClassificationFieldProvenance::explicit_zone(),
        }
    }
}

impl Default for ClassificationProvenance {
    fn default() -> Self {
        Self::explicit()
    }
}

/// The persistent state record for a single Canon run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Run {
    /// The human-readable run ID (e.g. `R-20240513-abcd1234`).
    pub id: String,
    /// The governed mode of the run.
    pub mode: Option<Mode>,
    /// The risk class assigned to the run.
    pub risk: Option<RiskClass>,
    /// The usage zone assigned to the run.
    pub zone: Option<UsageZone>,
    /// Whether the run targets a new or existing system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    /// The current lifecycle state of the run.
    pub state: RunState,
    /// When the run was created.
    pub created_at: OffsetDateTime,
    /// The artifact contract declared for this run.
    pub artifact_contract: Option<ArtifactContract>,
    /// Reference to the evidence bundle file for this run.
    pub evidence_bundle_ref: Option<String>,
    /// Invocation request IDs currently waiting for human approval.
    pub pending_invocation_ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use super::{
        BacklogGranularity, BacklogPlanningContext, ClosureAssessment, ClosureDecompositionScope,
        ClosureFinding, ClosureFindingSeverity, ClosureStatus, ImplementationExecutionContext,
        RefactorExecutionContext, RunContext, UpstreamContext,
    };
    use crate::domain::execution::{
        ExecutionPosture, MutationBounds, MutationExpansionPolicy, SafetyNetEvidence,
        SafetyNetEvidenceProvenance, SafetyNetEvidenceStatus,
    };
    use crate::domain::run::{InputFingerprint, InputSourceKind, SystemContext};

    #[test]
    fn run_context_deserializes_without_mode_specific_execution_blocks() {
        let context_toml = toml::to_string_pretty(&RunContext {
            repo_root: "/repo".to_string(),
            owner: None,
            inputs: vec!["canon-input/change.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: vec![InputFingerprint {
                path: "canon-input/change.md".to_string(),
                source_kind: InputSourceKind::Path,
                size_bytes: 42,
                modified_unix_seconds: 0,
                content_digest_sha256: None,
                snapshot_ref: None,
            }],
            system_context: None,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        })
        .expect("serialize minimal run context");

        assert!(!context_toml.contains("implementation_execution"));
        assert!(!context_toml.contains("refactor_execution"));
        assert!(!context_toml.contains("backlog_planning"));

        let context: RunContext = toml::from_str(&context_toml).expect("context toml");

        assert_eq!(context.system_context, None);
        assert!(context.upstream_context.is_none());
        assert!(context.implementation_execution.is_none());
        assert!(context.refactor_execution.is_none());
        assert!(context.backlog_planning.is_none());
    }

    #[test]
    fn run_context_serializes_mode_specific_execution_blocks_when_present() {
        let context = RunContext {
            repo_root: "/repo".to_string(),
            owner: Some("staff-engineer".to_string()),
            inputs: vec!["canon-input/implementation.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: vec![InputFingerprint {
                path: "canon-input/implementation.md".to_string(),
                source_kind: InputSourceKind::Path,
                size_bytes: 42,
                modified_unix_seconds: 0,
                content_digest_sha256: None,
                snapshot_ref: None,
            }],
            system_context: Some(SystemContext::Existing),
            upstream_context: Some(UpstreamContext {
                feature_slice: Some("auth session revocation".to_string()),
                primary_upstream_mode: Some("change".to_string()),
                source_refs: vec![
                    "docs/changes/R-20260422-AUTHREVOC/change-surface.md".to_string(),
                ],
                carried_forward_items: vec![
                    "Revocation output formatting stays stable.".to_string(),
                ],
                excluded_upstream_scope: Some("login UI flow".to_string()),
            }),
            implementation_execution: Some(ImplementationExecutionContext {
                plan_sources: vec!["canon-input/implementation.md".to_string()],
                mutation_bounds: MutationBounds {
                    declared_paths: vec!["src/auth".to_string()],
                    owners: vec!["staff-engineer".to_string()],
                    source_refs: vec!["canon-input/implementation.md".to_string()],
                    expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                },
                task_targets: vec!["auth-storage".to_string()],
                safety_net: vec![SafetyNetEvidence {
                    target_surface: "src/auth".to_string(),
                    evidence_kind: "existing-test".to_string(),
                    provenance: SafetyNetEvidenceProvenance::PreExisting,
                    evidence_refs: vec!["tests/auth".to_string()],
                    status: SafetyNetEvidenceStatus::Satisfied,
                }],
                execution_posture: ExecutionPosture::RecommendationOnly,
                rollback_expectations: vec!["rollback on auth regression".to_string()],
                post_approval_execution_consumed: false,
            }),
            refactor_execution: Some(RefactorExecutionContext {
                preserved_behavior: vec!["public API remains stable".to_string()],
                structural_rationale: Some("untangle service boundaries".to_string()),
                refactor_scope: MutationBounds {
                    declared_paths: vec!["src/reviewer".to_string()],
                    owners: vec!["staff-engineer".to_string()],
                    source_refs: vec!["canon-input/refactor.md".to_string()],
                    expansion_policy: MutationExpansionPolicy::DenyWithoutApproval,
                },
                safety_net: Vec::new(),
                no_feature_addition_target: Some("no new CLI surface".to_string()),
                allowed_exceptions: Vec::new(),
                execution_posture: ExecutionPosture::RecommendationOnly,
                post_approval_execution_consumed: false,
            }),
            backlog_planning: Some(BacklogPlanningContext {
                mode: "backlog".to_string(),
                delivery_intent: "Prepare a bounded roadmap for auth-session hardening.".to_string(),
                desired_granularity: BacklogGranularity::EpicPlusSlice,
                planning_horizon: Some("next two releases".to_string()),
                source_refs: vec!["docs/changes/R-20260422-AUTHREVOC/implementation-plan.md".to_string()],
                priority_inputs: vec!["Reduce auth-session rollback risk first.".to_string()],
                constraints: vec!["Keep the packet above task-level planning.".to_string()],
                out_of_scope: vec!["Login UI redesign".to_string()],
                closure_assessment: ClosureAssessment {
                    status: ClosureStatus::Downgraded,
                    findings: vec![ClosureFinding {
                        category: "missing-exclusion".to_string(),
                        severity: ClosureFindingSeverity::Warning,
                        affected_scope: "whole-run".to_string(),
                        recommended_followup: "Strengthen the explicit exclusions before downstream implementation planning.".to_string(),
                    }],
                    decomposition_scope: ClosureDecompositionScope::RiskOnlyPacket,
                    notes: Some("The backlog packet stayed closure-limited in this sample.".to_string()),
                },
            }),
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        };

        let serialized = toml::to_string_pretty(&context).expect("serialize run context");

        assert!(serialized.contains("[implementation_execution]"));
        assert!(serialized.contains("[implementation_execution.mutation_bounds]"));
        assert!(serialized.contains("execution_posture = \"recommendation-only\""));
        assert!(serialized.contains("[upstream_context]"));
        assert!(serialized.contains("primary_upstream_mode = \"change\""));
        assert!(serialized.contains("[refactor_execution]"));
        assert!(serialized.contains("[refactor_execution.refactor_scope]"));
        assert!(serialized.contains("[backlog_planning]"));
        assert!(serialized.contains("desired_granularity = \"epic-plus-slice\""));
    }
}
