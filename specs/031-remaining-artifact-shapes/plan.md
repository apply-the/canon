# Implementation Plan: Remaining Industry-Standard Artifact Shapes

**Branch**: `031-remaining-artifact-shapes` | **Date**: 2026-05-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/031-remaining-artifact-shapes/spec.md`

## Summary

Complete the remaining artifact-shapes rollout by extending persona-aware,
industry-standard packet guidance to `implementation`, `refactor`, and
`verification`, preserving those authored sections through the existing
markdown renderer and artifact-contract surfaces, updating docs and release
alignment for `0.31.0`, and proving the slice with focused contract, renderer,
run, coverage, `cargo clippy`, and `cargo fmt` validation.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature changes existing authoring, rendering, docs, tests, and release surfaces across three already-modeled modes without introducing a new runtime domain, storage model, or approval flow  
**Scope In**: 031 planning artifacts; remaining shape and persona guidance for `implementation`, `refactor`, and `verification`; mirrored skill updates; shared renderer and contract validation; roadmap and operator-doc alignment; `0.31.0` version alignment; focused coverage for modified or new Rust files; and final `cargo clippy` plus `cargo fmt` closeout  
**Scope Out**: new Canon modes; approval or evidence semantic changes; `.canon/` schema changes; publish-layer changes; distribution or packaging work beyond release-surface alignment; and unrelated roadmap items  

**Invariants**:

- Persona guidance MUST remain subordinate to Canon's explicit artifact contracts, missing-authored-body markers, evidence posture, and approval semantics.
- Existing `.canon/` storage, canonical `run_id` identity, publish destinations, and approval semantics MUST remain unchanged.
- Modes outside `implementation`, `refactor`, and `verification` MUST keep their current observable behavior unless a later scoped feature expands coverage.

**Decision Log**: `specs/031-remaining-artifact-shapes/decision-log.md`  
**Validation Ownership**: Generation happens through planning artifacts, skill source plus mirrored-skill updates, shared renderer or contract changes, and focused tests on the implementation branch; validation happens through focused automated tests, skill-sync validation, coverage review for touched Rust files, and a separate final diff and artifact review before merge.  
**Approval Gates**: Standard maintainer review plus repository quality gates (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, focused story-level regression coverage, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, and full-workspace regression as needed); no new runtime approval gate is introduced by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run --workspace --all-features`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused mode-specific contract and renderer and run tests, doc regressions, and `scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows with repo-local AI skill materialization  
**Project Type**: Rust CLI workspace with embedded skill sources, mirrored AI-facing skills, contract-based markdown rendering, and repository documentation artifacts  
**Existing System Touchpoints**: `defaults/embedded-skills/canon-implementation/skill-source.md`, `defaults/embedded-skills/canon-refactor/skill-source.md`, `defaults/embedded-skills/canon-verification/skill-source.md`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `.agents/skills/canon-implementation/SKILL.md`, `.agents/skills/canon-refactor/SKILL.md`, `.agents/skills/canon-verification/SKILL.md`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `tests/implementation_authoring_docs.rs`, `tests/implementation_authoring_renderer.rs`, `tests/implementation_contract.rs`, `tests/implementation_run.rs`, `tests/refactor_authoring_docs.rs`, `tests/refactor_authoring_renderer.rs`, `tests/refactor_contract.rs`, `tests/refactor_run.rs`, `tests/refactor_preservation_run.rs`, `tests/verification_authoring_docs.rs`, `tests/verification_authoring_renderer.rs`, `tests/verification_contract.rs`, `tests/verification_run.rs`, `tests/skills_bootstrap.rs`, `docs/templates/canon-input/implementation.md`, `docs/templates/canon-input/refactor.md`, `docs/templates/canon-input/verification.md`, `docs/examples/canon-input/implementation-auth-session-revocation.md`, `docs/examples/canon-input/refactor-auth-session-cleanup.md`, `docs/examples/canon-input/verification-e2e-flakiness.md`, `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, `CHANGELOG.md`, `Cargo.toml`, and `Cargo.lock`  
**Performance Goals**: Preserve current authoring and packet-emission responsiveness with no extra runtime round-trip, no new persistence surface, and no manual synchronization step beyond the existing skill-source mirror workflow  
**Constraints**: Keep `.canon/` layout unchanged; preserve exact canonical H2 section contracts and missing-body honesty; keep non-targeted modes behaviorally stable; keep version surfaces aligned to `0.31.0`; and provide focused coverage for every modified or new Rust file  
**Scale/Scope**: One remaining rollout slice spanning three mode-specific skill pairs, shared artifact rendering and contract rules, release-facing docs, mirrored runtime compatibility references, and focused regression coverage across the existing workspace

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
specs/031-remaining-artifact-shapes/
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
defaults/embedded-skills/
├── canon-implementation/skill-source.md
├── canon-refactor/skill-source.md
├── canon-verification/skill-source.md
└── canon-shared/references/runtime-compatibility.toml

.agents/skills/
├── canon-implementation/SKILL.md
├── canon-refactor/SKILL.md
├── canon-verification/SKILL.md
└── canon-shared/references/runtime-compatibility.toml

crates/canon-engine/src/
└── artifacts/
    ├── contract.rs
    └── markdown.rs

docs/
├── guides/
│   └── modes.md
├── templates/
│   └── canon-input/
│       ├── implementation.md
│       ├── refactor.md
│       └── verification.md
└── examples/
    └── canon-input/
        ├── implementation-auth-session-revocation.md
        ├── refactor-auth-session-cleanup.md
        └── verification-e2e-flakiness.md

tests/
├── implementation_authoring_docs.rs
├── implementation_authoring_renderer.rs
├── implementation_contract.rs
├── implementation_run.rs
├── refactor_authoring_docs.rs
├── refactor_authoring_renderer.rs
├── refactor_contract.rs
├── refactor_run.rs
├── refactor_preservation_run.rs
├── verification_authoring_docs.rs
├── verification_authoring_renderer.rs
├── verification_contract.rs
├── verification_run.rs
└── skills_bootstrap.rs
```

**Structure Decision**: Keep the existing Rust workspace, embedded-skill, mirrored-skill, and documentation layout. Implement the feature by updating the three target skill source plus mirror pairs, reusing the existing markdown renderer and artifact-contract surfaces, and extending focused mode-specific tests and release surfaces rather than adding a new crate or persistence layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.
