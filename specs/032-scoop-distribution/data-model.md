# Data Model: Scoop Distribution Follow-On

## Release Bundle

- **Purpose**: Represents the canonical package outputs for a specific Canon
  version.
- **Fields**:
  - `version`
  - `tag`
  - `release_url`
  - `checksum_manifest`
  - `assets[]`
- **Relationships**:
  - Owns one or more `DistributionAsset` records.
  - Feeds exactly one `ScoopManifest` for the Windows Scoop channel.

## Distribution Asset

- **Purpose**: Describes a versioned platform artifact that can be installed or
  referenced by one or more distribution channels.
- **Fields**:
  - `asset_id`
  - `filename`
  - `os`
  - `arch`
  - `archive_format`
  - `binary_name`
  - `sha256`
  - `download_url`
  - `channels[]`
- **Relationships**:
  - Belongs to one `ReleaseBundle`.
  - May be referenced by zero or more `DistributionChannelRecord` entries.
  - The Windows x86_64 asset is referenced by both the `WingetManifestBundle`
    and the `ScoopManifest`.

## Scoop Manifest

- **Purpose**: Captures the publication-ready Scoop metadata derived from the
  canonical Windows release asset.
- **Fields**:
  - `manifest_asset_name`
  - `app_name`
  - `version`
  - `description`
  - `homepage`
  - `license`
  - `architecture.64bit.url`
  - `architecture.64bit.hash`
  - `bin`
- **Relationships**:
  - Reads from one `DistributionAsset` representing the Windows zip.
  - Is published as a versioned GitHub Release artifact.
  - Is manually copied or renamed to the bucket-facing `canon.json` path during
    Scoop submission.

## Distribution Channel Record

- **Purpose**: Records how a platform should install Canon and whether the path
  is primary, secondary, fallback, or deferred.
- **Fields**:
  - `channel_name`
  - `platform`
  - `status` (`primary`, `secondary`, `fallback`, `deferred`)
  - `artifact_source`
  - `documentation_anchor`
- **Relationships**:
  - References the Windows `winget` and Scoop manifest artifacts for supported
    Windows package-manager channels.
  - References the Windows archive as the fallback path.

## Roadmap Candidate Record

- **Purpose**: Represents an active, delivered, or removed next-feature
  direction in the maintained roadmap.
- **Fields**:
  - `title`
  - `status` (`active`, `delivered`, `removed`, `deferred`)
  - `rationale`
  - `continuity_note`
- **Relationships**:
  - Changes in this record are documented in `decision-log.md` and reflected in
    `ROADMAP.md`.