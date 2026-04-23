# Data Model: Controlled Execution Modes (`implementation` and `refactor`)

## Overview

This feature extends the existing Canon run model rather than introducing a second execution model. The core design centers on richer mode-specific data persisted through existing runtime surfaces: `context.toml`, artifact bundles, invocation policy decisions, and publish outputs.

Folder-backed `canon-input/implementation/` and `canon-input/refactor/` packets may split content into `brief.md`, `source-map.md`, and optional `selected-context.md`. The current-mode `brief.md` remains authoritative; runtime-derived upstream lineage is persisted as provenance-only metadata and does not replace the declared execution or preservation contract.

## Entities

### Implementation Execution Context

- **Purpose**: Captures the machine-checkable execution metadata required for `implementation` runs.
- **Persistence**: Added to `.canon/runs/<RUN_ID>/context.toml` as an optional mode-specific block.
- **Fields**:
  - `mode`: fixed to `implementation`
  - `plan_sources`: references to the prior bounded plan or authored brief that authorizes the work
  - `mutation_bounds`: normalized `MutationBounds`
  - `task_targets`: identifiers or headings for the bounded tasks being executed
  - `safety_net`: one or more `SafetyNetEvidence` records for touched surfaces
  - `execution_posture`: `ExecutionPosture`
  - `rollback_expectations`: optional notes or references required when mutation is recommended
- **Validation Rules**:
  - `plan_sources` must not be empty
  - `mutation_bounds` must be explicit before consequential mutation is recommended
  - `safety_net` is required when the run touches an existing code surface
  - `execution_posture = mutating` is invalid for red-zone or systemic-impact runs in this feature slice

### Refactor Execution Context

- **Purpose**: Captures the preservation-specific metadata required for `refactor` runs.
- **Persistence**: Added to `.canon/runs/<RUN_ID>/context.toml` as an optional mode-specific block.
- **Fields**:
  - `mode`: fixed to `refactor`
  - `preserved_behavior`: references to the externally meaningful behavior that must remain unchanged
  - `structural_rationale`: concise explanation of why the refactor is being performed
  - `refactor_scope`: normalized `MutationBounds`
  - `safety_net`: one or more `SafetyNetEvidence` records for touched surfaces
  - `no_feature_addition_target`: statement or reference describing the forbidden feature-expansion boundary
  - `allowed_exceptions`: explicit, reviewable exceptions if any deviation is authorized
  - `execution_posture`: `ExecutionPosture`
- **Validation Rules**:
  - `preserved_behavior` must not be empty
  - `structural_rationale` must not be empty
  - `no_feature_addition_target` must exist before completion
  - Any `allowed_exceptions` must be explicit and traceable; otherwise semantic or contract drift is blocking

### MutationBounds

- **Purpose**: Machine-checkable declaration of where a run may perform consequential mutation.
- **Persistence**: Stored in mode-specific run context and projected into `InvocationConstraintSet.allowed_paths` for adapter enforcement.
- **Fields**:
  - `declared_paths`: repo-relative files, directories, or contracts the run may touch
  - `owners`: optional human or team owners for the bounded surface
  - `source_refs`: authored input or artifact references from which the bounds were derived
  - `expansion_policy`: fixed to `deny-without-approval` for this feature slice
- **Validation Rules**:
  - `declared_paths` must not be empty for mutating posture
  - broad wildcard or repo-root entries require approval or recommendation-only fallback
  - any adapter request outside `declared_paths` is denied before execution

### SafetyNetEvidence

- **Purpose**: Captures the focused validation evidence that justifies mutation on a touched surface.
- **Persistence**: Referenced in mode-specific context and emitted into mode-specific Markdown artifacts.
- **Fields**:
  - `target_surface`: repo-relative path or contract name covered by the safety net
  - `evidence_kind`: `existing-test`, `new-test`, `regression-check`, `manual-proof-with-approval`, or similar bounded category
  - `provenance`: `pre-existing` or `authored-in-run`
  - `evidence_refs`: artifact references, test names, or validation outputs
  - `status`: `satisfied`, `missing`, or `exception-approved`
- **Validation Rules**:
  - repository-wide coverage alone does not satisfy the record
  - each consequentially touched surface must resolve to at least one evidence record or an explicit approved exception

### ExecutionPosture

- **Purpose**: Represents whether a run is permitted to enact consequential mutation or must stay recommendation-only.
- **Persistence**: Derived from invocation constraints and stored in mode-specific context plus emitted summaries.
- **States**:
  - `mutating`
  - `recommendation-only`
- **Validation Rules**:
  - `recommendation-only` must be selected for red-zone or systemic-impact work in this feature slice
  - `recommendation-only` must be selected when safety-net evidence is missing and no exception is approved

### UpstreamContext

- **Purpose**: Captures provenance from folder-backed carry-forward packets without making earlier packets authoritative for the current run.
- **Persistence**: Added to `.canon/runs/<RUN_ID>/context.toml` as an optional run-level block alongside mode-specific execution metadata.
- **Fields**:
  - `feature_slice`: short statement of the bounded slice being continued in the current run
  - `primary_upstream_mode`: the upstream Canon mode the current packet is primarily continuing from
  - `source_refs`: explicit upstream packet or artifact references from `source-map.md`
  - `carried_forward_items`: decisions, invariants, or other bounded carry-forward items restated for the current run
  - `excluded_upstream_scope`: explicit upstream scope that is intentionally not being continued
- **Validation Rules**:
  - present only when authored inputs provide recognizable upstream lineage markers
  - informative for inspect/evidence surfaces, but never substitutes for missing current-mode requirements
  - must not trigger implicit reads from prior `.canon/` runs or published docs outside the current authored inputs

### Mode Artifact Bundle

- **Purpose**: Names the durable Markdown artifacts required for a completed or recommendation-only run.
- **Persistence**: Existing artifact bundle under `.canon/runs/<RUN_ID>/artifacts/` with explicit contract requirements in `crates/canon-engine/src/artifacts/contract.rs`.
- **Implementation Bundle**:
  - `task-mapping.md`
  - `mutation-bounds.md`
  - `implementation-notes.md`
  - `completion-evidence.md`
  - `validation-hooks.md`
  - `rollback-notes.md`
- **Refactor Bundle**:
  - `preserved-behavior.md`
  - `refactor-scope.md`
  - `structural-rationale.md`
  - `regression-evidence.md`
  - `contract-drift-check.md`
  - `no-feature-addition.md`

## Relationships

- A `RunManifest` identifies the run and its canonical identity.
- A `RunContext` stores generic run metadata plus an optional `UpstreamContext` and an optional `Implementation Execution Context` or `Refactor Execution Context`.
- `MutationBounds` are projected into `InvocationConstraintSet.allowed_paths` so adapter requests can be enforced consistently.
- `SafetyNetEvidence` is referenced by both the mode-specific run context and the corresponding artifact bundle.
- `ExecutionPosture` drives invocation outcomes, summary text, and publish/inspect labeling without changing `RunState`.
- `UpstreamContext` enriches `inspect evidence` and `context.toml` with machine-readable lineage while leaving the current-mode brief as the governing source.

## State Transitions

### Implementation / Refactor Run Lifecycle

1. `Draft` -> `ContextCaptured`: authored inputs are bound, snapshotted, and persisted in `context.toml`
2. `ContextCaptured` -> `Classified`: risk, zone, and system context are established
3. `Classified` -> `Contracted`: mode-specific artifact contract and mode-specific context blocks are materialized
4. `Contracted` -> `Gated`: dedicated implementation/refactor gate evaluators assess inputs, bounds, safety-net evidence, and release readiness
5. `Gated` -> `Executing` or `Verifying`: bounded generation, validation, and any allowed mutation steps proceed
6. `Executing` / `Verifying` -> `Completed`, `Blocked`, or `AwaitingApproval`: final state remains on the existing `RunState` enum while execution posture is expressed separately as `mutating` or `recommendation-only`

## Compatibility Notes

- No new run identity fields are added; canonical display id, UUID, short id, slug, and `@last` handling remain unchanged.
- No new publish directory model is added; publish continues to use `docs/implementation/<RUN_ID>/` and `docs/refactors/<RUN_ID>/` by default.
- No authored-input file is ever rewritten; snapshots remain the durable runtime record.
