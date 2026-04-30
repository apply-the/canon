# Feature Specification: Distribution Channels Beyond GitHub Releases

**Feature Branch**: `025-distribution-channels`  
**Created**: 2026-04-29  
**Status**: Draft  
**Input**: User description: "Create Feature 025 Distribution Channels Beyond GitHub Releases. Deliver a first slice that makes Canon installable through Homebrew using the existing GitHub Release archives as the single source of truth. Reuse the current release packaging and verification flow, add machine-readable release metadata that records asset URLs, checksums, and canonical filenames, and generate or update the Homebrew formula from that metadata in a dedicated tap repository workflow. This slice must not introduce a parallel packaging pipeline. GitHub Releases remain canonical. The implementation must stay forward-compatible with future winget and Scoop support, but 025 should only ship Homebrew behavior end-to-end. The feature must include release metadata generation from the existing release workflow, Homebrew formula rendering or update flow, validation that formula URLs and sha256 values match verified release artifacts, install documentation updates, and tests or verification coverage for release metadata and formula correctness. Out of scope for 025: shipping winget manifests, shipping Scoop manifests, apt or deb packaging, and changing the existing archive naming convention unless strictly required."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice extends Canon's release and install surfaces, introduces channel-facing metadata, and adds distribution automation that can affect user installation paths across supported platforms, but it does not alter Canon's core runtime, mode model, or governance semantics.  
**Scope In**:

- Add a first package-manager distribution slice for Canon through Homebrew.
- Advance Canon release-facing version surfaces to `0.25.0` for this delivery.
- Keep GitHub Releases as the canonical source of downloadable binaries and checksums.
- Add machine-readable release metadata that describes the canonical release assets needed by package-manager channels.
- Add a Homebrew formula generation or update flow grounded in canonical release assets rather than hand-maintained package metadata.
- Support a dedicated Homebrew tap repository integration path or an equivalent ready-to-apply tap update artifact when direct publication is unavailable.
- Add release-surface validation that proves formula URLs and checksums match the already verified release assets.
- Update install documentation and release-facing guidance to describe the Homebrew path alongside the existing direct-download fallback.
- Preserve forward compatibility for future `winget` and Scoop work by shaping the release metadata contract around reusable asset identifiers and checksums.

**Scope Out**:

- Shipping `winget` manifests in this slice.
- Shipping Scoop manifests in this slice.
- Adding `apt`, Debian, or other Linux distro packaging.
- Replacing GitHub Releases as the release source of truth.
- Introducing a parallel build or packaging pipeline outside the existing release workflow.
- Changing Canon's existing archive naming convention unless a change is required to preserve release-surface correctness.

**Invariants**:

- GitHub Releases MUST remain the single source of truth for Canon distribution artifacts in this slice.
- Homebrew distribution MUST consume the same canonical archives and checksums already validated by the release workflow rather than rebuilding Canon from a separate path.
- Existing direct archive installation instructions MUST remain available as a supported fallback.
- Core Canon runtime behavior, mode execution semantics, and governed packet behavior MUST remain unchanged by this feature.

**Decision Traceability**: Decisions for this feature begin in this specification and continue in `specs/025-distribution-channels/decision-log.md`, with validation evidence recorded in `specs/025-distribution-channels/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install Canon Through Homebrew (Priority: P1)

As a Canon user on macOS or Linux, I want to install and upgrade Canon through
Homebrew so I do not need to download and unpack release archives manually.

**Why this priority**: This is the first user-visible outcome of the feature.
Without a working Homebrew path, the distribution-channel slice does not
deliver end-user value.

**Independent Test**: On a supported macOS or Linux environment, a user can
install Canon from the Homebrew tap, run `canon --version`, and observe the
released version without manual archive extraction.

**Acceptance Scenarios**:

1. **Given** a Canon version has been released with the canonical verified
   archives, **When** a user installs Canon from the Homebrew tap, **Then**
   the installed binary version matches the released version.
2. **Given** a user already has an older Homebrew-installed version, **When**
   the tap is updated for a newer release, **Then** the user can upgrade
   through Homebrew without changing the manual archive fallback path.
3. **Given** a release asset or checksum is incomplete for a Homebrew-supported
   platform, **When** the channel update would otherwise be generated,
   **Then** the Homebrew distribution surface fails closed instead of pointing
   users at an invalid artifact.

---

### User Story 2 - Publish A Canonical Homebrew Update From The Release Flow (Priority: P2)

As a Canon maintainer, I want the existing release flow to generate the
metadata and Homebrew update inputs automatically so I do not maintain package
manager state by hand.

**Why this priority**: End-user installability is only sustainable if the
release workflow remains the authoritative packaging path instead of requiring
manual formula edits after every tag.

**Independent Test**: From a valid release candidate, a maintainer can produce
machine-readable release metadata plus a Homebrew formula update artifact or
tap-repository sync path whose URLs and checksums match the verified release
bundle.

**Acceptance Scenarios**:

1. **Given** the release workflow has produced the canonical archives and
   checksum manifest, **When** the distribution-channel step runs, **Then** it
   emits machine-readable release metadata derived from those verified assets.
2. **Given** the metadata is valid, **When** the Homebrew update step runs,
   **Then** it generates or updates the formula using canonical asset URLs and
   checksums instead of hand-maintained package values.
3. **Given** direct tap publication is not available, **When** the Homebrew
   update step completes, **Then** it still emits a durable ready-to-apply
   formula artifact instead of silently dropping the package-manager output.

---

### User Story 3 - Verify Release And Install Surfaces Stay Consistent (Priority: P3)

As a release reviewer, I want the distribution metadata, Homebrew formula, and
install documentation to stay consistent with the verified release bundle so
future distribution work can build on a trustworthy contract.

**Why this priority**: Distribution channels become a long-term maintenance
surface. If the metadata contract, formula, and docs drift from the canonical
release assets, every later channel will amplify that inconsistency.

**Independent Test**: A reviewer can inspect the release bundle, distribution
metadata, formula output, and install docs and confirm they all point to the
same canonical assets and checksums.

**Acceptance Scenarios**:

1. **Given** a release bundle has been assembled, **When** validation runs,
   **Then** it proves the Homebrew formula URLs and sha256 values match the
   already verified release assets.
2. **Given** install documentation references Homebrew, **When** a reviewer
   compares docs with the release metadata, **Then** the documented install
   path matches the shipped distribution surface and keeps the direct-download
   fallback visible.
3. **Given** future Windows channels are not yet shipped, **When** a reviewer
   inspects the release metadata contract, **Then** it remains broad enough to
   support future `winget` and Scoop slices without introducing release-note
   scraping or a second artifact inventory.

### Edge Cases

- A release is missing one archive needed by a Homebrew-supported platform; the
  distribution update must fail closed rather than publish a partial formula.
- Release metadata and checksum evidence disagree for the same asset; the
  release must surface an explicit validation failure.
- A Homebrew tap publication step cannot authenticate or reach the tap
  repository; the feature must preserve a durable formula artifact for manual
  application rather than silently succeeding.
- A future archive for an unsupported platform appears in GitHub Releases; the
  first slice must ignore it unless the Homebrew contract explicitly expands.
- A manual-install user still follows the archive-based path; the existing
  fallback instructions must remain valid after the new channel lands.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon's tagged release flow MUST continue to treat GitHub
  Releases as the single source of truth for published distribution artifacts.
- **FR-002**: The release flow MUST emit machine-readable release metadata for
  each released version that identifies the canonical asset filenames, download
  locations, version, and checksums needed by package-manager channels.
- **FR-003**: The release metadata MUST be derived from the already verified
  release bundle rather than hand-maintained separately.
- **FR-004**: The first slice MUST support Homebrew as a user-facing install
  path for Canon on supported macOS and Linux targets.
- **FR-005**: The Homebrew distribution path MUST install Canon from canonical
  prebuilt release artifacts instead of building the binary from a second build
  path.
- **FR-006**: The distribution flow MUST generate or update Homebrew formula
  content from the canonical release metadata rather than hardcoded package
  values.
- **FR-007**: The repository MUST support a dedicated Homebrew tap repository
  integration path or emit an equivalent ready-to-apply tap update artifact
  when direct tap publication is unavailable.
- **FR-008**: If a required release asset, URL, or checksum for the Homebrew
  contract is missing or inconsistent, the distribution update MUST fail closed
  and report the mismatch explicitly.
- **FR-009**: The existing direct-download installation path documented for
  Canon MUST remain supported in parallel with the Homebrew channel.
- **FR-010**: The feature MUST NOT introduce a parallel packaging pipeline for
  Homebrew beyond the existing release packaging and verification flow.
- **FR-011**: The feature MUST NOT require a change to the existing release
  archive naming convention unless such a change is required to preserve
  canonical asset correctness.
- **FR-012**: Release-facing documentation MUST describe the Homebrew install
  path and keep the direct-download fallback visible.
- **FR-013**: Validation surfaces MUST prove that generated Homebrew formula
  URLs and checksums match the canonical release bundle.
- **FR-014**: The distribution metadata contract MUST remain reusable for later
  `winget` and Scoop work without implementing those channels in this slice.
- **FR-015**: The feature MUST add focused automated validation or structured
  release-surface checks for distribution metadata correctness and Homebrew
  formula correctness.
- **FR-016**: The feature MUST preserve Canon's core runtime behavior, mode
  execution surfaces, and governed packet semantics unchanged.
- **FR-017**: Release-facing version surfaces for this feature delivery MUST be
  advanced consistently to `0.25.0`.

### Key Entities *(include if feature involves data)*

- **Release Metadata Record**: The machine-readable description of a Canon
  release version, its canonical asset filenames, download locations, and
  checksums.
- **Homebrew Formula Update**: The rendered or synchronized package-manager
  artifact that maps a Canon release to the Homebrew install surface.
- **Distribution Publication Attempt**: The recorded result of trying to sync
  the Homebrew update to its destination, including success, failure, or
  artifact-only fallback.
- **Verified Release Bundle**: The already validated set of release archives,
  checksum manifest, and release notes that the distribution surface must reuse.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A supported macOS or Linux user can install Canon through
  Homebrew and observe a `canon --version` value that matches the released
  version.
- **SC-002**: Every tagged release produces one machine-readable metadata
  artifact that fully describes the Homebrew-supported assets and their
  checksums.
- **SC-003**: Distribution validation blocks publication whenever a formula URL
  or checksum diverges from the verified release bundle.
- **SC-004**: Canon's install documentation presents a working Homebrew path
  while preserving a visible direct-download fallback for the same release.

## Validation Plan *(mandatory)*

- **Structural validation**: Release metadata schema checks, release-surface
  validation, formula rendering checks, and documentation link or reference
  verification.
- **Logical validation**: Channel-specific smoke validation for the Homebrew
  install path, checksum-to-formula consistency checks, and release workflow
  scenarios covering both tap publication and artifact-only fallback.
- **Independent validation**: Separate review of release metadata, generated
  formula output, and install documentation against the verified release bundle.
- **Evidence artifacts**: Validation results and findings recorded in
  `specs/025-distribution-channels/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Keep GitHub Releases as the source of truth and layer Homebrew on
  top of the existing release artifacts, **Rationale**: this minimizes new
  packaging paths and keeps distribution metadata anchored to the already
  verified release bundle.

## Non-Goals

- Shipping `winget` support in this slice.
- Shipping Scoop support in this slice.
- Adding `apt`, Debian, or other Linux distro packaging.
- Replacing archive-based installation as a documented fallback.
- Reworking Canon's runtime or governed execution model as part of package
  distribution.

## Assumptions

- Canon's existing release workflow remains the canonical place where tagged
  release archives are produced and verified.
- A dedicated Homebrew tap repository can exist separately from this repo, or a
  durable formula artifact can be consumed manually until full tap publication
  is configured.
- The current archive naming convention is sufficiently stable to anchor the
  first distribution-channel slice.
- Later Windows channel work will consume the same metadata contract rather
  than re-inventing a separate release inventory.
