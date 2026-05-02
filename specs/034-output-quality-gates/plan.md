# Implementation Plan: Output Quality Gates

**Branch**: `034-output-quality-gates` | **Date**: 2026-05-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/034-output-quality-gates/spec.md`

## Summary

Deliver feature 034 as one end-to-end slice by introducing a shared
output-quality assessment rooted in the existing clarity and placeholder-aware
engine helpers, exposing explicit `structurally-complete`,
`materially-useful`, and `publishable` posture through inspect and summary
surfaces, tightening fallback artifacts that still sound stronger than the
authored evidence, and synchronizing `0.34.0` version anchors, skill mirrors,
docs, roadmap cleanup, coverage, `cargo clippy`, and `cargo fmt` around that
contract.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact because the feature changes shared
runtime posture across inspect results, summary language, packet rendering,
release-facing guidance, and validation expectations for multiple governed
mode families without changing persistence or mode inventory  
**Scope In**: 034 planning artifacts; shared output-quality assessment in the
engine; inspect, summary, and artifact posture updates; fallback tightening for
targeted packet families; mirrored skill, template, and docs synchronization;
`0.34.0` version alignment; roadmap cleanup; explicit docs and changelog
closeout; and focused Rust coverage plus `cargo clippy` and `cargo fmt`
validation for touched files  
**Scope Out**: new modes, adapters, or external integrations; changes to
`.canon/` storage, publish destinations, run identity, or approval targets;
publish command semantic changes beyond better posture reporting; and splitting
runtime and authoring work into separate deliveries

**Invariants**:

- Canon MUST preserve explicit honesty markers such as `## Missing Authored Body`, `## Missing Evidence`, blocked posture, unsupported posture, and unresolved findings.
- Canon MUST NOT treat heading presence or section count alone as evidence that a packet is materially useful or publishable.
- Canon MUST keep materially closed decisions distinct from shallow reasoning so it does not fabricate alternatives to satisfy heuristics.
- Existing `.canon/` storage, publish semantics, approval flows, and recommendation-only posture MUST remain unchanged.

**Decision Log**: `specs/034-output-quality-gates/decision-log.md`  
**Validation Ownership**: Generation happens through planning artifacts,
runtime code changes, skill and documentation updates, and targeted tests on
the feature branch; validation happens through focused engine and contract
tests, coverage review for touched Rust files, skill-sync checks, lint and
format commands, and a separate final review of emitted packet posture.  
**Approval Gates**: Explicit maintainer ownership because this is
systemic-impact work, plus repository quality gates (`cargo fmt --check`,
`cargo clippy --workspace --all-targets --all-features -- -D warnings`,
focused tests, coverage evidence for touched Rust files, and
`/bin/bash scripts/validate-canon-skills.sh`); no new runtime approval gate is
introduced.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused engine and contract tests, integration checks for skill-sync or version anchors, and `/bin/bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with shared orchestrator services, markdown artifact rendering, embedded skill sources, mirrored `.agents` skills, and repository documentation artifacts  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/service/clarity.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/service/context_parse.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-cli/src/output.rs`, `tests/contract/inspect_clarity.rs`, targeted engine tests under `crates/canon-engine/src/orchestrator/service/`, `defaults/embedded-skills/`, `.agents/skills/`, `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, publication guides, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, and runtime-compatibility references  
**Performance Goals**: Preserve current local CLI responsiveness, add no required network round trips, and avoid new persistence or multi-pass artifact generation flows beyond the existing mode pipeline  
**Constraints**: Reuse existing clarity and placeholder-aware helpers where possible; keep non-target mode behavior stable unless shared posture must change; avoid brittle repo-doc prose tests; ship version, docs, roadmap cleanup, and validation in the same feature; and provide focused automated coverage for every touched Rust file  
**Scale/Scope**: One cross-cutting feature spanning shared engine services, artifact renderers, inspect and summary output, mirrored skills, repository docs, release anchors, and targeted regression coverage across the current governed mode set

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
specs/034-output-quality-gates/
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
crates/canon-engine/src/
├── artifacts/
│   └── markdown.rs
├── orchestrator/
│   └── service.rs
└── orchestrator/service/
    ├── clarity.rs
    ├── context_parse.rs
    ├── inspect.rs
    └── summarizers.rs

crates/canon-cli/src/
└── output.rs

defaults/embedded-skills/
├── canon-inspect-clarity/skill-source.md
└── canon-shared/references/
    ├── output-shapes.md
    └── runtime-compatibility.toml

.agents/skills/
├── canon-inspect-clarity/SKILL.md
└── canon-shared/references/
    ├── output-shapes.md
    └── runtime-compatibility.toml

docs/
├── guides/
│   ├── modes.md
│   ├── publishing-to-scoop.md
│   └── publishing-to-winget.md
├── examples/canon-input/
└── templates/canon-input/

tests/
├── contract/
│   └── inspect_clarity.rs
└── integration/
    └── skills_bootstrap.rs
```

**Structure Decision**: Keep the existing Rust workspace, shared orchestrator
services, renderer helpers, embedded skill sources, mirrored `.agents` skills,
and repository documentation layout. Implement 034 by introducing one shared
output-quality classification seam and threading it through existing inspect,
summary, artifact, and release-alignment surfaces rather than adding a new
crate or persistence layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.
