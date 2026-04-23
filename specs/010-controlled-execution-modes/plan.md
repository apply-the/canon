# Implementation Plan: Controlled Execution Modes (`implementation` and `refactor`)

**Branch**: `010-controlled-execution-modes` | **Date**: 2026-04-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/010-controlled-execution-modes/spec.md`

**Note**: This plan translates the approved spec into a repository-anchored design for promoting `implementation` and `refactor` from staged mode profiles to governed runtime modes without introducing a parallel identity, publish, or storage model.

## Summary

Promote `implementation` and `refactor` by extending existing Canon runtime primitives instead of creating a second execution system. The plan specializes artifact contracts in `crates/canon-engine/src/artifacts/contract.rs`, adds dedicated gatekeeper evaluation and mode step sequences, extends canonical `canon-input/<mode>.md|/` binding and immutable input snapshotting, persists provenance-only `upstream_context` metadata from folder-backed carry-forward packets, reuses `InvocationConstraintSet.allowed_paths` plus `recommendation_only` for bounded execution posture, and keeps run identity, `context.toml`, publish routing, inspect surfaces, and CLI entrypoints unchanged. The delivery also updates embedded skills, mode guide/docs, and test suites so the product description matches runtime truth.

## Governance Context

**Execution Mode**: `change`
**Risk Classification**: `bounded-impact`. The work changes runtime behavior across CLI, engine, defaults, embedded skills, and tests, but it remains bounded to the existing Canon workspace, storage layout, and publish surfaces.
**Scope In**: Promote `implementation` and `refactor` to governed runtime modes; add mode-specific artifact contracts, gating, authored-input binding, execution posture handling, docs/skills updates, and regression coverage.
**Scope Out**: New top-level CLI commands, parallel run identity or publish models, red-zone/systemic mutating execution, promotion of `incident` or `migration`, and any workflow that writes back into `canon-input/`.

**Invariants**:

- Execution-heavy modes must continue to route through the gatekeeper before consequential mutation is allowed.
- `implementation` must require explicit mutation bounds and plan/task linkage before execution is recommended.
- `refactor` must require declared preserved behavior, structural rationale, and a no-feature-addition posture.
- Authored inputs under `canon-input/` remain read-only source material and are snapshotted immutably into the run.
- Recommendation-only behavior must reuse the existing invocation constraint and outcome model, not a new run-state taxonomy.
- Canonical run identity, lookup, list, inspect, resume, and publish behavior must remain backward-compatible.
- Validation remains separate from generation through tool validation, gate evaluation, and independent review artifacts.
- Persisted runtime artifacts remain under `.canon/runs/<RUN_ID>/` and published artifacts remain under the existing mode-specific default destinations.

**Decision Log**: `specs/010-controlled-execution-modes/decision-log.md`  
**Validation Ownership**: Generation work updates mode contracts, gating, input binding, and artifact emission. Validation is owned separately by gatekeeper evaluation, contract/integration tests, skill validation scripts, and an independent review pass recorded in `validation-report.md`.  
**Approval Gates**: Green/yellow bounded-impact runs may proceed within declared mutation bounds; broadened mutation scope or gate exceptions require explicit approval; red-zone or systemic-impact work remains recommendation-only in this feature slice.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`  
**Storage**: Local filesystem under `.canon/`, TOML manifests and `context.toml`, Markdown artifacts, repo-local skill source documents under `defaults/embedded-skills/` and `.agents/skills/`  
**Testing**: `cargo test`, `cargo nextest run`, contract and integration suites under `tests/`, `scripts/validate-canon-skills.sh`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`  
**Target Platform**: Cross-platform local CLI workflows on developer workstations with local filesystem access (macOS, Linux, Windows/PowerShell)  
**Project Type**: Rust CLI plus engine/adapters workspace  
**Existing System Touchpoints**: `crates/canon-cli/src/app.rs`; `crates/canon-engine/src/domain/{mode,run,execution}.rs`; `crates/canon-engine/src/orchestrator/{classifier,gatekeeper,invocation,publish,service}.rs`; `crates/canon-engine/src/artifacts/{contract,markdown}.rs`; `crates/canon-engine/src/persistence/{manifests,store}.rs`; `defaults/methods/*.toml`; `defaults/policies/*.toml`; `defaults/embedded-skills/canon-{implementation,refactor}/skill-source.md`; `MODE_GUIDE.md`; `tests/integration/mode_profiles.rs`; `tests/direct_runtime_coverage.rs`; new implementation/refactor-specific tests  
**Performance Goals**: Preserve current local CLI responsiveness for `run`, `status`, `inspect`, and `publish`; keep gating and invocation constraint evaluation linear in the size of the emitted artifact bundle and declared path set; add no network dependency or background daemon  
**Constraints**: No new top-level CLI surface for basic lifecycle visibility; no parallel identity or publish model; preserve immutable input snapshots and current `.canon/runs/<RUN_ID>/` layout; reuse `recommendation_only` posture; keep red/systemic mutation recommendation-only; maintain truthful docs and skill messaging throughout  
**Scale/Scope**: One feature spanning three Rust crates, defaults/policies, embedded skills, user-facing docs, and multiple new contract/integration test suites for two execution-heavy modes

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
specs/010-controlled-execution-modes/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── implementation-execution-contract.md
│   └── refactor-execution-contract.md
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
│       ├── modes/
│       ├── orchestrator/
│       ├── persistence/
│       └── review/
└── canon-adapters/
│   └── src/
│       ├── capability.rs
│       ├── copilot_cli.rs
│       ├── dispatcher.rs
│       ├── filesystem.rs
│       ├── mcp_stdio.rs
│       └── shell.rs
defaults/
├── methods/
├── policies/
└── embedded-skills/
.agents/
└── skills/
tests/
├── contract/
├── integration/
├── direct_runtime_coverage.rs
└── mode_profiles.rs
MODE_GUIDE.md
README.md
scripts/
└── validate-canon-skills.sh
```

**Structure Decision**: Keep the existing Rust workspace and Canon runtime layout intact. Add plan artifacts under `specs/010-controlled-execution-modes/`, implement runtime changes in `canon-engine` and `canon-cli`, synchronize mode definitions in `defaults/`, and update both embedded-skill sources and materialized `.agents/skills/` outputs through the existing agent-context and skill materialization flow.

## Complexity Tracking

No constitution deviations are planned. The design keeps the work inside the existing runtime, identity, and publish surfaces instead of introducing a broader execution framework.

## Workstreams

1. **Mode Contracts**: Replace the generic fallback artifact contract for `implementation` and `refactor` with explicit bundles that match the spec and gate profiles.
2. **Execution Controls**: Add machine-checkable mutation-bounds, safety-net metadata, and provenance-only upstream lineage to persisted run context and enforce bounded execution through invocation constraints and dedicated gatekeeper evaluators.
3. **Input Binding & Skills**: Wire `canon-input/implementation.*` and `canon-input/refactor.*` into CLI/runtime binding, immutable snapshots, folder-backed carry-forward packet guidance, MODE_GUIDE, embedded skills, and materialized skill files.
4. **Recommendation-Only Posture**: Reuse `recommendation_only` and `ToolOutcomeKind::RecommendationOnly` for high-risk or insufficient-evidence flows while leaving `RunState` and publish surfaces intact.
5. **Verification & Non-Regression**: Add focused contract and integration suites for both modes and update staged-mode assumptions in existing tests.

## Phase Outcomes

### Phase 0: Research

- Resolve the current runtime gaps: catch-all artifact contracts, missing gatekeeper evaluators, missing canonical input binding for `implementation` and `refactor`, and staged-depth assertions in tests.
- Decide where machine-checkable mutation bounds, preservation targets, and safety-net evidence live in the persisted runtime model.
- Decide how recommendation-only posture is surfaced without adding a new run-state family.

### Phase 1: Design

- Define concrete data model additions around `RunContext`, invocation constraints, artifact bundles, and execution posture.
- Define user-facing contracts for `implementation` and `refactor` CLI behavior, emitted artifacts, and publish/list/status/inspect compatibility.
- Update planning artifacts so downstream tasks can be generated without reopening design questions.

### Phase 2: Implementation Preparation

- Leave a task-ready plan for runtime code changes across CLI, engine, defaults, docs, skills, and tests.
- Keep post-design validation explicit so task generation can separate code generation from proof of behavior.

