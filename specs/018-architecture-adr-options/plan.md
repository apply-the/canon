# Implementation Plan: Architecture ADR And Options

**Branch**: `018-architecture-adr-options` | **Date**: 2026-04-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/018-architecture-adr-options/spec.md`

## Summary

Upgrade the `architecture` mode with a tighter authored decision contract that combines three roadmap threads in one bounded slice: real authored-body preservation for decision sections, an ADR-like decision shape, and explicit option-analysis sections. The implementation keeps the existing architecture packet and C4 artifacts intact while strengthening the decision-facing artifacts, updating the `architecture` skill/template/example/docs set, and adding focused contract, renderer, and run validation for both positive and missing-body cases.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact. The slice deepens one delivered mode and its tests/docs without adding a new runtime domain, changing approval/publish semantics, or widening beyond repository-local artifacts.
**Scope In**:

- Strengthen `architecture` authored decision sections and emitted decision-facing artifacts.
- Preserve ADR-like and option-analysis sections verbatim when authored canonically.
- Keep the existing C4 packet intact while improving architecture decision readability.
- Update the corresponding `architecture` method metadata, skill sources, mirrored skill file, template, example, and mode guide.
- Add focused contract, renderer, and run coverage for positive and missing-body cases.

**Scope Out**:

- Extending the same pattern to `requirements`, `change`, or any other mode.
- Adding new runtime state, gates, publish destinations, or external evidence collectors.
- Replacing the current C4 artifact family or renaming architecture artifact files.
- Broader roadmap work for packaging, protocol interoperability, supply chain, or security assessment.

**Invariants**:

- `architecture` remains critique-first and evidence-backed; missing authored decision context must stay explicit.
- `system-context.md`, `container-view.md`, and `component-view.md` remain behaviorally unchanged.
- Existing run identity, state transitions, approval behavior, evidence linking, and publish destinations remain unchanged.
- Non-target modes remain behaviorally unchanged.

**Decision Log**: `specs/018-architecture-adr-options/decision-log.md`  
**Validation Ownership**: Generation changes live in method metadata, skill/docs surfaces, artifact contract and renderer logic, and focused tests. Validation lives in focused contract, renderer, and run tests plus an independent artifact review before closeout.  
**Approval Gates**: bounded-impact does not add special human approvals beyond standard review.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024.  
**Primary Dependencies**: existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.  
**Storage**: repository files plus existing `.canon/` runtime persistence; no schema or layout changes.  
**Testing**: `cargo test`, focused top-level renderer and run tests, `tests/contract/*`, and `/bin/bash scripts/validate-canon-skills.sh`.  
**Target Platform**: existing Canon CLI support matrix on macOS, Linux, and Windows.  
**Project Type**: Rust workspace CLI plus engine.  
**Existing System Touchpoints**:

- `crates/canon-engine/src/artifacts/contract.rs`
- `crates/canon-engine/src/artifacts/markdown.rs`
- `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- `defaults/methods/architecture.toml`
- `defaults/embedded-skills/canon-architecture/skill-source.md`
- `.agents/skills/canon-architecture/SKILL.md`
- `docs/templates/canon-input/architecture.md`
- `docs/examples/canon-input/architecture-state-management.md`
- `docs/guides/modes.md`
- `ROADMAP.md`
- `tests/contract/architecture_contract.rs`
- `tests/contract/architecture_c4_contract.rs`
- `tests/architecture_c4_renderer.rs`
- `tests/architecture_c4_run.rs`
- `tests/integration/architecture_run.rs`

**Performance Goals**: N/A for throughput; this slice optimizes decision fidelity and reviewability, not runtime speed.  
**Constraints**: preserve artifact file names and publish paths; preserve C4 behavior; keep authored decision honesty explicit; keep new agent-context plan fields on one line for stable `AGENTS.md` updates.  
**Scale/Scope**: one target mode, one method metadata file, one skill source plus one mirrored skill file, one template, one worked example, one guide entry, one roadmap entry, renderer/contract updates, and focused architecture tests.

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
specs/018-architecture-adr-options/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── architecture-decision-shape.md
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
      ├── gatekeeper.rs
      └── service/
        └── summarizers.rs

defaults/
├── methods/
│   └── architecture.toml
└── embedded-skills/
  └── canon-architecture/skill-source.md

.agents/
└── skills/
  └── canon-architecture/SKILL.md

docs/
├── templates/canon-input/architecture.md
├── examples/canon-input/architecture-state-management.md
└── guides/modes.md

tests/
├── architecture_c4_renderer.rs
├── architecture_c4_run.rs
├── integration/architecture_run.rs
└── contract/
  ├── architecture_contract.rs
  └── architecture_c4_contract.rs
```

**Structure Decision**: Extend the existing Rust workspace and current `architecture` docs surface rather than introducing a new crate, a new mode, or a parallel docs hierarchy. The implementation stays centered on artifact contracts, renderer logic, skill/doc synchronization, and focused architecture-only tests.

## Complexity Tracking

No constitution violations. No deviations require justification.
