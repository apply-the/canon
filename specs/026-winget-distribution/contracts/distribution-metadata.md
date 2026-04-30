# Contract: Distribution Metadata

## Purpose

Define the machine-readable release metadata fields that the Windows packaging
channel can trust when rendering a `winget` publication bundle.

## Top-Level Shape

The metadata document MUST retain the current top-level shape:

```json
{
  "version": "0.25.0",
  "tag": "v0.25.0",
  "release_url": "https://github.com/apply-the/canon/releases/tag/v0.25.0",
  "release_notes": "release-notes.md",
  "checksum_manifest": "canon-0.25.0-SHA256SUMS.txt",
  "generated_at": "2026-04-30T00:00:00Z",
  "assets": []
}
```

## Windows Asset Requirements

For the supported Windows release asset, the metadata MUST provide:

- `asset_id = "windows-x86_64"`
- `os = "windows"`
- `arch = "x86_64"`
- `archive_format = "zip"`
- `binary_name = "canon.exe"`
- `sha256` matching the checksum manifest
- `download_url` pointing at the canonical GitHub Release asset
- `channels` containing `"winget"`

## Channel Semantics

- `channels` MUST remain an array of package-manager channels supported by the
  asset.
- Windows assets MUST NOT advertise unsupported channels in this slice.
- Deferred channels such as Scoop MUST NOT appear in the Windows asset channel
  list until their publication surface exists.

## Validation Rules

- Metadata generation MUST fail if the Windows archive or checksum entry is
  missing.
- Metadata validation MUST reject any Windows asset whose URL, checksum, or
  binary name do not match the release bundle.