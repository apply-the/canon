# Feature Specification: Winget Distribution And Roadmap Refocus

**Feature Branch**: `026-winget-distribution`  
**Created**: 2026-04-30  
**Status**: Draft  
**Input**: User description: "Remove MCP from the roadmap, prioritize stronger next-feature candidates, and implement the concrete Windows distribution follow-up with Speckit artifacts and delivery."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact; this slice changes the existing release and install surface, but it stays inside repository-owned packaging, documentation, and release automation without altering Canon runtime semantics or widening execution trust boundaries.  
**Scope In**: Windows package-manager distribution through `winget`; release-surface artifacts needed to publish Windows package metadata from GitHub Releases; install and release documentation for the Windows package-manager path; roadmap cleanup that removes MCP / Protocol Interoperability as an active next-feature candidate and keeps future packaging work grounded in concrete channels.  
**Scope Out**: Homebrew delivery; Scoop delivery; Debian or `apt` packaging; runtime MCP adapter enablement; Canon as an MCP server; protocol-interoperability architecture work; new non-distribution runtime modes.

**Invariants**:

- GitHub Releases MUST remain the canonical source of downloadable Canon binaries and checksums.
- Windows direct-download installation MUST remain available as a fallback when `winget` is unavailable or not yet refreshed.
- This slice MUST NOT introduce protocol-interoperability runtime behavior, new trust boundaries, or non-distribution feature scope under the packaging umbrella.

**Decision Traceability**: Decisions and validation evidence for this slice MUST be recorded in `specs/026-winget-distribution/decision-log.md` and `specs/026-winget-distribution/validation-report.md`, with any product-priority cleanup reflected in `ROADMAP.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish A Winget-Ready Release (Priority: P1)

As a Canon maintainer, I want a release process that produces Windows package-manager publication inputs from the same GitHub Release assets we already ship, so I can publish a Windows install channel without hand-deriving URLs, version strings, or checksums.

**Why this priority**: This is the concrete next distribution step with immediate user-facing value and the clearest path from current release artifacts to a real install channel.

**Independent Test**: A maintainer can run the release packaging flow for a version and obtain a complete Windows package-manager manifest set that references the published release asset and passes focused validation without changing the release asset layout.

**Acceptance Scenarios**:

1. **Given** a packaged Windows release asset and checksum manifest, **When** the release bundle is assembled, **Then** the system includes a Windows package-manager publication artifact derived from the same release metadata.
2. **Given** a release artifact or checksum mismatch, **When** the Windows package-manager publication artifact is generated or verified, **Then** the release is marked invalid before publication guidance is treated as ready.

---

### User Story 2 - Install Canon On Windows Through A Familiar Channel (Priority: P2)

As a Windows user, I want Canon to document and publish a primary `winget` install path, so I can install or upgrade Canon without manually downloading and unpacking archives.

**Why this priority**: Once the publication surface exists, the user-facing value comes from making Windows installation discoverable and routine.

**Independent Test**: A reviewer can read the install guidance and identify the primary Windows package-manager command, the fallback archive path, and the supported architecture without consulting chat history or ad hoc maintainer notes.

**Acceptance Scenarios**:

1. **Given** the published install documentation, **When** a Windows user looks for installation instructions, **Then** the primary package-manager path and archive fallback are both explicit.
2. **Given** a Windows user who already installed Canon through the package manager, **When** they look for upgrade guidance, **Then** the documentation points them to the same channel rather than a separate manual process.

---

### User Story 3 - Keep The Roadmap Focused On Concrete Value (Priority: P3)

As a Canon maintainer, I want the roadmap to stop advertising protocol work with no immediate unlock, so future planning stays anchored to concrete distribution, authoring, and evidence improvements.

**Why this priority**: Removing speculative next-feature direction reduces drift and keeps roadmap energy on slices with immediate delivery value.

**Independent Test**: A roadmap reader can review the next-feature section and find Windows distribution plus established authoring/evidence directions, without seeing MCP or Protocol Interoperability presented as active next work.

**Acceptance Scenarios**:

1. **Given** the updated roadmap, **When** a maintainer reviews the next-feature candidates, **Then** Protocol Interoperability is no longer listed as an active roadmap feature.
2. **Given** the updated roadmap, **When** a maintainer scans the distribution section, **Then** the next concrete Windows slice is explicit and deferred channels remain clearly bounded.

### Edge Cases

- What happens when the Windows release asset exists but its checksum entry is missing or differs from the package-manager publication input?
- How does the system handle a release candidate where `winget` publication inputs are ready but the canonical GitHub Release asset was not published yet?
- Which invariant is most likely to be stressed when Windows package-manager guidance drifts away from the direct-download fallback or from the canonical release URL structure?
- What happens when a future Windows architecture is discussed before the release pipeline produces a matching packaged binary?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST derive Windows package-manager publication inputs from the canonical GitHub Release version, asset URL, and checksum surface rather than from hand-authored release notes.
- **FR-002**: The system MUST preserve GitHub Releases as the canonical host for Windows binaries, release notes, and checksums even when a Windows package-manager channel is added.
- **FR-003**: The system MUST emit a durable Windows package-manager artifact bundle that a maintainer can use to publish or update the `winget` package definition for the current Canon version.
- **FR-004**: The system MUST validate that Windows package-manager publication artifacts reference a packaged Windows binary that exists in the release surface and matches the declared checksum.
- **FR-005**: The system MUST document `winget` as the primary Windows package-manager installation path while preserving the existing direct-download archive fallback.
- **FR-006**: The system MUST describe the supported Windows distribution scope clearly, including the deferred status of Scoop and any unsupported Windows architectures.
- **FR-007**: The roadmap MUST remove Protocol Interoperability / MCP from the active next-feature list and re-center the next-feature narrative on concrete distribution and authoring/evidence work.
- **FR-008**: The feature MUST record design decisions, scope boundaries, and validation evidence in durable artifacts before implementation is considered complete.
- **FR-009**: The release and documentation surface MUST remain understandable to the intended artifact audience: maintainers publishing releases and Windows users installing Canon.

### Key Entities *(include if feature involves data)*

- **Windows Release Asset**: The packaged Canon binary archive for the supported Windows target, including its versioned filename, download location, and checksum.
- **Winget Publication Artifact**: The machine-readable package-manager definition set derived from Canon release metadata and used to publish or update the Windows channel.
- **Distribution Channel Record**: The declared install path for a platform, including whether it is primary, fallback, or deferred.
- **Roadmap Candidate**: A named next-feature direction with an explicit justification for why it remains active or is removed.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For a release candidate, a maintainer can produce a complete Windows package-manager publication bundle from Canon release outputs in one bounded workflow without manually recalculating checksums.
- **SC-002**: A Windows installer can identify the primary install path and fallback archive path from the documentation in under 2 minutes.
- **SC-003**: Release validation rejects any Windows package-manager publication artifact that references a missing Windows binary or mismatched checksum before release publication is declared ready.
- **SC-004**: Roadmap readers encounter zero active next-feature references to MCP, Protocol Interoperability, or speculative protocol-server work after this slice lands.

## Validation Plan *(mandatory)*

- **Structural validation**: Focused release-surface validation for Windows package-manager artifacts, documentation consistency checks, schema or manifest-shape checks for generated Windows publication files, and formatting / lint validation for touched repository documents.
- **Logical validation**: Focused release and packaging tests covering Windows artifact metadata, package-manager publication artifact generation, and install-doc plus roadmap walkthrough scenarios.
- **Independent validation**: A separate review pass over roadmap scope cleanup, release-surface evidence, and Windows install guidance after implementation artifacts are generated.
- **Evidence artifacts**: `specs/026-winget-distribution/validation-report.md`, focused tests under `tests/`, release-surface scripts and workflow evidence, and updated user-facing docs plus changelog entries.

## Decision Log *(mandatory)*

- **D-001**: Choose `winget` as the next concrete distribution slice and remove MCP / Protocol Interoperability from the active roadmap, **Rationale**: `winget` extends an existing release surface into immediate Windows user value, while MCP has no concrete near-term server or consumer target that justifies roadmap priority today.

## Non-Goals

- Introduce Homebrew, Scoop, `apt`, or any other new package-manager channel beyond the Windows `winget` slice.
- Enable MCP runtime adapters, build Canon as an MCP server, or add protocol-interoperability abstractions to the core runtime.
- Expand this feature into a broad packaging rewrite or a generic multi-channel distribution framework.

## Assumptions

- Canon will continue to publish versioned Windows archives to GitHub Releases for the supported Windows target.
- `winget` publication can remain bounded to the currently supported Windows architecture instead of widening Windows target coverage in the same slice.
- Maintainers want a repository-owned, reviewable Windows package-manager artifact surface rather than an undocumented manual checklist.
- Future distribution follow-ons may revisit Scoop or other channels later, but they are intentionally deferred until `winget` is stable.
