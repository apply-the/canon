# Implementation Plan: Governed Reasoning Posture Contract

**Branch**: `058-governed-reasoning-posture-contract` | **Date**: 2026-05-18 | **Spec**: [/Users/rt/workspace/apply-the/canon/specs/058-governed-reasoning-posture-contract/spec.md](/Users/rt/workspace/apply-the/canon/specs/058-governed-reasoning-posture-contract/spec.md)
**Input**: Feature specification from `/specs/058-governed-reasoning-posture-contract/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Define and validate Canon's `governed_reasoning_posture_v1` producer contract,
keep the stable integration doc and the Boundline consumer brief aligned on one
active release window, reconcile release metadata that downstream validation
relies on, and treat the staged gatekeeper module split as maintainability
follow-through that must preserve gate behavior rather than broaden policy
scope.

## Governance Context

**Execution Mode**: architecture  
**Risk Classification**: systemic-impact; this slice defines a cross-repo Canon-owned contract that downstream Boundline reasoning activation relies on and it also touches release metadata and runtime-adjacent validation surfaces  
**Scope In**:
- Canon-owned `governed_reasoning_posture_v1` contract identity, required fields, supported vocabulary, and compatibility rules
- the active Boundline and Canon release window used by the first downstream consumer
- executable contract validation and release-surface alignment checks for workspace metadata, plugin manifests, and runtime-compatibility references
- the stable integration doc under `docs/integration/` and the feature-local Canon contract brief under `specs/058-governed-reasoning-posture-contract/contracts/`
- bounded gatekeeper code organization follow-through for the already touched runtime surface, provided behavior stays stable

**Scope Out**:
- Boundline runtime activation, participant routing, confidence handling, trace emission, or operator-facing execution decisions
- Canon-owned execution loops for self-consistency, debate, reflexion, or adjudication
- new gate policy semantics, approval requirements, or runtime governance decisions unrelated to the existing gate behavior
- any attempt to let the reasoning-posture contract become a general-purpose downstream orchestration API

**Invariants**:

- Canon remains the semantic producer of reasoning posture, while Boundline remains the runtime consumer.
- Unsupported contract lines, incomplete required fields, or incompatible release windows fail closed.
- The stable integration doc is normative; feature-local planning artifacts may explain or mirror it, but may not supersede it.
- The gatekeeper split is valid only as behavior-preserving maintainability work.
- Release metadata used by downstream validation must stay aligned with the contract this branch declares.

**Decision Log**: `/Users/rt/workspace/apply-the/canon/specs/058-governed-reasoning-posture-contract/decision-log.md`  
**Validation Ownership**: Canon authors the contract, release-alignment surfaces, and gatekeeper maintainability changes; validation comes from executable Canon tests, cross-repo comparison with Boundline `061-reasoning-profile-contracts`, and recorded evidence in `/Users/rt/workspace/apply-the/canon/specs/058-governed-reasoning-posture-contract/validation-report.md`  
**Approval Gates**: Human review of the cross-repo contract boundary and of any touched runtime-adjacent gatekeeper behavior before merge

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2024; Markdown for contract and planning artifacts  
**Primary Dependencies**: Existing workspace dependencies only (`serde`, `serde_json`, `serde_yaml`, `strum`, `strum_macros`, `thiserror`, `time`, `toml`, `tracing`, `uuid`, and Rust standard-library filesystem and path APIs); no new runtime dependencies are planned for this slice  
**Storage**: Repository files only; stable docs under `docs/integration/`, release surfaces at repo root and under `assistant/` and `defaults/`, tests under `tests/`, and feature-local planning artifacts under `specs/058-governed-reasoning-posture-contract/`  
**Testing**: `cargo test --test governed_reasoning_posture_contract`, targeted release-surface tests such as `cargo test --test assistant_plugin_packages metadata_paths_and_versions_are_aligned -- --exact`, targeted gatekeeper tests under `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs`, `cargo test --no-run --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, and `cargo fmt`  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI and library workspace plus integration documentation  
**Existing System Touchpoints**:
- `docs/integration/governed-reasoning-posture-contract.md`
- `tests/contract/governed_reasoning_posture_contract.rs`
- `tests/governed_reasoning_posture_contract.rs`
- `assistant/plugin-metadata.json`
- `.claude-plugin/manifest.json`
- `.codex-plugin/plugin.json`
- `.cursor-plugin/manifest.json`
- `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- `README.md`
- `ROADMAP.md`
- `CHANGELOG.md`
- `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/context.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/entrypoints.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/rules.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs`

**Performance Goals**: A maintainer should be able to determine the active contract line and release window in under 10 minutes from repo artifacts alone; gatekeeper behavior on representative mode coverage should remain unchanged after the module split; no material runtime overhead is justified for the maintainability refactor  
**Constraints**: Preserve the Canon/Boundline producer-consumer boundary, avoid panic-prone runtime logic outside tests, keep the gatekeeper split behavior-preserving, and do not leave stale release metadata that causes downstream validation drift  
**Scale/Scope**: One stable contract line, one active release pairing, one feature-local contract brief, one contract-test harness, and one bounded gatekeeper module split

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work.
- [x] Risk classification is explicit and autonomy is appropriate for that risk.
- [x] Scope boundaries and exclusions are recorded.
- [x] Invariants are explicit before implementation.
- [x] Required artifacts and owners are identified.
- [x] Decision logging is planned and linked to a durable artifact.
- [x] Validation plan separates generation from validation.
- [x] Declared-risk approval checkpoints are named where required by the risk classification.
- [x] Any constitution deviations are documented in Complexity Tracking; none are required for this slice.

## Project Structure

### Documentation (this feature)

```text
specs/058-governed-reasoning-posture-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── checklists/
│   └── requirements.md
├── contracts/
│   └── governed-reasoning-posture-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
└── canon-engine/
    └── src/
        └── orchestrator/
            ├── gatekeeper.rs
            └── gatekeeper/
                ├── context.rs
                ├── entrypoints.rs
                ├── rules.rs
                └── tests.rs

docs/
└── integration/
    └── governed-reasoning-posture-contract.md

tests/
├── contract/
│   └── governed_reasoning_posture_contract.rs
└── governed_reasoning_posture_contract.rs

assistant/
└── plugin-metadata.json

defaults/
└── embedded-skills/
    └── canon-shared/
        └── references/
            └── runtime-compatibility.toml

.claude-plugin/
.codex-plugin/
.cursor-plugin/
README.md
ROADMAP.md
CHANGELOG.md
Cargo.toml
Cargo.lock
```

**Structure Decision**: Keep the feature inside the existing Canon documentation,
validation, release-metadata, and gatekeeper surfaces. The contract is already
published from `docs/integration/`, the release alignment story already touches
assistant and runtime-compatibility metadata, and the gatekeeper branch work is
best handled as a bounded sibling-module split rather than as a new subsystem.

## Complexity Tracking

No constitution violations identified.
