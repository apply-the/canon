# Implementation Plan: Domain Modeling And Boundary Design

**Branch**: `017-domain-boundary-design` | **Date**: 2026-04-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/017-domain-boundary-design/spec.md`

## Summary

Extend `system-shaping`, `architecture`, and `change` with first-class domain-modeling surfaces that make bounded contexts, ubiquitous language, context relationships, ownership boundaries, and preserved domain invariants explicit in emitted packets. The implementation stays additive: `system-shaping` gains a dedicated domain-model artifact, `architecture` gains a dedicated context-map artifact, and `change` strengthens its existing bounded-change artifacts with domain-slice and cross-context reasoning. Supporting work updates the renderer and artifact contracts, the three mode skills, their materialized mirrors, templates, examples, method metadata, and focused validation coverage without changing run identity, approval semantics, or publish destinations.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact. The feature deepens three existing modes and their docs/tests but does not add a new mode, alter persistence layout, or change approval/publish mechanics.
**Scope In**:

- Add domain-modeling artifact surfaces to `system-shaping` and `architecture`.
- Strengthen `change` with explicit domain-slice, preserved-invariant, ownership-boundary, and cross-context sections in its existing packet.
- Update artifact contracts, renderers, summaries, skill guidance, templates, examples, and focused tests for the three target modes.
- Update roadmap and mode guidance so the delivered slice is documented honestly.

**Scope Out**:

- Introducing a standalone domain-modeling mode.
- Reworking persistence, publish destinations, gate semantics, or run identity.
- Extending the slice to `implementation`, `refactor`, `review`, `verification`, `incident`, or `migration`.
- General industry-standard artifact-shape work beyond the domain-modeling slice.
- Packaging, distribution, protocol, or hosted workflow features.

**Invariants**:

- `system-shaping`, `architecture`, and `change` remain critique-first and evidence-backed; domain outputs cannot become unchallenged restatements of the brief.
- Existing run identity, state transitions, approval behavior, evidence linking, and publish destinations remain unchanged.
- Domain outputs remain bounded to authored source material plus explicit surfaced assumptions; Canon must not invent authoritative boundaries unsupported by the brief.
- `change` remains a bounded existing-system modification workflow rather than broad greenfield redesign.
- Non-target modes remain behaviorally unchanged.

**Decision Log**: `specs/017-domain-boundary-design/decision-log.md`  
**Validation Ownership**: Generation changes live in artifact contracts, renderers, orchestrator summaries, skill/docs surfaces, and method metadata. Validation lives in focused contract, renderer, docs, and run tests plus an independent artifact review before implementation closeout.  
**Approval Gates**: bounded-impact does not add special human approvals beyond standard review.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024.  
**Primary Dependencies**: existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.  
**Storage**: repository files plus existing `.canon/` runtime persistence; no schema or layout changes.  
**Testing**: `cargo test`, focused top-level tests and `tests/contract/` coverage, and `/bin/bash scripts/validate-canon-skills.sh`.  
**Target Platform**: existing Canon CLI support matrix (macOS, Linux, Windows).  
**Project Type**: Rust workspace CLI + engine.  
**Existing System Touchpoints**:

- `crates/canon-engine/src/artifacts/contract.rs`
- `crates/canon-engine/src/artifacts/markdown.rs`
- `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`
- `crates/canon-engine/src/orchestrator/service/mode_change.rs`
- `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- `defaults/methods/system-shaping.toml`
- `defaults/methods/architecture.toml`
- `defaults/methods/change.toml`
- `defaults/embedded-skills/canon-system-shaping/skill-source.md`
- `defaults/embedded-skills/canon-architecture/skill-source.md`
- `defaults/embedded-skills/canon-change/skill-source.md`
- `.agents/skills/canon-system-shaping/SKILL.md`
- `.agents/skills/canon-architecture/SKILL.md`
- `.agents/skills/canon-change/SKILL.md`
- `docs/templates/canon-input/system-shaping.md`
- `docs/templates/canon-input/architecture.md`
- `docs/templates/canon-input/change.md`
- `docs/examples/canon-input/system-shaping-billing.md`
- `docs/examples/canon-input/architecture-state-management.md`
- `docs/examples/canon-input/change-add-caching.md`
- `docs/guides/modes.md`
- `ROADMAP.md`

**Performance Goals**: N/A; this slice changes packet fidelity and reviewability, not throughput.  
**Constraints**: preserve existing publish destinations and run lifecycle; keep docs in the current per-mode naming convention; prefer additive artifact changes and explicit uncertainty over inferred certainty; keep new plan metadata fields on one line so agent-context updates stay stable.  
**Scale/Scope**: three target modes, one new dedicated domain-model artifact in `system-shaping`, one new dedicated context-map artifact in `architecture`, strengthened `change` artifact sections, synchronized skills/docs/examples, and focused validation additions.

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
specs/017-domain-boundary-design/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── system-shaping-domain-modeling.md
│   ├── architecture-context-map.md
│   └── change-domain-slice.md
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
        ├── mode_shaping.rs
        ├── mode_change.rs
        └── summarizers.rs

defaults/
├── methods/
│   ├── system-shaping.toml
│   ├── architecture.toml
│   └── change.toml
└── embedded-skills/
  ├── canon-system-shaping/skill-source.md
  ├── canon-architecture/skill-source.md
  └── canon-change/skill-source.md

.agents/
└── skills/
  ├── canon-system-shaping/SKILL.md
  ├── canon-architecture/SKILL.md
  └── canon-change/SKILL.md

docs/
├── templates/canon-input/
│   ├── system-shaping.md
│   ├── architecture.md
│   └── change.md
├── examples/canon-input/
│   ├── system-shaping-billing.md
│   ├── architecture-state-management.md
│   └── change-add-caching.md
└── guides/modes.md

tests/
├── system_shaping_contract.rs
├── system_shaping_run.rs
├── architecture_contract.rs
├── architecture_run.rs
├── change_contract.rs
├── change_run.rs
└── contract/
  ├── system_shaping_contract.rs
  ├── architecture_contract.rs
  └── change_contract.rs
```

**Structure Decision**: Extend the existing Rust workspace and current per-mode docs layout rather than creating new crates or a parallel docs hierarchy. `system-shaping` and `architecture` changes stay in `mode_shaping.rs`, while `change` remains isolated in `mode_change.rs`.

## Complexity Tracking

No constitution violations. No deviations require justification.
