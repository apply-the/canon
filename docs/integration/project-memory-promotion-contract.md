# Project Memory Promotion Contract

> **Promoted from**: `specs/048-project-memory-promotion-policy/contracts/boundline-project-memory-promotion-contract.md`
>
> This is the stable Canon integration document. The authoritative feature-local
> source remains the file referenced above. Future revisions follow the
> versioning and change policy defined in this contract.

## Contract Identity

- `contract_version`: `0.1.0`
- `owner`: `canon`
- `known_consumers`:
  - `boundline`
- `authoritative_source`: `specs/048-project-memory-promotion-policy/contracts/boundline-project-memory-promotion-contract.md`
- `stable_doc`: `docs/integration/project-memory-promotion-contract.md`

## Purpose

Define the Canon-owned producer contract for promoting governed packet output
into project-visible knowledge surfaces that consumers can rely on without
redefining Canon publish semantics or turning Canon into the delivery
orchestrator.

## Scope

This contract defines:

- Canon publish-profile expectations for project memory and evidence promotion.
- Canon-owned promotion states and their meanings.
- Canon-owned lineage metadata required on promoted outputs.
- Canon-owned non-destructive update strategies for project-visible documents.
- Consumer-visible compatibility and versioning rules.

This contract does not define:

- Consumer delivery-path selection.
- Consumer stage-planner behavior.
- Consumer assurance-profile logic.
- Consumer governed-stage orchestration policy.
- Canon execution as a delivery orchestrator.

## Ownership Boundary

### Canon Owns

- publish profiles
- promotion policy
- promotion states and their semantics
- lineage metadata shape
- document update strategies
- pending, audit, and evidence publication behavior

### Consumers Own

- delivery paths
- stage planner
- assurance profiles
- governed stage orchestration
- session projection and next-action selection
- consumption of Canon refs and promoted knowledge

### Boundary Rules

- Consumers MUST NOT redefine Canon promotion semantics.
- Canon MUST NOT become the delivery orchestrator.
- Canon-produced project memory remains a promoted projection of governed
  output, not a replacement for `.canon/` runtime state.
- Consumers may consume Canon-promoted knowledge, but they remain responsible
  for deciding when to continue, stop, confirm, or replan.

## Shared Surface Model

### Stable Project Memory Surfaces

- `docs/project/overview.md`
- `docs/project/product-context.md`
- `docs/project/domain-language.md`
- `docs/project/domain-model.md`
- `docs/project/architecture-map.md`
- `docs/project/decision-index.md`
- `docs/project/delivery-map.md`
- `docs/project/operational-context.md`

### Evidence And Index Surfaces

- `docs/evidence/`
- `docs/project/pending-decisions.md`
- `docs/project/open-risks.md`
- `docs/project/audit-log.md`

### Current Canon Default Routing

- Canon's current `project-memory` implementation resolves to one canonical
  stable, pending, risk, or audit surface rather than to a dated packet
  directory.
- Stable and pending publication target the named `docs/project/*.md`
  surfaces above.
- Evidence-bearing publication may also emit supporting artifacts under
  `docs/evidence/<mode>/<RUN_ID>/` while keeping the consumer-facing surface in
  the named `docs/project/*.md` files.

## Publish Profile Contract

- Canon MUST define a `project-memory` publish profile.
- The `project-memory` publish profile may update stable project memory,
  evidence summaries, or index surfaces depending on the promotion policy.
- The profile MUST preserve governed artifacts under `.canon/` and treat
  project-visible output as a projection, not a new source of runtime truth.

## Promotion Policy States

Canon owns the allowed promotion-state vocabulary.

### `auto`

- Eligible output is promoted automatically to the target stable surface.
- Consumers may treat the target as stable project memory.

### `auto-if-approved`

- Output is promoted automatically only when approval state and readiness meet
  Canon policy.
- Consumers may not infer approval; they must read the emitted metadata.
- Canon's current implementation emits `approval_state` as the serialized
  `RunState` label.
- Consumers may treat `auto-if-approved` as stable only when metadata reports
  `approval_state = Completed` and `readiness = complete`; otherwise they MUST
  keep it non-authoritative.

### `pending-index`

- Canon updates pending or audit surfaces only.
- Consumers MUST treat the result as visible but not yet stable project
  memory.

### `index-only`

- Canon records the event in index or audit surfaces without mutating stable
  project-memory targets.
- Consumers MUST NOT treat the result as accepted project knowledge.

### `evidence-only`

- Canon updates evidence-facing output without promoting the result into stable
  project-memory documents.
- Consumers may use the evidence for assurance or review, but not as stable
  architectural or product truth by default.

### `manual`

- Canon records the promotion candidate, but stable publication requires an
  explicit manual action.
- Consumers must not assume a stable projection exists until it does.

## Lineage Metadata Contract

Promoted outputs or their sidecars MUST preserve enough metadata for consumers
to recover source lineage without reopening `.canon/`.

### Required Semantic Fields

- `contract_version`
- `source_run`
- `mode`
- `promotion_state`

### Conditional Decision Fields

- `approval_state`
- `readiness`

`approval_state` and `readiness` MUST be present when the promotion state
cannot be interpreted safely without approval or completion metadata,
including Canon's current `auto-if-approved` state.

### Optional Producer Provenance Fields

- `profile`
- `published_at`
- `update_strategy`
- `source_artifacts`

Canon's current implementation emits these provenance fields, but consumers
MUST NOT rely on them as the minimum semantic gate for understanding whether a
surface is compatible, pending, evidence-only, or stable.

### Minimum Orchestration Contract

- Boundline and any future framework-neutral consumer validate the minimum
  orchestration contract formed by the required semantic fields plus the
  conditional decision fields.
- Optional producer provenance fields may be preserved, surfaced, or reused
  when available, but their absence alone MUST NOT make an otherwise
  interpretable surface incompatible.
- A framework-specific adapter may enforce stricter provenance requirements,
  but that stricter validation lives above this shared consumer contract.

### Metadata Rules

- Metadata MAY live in `packet-metadata.json`, document front matter, managed
  blocks, or another Canon-defined durable sidecar, but Canon owns the shape.
- Canon's current implementation emits file-adjacent metadata sidecars as
  `<surface>.packet-metadata.json` for file-backed surfaces and retains
  `packet-metadata.json` only for explicit directory overrides.
- Consumers may ignore optional producer provenance fields they do not yet use,
  but they MUST preserve the meanings of the required semantic and conditional
  decision fields.
- Consumers MUST tolerate additive metadata fields they do not yet understand.
- Canon MUST NOT remove or silently repurpose required semantic fields,
  conditional decision fields, or the meaning of any emitted provenance field
  without a contract-version change.

## Non-Destructive Update Strategies

Canon owns the strategy vocabulary for modifying project-visible documents.

### `managed-blocks`

- Canon updates only explicitly managed ranges inside a curated document.
- Human-authored content outside the managed range MUST be preserved.

### `proposal-files`

- Canon emits a proposal artifact when safe in-place promotion is not credible.
- Consumers may surface the proposal, but proposal existence is not equivalent
  to accepted project memory.

### `append-only-index`

- Canon appends entries to index or audit surfaces without rewriting existing
  historical entries.
- Consumers may use these surfaces for visibility, not as stable replacement
  for the canonical project-memory document.

## Shared Alignment Points

The Canon owner-side spec and the consumer integration-side spec must stay
aligned on:

- stage taxonomy and mode-to-stage mapping
- target surface categories (`docs/project`, `docs/evidence`, pending/index)
- promotion-state vocabulary and semantics
- lineage metadata field names and meanings
- update-strategy vocabulary and meanings
- compatibility window and pre-1.0 change policy

## Compatibility Rules

- Consumers MUST treat the Canon contract brief as the source of truth for
  promotion semantics.
- The current pre-stable contract line is `0.1.x`; consumers MAY ignore
  additive fields they do not understand within that line, but they MUST NOT
  claim compatibility with `0.2.0+`, `1.x`, or malformed `contract_version`
  values unless they have been updated to do so.
- Consumers MUST reject or explicitly degrade behavior when `contract_version`
  falls outside their documented supported line.

## Pre-1.0 Change Policy

- While the contract remains `0.x`, Canon may introduce incompatible changes by
  publishing a new minor `contract_version`.
- Changes to promotion states, required metadata fields, update-strategy
  meanings, or target surfaces MUST be reflected in the authoritative contract
  brief and this stable integration-doc path.
- Known consumers must not claim compatibility with a new minor or major
  contract version until their integration specs and acceptance criteria are
  updated.

## Deprecation And Grace Periods

- No compatibility grace period is guaranteed while `contract_version` remains
  `0.x`.
- Canon may replace or remove fields, states, or strategies in a new minor
  contract version instead of carrying a deprecated path.
- Consumers must treat any unsupported minor or major line as requiring an
  explicit update before reuse.

## Non-Goals

- Making Canon the orchestrator for bounded delivery work.
- Allowing consumers to redefine Canon publish or promotion semantics.
- Replacing `.canon/` runtime artifacts with project-visible projections.
