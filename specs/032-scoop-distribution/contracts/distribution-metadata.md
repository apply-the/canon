# Contract: Distribution Metadata

## Purpose

Define the machine-readable release metadata fields that package-manager
channels can trust when rendering publication artifacts from GitHub Releases.

## Top-Level Shape

The metadata document MUST retain the current top-level shape:

```json
{
  "version": "0.32.0",
  "tag": "v0.32.0",
  "release_url": "https://github.com/apply-the/canon/releases/tag/v0.32.0",
  "release_notes": "release-notes.md",
  "checksum_manifest": "canon-0.32.0-SHA256SUMS.txt",
  "generated_at": "2026-05-01T00:00:00Z",
  "assets": []
}
```

## Asset Contract

Each entry in `assets` MUST include:

- `asset_id`
- `filename`
- `os`
- `arch`
- `archive_format`
- `binary_name`
- `sha256`
- `download_url`
- `channels`

## Windows Package-Manager Expectations

The canonical Windows entry MUST remain a single asset record with these
properties:

- `asset_id`: `windows-x86_64`
- `filename`: `canon-<VERSION>-windows-x86_64.zip`
- `os`: `windows`
- `arch`: `x86_64`
- `archive_format`: `zip`
- `binary_name`: `canon.exe`
- `download_url`: derived from the GitHub Release tag and filename
- `sha256`: read from `canon-<VERSION>-SHA256SUMS.txt`
- `channels`: exactly `[
  "winget",
  "scoop"
]`

The feature MUST NOT create separate metadata entries for the same Windows zip
just to satisfy different package-manager channels.

## Validation Rules

- Every expected archive in the release bundle MUST have a matching metadata
  entry.
- Every metadata `download_url` MUST point to the tagged GitHub Release asset.
- Every metadata `sha256` MUST match the checksum manifest.
- Package-manager renderers MUST reject missing Windows URL, filename, or hash
  values instead of inventing defaults.