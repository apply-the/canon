//! Review domain types for the PR review workflow.
//!
//! Defines the core data structures for review layers, early signal findings,
//! early signal events, and coverage accounting. These types are used by the
//! early signal pass executor, the layer directory generator, the accept/finalize
//! validators, and the CLI stdout renderer.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use time::OffsetDateTime;

// ── Early Signal Severity ─────────────────────────────────────────────────────

/// Severity classification for an early signal finding.
///
/// Ordered from most to least severe. `Blocking` findings prevent the review
/// from meaningfully proceeding without remediation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EarlySignalSeverity {
    /// The finding blocks successful build, test, or validation.
    Blocking,
    /// High-confidence issue requiring attention before deeper review.
    High,
    /// Notable issue that should be addressed.
    Medium,
    /// Minor issue or style concern.
    Low,
    /// Informational observation, not a defect.
    Info,
}

// ── Early Signal Finding ──────────────────────────────────────────────────────

/// A high-confidence problem discovered during the early signal pass (layer 1).
///
/// Each finding carries a stable ID, a reference to the detection rule, a
/// severity classification, and location information. Finding IDs are stable
/// across all output channels (stdout, trace, findings artifacts).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EarlySignalFinding {
    /// Stable finding ID, format `ES<NNN>`. Consistent across all outputs.
    pub finding_id: String,
    /// The PR review run this finding belongs to.
    pub run_id: String,
    /// Dot-separated rule identifier, e.g. `build.command.removed_file_reference`.
    pub rule_id: String,
    /// Severity classification.
    pub severity: EarlySignalSeverity,
    /// Category bucket: `build_ci`, `manifest`, `schema`, `reference`, `test`, `naming`, `validation`.
    pub category: String,
    /// Repository-relative file path.
    pub path: String,
    /// Optional line range start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u32>,
    /// Optional line range end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    /// Short human-readable description.
    pub summary: String,
    /// References to context entries supporting this finding.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_context_ids: Vec<String>,
    /// Suggested next review layer to investigate.
    pub suggested_layer: String,
    /// Whether this finding should become a review comment candidate.
    pub actionable_comment_candidate: bool,
}

// ── Early Signal Event ────────────────────────────────────────────────────────

/// The kind of early signal lifecycle event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EarlySignalEventKind {
    /// The early signal pass has started.
    Started,
    /// A single changed file has been classified.
    FileClassified,
    /// A finding was detected by a check rule.
    FindingDetected,
    /// The early signal pass completed successfully.
    Completed,
    /// The early signal pass was skipped via `--skip-early-signal`.
    Skipped,
    /// The early signal pass encountered a non-recoverable error.
    Failed,
}

/// Payload for an `early_signal.started` event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartedPayload {
    /// Total number of changed files to classify.
    pub total_files: u32,
}

/// Payload for an `early_signal.file_classified` event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileClassifiedPayload {
    /// Repository-relative path of the classified file.
    pub path: String,
    /// Risk classification: `high`, `medium`, `low`.
    pub risk_class: String,
    /// Reason for the risk classification.
    pub reason: String,
}

/// Payload for an `early_signal.completed` summary event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletedPayload {
    /// Total number of files classified.
    pub total_files_classified: u32,
    /// Total number of findings across all rules.
    pub total_findings: u32,
    /// Findings grouped by severity (e.g. `{"blocking": 1, "low": 2}`).
    pub findings_by_severity: BTreeMap<String, u32>,
    /// Findings grouped by bucket/category.
    pub findings_by_bucket: BTreeMap<String, u32>,
    /// Files identified as high-risk.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub high_risk_files: Vec<String>,
    /// Suggested next review layers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suggested_next_layers: Vec<String>,
    /// Status of the early signal pass: `completed`, `skipped_with_reason`, or `failed`.
    pub early_signal_status: String,
}

/// Payload for an `early_signal.skipped` event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkippedPayload {
    /// Reason the early signal pass was skipped.
    pub reason: String,
    /// Source of the skip decision: `operator` or `agent`.
    pub source: String,
    /// Impact on review confidence: `low`, `medium`, `high`.
    pub confidence_impact: String,
}

/// Payload for an `early_signal.failed` event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailedPayload {
    /// Human-readable error description.
    pub error: String,
    /// The rule that failed, if a specific rule is implicated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// Number of findings that were collected before the failure.
    pub partial_findings_count: u32,
}

/// A structured JSON record emitted during early signal execution.
///
/// The `event` field discriminates the payload variant. Stdout receives one
/// event per line when `--output json`. The trace JSONL receives events with
/// additional diagnostic fields (duration, host info, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarlySignalEvent {
    /// Discriminator for the event payload.
    pub event: EarlySignalEventKind,
    /// The PR review run this event belongs to.
    pub run_id: String,
    /// When the event was recorded.
    pub timestamp: OffsetDateTime,
    /// Total files (only for `started`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_files: Option<u32>,
    /// File classification data (only for `file_classified`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_classified: Option<FileClassifiedPayload>,
    /// Finding payload (only for `finding_detected`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<EarlySignalFinding>,
    /// Summary payload (only for `completed`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<CompletedPayload>,
    /// Skip payload (only for `skipped`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<SkippedPayload>,
    /// Failure payload (only for `failed`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<FailedPayload>,
    // ── Trace-only fields ──
    /// Wall-clock duration in milliseconds (trace only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// Duration of a single rule check (trace only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_duration_ms: Option<u64>,
    /// Rule IDs that were skipped and why (trace only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_rules: Option<Vec<String>>,
    /// Error backtrace if available (trace only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    /// Host info string (trace only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_info: Option<String>,
}

// ── Review Layer ──────────────────────────────────────────────────────────────

/// Identifies who is responsible for executing a review layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerExecutor {
    /// Canon runs this layer deterministically (layer 1).
    Canon,
    /// The LLM agent performs semantic reasoning (layers 2–6).
    LlmAgent,
}

/// The completion status of a review layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayerStatus {
    /// Layer has not been started.
    Pending,
    /// Layer is actively being executed.
    InProgress,
    /// Layer has been completed with a valid output.
    Completed,
    /// Layer has been explicitly deferred with a reason.
    Deferred,
    /// Layer execution failed.
    Failed,
}

/// One of the seven ordered phases in the PR review workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewLayer {
    /// Ordinal position 1–7, unique.
    pub ordinal: u8,
    /// Slug: `early-signal`, `application-source`, etc.
    pub name: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Who executes this layer.
    pub executed_by: LayerExecutor,
    /// Current completion status.
    pub status: LayerStatus,
    /// Required if status is `Deferred`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferral_reason: Option<String>,
    /// Path to the layer's output.md.
    pub output_path: PathBuf,
    /// Number of findings produced by this layer.
    #[serde(default)]
    pub findings_count: u32,
}

// ── Coverage Accounting ───────────────────────────────────────────────────────

/// One entry in the coverage accounting table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerCoverageEntry {
    /// Layer slug.
    pub layer_name: String,
    /// Disposition: `reviewed`, `deferred`, or `skipped`.
    pub status: String,
    /// Required if status is `deferred` or `skipped`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Path to the layer's output artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_ref: Option<String>,
}

/// The final artifact listing each review layer's disposition.
///
/// Produced at `finalize`. The `overall_confidence` field reflects whether
/// all layers were reviewed (`high`), one was deferred/skipped (`medium`),
/// multiple were deferred (`low`), or no semantic layers were reviewed
/// (`insufficient`). A skipped early signal automatically caps confidence
/// at `medium` or lower.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageAccounting {
    /// The PR review run this accounting belongs to.
    pub run_id: String,
    /// One entry per layer (1–7).
    pub layers: Vec<LayerCoverageEntry>,
    /// Overall review confidence.
    pub overall_confidence: String,
    /// High-risk areas explicitly not reviewed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deferred_areas: Vec<String>,
}

// ── File Classification ──────────────────────────────────────────────────────

/// A classification result for a single changed file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileClassification {
    /// Repository-relative path.
    pub path: String,
    /// Risk classification: `high`, `medium`, `low`.
    pub risk_class: String,
    /// Reason for the classification.
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    #[test]
    fn early_signal_finding_serialization_round_trip() {
        let finding = EarlySignalFinding {
            finding_id: "ES001".to_string(),
            run_id: "prr-test".to_string(),
            rule_id: "reference.dangling_import".to_string(),
            severity: EarlySignalSeverity::Blocking,
            category: "reference".to_string(),
            path: "src/main.rs".to_string(),
            start_line: Some(5),
            end_line: Some(5),
            summary: "dangling import".to_string(),
            evidence_context_ids: vec!["C001".to_string()],
            suggested_layer: "diff".to_string(),
            actionable_comment_candidate: true,
        };

        let json = serde_json::to_string(&finding).expect("serialize");
        let round_trip: EarlySignalFinding = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_trip.finding_id, "ES001");
        assert_eq!(round_trip.rule_id, "reference.dangling_import");
        assert_eq!(round_trip.severity, EarlySignalSeverity::Blocking);
    }

    #[test]
    fn early_signal_event_kind_serialization() {
        let kinds = [
            (EarlySignalEventKind::Started, "started"),
            (EarlySignalEventKind::FileClassified, "file_classified"),
            (EarlySignalEventKind::FindingDetected, "finding_detected"),
            (EarlySignalEventKind::Completed, "completed"),
            (EarlySignalEventKind::Skipped, "skipped"),
            (EarlySignalEventKind::Failed, "failed"),
        ];

        for (kind, expected) in kinds {
            let json = serde_json::to_string(&kind).expect("serialize");
            assert!(json.contains(expected), "expected {expected} in {json}");
            let round_trip: EarlySignalEventKind =
                serde_json::from_str(&json).expect("deserialize");
            assert_eq!(round_trip, kind);
        }
    }

    #[test]
    fn event_serialization_discriminates_kinds() {
        let started = EarlySignalEvent {
            event: EarlySignalEventKind::Started,
            run_id: "prr-1".to_string(),
            timestamp: OffsetDateTime::UNIX_EPOCH,
            total_files: Some(10),
            file_classified: None,
            finding: None,
            completed: None,
            skipped: None,
            failed: None,
            duration_ms: None,
            rule_duration_ms: None,
            skipped_rules: None,
            stack_trace: None,
            host_info: None,
        };
        let json = serde_json::to_string(&started).expect("serialize");
        assert!(json.contains("\"event\":\"started\""));
        assert!(json.contains("\"total_files\":10"));
    }

    #[test]
    fn coverage_accounting_defaults() {
        let accounting = CoverageAccounting {
            run_id: "prr-1".to_string(),
            layers: vec![],
            overall_confidence: "high".to_string(),
            deferred_areas: vec![],
        };
        assert_eq!(accounting.overall_confidence, "high");
        assert!(accounting.layers.is_empty());
    }
}
