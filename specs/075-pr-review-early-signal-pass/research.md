# Research: Early Signal Pass (First-Pass Risk Discovery)

**Feature**: 075-pr-review-early-signal-pass
**Phase**: 0 — Outline & Research
**Date**: 2026-06-09

## Research Tasks & Decisions

### 1. Early Signal Check Implementation Approach

**Decision**: Implement early signal checks as a set of deterministic rule functions in a new `early_signal.rs` module under `canon-engine/src/orchestrator/service/`. Each rule is a pure function taking a diff context and returning `Vec<EarlySignalFinding>`.

**Rationale**: Early signal checks are deterministic file-system and manifest inspections per spec FR-005. They do not invoke the LLM, shell out to build tools, or require network access. Pure functions are trivially testable in `#[cfg(test)]`.

**Alternatives considered**:
- Separate `canon-adapters` check plugin system: Over-engineered for v1. Ten deterministic checks don't warrant a plugin architecture.
- Shell-out to external linters: Adds latency, dependency on tool availability, and nondeterminism. Rejected.

**Check rules identified** (from spec FR-005):
1. `build.command.removed_file_reference` — detects files referenced in build manifests (Justfile, Makefile, Cargo.toml workspace members) that no longer exist on disk
2. `manifest.stale_dependency` — detects Cargo.toml dependency versions that don't resolve
3. `manifest.schema_drift` — detects serialized artifact shapes (e.g., run.toml, context.toml) with fields that don't match their domain types
4. `reference.dangling_import` — detects `mod`/`use` statements referencing removed files
5. `test.missing_for_changed_behavior` — detects changed `pub fn` signatures without corresponding test changes
6. `naming.drift` — detects renamed files without corresponding import updates
7. `validation.failure` — detects `cargo check` or `cargo clippy` failures on the diff

### 2. JSON Event Schema Design

**Decision**: Define a typed Rust struct `EarlySignalEvent` with serde derives in `canon-engine/src/domain/review.rs`. The struct serializes to the JSON schema specified in the clarification session. Use an enum for event type (`EarlySignalEventKind`).

**Rationale**: Typed serde structs satisfy the Rust language rules (no ad hoc `json!` assembly). The enum enforces exhaustiveness. The schema is stable because it's code-generated from the struct definition.

**Event types**: `Started`, `FileClassified`, `FindingDetected`, `Completed`, `Skipped`, `Failed` — each variant carries only the fields relevant to that event type.

**Finding ID format**: `ES<NNN>` where NNN is a zero-padded sequential number within the run. Prefix `ES` distinguishes from other finding types.

### 3. Layer Directory Generation

**Decision**: `prepare` creates a `layers/` directory under the run's `pr-review/` path. Each layer gets a numbered subdirectory: `01-early-signal/`, `02-application-source/`, etc. Each contains `instructions.md` (review guidance), `required-context.tsv` (files to load), and an empty `output.md`.

**Rationale**: Flat numbered directories with descriptive slugs make the structure both machine-parseable (by number) and human-readable (by slug). The TSV format for context is LLM-scannable per spec requirement.

**Layer order** (from spec FR-007):
1. `01-early-signal` — Canon-executed
2. `02-application-source` — LLM-executed
3. `03-high-risk-surfaces` — LLM-executed
4. `04-related-context` — LLM-executed
5. `05-logical-stress` — LLM-executed
6. `06-tests` — LLM-executed
7. `07-coverage-accounting` — Canon-compiled at `finalize`

### 4. Run State Extension

**Decision**: Add `AwaitingReviewerOutput` variant to the existing `RunState` enum in `canon-engine/src/domain/run.rs`. This state is set by `prepare` after deterministic work completes. `accept` transitions from `AwaitingReviewerOutput` to either `Completed` (if all layers done) or `AwaitingApproval` (if gates remain).

**Rationale**: A dedicated state makes state-machine validation explicit. `accept` can guard on this state. The `finalize` gate can check that the state isn't still `AwaitingReviewerOutput`.

**Alternative considered**: Reuse existing `Draft` or `AwaitingApproval` states — rejected because these don't express "Canon's part is done, now waiting for the LLM agent."

### 5. Stdout JSON Rendering

**Decision**: Implement a `render_early_signal_events` function in `canon-cli/src/output/early_signal.rs` that receives `Vec<EarlySignalEvent>` and writes one JSON object per line to stdout when `--output json` is set. Uses existing `OutputFormat::Json` dispatch pattern.

**Rationale**: The existing Canon CLI already has an `--output json` flag with consistent rendering conventions. Adding to this pattern avoids introducing a new output mechanism.

**Human-readable fallback**: When `--output text` (default), render a markdown-style summary instead of JSON. The trace.jsonl is always persisted regardless of output format.

### 6. Acceptance Validation

**Decision**: `accept` reads each layer's `output.md` from disk and validates:
1. File exists and is non-empty
2. Contains required sections (layer-specific)
3. Layer coverage record is present
4. Deferred layers have non-empty reason strings

Validation failures produce structured errors referencing the layer path and missing requirement.

**Rationale**: File-system validation is deterministic and testable. Per spec FR-028, `accept` must not infer completion from `instructions.md` presence alone.

### 7. Existing Code Integration

**Decision**: Extend `mode_pr_review_prepare.rs` with the early signal execution step, layer directory generation, and review-plan.md rendering. Extend `mode_pr_review_accept.rs` with layer output validation. Extend `mode_pr_review_finalize.rs` with coverage accounting compilation.

**Rationale**: The `pr-review` mode already has prepare/accept/finalize sub-phases from spec 074. Early signal pass and layer management fit naturally into these phases without restructuring.

**Files to modify** (from grep of existing codebase):
- `crates/canon-cli/src/app.rs` — add `--skip-early-signal` to `PrReviewPrepare`
- `crates/canon-cli/src/commands/pr_review.rs` — dispatch early signal events
- `crates/canon-engine/src/orchestrator/service/mode_pr_review_prepare.rs` — early signal execution + layer dir generation
- `crates/canon-engine/src/orchestrator/service/mode_pr_review_accept.rs` — layer output validation
- `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs` — coverage accounting
- `crates/canon-engine/src/persistence/layout.rs` — new path helpers
- `crates/canon-engine/src/domain/run.rs` — `AwaitingReviewerOutput` state

**New files**:
- `crates/canon-engine/src/orchestrator/service/early_signal.rs` — check executor + rule definitions
- `crates/canon-engine/src/domain/review.rs` — `ReviewLayer`, `EarlySignalFinding`, `EarlySignalEvent`
- `crates/canon-cli/src/output/early_signal.rs` — stdout JSON rendering
