# Data Model: Distribution Channels Beyond GitHub Releases

## DistributionReleaseManifest

- **Purpose**: Canonical machine-readable description of one released Canon
  version and the assets available for downstream channels.
- **Fields**:
  - `version`: released Canon version without the leading `v`
  - `tag`: canonical Git tag for the release
  - `release_url`: release page URL backing the asset downloads
  - `release_notes`: relative or published path to the release notes artifact
  - `checksum_manifest`: filename of the checksum manifest used for validation
  - `generated_at`: generation timestamp for the metadata artifact
  - `assets`: list of `DistributionAsset` entries
- **Validation rules**:
  - `version` must match the verified release bundle
  - `tag` must equal `v<version>`
  - `assets` cannot be empty
  - Every asset referenced in the checksum manifest must appear exactly once in
    `assets`

## DistributionAsset

- **Purpose**: Describes one downloadable Canon archive that a distribution
  channel may consume.
- **Fields**:
  - `asset_id`: stable identifier such as `macos-arm64` or `windows-x86_64`
  - `filename`: canonical archive filename
  - `os`: `macos`, `linux`, or `windows`
  - `arch`: `arm64` or `x86_64`
  - `archive_format`: `tar.gz` or `zip`
  - `binary_name`: installed executable name contained in the archive
  - `sha256`: checksum from the verified manifest
  - `download_url`: canonical GitHub Release URL for the asset
  - `channels`: list of distribution channels allowed to consume the asset
- **Validation rules**:
  - `download_url` must be derivable from the canonical release page and
    `filename`
  - `sha256` must match the checksum manifest exactly
  - `channels` must include `homebrew` only for assets Homebrew can consume in
    this slice

## HomebrewFormulaInput

- **Purpose**: Channel-specific view of the release manifest used to render the
  Homebrew formula.
- **Fields**:
  - `formula_name`: `canon`
  - `class_name`: `Canon`
  - `description`: user-facing description
  - `homepage`: upstream project homepage
  - `license`: SPDX identifier or accepted expression
  - `version`: version used by the formula
  - `variants`: list of `HomebrewVariant` entries
  - `test_command`: basic smoke test command sequence
- **Validation rules**:
  - `variants` must cover all Homebrew-supported OS and CPU combinations in the
    manifest
  - `version` must match the release manifest

## HomebrewVariant

- **Purpose**: Maps one Homebrew platform branch to one canonical release
  asset.
- **Fields**:
  - `system`: `macos` or `linux`
  - `cpu`: `arm64` or `x86_64`
  - `asset_id`: linked release asset
  - `url`: asset download URL
  - `sha256`: asset checksum
- **Relationships**:
  - Many `HomebrewVariant` entries belong to one `HomebrewFormulaInput`
  - Each `HomebrewVariant` references exactly one `DistributionAsset`

## TapPublicationPlan

- **Purpose**: Defines how the rendered Homebrew formula reaches its
  destination.
- **Fields**:
  - `tap_repository`: destination repository name
  - `formula_path`: destination path, expected to be `Formula/canon.rb`
  - `publication_mode`: `pull-request` or `artifact-only`
  - `branch_prefix`: branch naming convention for generated updates
  - `commit_subject`: canonical commit or PR title prefix
  - `fallback_artifact`: rendered formula artifact retained when publication is
    not possible
- **Validation rules**:
  - `publication_mode=artifact-only` must still define `fallback_artifact`
  - `formula_path` must remain stable across releases unless explicitly changed

## DistributionPublicationAttempt

- **Purpose**: Records the outcome of trying to publish the Homebrew update.
- **Fields**:
  - `version`: release version
  - `publication_mode`: actual mode used for the attempt
  - `status`: `rendered`, `published`, or `failed`
  - `artifact_path`: rendered formula artifact path
  - `tap_reference`: PR reference or branch name when publication succeeds
  - `failure_reason`: explicit error summary when publication fails
- **Validation rules**:
  - `failed` attempts must preserve `artifact_path`
  - `published` attempts must record a non-empty `tap_reference`

## Relationships Summary

- One `DistributionReleaseManifest` owns many `DistributionAsset` entries.
- One `HomebrewFormulaInput` is derived from one `DistributionReleaseManifest`.
- Each `HomebrewVariant` references one `DistributionAsset`.
- One `TapPublicationPlan` governs one `DistributionPublicationAttempt` per
  release execution.

## State Transitions

- `DistributionReleaseManifest`: `assembled` -> `verified` -> `published`
- `HomebrewFormulaInput`: `derived` -> `rendered` -> `validated`
- `DistributionPublicationAttempt`: `pending` -> `published`, `rendered`, or
  `failed`