# Feature Specification: Ordered Artifact Filenames

**Feature Branch**: `046-ordered-artifact-filenames`
**Created**: 2026-05-12
**Status**: Draft
**Input**: User description: "Add numeric prefix ordering to Canon mode artifact filenames so emitted packets sort naturally like a Confluence page tree (01-, 02-, 03-). Derived from DFM for Escape architecture run analysis showing that unordered artifact sets are harder to navigate."

## Governance Context *(mandatory)*

**Mode**: change
**Risk Classification**: bounded-impact; the change touches every mode's artifact contract and rendering path, but preserves all existing content and governance semantics.
**Scope In**: artifact filename emission in `canon-engine`, artifact contract definitions, markdown renderer, publish paths, documentation, tests, changelog, and roadmap entry.
**Scope Out**: artifact content changes, mode semantics, gate logic, approval flow, governance adapter JSON shape, CLI argument surface, and skill file behavior.

**Invariants**:

- Every artifact file that existed before this change must still be emitted with identical content; only the filename gains a numeric prefix.
- Existing published packet directories and `.canon/artifacts/` layout must remain structurally compatible; the only difference is the filename prefix.
- The ordering must reflect the intended reading order of the packet, not alphabetical sort.

**Decision Traceability**: `specs/046-ordered-artifact-filenames/decision-log.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ordered artifact listing (Priority: P1)

When a user runs `canon run` in any mode, the emitted artifact files under `.canon/artifacts/<RUN_ID>/<mode>/` carry a two-digit numeric prefix (e.g. `01-architecture-overview.md`, `02-architecture-decisions.md`) so that `ls` and file browsers show them in reading order.

**Why this priority**: This is the core behavior change. Without it, artifact directories sort alphabetically and the reading order is lost.

**Independent Test**: Run `canon run --mode architecture` with a valid brief, then list the artifact directory and verify every filename starts with a two-digit prefix and the sequence matches the intended packet reading order.

**Acceptance Scenarios**:

1. **Given** a valid architecture brief, **When** `canon run --mode architecture` completes, **Then** every emitted artifact filename starts with `NN-` where NN is a zero-padded two-digit number.
2. **Given** any mode, **When** the run completes, **Then** the numeric ordering matches the artifact family ordering defined in the mode profile.
3. **Given** a mode with optional artifacts (e.g. architecture component-view), **When** the optional artifact is omitted, **Then** the remaining files keep contiguous numbering without gaps.

---

### User Story 2 - Published packets preserve ordering (Priority: P2)

When a user runs `canon publish <RUN_ID>`, the published files in the destination directory carry the same numeric prefix ordering as the artifacts under `.canon/`.

**Why this priority**: Publishing is the public handoff; if it drops the prefix the reading order is lost for reviewers.

**Independent Test**: Run `canon publish` after a completed run, then verify the published filenames match the prefixed artifact filenames.

**Acceptance Scenarios**:

1. **Given** a completed run with prefixed artifacts, **When** `canon publish <RUN_ID>` completes, **Then** every published file retains its numeric prefix.
2. **Given** a published directory, **When** a reviewer lists the files, **Then** the default sort matches the intended reading order.

---

### User Story 3 - Manifest and metadata reference new filenames (Priority: P3)

Machine-readable manifests (`view-manifest.json`, `packet-metadata.json`, artifact contract TOML) reference the prefixed filenames so downstream tooling resolves them correctly.

**Why this priority**: Without updated references, programmatic consumers break.

**Independent Test**: Parse the manifest JSON after a run and verify every artifact path uses the prefixed filename.

**Acceptance Scenarios**:

1. **Given** a completed architecture run, **When** `view-manifest.json` is parsed, **Then** every artifact path contains the `NN-` prefix.
2. **Given** a completed run in any mode, **When** `packet-metadata.json` is parsed, **Then** the `primary_artifact` value uses the prefixed filename.

---

### Edge Cases

- What happens when a mode emits more than 99 artifacts? Use three-digit prefixes only if the count exceeds 99; otherwise two digits suffice. Current maximum is 15 (architecture).
- How does the system handle optional artifacts that are omitted? Remaining files use contiguous numbering; no gaps.
- What happens to `architecture-overview.md` being referenced as the primary artifact in run summaries? The reference updates to `01-architecture-overview.md`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Every artifact filename emitted by `canon run` in any mode MUST start with a zero-padded two-digit prefix followed by a hyphen (e.g. `01-problem-statement.md`).
- **FR-002**: The numeric order MUST reflect the intended reading order of the packet, matching the `artifact_families` ordering in the mode profile.
- **FR-003**: Optional artifacts that are omitted MUST NOT leave gaps in the numbering sequence.
- **FR-004**: `canon publish` MUST preserve the numeric prefix in the published destination.
- **FR-005**: Machine-readable manifests (`view-manifest.json`, `packet-metadata.json`) MUST reference the prefixed filenames.
- **FR-006**: Existing artifact content MUST NOT change; only the filename prefix is added.
- **FR-007**: Mermaid sidecar files (`.mmd`) MUST share the same numeric prefix as their companion markdown artifact.

### Key Entities

- **ArtifactFilename**: the emitted filename; gains a `NN-` prefix while keeping the existing slug and extension.
- **ModeProfile.artifact_families**: the source of truth for the reading order within each mode.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of emitted artifact filenames across all 16 modes carry a valid numeric prefix.
- **SC-002**: `ls` output of any artifact directory matches the intended reading order without resorting.
- **SC-003**: All existing tests continue to pass after updating expected filenames.
- **SC-004**: Coverage for modified Rust files stays at or above 95%.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **Logical validation**: `cargo nextest run --workspace --all-features`; targeted contract and integration tests for every mode's artifact emission.
- **Independent validation**: `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` with 95% threshold on modified files.
- **Evidence artifacts**: `specs/046-ordered-artifact-filenames/validation-report.md`

## Decision Log *(mandatory)*

- **D-001**: Use two-digit zero-padded prefix (`01-`, `02-`, ...) rather than three-digit. **Rationale**: Current maximum artifact count per mode is 15; two digits are sufficient and more readable.
- **D-002**: Derive ordering from `artifact_families` in `ModeProfile` rather than alphabetical. **Rationale**: The mode profile already encodes the intended packet structure.
- **D-003**: Keep contiguous numbering when optional artifacts are omitted rather than reserving slots. **Rationale**: Gaps confuse readers and suggest missing files.

## Non-Goals

- Changing artifact content or structure.
- Adding new modes or artifact families.
- Changing CLI arguments or governance adapter JSON contract.
- Reordering or renaming the `artifact_families` entries in mode profiles.
