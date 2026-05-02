# Data Model: Release Provenance And Channel Integrity

## Entity: Release Provenance Record

- **Purpose**: Represents the canonical machine-readable record that binds one
  Canon version to the GitHub Release bundle from which all package-manager
  channels are derived.
- **Fields**:
  - `version`: released semantic version
  - `tag`: canonical Git tag for the release
  - `release_url`: GitHub Release URL
  - `release_notes`: canonical release-notes artifact filename
  - `checksum_manifest`: canonical checksum artifact filename
  - `generated_at`: UTC generation timestamp
  - `source_of_truth`: explicit declaration of the authoritative release host
    and bundle provenance model
  - `assets`: canonical distribution asset records
  - `channels`: explicit distribution channel contracts
- **Validation Rules**:
  - The record must describe exactly one released version.
  - `source_of_truth` must remain compatible with GitHub Releases as the sole
    downloadable artifact host.
  - `assets` and `channels` must not contradict each other.

## Entity: Source Of Truth Declaration

- **Purpose**: States which release surface is authoritative for binaries,
  checksums, and release notes.
- **Fields**:
  - `kind`: canonical release host identifier
  - `artifact_inventory`: the bundle inventory surface that downstream
    channels must trust
  - `checksum_source`: canonical checksum artifact identifier
  - `release_notes_source`: canonical release notes artifact identifier
- **Validation Rules**:
  - All values must remain consistent with the release bundle described by the
    provenance record.
  - No alternate release host may be introduced in this slice.

## Entity: Distribution Asset Record

- **Purpose**: Describes one canonical downloadable release artifact.
- **Fields**:
  - `asset_id`: stable identifier such as `macos-arm64` or `windows-x86_64`
  - `filename`: canonical archive filename
  - `os`: target operating system
  - `arch`: target architecture
  - `archive_format`: release archive format
  - `binary_name`: executable inside the archive
  - `sha256`: canonical checksum from the release bundle
  - `download_url`: canonical GitHub Release asset URL
  - `channels`: package-manager channels allowed to consume the asset
- **Validation Rules**:
  - `filename`, `sha256`, and `download_url` must match the verified release
    bundle.
  - The `channels` list must not advertise a channel that lacks a matching
    channel contract.

## Entity: Distribution Channel Contract

- **Purpose**: Explicitly describes one repository-owned package-manager
  channel and the canonical assets or generated artifacts it is allowed to use.
- **Fields**:
  - `channel`: channel identifier such as `homebrew`, `winget`, or `scoop`
  - `asset_ids`: canonical asset ids the channel may consume
  - `generated_artifacts`: expected rendered filenames or manifest bundle
    members for that channel
- **Validation Rules**:
  - Every `asset_id` must exist in the canonical asset inventory.
  - The contract must fail closed if a renderer requires an asset id or
    generated artifact not declared here.
  - A channel contract must not authorize a package-manager output that the
    release bundle cannot support.

## Entity: Generated Channel Artifact Expectation

- **Purpose**: Represents the expected output shape for a derived package-
  manager artifact.
- **Fields**:
  - `filename`: rendered artifact name such as `canon.rb` or `canon.json`
  - `channel`: owning distribution channel
  - `kind`: formula, manifest, or manifest-member type
- **Validation Rules**:
  - The expectation must be satisfiable by the renderer for that channel.
  - The expectation must remain consistent with the corresponding template or
    output bundle shape.

## Relationships

- One **Release Provenance Record** owns many **Distribution Asset Records**.
- One **Release Provenance Record** owns many **Distribution Channel
  Contracts**.
- Each **Distribution Channel Contract** references one or more
  **Distribution Asset Records** through `asset_ids`.
- Each **Distribution Channel Contract** owns one or more **Generated Channel
  Artifact Expectations**.

## State Semantics

- The provenance record is valid only when the top-level source-of-truth
  declaration, asset inventory, and channel contracts all agree.
- A renderer may proceed only when its channel contract is present and valid.
- A release-surface verifier may declare the bundle ready only when provenance,
  assets, generated artifacts, and package-channel expectations all remain
  aligned.