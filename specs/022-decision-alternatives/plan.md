# Implementation Plan: Decision Alternatives, Pattern Choices, And Framework Evaluations

**Branch**: `022-decision-alternatives` | **Date**: 2026-04-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/022-decision-alternatives/spec.md`

## Summary

Deliver the first Decision Alternatives slice by extending `system-shaping`,
`change`, `implementation`, and `migration` with explicit option-analysis or
framework-evaluation authored sections, aligning `architecture` as the existing
reference implementation, rolling bounded persona guidance forward into the
in-scope and adjacent review or operational modes, and proving the behavior via
focused contract, renderer, docs, run, and release-surface validation for the
`0.22.0` release.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature modifies authored
contracts, skill guidance, renderer preservation behavior, tests, and release
documentation across already-modeled modes without introducing a new runtime
domain or persistence model  
**Scope In**: roadmap and planning artifacts for `022`; shared decision or
framework-evaluation authored contracts for `system-shaping`, `architecture`,
`change`, `implementation`, and `migration`; persona guidance updates for those
runtime-targeted modes plus guidance-only persona completion for `review`,
`pr-review`, `verification`, and `incident`; focused docs/examples/tests; and
repository version or release-surface updates for `0.22.0`  
**Scope Out**: new modes such as `security-assessment` or
`supply-chain-analysis`; live external evidence collectors; package-manager
distribution work; protocol interoperability; runtime schema changes under
`.canon/`; and any approval or publish semantics change

**Invariants**:

- Canon MUST stay critique-first and evidence-backed; option packets cannot
  invent alternatives, evidence, or unjustified confidence.
- Persona guidance MUST remain subordinate to explicit artifact contracts,
  gap markers, approval posture, and risk semantics.
- Existing run identity, publish destinations, approval targets,
  recommendation-only posture, and `.canon/` persistence layout MUST remain
  unchanged.

**Decision Log**: `specs/022-decision-alternatives/decision-log.md`  
**Validation Ownership**: Generation happens through skill, template, example,
renderer, contract, and documentation updates on the implementation branch;
validation happens through focused automated tests, skill-sync validation,
release-surface checks, and a separate review of the emitted packet and docs
surfaces before merge.  
**Approval Gates**: Standard maintainer review plus existing repository quality
gates (`cargo fmt`, `cargo clippy`, focused and full test coverage, and
`scripts/validate-canon-skills.sh`); no new runtime approval gate is added by
this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run`, focused docs or renderer or run tests, and `/bin/bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows with repo-local AI skill materialization  
**Project Type**: Rust CLI workspace with embedded skill sources, mirrored AI-facing skills, contract-based markdown rendering, and documentation artifacts  
**Existing System Touchpoints**: `ROADMAP.md`; `README.md`; `CHANGELOG.md`; `docs/guides/modes.md`; `Cargo.toml`; `defaults/embedded-skills/canon-system-shaping/skill-source.md`; `defaults/embedded-skills/canon-architecture/skill-source.md`; `defaults/embedded-skills/canon-change/skill-source.md`; `defaults/embedded-skills/canon-implementation/skill-source.md`; `defaults/embedded-skills/canon-migration/skill-source.md`; `defaults/embedded-skills/canon-review/skill-source.md`; `defaults/embedded-skills/canon-pr-review/skill-source.md`; `defaults/embedded-skills/canon-verification/skill-source.md`; `defaults/embedded-skills/canon-incident/skill-source.md`; mirrored `.agents/skills/.../SKILL.md` counterparts; `crates/canon-engine/src/artifacts/markdown.rs`; `crates/canon-engine/src/artifacts/contract.rs`; `docs/templates/canon-input/`; `docs/examples/canon-input/`; `tests/system_shaping_*`; `tests/architecture_*`; `tests/change_*`; `tests/implementation_*`; `tests/migration_*`; `tests/review_*`; `tests/pr_review_*`; `tests/verification_*`; `tests/incident_*`  
**Performance Goals**: Preserve current operator workflow and packet emission responsiveness; add no extra runtime step beyond the existing skill-guided authoring process  
**Constraints**: Keep `.canon/` schema and publish destinations unchanged; preserve skill-source to mirror synchronization; preserve explicit gap honesty and approval or risk semantics; keep live ecosystem-data collection out of scope for this slice; keep release surfaces consistent with `0.22.0`  
**Scale/Scope**: One feature branch spanning five runtime-targeted modes, four guidance-only persona completions, shared markdown artifact helpers, release-facing docs and compatibility references, and focused regression coverage

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
specs/022-decision-alternatives/
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
├── canon-architecture/skill-source.md
├── canon-change/skill-source.md
├── canon-implementation/skill-source.md
├── canon-migration/skill-source.md
├── canon-review/skill-source.md
├── canon-pr-review/skill-source.md
├── canon-verification/skill-source.md
└── canon-incident/skill-source.md

.agents/skills/
├── canon-system-shaping/SKILL.md
├── canon-architecture/SKILL.md
├── canon-change/SKILL.md
├── canon-implementation/SKILL.md
├── canon-migration/SKILL.md
├── canon-review/SKILL.md
├── canon-pr-review/SKILL.md
├── canon-verification/SKILL.md
└── canon-incident/SKILL.md

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
├── architecture_*.rs
├── change_*.rs
├── implementation_*.rs
├── migration_*.rs
├── review_*.rs
├── pr_review_*.rs
├── verification_*.rs
└── incident_*.rs
```

**Structure Decision**: Keep the existing Rust workspace plus skill-source and
documentation layout. Implement the feature by updating mode-specific skill
source and mirror pairs, shared artifact rendering and contract code,
operator-facing templates/examples/docs, and focused tests instead of adding a
new crate or runtime storage layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.

