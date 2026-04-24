# Implementation Plan: Backlog Mode (Delivery Decomposition)

**Branch**: `012-backlog-mode` | **Date**: 2026-04-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/012-backlog-mode/spec.md`

**Note**: This plan translates the approved spec into a repository-anchored design for promoting `backlog` into a first-class governed mode that bridges bounded upstream decisions into durable delivery decomposition artifacts without introducing a parallel runtime, identity, or publication model.

## Summary

Add `backlog` by extending the existing Canon runtime surfaces rather than inventing a separate planning subsystem. The design reuses the current CLI `run/status/inspect/list/resume/publish` flow, canonical `canon-input/<mode>.md|/` authored-input binding, immutable input snapshots, artifact contracts, publish routing, and run identity model. The delivery introduces backlog-specific artifact bundles, closure-oriented gate evaluation, source-trace capture for upstream artifacts and authored priorities, and durable planning outputs that later `implementation` work can consume without losing rationale or dependency context. Documentation, defaults, embedded skills, and regression coverage move in lockstep so the runtime truth and product messaging stay aligned.

## Governance Context

**Execution Mode**: `change`
**Risk Classification**: `bounded-impact`. The work changes Canon runtime behavior across CLI, engine, defaults, embedded skills, and tests, but stays inside the existing product boundary, storage layout, publish flow, and mode-driven workflow system.
**Scope In**: Add `backlog` as a first-class mode; define authored-input binding, closure gating, traceability rules, backlog artifact contracts, publish/list/status/inspect compatibility, and supporting docs/defaults/skills/test coverage.
**Scope Out**: Implementation task generation, story points, sprint planning, team-capacity heuristics, tool-specific ticket output, new persistence or identity models, and unrelated mode promotion such as `incident` or `migration`.

**Invariants**:

- Backlog must stop at epics, sub-epics, delivery slices, and story candidates; it must not emit fine-grained implementation task lists.
- Every epic, slice, dependency, and sequencing decision must remain traceable to a bounded source artifact, an explicit authored priority, or a named planning gap.
- If upstream architecture or system shape is not sufficiently closed for credible decomposition, the runtime must block or downgrade the result explicitly instead of emitting a false full packet.
- The current backlog brief remains authoritative; upstream artifacts provide provenance and evidence, not silent overrides.
- Authored inputs under `canon-input/backlog.*` remain read-only source material and are snapshotted immutably into the run.
- Validation remains separated from generation through gate evaluation, contract and integration tests, and independent packet review.
- Canonical run identity, inspect/status/list/resume behavior, and publish routing remain backward-compatible.

**Decision Log**: `specs/012-backlog-mode/decision-log.md`  
**Validation Ownership**: Generation work adds backlog mode contracts, binding, and packet emission. Validation is owned separately by structural checks, contract and integration tests, skill validation, and an independent review of emitted backlog packets recorded in `validation-report.md`.  
**Approval Gates**: No special human gate is added by the plan itself; bounded-impact controls rely on explicit closure checks, preserved scope boundaries, and independent validation before completion. Any broadened scope or weakened traceability must be treated as a plan violation, not silently accepted.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`  
**Storage**: Local filesystem under `.canon/`, TOML manifests and `context.toml`, Markdown artifacts, repo-local skill source documents under `defaults/embedded-skills/` and `.agents/skills/`  
**Testing**: `cargo test`, `cargo nextest run`, contract and integration tests under `tests/`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflows on macOS, Linux, and Windows/PowerShell with local filesystem access  
**Project Type**: Rust CLI plus engine/adapters workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/mode.rs`; `crates/canon-engine/src/orchestrator/{service,publish,gatekeeper,invocation}.rs`; `crates/canon-engine/src/artifacts/{contract,markdown}.rs`; `crates/canon-engine/src/persistence/{manifests,store}.rs`; `crates/canon-cli/src/app.rs`; `defaults/methods/*.toml`; `defaults/policies/*.toml`; `defaults/embedded-skills/`; `.agents/skills/`; `README.md`; `NEXT_FEATURES.md`; test suites under `tests/`  
**Performance Goals**: Preserve current local CLI responsiveness for run creation, status, inspect, and publish; keep closure checks and traceability evaluation linear in authored input and artifact size; add no network dependency or background daemon  
**Constraints**: No new top-level CLI surface; no parallel identity or publish model; no mutation of authored backlog inputs; no fabricated task-level detail; no closure bypass when source artifacts are insufficiently bounded  
**Scale/Scope**: One feature spanning CLI binding, engine mode modeling, artifact contracts, publish defaults, docs, skills, and new contract/integration coverage for a new planning mode

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
specs/012-backlog-mode/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── backlog-authored-input-contract.md
│   └── backlog-packet-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   └── src/
│       ├── app.rs
│       ├── commands/
│       ├── commands.rs
│       ├── error.rs
│       ├── main.rs
│       └── output.rs
├── canon-engine/
│   └── src/
│       ├── artifacts/
│       ├── domain/
│       ├── orchestrator/
│       └── persistence/
└── canon-adapters/
    └── src/
defaults/
├── methods/
├── policies/
└── embedded-skills/
.agents/
└── skills/
tests/
├── contract/
├── integration/
└── fixtures/
README.md
NEXT_FEATURES.md
scripts/
└── validate-canon-skills.sh
```

**Structure Decision**: Keep the existing Rust workspace and Canon runtime layout intact. Add the new planning artifacts under `specs/012-backlog-mode/`, implement backlog runtime changes primarily in `canon-engine` and `canon-cli`, synchronize mode and skill defaults in `defaults/`, and validate the resulting behavior with existing structural checks plus new contract and integration coverage.

## Complexity Tracking

No constitution deviations are planned. The design stays inside the existing run, artifact, and publish surfaces instead of introducing a broader planning subsystem.

## Workstreams

1. **Mode Surface**: Add `backlog` to Canon's mode taxonomy, CLI binding, canonical authored-input auto-binding, and publish/list/status/inspect compatibility surfaces.
2. **Closure & Traceability**: Define how backlog evaluates architecture closure, captures source references and authored priorities, and records planning gaps without silently widening scope.
3. **Artifact Contracts**: Define durable backlog outputs for overview, epic hierarchy, capability mapping, dependencies, slices, sequencing, acceptance anchors, and planning risks.
4. **Docs & Skills**: Update README, mode-facing docs, embedded skills, and materialized skills so backlog is discoverable and honestly described.
5. **Verification & Regression**: Add focused contract and integration checks for authored input discovery, immutable input snapshots, closure gating, artifact emission, traceability, and publish behavior.

## Phase Outcomes

### Phase 0: Research

- Decide how `backlog` should fit into the existing mode taxonomy and publish surface without adding a parallel planning workflow.
- Decide where closure findings, source trace links, and planning risks should live in the persisted runtime model and emitted artifacts.
- Decide the canonical authored-input shape for single-file and folder-backed backlog packets.
- Decide how much downstream handoff context is required so backlog outputs remain useful to `implementation` without becoming task lists.

### Phase 1: Design

- Define concrete data-model additions around backlog run context, authored input lineage, closure findings, and the backlog packet artifact bundle.
- Define user-facing contracts for backlog authored inputs and emitted backlog packets.
- Define a quickstart that exercises both a successful bounded backlog run and a blocked closure-check run.
- Record design decisions and the layered validation plan in durable artifacts.

### Phase 2: Implementation Preparation

- Leave a task-ready design for adding backlog mode across CLI, engine, defaults, docs, skills, and tests.
- Keep post-design validation explicit so later task generation can separate code generation from proof of behavior.
