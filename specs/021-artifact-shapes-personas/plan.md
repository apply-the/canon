# Implementation Plan: Industry-Standard Artifact Shapes With Personas

**Branch**: `021-artifact-shapes-personas` | **Date**: 2026-04-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/021-artifact-shapes-personas/spec.md`

## Summary

Deliver the first slice of persona-aware industry-standard packet shaping for
`requirements`, `architecture`, and `change` by updating roadmap and operator
docs, extending the corresponding skill source and mirrored skill guidance,
preserving the shaped sections through the renderer and artifact-contract
surface, and proving the behavior with focused regression and negative-path
validation.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature changes existing
authoring, rendering, and validation behavior across three already-supported
modes without introducing a new runtime domain or persistence model  
**Scope In**: roadmap and planning artifacts for persona-aware artifact shapes;
first-slice changes for `requirements`, `architecture`, and `change` skill
source plus mirrored skills; renderer and artifact-contract support for the new
packet expectations; focused docs/examples/tests that prove the slice  
**Scope Out**: new Canon modes; full persona rollout for every remaining mode;
package-manager distribution work; protocol interoperability; runtime schema or
filesystem layout changes under `.canon/`

**Invariants**:

- Persona guidance MUST remain subordinate to Canon's explicit artifact
  contracts, missing-authored-body markers, evidence posture, and approval
  semantics.
- Existing run, publish, inspect, and approval behavior MUST remain unchanged
  for the first slice.
- Modes outside `requirements`, `architecture`, and `change` MUST keep their
  current observable behavior unless a later scoped feature expands coverage.

**Decision Log**: `specs/021-artifact-shapes-personas/decision-log.md`  
**Validation Ownership**: Generation happens in the implementation branch
through skill, renderer, and documentation updates; validation happens through
focused automated tests, skill-sync validation, and a separate review of the
produced packet/evidence surfaces before merge.  
**Approval Gates**: Standard maintainer review plus existing repository quality
gates (`cargo fmt`, targeted and full test coverage as needed, and skill-sync
validation); no new runtime approval gate is introduced by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run --no-fail-fast`, focused
integration/unit tests, and `scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and
Windows with repo-local AI skill materialization  
**Project Type**: Rust CLI workspace with embedded skill sources, mirrored AI-facing skills, contract-based markdown rendering, and documentation artifacts  
**Existing System Touchpoints**: `ROADMAP.md`; `docs/guides/modes.md`;
`defaults/embedded-skills/canon-requirements/skill-source.md`;
`defaults/embedded-skills/canon-architecture/skill-source.md`;
`defaults/embedded-skills/canon-change/skill-source.md`;
`.agents/skills/canon-requirements/SKILL.md`;
`.agents/skills/canon-architecture/SKILL.md`;
`.agents/skills/canon-change/SKILL.md`;
`crates/canon-engine/src/artifacts/markdown.rs`;
`crates/canon-engine/src/artifacts/contract.rs`;
focused test surfaces under `tests/integration/` and crate-local unit tests  
**Performance Goals**: Preserve the current operator workflow and packet
emission responsiveness; add no extra manual step to the run flow beyond the
existing skill-guided authoring process  
**Constraints**: Keep `.canon/` schema and publish destinations unchanged;
preserve skill-source to mirror synchronization; preserve explicit missing-gap
honesty and approval/risk semantics; keep the first slice bounded to three
modes  
**Scale/Scope**: One feature branch spanning roadmap/planning artifacts, three
skill pairs, shared renderer/contract helpers, and focused regression coverage

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
specs/021-artifact-shapes-personas/
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
├── canon-requirements/skill-source.md
├── canon-architecture/skill-source.md
└── canon-change/skill-source.md

.agents/skills/
├── canon-requirements/SKILL.md
├── canon-architecture/SKILL.md
└── canon-change/SKILL.md

crates/canon-engine/src/
└── artifacts/
    ├── contract.rs
    └── markdown.rs

docs/
├── guides/modes.md
└── examples/canon-input/

tests/
├── integration/
│   ├── architecture_run.rs
│   ├── change_run.rs
│   └── run_lookup.rs
└── [crate-local unit tests live beside the touched Rust modules]
```

**Structure Decision**: Keep the existing Rust workspace plus skill-source and
documentation layout. Implement the feature by updating first-slice skill
source/mirror pairs, shared artifact rendering and contract code, and focused
tests/docs rather than adding a new crate or runtime storage layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.
