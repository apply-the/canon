# Implementation Plan: Early Signal Pass (First-Pass Risk Discovery)

**Branch**: `075-pr-review-early-signal-pass` | **Date**: 2026-06-09 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/075-pr-review-early-signal-pass/spec.md`

## Summary

Integrate the early signal pass as the default-on first layer of `canon pr-review prepare`. Canon runs deterministic file-system and manifest checks (broken builds, stale manifests, schema drift, dangling references, missing tests, naming drift, validation failures) and emits structured JSON events on stdout + persisted JSONL traces. The LLM agent receives a full ordered review plan with layer instructions in a single `prepare` invocation, performs semantic review for layers 2–6, then invokes `accept` for validation. Finalization gates on honest coverage accounting. The CLI surface stays `prepare` → `accept` → `finalize` with an opt-out `--skip-early-signal` flag. For consistency, `--output json` is also available on `accept` and `finalize`.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024

**Primary Dependencies**: `clap` (CLI flags), `serde` + `serde_json` (event serialization), `tracing` (spans), `canon-engine` (orchestration, persistence, layout), `canon-cli` (CLI dispatch), `canon-adapters` (filesystem/shell adapters for checks)

**Storage**: Filesystem under `.canon/runs/<run_id>/pr-review/`. Existing layout: `run.toml`, `context.toml`, `state.toml`, `artifacts/`. New subdirectories: `early-signal/` (findings artifacts), `traces/early-signal.jsonl`, `layers/` (per-layer instructions and outputs).

**Testing**: `cargo test --lib` for unit tests (early signal check rules, event serialization, layer directory generation). `cargo test --test` for integration tests (`canon pr-review prepare` CLI contract, `--skip-early-signal` validation, `accept`/`finalize` gating). `assert_cmd` for CLI output assertions. `tempfile` for isolated workspace fixtures.

**Target Platform**: macOS + Linux CLI

**Project Type**: CLI tool with library crate (`canon-engine`) and binary crate (`canon-cli`)

**Performance Goals**: Early signal pass completes in ≤30 seconds for PRs up to 50 changed files (SC-001). Single `prepare` invocation must generate all 7 layer directories and artifacts.

**Constraints**: No new runtime dependencies. All early signal checks are deterministic (no LLM invocation). Stdout JSON must be stable for agent consumption. Trace JSONL persists regardless of `--output` flag. Layer progression does not add per-layer CLI subcommands.

**Scale/Scope**: 7 layers, ~10 early signal check rules, 29 FRs. Changes local to `canon-cli` (CLI flags, dispatch) and `canon-engine` (early signal executor, layer directory generation, event emission, `accept`/`finalize` gating).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|---|---|---|
| I. Method over prompting | PASS | Early signal pass is an explicit, deterministic method with ordered steps, required artifacts, and validation gates defined in FR-001–FR-007. |
| II. Artifact-first engineering | PASS | Every layer produces durable artifacts: `findings.json`, `findings.tsv`, `summary.md`, `trace.jsonl`, `instructions.md`, `output.md`, `review-plan.md`, coverage accounting. |
| III. Separation of generation and validation | PASS | Canon generates deterministic early signal findings and layer instructions. LLM agent performs semantic reasoning (layers 2–6). `accept` validates reviewer output independently. `finalize` compiles coverage accounting. |
| IV. Risk-aware execution | PASS | Early signal findings carry explicit severity. `--skip-early-signal` records impact on review confidence. High-risk areas must be reviewed or explicitly deferred. |
| V. Mode-driven workflows | PASS | This feature extends the existing `pr-review` mode (spec 074). No cross-mode leakage. |
| VI. Decision traceability | PASS | All findings carry rule IDs, finding IDs, category, and evidence context IDs. Skip decisions carry reasons. Coverage accounting records explicit deferral reasons. |
| VII. Invariants before implementation | PASS | Core invariants: early signal pass always runs by default; Canon never stops after layer 1; finalization requires honest coverage accounting; finding IDs are stable across all outputs. |
| VIII. Bounded context awareness | PASS | Scope bounded to `pr-review prepare/accept/finalize` phases. No changes to other Canon modes. |
| IX. Progressive autonomy | PASS | Canon handles deterministic, low-risk checks (layer 1). Semantic reasoning (layers 2–6) delegated to LLM agent under structured validation. |
| X. Layered verification | PASS | Seven ordered review layers with explicit deferral. `accept` validates structural completeness. `finalize` validates coverage accounting honesty. |

**Gate result**: ALL PASS — No violations. Proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/075-pr-review-early-signal-pass/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── cli-interface.md
│   └── events-schema.md
└── tasks.md             # Phase 2 output (speckit.tasks)
```

### Source Code (repository root)

```text
crates/
├── canon-cli/src/
│   ├── app.rs                       # --skip-early-signal flag on PrReviewPrepare, --output json
│   ├── commands/
│   │   └── pr_review.rs             # prepare/accept/finalize dispatch
│   └── output/
│       └── early_signal.rs          # NEW: stdout JSON event rendering
├── canon-engine/src/
│   ├── domain/
│   │   ├── run.rs                   # RunState: add AwaitingReviewerOutput
│   │   └── review.rs                # NEW: ReviewLayer, EarlySignalFinding, EarlySignalEvent (separate from review/findings.rs)
│   ├── orchestrator/service/
│   │   ├── mode_pr_review.rs        # Extend prepare/accept/finalize
│   │   ├── mode_pr_review_prepare.rs # Extend: run early signal pass, generate layer dirs
│   │   ├── mode_pr_review_accept.rs # Extend: validate layer outputs
│   │   ├── mode_pr_review_finalize.rs # Extend: coverage accounting
│   │   └── early_signal.rs          # NEW: early signal check executor, rule definitions, persistence helpers
│   ├── persistence/
│   │   └── layout.rs                # paths: early-signal/, traces/, layers/
│   └── review/
│       └── findings.rs              # unchanged; early signal uses separate domain types
└── canon-adapters/src/
    └── [no changes expected]

tests/
├── contract/
│   └── pr_review_early_signal.rs    # NEW: CLI contract tests
├── integration/
│   └── pr_review_prepare.rs         # Extend: early signal in prepare flow
└── [existing test files]
```

**Structure Decision**: No new crates. Changes are additive within existing `canon-cli` (CLI flags, stdout rendering) and `canon-engine` (domain types, executor, persistence). The `canon-adapters` crate is unchanged — early signal checks use existing `FilesystemAdapter` and `ShellAdapter`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| New `domain/review.rs` separate from existing `review/findings.rs` | Early signal types (`ReviewLayer`, `EarlySignalFinding`, `EarlySignalEvent`) are domain-level abstractions that predate and are consumed by the existing review pipeline. The existing `review/findings.rs` carries reviewer-authored findings (LLM output); early signal findings are Canon-generated deterministic checks. Merging them would couple two distinct provenance paths. | A single unified finding type was rejected because Canon-generated checks and LLM-authored findings have different validation rules, severity systems, and persistence contracts. |
| Persistence in `early_signal.rs` rather than `store.rs` | Early signal persistence is self-contained (write findings.tsv/json/summary.md, append trace.jsonl). Adding it to the central `store.rs` would require widening its API with early-signal-specific knowledge that no other mode uses. | `store.rs` would need early-signal-aware methods (`persist_early_signal_findings`, `append_early_signal_trace`) that are dead weight for every other mode. Keeping persistence local avoids the abstraction cost. |
