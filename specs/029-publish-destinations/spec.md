# Feature Specification: Structured External Publish Destinations

**Feature Branch**: `029-publish-destinations`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Implement structured external publish destinations so published Canon packets land outside `.canon/` into human-browsable directory structures with date-prefixed descriptors and preserved run-id traceability in packet metadata. Keep `.canon/` as runtime and evidence storage only. Include a task for the version bump, a task for impacted docs and changelog updates, coverage for modified or new Rust files, clippy cleanup, and cargo fmt execution."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice changes the cross-mode
publish surface, publish metadata, docs, and release-facing version anchors,
but it does not change `.canon/` runtime storage, approval semantics, run
identity, or the publish CLI contract itself.  
**Scope In**:

- Replace default run-id-only publish destinations with structured,
  human-browsable external paths for publishable Canon packets.
- Preserve run-id traceability through published metadata instead of relying on
  the path name alone.
- Keep explicit `publish --to` overrides working while making default publish
  destinations predictable and reviewable.
- Update impacted runtime docs, examples, guides, roadmap, changelog, and
  version surfaces for the delivered `0.29.0` slice.
- Add focused validation and coverage for every modified or newly created Rust
  file in the publish-path implementation.

**Scope Out**:

- Changes to `.canon/` runtime persistence layout, run directories, or
  evidence storage.
- New remote publish destinations, hosted publishing, or network adapters.
- New approval, risk, or recommendation-only semantics.
- CLI redesign beyond preserving the existing `publish` command and `--to`
  override behavior.

**Invariants**:

- `.canon/` remains the governed runtime and evidence storage surface; publish
  remains an external materialization step.
- Run identity stays anchored to the existing `run_id`; structured publish
  paths must not replace or weaken canonical run traceability.
- Approval-gated operational packets remain publishable only where current
  policy already allows them.
- Explicit publish destination overrides remain supported and must not be
  silently rewritten into the default structured layout.

**Decision Traceability**: Decisions and tradeoffs for this feature will be
recorded in `specs/029-publish-destinations/decision-log.md`, with validation
evidence captured in `specs/029-publish-destinations/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Browse Published Packets By Meaningful Path (Priority: P1)

As a maintainer browsing published Canon output in the repository, I want
default publish destinations to use readable family folders plus a
date-prefixed descriptor instead of a raw run-id-only directory, so I can tell
what a packet is from the path alone.

**Why this priority**: This is the visible product outcome of the feature and
unlocks the main repository-navigation value without requiring any new publish
transport or mode semantics.

**Independent Test**: Publish one completed run without `--to` and verify the
packet lands under a structured default path that includes the existing family
root plus a date-prefixed descriptor rather than only the run id.

**Acceptance Scenarios**:

1. **Given** a completed publishable run with descriptive metadata, **When** a
   maintainer runs `publish` without an override, **Then** the packet lands in
   a structured default directory under the correct external family root using
   a date-prefixed descriptor.
2. **Given** a completed publishable run without a useful authored descriptor,
   **When** a maintainer runs `publish`, **Then** the packet still lands in a
   structured default directory using a stable fallback descriptor rather than
   failing or reusing the run id as the only visible label.
3. **Given** a maintainer provides an explicit publish destination override,
   **When** `publish` runs, **Then** Canon writes to that override path instead
   of forcing the default structured location.

---

### User Story 2 - Recover Traceability From Published Metadata (Priority: P2)

As a reviewer reading a published packet outside `.canon/`, I want run id,
mode, risk, zone, publish timestamp, and source lineage to remain recoverable
from the published output itself, so structured paths do not weaken auditability.

**Why this priority**: Once the path stops being run-id-only, traceability must
be intentionally preserved in the published packet and not left to operator
guesswork.

**Independent Test**: Publish one run and verify the published output includes
machine-readable or reviewer-readable metadata sufficient to recover its
runtime identity and lineage without opening `.canon/`.

**Acceptance Scenarios**:

1. **Given** a publishable run, **When** Canon publishes the packet, **Then**
   the published output preserves run id, mode, risk, zone, publish timestamp,
   and source artifact lineage in a durable metadata surface.
2. **Given** a packet published under a structured path, **When** a reviewer
   inspects only the published output, **Then** they can recover the canonical
   run id and originating packet family without consulting chat history.
3. **Given** an approval-gated operational packet that is publishable under
   existing policy, **When** it is published, **Then** the same structured path
   and metadata contract apply without weakening the gate posture.

---

### User Story 3 - Ship 0.29.0 With Aligned Release Surfaces And Validation (Priority: P3)

As a maintainer shipping this slice, I want the version bump, impacted docs,
changelog, coverage expectations, `cargo clippy`, and `cargo fmt` explicitly
tracked in the feature workflow, so the release surface is trustworthy and the
publish contract does not drift from the code.

**Why this priority**: This repo treats docs, compatibility anchors, and final
validation as part of the delivered contract rather than post-feature cleanup.

**Independent Test**: Inspect the generated task list and final validation
artifacts to confirm there are explicit tasks for version bump, docs plus
changelog updates, coverage for touched Rust files, `cargo clippy`, and
`cargo fmt`.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects the
   release surfaces, **Then** `Cargo.toml`, `Cargo.lock`, shared runtime
   compatibility references, docs, and changelog consistently report `0.29.0`.
2. **Given** the completed task plan, **When** a maintainer reviews it,
   **Then** it includes an explicit version-bump task and an explicit impacted
   docs plus changelog task.
3. **Given** the final validation report, **When** a maintainer reviews it,
   **Then** it records coverage for every modified or new Rust file together
   with clean `cargo clippy` and `cargo fmt` execution.

### Edge Cases

- A run lacks a usable title or slug; the default publish path must still be
  readable and stable without inventing identity semantics.
- Two runs on the same date resolve to the same descriptor candidate; the
  publish layout must stay unambiguous without discarding canonical run
  traceability.
- A publish destination override points outside the repository root; Canon must
  preserve the override behavior instead of forcing repo-relative structure.
- A destination path already exists as a file or collides with a non-directory
  surface; publish must fail explicitly instead of partially writing output.
- Approval-gated operational packets must continue to honor existing publish
  allowances and must not become more permissive because the path contract
  changed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST materialize default published packets under external
  family roots using a canonical structure equivalent to
  `<publish-root>/<family>/<YYYY-MM-DD>-<descriptor>/`.
- **FR-002**: Canon MUST derive the default publish descriptor from authored or
  persisted packet metadata when available and MUST fall back to a stable,
  human-readable descriptor when no better label exists.
- **FR-003**: Canon MUST preserve canonical run traceability in published
  output metadata so reviewers can recover the originating run without relying
  on a run-id-only directory name.
- **FR-004**: Published metadata MUST preserve at least the run id, mode, risk,
  zone, publish timestamp, and source artifact lineage for the packet.
- **FR-005**: Explicit publish destination overrides MUST continue to work and
  MUST NOT be silently rewritten into the default structured destination.
- **FR-006**: Existing publish eligibility rules for completed and approval-
  gated operational packets MUST remain unchanged.
- **FR-007**: Canon MUST continue to reject publish destinations that resolve to
  existing non-directory filesystem entries.
- **FR-008**: Default structured publish destinations MUST remain outside
  `.canon/`; the feature MUST NOT turn `.canon/` into the primary reading
  surface.
- **FR-009**: Impacted guides, templates, examples, and release-facing docs
  MUST describe the structured publish contract accurately without overstating
  behavior.
- **FR-010**: Cargo manifests, lockfile surfaces, shared runtime compatibility
  references, and release-facing docs MUST align to `0.29.0` for this slice.
- **FR-011**: The generated task plan MUST include an explicit version-bump
  task, an explicit impacted-docs-and-changelog task, a coverage task for
  modified or new Rust files, a `cargo clippy` task, and a `cargo fmt` task.
- **FR-012**: Modified or newly created Rust files in the publish-path slice
  MUST receive focused automated validation coverage before the feature is
  complete.
- **FR-013**: Existing CLI semantics for `publish`, including the `run_id`
  input contract and `--to` option, MUST remain stable.
- **FR-014**: The feature MUST NOT change run identity generation, `.canon/`
  storage layout, or approval-resolution flows.

### Key Entities *(include if feature involves data)*

- **Publish Destination Descriptor**: The human-readable label used in the
  default external publish path alongside the publish date.
- **Published Packet Metadata**: The durable identity and lineage information
  materialized with a published packet so it remains traceable outside
  `.canon/`.
- **Publish Family Root**: The external repository-facing directory group for a
  packet family, such as architecture, changes, reviews, incidents, or specs.
- **Source Artifact Lineage**: The list of runtime-generated artifacts from the
  originating run that were materialized into the published packet.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can inspect the default publish path of any covered
  packet and identify the packet family and a human-readable descriptor without
  needing the raw run id in the path name.
- **SC-002**: A reviewer can recover the canonical run id, mode, risk, and
  source lineage from the published packet itself without opening `.canon/`.
- **SC-003**: Explicit publish destination overrides continue to succeed for
  valid paths and continue to fail cleanly for invalid file destinations.
- **SC-004**: Release-facing version surfaces and impacted docs consistently
  describe `0.29.0` and the structured publish contract for this slice.
- **SC-005**: Every modified or newly added Rust file in the implementation is
  covered by focused automated validation and the final validation record is
  clean on `cargo fmt` and `cargo clippy` expectations.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets --all-features -- -D warnings`, publish-path unit tests,
  release-surface consistency checks, and skill or docs sync checks where
  relevant.
- **Logical validation**: Focused publish-path tests for default destinations,
  metadata materialization, override preservation, approval-gated operational
  publishing, and at least one release-surface regression test for `0.29.0`.
- **Independent validation**: Read-only review of the final diff and publish
  outputs to confirm traceability remains explicit and `.canon/` stays runtime-
  only.
- **Evidence artifacts**: `specs/029-publish-destinations/validation-report.md`,
  focused test outputs, `lcov.info`, and the final release-facing doc diffs.

## Decision Log *(mandatory)*

- **D-001**: Keep the first slice centered on default publish destination
  structure plus published metadata, **Rationale**: this delivers the browsing
  and traceability value without widening scope into remote transport or a new
  publish CLI contract.

## Non-Goals

- Adding hosted or remote publish targets.
- Changing `.canon/` into a human-browsable published-document surface.
- Replacing canonical `run_id` identity with a descriptor-derived identity.
- Reworking mode artifacts that are unrelated to publish destination structure
  or publish metadata.

## Assumptions

- Existing run manifests already contain enough descriptive metadata to derive
  a readable publish descriptor for many packets, with a stable fallback for
  the rest.
- The current publish family roots under `specs/` and `docs/` remain the right
  external homes for the first slice.
- Reviewers benefit more from a readable path plus recoverable metadata than
  from keeping the run id as the only visible path anchor.
- The user wants the feature shipped as `0.29.0`, so version bump and
  release-surface alignment are part of the core scope rather than follow-up
  cleanup.
