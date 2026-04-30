# Implementation Plan: Winget Distribution And Roadmap Refocus

**Branch**: `026-winget-distribution` | **Date**: 2026-04-30 | **Spec**: `specs/026-winget-distribution/spec.md`
**Input**: Feature specification from `/specs/026-winget-distribution/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Deliver the next concrete distribution slice by turning the existing Windows
release archive into a reviewable `winget` publication bundle while keeping
GitHub Releases as the canonical binary source. The implementation will extend
the current release metadata and verification surfaces, generate a multi-file
`winget` manifest set that describes the existing Windows zip as an archive
containing a portable executable, update the release workflow and user-facing
install guidance, and remove speculative MCP / Protocol Interoperability work
from the active roadmap.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; the change modifies packaging,
workflow, docs, and roadmap surfaces in an existing system, but it does not
change Canon runtime behavior, execution policy, or trust boundaries.  
**Scope In**: Windows `winget` manifest generation and validation; release
workflow wiring for Windows publication artifacts; install, release, changelog,
and roadmap documentation updates; focused release-surface and documentation
tests.  
**Scope Out**: Homebrew delivery, Scoop delivery, Debian packaging, MCP runtime
enablement, Canon as an MCP server, generic protocol-interoperability work, and
new runtime modes.

**Invariants**:

- GitHub Releases remain the canonical source for Canon versioning, Windows
  binaries, and checksums.
- The existing Windows direct-download archive path remains documented and valid
  as a fallback.
- The feature stays bounded to repository-owned release, packaging, and
  documentation surfaces; it must not widen into runtime adapter or protocol
  behavior.

**Decision Log**: `specs/026-winget-distribution/decision-log.md`  
**Validation Ownership**: Generation work updates release scripts, workflow,
docs, and feature artifacts; validation is performed through focused release
tests, document checks, script validation, and an independent review of roadmap
cleanup and distribution evidence.  
**Approval Gates**: No special human approval gate beyond normal review is
required for bounded-impact work; independent validation evidence remains
mandatory before completion.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Bash and PowerShell release helpers and GitHub Actions YAML  
**Primary Dependencies**: existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), shell tooling (`jq`, `shasum`, `unzip`), and Windows Package Manager manifest schema v1.12.0  
**Storage**: repository files plus ephemeral release artifacts in `dist/` during packaging and validation  
**Testing**: `cargo test`, focused release/documentation tests, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and script-level validation  
**Target Platform**: GitHub-hosted release automation and Windows x86_64 installation through `winget` with GitHub Releases as the binary host  
**Project Type**: Rust CLI with governed release, packaging, and documentation surfaces  
**Existing System Touchpoints**: `.github/workflows/release.yml`, `.github/release-notes-template.md`, `scripts/release/package-windows.ps1`, `scripts/release/write-distribution-metadata.sh`, `scripts/release/verify-release-surface.sh`, `README.md`, `CHANGELOG.md`, `ROADMAP.md`, and focused release docs/tests under `tests/`  
**Performance Goals**: keep release-surface generation bounded to the existing packaging workflow and avoid any manual checksum or URL derivation step for Windows package publication  
**Constraints**: GitHub Releases stay canonical, Windows support remains x86_64-first, direct-download fallback stays documented, and no protocol-interoperability work can enter scope through packaging  
**Scale/Scope**: one new Windows package-manager channel, one new feature spec directory, one focused release-test slice, and bounded updates to existing packaging/docs surfaces

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/026-winget-distribution/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── distribution-metadata.md
│   └── winget-manifest-bundle.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
├── release-notes-template.md
└── workflows/
    └── release.yml

scripts/
└── release/
    ├── package-windows.ps1
    ├── verify-release-surface.sh
    ├── write-distribution-metadata.sh
    ├── render-homebrew-formula.sh
    ├── sync-homebrew-tap.sh
    └── render-winget-manifests.sh

packaging/
└── winget/
    ├── defaultLocale.yaml.tpl
    ├── installer.yaml.tpl
    └── version.yaml.tpl

tests/
├── release_024_docs.rs
└── release_026_winget_distribution.rs

README.md
CHANGELOG.md
ROADMAP.md
Cargo.toml
```

**Structure Decision**: Keep the feature localized to the existing release automation surface by adding one new renderer script, one new packaging template directory, one focused release test file, and bounded documentation updates rather than introducing a new crate or generic packaging subsystem.

## Complexity Tracking

No constitution deviations are currently identified.

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/research.md
# Research: Winget Distribution And Roadmap Refocus

## Decision 1: Reuse the existing Windows zip as a winget archive installer

**Decision**: Treat the current `canon-<VERSION>-windows-x86_64.zip` asset as the
installer artifact for `winget` and describe it using installer schema v1.12.0
with `InstallerType: zip`, `NestedInstallerType: portable`, and a nested
`canon.exe` entry with a command alias.

**Rationale**: The existing packaging flow already produces a versioned Windows
zip that contains `canon.exe`. The winget installer schema explicitly supports
archive installers with nested portable executables, so Canon can add a Windows
package-manager channel without inventing a second Windows binary format.

**Alternatives considered**:

- Generate a second Windows installer format just for `winget`: rejected
  because it widens packaging scope and introduces parallel Windows artifacts
  without proven user value.
- Rely only on `wingetcreate` interactive authoring: rejected because it does
  not create a durable repository-owned artifact contract for release review.

## Decision 2: Generate a multi-file manifest bundle, not a singleton manifest

**Decision**: Emit the recommended multi-file winget manifest bundle with
separate `version`, `defaultLocale`, and `installer` files.

**Rationale**: The multi-file layout is the recommended shape for richer
metadata, keeps installer concerns separate from package metadata, and makes it
easier to review generated publication artifacts in-repo and in release
artifacts.

**Alternatives considered**:

- Use a singleton manifest: rejected because it compresses concerns into one
  file and provides less room for durable metadata and validation.
- Defer machine-readable manifests and document a manual maintainer checklist:
  rejected because the whole point of the slice is to eliminate hand-derived
  publication steps.

## Decision 3: Keep publication artifact generation repo-owned, but keep final winget submission human-driven

**Decision**: Canon will generate the manifest bundle and release-ready
publication inputs in-repo, but it will not automate submission to the
`winget-pkgs` repository in this slice.

**Rationale**: Repository-owned manifest generation gives maintainers a durable,
reviewable output while staying within Canon's bounded release posture. Final
submission still requires repository, credential, and external workflow choices
that are better left explicit rather than hidden behind automation.

**Alternatives considered**:

- Open or merge `winget-pkgs` pull requests automatically: rejected because it
  introduces external-state automation, token management, and failure handling
  that are unnecessary for the first slice.
- Leave publication entirely manual with no generated artifact: rejected
  because it fails the artifact-first requirement and preserves checksum drift
  risk.

## Decision 4: Remove Protocol Interoperability from the active roadmap rather than renaming it

**Decision**: Remove the Protocol Interoperability / MCP feature from
`ROADMAP.md` instead of rewording or parking it as near-term work.

**Rationale**: The roadmap should advertise concrete next-value slices. No
named MCP server or consumer target currently unlocks immediate value for Canon,
while Windows distribution and authoring/evidence quality have clearer product
traction.

**Alternatives considered**:

- Keep the section but mark it deferred: rejected because it still spends
  roadmap attention on a speculative direction.
- Replace it with a generic interoperability note: rejected because that would
  remain too abstract to guide next delivery work.

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/data-model.md
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

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/contracts/distribution-metadata.md
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

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/contracts/winget-manifest-bundle.md
# Contract: Winget Manifest Bundle

## Purpose

Define the publication artifact bundle Canon generates for Windows Package
Manager submission.

## Output Layout

The bundle MUST contain three YAML files for one Canon version:

```text
<bundle-root>/
├── ApplyThe.Canon.yaml
├── ApplyThe.Canon.installer.yaml
└── ApplyThe.Canon.locale.en-US.yaml
```

Alternative filenames that keep the same multi-file manifest semantics are
acceptable if they remain stable and documented.

## Required Version Manifest Fields

- `PackageIdentifier`
- `PackageVersion`
- `DefaultLocale`
- `ManifestType: version`
- `ManifestVersion: 1.12.0`

## Required Default Locale Manifest Fields

- `PackageIdentifier`
- `PackageVersion`
- `PackageLocale: en-US`
- `Publisher`
- `PackageName`
- `License`
- `ShortDescription`
- `ManifestType: defaultLocale`
- `ManifestVersion: 1.12.0`

## Required Installer Manifest Fields

- `PackageIdentifier`
- `PackageVersion`
- `Installers` with exactly one Windows x64 entry in this slice
- `InstallerType: zip`
- `NestedInstallerType: portable`
- `InstallerUrl` referencing the canonical GitHub Release zip
- `InstallerSha256` matching the release checksum manifest
- `NestedInstallerFiles` containing `canon.exe`
- `PortableCommandAlias: canon`
- `ManifestType: installer`
- `ManifestVersion: 1.12.0`

## Validation Rules

- The bundle MUST fail validation if the installer URL or checksum diverge from
  the release metadata.
- The installer bundle MUST not reference unsupported architectures or deferred
  Windows channels.
- The generated YAML MUST remain deterministic for the same release metadata
  input.

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/quickstart.md
# Quickstart: Winget Distribution And Roadmap Refocus

## Goal

Exercise the Windows distribution slice from release metadata generation through
artifact validation and documentation verification.

## Steps

1. Build or stage the standard release bundle so the Windows archive,
   release notes, and checksum manifest exist.
2. Generate distribution metadata from the release bundle.
3. Render the `winget` manifest bundle from that metadata.
4. Verify the release surface, including the Windows package-manager bundle.
5. Run the focused release and documentation tests for this feature.
6. Review the updated install docs, changelog, and roadmap to confirm Windows
   distribution guidance and MCP removal are consistent.

## Expected Result

- The release bundle includes a Windows package-manager publication artifact.
- Validation rejects missing or mismatched Windows metadata.
- Documentation names `winget` as the primary Windows package-manager path and
  keeps the archive fallback visible.
- `ROADMAP.md` no longer presents Protocol Interoperability / MCP as active
  next work.

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/decision-log.md
# Decision Log: Winget Distribution And Roadmap Refocus

## D-001: Choose `winget` as the next concrete distribution slice

- **Status**: Accepted
- **Rationale**: Windows distribution through `winget` has immediate user value,
  fits the existing GitHub Releases packaging posture, and is more concrete
  than speculative protocol work.

## D-002: Reuse the existing Windows zip as an archive installer with a nested portable executable

- **Status**: Accepted
- **Rationale**: The current release artifact already contains `canon.exe`, and
  the winget installer schema supports `zip` installers with
  `NestedInstallerType: portable` and nested file aliases.

## D-003: Keep final `winget-pkgs` submission manual in the first slice

- **Status**: Accepted
- **Rationale**: Repository-owned manifest generation is enough to create a
  durable artifact and release proof surface without pulling external-state
  automation into the first packaging increment.

## D-004: Remove Protocol Interoperability / MCP from the active roadmap

- **Status**: Accepted
- **Rationale**: No concrete MCP consumer or server target currently unlocks
  enough value to justify roadmap priority over packaging and authoring quality.

*** Add File: /Users/rt/workspace/apply-the/canon/specs/026-winget-distribution/validation-report.md
# Validation Report: Winget Distribution And Roadmap Refocus

## Planned Structural Validation

- Verify generated Windows package-manager artifacts against the declared
  release metadata and checksum surface.
- Validate release workflow, release helper scripts, and documentation edits for
  formatting or schema drift.

## Planned Logical Validation

- Add a focused release-surface test that exercises Windows metadata, manifest
  generation, and verification behavior.
- Extend documentation validation so Windows install guidance, changelog, and
  roadmap expectations remain aligned.

## Planned Independent Validation

- Perform an independent review of roadmap cleanup and Windows distribution
  evidence after code and docs are updated.
- Confirm the feature stays bounded to packaging and documentation surfaces and
  does not reintroduce protocol or MCP work through implementation drift.

## Open Validation Risks

- Windows Package Manager submission mechanics are external to this repository,
  so the first slice validates generated manifests and documented maintainer
  steps rather than live community-repository submission.
- The Windows archive currently packages a portable executable inside a zip, so
  manifest validation must explicitly confirm nested portable-file metadata.
