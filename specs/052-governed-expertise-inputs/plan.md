# Implementation Plan: Governed Expertise Inputs

**Branch**: `052-governed-expertise-inputs` | **Date**: 2026-05-14 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/052-governed-expertise-inputs/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Stabilize a Canon-owned expertise-input contract for `domain-language` and
`domain-model` outputs by adding a dedicated integration contract, a source-level
classification surface aligned with current mode profiles and publication
lineage, and focused docs and tests so Boundline can consume governed expertise
inputs without Canon choosing runtime roles. The first slice stays inside the
existing mode, publish-profile, and integration-doc surfaces, bumps Canon from
`0.51.0` to `0.52.0`, and closes with focused tests, clippy, formatting, and
95% coverage on all modified files.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact; this slice changes Canon-owned
consumer-facing contract semantics across repos without changing Canon runtime
authority or moving selection logic into Canon.
**Scope In**:
- a stable Canon contract for governed expertise inputs
- source-level expertise classification aligned with Canon mode semantics
- publication and lineage rules for identifying expertise inputs
- focused tests and docs that keep contract and source behavior in sync

**Scope Out**:
- runtime role selection, expert-pack activation, or provider routing
- a Canon runtime registry, marketplace, or plugin distribution system
- new publish destinations outside current project-memory and evidence-facing
  surfaces
- Boundline planning or inspection behavior

**Invariants**:

- Canon remains the semantic producer of governed expertise inputs
- Boundline remains the owner of runtime role selection and pack activation
- expertise inputs reuse Canon's existing publication and lineage semantics
- expertise inputs remain readable governed artifacts, not opaque runtime bundles

**Decision Log**: `specs/052-governed-expertise-inputs/decision-log.md`  
**Validation Ownership**: Implementation updates Canon-owned docs and
source-level classification surfaces; validation comes from contract review,
targeted unit tests, `cargo fmt --all`, `cargo clippy --workspace --all-targets
--all-features -- -D warnings`, executable test coverage, and modified-file
coverage checks.  
**Approval Gates**: Human review of the stable expertise-input contract and the
source-level classification changes before merge.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2024  
**Primary Dependencies**: Existing workspace dependencies including `serde`, `serde_json`, `thiserror`, `toml`, `tracing`, `uuid`, and standard-library types; no new runtime dependencies planned for this slice  
**Storage**: `.canon/` packet metadata, repo-visible docs under `docs/integration/`, existing project-memory and evidence-facing publication surfaces, and feature-local spec artifacts under `specs/052-governed-expertise-inputs/`  
**Testing**: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, targeted unit tests for expertise classification and publish-profile helpers, `cargo test --no-run --all-targets`, `cargo nextest run --workspace --all-features` when feasible, and modified-file coverage validation at 95% or higher  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI and library workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `docs/integration/project-memory-promotion-contract.md`, new expertise-input integration docs under `docs/integration/`, and feature-local contracts under `specs/052-governed-expertise-inputs/contracts/`  
**Performance Goals**: No material regression in publish-path responsiveness and no increase in ambiguity for downstream consumers classifying supported expertise inputs  
**Constraints**: Preserve Canon's artifact-first boundary, avoid a second packet or runtime channel, keep Boundline behavior out of Canon, keep unsupported expertise kinds fail-closed, and avoid panic-based runtime error handling outside tests  
**Scale/Scope**: Two initial expertise kinds, a small set of touched source files, one stable contract line, and one focused source-level classification surface for downstream consumers

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
specs/052-governed-expertise-inputs/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── governed-expertise-input-contract.md
│   └── expertise-classification-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── mode.rs
│       │   └── publish_profile.rs
│       └── orchestrator/
│           └── publish.rs

docs/
└── integration/
    ├── governed-expertise-input-contract.md
    └── project-memory-promotion-contract.md

specs/
└── 052-governed-expertise-inputs/
```

**Structure Decision**: Keep the slice inside the existing Canon mode,
publish-profile, publish-orchestration, and integration-doc surfaces. The
feature narrows and stabilizes existing Canon semantics for downstream
consumers, so no new crate, registry, or publish subsystem is justified.

## Complexity Tracking

No constitution violations identified.
