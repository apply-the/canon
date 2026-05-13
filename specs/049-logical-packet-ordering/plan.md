# Implementation Plan: Logical Packet Ordering

**Branch**: `049-logical-packet-ordering` | **Date**: 2026-05-13 | **Spec**: `specs/049-logical-packet-ordering/spec.md`
**Input**: Feature specification from `specs/049-logical-packet-ordering/spec.md`

## Summary

Establish a Canon-owned logical packet-ordering contract for new packets. The feature adds explicit numeric ordering for reader-facing artifacts, packet metadata fields that declare the primary artifact and ordered artifact list, publish and summary behavior that preserves that order, backward-compatible handling for legacy packet names, and updated mode documentation that explains the distinction between `domain-language` and `domain-model`.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; the feature touches packet contracts, metadata, publish behavior, status and inspect presentation, and mode docs across the active mode catalog, but it preserves governance semantics and historical run readability.  
**Scope In**: ordered artifact naming for new packets, primary-artifact and artifact-order metadata, publish-order preservation, status and inspect packet summaries, backward-compatibility aliases or equivalent legacy handling, per-mode ordered artifact docs, and `domain-language` versus `domain-model` clarification.  
**Scope Out**: project-memory promotion policy, Boundline integration, safety-net mode, workspace mutation beyond packet surfaces, automatic migration of historical runs, and governance model changes.

**Invariants**:

- Existing governed runs remain readable without rewrite; compatibility behavior absorbs legacy packet names for historical runs.
- Packet readability reflects Canon-owned reading order instead of alphabetical sorting or incidental emission order.
- Sidecars remain outside the ordered packet body unless Canon explicitly declares them reader-facing artifacts.

**Decision Log**: `specs/049-logical-packet-ordering/decision-log.md`  
**Validation Ownership**: implementation work generates code and artifacts; focused tests, coverage, and human review validate ordering, compatibility, and documentation correctness.  
**Approval Gates**: standard PR review is required; no extra approval gates beyond bounded-impact closeout checks.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; `serde`, `serde_json`, `toml`, `thiserror`, `tracing`, `uuid`, `time`  
**Storage**: local filesystem under `.canon/` plus repo-visible published packet directories and docs  
**Testing**: `cargo test`, `cargo nextest run`, `cargo llvm-cov`, targeted integration and contract tests  
**Target Platform**: macOS, Linux, Windows  
**Project Type**: CLI + library workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-engine/src/persistence/store.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-cli/src/output.rs`, packet tests under `tests/`, and mode docs under `docs/guides/`  
**Performance Goals**: no material regression in packet generation or publish responsiveness; ordering metadata must remain deterministic  
**Constraints**: new packets use contiguous numeric ordering, sidecars stay distinct from packet-body artifacts, old packets must remain readable, and the feature must close with >=95% touched-file coverage plus clean clippy and fmt  
**Scale/Scope**: current packet-emitting mode catalog plus future-mode ordering rules; multi-file contract update across engine, CLI-adjacent summaries, tests, and docs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/049-logical-packet-ordering/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── artifacts/
│       │   ├── contract.rs
│       │   └── markdown.rs
│       ├── domain/
│       │   ├── artifact.rs
│       │   └── mode.rs
│       └── orchestrator/
│           ├── publish.rs
│           ├── verification_runner.rs
│           └── service/
│               ├── mode_shaping.rs
│               └── summarizers.rs
├── canon-cli/
│   └── src/
└── canon-adapters/

docs/
└── guides/
    └── modes.md

tests/
├── contract/
├── integration/
├── architecture_c4_run.rs
└── requirements_authoring_run.rs
```

**Structure Decision**: Reuse the existing Rust workspace structure. The core ordering logic lives in `canon-engine`, with documentation and cross-mode behavior validated through integration and contract tests under `tests/`.

## Complexity Tracking

No constitution violations identified.
