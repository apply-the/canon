# Implementation Plan: Brainstorming Ideation Mode

**Branch**: `068-ideation-mode` | **Date**: 2026-06-03 | **Spec**: [spec.md](file:///Users/rt/workspace/apply-the/canon/specs/068-ideation-mode/spec.md)

**Input**: Feature specification from `/specs/068-ideation-mode/spec.md`

## Summary

Implement a new `brainstorming` mode in the `canon-engine` and `canon-cli`. The primary output artifact is an Option Map with trade-offs and spike proposals, serving as a safer, more creative space prior to formal shaping.

## Governance Context

- **Execution Mode**: `brainstorming`
- **Risk Classification**: Green (read-only exploration)
- **Scope Boundaries**: IN-SCOPE: Defining problem framing, options, trade-offs, unknowns, spikes. OUT-OF-SCOPE: Emitting production code, schema, runtime state mutation.
- **Invariants**:
  1. The agent MUST NOT write implementation code.
  2. The agent MUST generate at least 3 distinct conceptual approaches to the problem.
- **Validation Ownership**: Structural/logical by automation/engine; Independent by senior engineer reviewing the output packet.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024

**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `tracing`, `canon-engine`, `canon-cli`

**Storage**: Local `.canon/` and `specs/` directory files

**Testing**: `cargo test`, `assert_cmd` for CLI integration

**Target Platform**: CLI / Local Dev Environment

**Project Type**: CLI tool and library (`canon-cli`, `canon-engine`)

**Performance Goals**: N/A (Standard CLI response times)

**Constraints**: Must integrate smoothly with existing Canon execution modes

**Scale/Scope**: Single feature addition to existing Rust workspace

## Constitution Check

*GATE: Passed. All work respects artifact-driven methods, separation of generation/validation, risk-aware execution, and mode-driven workflows. Rust language rules (no panics, typed serde models) will be followed.*

## Project Structure

### Documentation (this feature)

```text
specs/068-ideation-mode/
├── plan.md              
├── research.md          
├── data-model.md        
├── quickstart.md        
├── contracts/           
└── tasks.md             
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── modes/
│       │   └── brainstorming.rs
│       └── artifacts/
│           └── option_map.rs
├── canon-cli/
│   └── src/
│       └── commands/
│           └── brainstorm.rs
└── tests/
    └── integration/
        └── brainstorm_test.rs
```

**Structure Decision**: Added a new mode `brainstorming.rs` to `canon-engine/src/modes/` and the associated CLI command in `canon-cli`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

(No violations)
