# Implementation Plan: Review Mode Completion

**Branch**: `007-review-mode-completion` | **Date**: 2026-04-19 | **Spec**: `specs/007-review-mode-completion/spec.md`
**Input**: Feature specification from `/specs/007-review-mode-completion/spec.md`

## Summary

Promote `review` and `verification` from contract-only taxonomy entries into runnable, evidence-backed Canon modes by extending the existing document-backed analysis pipeline and reusing the `pr-review` disposition and artifact semantics where they fit cleanly. The first implementation slice will establish shared runtime foundations for both modes and deliver full end-to-end `review` support before adding `verification` on the same substrate.

## Governance Context

**Execution Mode**: brownfield  
**Risk Classification**: systemic-impact because the change crosses runtime dispatch, artifact contracts, gate evaluation, evidence lineage, skill truthfulness, user-facing guidance, and the 0.7.0 release surface.  
**Scope In**:
- Engine, CLI, and artifact support needed to run `review` and `verification`
- Evidence, gate, and mode-result plumbing needed to keep `status`, `inspect`, `approve`, and `resume` coherent
- Skill and documentation updates required to stop presenting both modes as support-state-only

**Scope Out**:
- New protocols or remote surfaces
- Mutation-heavy workflow completion for `implementation` or `refactor`
- Operational mode delivery for `incident` or `migration`
- Packaging/distribution work outside release notes and guidance alignment

**Invariants**:

- `review` stays distinct from diff-backed `pr-review`
- `verification` remains a challenge workflow, not a planning or mutation workflow
- All emitted outputs remain durable, Canon-backed, and inspectable through existing surfaces
- No artifact, gate, or summary may claim evidence that Canon cannot persist and name concretely

**Decision Log**: `specs/007-review-mode-completion/decision-log.md`  
**Validation Ownership**: generation and runtime wiring land in engine, adapter, and skill code; validation is split across contract tests, integration tests, repo gate runs, and a separate feature validation report.  
**Approval Gates**: systemic-impact or red-zone work continues to require explicit `Risk` approval; `review` additionally preserves `ReviewDisposition` approval when its packet records unresolved must-fix findings.

## Technical Context

**Language/Version**: Rust 1.94.1  
**Primary Dependencies**: existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`  
**Storage**: local filesystem under `.canon/`, Markdown artifacts, TOML manifests and policies  
**Testing**: `cargo test`, `cargo nextest run`, contract tests under `tests/contract`, integration tests under `tests/integration`  
**Target Platform**: local CLI on macOS, Linux, and Windows  
**Project Type**: Rust workspace CLI and runtime  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/gatekeeper.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-adapters/src/copilot_cli.rs`, `defaults/methods/*.toml`, `.agents/skills/**`, `README.md`, `MODE_GUIDE.md`, `NEXT_FEATURES.md`, `tests/contract/**`, `tests/integration/**`  
**Performance Goals**: keep `run`, `status`, and inspection behavior for the new modes within the same order of cost as current document-backed modes and avoid any new network-visible runtime dependencies  
**Constraints**: local-first only, no fabricated evidence, no protocol-specific subsystem, inspection compatibility must remain intact, release messaging must reflect shipped truth for 0.7.0  
**Scale/Scope**: one Rust workspace with three main crates, existing governed mode infrastructure, and multiple AI-skill materialization surfaces

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
specs/007-review-mode-completion/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── review-verification-run.md
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
│       └── output.rs
└── canon-engine/
    └── src/
        ├── artifacts/
        │   ├── contract.rs
        │   └── markdown.rs
        ├── domain/
        │   ├── gate.rs
        │   └── mode.rs
        ├── modes/
        │   ├── pr_review.rs
        │   ├── review.rs
        │   └── verification.rs
        └── orchestrator/
            ├── gatekeeper.rs
            ├── service.rs
            └── verification_runner.rs

defaults/
├── methods/
│   ├── pr-review.toml
│   ├── review.toml
│   └── verification.toml
└── embedded-skills/
    ├── canon-review/
    └── canon-verification/

.agents/skills/
├── canon-review/
└── canon-verification/

tests/
├── contract/
└── integration/
```

**Structure Decision**: keep the existing Rust workspace and extend the same engine, CLI, methods, and skill surfaces already used by other governed modes. No new top-level subsystem is warranted.

## Complexity Tracking

No constitution deviations are currently required.

