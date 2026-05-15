# Feature Specification: pr-review Explicit Comment Scope

**Feature Branch**: `053-pr-review-scope`
**Created**: 2026-05-14
**Status**: Draft
**Input**: Add explicit scope model to pr-review Conventional Comments: differentiate PR-level, file, and surface-scoped comments, with governed packet contract preservation and optional future line/span anchor support.

## Governance Context

**Mode**: change
**Risk Classification**: bounded-impact. The feature extends the delivered `pr-review` mode
by enriching the Conventional Comments artifact shape with an explicit scope field.
It touches artifact rendering, domain types, and skill documentation; it does not alter
approval gates, evidence pipelines, run state, or publish surfaces.
**Scope In**:

- Extend the comment model to carry a `ConventionalCommentScope` enum field that
  differentiates `pr`, `file`, and `surface` level.
- Update artifact rendering so every emitted Conventional Comment entry carries a durable
  scope annotation derived from the review evidence.
- Preserve complete backward compatibility with the existing governed packet contract and
  all downstream surfaces consuming `conventional-comments.md` and `review-summary.md`.
- Update skill documentation to reflect the new scope model.
- Raise unit and integration test coverage for the changed Rust code to 95% on modified files.

**Scope Out**:

- Line-level or span-level anchors (deferred to a later slice).
- Changes to approval semantics, gate kinds, or the diff-inspection evidence pipeline.
- Changes to non-PR `review` mode.
- Host-specific export formats (GitHub PR comment threads, GitLab MR notes, etc.).
- Changes to `review-summary.md` primary-artifact role or status surfaces.

**Invariants**:

- Every Conventional Comment entry MUST carry exactly one scope annotation derived
  deterministically from the persisted review evidence without fabricating absent precision.
- The system MUST NOT emit a `file` or `surface` scope annotation if no concrete changed
  surface can be traced to that finding.
- `review-summary.md` MUST remain the primary artifact and canonical status surface
  for `pr-review` throughout and after this feature.
- The existing approval and readiness semantics for unresolved must-fix findings MUST be
  preserved unchanged.
- Rust code changes MUST comply with Canon language rules: no panic-prone control flow
  outside tests/entrypoints, no magic strings or magic numbers in owned logic, stable serde
  shapes via typed models.

**Decision Traceability**: `specs/053-pr-review-scope/` directory; validation evidence
linked from the implementing Canon run.

## User Scenarios & Testing

### User Story 1 - Reviewer Sees Scoped Conventional Comments (Priority: P1)

A reviewer runs `canon run --mode pr-review` and receives a Conventional Comments artifact
where each entry explicitly states whether the comment applies to the whole PR, to a
specific file, or to a broader surface group, so the output can be prioritized and routed
without guessing where each comment lands.

**Why this priority**: This is the core product value of this feature. Without a durable
scope model, a consumer cannot distinguish a structural PR-wide concern from a
file-specific nitpick.

**Independent Test**: Run `pr-review` against a diff with findings on specific files and
findings that apply to the overall change. Verify that the emitted Conventional Comments
artifact contains entries with explicit scope annotations and that file-scoped entries name
the actual changed surface.

**Acceptance Scenarios**:

1. **Given** a `pr-review` diff with findings traceable to one or more specific changed
   files, **When** the Conventional Comments artifact is emitted, **Then** each entry
   carries a `scope` field set to `file` and names the specific changed surface(s).
2. **Given** a `pr-review` diff with structural findings that apply to the overall PR
   rather than any single file, **When** the artifact is emitted, **Then** those entries
   carry a `scope` field set to `pr`.
3. **Given** a finding for which no concrete changed surface can be traced, **When** the
   artifact is emitted, **Then** the entry scope defaults to `pr` and does not invent file
   names or surface labels.

---

### User Story 2 - Approved Packet Preserves Existing Workflow (Priority: P2)

An engineering lead reviews the scope-annotated Conventional Comments but still expects
the existing Canon approval workflow and disposition gates to behave identically to before.

**Why this priority**: The new scope field must be purely additive; it must not break the
approval gate, the status surface, or the review-summary primary artifact.

**Independent Test**: Run `pr-review` on a diff that currently triggers `AwaitingApproval`
and verify that approval state, gate results, `review-summary.md`, and next-step surfaces
are unchanged.

**Acceptance Scenarios**:

1. **Given** unresolved must-fix findings, **When** the scoped Conventional Comments
   artifact is emitted, **Then** the run still blocks on unresolved disposition and
   `review-summary.md` remains the primary artifact.
2. **Given** a completed `pr-review` run, **When** the packet is published, **Then** scope
   annotations appear in the published artifact and the packet metadata contract is not
   broken.

---

### User Story 3 - Published Packet Is Readable Without Canon Runtime (Priority: P3)

A downstream reader opens the published review packet and understands which comments apply
to which scope level without any runtime tooling.

**Why this priority**: The artifact is the system of record; its readability outside the
CLI is a delivery-completeness requirement.

**Independent Test**: Publish a completed `pr-review` run and open the
`conventional-comments.md` artifact. Verify that scope annotations are human-readable and
their meaning is self-evident from the artifact text.

**Acceptance Scenarios**:

1. **Given** a published `pr-review` packet, **When** a reader opens the Conventional
   Comments artifact, **Then** scope levels (`pr`, `file`, `surface`) are visible and
   legible without runtime manifests.
2. **Given** the current review evidence lacks line-level precision, **When** the artifact
   is published, **Then** no line-anchored scope appears and scope stays at `pr` or `file`
   level.

### Edge Cases

- A diff produces findings where every changed surface belongs to the same logical group:
  all entries should be consistently `surface`-scoped.
- A diff has no changed surfaces at all: all entries must fall back to `pr` scope without
  fabricating surfaces.
- Multiple findings map to the same set of changed surfaces: each entry carries its own
  scope independently.
- A finding has a non-empty `changed_surfaces` list but no surface matches any known
  surface-group heuristic: scope should be `file`-derived from the surface list.
- The Conventional Comments artifact is emitted for an empty diff: artifact remains valid
  and carries no phantom scope annotations.

## Requirements

### Functional Requirements

- **FR-001**: A `ConventionalCommentScope` enum MUST be defined in the engine domain with
  variants `Pr`, `File`, and `Surface`, serialized kebab-case via serde.
- **FR-002**: The `ReviewFinding` type MUST carry a `scope: ConventionalCommentScope` field
  derived deterministically during packet construction from `changed_surfaces` and
  `category`.
- **FR-003**: The artifact renderer for `conventional-comments.md` MUST emit each entry
  with the scope annotation visible in the Markdown output.
- **FR-004**: When `changed_surfaces` is empty or no file-level trace exists, scope MUST
  be `Pr`.
- **FR-005**: When `changed_surfaces` is non-empty and the finding applies to specific
  files, scope MUST be `File` and the rendered entry MUST include the surface name(s).
- **FR-006**: When `changed_surfaces` represents a recognized surface group (e.g., all test
  files, all contract files), scope MAY be promoted to `Surface` with the group label.
- **FR-007**: The system MUST NOT fabricate line numbers, diff offsets, or code-host
  anchors in any scope annotation.
- **FR-008**: `review-summary.md` primary artifact role and structure MUST be preserved
  without modification.
- **FR-009**: Existing approval and readiness semantics MUST be preserved unchanged.
- **FR-010**: Skill documentation for `canon-pr-review` MUST reflect the scope model and
  the three scope levels.
- **FR-011**: Modified and created Rust source files MUST have 95% line coverage as
  measured by `cargo llvm-cov`.
- **FR-012**: All Clippy warnings in modified files MUST be resolved (`-D warnings`).
- **FR-013**: The workspace version MUST be bumped at the start of the feature branch.

### Key Entities

- **ConventionalCommentScope**: Enum `Pr | File | Surface`, serde kebab-case; annotates
  the reach of each Conventional Comment entry.
- **ReviewFinding**: Extended with `scope: ConventionalCommentScope` derived at packet
  construction time from `changed_surfaces` and `category`.
- **ConventionalCommentEntry**: Rendered artifact unit that includes a visible scope
  annotation alongside comment kind, title, and rationale.

## Success Criteria

### Measurable Outcomes

- **SC-001**: 100% of Conventional Comment entries in `pr-review` run artifacts carry an
  explicit scope annotation.
- **SC-002**: 0% of emitted scope annotations fabricate file names or surfaces absent from
  the finding's `changed_surfaces`.
- **SC-003**: 100% of existing `pr-review` approval and gate outcomes remain identical
  before and after this feature (verified by regression tests).
- **SC-004**: `review-summary.md` structure and content remain unchanged for identical
  review evidence inputs.
- **SC-005**: 95% line coverage on all modified and created Rust source files.
- **SC-006**: `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits
  clean after the feature.

## Validation Plan

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets
  --all-features -- -D warnings`, `cargo deny check licenses advisories bans sources`.
- **Logical validation**: Unit tests for scope derivation in `findings.rs` covering all
  scope variants and edge cases; integration tests for artifact rendering verifying scope
  annotation format; regression tests confirming `review-summary.md` and gate outputs are
  unchanged.
- **Independent validation**: Coverage measurement via `cargo llvm-cov` on modified files;
  manual inspection of a sample rendered artifact confirming readable scope annotations.
- **Evidence artifacts**: Test output, clippy report, and coverage report linked from the
  implementing Canon run.

## Decision Log

- **D-001**: Introduce `ConventionalCommentScope` as a typed enum rather than a raw string
  field, **Rationale**: Aligns with Canon language rules against magic strings in stable
  serialization paths; enables exhaustive matching and prevents scope drift.
- **D-002**: Derive scope during `ReviewFinding` construction (at packet-build time) rather
  than at render time, **Rationale**: Keeps the derivation rule in a single place, makes
  scope a first-class property of the domain model, and avoids duplicating logic across
  renderers.
- **D-003**: Default to `Pr` scope when no concrete surface trace exists, **Rationale**:
  Honest degradation is preferred over fabricating precision the evidence does not support.

## Non-Goals

- Line-level or span-level anchors (a separate future slice).
- Host-specific review comment threading (GitHub, GitLab, Bitbucket).
- Changes to the `review` (non-PR) mode.
- Changes to approval gate kinds or evidence pipeline structure.
- Any UI or web-facing output beyond the existing governed Markdown packet.

## Assumptions

- The existing `changed_surfaces` field on `ReviewFinding` provides sufficient evidence to
  derive `file` and `surface` scope without additional diff-parsing.
- `cargo llvm-cov` remains available in the development environment.
- `surface` scope is emitted only when all surfaces in a finding match the same
  classification heuristic (e.g., all test files); otherwise `file` scope is used.
