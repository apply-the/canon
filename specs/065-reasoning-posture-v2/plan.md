# Implementation Plan: Governed Reasoning Posture v2

**Branch**: `065-reasoning-posture-v2` | **Date**: 2026-06-02 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/065-reasoning-posture-v2/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Publish a new Canon-owned `governed_reasoning_posture_v2` contract line that
replaces weak `v1` semantics with typed profile-selection, independence,
confidence-handoff, provenance, and compatibility subcontracts; define
active-versus-legacy coexistence and migration rules between `v1` and `v2`;
publish machine-checkable valid and invalid examples; align the release on
Canon `0.64.0`; and extend executable validation so contract drift, ambiguous
dual-line publication, malformed payloads, and stale release metadata fail
closed before downstream consumers have to infer Canon intent.

## Governance Context

**Execution Mode**: architecture  
**Risk Classification**: systemic-impact; this slice introduces a new cross-repo
Canon-owned contract line with stronger semantics, migration rules, and release
alignment obligations that downstream Boundline governed execution depends on  
**Scope In**:
- a successor stable reasoning-posture contract under
  `governed_reasoning_posture_v2`
- typed producer subcontracts for profile selection, minimum independence,
  confidence handoff, provenance, and compatibility windows
- active-versus-legacy coexistence rules and explicit migration rejection
  behavior between `v1` and `v2`
- machine-checkable valid and invalid posture examples and executable validation
  for contract docs, examples, migration rules, and release metadata
- release-facing truth surfaces required to publish Canon `0.64.0`

**Scope Out**:
- Canon-owned execution loops for debate, reflexion, self-consistency, or other
  reasoning runtimes
- Canon-owned runtime routing, participant selection, provider choice,
  confidence synthesis, trace emission, or final acceptance authority
- hidden Boundline runtime behavior changes disguised as contract evolution
- silent reinterpretation of `governed_reasoning_posture_v1`

**Invariants**:

- Canon remains the semantic producer of reasoning posture, compatibility,
  provenance requirements, and contract-line evolution.
- Boundline remains the runtime consumer responsible for activation, routing,
  provider choice, runtime prompting, confidence synthesis, trace emission, and
  final acceptance authority.
- `governed_reasoning_posture_v1` keeps its current meaning and may coexist
  with `v2` only under an explicit active-versus-legacy publication rule.
- Typed `v2` payloads fail closed on omitted required blocks, contradictory
  selector data, invalid provenance, incomplete confidence handoff data,
  impossible independence requirements, and incompatible version windows.
- Machine-checkable examples are part of the contract surface, not optional
  supplementary documentation.
- Any touched Rust source remains free of unresolved Clippy warnings and meets
  the 95% modified-file coverage target.

**Decision Log**: `specs/065-reasoning-posture-v2/decision-log.md`  
**Validation Ownership**: Canon authors the stable contract, feature-local
brief, example corpus, release-alignment surfaces, and executable validation;
validation evidence comes from contract tests, example-validation fixtures,
release-surface alignment checks, independent review and approval results, and
recorded planning artifacts in `specs/065-reasoning-posture-v2/` including
`validation-report.md`  
**Approval Gates**: Human review of the semantic delta from `v1`, the
active-versus-legacy coexistence rule, and any release-surface claims that bind
Boundline consumers before merge

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2024; Markdown plus machine-checkable contract artifacts in YAML/TOML/JSON  
**Primary Dependencies**: Existing workspace dependencies only (`serde`, `serde_json`, `serde_yaml`, `toml`, `strum`, `strum_macros`, `thiserror`, `time`, `tracing`, `uuid`, `assert_cmd`, `predicates`, `tempfile`) and Rust standard-library filesystem/path APIs; no new external runtime dependencies are planned for this slice  
**Storage**: Repository files only; stable contract docs under `tech-docs/integration/`, feature-local artifacts under `specs/065-reasoning-posture-v2/`, release metadata at repo root and under `assistant/` and `defaults/`, and executable validation under `tests/`  
**Testing**: `cargo test --test governed_reasoning_posture_contract`, targeted example-validation tests for valid and invalid `v2` payload fixtures, `cargo test --no-run --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, and `cargo fmt`  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI and library workspace plus stable contract documentation and release metadata  
**Existing System Touchpoints**:
- `tech-docs/integration/governed-reasoning-posture-contract.md`
- `tests/contract/governed_reasoning_posture_contract.rs`
- `tests/governed_reasoning_posture_contract.rs`
- `Cargo.toml`
- `CHANGELOG.md`
- `README.md`
- `ROADMAP.md`
- `docs/reference/cli.md`
- `assistant/plugin-metadata.json`
- `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- `crates/canon-engine/src/domain/publish_profile/authority.rs`
- `crates/canon-engine/src/domain/publish_profile/publication.rs`
- `crates/canon-engine/src/domain/publish_profile/semantic.rs`

**Performance Goals**: A maintainer can identify the active contract line,
selector rules, provenance rules, confidence-handoff behavior, and migration
boundary in under 10 minutes from repository artifacts alone; validation should
remain linear in fixture size and local file reads, with no network dependency  
**Constraints**: Preserve the Canon-versus-Boundline ownership boundary, forbid
implicit fallback between `v1` and `v2`, keep stable docs canonical, keep
release metadata aligned with Canon `0.64.0`, preserve no-panic/no-magic-literal
Rust rules, and keep machine-checkable examples and executable validation in
lockstep  
**Scale/Scope**: One new contract line, one stable contract doc update, one
feature-local contract brief, one example corpus, one contract test harness,
and one release-alignment slice across repo metadata and user-facing docs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Research Gate

- [x] Execution mode is declared and matches the requested work.
- [x] Risk classification is explicit and appropriate for a cross-repo protocol
  change.
- [x] Scope boundaries and exclusions are recorded.
- [x] Invariants are explicit before design work.
- [x] Required artifacts and validation owners are identified.
- [x] Decision logging is planned and linked to durable artifacts.
- [x] Validation planning separates contract generation from independent
  verification.
- [x] Human review checkpoints are named for this systemic-impact slice.
- [x] No constitution deviations require Complexity Tracking.

### Post-Design Re-check

- [x] `research.md` resolves the contract-line, selector, coexistence,
  confidence-handoff, provenance, independence, and release-surface decisions.
- [x] `data-model.md` defines the typed `v2` entities, validation rules, and
  relationships needed for design and implementation planning.
- [x] `contracts/` captures the main `v2` interface shape, migration contract,
  and required machine-checkable example corpus.
- [x] `quickstart.md` defines the maintainer-facing validation path for the new
  contract line.
- [x] The design artifacts preserve the Canon-versus-Boundline ownership
  boundary and keep validation layered and fail closed.

## Project Structure

### Documentation (this feature)

```text
specs/065-reasoning-posture-v2/
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
│   ├── governed-reasoning-posture-v2.md
│   ├── governed-reasoning-posture-v2-examples.md
│   └── governed-reasoning-posture-v2-migration.md
└── tasks.md
```

### Source Code (repository root)

```text
tech-docs/
└── integration/
    └── governed-reasoning-posture-contract.md

tests/
├── contract/
│   └── governed_reasoning_posture_contract.rs
└── governed_reasoning_posture_contract.rs

crates/
└── canon-engine/
    └── src/
        └── domain/
            └── publish_profile/
                ├── authority.rs
                ├── publication.rs
                └── semantic.rs

assistant/
└── plugin-metadata.json

defaults/
└── embedded-skills/
    └── canon-shared/
        └── references/
            └── runtime-compatibility.toml

docs/
└── reference/
    └── cli.md

README.md
ROADMAP.md
CHANGELOG.md
Cargo.toml
```

**Structure Decision**: Keep the feature inside the existing stable
integration-doc, contract-test, release-metadata, and publish-profile surfaces.
The contract remains canonical in `tech-docs/integration/`, the feature-local
contracts exist for planning and review, executable validation stays under
`tests/contract/`, and release-facing truth surfaces remain at repo root and in
existing docs rather than in a new subsystem.

## Complexity Tracking

No constitution violations identified.
