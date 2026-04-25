# Implementation Plan: High-Risk Operational Programs

**Branch**: `014-high-risk-ops` | **Date**: 2026-04-25 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/014-high-risk-ops/spec.md`

## Summary

Promote `incident` and `migration` from modeled skeletons to full-depth,
governed operational modes without weakening Canon's recommendation-only
posture or existing validation model. The implementation will reuse existing
mode metadata, publish paths, and gate families, then complete the missing
runtime surfaces: mode dispatch, artifact contracts, markdown rendering,
status/summary behavior, inspect/publish readability, tests, and skill/docs
guidance.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact. The work completes the remaining
modeled operational modes and changes cross-cutting runtime behavior around
gating, readiness posture, and artifact expectations.  
**Scope In**: `incident` and `migration` runtime handlers, gate evaluation,
artifact contracts, markdown rendering, status/summary surfaces,
inspect/publish/readability behavior, docs, defaults, skills, and focused test
coverage required to make both modes honest and runnable.  
**Scope Out**: new modes beyond `incident` and `migration`, autonomous
operational execution, protocol/packaging work, and output-polish changes to
unrelated delivered modes.

**Invariants**:

- High-risk operational packets must expose blast radius, containment,
  compatibility, sequencing, fallback, and evidence gaps explicitly rather
  than implying unsupported readiness.
- `incident` and `migration` must remain recommendation-only in v0.x; no
  privileged mutation or live-system action may be automated by this feature.
- Validation ownership must remain split: generation creates packets and
  contracts, while structural, logical, and independent validation stay
  separately evidenced.
- Existing delivered modes must not lose gate enforcement, traceability,
  publishability, or artifact quality as a side effect of this work.

**Decision Log**: `specs/014-high-risk-ops/decision-log.md`  
**Validation Ownership**: Generation updates runtime dispatch, contracts,
renderers, tests, docs, and skill guidance; validation is closed separately by
contract/runtime tests, publish/readability checks, and an independent review
of emitted incident and migration packets recorded in `validation-report.md`.  
**Approval Gates**: Systemic-impact work requires explicit human ownership;
feature delivery must preserve and implement the mode gate families already
declared for `incident` (`Risk`, `IncidentContainment`, `Architecture`,
`ReleaseReadiness`) and `migration` (`Exploration`, `Architecture`,
`MigrationSafety`, `Risk`, `ReleaseReadiness`).

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`  
**Storage**: Local filesystem under `.canon/` for runtime artifacts and evidence, plus published markdown under `docs/incidents/` and `docs/migrations/`  
**Testing**: `cargo test`, `cargo nextest run`, targeted contract/integration/runtime coverage suites under `tests/`  
**Target Platform**: Cross-platform local CLI workflows on macOS, Linux, and Windows  
**Project Type**: Rust CLI + engine workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/{mode,gate}.rs`, `crates/canon-engine/src/modes/{incident,migration}.rs`, `crates/canon-engine/src/artifacts/{contract,markdown}.rs`, `crates/canon-engine/src/orchestrator/{classifier,publish}.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/persistence/store.rs`, `defaults/methods/{incident,migration}.toml`, `defaults/embedded-skills/canon-{incident,migration}/skill-source.md`, `.agents/skills/canon-{incident,migration}/SKILL.md`, `tests/direct_runtime_coverage.rs`, `tests/integration/mode_profiles.rs`, `tests/contract/{inspect_modes,runtime_filesystem,runtime_evidence_contract}.rs`, `README.md`, and `MODE_GUIDE.md`  
**Performance Goals**: Keep packet generation, status summarization, and publish rendering effectively linear in artifact count and bounded input surfaces, with no material slowdown for already delivered modes  
**Constraints**: Preserve recommendation-only posture, preserve existing runtime/publish contracts, expose missing evidence honestly, and avoid regressions to current modes while strengthening high-risk operational gating  
**Scale/Scope**: Two modeled modes, twelve packet artifacts across both modes, one shared runtime plumbing pass, and focused docs/skill/test updates across the existing workspace

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
specs/014-high-risk-ops/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── incident-artifact-contract.md
│   └── migration-artifact-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   └── src/
│       └── output.rs
└── canon-engine/
    └── src/
        ├── artifacts/
        │   ├── contract.rs
        │   └── markdown.rs
        ├── domain/
        │   ├── gate.rs
        │   └── mode.rs
        ├── modes/
        │   ├── incident.rs
        │   └── migration.rs
        ├── orchestrator/
        │   ├── classifier.rs
        │   ├── publish.rs
        │   └── service/
        │       ├── summarizers.rs
        │       └── tests.rs
        └── persistence/
            └── store.rs

defaults/
├── embedded-skills/
│   ├── canon-incident/
│   └── canon-migration/
└── methods/
    ├── incident.toml
    └── migration.toml

.agents/
└── skills/
    ├── canon-incident/
    └── canon-migration/

tests/
├── contract/
│   ├── inspect_modes.rs
│   ├── runtime_evidence_contract.rs
│   └── runtime_filesystem.rs
├── integration/
│   └── mode_profiles.rs
├── direct_runtime_coverage.rs
├── artifact_confinement.rs
└── policy_and_traces.rs

README.md
MODE_GUIDE.md
```

**Structure Decision**: Keep the work inside the existing Canon workspace.
Implementation adds missing operational-mode runtime surfaces rather than
introducing new crates or a new execution architecture.

## Workstreams

1. Promote `incident` and `migration` from skeleton metadata to executable,
   governed runtime paths.
2. Define explicit operational artifact contracts and markdown rendering for
   blast radius, containment, compatibility, sequencing, and fallback.
3. Extend summaries, publish/inspect surfaces, docs, skills, and
   non-regression tests so the modes are honest and reviewable end to end.

## Phase Outcomes

### Phase 0: Research

- Confirm paired delivery for `incident` and `migration` as the roadmap's next
  mode-completion slice.
- Decide whether to retain the current artifact families and deepen them or
  rename them.
- Decide how high-risk runs surface missing evidence, blocks, downgrade
  posture, and readiness without weakening recommendation-only semantics.
- Decide an implementation ordering that minimizes risk and reuse gaps.

### Phase 1: Design

- Define the shared operational data model plus incident-specific and
  migration-specific packet expectations.
- Define artifact contracts for both modes and the quickstart path for real
  packet exercise.
- Record design decisions and the layered validation plan.
- Update agent context from the completed plan.

### Phase 2: Implementation Preparation

- Leave a task-ready design that sequences runtime dispatch/gating first,
  artifact rendering and summaries second, and docs/skills/non-regression
  validation last.

## Complexity Tracking

No constitution deviations are planned.
