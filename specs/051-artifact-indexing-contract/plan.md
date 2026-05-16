# Implementation Plan: Artifact Indexing Contract

**Branch**: `051-artifact-indexing-contract` | **Date**: 2026-05-14 | **Spec**: [/Users/rt/workspace/apply-the/canon/specs/051-artifact-indexing-contract/spec.md](/Users/rt/workspace/apply-the/canon/specs/051-artifact-indexing-contract/spec.md)
**Input**: Feature specification from `/specs/051-artifact-indexing-contract/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Stabilize a Canon-owned artifact indexing contract by consolidating indexable
artifact metadata across existing publish surfaces, resolving the undefined
`safety-net packets` vocabulary, documenting the metadata carrier and discovery
rule for each supported artifact class, and versioning compatibility rules
without moving runtime indexing or orchestration into Canon. The first slice
extends the existing normative contract at
`docs/integration/project-memory-promotion-contract.md` rather than creating a
second stable authority surface, stays inside existing publish-profile,
artifact, and publish projection surfaces, bumps Canon to `0.55.0`, and closes
with docs review, targeted tests, clippy, formatting, and coverage on modified
files.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact; this slice changes Canon-owned
consumer-facing artifact metadata and contract clarity without changing Canon
runtime authority or moving Boundline behavior into Canon.
**Scope In**:
- Canon-owned stable contract text for indexable artifact classes and metadata
- explicit required versus optional indexing metadata
- resolution of ambiguous `safety-net packets` vocabulary
- additive versus breaking compatibility policy for artifact indexing metadata
- targeted producer-side touchpoints that emit or validate the stabilized
  metadata shape

**Scope Out**:
- Boundline runtime indexing, context assembly, or stop semantics
- Canon as a runtime registry or delivery orchestrator
- new publish destinations beyond current Canon docs and evidence surfaces
- councils, review policy, or adaptive governance behavior

**Invariants**:

- Canon remains the semantic producer of published artifact metadata
- `.canon/` remains the authoritative governed runtime and evidence store
- consumers may pin Canon contracts, but Canon does not define consumer control flow
- repo-visible docs remain human-readable even when machine-readable indexing metadata is clarified

**Decision Log**: `specs/051-artifact-indexing-contract/decision-log.md`  
**Validation Ownership**: The implementation updates Canon-owned docs and any
targeted producer-side metadata emitters or validators; validation comes from
contract review, focused unit tests, `cargo fmt --all`, `cargo clippy
--workspace --all-targets --all-features -- -D warnings`, executable publish
path checks, and coverage on modified files.  
**Approval Gates**: Human review of the Canon-owned contract text and any
producer-side metadata changes before merge.

## Technical Context

**Language/Version**: Rust 1.95.0, edition 2024  
**Primary Dependencies**: Existing workspace dependencies including `serde`, `serde_json`, `sha2`, `time`, `thiserror`, `toml`, `tracing`, and `uuid`; no new runtime dependencies planned for this slice  
**Storage**: `.canon/` runtime artifacts, repo-visible docs under `docs/integration/project-memory-promotion-contract.md` and other evidence-facing published surfaces, plus feature-local spec artifacts under `specs/051-artifact-indexing-contract/`  
**Testing**: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, targeted unit tests for publish-profile and artifact metadata behavior, `cargo test --no-run --all-targets`, `cargo nextest run --workspace --all-features`, and file-scoped coverage validation for modified files  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI and library workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/publish_profile.rs`, `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `docs/integration/project-memory-promotion-contract.md`, and prior contract material from `specs/050-project-memory-control/` as supporting context only  
**Performance Goals**: No material regression in publish-path responsiveness and no increase in consumer ambiguity about artifact semantics  
**Constraints**: Preserve Canon producer boundaries, avoid turning Canon into a runtime registry, keep Boundline behavior out of scope, avoid incompatible metadata drift within a contract line, and maintain readable repo-visible artifacts  
**Scale/Scope**: One stable contract line, a small number of existing producer-side metadata surfaces, and one feature-local contract bundle for downstream consumers

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
specs/051-artifact-indexing-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── artifact-indexing-contract.md
│   └── evidence-block-metadata-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── artifact.rs
│       │   └── publish_profile.rs
│       └── orchestrator/
│           └── publish.rs

docs/
└── integration/
  └── project-memory-promotion-contract.md

specs/
└── 051-artifact-indexing-contract/
```

**Structure Decision**: Keep the slice inside the existing Canon workspace,
publish-metadata domain types, publish orchestration, and stable integration
docs. No new crate, runtime service, or registry surface is justified because
the feature narrows and version-stabilizes existing producer semantics rather
than adding a new runtime subsystem.

## Complexity Tracking

No constitution violations identified.
