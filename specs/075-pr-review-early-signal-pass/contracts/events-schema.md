# Events Schema Contract: Early Signal Events

**Feature**: 075-pr-review-early-signal-pass

## Channel: stdout JSON (`--output json`)

Each line is a single JSON object with an `event` discriminator field.

### `early_signal.started`

```json
{
  "event": "early_signal.started",
  "run_id": "prr_<uuid>",
  "timestamp": "2026-06-09T12:00:00Z",
  "total_files": 15
}
```

### `early_signal.file_classified`

```json
{
  "event": "early_signal.file_classified",
  "run_id": "prr_<uuid>",
  "timestamp": "2026-06-09T12:00:01Z",
  "path": "crates/canon-engine/src/orchestrator/service/run_op.rs",
  "risk_class": "high",
  "reason": "contains match statement on Mode enum, all variants must be covered"
}
```

### `early_signal.finding_detected`

```json
{
  "event": "early_signal.finding_detected",
  "run_id": "prr_<uuid>",
  "timestamp": "2026-06-09T12:00:02Z",
  "rule_id": "build.command.removed_file_reference",
  "finding_id": "ES001",
  "severity": "blocking",
  "category": "build_ci",
  "path": "justfile",
  "start_line": 12,
  "end_line": 18,
  "summary": "Validation command still references a removed or relocated package.",
  "evidence_context_ids": ["C001", "C014"],
  "suggested_layer": "diff",
  "actionable_comment_candidate": true
}
```

### `early_signal.completed`

```json
{
  "event": "early_signal.completed",
  "run_id": "prr_<uuid>",
  "timestamp": "2026-06-09T12:00:05Z",
  "total_files_classified": 15,
  "total_findings": 3,
  "findings_by_severity": {"blocking": 1, "medium": 1, "low": 1},
  "findings_by_bucket": {"build_ci": 1, "reference": 1, "naming": 1},
  "high_risk_files": ["crates/canon-engine/src/orchestrator/service/run_op.rs"],
  "suggested_next_layers": ["app-source", "high-risk-surfaces"],
  "early_signal_status": "completed"
}
```

### `early_signal.skipped`

```json
{
  "event": "early_signal.skipped",
  "run_id": "prr_<uuid>",
  "timestamp": "2026-06-09T12:00:00Z",
  "reason": "Debugging only — verifying accept/finalize flow with pre-populated layer outputs.",
  "source": "operator",
  "confidence_impact": "medium"
}
```

### `early_signal.failed`

```json
{
  "event": "early_signal.failed",
  "run_id": "prr_<uuid>",
  "timestamp": "2026-06-09T12:00:10Z",
  "error": "Shell command 'cargo check' timed out after 60 seconds.",
  "rule_id": "validation.failure",
  "partial_findings_count": 2
}
```

## Channel: trace JSONL (`.canon/runs/<run_id>/pr-review/traces/early-signal.jsonl`)

Same event types as stdout, plus additional diagnostic fields:

### Additional trace fields

| Field | Type | When present | Description |
|---|---|---|---|
| `duration_ms` | `u64` | `completed`, `failed` | Wall-clock duration of the entire pass or a rule execution. |
| `rule_duration_ms` | `u64` | `finding_detected`, rule-scoped events | Duration of a single rule check. |
| `skipped_rules` | `Vec<String>` | `completed` | Rule IDs that were skipped and why. |
| `stack_trace` | `Option<String>` | `failed` | Error backtrace if available. |
| `host_info` | `String` | `started` | `uname -a` output or equivalent. |

## Stable ID contract

- `finding_id` format: `ES` followed by 3-digit zero-padded sequential number (`ES001`, `ES002`, ...).
- `finding_id` is stable within a run: the same finding has the same ID in stdout, trace, `findings.json`, `findings.tsv`, and `summary.md`.
- Rule IDs are fixed constants per check rule, documented in this file. Adding a new rule requires adding its ID here.

## Defined rule IDs

| Rule ID | Check |
|---|---|
| `build.command.removed_file_reference` | Build manifest references a removed file |
| `manifest.stale_dependency` | Cargo.toml dependency version doesn't resolve |
| `manifest.schema_drift` | Serialized artifact shape doesn't match domain type |
| `reference.dangling_import` | `mod`/`use` references a removed file |
| `test.missing_for_changed_behavior` | Changed `pub fn` without test changes |
| `naming.drift` | Renamed file without import updates |
| `validation.failure` | `cargo check` or `cargo clippy` failure |
