# Implementation Plan: Agent-Governed Onion-Layer PR Review

**Branch**: `074-pr-review-onion-orchestration` | **Date**: 2026-06-08 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/074-pr-review-onion-orchestration/spec.md`

## Summary

Refactor Canon `pr-review` into an agent-governed, onion-layer review orchestration workflow. Canon prepares layered context, orchestrates five progressive LLM review steps (diff → whole-file → related-context → logical-stress → tests), validates structured reviewer output, and renders actionable artifacts. The LLM performs semantic reasoning; Canon structures the workflow, validates outputs, and preserves traceability. File-based handoff with compact TSV context indexes ensures token-efficient progressive discovery. Deterministic stub adapters and fixture outputs enable full CI testing without live LLM calls.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024

**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `tracing`, `canon-engine`, `canon-cli`, `canon-adapters`

**Storage**: Local `.canon/runs/<run-id>/pr-review/` — Markdown instructions, TSV context indexes, JSON validated outputs, patch files, LLM-authored `output.md`

**Testing**: `cargo test`, `cargo nextest run`, deterministic stub adapters and fixture files

**Target Platform**: Linux/macOS CLI

**Project Type**: CLI application with orchestration engine

**Performance Goals**: N/A (review orchestration is latency-tolerant batch workload)

**Constraints**: Review context must fit within LLM context window (target: compact TSV indexes over duplicated payloads). >20 changed files or >500 lines triggers explicit sampling.

**Scale/Scope**: Single repository PR diff (base..head). 5 review layers per run.

## Constitution Check

*GATE: Passed.*

| Principle | Status | Evidence |
|---|---|---|
| I. Method over prompting | ✅ | Onion-layer workflow is an explicit 5-step method with ordered phases (prepare → layer steps → accept → finalize) |
| II. Artifact-first | ✅ | Every layer emits `output.md`, context indexes, coverage records, and final artifacts |
| III. Separation of generation and validation | ✅ | LLM generates `output.md` (layer + aggregate), Canon validates and compiles into `canonical-review-output.json` |
| IV. Risk-aware execution | ✅ | Risk classification: Systemic-Impact for pr-review behavior; deterministic validation gates at accept phase |
| V. Mode-driven workflows | ✅ | Mode is `pr-review` with sub-commands `prepare`, `accept`, `finalize` |
| VI. Decision traceability | ✅ | Layer skip/failure records, context IDs for evidence, run-state.json |
| VII. Invariants before implementation | ✅ | Invariants: layer completion rules, finalize blocking, file-based handoff, empty review must be explained |
| VIII. Bounded context awareness | ✅ | Context indexes bound review scope; progressive discovery avoids unbounded context |

## Project Structure

### Documentation (this feature)

```text
specs/074-pr-review-onion-orchestration/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
├── tasks.md             # Phase 2 output (speckit.tasks)
└── checklists/
    └── requirements.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       └── orchestrator/
│           └── service/
│               ├── mode_pr_review.rs        # Refactored: prepare/accept/finalize phases
│               ├── mode_pr_review_prepare.rs # NEW: prepare phase orchestration
│               ├── mode_pr_review_accept.rs  # NEW: accept phase with validation
│               └── mode_pr_review_finalize.rs# NEW: finalize phase with rendering
│
└── canon-cli/
    └── src/
        └── commands/
            └── pr_review.rs                 # Extended: prepare/accept/finalize sub-commands
```

```text
crates/
├── canon-engine/
│   └── src/
│       └── review/
│           ├── onion.rs         # NEW: onion-layer model, layer state machine
│           ├── context.rs       # NEW: context index builder, TSV/JSON serialization
│           ├── validate.rs      # NEW: reviewer output validation (schema, paths, IDs, severities)
│           ├── render.rs        # NEW: Markdown artifact renderers (summary, comments, report)
│           └── findings.rs      # Extended: CanonicalCommentSet, ReviewFindingEntry
│
└── canon-adapters/
    └── src/
        └── reviewer_stub.rs     # Extended: per-layer stub findings for testing
```

```text
tests/
├── contract/
│   └── pr_review_onion_contract.rs  # NEW: contract tests for context index, layer output, validation
├── integration/
│   └── pr_review_onion_run.rs       # NEW: full prepare→accept→finalize integration tests
└── unit/
    └── pr_review_onion_unit.rs      # NEW: unit tests for state machine, validation, rendering
```

## Complexity Tracking

No violations. All principles pass. The onion-layer model is inherently complex (5 layers × 14 states) but decomposed into discrete, testable phases.
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
