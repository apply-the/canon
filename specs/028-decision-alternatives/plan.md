# Implementation Plan: Decision Alternatives, Pattern Choices, And Framework Evaluations

**Branch**: `028-decision-alternatives` | **Date**: 2026-05-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/028-decision-alternatives/spec.md`

## Summary

Deliver the 028 decision-support slice by extending `system-shaping` and
`change` with explicit structural decision-analysis sections, extending
`implementation` and `migration` with framework-evaluation and
decision-evidence sections, preserving `architecture` as the regression
baseline, and aligning versioned docs plus runtime compatibility references to
`0.28.0` with focused Rust coverage, `cargo clippy`, and `cargo fmt` closeout.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature modifies authored
contracts, skill guidance, renderer preservation behavior, tests, and
release-facing documentation across already-modeled modes without introducing a
new runtime domain, persistence model, or approval semantic  
**Scope In**: roadmap and planning artifacts for `028`; shared decision or
framework-evaluation authored contracts for `system-shaping`, `change`,
`implementation`, and `migration`; `architecture` regression alignment;
targeted skill and template guidance; focused docs/examples/tests; and
repository version or release-surface updates for `0.28.0`  
**Scope Out**: new modes; live authenticated evidence collectors; distribution
channel expansion; protocol interoperability; runtime schema changes under
`.canon/`; and any approval or publish semantic change

**Invariants**:

- Canon MUST stay critique-first and evidence-backed; packets cannot invent
  viable alternatives, evidence, or unjustified confidence.
- Persona guidance MUST remain subordinate to explicit artifact contracts,
  gap markers, approval posture, and risk semantics.
- Existing run identity, publish destinations, approval targets,
  recommendation-only posture, and `.canon/` persistence layout MUST remain
  unchanged.

**Decision Log**: `specs/028-decision-alternatives/decision-log.md`  
**Validation Ownership**: Generation happens through skill, template, example,
renderer, contract, code, and documentation updates on the implementation
branch; validation happens through focused automated tests, skill-sync
validation, release-surface checks, coverage review for touched Rust files, and
independent review of the emitted packet and docs surfaces before merge.  
**Approval Gates**: Standard maintainer review plus repository quality gates
(`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, targeted and full test coverage, and `/bin/bash scripts/validate-canon-skills.sh`); no new runtime approval gate is added by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused docs/renderer/run tests, and `/bin/bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows with repo-local AI skill materialization  
**Project Type**: Rust CLI workspace with embedded skill sources, mirrored AI-facing skills, contract-based markdown rendering, and repository documentation artifacts  
**Existing System Touchpoints**: `ROADMAP.md`, `README.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `docs/guides/modes.md`, `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-change/skill-source.md`, `defaults/embedded-skills/canon-implementation/skill-source.md`, `defaults/embedded-skills/canon-migration/skill-source.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, mirrored `.agents/skills/.../SKILL.md` counterparts, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `docs/templates/canon-input/`, `docs/examples/canon-input/`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and focused mode tests under `tests/`  
**Performance Goals**: Preserve current packet-emission responsiveness and add no new mandatory network round trip or runtime state mutation  
**Constraints**: Keep `.canon/` schema and publish destinations unchanged; preserve skill-source to mirror synchronization; preserve explicit gap honesty and approval/risk semantics; defer live authenticated evidence harvesting; keep release surfaces consistent with `0.28.0`; and provide focused coverage for every modified or newly added Rust file  
**Scale/Scope**: One feature branch spanning four runtime-targeted modes, one regression anchor mode, shared markdown artifact helpers, release-facing docs, version surfaces, and focused regression coverage

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
specs/028-decision-alternatives/
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
├── canon-system-shaping/skill-source.md
├── canon-change/skill-source.md
├── canon-implementation/skill-source.md
├── canon-migration/skill-source.md
├── canon-architecture/skill-source.md
└── canon-shared/references/runtime-compatibility.toml

.agents/skills/
├── canon-system-shaping/SKILL.md
├── canon-change/SKILL.md
├── canon-implementation/SKILL.md
├── canon-migration/SKILL.md
├── canon-architecture/SKILL.md
└── canon-shared/references/runtime-compatibility.toml

crates/canon-engine/src/
└── artifacts/
    ├── contract.rs
    └── markdown.rs

docs/
├── guides/modes.md
├── templates/canon-input/
└── examples/canon-input/

tests/
├── system_shaping_*.rs
├── change_*.rs
├── implementation_*.rs
├── migration_*.rs
├── architecture_*.rs
└── release_*.rs
```

**Structure Decision**: Keep the existing Rust workspace plus skill-source and
documentation layout. Implement the feature by updating mode-specific skill
source and mirror pairs, shared artifact rendering and contract code,
operator-facing templates/examples/docs, release/version surfaces, and focused
tests instead of adding a new crate or runtime storage layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.