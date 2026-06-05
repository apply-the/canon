# Implementation Plan: Observability Design Mode

**Branch**: `071-observability-design` | **Date**: 2026-06-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/071-observability-design/spec.md`

## Summary

Implement the `observability-design` mode within Canon. The mode operates as a read-only workflow (Green risk) that parses system architecture documents using a reasoning-heavy LLM pass to infer critical system boundaries, and outputs standardized telemetry contracts, SLI/SLO alerts, and an instrumentation checklist. 

## Governance Context

- **Execution Mode**: `observability-design`
- **Risk**: Green (advisory design artifacts, no runtime mutation)
- **Scope Boundaries**: Telemetry contracts, SLI/SLO definitions, Runbook stubs generation. Out of scope: actual runtime instrumentation or external API integrations (Datadog/Prometheus).
- **Invariants**: 
  - Generated artifacts must be read-only advisory design artifacts.
  - Linked implementation runs cannot close unless the instrumentation checklist items are verifiably present in the code.
- **Validation Ownership**: Generated checklists serve as validation gates for downstream implementation modes. Structural validation requires the presence of four distinct artifact files.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024

**Primary Dependencies**: `clap`, `serde`, `serde_json`, `tracing`, `canon-engine`, `canon-cli`

**Storage**: Local `.canon/` and `specs/` directory files

**Testing**: `cargo test` and `cargo nextest run`

**Target Platform**: CLI environments (Linux, macOS, Windows)

**Project Type**: CLI mode addition

**Performance Goals**: N/A (interactive LLM workflow)

**Constraints**: Must run as a non-mutating workflow, generating standard markdown outputs.

**Scale/Scope**: Impacts the `canon-engine` orchestrator routing and `canon-cli` command surface.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Method over prompting: Explicit `observability-design` mode defined.
- [x] Artifact-first engineering: Mode produces `telemetry-plan.md`, `03-slo-alerts.md`, `04-runbook.md`, and `05-instrumentation-checklist.md`.
- [x] Separation of generation and validation: Observability checklists are validated downstream by implementation tasks.
- [x] Risk-aware execution: Explicitly marked as Green risk.
- [x] Decision traceability: Required decisions logged in `decision-log.md`.
- [x] Invariants before implementation: Defined read-only invariant and downstream check.
- [x] Layered verification: Structural output check and downstream validation.

## Project Structure

### Documentation (this feature)

```text
specs/071-observability-design/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── cli.md           # CLI interface contract
├── decision-log.md      # Decisions
└── validation-report.md # Validation plan
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   ├── src/commands/observability_design.rs
│   └── tests/integration/test_observability_design.rs
└── canon-engine/
    ├── src/observability/
    │   ├── mod.rs
    │   ├── evaluator.rs
    │   └── generators.rs
    └── tests/unit/test_observability_design.rs
```

**Structure Decision**: Added new CLI command module `observability_design.rs` and matching core engine orchestration logic under `canon-engine/src/observability/`.
