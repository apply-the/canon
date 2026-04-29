# Contract: Release Metadata Artifact

## Purpose

Define the machine-readable release artifact consumed by distribution channels.
This contract keeps package-manager automation aligned with the canonical
GitHub Release bundle.

## Artifact Identity

- **Filename**: `canon-<VERSION>-distribution-metadata.json`
- **Producer**: Canon release workflow after release-surface verification
- **Primary consumer**: Homebrew channel automation in feature `025`
- **Future consumers**: `winget`, Scoop, and other release-driven channels

## Required Top-Level Fields

- `version`: Canon version without a leading `v`
- `tag`: canonical release tag with leading `v`
- `release_url`: published GitHub Release page URL
- `release_notes`: release notes artifact filename or published path
- `checksum_manifest`: checksum filename for the same release
- `generated_at`: timestamp recorded when metadata was produced
- `assets`: non-empty array of asset objects

## Asset Object Rules

Each asset entry MUST contain:

- `asset_id`
- `filename`
- `os`
- `arch`
- `archive_format`
- `binary_name`
- `sha256`
- `download_url`
- `channels`

Each asset entry MUST satisfy these invariants:

- `filename` matches the verified release bundle exactly
- `sha256` matches the checksum manifest exactly
- `download_url` resolves to the canonical GitHub Release asset URL for that
  filename
- `channels` includes `homebrew` only for macOS and Linux tarball assets in
  this slice

## Example Shape

```json
{
  "version": "0.25.0",
  "tag": "v0.25.0",
  "release_url": "https://github.com/apply-the/canon/releases/tag/v0.25.0",
  "release_notes": "release-notes.md",
  "checksum_manifest": "canon-0.25.0-SHA256SUMS.txt",
  "generated_at": "2026-04-29T12:00:00Z",
  "assets": [
    {
      "asset_id": "macos-arm64",
      "filename": "canon-0.25.0-macos-arm64.tar.gz",
      "os": "macos",
      "arch": "arm64",
      "archive_format": "tar.gz",
      "binary_name": "canon",
      "sha256": "<sha256>",
      "download_url": "https://github.com/apply-the/canon/releases/download/v0.25.0/canon-0.25.0-macos-arm64.tar.gz",
      "channels": ["homebrew"]
    }
  ]
}
```

## Failure Semantics

- Metadata generation MUST fail if any expected release asset is missing.
- Metadata generation MUST fail if a checksum cannot be resolved for a listed
  asset.
- The contract MUST preserve Windows assets in the manifest even though
  Homebrew does not consume them yet.