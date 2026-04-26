# Implementation Plan: Mode Authoring Specialization

**Branch**: `016-mode-authoring-specialization` | **Date**: 2026-04-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/016-mode-authoring-specialization/spec.md`

## Summary

Deliver the first bounded slice of Mode Authoring Specialization by extending `requirements`, `discovery`, and `change` with explicit authored-body contracts, renderer preservation of authored H2 sections, and honest `## Missing Authored Body` fallbacks that name the missing canonical heading. The slice updates the three mode skills, their materialized `.agents/skills/` mirrors, the relevant docs templates and examples, the mode guidance, roadmap, and focused validation coverage. Existing reference modes (`backlog`, `architecture`, `pr-review`) remain unchanged.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact. The slice changes authored-body behavior in three existing modes and their supporting docs/tests, but does not change mode count, persistence layout, approval gates, or publish destinations.
**Scope In**:

- Renderer and orchestrator changes for `requirements`, `discovery`, and `change` only.
- Skill source and materialized skill updates for those three modes.
- Updates to the existing templates `docs/templates/canon-input/requirements.md`, `docs/templates/canon-input/discovery.md`, and `docs/templates/canon-input/change.md`.
- Updates to the existing worked examples `docs/examples/canon-input/requirements-api-v2.md`, `docs/examples/canon-input/discovery-legacy-migration.md`, and `docs/examples/canon-input/change-add-caching.md`.
- Mode guidance and roadmap updates that document the delivered first slice honestly.
- Focused contract, renderer, run, and docs validation for the three updated modes.

**Scope Out**:

- `architecture`, `backlog`, and `pr-review` runtime behavior changes.
- Extending the pattern to `system-shaping`, `implementation`, `refactor`, `review`, `verification`, `incident`, or `migration`.
- Industry-standard artifact-shape work beyond the authored-body contract.
- Any change to run identity, persistence layout, evidence model, or publish destinations.

**Invariants**:

- Updated modes remain critique-first and evidence-backed; authored-body preservation is additive and cannot bypass critique or validation.
- Missing authored content is surfaced explicitly with `## Missing Authored Body` that references the missing canonical heading; no generic filler may be fabricated in its place.
- Canonical headings use exact-match semantics unless a mode contract documents a specific compatibility alias.
- Reference modes already specialized (`backlog`, `architecture`, `pr-review`) remain behaviorally unchanged.
- Skill guidance, templates, and examples for each updated mode stay synchronized on the same canonical authored-heading contract.

**Decision Log**: `specs/016-mode-authoring-specialization/decision-log.md`  
**Validation Ownership**: Generation changes live in renderer/orchestrator/skill/docs surfaces. Validation lives in focused tests, docs checks, skill validation, and an independent post-task review pass before implementation.  
**Approval Gates**: bounded-impact does not add explicit human approval gates beyond standard review.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024.  
**Primary Dependencies**: existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.  
**Storage**: repository files plus existing `.canon/` runtime persistence; no schema or layout changes.  
**Testing**: `cargo test`, targeted top-level wrappers and `tests/contract/` / `tests/integration/` coverage, plus `/bin/bash scripts/validate-canon-skills.sh`; PowerShell validator parity is reviewed whenever validator logic changes.  
**Target Platform**: existing Canon CLI support matrix (macOS, Linux, Windows).  
**Project Type**: Rust workspace CLI + engine.  
**Existing System Touchpoints**:

- `crates/canon-engine/src/artifacts/markdown.rs`
- `crates/canon-engine/src/artifacts/contract.rs`
- `crates/canon-engine/src/orchestrator/service/mode_requirements.rs`
- `crates/canon-engine/src/orchestrator/service/mode_discovery.rs`
- `crates/canon-engine/src/orchestrator/service/mode_change.rs`
- `defaults/embedded-skills/canon-requirements/skill-source.md`
- `defaults/embedded-skills/canon-discovery/skill-source.md`
- `defaults/embedded-skills/canon-change/skill-source.md`
- `.agents/skills/canon-requirements/SKILL.md`
- `.agents/skills/canon-discovery/SKILL.md`
- `.agents/skills/canon-change/SKILL.md`
- `docs/templates/canon-input/requirements.md`
- `docs/templates/canon-input/discovery.md`
- `docs/templates/canon-input/change.md`
- `docs/examples/canon-input/requirements-api-v2.md`
- `docs/examples/canon-input/discovery-legacy-migration.md`
- `docs/examples/canon-input/change-add-caching.md`
- `docs/guides/modes.md`
- `ROADMAP.md`

**Performance Goals**: N/A; this slice changes authored-body fidelity, not throughput.  
**Constraints**: keep reference-mode behavior stable; preserve current inspect/publish/runtime surfaces; use the existing template/example file naming convention instead of creating a second docs hierarchy; negative validation should derive incomplete fixtures from the updated examples instead of adding a second example set unless a later task proves that insufficient.  
**Scale/Scope**: three modes, three skills, three templates, three examples, focused renderer/orchestrator updates, and new targeted tests.

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
specs/016-mode-authoring-specialization/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── requirements-authoring.md
│   ├── discovery-authoring.md
│   └── change-authoring.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
└── canon-engine/
  └── src/
    ├── artifacts/
    │   ├── contract.rs
    │   └── markdown.rs
    └── orchestrator/
      └── service/
        ├── mode_requirements.rs
        ├── mode_discovery.rs
        └── mode_change.rs

defaults/
└── embedded-skills/
  ├── canon-requirements/skill-source.md
  ├── canon-discovery/skill-source.md
  └── canon-change/skill-source.md

.agents/
└── skills/
  ├── canon-requirements/SKILL.md
  ├── canon-discovery/SKILL.md
  └── canon-change/SKILL.md

docs/
├── templates/canon-input/
│   ├── requirements.md
│   ├── discovery.md
│   └── change.md
├── examples/canon-input/
│   ├── requirements-api-v2.md
│   ├── discovery-legacy-migration.md
│   └── change-add-caching.md
└── guides/modes.md

tests/
├── requirements_authoring_renderer.rs
├── discovery_authoring_renderer.rs
├── change_authoring_renderer.rs
├── requirements_authoring_run.rs
├── discovery_authoring_run.rs
├── change_authoring_run.rs
├── requirements_authoring_docs.rs
├── discovery_authoring_docs.rs
├── change_authoring_docs.rs
└── contract/
  ├── requirements_authoring_contract.rs
  ├── discovery_authoring_contract.rs
  └── change_authoring_contract.rs

tests/integration/
└── [optional helper modules if existing wrapper patterns are reused]
```

**Structure Decision**: extend the existing Rust workspace and existing docs file naming rather than introducing new crates or a second docs hierarchy. New tests live alongside current top-level tests and `tests/contract/` contract coverage.

## Complexity Tracking

No constitution violations. No deviations require justification.
