# Implementation Plan: Mode Context Split

**Branch**: `008-mode-context-split` | **Date**: 2026-04-20 | **Spec**: `specs/008-mode-context-split/spec.md`
**Input**: Feature specification from `/specs/008-mode-context-split/spec.md`

**Note**: This plan covers Phase 0 research and Phase 1 design artifacts for the semantic split between governed work type and system context. Phase 2 implementation will execute the shared runtime refactor first, then public-surface renames, then coverage-driven regression expansion.

## Summary

Replace Canon's overloaded `brownfield-change` mode with a uniform two-axis model: `change` becomes the governed work type, `system_context` becomes an explicit runtime field, and the old preserved-behavior semantics are carried forward only through `change + existing`. The design keeps the runtime minimal by rejecting `change + new`, renames all public input and artifact paths in the same tranche, and treats targeted test expansion plus `cargo llvm-cov` recovery as required feature scope rather than optional follow-up.

## Governance Context

**Execution Mode**: architecture  
**Risk Classification**: systemic-impact because the feature changes the public mode catalog, CLI contract, persisted run context, gate and policy inputs, artifact namespaces, skill truth, documentation, and validation expectations across shipped Canon surfaces.  
**Scope In**:
- Introduce explicit `system_context = new | existing` across runtime request, persisted context, inspect output, policies, gates, and artifact generation
- Replace `brownfield-change` with `change` across CLI parsing, mode registry, methods, artifact paths, canonical input paths, skills, docs, and tests
- Preserve existing bounded-change behavior through `change + existing` and explicitly reject `change + new`
- Add targeted contract and integration validation to recover patch coverage in the touched runtime and CLI surfaces

**Scope Out**:
- Policy-system redesign beyond feeding the new context field into existing evaluation paths
- New governed modes beyond the rename to `change`
- Backward-compatibility aliases for legacy public naming or input paths
- Artifact schema redesign except where rename or explicit context visibility requires contract updates
- Implicit defaults for `system_context`

**Invariants**:

- Mode and system context remain separate axes everywhere Canon parses, persists, explains, or validates a run.
- No public Canon surface ships with `brownfield` or `greenfield` naming after this feature lands.
- `change + existing` preserves the bounded change surface, preserved invariants, validation expectations, and readiness gates previously enforced for `brownfield-change`.
- Required-context modes fail before run creation when `system_context` is missing, and optional-context modes never invent a hidden context value.
- Artifact, evidence, status, approval, and resume flows remain durable and coherent after the rename and context split.

**Decision Log**: `specs/008-mode-context-split/decision-log.md`  
**Validation Ownership**: runtime and documentation changes land in engine, CLI, method, skill, and doc surfaces; validation is split across contract tests, integration tests, skill validators, coverage runs, and a separate feature validation report reviewed independently from implementation.  
**Approval Gates**: systemic-impact work requires a named human owner for merge readiness and independent validation evidence before acceptance; if implementation touches recommendation-only mutation paths, those remain non-executing without explicit owner approval.

## Implementation Checkpoints

- **Reconfirmed On**: 2026-04-20 during `/speckit.implement`
- **Execution Mode Check**: remains `architecture` for the feature packet while implementation proceeds under the approved systemic-impact scope
- **Risk Check**: remains `systemic-impact`; no scope reduction removes the need for explicit human merge ownership and independent validation evidence
- **Scope Boundary Check**: implementation remains limited to mode/context semantics, persistence, public surface renames, and validation depth recovery
- **Invariant Check**: mode and `system_context` stay separate axes; no public `brownfield` or `greenfield` naming survives acceptance; `change + existing` preserves bounded-change semantics; optional modes persist no invented context

## Independent Review Checkpoints

- **Checkpoint A**: after foundational runtime plumbing lands, review `Mode`, `RunRequest`, `RunContext`, and CLI parsing for semantic separation and absence of hidden defaults
- **Checkpoint B**: after US1 and US2 land, review `change + existing`, rejected `change + new`, and required-versus-optional context enforcement before public-surface cleanup is accepted
- **Checkpoint C**: before merge, review documentation truth, skill validator truth, coverage evidence, and modified-file inventory against `validation-report.md`
- **Merge Ownership Expectation**: a named human owner must confirm systemic-impact readiness before merge; implementation may prepare the evidence but does not self-approve it

## Technical Context

**Language/Version**: Rust 1.95.0  
**Primary Dependencies**: existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`  
**Storage**: local filesystem under `.canon/`, TOML manifests and `context.toml`, Markdown artifacts, repo-local skill source documents under `defaults/` and `.agents/skills/`  
**Testing**: `cargo test`, `cargo nextest run`, targeted contract tests under `tests/contract`, targeted integration tests under `tests/integration`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`  
**Target Platform**: local CLI on macOS, Linux, and Windows  
**Project Type**: Rust workspace CLI and runtime  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/orchestrator/classifier.rs`, `crates/canon-engine/src/orchestrator/gatekeeper.rs`, `crates/canon-engine/src/orchestrator/invocation.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/persistence/layout.rs`, `crates/canon-engine/src/persistence/manifests.rs`, `crates/canon-engine/src/persistence/store.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/modes/brownfield_change.rs`, `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands/run.rs`, `crates/canon-cli/src/output.rs`, `crates/canon-adapters/src/copilot_cli.rs`, `defaults/methods/*.toml`, `defaults/embedded-skills/**`, `.agents/skills/**`, `README.md`, `MODE_GUIDE.md`, `NEXT_FEATURES.md`, `tests/contract/**`, `tests/integration/**`  
**Performance Goals**: keep `run`, `status`, and inspect flows within the current local CLI order of cost, add no network dependency, and keep coverage additions focused on high-signal runtime branches rather than broad slow end-to-end duplication  
**Constraints**: breaking changes are allowed, no legacy public naming remains, no implicit defaults for `system_context`, `change + new` is rejected in the first slice, docs and validators must ship with runtime truth, and the touched patch must recover to at least 85% line coverage  
**Scale/Scope**: one Rust workspace with three crates, mode registry and method-policy surfaces, local `.canon/` persistence model, materialized skill surfaces, and contract plus integration regression suites

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
specs/008-mode-context-split/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── mode-context-run-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-adapters/
│   └── src/
│       └── copilot_cli.rs
├── canon-cli/
│   └── src/
│       ├── app.rs
│       ├── commands/
│       │   └── run.rs
│       └── output.rs
└── canon-engine/
    └── src/
        ├── artifacts/
        │   ├── contract.rs
        │   └── markdown.rs
        ├── domain/
        │   ├── mode.rs
        │   └── run.rs
        ├── modes/
        │   └── brownfield_change.rs
        ├── orchestrator/
        │   ├── classifier.rs
        │   ├── gatekeeper.rs
        │   ├── invocation.rs
        │   └── service.rs
        └── persistence/
            ├── layout.rs
            ├── manifests.rs
            └── store.rs

defaults/
├── methods/
│   ├── architecture.toml
│   ├── brownfield-change.toml
│   ├── implementation.toml
│   ├── incident.toml
│   ├── migration.toml
│   ├── refactor.toml
│   └── system-shaping.toml
└── embedded-skills/
    ├── canon-brownfield/
    ├── canon-shared/
    └── canon-system-shaping/

.agents/skills/
├── canon-brownfield/
└── canon-shared/

tests/
├── contract/
│   ├── architecture_contract.rs
│   ├── brownfield_contract.rs
│   ├── brownfield_invocation_contract.rs
│   ├── cli_contract.rs
│   ├── inspect_modes.rs
│   ├── invocation_cli_contract.rs
│   ├── runtime_evidence_contract.rs
│   └── system_shaping_contract.rs
└── integration/
    ├── brownfield_governed_execution.rs
    ├── brownfield_run.rs
    ├── system_shaping_run.rs
    └── verification_run.rs
```

**Structure Decision**: keep the existing Rust workspace and refactor the current mode, persistence, method, skill, and documentation surfaces in place. The design deliberately renames brownfield-specific public files and tests to change-centric equivalents rather than introducing a second compatibility layer.

## Complexity Tracking

No constitution deviations are currently required.
