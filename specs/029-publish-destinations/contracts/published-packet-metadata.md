# Contract: Published Packet Metadata

## Scope

This contract defines the metadata that must remain recoverable from a
published Canon packet once the external path stops being run-id-only.

## Required Metadata Fields

- `run_id`
- `mode`
- `risk`
- `zone`
- `publish_timestamp`
- `descriptor`
- `destination`
- `source_artifacts`

## Metadata Rules

- Metadata MUST be emitted as `packet-metadata.json` with the published
  packet, not only preserved in `.canon/`.
- Metadata MUST keep canonical run identity recoverable even when the external
  path is descriptor-based.
- Metadata MUST describe the materialized artifact lineage for the packet using
  canonical source artifact references.
- Metadata emission MUST NOT require rewriting every published artifact body in
  the first slice.

## Non-Goals

- Replacing runtime manifests under `.canon/`.
- Creating a new approval or publish-state record.
- Turning published metadata into a new source of truth for run identity.