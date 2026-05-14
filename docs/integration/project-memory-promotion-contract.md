# Project Memory Promotion Contract

## Contract Identity

- `owner`: `canon`
- `current_contract_line`: `v1`
- `stable_doc`: `docs/integration/project-memory-promotion-contract.md`
- `primary_consumer`: `boundline`

## Purpose

Define the Canon-owned producer semantics for repo-visible project memory and
evidence publication so consumers can rely on Canon output without redefining
Canon publish policy or turning Canon into the delivery orchestrator.

## Authority And Sync Rules

- This stable document is the normative source for ownership, compatibility,
  target classes, and publish-policy semantics.
- Feature-local contracts may elaborate this contract with examples and
  supporting shapes, but they do not supersede it.
- If a conflict appears between this stable document and a feature-local brief,
  this stable document wins and the feature-local brief must be realigned in
  the same change before merge.
- Consumers may rely on Canon contracts but may not redefine Canon promotion
  semantics.

## Supporting Shape Briefs

- `specs/050-project-memory-control/contracts/project-memory-promotion-contract.md`
- `specs/050-project-memory-control/contracts/governed-stage-ref-contract.md`
- `specs/050-project-memory-control/contracts/promotion-event-contract.md`
- `specs/050-project-memory-control/contracts/evidence-ref-contract.md`
- `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md` (derived, non-normative)
- `specs/051-artifact-indexing-contract/contracts/evidence-block-metadata-contract.md` (derived, non-normative)

## Default Targets

- `docs/project/` for stable or pending project-memory surfaces
- `docs/evidence/` for readable evidence summaries and mixed-producer evidence
  blocks

## Target Classes

- `stable`: policy-eligible managed updates to stable repo-visible knowledge
- `pending`: visible but not yet stable project-memory surfaces
- `proposal`: proposal artifacts used when safe stable publication is not
  credible
- `evidence`: readable evidence summaries without stable project-memory
  promotion
- `index`: append-only or summary surfaces for visibility without stable
  overwrite

## Indexable Artifact Classes

- `managed-surface`: stable or pending repo-visible project-memory documents
  updated through Canon-managed blocks
- `proposal-artifact`: proposal files emitted instead of mutating a stable
  project-memory target
- `evidence-bundle`: readable evidence artifacts published under
  `docs/evidence/` or another evidence-facing destination
- `index-surface`: append-only index or summary surfaces used for visibility
  without stable overwrite

## Metadata Carrier And Discovery Rules

- `managed-surface` uses the `managed-surface-envelope` carrier:
  read the `project-memory:managed` start marker for `producer`, `source_ref`,
  and `contract_version`, then read the adjacent
  `<surface>.packet-metadata.json` sidecar for the full promoted lineage
  envelope.
- `proposal-artifact`, `evidence-bundle`, and `index-surface` use the
  `packet-metadata-sidecar` carrier:
  read `packet-metadata.json` for packet roots or `<surface>.packet-metadata.json`
  adjacent to the published surface for canonical indexing metadata.
- If an artifact class is not listed here, it is outside the stable V1
  indexing contract and consumers MUST NOT infer its metadata carrier.
- `safety-net packets` is not a Canon V1 artifact class and MUST NOT be used as
  consumer-facing contract vocabulary.

## Managed Block Format

```md
<!-- project-memory:managed:start producer="canon|boundline" source_ref="..." contract_version="v1" -->
...
<!-- project-memory:managed:end -->
```

- `producer`, `source_ref`, and `contract_version` are mandatory marker fields.
- Boundline-managed blocks may contribute readable evidence text and
  attribution metadata for `producer="boundline"` content.
- Only Canon-owned shapes define `promotion_state`, `approval_state`,
  `packet_readiness`, and target-routing semantics.

## Promotion State Expectations

- Stable managed-block updates are reserved for completed and policy-eligible
  knowledge.
- `blocked` means approval, readiness, or another Canon policy prerequisite is
  missing; stable targets are forbidden and Canon routes to pending, proposal,
  or evidence surfaces instead.
- `conflicting` means Canon detects contradictory or overlapping candidate
  knowledge that requires human resolution; stable targets are forbidden and
  Canon routes to proposal, pending, or evidence surfaces instead.
- `pending` remains visible but not yet stable.
- `index-only` and `evidence-only` outputs never overwrite stable
  project-memory surfaces.

## Required V1 Lineage Fields

- `contract_version`
- `producer`
- `source_ref`
- `source_artifacts`
- `promotion_state`
- `promoted_at`
- `content_digest`

## Optional V1 Lineage Fields

- `mode`
- `stage`
- `owner`
- `risk`
- `zone`
- `approval_state`
- `packet_readiness`
- `promotion_profile`

## Standalone JSON Shape Identifier

When Canon or a consumer serializes one of the shared JSON contract shapes
outside this document, the payload should include a stable `kind` field such as
`project_memory_promotion`, `governed_stage_ref`, `promotion_event`, or
`evidence_ref` so embedded payloads and logs remain self-describing.

`kind` is recommended in V1. It is not a required V1 lineage field yet because
the current producer and consumer implementations do not validate it. Promote it
to required status only in the next compatible contract minor after validation is
wired on both sides.

## Compatibility Policy

- Additive V1 changes are backward-compatible.
- Removing or renaming required fields is breaking.
- Breaking changes require a new major contract line.
- Canon supports the previous minor published contract revision for one full
  minor release cycle.

## Non-Goals

- Making Canon the orchestrator for bounded delivery work
- Allowing consumers to redefine Canon publish or promotion semantics
- Replacing `.canon/` runtime artifacts with repo-visible projections
