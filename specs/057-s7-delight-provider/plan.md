# Implementation Plan: Canon S7 Delight Provider Contracts

**Branch**: `PLACEHOLDER` | **Date**: PLACEHOLDER | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/057-s7-delight-provider/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Define one stable Markdown integration contract that enumerates the governed
artifact classes, metadata requirements, schema version, compatibility
signaling rules, and degradation conditions under which Boundline S7 may
consume Canon-governed outputs. The contract is a documentation-only artifact;
the stable contract uses six consumer-facing artifact classes (packets,
approval states, readiness signals, security findings, audit findings, and
promotion references) while existing Canon Rust types remain the normative
schema anchors behind those contract categories;
existing Rust types (`AuthorityGovernanceV1Envelope`, `AdaptiveGovernanceV1Envelope`,
`SemanticArtifactDescriptor`, `ExpertiseInputMetadata`) serve as the normative
schema reference without modification. No new runtime Rust code is planned.

## Governance Context

**Execution Mode**: Architecture (Canon runtime boundary contract)
**Risk Classification**: Systemic-Impact — misalignment of this contract
affects all downstream S7 consumer surfaces and Canon governance authority.
**Scope In**: Canon's delight-provider contract definition (artifact classes,
metadata requirements, schema version, compatibility signaling, deprecation
procedure); stable integration document at `docs/integration/`; feature-local
contract brief at `specs/057-s7-delight-provider/contracts/`.
**Scope Out**: Boundline UX, CLI rendering, chat-assistant command naming,
operator-facing explanation vocabulary, Boundline's runtime decision logic or
delight orchestration. No new Rust types or runtime code.

**Invariants**:

- Canon remains the authoritative owner of governance semantics, packets,
  approval states, and promotion references. S7 is a bounded consumer only.
- S7 consumption from Canon MUST be explicitly contracted; no implicit or
  ambient Canon semantics become available to Boundline delight surfaces.
- Every governed artifact class in the contract MUST carry a compatibility
  contract line, a schema version reference, and degradation conditions.
- Boundline MUST support a degraded-state outcome when any contracted Canon
  artifact is missing, stale, incompatible, or outside the contracted scope.
- Contract extension requires explicit amendment documented in both Canon and
  Boundline specifications before any new artifact class becomes authorized.
- The stable contract vocabulary is consumer-facing and may group multiple
  Canon metadata shapes beneath one contracted artifact class when that keeps
  the Canon-to-Boundline boundary explicit and reviewable.

**Decision Log**: [specs/057-s7-delight-provider/decision-log.md](decision-log.md)
**Validation Ownership**: Contract generation owned by Canon; independent
validation by cross-team review (Canon + Boundline); structural validation
by schema cross-reference check; logical validation by artifact-class review.
Validation results recorded in [specs/057-s7-delight-provider/validation-report.md](validation-report.md).
**Approval Gates**: Systemic-Impact requires explicit human ownership sign-off
before the stable integration contract lands in `docs/integration/`.
**Implementation Confirmation**: Rechecked on 2026-05-17 against
`specs/057-s7-delight-provider/tasks.md`; architecture mode, systemic-impact
approval ownership, six-class contract vocabulary, and the stable-doc gate all
remain in force for implementation.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024 (existing types only, no new Rust code);
Markdown for all contract artifacts.
**Primary Dependencies**: Existing workspace crates `canon-engine`, `canon-cli`,
`canon-adapters`; `serde`, `serde_json` output contracts already in place.
**Storage**: Repository files only; no new persistent schema.
**Testing**: Contract cross-reference checks (Markdown/structural); `cargo test`
for any touched Rust files; independent cross-team review.
**Target Platform**: Repository-local Markdown + `docs/integration/` stable doc.
**Project Type**: Integration contract documentation.
**Existing System Touchpoints**:
- `crates/canon-engine/src/domain/publish_profile.rs` — defines
  `AuthorityGovernanceV1Envelope`, `AdaptiveGovernanceV1Envelope`,
  `SemanticArtifactDescriptor`, `ExpertiseInputMetadata`,
  `SEMANTIC_ARTIFACT_CONTRACT_LINE_V1`, and related constants. These are the
  normative Rust schema anchors referenced by the new contract.
- `docs/integration/project-memory-promotion-contract.md` — normative stable
  document for promotion semantics; contract format and authority rules are
  reused here.
- `docs/integration/governed-expertise-input-contract.md` — normative stable
  document for expertise-input semantics; reuse of `current_contract_line: v1`
  pattern and classification rule format.
- `docs/integration/governance-adapter.md` — JSON envelope examples for
  `authority-governance-v1` and `adaptive-governance-v1`; reused as
  reference metadata shapes for the delight-provider contract.
**Performance Goals**: N/A (documentation artifact).
**Constraints**: Contract MUST NOT require changes to existing Rust types.
**Scale/Scope**: One stable integration document + one feature-local brief.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work — Architecture
- [x] Risk classification is explicit and autonomy is appropriate — Systemic-Impact, human gate required before docs/integration/ landing
- [x] Scope boundaries and exclusions are recorded — Scope In/Out in Governance Context
- [x] Invariants are explicit before implementation — five invariants above
- [x] Required artifacts are identified — decision-log.md, validation-report.md, contracts/, docs/integration/delight-provider-contract.md
- [x] Decision logging is planned and linked — decision-log.md
- [x] Validation plan separates generation from validation — Canon authors, cross-team validates
- [x] Declared-risk approval checkpoints named — human gate before docs/integration/ landing
- [x] Constitution deviations documented — none required

## Project Structure

### Documentation (this feature)

```text
specs/057-s7-delight-provider/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── decision-log.md
├── validation-report.md
└── contracts/
    └── delight-provider-contract.md
```

### Published Integration Artifact

```text
docs/integration/
└── delight-provider-contract.md    ← stable integration contract (after approval gate)
```

**Structure Decision**: Documentation-only feature. All design artifacts land
under `specs/057-s7-delight-provider/`. The stable integration document lands
under `docs/integration/` after the Systemic-Impact human approval gate.

## Complexity Tracking

> No constitution deviations required for this feature.

### Source Code (repository root)

No new production source code files are planned. The only Rust implementation
work is the contract validation harness in `tests/contract/`, while existing
Rust types in `crates/canon-engine/src/domain/publish_profile.rs` remain the
normative schema anchors for the contract without modification.
