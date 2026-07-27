//! Typed state and stable errors for the Canon 0.72.6 bridge.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(super) const SUPPORTED_SOURCE_VERSION: &str = "0.72.6";
pub(super) const SOURCE_SCHEMA_VERSION: &str = "canon-workspace-state-0.72.6";
pub(super) const TARGET_SCHEMA_VERSION: &str = "canon-workspace-state-0.90";
pub(super) const MIGRATION_IMPLEMENTATION_VERSION: &str = "canon-bridge-090-v1";

/// A `.canon` state root and explicit historical release assertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationSource {
    /// Product-owned `.canon` state root.
    pub root: PathBuf,
    /// Source version supplied by the bridge caller.
    pub declared_version: String,
}

impl MigrationSource {
    /// Creates a source descriptor without mutating it.
    pub fn new(root: impl Into<PathBuf>, declared_version: impl Into<String>) -> Self {
        Self { root: root.into(), declared_version: declared_version.into() }
    }
}

/// Planned handling of the inspected run inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationAction {
    /// All records remain historical but active in the converted registry.
    Convert,
    /// One or more records must be archived outside the active registry.
    ArchiveUnsupported,
}

/// One recognized historical Canon record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyStateItem {
    /// Logical run identity.
    pub identity: String,
    /// Historical record kind.
    pub kind: String,
    /// Historical lifecycle state.
    pub state: String,
    /// Historical mode.
    pub mode: String,
    /// Whether the run is excluded from ordinary 0.90 loading.
    pub archive_required: bool,
    /// Stable archive reason.
    pub archive_reason: Option<String>,
    /// Whether historical verification material exists.
    pub has_legacy_verification: bool,
}

/// Deterministic non-mutating source inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationInspection {
    /// Admitted source.
    pub source: MigrationSource,
    /// Digest of every source byte and logical path.
    pub source_digest: String,
    /// Explicit supported release.
    pub source_version: String,
    /// Schema fingerprint extracted from the tagged source.
    pub source_schema_version: String,
    /// Ordered run inventory.
    pub items: Vec<LegacyStateItem>,
    /// Planned action.
    pub action: MigrationAction,
    /// Ordered warnings.
    pub warnings: Vec<String>,
}

/// Exact source-bound bridge plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMigrationPlan {
    /// Deterministic migration identifier.
    pub migration_id: String,
    /// Admitted source.
    pub source: MigrationSource,
    /// Inspected source digest.
    pub source_digest: String,
    /// Historical schema identity.
    pub source_schema_version: String,
    /// Target schema identity.
    pub target_schema_version: String,
    /// Migrator implementation identity.
    pub implementation_version: String,
    /// Planned action.
    pub action: MigrationAction,
    /// Ordered inventory.
    pub items: Vec<LegacyStateItem>,
    /// Ordered warnings.
    pub warnings: Vec<String>,
}

/// Durable fault boundaries in lifecycle order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationBoundary {
    /// No backup effect has begun.
    BeforeBackup,
    /// Backup bytes are durable.
    BackupCreated,
    /// Backup verification is durable.
    BackupVerified,
    /// Staging is incomplete.
    DuringTargetStaging,
    /// Staged target verification is durable.
    StagedTargetVerified,
    /// Replacement has not begun.
    BeforeReplacement,
    /// Replacement occurred without durable completion.
    AfterReplacement,
    /// Completion is durable before lock release.
    AfterCompletion,
}

impl MigrationBoundary {
    /// Returns all declared fault points.
    pub const fn all() -> &'static [Self] {
        &[
            Self::BeforeBackup,
            Self::BackupCreated,
            Self::BackupVerified,
            Self::DuringTargetStaging,
            Self::StagedTargetVerified,
            Self::BeforeReplacement,
            Self::AfterReplacement,
            Self::AfterCompletion,
        ]
    }

    /// Returns the journal label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BeforeBackup => "before_backup",
            Self::BackupCreated => "after_backup_creation",
            Self::BackupVerified => "after_backup_verification",
            Self::DuringTargetStaging => "during_target_staging",
            Self::StagedTargetVerified => "after_staged_target_verification",
            Self::BeforeReplacement => "before_replacement",
            Self::AfterReplacement => "after_replacement_before_completion",
            Self::AfterCompletion => "after_completion_record",
        }
    }
}

/// Fault control used by recovery qualification.
#[derive(Debug, Clone, Copy, Default)]
pub struct MigrationControl {
    /// Boundary after which apply returns control.
    pub stop_after: Option<MigrationBoundary>,
}

impl MigrationControl {
    /// Creates a stop control.
    pub const fn stop_after(boundary: MigrationBoundary) -> Self {
        Self { stop_after: Some(boundary) }
    }
}

/// Typed apply or recovery status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationStatus {
    /// Source is unchanged and a fresh apply is safe.
    SafeRetry,
    /// Target is committed and verified.
    Complete,
    /// Exact target was already committed.
    AlreadyMigrated,
    /// Durable reconciliation is required.
    RecoveryRequired,
}

/// One deterministic verification result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalVerificationResult {
    /// Stable check identity.
    pub check: String,
    /// Check outcome.
    pub passed: bool,
    /// Portable detail.
    pub detail: String,
}

/// A semantic limitation retained in the report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticLossRecord {
    /// Stable code.
    pub code: String,
    /// Logical item identity.
    pub item_identity: String,
    /// Portable explanation.
    pub detail: String,
}

/// A run that cannot enter the active 0.90 registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedStateRecord {
    /// Logical run identity.
    pub item_identity: String,
    /// Stable reason.
    pub reason_code: String,
    /// Whether a newly admitted run is mandatory.
    pub requires_new_admitted_session: bool,
}

/// Portable deterministic conversion report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionReport {
    /// Deterministic migration identity.
    pub migration_id: String,
    /// Source product.
    pub source_product: String,
    /// Source version.
    pub source_version: String,
    /// Source schema fingerprint.
    pub source_schema_version: String,
    /// Target release line.
    pub target_version_line: String,
    /// Target schema.
    pub target_schema_version: String,
    /// Complete source digest.
    pub source_digest: String,
    /// Volatile inspection timestamp.
    pub inspection_timestamp_ms: u64,
    /// Volatile start timestamp.
    pub migration_start_timestamp_ms: u64,
    /// Volatile completion timestamp.
    pub migration_completion_timestamp_ms: Option<u64>,
    /// Logical backup identity.
    pub backup_identity: Option<String>,
    /// Verified backup digest.
    pub backup_digest: Option<String>,
    /// Logical staging identity.
    pub staged_output_identity: Option<String>,
    /// Verified staged digest.
    pub staged_output_digest: Option<String>,
    /// Converted run count.
    pub converted_item_count: usize,
    /// Archived run count.
    pub archived_item_count: usize,
    /// Skipped run count.
    pub skipped_item_count: usize,
    /// Ordered warnings.
    pub warnings: Vec<String>,
    /// Ordered semantic limitations.
    pub semantic_losses: Vec<SemanticLossRecord>,
    /// Ordered unsupported records.
    pub unsupported_states: Vec<UnsupportedStateRecord>,
    /// Ordered verification results.
    pub verification_results: Vec<CanonicalVerificationResult>,
    /// Terminal status.
    pub terminal_status: MigrationStatus,
    /// Stable terminal reason.
    pub terminal_reason_code: String,
    /// Recovery guidance when required.
    pub recovery_instructions: Option<String>,
}

impl ConversionReport {
    /// Clears declared volatile fields for semantic comparison.
    pub fn normalized(&self) -> Self {
        let mut report = self.clone();
        report.inspection_timestamp_ms = 0;
        report.migration_start_timestamp_ms = 0;
        report.migration_completion_timestamp_ms =
            report.migration_completion_timestamp_ms.map(|_| 0);
        if report.archived_item_count > 0 {
            report.staged_output_digest =
                report.staged_output_digest.map(|_| "volatile-archive-digest".to_string());
        }
        report
    }
}

/// Apply or recovery result.
#[derive(Debug, Clone)]
pub struct MigrationOutcome {
    /// Terminal or recoverable status.
    pub status: MigrationStatus,
    /// Portable report.
    pub report: ConversionReport,
    /// Internal backup root.
    pub backup_root: PathBuf,
    /// Verified backup digest.
    pub backup_digest: Option<String>,
}

/// Frozen repository-local failure codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationReasonCode {
    UnsupportedSourceVersion,
    UnsupportedFutureSchema,
    CorruptLegacyState,
    MixedLegacyState,
    SourceChanged,
    MigrationInProgress,
    BackupFailed,
    BackupVerificationFailed,
    StagingFailed,
    StagedVerificationFailed,
    ReplacementFailed,
    RecoveryRequired,
    ArchiveRequired,
    ArchiveVerificationFailed,
    InsufficientSpace,
    PermissionDenied,
    UnsafePath,
    UnsupportedFilesystem,
    IdempotencyConflict,
}

impl MigrationReasonCode {
    /// Returns the stable wire value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedSourceVersion => "unsupported_source_version",
            Self::UnsupportedFutureSchema => "unsupported_future_schema",
            Self::CorruptLegacyState => "corrupt_legacy_state",
            Self::MixedLegacyState => "mixed_legacy_state",
            Self::SourceChanged => "source_changed",
            Self::MigrationInProgress => "migration_in_progress",
            Self::BackupFailed => "backup_failed",
            Self::BackupVerificationFailed => "backup_verification_failed",
            Self::StagingFailed => "staging_failed",
            Self::StagedVerificationFailed => "staged_verification_failed",
            Self::ReplacementFailed => "replacement_failed",
            Self::RecoveryRequired => "recovery_required",
            Self::ArchiveRequired => "archive_required",
            Self::ArchiveVerificationFailed => "archive_verification_failed",
            Self::InsufficientSpace => "insufficient_space",
            Self::PermissionDenied => "permission_denied",
            Self::UnsafePath => "unsafe_path",
            Self::UnsupportedFilesystem => "unsupported_filesystem",
            Self::IdempotencyConflict => "idempotency_conflict",
        }
    }
}

impl std::fmt::Display for MigrationReasonCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Typed migration failure.
#[derive(Debug, Error)]
#[error("{reason}: {detail}")]
pub struct MigrationError {
    reason: MigrationReasonCode,
    detail: String,
}

impl MigrationError {
    pub(super) fn new(reason: MigrationReasonCode, detail: impl Into<String>) -> Self {
        Self { reason, detail: detail.into() }
    }

    /// Returns the stable reason code.
    pub const fn reason_code(&self) -> MigrationReasonCode {
        self.reason
    }
}
