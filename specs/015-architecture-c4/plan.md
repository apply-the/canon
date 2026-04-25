# Implementation Plan: Stronger Architecture Outputs (C4 Model)

**Branch**: `015-architecture-c4` | **Date**: 2026-04-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/015-architecture-c4/spec.md`

## Summary

Extend the existing `architecture` mode with three textual C4 model artifacts (`system-context.md`, `container-view.md`, `component-view.md`), driven by authored H2 sections in the supplied brief. The renderer preserves authored content verbatim and emits explicit `## Missing Authored Body` markers when sections are absent. The existing critique-first artifact set (`architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`, `boundary-map.md`, `readiness-assessment.md`) remains intact and unchanged in shape. The `canon-architecture` skill is updated to require authored C4 sections, and starter templates plus realistic examples ship under `docs/templates/canon-input/architecture/` and `docs/examples/canon-input/architecture/`.

## Governance Context

**Execution Mode**: change

**Risk Classification**: bounded-impact. The work extends an existing mode by adding artifacts, renderer logic, a skill update, and supporting docs. No run identity, persistence, publish flow, or other modes are touched. Risk is bounded to the architecture artifact set and its docs.

**Scope In**:

- Architecture artifact contract additions (3 new artifacts, gate associations).
- Architecture renderer changes that extract authored C4 sections and preserve them verbatim, with `## Missing Authored Body` fallbacks.
- `canon-architecture` skill source and materialized SKILL.md updates.
- Starter template and realistic example for architecture briefs that include the new C4 sections.
- Targeted Rust tests for the new artifact contract, renderer behavior, and end-to-end run.

**Scope Out**:

- Changes to other modes.
- C4 Level 4 (Code) views or generated diagrams.
- Authored-body rollout for non-C4 sections in other modes (Mode Authoring Specialization).
- Domain Modeling artifacts (`Domain Modeling And Boundary Design`).
- Run identity, persistence, or publish layout changes.

**Invariants**:

- `architecture` remains critique-first; C4 artifacts are additive.
- Authored content is preserved verbatim; missing content emits an explicit marker, never fabricated.
- Existing architecture artifact rendering and tests remain unchanged.
- The new artifact set continues to flow through the existing inspect/publish surfaces unmodified.

**Decision Log**: `specs/015-architecture-c4/decision-log.md`
**Validation Ownership**: Generation lives in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs`. Validation lives in independent Rust tests under `tests/` and `tests/contract/`, plus the skill validator script and an independent published-packet review.
**Approval Gates**: bounded-impact does not require explicit human approval gates beyond the standard merge review.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024.
**Primary Dependencies**: existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`); `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.
**Storage**: local filesystem under `.canon/`; no schema or layout changes; published architecture artifacts continue to land in their existing publish destination.
**Testing**: `cargo test`, `cargo nextest run`, plus `/bin/bash scripts/validate-canon-skills.sh` for skill structure.
**Target Platform**: macOS, Linux, Windows (the existing Canon CLI surface).
**Project Type**: CLI + engine workspace.
**Existing System Touchpoints**:

- `crates/canon-engine/src/artifacts/contract.rs` (architecture artifact contract list and gate associations).
- `crates/canon-engine/src/artifacts/markdown.rs` (architecture renderer).
- `defaults/embedded-skills/canon-architecture/skill-source.md` and `.agents/skills/canon-architecture/SKILL.md` (skill content).
- `docs/templates/canon-input/` and `docs/examples/canon-input/` (template + example surface).
- existing architecture-related tests under `tests/` and `tests/contract/`.

**Performance Goals**: N/A; this is a documentation and renderer surface change.
**Constraints**: must not regress existing architecture tests; must not change the public publish destination or run identity contract.
**Scale/Scope**: 3 new artifacts, ~3 renderer cases, 1 skill update, 1 template, 1 example, ~5 test additions.

## Constitution Check

- [X] Execution mode is declared and matches the requested work
- [X] Risk classification is explicit and autonomy is appropriate for that risk
- [X] Scope boundaries and exclusions are recorded
- [X] Invariants are explicit before implementation
- [X] Required artifacts and owners are identified
- [X] Decision logging is planned and linked to a durable artifact
- [X] Validation plan separates generation from validation
- [X] Declared-risk approval checkpoints are named where required by the risk classification (none required at bounded-impact)
- [X] Any constitution deviations are documented in Complexity Tracking (none)

## Project Structure

### Documentation (this feature)

```text
specs/015-architecture-c4/
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
crates/
├── canon-cli/
└── canon-engine/
    └── src/
        └── artifacts/
            ├── contract.rs    # add C4 artifact entries
            └── markdown.rs    # add C4 render branches and authored-section extraction

defaults/
└── embedded-skills/
    └── canon-architecture/
        └── skill-source.md   # require authored C4 H2 sections

.agents/
└── skills/
    └── canon-architecture/
        └── SKILL.md          # mirror of skill-source.md

docs/
├── templates/
│   └── canon-input/
│       └── architecture/
│           └── brief.md      # new: template with all required H2 sections
└── examples/
    └── canon-input/
        └── architecture/
            └── brief.md      # new: realistic authored example

tests/
├── architecture_c4_contract.rs       # new
├── architecture_c4_renderer.rs       # new
├── architecture_c4_run.rs            # new
└── contract/
    └── architecture_c4_contract.rs   # new (artifact contract surface)
```

**Structure Decision**: extend the existing single-workspace Rust layout. No new crates. New tests live alongside existing architecture-related tests under `tests/` and `tests/contract/`.

## Complexity Tracking

No constitution violations. No deviations require justification.
