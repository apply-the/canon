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
    pub uuid: Uuid,
    pub run_id: String,
    pub short_id: String,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SystemContext {
    New,
    Existing,
}

impl SystemContext {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunState {
    Draft,
    ContextCaptured,
    Classified,
    Contracted,
    Gated,
    Executing,
    AwaitingApproval,
    Verifying,
    Completed,
    Blocked,
    Failed,
    Aborted,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContext {
    pub repo_root: String,
    pub owner: Option<String>,
    pub inputs: Vec<String>,
    pub excluded_paths: Vec<String>,
    pub input_fingerprints: Vec<InputFingerprint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_context: Option<UpstreamContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation_execution: Option<ImplementationExecutionContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refactor_execution: Option<RefactorExecutionContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backlog_planning: Option<BacklogPlanningContext>,
    #[serde(default, skip)]
    pub inline_inputs: Vec<InlineInput>,
    pub captured_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_slice: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_upstream_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub carried_forward_items: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded_upstream_scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplementationExecutionContext {
    pub plan_sources: Vec<String>,
    pub mutation_bounds: MutationBounds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub task_targets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety_net: Vec<SafetyNetEvidence>,
    pub execution_posture: ExecutionPosture,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rollback_expectations: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub post_approval_execution_consumed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorExecutionContext {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preserved_behavior: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structural_rationale: Option<String>,
    pub refactor_scope: MutationBounds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety_net: Vec<SafetyNetEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_feature_addition_target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_exceptions: Vec<String>,
    pub execution_posture: ExecutionPosture,
    #[serde(default, skip_serializing_if = "is_false")]
    pub post_approval_execution_consumed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BacklogGranularity {
    EpicOnly,
    EpicPlusSlice,
    EpicPlusSlicePlusStoryCandidate,
}

impl BacklogGranularity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EpicOnly => "epic-only",
            Self::EpicPlusSlice => "epic-plus-slice",
            Self::EpicPlusSlicePlusStoryCandidate => "epic-plus-slice-plus-story-candidate",
        }
    }

    pub fn from_label(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "epic-only" => Some(Self::EpicOnly),
            "epic-plus-slice" => Some(Self::EpicPlusSlice),
            "epic-plus-slice-plus-story-candidate" => Some(Self::EpicPlusSlicePlusStoryCandidate),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClosureStatus {
    Sufficient,
    Downgraded,
    Blocked,
}

impl ClosureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sufficient => "sufficient",
            Self::Downgraded => "downgraded",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClosureDecompositionScope {
    FullPacket,
    RiskOnlyPacket,
}

impl ClosureDecompositionScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FullPacket => "full-packet",
            Self::RiskOnlyPacket => "risk-only-packet",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClosureFindingSeverity {
    Warning,
    Blocking,
}

impl ClosureFindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Blocking => "blocking",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClosureFinding {
    pub category: String,
    pub severity: ClosureFindingSeverity,
    pub affected_scope: String,
    pub recommended_followup: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClosureAssessment {
    pub status: ClosureStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<ClosureFinding>,
    pub decomposition_scope: ClosureDecompositionScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ClosureAssessment {
    pub fn sufficient() -> Self {
        Self {
            status: ClosureStatus::Sufficient,
            findings: Vec::new(),
            decomposition_scope: ClosureDecompositionScope::FullPacket,
            notes: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BacklogPlanningContext {
    pub mode: String,
    pub delivery_intent: String,
    pub desired_granularity: BacklogGranularity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planning_horizon: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priority_inputs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub out_of_scope: Vec<String>,
    pub closure_assessment: ClosureAssessment,
}

fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineInput {
    pub label: String,
    pub contents: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputSourceKind {
    #[default]
    Path,
    Inline,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputFingerprint {
    pub path: String,
    #[serde(default)]
    pub source_kind: InputSourceKind,
    pub size_bytes: u64,
    pub modified_unix_seconds: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_digest_sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClassificationSource {
    Explicit,
    InferredConfirmed,
    InferredOverridden,
}

impl ClassificationSource {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationFieldProvenance {
    pub source: ClassificationSource,
    pub rationale: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signals: Vec<String>,
}

impl ClassificationFieldProvenance {
    pub fn new(
        source: ClassificationSource,
        rationale: impl Into<String>,
        signals: Vec<String>,
    ) -> Self {
        Self { source, rationale: rationale.into(), signals }
    }

    pub fn explicit_risk() -> Self {
        Self::new(
            ClassificationSource::Explicit,
            "Risk class was supplied explicitly at run start.",
            Vec::new(),
        )
    }

    pub fn explicit_zone() -> Self {
        Self::new(
            ClassificationSource::Explicit,
            "Usage zone was supplied explicitly at run start.",
            Vec::new(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassificationProvenance {
    pub risk: ClassificationFieldProvenance,
    pub zone: ClassificationFieldProvenance,
}

impl ClassificationProvenance {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Run {
    pub id: String,
    pub mode: Option<Mode>,
    pub risk: Option<RiskClass>,
    pub zone: Option<UsageZone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_context: Option<SystemContext>,
    pub state: RunState,
    pub created_at: OffsetDateTime,
    pub artifact_contract: Option<ArtifactContract>,
    pub evidence_bundle_ref: Option<String>,
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
