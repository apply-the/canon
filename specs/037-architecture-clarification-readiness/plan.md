# Implementation Plan: Architecture Clarification, Assumptions, And Readiness Reroute

**Branch**: `037-architecture-clarification-readiness` | **Date**: 2026-05-02 | **Spec**: `specs/037-architecture-clarification-readiness/spec.md`
**Input**: Feature specification from `/specs/037-architecture-clarification-readiness/spec.md`

## Summary

Deliver `0.37.0` as a bounded architecture-quality slice by tightening
`inspect clarity` for architecture mode into decision-changing questions with
explicit default-if-skipped behavior, materializing working assumptions,
unresolved questions, readiness posture, and recommended next mode in
`readiness-assessment.md`, and aligning architecture templates, skills, docs,
roadmap, changelog, and validation artifacts around one coherent clarification
and reroute contract.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; this work changes architecture
clarity contracts, architecture artifact rendering, shared guidance, version
surfaces, and tests, but it does not introduce a new governed mode, new
persistence subsystem, or new approval lifecycle.  
**Scope In**: architecture-mode clarity question shaping and reroute guidance;
additive architecture readiness contract updates for working assumptions,
unresolved questions, and recommended next mode; bounded updates to shared
clarity helpers, architecture artifact rendering, tests, templates, examples,
skill guidance, README or mode docs, roadmap cleanup, version bump, and
changelog.  
**Scope Out**: live interview orchestration beyond existing Canon surfaces; a
new governed mode; a separate clarification-state store; redesign of C4
artifacts or approval semantics; broad behavior changes across unrelated modes
beyond bounded reuse of shared clarity helpers.

**Invariants**:

- Architecture mode continues to ask only decision-changing questions and must
  not fabricate certainty when the brief is weak or under-bounded.
- Missing authored sections continue to surface as `## Missing Authored Body`
  instead of being rewritten as assumptions.
- Existing run, artifact, approval, and `.canon/` persistence semantics remain
  unchanged.
- Mode reroute guidance remains recommendation-only and points to existing
  modes such as `discovery`, `requirements`, or `system-shaping`.

**Decision Log**: `specs/037-architecture-clarification-readiness/decision-log.md`  
**Validation Ownership**: Generation updates architecture clarity contracts,
artifact rendering, docs, skills, version surfaces, and feature artifacts;
validation is performed through focused Rust contract and run tests,
doc/template alignment checks, coverage review for touched Rust files,
`cargo fmt --check`, `cargo clippy`, `cargo nextest run`, and an independent
review of reroute and readiness semantics.  
**Approval Gates**: No new human approval gate beyond normal review is needed
for bounded-impact work; independent validation evidence remains mandatory.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources, templates, and documentation artifacts  
**Primary Dependencies**: existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), existing `serde`/`serde_json` surfaces, shared architecture skill documents, and existing Spec Kit scripts  
**Storage**: repository files plus the existing `.canon/` runtime artifact layout only; no new persistence family  
**Testing**: `cargo test`, focused architecture clarity and run tests, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo nextest run`  
**Target Platform**: local CLI-driven governed architecture workflows for repository-backed Canon projects  
**Project Type**: Rust CLI workspace with Markdown artifact contracts, templates, and shared AI skill sources  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/service/clarity.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-cli/src/output.rs`, `tests/contract/inspect_clarity.rs`, `tests/contract/architecture_contract.rs`, `tests/architecture_c4_run.rs`, `tests/architecture_decision_shape_docs.rs`, `docs/guides/modes.md`, `docs/templates/canon-input/architecture.md`, `docs/examples/canon-input/architecture-state-management.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, and runtime compatibility references  
**Performance Goals**: keep architecture clarification deterministic and bounded to at most five inspect questions, preserve materially closed decisions without synthetic churn, and avoid any new persistence or orchestration path  
**Constraints**: preserve the existing architecture artifact family and gates except for additive readiness sections; keep recommendation-only posture; keep version alignment at `0.37.0`; provide focused automated validation coverage for touched Rust files and finish with clean `cargo fmt` and `cargo clippy`  
**Scale/Scope**: one architecture clarity and readiness contract expansion, bounded updates to shared docs and skills, one focused Rust test slice, one version bump, and one roadmap cleanup

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
specs/037-architecture-clarification-readiness/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── architecture-clarity.md
│   └── architecture-readiness.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   └── src/
│       └── output.rs
└── canon-engine/
    └── src/
        ├── artifacts/
        │   ├── contract.rs
        │   └── markdown.rs
        └── orchestrator/
            └── service/
                ├── clarity.rs
                ├── inspect.rs
                └── service.rs

tests/
├── architecture_c4_run.rs
├── architecture_decision_shape_docs.rs
└── contract/
    ├── architecture_contract.rs
    └── inspect_clarity.rs

docs/
├── examples/canon-input/architecture-state-management.md
├── guides/modes.md
└── templates/canon-input/architecture.md

defaults/embedded-skills/canon-architecture/skill-source.md
.agents/skills/canon-architecture/SKILL.md
README.md
ROADMAP.md
CHANGELOG.md
Cargo.toml
Cargo.lock
defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml
.agents/skills/canon-shared/references/runtime-compatibility.toml
```

**Structure Decision**: Keep the slice localized to the existing architecture
clarity and artifact pipeline by extending shared clarity parsing,
architecture artifact contracts, tests, and bounded documentation or skill
surfaces instead of introducing a new mode or persistence subsystem.

## Complexity Tracking

No constitution deviations are currently identified.
