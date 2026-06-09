# Data Model: Early Signal Pass

**Feature**: 075-pr-review-early-signal-pass
**Phase**: 1 — Design & Contracts

## Entities

### EarlySignalFinding

A high-confidence problem discovered during the early signal pass (layer 1).

| Field | Type | Description |
|---|---|---|
| `finding_id` | `String` | Stable ID, format `ES<NNN>`. Consistent across all outputs. |
| `run_id` | `String` | The PR review run this finding belongs to. |
| `rule_id` | `String` | Dot-separated rule identifier, e.g. `build.command.removed_file_reference`. |
| `severity` | `EarlySignalSeverity` | Enum: `Blocking`, `High`, `Medium`, `Low`, `Info`. |
| `category` | `String` | Bucket: `build_ci`, `manifest`, `schema`, `reference`, `test`, `naming`, `validation`. |
| `path` | `String` | Repository-relative file path. |
| `start_line` | `Option<u32>` | Optional line range start. |
| `end_line` | `Option<u32>` | Optional line range end. |
| `summary` | `String` | Short human-readable description. |
| `evidence_context_ids` | `Vec<String>` | References to context entries supporting this finding. |
| `suggested_layer` | `String` | Suggested next review layer to investigate (e.g., `diff`, `app-source`). |
| `actionable_comment_candidate` | `bool` | Whether this finding should become a review comment. |

**Validation rules**:
- `finding_id` must be unique within a run and stable across all output channels.
- `rule_id` must correspond to a defined early signal rule.
- `severity` must be `Blocking` for findings that prevent successful build/test/validation.
- `path` must be a repository-relative path that exists in the diff or workspace.
- `evidence_context_ids` must reference entries present in the run's context index.

### EarlySignalEvent

A structured JSON record emitted during early signal execution.

| Field | Type | Description |
|---|---|---|
| `event` | `EarlySignalEventKind` | Enum variant: `Started`, `FileClassified`, `FindingDetected`, `Completed`, `Skipped`, `Failed`. |
| `run_id` | `String` | The PR review run this event belongs to. |
| `timestamp` | `OffsetDateTime` | When the event was recorded. |

**Per-event-kind payloads**:
- `Started`: `total_files` (u32).
- `FileClassified`: `path` (String), `risk_class` (String), `reason` (String).
- `FindingDetected`: Full `EarlySignalFinding` payload inline.
- `Completed`: `total_files_classified` (u32), `total_findings` (u32), `findings_by_severity` (map), `findings_by_bucket` (map), `high_risk_files` (Vec<String>), `suggested_next_layers` (Vec<String>), `early_signal_status` (String).
- `Skipped`: `reason` (String), `source` (String), `confidence_impact` (String).
- `Failed`: `error` (String), `rule_id` (Option<String>), `partial_findings_count` (u32).

### ReviewLayer

One of the seven ordered phases in the PR review workflow.

| Field | Type | Description |
|---|---|---|
| `ordinal` | `u8` | 1-7, unique. |
| `name` | `String` | Slug: `early-signal`, `application-source`, etc. |
| `display_name` | `String` | Human-readable: "Early Signal Pass". |
| `executed_by` | `LayerExecutor` | Enum: `Canon` or `LlmAgent`. |
| `status` | `LayerStatus` | Enum: `Pending`, `InProgress`, `Completed`, `Deferred`, `Failed`. |
| `deferral_reason` | `Option<String>` | Required if status is `Deferred`. |
| `output_path` | `PathBuf` | Path to `layers/<NN>-<name>/output.md`. |
| `findings_count` | `u32` | Number of findings produced by this layer. |

### CoverageAccounting

The final artifact listing each review layer's disposition.

| Field | Type | Description |
|---|---|---|
| `run_id` | `String` | The PR review run. |
| `layers` | `Vec<LayerCoverageEntry>` | One entry per layer (1-7). |
| `overall_confidence` | `String` | `high`, `medium`, `low`, or `insufficient`. |
| `deferred_areas` | `Vec<String>` | High-risk areas explicitly not reviewed. |

**LayerCoverageEntry**:
| Field | Type | Description |
|---|---|---|
| `layer_name` | `String` | `early-signal`, `application-source`, etc. |
| `status` | `String` | `reviewed`, `deferred`, `skipped`. |
| `reason` | `Option<String>` | Required if status is `deferred` or `skipped`. |
| `artifact_ref` | `Option<String>` | Path to the layer's output artifact. |

### RunState extension

Add one variant to the existing `RunState` enum:

```rust
/// Canon has completed deterministic preparation (early signal pass,
/// file classification, context indexes, layer instructions).
/// Waiting for the LLM agent to write layer outputs and invoke `accept`.
AwaitingReviewerOutput,
```

**State transitions**:
- `Draft` → `AwaitingReviewerOutput` (after `prepare` completes)
- `AwaitingReviewerOutput` → `Completed` (after `accept` validates all layers — canonical post-accept state)
- `AwaitingReviewerOutput` → `AwaitingApproval` (if `accept` finds gating issues requiring approval)
