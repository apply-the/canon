# Implementation Plan: Governance Runtime Framing

**Branch**: `040-governance-runtime-framing` | **Date**: 2026-05-03 | **Spec**: [/Users/rt/workspace/apply-the/canon/specs/040-governance-runtime-framing/spec.md](/Users/rt/workspace/apply-the/canon/specs/040-governance-runtime-framing/spec.md)
**Input**: Feature specification from `/specs/040-governance-runtime-framing/spec.md`

## Summary

Reframe Canon's public identity around one explicit promise: it is the governed packet runtime for AI-assisted engineering, not a generic agent framework or opaque orchestration loop. Implement that framing across the README, getting-started flow, mode guide, and one dedicated governance adapter integration guide; align the `0.40.0` release surfaces; and add documentation guardrails so the human CLI story and machine-facing adapter boundary remain synchronized.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact, because the feature changes public product framing and integration-facing guidance while intentionally preserving the current runtime and adapter semantics.  
**Scope In**: README, getting-started guide, mode guide, new governance adapter integration doc, version-bearing repo surfaces, changelog, roadmap, and Rust-based doc guardrail tests.  
**Scope Out**: governance adapter schema changes, new runtime behavior, Synod-specific orchestration mapping in Canon core docs, and any new roadmap beyond the delivered `040` feature.

**Invariants**:

- Canon remains explicitly local-first, file-backed, approval-aware, and governed rather than being described as a generic agent framework.
- The human CLI surface and the governance adapter remain documented as two access paths to the same runtime, not separate products.
- The `canon governance` contract semantics, approval states, and packet lifecycle remain unchanged by this feature.

**Decision Log**: `specs/040-governance-runtime-framing/decision-log.md`  
**Validation Ownership**: Content generation happens in the repo docs and validation artifacts; verification is performed by Rust guardrail tests, repository validation commands, focused coverage checks, and an explicit independent doc readback against the spec.  
**Approval Gates**: No extra runtime approval gate is introduced beyond standard review for bounded-impact repository changes.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown documentation  
**Primary Dependencies**: Existing workspace crates, `assert_cmd`, `predicates`, `serde_json`, `tempfile`, `toml`, and the existing Spec Kit shell scripts  
**Storage**: Repository files and `.canon/` runtime semantics are documented but not structurally changed  
**Testing**: `cargo nextest run`, targeted Rust integration or contract tests, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`  
**Target Platform**: Local CLI usage and repo-based orchestrator integration docs for macOS, Linux, and Windows readers  
**Project Type**: Rust CLI workspace with repository documentation and Rust-based docs guardrail tests  
**Existing System Touchpoints**: `README.md`, `docs/guides/getting-started.md`, `docs/guides/modes.md`, new `docs/integration/governance-adapter.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, and Rust tests under `tests/`  
**Performance Goals**: N/A for runtime behavior; the feature optimizes clarity, contract readability, and drift resistance rather than execution speed  
**Constraints**: Preserve existing adapter semantics, keep the happy path brutally simple for human operators, avoid Synod-specific orchestration logic in Canon core docs, and land the feature as one delivered macrofeature without slice splitting  
**Scale/Scope**: One release-line bump, one new integration guide, several existing docs updates, one roadmap closeout, one changelog entry, and focused Rust guardrail coverage for touched tests or docs checks

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [X] Execution mode is declared and matches the requested work
- [X] Risk classification is explicit and autonomy is appropriate for that risk
- [X] Scope boundaries and exclusions are recorded
- [X] Invariants are explicit before implementation
- [X] Required artifacts and owners are identified
- [X] Decision logging is planned and linked to a durable artifact
- [X] Validation plan separates generation from validation
- [X] Declared-risk approval checkpoints are named where required by the risk classification
- [X] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/040-governance-runtime-framing/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── governance-adapter-doc-surface.md
└── tasks.md
```

### Source Code (repository root)

```text
README.md
CHANGELOG.md
ROADMAP.md
Cargo.toml
Cargo.lock
docs/
├── guides/
│   ├── getting-started.md
│   └── modes.md
└── integration/
    └── governance-adapter.md
tests/
├── contract/
├── integration/
└── *.rs
```

**Structure Decision**: Keep the implementation in the existing Rust CLI workspace and repository-doc layout. All source changes stay in release-facing markdown, workspace version surfaces, and Rust guardrail tests under `tests/` that protect documentation and contract alignment.

## Complexity Tracking

No constitution deviations are expected for this feature.
