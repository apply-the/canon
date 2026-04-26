# Implementation Plan: Mode Authoring Specialization Follow-On

**Branch**: `019-authoring-specialization-remaining` | **Date**: 2026-04-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/019-authoring-specialization-remaining/spec.md`

## Summary

Complete the second Mode Authoring Specialization slice by extending `system-shaping`, `implementation`, and `refactor` to use explicit authored H2 contracts across their remaining packet artifacts, preserve authored sections verbatim in the markdown renderer, emit honest `## Missing Authored Body` markers for absent required sections, and synchronize the contract across skills, templates, worked examples, docs, roadmap text, and focused tests.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact. The slice deepens three existing modes and their repository-local docs/tests without introducing a new runtime domain, changing approval or publish semantics, or widening beyond authored-body contracts and their evidence surfaces.
**Scope In**:

- Extend the authored-body contract for `system-shaping`, `implementation`, and `refactor` packet artifacts.
- Reuse the existing authored-section preservation helper for the targeted modes.
- Ensure orchestrator paths pass real authored brief content to the renderer.
- Update embedded skills, mirrored `.agents` skill files, templates, worked examples, mode guidance, roadmap text, and focused tests.

**Scope Out**:

- Reopening already-specialized modes such as `requirements`, `discovery`, `change`, and `architecture`.
- Rolling the same specialization into `review`, `verification`, `incident`, or `migration`.
- Changing run identity, gate semantics, recommendation-only posture, persistence layout, publish destinations, or evidence-bundle behavior.
- Introducing new Canon modes, artifact families, or external evidence collectors.

**Invariants**:

- `system-shaping`, `implementation`, and `refactor` remain critique-first and evidence-backed.
- Missing required authored sections emit `## Missing Authored Body` naming the canonical heading instead of plausible filler.
- `implementation` and `refactor` keep their existing recommendation-only and approval-aware posture in v0.x.
- Non-target modes and already-delivered specialization behavior remain unchanged.
- Canonical H2 headings remain the contract; near-match headings count as missing unless explicitly documented as aliases.

**Decision Log**: `specs/019-authoring-specialization-remaining/decision-log.md`  
**Validation Ownership**: Generation work updates artifact contracts, renderer logic, orchestrator handoff, skills/docs/examples, and focused tests. Validation remains separate via structural checks, focused logical tests, skill synchronization validation, and independent artifact review recorded in `validation-report.md`.  
**Approval Gates**: bounded-impact does not add special human approvals beyond standard review.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024.  
**Primary Dependencies**: existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.  
**Storage**: repository files plus existing `.canon/` runtime persistence; no schema or layout changes.  
**Testing**: `cargo test`, focused top-level renderer/run tests, `tests/contract/*`, docs-sync assertions, and `/bin/bash scripts/validate-canon-skills.sh`.  
**Target Platform**: existing Canon CLI support matrix on macOS, Linux, and Windows.  
**Project Type**: Rust workspace CLI plus engine.  
**Existing System Touchpoints**:

- `crates/canon-engine/src/artifacts/contract.rs`
- `crates/canon-engine/src/artifacts/markdown.rs`
- `crates/canon-engine/src/orchestrator/service/`
- `defaults/embedded-skills/canon-system-shaping/skill-source.md`
- `defaults/embedded-skills/canon-implementation/skill-source.md`
- `defaults/embedded-skills/canon-refactor/skill-source.md`
- `.agents/skills/canon-system-shaping/SKILL.md`
- `.agents/skills/canon-implementation/SKILL.md`
- `.agents/skills/canon-refactor/SKILL.md`
- `docs/templates/canon-input/system-shaping.md`
- `docs/templates/canon-input/implementation.md`
- `docs/templates/canon-input/refactor.md`
- `docs/examples/canon-input/system-shaping-billing.md`
- `docs/examples/canon-input/implementation-auth-session-revocation.md`
- `docs/examples/canon-input/refactor-auth-session-cleanup.md`
- `docs/guides/modes.md`
- `ROADMAP.md`
- `tests/contract/`
- `tests/`

**Performance Goals**: N/A for throughput; this slice optimizes authored-packet fidelity, reviewability, and contract honesty rather than runtime speed.  
**Constraints**: preserve artifact file names and publish paths; preserve recommendation-only posture for execution-heavy modes; preserve critique-first behavior; keep agent-context plan fields on one line for stable `AGENTS.md` updates; avoid changes outside the three targeted modes except synchronized docs references.  
**Scale/Scope**: three target modes, one renderer/contract surface, one orchestrator handoff surface, three embedded skills plus mirrored `.agents` copies, three templates, three worked examples, one mode guide, one roadmap entry, and focused contract/renderer/run/docs tests.

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

Post-design re-check: Phase 0 and Phase 1 artifacts preserve the declared change mode, bounded-impact classification, explicit scope boundaries, durable decision logging, and separated validation ownership. No new constitution deviations were introduced during planning.

## Project Structure

### Documentation (this feature)

```text
specs/019-authoring-specialization-remaining/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── mode-authored-body-contracts.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── artifacts/
│       │   ├── contract.rs
│       │   └── markdown.rs
│       └── orchestrator/
│           └── service/
└── canon-cli/

defaults/
└── embedded-skills/
    ├── canon-system-shaping/skill-source.md
    ├── canon-implementation/skill-source.md
    └── canon-refactor/skill-source.md

.agents/
└── skills/
    ├── canon-system-shaping/SKILL.md
    ├── canon-implementation/SKILL.md
    └── canon-refactor/SKILL.md

docs/
├── templates/canon-input/
│   ├── system-shaping.md
│   ├── implementation.md
│   └── refactor.md
├── examples/canon-input/
│   ├── system-shaping-billing.md
│   ├── implementation-auth-session-revocation.md
│   └── refactor-auth-session-cleanup.md
└── guides/modes.md

tests/
├── contract/
└── integration/
```

**Structure Decision**: Extend the existing Rust workspace and current Canon doc surfaces rather than introducing a new crate, mode, or parallel documentation hierarchy. The work stays centered on artifact contracts, renderer behavior, orchestrator handoff, skill/template/example synchronization, and focused targeted tests.

## Complexity Tracking

No constitution violations. No deviations require justification.
