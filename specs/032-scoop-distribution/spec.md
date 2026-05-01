# Feature Specification: Scoop Distribution Follow-On

**Feature Branch**: `032-scoop-distribution`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Deliver Scoop as a secondary Windows distribution channel using the existing canonical GitHub Release archives, release metadata, and package-manager validation flow. Reuse the current distribution pipeline instead of introducing a parallel packaging path. Generate or update Scoop manifest artifacts from canonical release metadata, validate manifest URLs and checksums against verified release assets, update install and release-facing docs, keep GitHub Releases as the single source of truth, and include explicit planning for version bump, impacted docs plus changelog updates, coverage for modified or new Rust files, cargo clippy cleanup, and cargo fmt execution."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice extends the existing release and installation surface through an additional package-manager channel, but it remains inside repository-owned packaging, metadata, workflow, and documentation surfaces without changing Canon runtime semantics or widening trust boundaries.  
**Scope In**: repository-owned Scoop manifest rendering from canonical release metadata; release workflow wiring that emits or publishes Scoop-ready artifacts; validation that Scoop manifests reference verified GitHub Release URLs and checksums; install and release documentation for Scoop; roadmap cleanup that removes already-delivered features from the active remaining-candidates section; `0.32.0` release alignment across manifests, runtime compatibility references, docs, and changelog; focused coverage and validation closeout for changed or new Rust files.  
**Scope Out**: Homebrew or `winget` redesign; Debian or `apt` packaging; replacing GitHub Releases as the canonical source of truth; introducing a second build pipeline for Scoop; widening Windows architecture support beyond the already shipped release targets; unrelated runtime modes or governance changes.

**Invariants**:

- GitHub Releases MUST remain the canonical source of downloadable Canon binaries, filenames, and checksums.
- Scoop distribution MUST consume the same verified release metadata and packaged archives already used by the existing release surface rather than rebuilding Canon through a separate path.
- Existing direct-download installation guidance and the already shipped Homebrew and `winget` paths MUST remain valid fallbacks or peer channels.
- Core Canon runtime behavior, mode execution semantics, `.canon/` storage, and approval posture MUST remain unchanged by this feature.

**Decision Traceability**: Decisions and validation evidence for this slice MUST be recorded in `specs/032-scoop-distribution/decision-log.md` and `specs/032-scoop-distribution/validation-report.md`, with roadmap cleanup reflected in `ROADMAP.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish A Scoop-Ready Release Artifact (Priority: P1)

As a Canon maintainer, I want the release workflow to produce Scoop-ready
manifest artifacts from the canonical release bundle so I can publish the
secondary Windows package-manager channel without hand-deriving URLs,
filenames, or checksums.

**Why this priority**: This is the concrete MVP. Without a deterministic
Scoop manifest set derived from the current release surface, the follow-on
channel does not exist.

**Independent Test**: A maintainer can assemble a release bundle and obtain a
reviewable Scoop manifest artifact set whose URLs and checksums match the
verified GitHub Release assets.

**Acceptance Scenarios**:

1. **Given** a packaged Windows release asset and verified distribution
   metadata, **When** the release bundle is assembled, **Then** the system
   emits a Scoop manifest artifact set derived from the same metadata.
2. **Given** a release asset URL, filename, or checksum mismatch, **When** the
   Scoop manifest artifact is generated or verified, **Then** the release is
   marked invalid before Scoop publication is treated as ready.

---

### User Story 2 - Install Canon Through Scoop On Windows (Priority: P2)

As a Windows user, I want Canon to document a Scoop install and upgrade path so
I can use a familiar package manager instead of manually downloading archives.

**Why this priority**: Once the publication surface exists, end-user value comes
from making the channel discoverable and usable.

**Independent Test**: A reviewer can read the installation guidance and identify
the Scoop install command, Scoop upgrade path, and direct-download fallback
without external context.

**Acceptance Scenarios**:

1. **Given** the updated installation docs, **When** a Windows user looks for a
   Scoop path, **Then** the docs show a primary Scoop command plus the direct
   archive fallback.
2. **Given** a user already installed Canon through Scoop, **When** they look
   for update guidance, **Then** the docs point them to the same Scoop channel
   rather than a separate manual process.

---

### User Story 3 - Keep Distribution And Roadmap Surfaces Focused (Priority: P3)

As a Canon maintainer, I want the roadmap and release-facing docs to describe
only the actually remaining distribution follow-ons, so the next-feature view
stays concrete and the shipped channels remain auditable.

**Why this priority**: The roadmap and docs are part of the shipped contract.
Leaving completed items in the active-candidate section creates drift exactly
where future planning is supposed to be most trustworthy.

**Independent Test**: A reviewer can inspect the roadmap, release-facing docs,
and Scoop artifact contract and confirm they all describe the same delivered and
deferred channels without stale completed-feature sections.

**Acceptance Scenarios**:

1. **Given** the updated roadmap, **When** a maintainer reviews the remaining
   candidates, **Then** already delivered feature blocks are absent from the
   active-candidate section.
2. **Given** the updated release-facing docs, **When** a reviewer compares them
   with the Scoop artifact contract, **Then** the documented installation and
   deferred-channel statements remain aligned.

### Edge Cases

- What happens when the Windows release asset exists but the canonical
  distribution metadata omits the checksum or filename required by Scoop?
- How does the system handle a release candidate where `winget` remains valid
  but Scoop manifest generation points to a stale or renamed release asset?
- Which invariant is most likely to be stressed when installation docs drift
  away from the actual published package-manager commands or fallback paths?
- What happens when a Scoop publication path is configured but the release
  pipeline can only emit an artifact bundle rather than publish directly?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST derive Scoop publication artifacts from the
  canonical GitHub Release version, filenames, asset URLs, and checksums rather
  than from hand-authored package-manager values.
- **FR-002**: The system MUST preserve GitHub Releases as the canonical host
  for Windows binaries, release notes, and checksums even when Scoop support is
  added.
- **FR-003**: The system MUST emit a durable Scoop manifest artifact set that a
  maintainer can use to publish or update the Scoop channel for the current
  Canon version.
- **FR-004**: The system MUST validate that Scoop manifests reference packaged
  release assets that exist and match the declared canonical checksums.
- **FR-005**: The system MUST document Scoop as a supported Windows installation
  and upgrade path while preserving the direct-download archive fallback.
- **FR-006**: The system MUST describe the relationship between `winget` and
  Scoop clearly, including any deferred Windows packaging work that remains out
  of scope.
- **FR-007**: The feature MUST include an explicit version-bump task for
  `0.32.0` and an explicit impacted-docs-plus-changelog task in the generated
  task plan.
- **FR-008**: The feature MUST include an explicit coverage task for modified or
  new Rust files plus explicit `cargo clippy` and `cargo fmt` closeout tasks in
  the generated task plan.
- **FR-009**: The roadmap MUST stop presenting already delivered features as
  active remaining candidates.
- **FR-010**: The feature MUST record design decisions, scope boundaries, and
  validation evidence in durable artifacts before implementation is complete.
- **FR-011**: The release and documentation surface MUST remain understandable
  to the intended artifact audience: maintainers publishing releases and Windows
  users installing Canon.

### Key Entities *(include if feature involves data)*

- **Scoop Manifest Bundle**: The machine-readable Scoop definition set derived
  from Canon release metadata and used to publish or update the Scoop channel.
- **Windows Release Asset**: The canonical packaged Canon binary archive for a
  supported Windows target, including its filename, download URL, and checksum.
- **Distribution Metadata Record**: The release-surface metadata artifact that
  captures canonical asset identifiers and checksums across package-manager
  channels.
- **Distribution Channel Record**: A declared install path for a platform,
  including whether it is primary, secondary, fallback, or deferred.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For a release candidate, a maintainer can produce a complete
  Scoop manifest bundle from Canon release outputs in one bounded workflow
  without manually recalculating checksums.
- **SC-002**: A Windows installer can identify the Scoop install path and the
  direct-download fallback from the documentation in under 2 minutes.
- **SC-003**: Release validation rejects any Scoop manifest that references a
  missing release asset or mismatched checksum before publication is declared
  ready.
- **SC-004**: Roadmap readers encounter zero active remaining-candidate feature
  blocks for already delivered distribution and `system-assessment` work after
  this slice's cleanup lands.

## Validation Plan *(mandatory)*

- **Structural validation**: Focused release-surface validation for Scoop
  manifest artifacts, documentation consistency checks, formatting validation,
  and lint or manifest-shape checks for touched packaging files.
- **Logical validation**: Focused tests covering Scoop manifest rendering,
  release metadata alignment, installation-doc walkthroughs, and roadmap
  consistency scenarios.
- **Independent validation**: A separate review pass over roadmap cleanup,
  release-surface evidence, and Windows install guidance after implementation
  artifacts are generated.
- **Evidence artifacts**: `specs/032-scoop-distribution/validation-report.md`,
  focused tests under `tests/`, release-surface scripts and workflow evidence,
  updated docs, and `lcov.info` for changed Rust sources.

## Decision Log *(mandatory)*

- **D-001**: Choose Scoop as the next concrete distribution slice after
  Homebrew and `winget`, **Rationale**: it extends the current canonical release
  metadata and validation surfaces into a second Windows package-manager path
  without introducing a new packaging pipeline or runtime domain.

## Non-Goals

- Redesign the existing Homebrew or `winget` flows.
- Introduce Debian, `apt`, or other Linux distribution packaging.
- Replace GitHub Releases as the source of truth for distribution assets.
- Expand this slice into a generic multi-channel packaging framework beyond the
  Scoop follow-on.

## Assumptions

- Canon will continue to publish Windows archives to GitHub Releases using the
  current repository-owned release workflow.
- The existing distribution metadata surface is sufficient to drive Scoop after
  bounded extensions rather than a redesign.
- Maintainers want a repository-owned, reviewable Scoop artifact surface rather
  than an undocumented manual checklist.
- Future package-manager follow-ons, if any, remain deferred until the current
  set of distribution channels is stable and auditable.
