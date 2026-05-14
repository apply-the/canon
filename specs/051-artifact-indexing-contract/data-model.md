# Data Model: Artifact Indexing Contract

## ArtifactMetadataCarrier

- `managed-surface-envelope`
  Meaning: producer attribution comes from the managed-block marker family and
  the full lineage comes from the adjacent surface sidecar.
- `packet-metadata-sidecar`
  Meaning: canonical indexing metadata is discovered from `packet-metadata.json`
  or `<surface>.packet-metadata.json` next to the published artifact.

## IndexableArtifactClass

- `managed-surface`
- `proposal-artifact`
- `evidence-bundle`
- `index-surface`

Each class maps to exactly one metadata carrier in V1.

## ManagedBlockDescriptor

- Fields: `producer`, `source_ref`, `contract_version`
- Purpose: stable producer attribution for managed repo-visible blocks

## LineageMetadata

### Required Fields

- `contract_version`
- `producer`
- `source_ref`
- `source_artifacts`
- `promotion_state`
- `promoted_at`
- `content_digest`

### Optional Fields

- `mode`
- `stage`
- `owner`
- `risk`
- `zone`
- `approval_state`
- `packet_readiness`
- `promotion_profile`