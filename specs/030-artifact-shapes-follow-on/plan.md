# Implementation Plan: Industry-Standard Artifact Shapes Follow-On

**Branch**: `030-artifact-shapes-follow-on` | **Date**: 2026-05-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/030-artifact-shapes-follow-on/spec.md`

## Summary

Deliver the follow-on artifact-shapes slice by extending persona-aware,
industry-standard packet guidance to `discovery`, `system-shaping`, and
`review`, preserving those authored sections through the existing markdown
renderer and contract surfaces, updating docs and release alignment for
`0.30.0`, and proving the slice with focused renderer, docs, run, coverage,
`cargo clippy`, and `cargo fmt` validation.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature changes existing
authoring, rendering, docs, tests, and release surfaces across three
already-modeled modes without introducing a new runtime domain, storage model,
or approval flow  
**Scope In**: 030 planning artifacts; follow-on shape and persona guidance for
`discovery`, `system-shaping`, and `review`; mirrored skill updates; shared
renderer and contract validation; roadmap and operator-doc alignment;
`0.30.0` version alignment; focused coverage for modified or new Rust files;
and final `cargo clippy` plus `cargo fmt` closeout  
**Scope Out**: new Canon modes; full remaining artifact-shapes rollout;
approval or evidence semantic changes; `.canon/` schema changes; publish-layer
changes beyond release-surface alignment; and unrelated packaging work

**Invariants**:

- Persona guidance MUST remain subordinate to Canon's explicit artifact
  contracts, missing-authored-body markers, evidence posture, and approval
  semantics.
- Existing `.canon/` storage, canonical `run_id` identity, and the structured
  external publish contract from 029 MUST remain unchanged.
- Modes outside `discovery`, `system-shaping`, and `review` MUST keep their
  current observable behavior unless a later scoped feature expands coverage.

**Decision Log**: `specs/030-artifact-shapes-follow-on/decision-log.md`  
**Validation Ownership**: Generation happens through planning artifacts, skill
source plus mirrored-skill updates, shared renderer and doc changes, and
focused tests on the implementation branch; validation happens through focused
automated tests, skill-sync validation, coverage review for touched Rust files,
and a separate final diff and artifact review before merge.  
**Approval Gates**: Standard maintainer review plus repository quality gates
(`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, focused story-level regression coverage, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, and full-workspace regression as needed); no new runtime approval gate is introduced by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run --workspace --all-features`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused mode-specific integration and renderer tests, release-surface checks, and `scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows with repo-local AI skill materialization  
**Project Type**: Rust CLI workspace with embedded skill sources, mirrored AI-facing skills, contract-based markdown rendering, and repository documentation artifacts  
**Existing System Touchpoints**: `defaults/embedded-skills/canon-discovery/skill-source.md`, `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-review/skill-source.md`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `.agents/skills/canon-discovery/SKILL.md`, `.agents/skills/canon-system-shaping/SKILL.md`, `.agents/skills/canon-review/SKILL.md`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `crates/canon-engine/src/artifacts/markdown.rs`, `tests/discovery_authoring_docs.rs`, `tests/discovery_authoring_renderer.rs`, `tests/discovery_authoring_run.rs`, `tests/system_shaping_domain_modeling_docs.rs`, `tests/system_shaping_authoring_renderer.rs`, `tests/system_shaping_run.rs`, `tests/review_authoring_docs.rs`, `tests/review_authoring_renderer.rs`, `tests/review_run.rs`, `tests/skills_bootstrap.rs`, `ROADMAP.md`, `README.md`, `docs/guides/modes.md`, `CHANGELOG.md`, `Cargo.toml`, and `Cargo.lock`  
**Performance Goals**: Preserve current authoring and packet-emission responsiveness with no extra runtime round-trip, no new persistence surface, and no manual synchronization step beyond the existing skill-source mirror workflow  
**Constraints**: Keep `.canon/` layout unchanged; preserve exact canonical H2 section contracts and missing-body honesty; keep non-targeted modes behaviorally stable; keep version surfaces aligned to `0.30.0`; and provide focused coverage for every modified or new Rust file  
**Scale/Scope**: One follow-on slice spanning three mode-specific skill pairs, shared artifact rendering rules, release-facing docs, mirrored runtime compatibility references, and focused regression coverage across the existing workspace

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
specs/030-artifact-shapes-follow-on/
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
├── canon-discovery/skill-source.md
├── canon-system-shaping/skill-source.md
├── canon-review/skill-source.md
└── canon-shared/references/runtime-compatibility.toml

.agents/skills/
├── canon-discovery/SKILL.md
├── canon-system-shaping/SKILL.md
├── canon-review/SKILL.md
└── canon-shared/references/runtime-compatibility.toml

crates/canon-engine/src/
└── artifacts/
    └── markdown.rs

docs/
└── guides/
    └── modes.md

tests/
├── discovery_authoring_docs.rs
├── discovery_authoring_renderer.rs
├── discovery_authoring_run.rs
├── system_shaping_domain_modeling_docs.rs
├── system_shaping_authoring_renderer.rs
├── system_shaping_run.rs
├── review_authoring_docs.rs
├── review_authoring_renderer.rs
├── review_run.rs
└── skills_bootstrap.rs
```

**Structure Decision**: Keep the existing Rust workspace, embedded-skill, and
documentation layout. Implement the feature by updating the three target skill
source plus mirror pairs, reusing the existing shared markdown renderer for
section preservation, and extending focused mode-specific tests and release
surfaces rather than adding a new crate or persistence layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.
