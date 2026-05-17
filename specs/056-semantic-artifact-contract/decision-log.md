# Decision Log: Semantic Artifact Contract

## D-001: Separate semantic contract surface from publication routing contract

- Date: 2026-05-17
- Status: Accepted for planning
- Context: Canon already has a stable promotion contract that owns publication
  target classes, managed-block lineage, and packet metadata routing.
- Decision: Keep semantic eligibility and provenance semantics in a dedicated
  semantic contract surface that extends, but does not replace, the promotion
  contract.
- Alternatives considered:
  - embed semantic rules only in the promotion contract
  - keep semantic semantics only in feature-local documentation
- Consequences: consumers get a stable semantic surface without blurring
  publication routing and semantic retrieval semantics.

## D-002: Reuse existing metadata carriers and discovery rules

- Date: 2026-05-17
- Status: Accepted for planning
- Context: Canon already exposes typed artifact-indexing metadata through
  managed-surface envelopes and packet metadata sidecars.
- Decision: Carry semantic metadata through the same metadata carrier family
  and discovery path already defined by the artifact-indexing contract.
- Alternatives considered:
  - dedicated semantic sidecar
  - Markdown-only semantic annotations without a typed metadata carrier
- Consequences: the contract stays additive and consumers do not need a second
  lookup mechanism.

## D-003: Exclude index surfaces from semantic eligibility

- Date: 2026-05-17
- Status: Accepted for planning
- Context: `index-surface` is indexable for visibility and discoverability, but
  it is not a stable unit of authored meaning suitable for semantic retrieval.
- Decision: Mark `index-surface` as semantically excluded and require the
  contract to say so explicitly.
- Alternatives considered:
  - make every indexable artifact class semantically eligible
  - leave exclusion implicit by omitting index surfaces from the semantic list
- Consequences: consumers can reject index-only surfaces without guessing at
  Canon intent.

## D-004: Keep provenance producer-owned and consumer fragments downstream-owned

- Date: 2026-05-17
- Status: Accepted for planning
- Context: Boundline S5.v2 needs stable provenance anchors but owns its own
  local fragment and retrieval runtime behavior.
- Decision: Canon semantic provenance boundaries are limited to `surface`,
  `managed_block`, and `section`; Canon does not own consumer fragment IDs,
  ranking, or retrieval policy.
- Alternatives considered:
  - Canon-owned fragment identifiers
  - more granular producer boundaries such as paragraph or span anchors
- Consequences: provenance remains stable and auditable without expanding
  Canon into a runtime retrieval system.

## D-005: Unsupported semantic states must be explicit

- Date: 2026-05-17
- Status: Accepted for planning
- Context: Consumers need deterministic rejection reasons when semantic
  metadata is excluded, incomplete, or incompatible.
- Decision: The contract must explicitly define unsupported conditions for
  excluded artifact classes, unsupported contract lines, missing required
  semantic fields, and invalid or non-Canon provenance references.
- Alternatives considered:
  - soft-warning semantics for missing metadata
  - consumer-specific fallback behavior without Canon-owned rejection reasons
- Consequences: downstream systems can preserve Canon intent when declining a
  candidate instead of inferring behavior from missing data.