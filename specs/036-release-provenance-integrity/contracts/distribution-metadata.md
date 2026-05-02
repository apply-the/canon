# Contract: Distribution Metadata

## Purpose

Define the canonical machine-readable release manifest that all package-manager
renderers and release verification logic must consume.

## Required Top-Level Fields

- `version`
- `tag`
- `release_url`
- `release_notes`
- `checksum_manifest`
- `generated_at`
- `source_of_truth`
- `assets`
- `channels`

## Source Of Truth Shape

The metadata MUST expose an explicit object with these fields:

- `kind`
- `artifact_inventory`
- `checksum_source`
- `release_notes_source`

The `source_of_truth` object MUST identify GitHub Releases as the canonical
artifact host for binaries, checksums, and release notes.

## Asset Inventory Shape

Each asset entry MUST include:

- `asset_id`
- `filename`
- `os`
- `arch`
- `archive_format`
- `binary_name`
- `sha256`
- `download_url`
- `channels`

## Channel Contract Shape

Each channel entry MUST include:

- `channel`
- `asset_ids`
- `generated_artifacts`

## Behavioral Requirements

- The metadata MUST remain self-consistent: every `asset_id` referenced by a
  channel contract must exist in `assets`.
- Asset-level `channels` membership and top-level `channels` contracts MUST
  agree.
- The metadata generator MUST fail when it cannot produce a complete canonical
  provenance record.
- Consumers MUST treat missing required top-level fields or malformed channel
  contracts as fatal errors.

## Compatibility Notes

- Existing asset-level fields remain the source for archive filenames, URLs,
  and checksums.
- The contract may extend with additive fields in later versions, but the 036
  slice requires the above fields and semantics.