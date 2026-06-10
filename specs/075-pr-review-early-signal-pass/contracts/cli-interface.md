# CLI Interface Contract: Early Signal Pass

**Feature**: 075-pr-review-early-signal-pass

## Commands

### `canon pr-review prepare`

**Purpose**: Run deterministic preparation: early signal pass, file classification, context indexes, ordered layer instructions.

**Signature**:

```
canon pr-review prepare --base <ref> --head <ref> [--skip-early-signal] [--output <format>]
```

**Flags**:

| Flag | Type | Required | Description |
|---|---|---|---|
| `--base` | String | Yes | Base Git ref (e.g., `main`, `HEAD~1`) |
| `--head` | String | Yes | Head Git ref (e.g., `feature-branch`, `HEAD`) |
| `--skip-early-signal` | Flag | No | Opt-out of early signal pass. Requires `--skip-reason`. |
| `--skip-reason` | String | Conditional | Required when `--skip-early-signal` is set. Non-empty string. |
| `--output` | OutputFormat | No | `text` (default) or `json`. Controls stdout format. |

**Stdout (--output json)**:

One JSON object per line. Event types:

```json
{"event":"early_signal.started","run_id":"...","timestamp":"...","total_files":15}
{"event":"early_signal.file_classified","run_id":"...","timestamp":"...","path":"src/main.rs","risk_class":"low","reason":"no structural changes"}
{"event":"early_signal.finding_detected","run_id":"...","rule_id":"build.command.removed_file_reference","finding_id":"ES001","severity":"blocking","category":"build_ci","path":"justfile","start_line":12,"end_line":18,"summary":"...","evidence_context_ids":["C001"],"suggested_layer":"diff","actionable_comment_candidate":true}
{"event":"early_signal.completed","run_id":"...","timestamp":"...","total_files_classified":15,"total_findings":2,"findings_by_severity":{"blocking":1,"low":1},"findings_by_bucket":{"build_ci":1,"naming":1},"high_risk_files":[],"suggested_next_layers":["app-source","related-context"],"early_signal_status":"completed"}
```

**Stdout (--output text, default)**:

Markdown summary with:
- Early signal pass status header
- Findings table (id, severity, category, path, summary)
- Summary counts
- Suggested next layers

**Exit codes**:

| Code | Meaning |
|---|---|
| 0 | Prepare completed successfully (with or without findings) |
| 1 | Validation error (e.g., `--skip-early-signal` without `--skip-reason`) |
| 2 | Early signal pass failed (non-recoverable error) |

**Side effects**:

- Creates run under `.canon/runs/<run_id>/pr-review/`
- Creates `early-signal/` with `findings.tsv`, `findings.json`, `summary.md`
- Creates `traces/early-signal.jsonl`
- Creates `layers/01-early-signal/` through `layers/07-coverage-accounting/` with `instructions.md` and `required-context.tsv`
- Creates `review-plan.md`
- Persists run manifest, context, state (`AwaitingReviewerOutput`), artifact contract

### `canon pr-review accept`

**Purpose**: Validate reviewer layer outputs.

**Signature**:

```
canon pr-review accept [--output <format>]
```

**Validation rules**:

1. Run state must be `AwaitingReviewerOutput`
2. Each required layer (1-7) must have either a non-empty `output.md` with valid coverage record, or a deferral with non-empty reason
3. Layer 1 (early signal) is pre-populated if executed; if skipped, `--skip-early-signal` metadata must be present
4. Coverage accounting must not have gaps (every layer accounted for)

**Exit codes**:

| Code | Meaning |
|---|---|
| 0 | All layers validated, run transitions to Completed or AwaitingApproval |
| 1 | Validation failure with specific layer error messages |

### `canon pr-review finalize`

**Purpose**: Compile coverage accounting, render final review artifacts.

**Signature**:

```
canon pr-review finalize [--output <format>]
```

**Validation rules**:

1. Run must have been accepted (`accept` must have succeeded)
2. Coverage accounting must be complete
3. Early signal status must be included in coverage section
4. Skipped early signal must reduce overall confidence and must not imply full early-risk coverage
