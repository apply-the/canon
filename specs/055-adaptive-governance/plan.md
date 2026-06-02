# Implementation Plan: Adaptive Governance Semantics

**Branch**: `055-adaptive-governance` | **Date**: 2026-05-16 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/055-adaptive-governance/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Extend Canon's governed semantic surfaces by defining the S4 governance-state
and rollout-profile vocabulary, preserving `authority-governance-v1` as the
required posture baseline, and introducing an optional `adaptive-governance-v1`
companion contract when machine-readable adaptive semantics are published. The
first implementation slice stays inside existing Canon domain metadata, packet
metadata, adapter documentation, and governed publication surfaces, while
keeping runtime confidence, trust, degradation, escalation, councils, and stop
behavior explicitly outside Canon ownership while preserving approval,
readiness, project-memory, lineage, and promotion-state semantics as Canon-
owned surfaces.

## Governance Context

**Execution Mode**: architecture  
**Risk Classification**: systemic-impact; this slice defines a cross-repo Canon-owned semantic contract that downstream runtimes will rely on for governed delivery behavior  
**Scope In**:
- Canon-owned vocabulary for `advisory`, `catch`, `rule`, and `hook`
- rollout-profile vocabulary for `minimal`, `guided`, `governed`, and `strict`
- compatibility rules between required `authority-governance-v1` posture semantics and optional `adaptive-governance-v1` companion semantics
- preservation of Canon approval, readiness, project-memory, lineage, and promotion-state semantics alongside the optional companion contract
- governed packet metadata, adapter projection docs, tests, and planning artifacts needed to keep the semantic boundary stable

**Scope Out**:
- runtime confidence computation, trust evolution, degradation choice, escalation targets, council assembly, and stop-transition logic for downstream runtimes
- provider or model routing directives
- reviewer assignment, override execution, or human-gate orchestration
- turning Canon into the operator-facing runtime controller for Boundline or another downstream system

**Invariants**:

- Canon remains the semantic authority for governed posture and MUST NOT become the runtime orchestrator for downstream delivery systems.
- `authority-governance-v1` remains the required S3 posture baseline and MUST NOT silently change meaning as part of S4.
- Any `adaptive-governance-v1` companion semantics remain advisory and semantic, and MUST NOT assign runtime councils, confidence, routes, or stop transitions.
- Approval, readiness, project-memory, lineage, and promotion-state semantics remain Canon-owned even when downstream runtimes enforce stronger or weaker governance behavior.

**Decision Log**: `specs/055-adaptive-governance/decision-log.md`  
**Validation Ownership**: Implementation updates Canon-owned semantic models, packet metadata, adapter documentation, and governed publication surfaces; validation comes from targeted unit and contract tests, documentation review, `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, executable test coverage, modified-file coverage checks, and recorded evidence in `specs/055-adaptive-governance/validation-report.md`  
**Approval Gates**: Human review of the contract boundary, packet metadata changes, and adapter documentation before merge

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2024  
**Primary Dependencies**: Existing workspace dependencies `serde`, `serde_json`, `serde_yaml`, `strum`, `strum_macros`, `thiserror`, `time`, `toml`, `tracing`, `uuid`, and Rust standard-library filesystem and path APIs; no new runtime dependencies planned for this slice  
**Storage**: Canon packet metadata and governed artifacts under `.canon/`, integration docs under `tech-docs/integration/`, repo-facing guides under `tech-docs/`, and feature-local planning artifacts under `specs/055-adaptive-governance/`  
**Testing**: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, targeted tests for policy, mode, metadata, publish, and governance adapter surfaces, `cargo test --no-run --all-targets`, `cargo nextest run --workspace --all-features` when feasible, and modified-file coverage validation at 95% or higher  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI and library workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/policy.rs`, `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/`, `tech-docs/integration/governance-adapter.md`, related governance docs under `tech-docs/`, and root release-facing surfaces such as `README.md`, `CHANGELOG.md`, and `Cargo.toml`  
**Performance Goals**: Downstream maintainers should be able to determine the required baseline and optional companion semantics from docs and packet metadata in under 10 minutes, and the additional semantic metadata must not materially regress governed packet publication or adapter projection performance  
**Constraints**: Preserve Canon's semantic-authority role, keep required versus optional contract layers explicit, avoid creating a second publication channel, keep downstream runtime behavior out of Canon, and avoid panic-prone runtime logic outside tests  
**Scale/Scope**: One required posture baseline, one optional adaptive companion contract, four governance-state terms, four rollout-profile terms, and a bounded set of touched domain, adapter, doc, and test files

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work; see Governance Context.
- [x] Risk classification is explicit and autonomy is appropriate for that risk; see Governance Context and Approval Gates.
- [x] Scope boundaries and exclusions are recorded; see Scope In and Scope Out.
- [x] Invariants are explicit before implementation; see Invariants.
- [x] Required artifacts and owners are identified; see Summary, Validation Ownership, and Project Structure.
- [x] Decision logging is planned and linked to a durable artifact; see Decision Log.
- [x] Validation plan separates generation from validation; see Validation Ownership and Technical Context.
- [x] Declared-risk approval checkpoints are named where required by the risk classification; see Approval Gates.
- [x] Any constitution deviations are documented in Complexity Tracking; none are needed for this slice.

## Project Structure

### Documentation (this feature)

```text
specs/055-adaptive-governance/
├── spec.md
├── plan.md
├── decision-log.md
├── research.md
├── data-model.md
├── quickstart.md
├── validation-report.md
├── contracts/
│   ├── adaptive-governance-v1-contract.md
│   └── adaptive-governance-adapter-projection.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── artifact.rs
│       │   ├── mode.rs
│       │   ├── policy.rs
│       │   └── publish_profile.rs
│       ├── orchestrator/
│       │   ├── publish.rs
│       │   ├── service.rs
│       │   └── service/
│       └── persistence/
├── canon-adapters/
└── canon-cli/

tech-docs/
├── governance-semantics-and-authority-zones.md
└── integration/
    └── governance-adapter.md

tests/
├── governance_adapter_surface.rs
├── mode_profiles.rs
├── policy_and_traces.rs
└── contract/

README.md
CHANGELOG.md
Cargo.toml
AGENTS.md
```

**Structure Decision**: Keep the slice inside the existing Canon policy,
mode, artifact metadata, publish, service, and adapter-documentation surfaces.
The feature clarifies and extends governed semantics that Canon already owns,
including approval, readiness, project-memory, lineage, and promotion-state
meaning, so no new crate, runtime channel, or orchestration subsystem is
justified.

## Complexity Tracking

No constitution violations identified.
