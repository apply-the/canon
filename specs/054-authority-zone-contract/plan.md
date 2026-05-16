# Implementation Plan: Authority Zone Contract

**Branch**: `054-authority-zone-contract` | **Date**: 2026-05-15 | **Spec**: [/Users/rt/workspace/apply-the/canon/specs/054-authority-zone-contract/spec.md](/Users/rt/workspace/apply-the/canon/specs/054-authority-zone-contract/spec.md)
**Input**: Feature specification from `/specs/054-authority-zone-contract/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Stabilize Canon's first authority-zone contract by introducing the
`authority-governance-v1` semantic envelope, defining required versus optional
fields for downstream consumers, extending the machine-facing governance
adapter and governed packet metadata surfaces, and documenting the authority,
persona, and stage-role-hint boundary so Boundline can fail closed without
Canon drifting into runtime orchestration. The first slice stays inside the
existing policy, mode, artifact metadata, integration-doc, and governance
adapter surfaces, bumps Canon from `0.53.0` to `0.54.0`, and closes with docs,
changelog, focused tests, clippy, formatting, and modified-file coverage
validation at 95% or higher.

## Governance Context

**Execution Mode**: architecture  
**Risk Classification**: systemic-impact; this slice defines a cross-repo Canon-owned contract that downstream runtimes will rely on for governance compatibility and fail-closed behavior  
**Scope In**:
- a stable `authority-governance-v1` contract line for governed packets and machine-facing surfaces
- Canon-owned `authority_zone`, `change_class`, `intended_persona`, `approval_state`, `packet_readiness`, and `risk` semantics
- optional additive provenance fields and advisory-only `stage_role_hints`
- integration docs, decision logging, tests, changelog, and release-surface updates that keep the contract coherent

**Scope Out**:
- runtime council composition, adjudication, or stop-transition logic for downstream runtimes
- provider or model routing directives
- reviewer assignment or domain-expert selection
- turning Canon into an operator-facing orchestration runtime above the existing governance boundary

**Invariants**:

- Canon remains the semantic authority for governed posture and MUST NOT become the runtime orchestrator for downstream delivery systems.
- Required `authority-governance-v1` fields stay fail-closed for consumers, while optional additive metadata stays ignorable without breaking compatible packets.
- `stage_role_hints` remain advisory metadata and MUST NOT become executable runtime directives.

**Decision Log**: `specs/054-authority-zone-contract/decision-log.md`  
**Validation Ownership**: Implementation updates Canon-owned domain types, packet metadata, adapter projection, docs, and release surfaces; validation comes from targeted unit and contract tests, documentation review, `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, executable test coverage, and modified-file coverage checks.  
**Approval Gates**: Human review of the contract line, adapter projection changes, and integration documentation before merge.

## Technical Context

**Language/Version**: Rust 1.95.0, edition 2024  
**Primary Dependencies**: Existing workspace dependencies `serde`, `serde_json`, `serde_yaml`, `strum`, `strum_macros`, `thiserror`, `time`, `toml`, `tracing`, `uuid`, and Rust standard-library filesystem and path APIs; no new runtime dependencies planned for this slice  
**Storage**: Canon packet metadata and governed artifacts under `.canon/`, integration docs under `docs/integration/`, repo-facing guides under `docs/`, and feature-local artifacts under `specs/054-authority-zone-contract/`  
**Testing**: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, targeted unit and integration tests for policy, mode, metadata, and governance adapter surfaces, `cargo test --no-run --all-targets`, `cargo nextest run --workspace --all-features` when feasible, and modified-file coverage validation at 95% or higher  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI and library workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/policy.rs`, `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, `crates/canon-engine/src/domain/approval.rs`, `docs/integration/governance-adapter.md`, related governance docs under `docs/`, and the release-facing `CHANGELOG.md`, `README.md`, and `Cargo.toml` surfaces  
**Performance Goals**: Downstream maintainers should be able to recover the required `authority-governance-v1` fields from docs and packet metadata in under 10 minutes, and the added contract metadata must not materially regress governed packet publication or adapter projection performance  
**Constraints**: Preserve the existing Canon/local-first governed runtime identity, keep required versus optional fields explicit, avoid a second publication channel, keep downstream runtime behavior out of Canon, and avoid panic-prone runtime logic outside tests  
**Scale/Scope**: One stable contract line, four authority zones, four change classes, one required-field subset plus bounded optional provenance fields, and a small set of touched domain, doc, and release files

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
specs/054-authority-zone-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── authority-governance-v1-contract.md
│   └── authority-governance-adapter-projection.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── approval.rs
│       │   ├── artifact.rs
│       │   ├── mode.rs
│       │   ├── policy.rs
│       │   └── publish_profile.rs
│       ├── orchestrator/
│       └── persistence/
├── canon-adapters/
└── canon-cli/

docs/
├── guides/
├── integration/
│   └── governance-adapter.md
└── templates/

tests/
├── governance_adapter_surface.rs
├── mode_profiles.rs
├── policy_and_traces.rs
└── contract/

CHANGELOG.md
README.md
Cargo.toml
```

**Structure Decision**: Keep the slice inside the existing Canon policy,
mode, artifact metadata, integration-doc, and adapter surfaces. The feature
clarifies and extends governed semantics that Canon already owns, so no new
crate, runtime channel, or orchestration subsystem is justified.

## Complexity Tracking

No constitution violations identified.
