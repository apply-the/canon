# Feature Specification: PR Review Conventional Comments

**Feature Branch**: `013-pr-review-comments`  
**Created**: 2026-04-24  
**Status**: Draft  
**Input**: User description: "Teach pr-review to emit Conventional Comments shaped review artifacts and review output that map findings into praise nitpick suggestion issue todo question thought and chore."

## Governance Context *(mandatory)*

**Mode**: pr-review
**Risk Classification**: bounded-impact. This feature changes the artifact shape,
reviewer-facing output, tests, and documentation for an already-delivered mode,
but it stays inside the existing `pr-review` lifecycle, evidence model,
approval gates, and publish surfaces.
**Scope In**:

- Extend `pr-review` to emit reviewer-facing feedback in a Conventional
  Comments shape.
- Preserve and, where needed, enrich the existing `review-summary.md` so the
  current disposition path remains readable.
- Add or update tests, docs, and skills that describe the delivered
  Conventional Comments behavior.

**Scope Out**:

- Adding line-level or host-specific review anchors when the current review
  packet does not contain enough evidence to support them.
- Reworking non-PR `review` mode.
- Implementing C4 outputs for `architecture`.
- Changing approval semantics, gate kinds, or the underlying diff-inspection
  evidence pipeline.

**Invariants**:

- `pr-review` MUST keep explicit review-disposition gating for unresolved
  must-fix findings.
- The feature MUST NOT fabricate line numbers, inline patch anchors, or code
  host metadata that the governed review packet does not actually contain.
- `review-summary.md` MUST remain available as the canonical packet summary and
  primary artifact for status and next-step surfaces unless a later feature
  explicitly changes that contract.
- Conventional Comments output MUST stay traceable to persisted review
  findings, changed surfaces, and the run evidence bundle.

**Decision Traceability**: Decisions for this feature will be recorded in
`specs/013-pr-review-comments/decision-log.md`, with validation evidence linked
from `specs/013-pr-review-comments/validation-report.md` and the implementing
Canon run.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reviewer Reads Conventional Comments Packet (Priority: P1)

A reviewer runs `pr-review` on a bounded diff and wants a packet that reads
like Conventional Comments instead of only generic review prose, so the output
can be reused in familiar review workflows.

**Why this priority**: This is the core product value of the feature. Without
it, `pr-review` still emits a review packet, but it does not yet match the
human review shape promised in the roadmap.

**Independent Test**: Run `canon run --mode pr-review ...` against a diff with
review findings, then verify the artifact packet includes a Conventional
Comments shaped artifact whose entries use valid comment kinds and remain tied
to the underlying findings.

**Acceptance Scenarios**:

1. **Given** a `pr-review` run with must-fix findings, **When** the packet is
   emitted, **Then** it includes a Conventional Comments shaped artifact whose
   entries classify each finding with a valid review comment kind.
2. **Given** a review finding tied to one or more changed surfaces, **When** it
   appears in the Conventional Comments artifact, **Then** the entry still
   names the affected surfaces and does not lose source traceability.
3. **Given** a note-only review packet, **When** the Conventional Comments
   artifact is emitted, **Then** the output remains useful and does not invent
   must-fix severity.

---

### User Story 2 - Approval Workflow Keeps Existing Semantics (Priority: P2)

An engineering lead uses the new reviewer-facing artifact but still needs the
existing Canon disposition workflow to behave exactly as before.

**Why this priority**: The new artifact must not weaken the current
review-disposition gates. If the feature breaks approval flow, it creates a
review-format regression instead of product value.

**Independent Test**: Run `pr-review` on a high-impact diff that currently
waits for approval, then verify that the run still blocks or awaits approval in
the same way while surfacing the new artifact alongside the existing summary.

**Acceptance Scenarios**:

1. **Given** unresolved must-fix findings, **When** the `pr-review` packet is
   emitted, **Then** the run still requires explicit disposition before final
   readiness can pass.
2. **Given** the reviewer inspects status or artifacts, **When** the run is in
   `AwaitingApproval`, **Then** the current `review-summary.md` primary-artifact
   path and next-step flow remain intact.

---

### User Story 3 - Published Packet Is Readable Outside Canon (Priority: P3)

A downstream reader opens the published review packet outside the runtime and
wants the Conventional Comments artifact to be understandable without hidden
state or host-specific assumptions.

**Why this priority**: The feature only closes if the artifact remains useful
after publication, not only inside the CLI output.

**Independent Test**: Publish a completed `pr-review` run and verify the
published packet includes a readable Conventional Comments artifact that keeps
comment kinds, rationale, and affected surfaces visible.

**Acceptance Scenarios**:

1. **Given** a published `pr-review` packet, **When** a reader opens the new
   artifact, **Then** they can understand the review comments without the
   status command or internal run manifests.
2. **Given** the current review packet lacks line-level anchors, **When** the
   Conventional Comments artifact is published, **Then** it stays file/surface
   scoped and does not pretend to be an inline code-host export.

### Edge Cases

- A review packet contains no must-fix findings and only note-level review
  observations.
- A diff produces no changed surfaces and the artifact must remain readable
  without pretending file-level precision.
- Multiple review findings map to the same Conventional Comments kind.
- A changed surface appears in more than one comment entry and must remain
  deduplicated enough to stay readable.
- The diff evidence is sufficient for surface-level comments but not for
  line-level anchors.
- The run is approval-gated and the new artifact must not imply the review is
  already accepted.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `pr-review` MUST emit a Conventional Comments shaped artifact in
  addition to the current review packet artifacts.
- **FR-002**: The Conventional Comments artifact MUST use only valid
  Conventional Comments kinds from this set: `praise`, `nitpick`, `suggestion`,
  `issue`, `todo`, `question`, `thought`, `chore`.
- **FR-003**: In the first slice, each emitted comment entry MUST remain
  traceable to exactly one persisted review finding and to the changed surfaces
  that finding refers to.
- **FR-004**: The system MUST preserve the existing `review-summary.md`
  artifact and its role in status, inspect, and next-step surfaces.
- **FR-005**: The system MUST preserve the existing approval and readiness
  semantics for unresolved must-fix findings.
- **FR-006**: The system MUST NOT fabricate line numbers, inline patch ranges,
  or code-host-specific metadata when the review packet only supports
  surface-level evidence.
- **FR-007**: When a review packet contains must-fix findings, the
  Conventional Comments artifact MUST represent them with comment kinds that do
  not understate their severity.
- **FR-008**: When a review packet contains only note-level findings, the
  Conventional Comments artifact MUST remain readable without falsely
  escalating those findings into must-fix comments.
- **FR-009**: The first-slice mapping from persisted review findings to
  Conventional Comments kinds MUST be deterministic and documented in the
  feature design artifacts.
- **FR-010**: The new artifact MUST publish through the existing `pr-review`
  packet flow and remain readable under `docs/reviews/prs/<RUN_ID>/`.
- **FR-011**: Documentation and skill surfaces that describe `pr-review` MUST
  reflect the delivered Conventional Comments behavior.

### Key Entities *(include if feature involves data)*

- **Conventional Comment Entry**: A reviewer-facing record with a valid comment
  kind, a short title, rationale/details, and the changed surfaces it applies
  to.
- **Comment Kind Mapping**: The deterministic rule that maps a persisted review
  finding into a Conventional Comments kind while preserving severity intent.
- **Review Packet**: The existing bounded packet derived from governed diff
  inspection and critique evidence, which remains the source of truth for the
  new artifact.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In `pr-review` runs with findings, 100% of emitted review packets
  include a Conventional Comments artifact with valid comment kinds only.
- **SC-002**: In approval-gated `pr-review` runs, 100% of runs preserve the
  current disposition state and continue to expose `review-summary.md` as the
  primary artifact.
- **SC-003**: In published `pr-review` packets, a reader can identify comment
  kind, rationale, and affected surfaces without opening internal runtime
  manifests.
- **SC-004**: No Conventional Comments artifact emitted by this feature
  includes fabricated line-level or code-host-specific anchors.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets --all-features -- -D warnings`, and documentation/skill
  consistency checks for touched surfaces.
- **Logical validation**: Focused contract and integration tests for the new
  `pr-review` artifact, comment kind mapping, approval-gated runs, and publish
  readability.
- **Independent validation**: Separate review of the emitted Conventional
  Comments packet to confirm it reads like reviewer output and does not hide or
  soften must-fix findings.
- **Evidence artifacts**: Validation results and review notes will be recorded
  in `specs/013-pr-review-comments/validation-report.md` and linked from the
  implementing run evidence bundle.

## Decision Log *(mandatory)*

- **D-001**: Deliver Conventional Comments as an additive `pr-review` artifact
  instead of replacing `review-summary.md`, **Rationale**: this keeps current
  status, approval, and next-step surfaces stable while adding the new
  reviewer-facing format.

## Non-Goals

- Exporting directly into GitHub, GitLab, or other host-specific inline review
  APIs.
- Adding exact line comments without governed evidence that supports them.
- Reworking the broader `review` mode or delivering the architecture C4 slice.

## Assumptions

- The current `ReviewPacket` remains the canonical source of findings for the
  Conventional Comments artifact.
- Surface-level changed-file evidence is sufficient for the first slice even
  when exact inline anchors are unavailable.
- Existing `pr-review` tests and publish surfaces will be extended rather than
  replaced.
- Conventional Comments interoperability in the first slice is artifact-shape
  compatibility, not direct code-host API submission.

