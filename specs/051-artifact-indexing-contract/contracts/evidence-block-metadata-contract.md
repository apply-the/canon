# Evidence Block Metadata Contract

- **Owner**: Canon
- **Status**: Derived, non-normative brief
- **Normative Stable Path**: `tech-docs/integration/project-memory-promotion-contract.md`

## Purpose

Clarify the Canon-owned metadata expectations for readable evidence blocks and
their downstream indexing surface.

## Managed Block Marker

```md
<!-- project-memory:managed:start producer="canon|boundline" source_ref="..." contract_version="v1" -->
...
<!-- project-memory:managed:end -->
```

- `producer`, `source_ref`, and `contract_version` are mandatory marker fields.
- The marker carries producer attribution and stable source identity for the
  managed range.

## Full Lineage Discovery

- For evidence embedded in a managed surface, consumers read the start marker
  for producer attribution and the adjacent `<surface>.packet-metadata.json`
  sidecar for the full promoted lineage.
- For evidence published as a bundle, consumers read `packet-metadata.json`
  adjacent to the bundle root.
- Those sidecars now also expose the typed `artifact_indexing` payload so
  consumers can recover the published artifact class and carrier rule without
  inferring them from destination paths alone.

## Ownership Boundary

- Boundline may contribute readable evidence text inside producer-neutral
  managed blocks when `producer="boundline"`.
- Only Canon-owned shapes define `promotion_state`, `approval_state`,
  `packet_readiness`, and target-routing semantics.

## Explicit Exclusions

- This brief does not define Boundline runtime indexing or stop semantics.
- This brief does not introduce `safety-net packets` as a Canon evidence class.