# Data Model: Installable CLI Distribution and Release UX

## Overview

This feature does not change Canon runtime data under `.canon/`. It introduces
an explicit product-distribution model for release assets, install flows, and
version-visible surfaces that must stay aligned.

## Entities

### ReleaseArtifact

Represents one downloadable release archive for a single operating system and
architecture.

| Field | Type | Notes |
| --- | --- | --- |
| `version` | string | Semantic version such as `0.6.0` |
| `os` | enum | `macos`, `linux`, `windows` |
| `arch` | enum | `arm64`, `x86_64` |
| `rust_target` | string | Internal build target such as `aarch64-apple-darwin` |
| `archive_name` | string | `canon-<VERSION>-<os>-<arch>.<ext>` |
| `archive_format` | enum | `tar.gz` or `zip` |
| `binary_name` | string | `canon` or `canon.exe` |
| `checksum_sha256` | string | Hex digest published in the checksum manifest |
| `publication_state` | enum | `planned`, `built`, `checksummed`, `published`, `validated` |

**Validation rules**:

- every published artifact must match one supported platform entry
- archive names must include version, os, and architecture
- Unix archives contain exactly one `canon` executable
- Windows archives contain exactly one `canon.exe` executable
- an artifact cannot reach `published` without a checksum entry

### ReleaseManifest

Represents the complete release surface for one Canon version.

| Field | Type | Notes |
| --- | --- | --- |
| `version` | string | Canon version for the release |
| `git_tag` | string | Expected public tag such as `v0.6.0` |
| `artifacts` | list of `ReleaseArtifact` | Complete platform matrix for the release |
| `checksum_manifest_name` | string | `canon-<VERSION>-SHA256SUMS.txt` |
| `release_notes_ref` | string | Link or artifact for public release notes |
| `review_state` | enum | `draft`, `ready-for-review`, `approved`, `published` |
| `validated_by` | optional string | Independent reviewer for release readiness |

**Validation rules**:

- a manifest is `ready-for-review` only when all required artifacts are built
  and checksummed
- a manifest is `approved` only when version parity and install-guide review
  pass
- a manifest is `published` only after assets and release notes are attached to
  the public release surface

### InstallationFlow

Represents the documented user journey for installing Canon on one platform.

| Field | Type | Notes |
| --- | --- | --- |
| `platform_entry` | string | `macos-arm64`, `linux-x86_64`, `windows-x86_64`, etc. |
| `artifact_name` | string | Archive the user is expected to choose |
| `path_directory_shape` | string | Example PATH directory for that platform |
| `extract_step` | string | Platform-appropriate archive extraction step |
| `install_step` | string | Move or copy binary into PATH directory |
| `verify_command` | string | `canon --version` |
| `smoke_test_command` | string | `canon init` in a fresh repo |
| `troubleshooting_notes` | list of strings | PATH shadowing, wrong architecture, uninitialized repo |

**Validation rules**:

- each supported platform entry must have one documented installation flow
- verification must prove both PATH resolution and version visibility
- smoke tests must not require Cargo or a source checkout

### RuntimeCompatibilityReference

Represents the shared skill-layer guidance for checking and recovering Canon
installation state.

| Field | Type | Notes |
| --- | --- | --- |
| `expected_workspace_version` | string | Canon version expected by this repo |
| `install_guidance_ref` | string | README install section or release URL |
| `version_command` | string | `canon --version` |
| `command_probe` | string | Compatibility fallback when semver is unavailable |
| `required_modes` | list of strings | Modes that prove the expected command contract |

**Validation rules**:

- install guidance must not direct daily users to Cargo for normal use
- the reference must stay aligned with the current release and README install
  section
- shared helper messaging must remain consistent across Bash and PowerShell

### ReleaseReadinessReview

Represents the independent validation bundle required before public release.

| Field | Type | Notes |
| --- | --- | --- |
| `release_version` | string | Version under review |
| `reviewer` | string | Human reviewer independent of asset generation |
| `artifact_matrix_passed` | boolean | All required archives and checksum entries exist |
| `version_parity_passed` | boolean | Tag, archive names, notes, and binary output match |
| `install_walkthrough_passed` | boolean | Fresh-environment install checks succeed |
| `docs_review_passed` | boolean | README and quickstart remain install-first |
| `notes` | string | Review observations and release blockers |

**Validation rules**:

- release publication requires an explicit review record
- failed install or docs review blocks publication even when builds succeeded

## Relationships

- `ReleaseManifest` owns many `ReleaseArtifact` records.
- `InstallationFlow` consumes one `ReleaseArtifact` and one supported platform
  entry.
- `RuntimeCompatibilityReference` points users from skill preflight failures to
  the relevant `InstallationFlow`.
- `ReleaseReadinessReview` validates one `ReleaseManifest` and its associated
  installation flows.

## State Transitions

### ReleaseArtifact

```text
planned
  -> built
  -> checksummed
  -> published
  -> validated
```

Rules:

- artifacts may not skip `checksummed`
- published artifacts remain invalid until manifest-level review passes

### ReleaseManifest

```text
draft
  -> ready-for-review
  -> approved
  -> published
```

Rules:

- `ready-for-review` requires all five public artifacts and the checksum
  manifest
- `approved` requires independent review evidence
- `published` requires attached release notes and public assets