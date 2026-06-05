# Implementation Plan: Policy Shaping Mode

**Branch**: `070-policy-shaping-mode` | **Date**: 2026-06-04 | **Spec**: [spec.md](file:///Users/rt/workspace/apply-the/canon/specs/070-policy-shaping-mode/spec.md)

**Input**: Feature specification from `/specs/070-policy-shaping-mode/spec.md`

## Summary

Implement the `policy-shaping` governance mode using a hybrid CLI + LLM architecture. The `canon` CLI orchestrates the formal validation pass and requires an explicit `Systemic Impact` approval gate to accept broad impact changes to rules, while relying on `.agents/skills` to semantically assess codebase compliance.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024

**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `tracing`, `canon-engine`, `canon-cli`

**Storage**: Local `.canon/` and `specs/` directory files

**Testing**: `cargo test`, `cargo nextest`, `assert_cmd`

**Target Platform**: macOS, Linux

**Project Type**: CLI Tool (`canon`)

**Performance Goals**: N/A (Performance tradeoffs made for safety and completeness via module grouping and LLM semantic checks).

**Constraints**: Must strictly follow AGENTS.md Rust rules (no panic-prone logic, typed serde models).

**Scale/Scope**: Must be capable of parsing the entire workspace without context overflow by implementing module-level aggregation in the impact report.

## Constitution Check

*GATE: Passed*
- Method over prompting: `policy-shaping` represents a formal new method.
- Artifact-first: Mode outputs `conformance-impact-report.md` and `04-migration.md` as explicit evidence.
- Separation of generation and validation: Impact report is assessed separately from policy drafting.
- Risk-aware: Declared as `Systemic Impact` (Red).
- Mode-driven: Creating a dedicated `policy-shaping` mode.
- Invariants before implementation: Check.
- Layered verification: Handled via the structured output validation strategy.
- Rust rules: Will use explicit error handling and typed `serde` derives.

## Project Structure

### Documentation (this feature)

```text
specs/070-policy-shaping-mode/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   ├── src/
│   │   ├── commands/
│   │   │   └── policy_shaping.rs
│   └── tests/
├── canon-engine/
│   ├── src/
│   │   ├── policy/
│   │   │   ├── evaluator.rs
│   │   │   └── report.rs
│   └── tests/
```

**Structure Decision**: Integrated directly into `canon-cli` and `canon-engine` workspace crates, as policy shaping is a core governance operation.

## Complexity Tracking

> **No violations**
