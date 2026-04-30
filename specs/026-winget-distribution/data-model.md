# Data Model: Winget Distribution And Roadmap Refocus

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
  - Feeds exactly one `WingetManifestBundle` for the supported Windows channel.

## Distribution Asset

- **Purpose**: Describes a versioned platform artifact that can be installed or
  referenced by a distribution channel.
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
  - The Windows x86_64 asset is referenced by the `WingetManifestBundle`.

## Winget Manifest Bundle

- **Purpose**: Captures the publication-ready Windows Package Manager metadata
  derived from the canonical Windows release asset.
- **Fields**:
  - `package_identifier`
  - `package_version`
  - `default_locale`
  - `publisher`
  - `package_name`
  - `license`
  - `short_description`
  - `installer_type`
  - `nested_installer_type`
  - `nested_files[]`
  - `commands[]`
  - `manifest_version`
  - `output_paths[]`
- **Relationships**:
  - Reads from one `DistributionAsset` representing the Windows archive.
  - Emits one version manifest, one default locale manifest, and one installer
    manifest.

## Distribution Channel Record

- **Purpose**: Records how a platform should install Canon and whether that path
  is primary, fallback, or deferred.
- **Fields**:
  - `channel_name`
  - `platform`
  - `status` (`primary`, `fallback`, `deferred`)
  - `artifact_source`
  - `documentation_anchor`
- **Relationships**:
  - References the `WingetManifestBundle` for Windows primary installation.
  - References the Windows archive as the Windows fallback path.

## Roadmap Candidate

- **Purpose**: Represents an active or removed next-feature direction in the
  maintained roadmap.
- **Fields**:
  - `title`
  - `status` (`active`, `removed`, `deferred`)
  - `rationale`
  - `replacement_focus`
- **Relationships**:
  - Changes in this entity are recorded in the feature `decision-log.md`.