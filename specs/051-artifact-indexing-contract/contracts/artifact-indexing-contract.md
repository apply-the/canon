# Artifact Indexing Contract

- **Owner**: Canon
- **Status**: Derived, non-normative brief
- **Normative Stable Path**: `docs/integration/project-memory-promotion-contract.md`
- **Current Contract Line**: V1

## Purpose

Summarize the V1 Canon artifact-indexing surface for downstream consumers
without creating a second authority surface.

## Authority Rule

- This brief mirrors the stable Canon contract and exists for feature-local
  planning and validation.
- If this brief and `docs/integration/project-memory-promotion-contract.md`
  diverge, the stable Canon contract wins.

## Supported V1 Indexable Artifact Classes

- `managed-surface`:
  stable or pending project-memory documents updated through Canon-managed
  blocks
- `proposal-artifact`:
  proposal files emitted when Canon must not mutate the stable target
- `evidence-bundle`:
  readable evidence artifacts published under `docs/evidence/` or another
  evidence-facing destination
- `index-surface`:
  append-only summary or index surfaces used for visibility without stable
  overwrite

## Metadata Carrier Mapping

- `managed-surface` -> `managed-surface-envelope`
  Discovery rule: read the `project-memory:managed` start marker for
  `producer`, `source_ref`, and `contract_version`, then read the adjacent
  `<surface>.packet-metadata.json` sidecar for the full promoted lineage.
- `proposal-artifact` -> `packet-metadata-sidecar`
  Discovery rule: read `packet-metadata.json` or
  `<surface>.packet-metadata.json` adjacent to the published proposal output.
- `evidence-bundle` -> `packet-metadata-sidecar`
  Discovery rule: read `packet-metadata.json` adjacent to the evidence bundle
  root or the surface-specific sidecar when Canon publishes onto a single
  evidence-facing document.
- `index-surface` -> `packet-metadata-sidecar`
  Discovery rule: read `<surface>.packet-metadata.json` adjacent to the
  append-only surface.

## Producer Sidecar Payload

Canon V1 producer sidecars for supported surfaces expose this typed
`artifact_indexing` payload:

```json
{
  "artifact_indexing": {
    "artifact_class": "managed-surface|proposal-artifact|evidence-bundle|index-surface",
    "metadata_carrier": "managed-surface-envelope|packet-metadata-sidecar",
    "discovery_rule": "consumer-facing discovery rule string"
  }
}
```

Publish paths MUST reject unsupported target-class and update-strategy
combinations instead of silently projecting an ambiguous artifact class.

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

## Explicit Exclusions

- `safety-net packets` is not a Canon V1 artifact class.
- Any artifact class not listed above is outside the stable indexing contract
  until Canon documents it in the stable path.