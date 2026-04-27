# Implementation Plan: Mode Authoring Specialization Completion

**Branch**: `020-authoring-specialization-completion` | **Date**: 2026-04-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/020-authoring-specialization-completion/spec.md`

## Summary

Complete the remaining Mode Authoring Specialization rollout by extending `review`, `verification`, `incident`, and `migration` to use explicit authored H2 contracts across every emitted artifact, preserve authored bodies verbatim in the markdown renderer, emit honest `## Missing Authored Body` markers for absent sections, keep each mode's existing governance posture intact, synchronize skills/templates/examples/docs, and ship the release/docs surfaces as Canon `0.20.0`.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact. The slice finishes an existing authored-body rollout across four already-shipped modes and related repository-local docs/tests without changing persistence layout, publish destinations, or mode governance semantics.
**Scope In**:

- Extend authored-body contracts for `review`, `verification`, `incident`, and `migration` artifact packets.
- Reuse existing authored-section extraction and missing-body marker helpers where they fit, while preserving mode-specific verdict and posture logic.
- Restore or preserve authored-source handoff into renderers for the targeted modes.
- Update embedded skills, mirrored `.agents` skills, starter templates, worked examples, guide text, roadmap/changelog/version surfaces, and focused tests.

**Scope Out**:

- Reopening already-completed specialization slices for other modes except for sync-level docs/version references.
- Changing run identity, approval target semantics, publish destinations, `.canon/` layout, or adapter behavior.
- Changing review/verification critique posture or incident/migration recommendation-only posture.
- Solving unrelated hook/test harness issues unless a narrow fix is required to validate this slice.

**Invariants**:

- Missing required authored sections emit `## Missing Authored Body` naming the canonical heading instead of plausible filler.
- `review` and `verification` remain critique-first and evidence-backed.
- `incident` and `migration` remain recommendation-only operational modes with their current safety and approval posture unchanged.
- Non-target modes and already-delivered specialization behavior remain unchanged.
- Skills, templates, examples, docs, and tests must describe the same canonical contract as the runtime surfaces.

**Decision Log**: `specs/020-authoring-specialization-completion/decision-log.md`  
**Validation Ownership**: Generation work updates contracts, renderer/orchestrator behavior, user-facing guidance, and focused tests. Validation remains separate through structural commands, focused logical test suites, skill validation, and independent artifact review recorded in `validation-report.md`.  
**Approval Gates**: bounded-impact adds no special human approval beyond normal review, but implementation must preserve any existing runtime approval targets and blocked-gate behavior for the targeted modes.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024.  
**Primary Dependencies**: existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.  
**Storage**: repository files plus existing `.canon/` runtime persistence; no schema or layout changes.  
**Testing**: focused `cargo test` targets under `tests/contract/`, `tests/integration/`, renderer/docs-sync tests at repo root, plus `/bin/bash scripts/validate-canon-skills.sh`; full-workspace checks stay available but are not the only evidence layer.  
**Target Platform**: existing Canon CLI support matrix on macOS, Linux, and Windows.  
**Project Type**: Rust workspace CLI plus engine.  
**Existing System Touchpoints**:

- `crates/canon-engine/src/artifacts/contract.rs`
- `crates/canon-engine/src/artifacts/markdown.rs`
- `crates/canon-engine/src/orchestrator/service/mode_review.rs` (shared review/verification execution path)
- `crates/canon-engine/src/orchestrator/service/mode_incident.rs`
- `crates/canon-engine/src/orchestrator/service/mode_migration.rs`
- `defaults/embedded-skills/canon-review/skill-source.md`
- `defaults/embedded-skills/canon-verification/skill-source.md`
- `defaults/embedded-skills/canon-incident/skill-source.md`
- `defaults/embedded-skills/canon-migration/skill-source.md`
- `.agents/skills/canon-review/SKILL.md`
- `.agents/skills/canon-verification/SKILL.md`
- `.agents/skills/canon-incident/SKILL.md`
- `.agents/skills/canon-migration/SKILL.md`
- `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- `docs/templates/canon-input/review.md`
- `docs/templates/canon-input/verification.md`
- `docs/templates/canon-input/incident/brief.md`
- `docs/templates/canon-input/migration/brief.md`
- `docs/examples/canon-input/review-db-migration.md`
- `docs/examples/canon-input/verification-e2e-flakiness.md`
- `docs/examples/canon-input/incident/brief.md`
- `docs/examples/canon-input/migration/brief.md`
- `docs/guides/modes.md`
- `ROADMAP.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `tests/contract/`
- `tests/integration/`

**Performance Goals**: N/A for throughput; the slice optimizes authored-packet fidelity, reviewability, and contract honesty rather than runtime speed.  
**Constraints**: preserve artifact file names, gate semantics, publish roots, and recommendation-only posture; preserve review/verification blocked-result behavior; keep `Language/Version`, `Primary Dependencies`, `Storage`, and `Project Type` on one line for stable agent-context updates; avoid scope expansion beyond the four targeted modes plus release/docs sync.  
**Scale/Scope**: four target modes, one shared contract surface, one renderer file, three orchestrator service files, four embedded skills plus mirrored `.agents` copies, four starter templates, four worked examples, one guide, roadmap/changelog/version surfaces, and focused contract/renderer/run/docs tests.

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

Post-design re-check: the completed planning artifacts preserve the declared change mode, bounded-impact scope, explicit invariants, and separated validation ownership. No constitution deviations were introduced during planning.

## Project Structure

### Documentation (this feature)

```text
specs/020-authoring-specialization-completion/
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
│               ├── mode_review.rs
│               ├── mode_incident.rs
│               └── mode_migration.rs
└── canon-cli/

defaults/
└── embedded-skills/
    ├── canon-review/skill-source.md
    ├── canon-verification/skill-source.md
    ├── canon-incident/skill-source.md
    ├── canon-migration/skill-source.md
    └── canon-shared/references/runtime-compatibility.toml

.agents/
└── skills/
    ├── canon-review/SKILL.md
    ├── canon-verification/SKILL.md
    ├── canon-incident/SKILL.md
    ├── canon-migration/SKILL.md
    └── canon-shared/references/runtime-compatibility.toml

docs/
├── templates/canon-input/
│   ├── review.md
│   ├── verification.md
│   ├── incident/brief.md
│   └── migration/brief.md
├── examples/canon-input/
│   ├── review-db-migration.md
│   ├── verification-e2e-flakiness.md
│   ├── incident/brief.md
│   └── migration/brief.md
└── guides/modes.md

tests/
├── contract/
└── integration/
```

**Structure Decision**: Extend the existing Rust workspace and current Canon documentation surfaces rather than introducing a new crate, mode, or parallel artifact hierarchy. The work stays centered on artifact contracts, renderer behavior, authored-source handoff, skill/template/example synchronization, release/docs surfaces, and focused tests.

## Complexity Tracking

No constitution violations. No deviations require justification.
