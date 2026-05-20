# Feature Specification: pr-review Optional Inline Anchors

**Feature Branch**: `060-pr-review-anchors`  
**Created**: 2026-05-19  
**Status**: Draft  
**Input**: User description: "lavora a enhancement di pr-review con scope esplicito per i Conventional Comments". Repository context shows explicit `pr`/`file`/`surface` scope is already delivered, so this feature captures the remaining follow-on: optional line or span anchors when durable review evidence exists.

## Governance Context

**Mode**: change  
**Risk Classification**: bounded-impact. This feature extends the delivered `pr-review` artifact contract with optional inline precision for findings that already have explicit scope. It changes review packet and published artifact expressiveness, but it does not alter Canon run control flow, approval gates, or non-review modes.  
**Scope In**:

- Add an optional anchor model for `pr-review` Conventional Comments that can point to one changed line or one contiguous changed span within a single changed surface.
- Preserve the existing explicit scope model so every comment still communicates whether it applies at `pr`, `file`, or `surface` level.
- Render anchors in published review artifacts in a host-agnostic, human-readable form.
- Degrade honestly to scope-only comments when durable inline precision is unavailable, ambiguous, stale, or broader than one concrete changed surface.
- Update reviewer-facing guidance, repository docs, and companion wiki examples so operators understand anchored versus unanchored output.
- Align the Canon workspace version and release-note surfaces required to publish the slice.

**Scope Out**:

- Any reimplementation of the already-delivered explicit `pr`/`file`/`surface` scope model.
- Host-specific exports such as GitHub review threads, GitLab merge-request notes, deep links, or platform URL generation.
- Changes to approval semantics, must-fix disposition rules, readiness gating, or the primary-artifact role of `review-summary.md`.
- Retroactive anchor generation for historical review packets that lack persisted coordinate evidence.
- Changes to non-PR `review` mode or to review finding categories unrelated to anchor precision.

**Invariants**:

- Every Conventional Comment entry MUST continue to carry exactly one explicit scope annotation, whether or not an inline anchor is present.
- The system MUST NOT fabricate line numbers, span bounds, file paths, or inline positions that are not supported by persisted review evidence.
- `review-summary.md` MUST remain the primary artifact and canonical status surface for `pr-review`.
- Approval and readiness outcomes for equivalent findings MUST remain unchanged by the presence or absence of optional anchors.

**Decision Traceability**: `specs/060-pr-review-anchors/` for authored decisions; implementing validation evidence linked from the corresponding Canon run under `.canon/`.

## User Scenarios & Testing

### User Story 1 - Reviewer Sees Precise Anchors When Evidence Exists (Priority: P1)

A reviewer opens the Conventional Comments artifact from a `pr-review` run and can immediately see not only the explicit scope of each finding but also the exact changed line or contiguous span for findings whose review evidence supports that precision.

**Why this priority**: This is the incremental value beyond the already-delivered scope model. Without optional anchors, reviewers still have to search within a file or surface even when Canon already knows the precise changed region.

**Independent Test**: Run `pr-review` against a bounded diff fixture that yields one finding with single-line evidence and one finding with contiguous-span evidence. Verify that the rendered Conventional Comments artifact shows visible anchors for both findings and still includes explicit scope.

**Acceptance Scenarios**:

1. **Given** a finding with durable evidence for one changed line in one changed surface, **When** the Conventional Comments artifact is emitted, **Then** the entry shows the explicit scope and a visible line anchor for that surface.
2. **Given** a finding with durable evidence for a contiguous changed span in one changed surface, **When** the artifact is emitted, **Then** the entry shows the explicit scope and a visible span anchor with clear start and end bounds.
3. **Given** multiple findings in the same packet, **When** the artifact is emitted, **Then** each finding independently includes an anchor only when its own evidence supports that precision.

---

### User Story 2 - Reviewer Gets Honest Degradation When Precision Is Missing (Priority: P2)

A reviewer still receives a trustworthy Conventional Comments artifact when Canon cannot support inline precision, because the system falls back to the existing explicit scope model instead of inventing positions.

**Why this priority**: Honest degradation protects trust in the review artifact. A broader but accurate comment is better than a precise-looking comment that points to the wrong location.

**Independent Test**: Run `pr-review` against fixtures where evidence is absent, ambiguous across multiple changed surfaces, or stale relative to the persisted diff. Verify that the emitted comments keep explicit scope but omit inline anchors.

**Acceptance Scenarios**:

1. **Given** a finding with no durable inline coordinate evidence, **When** the artifact is emitted, **Then** the entry keeps its explicit scope and omits any line or span anchor.
2. **Given** a finding whose evidence spans multiple changed surfaces without one durable inline target, **When** the artifact is emitted, **Then** the entry degrades to `file` or `surface` scope and does not fabricate a single inline anchor.
3. **Given** a finding whose stored coordinates no longer map cleanly to the persisted diff context, **When** the artifact is emitted, **Then** the entry omits the anchor and remains readable as a scope-only comment.

---

### User Story 3 - Published Packet Stays Portable Outside the Code Host (Priority: P3)

A downstream reader opens the published packet without access to any live code-host UI and still understands where anchored comments apply and what scope each comment covers.

**Why this priority**: Canon artifacts are meant to travel as durable records. Anchors only add value if they remain useful outside the original execution environment.

**Independent Test**: Publish a completed `pr-review` packet containing anchored and degraded comments, then have a second reader inspect the packet without runtime manifests or code-host tooling. Verify that the reader can identify the target surface and anchor bounds from the artifact text alone.

**Acceptance Scenarios**:

1. **Given** a published packet with anchored comments, **When** a downstream reader opens the Conventional Comments artifact, **Then** the anchor text is understandable without any host-specific syntax or external navigation.
2. **Given** a published packet with both anchored and unanchored comments, **When** the reader reviews the artifact, **Then** the distinction between precise inline anchors and broader scope-only comments is obvious from the text.

### Edge Cases

- A finding references a deleted, moved, or renamed surface whose stored coordinates can no longer be expressed durably: the artifact must fall back to scope-only output.
- A finding covers multiple disjoint changed regions in the same surface: the system must not collapse them into one fabricated contiguous span.
- A span resolves to a single changed line: the artifact should present it as a line anchor rather than an artificial multi-line span.
- A packet contains no changed surfaces at all: all findings must remain valid `pr`-scoped comments without inline anchors.
- A packet mixes anchored and unanchored findings for the same surface: each finding must still be rendered according to its own evidence quality.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST preserve explicit `pr`, `file`, or `surface` scope for every Conventional Comment entry, regardless of whether an inline anchor is present.
- **FR-002**: The system MUST support an optional anchor for a Conventional Comment entry that references either one changed line or one contiguous changed span within one changed surface.
- **FR-003**: The system MUST emit an inline anchor only when persisted review evidence identifies one concrete changed surface and one durable line or contiguous span for that finding.
- **FR-004**: When persisted evidence identifies a single changed line, the rendered comment MUST present a line anchor.
- **FR-005**: When persisted evidence identifies a contiguous changed range, the rendered comment MUST present a span anchor with clear start and end bounds.
- **FR-006**: When evidence is missing, ambiguous, stale, cross-surface, or otherwise insufficient for durable inline precision, the rendered comment MUST omit the anchor and degrade honestly to scope-only output.
- **FR-007**: The system MUST NOT fabricate line numbers, span bounds, file paths, inline positions, or host-specific placement metadata absent from persisted review evidence.
- **FR-008**: Published review artifacts MUST render optional anchors in a human-readable, host-agnostic form that remains understandable without live code-host context.
- **FR-009**: Adding optional anchors MUST NOT change must-fix disposition behavior, approval semantics, readiness outcomes, or the primary-artifact role of `review-summary.md`.
- **FR-010**: Historical or imported review packets that lack persisted anchor evidence MUST remain valid and continue to render without inline anchors.
- **FR-011**: Reviewer-facing guidance, repository docs, and companion wiki examples for `pr-review` MUST explain when Conventional Comments include anchors and when they degrade to scope-only output.
- **FR-012**: Validation evidence for this feature MUST cover anchored-line, anchored-span, no-anchor, ambiguous-anchor, and stale-anchor scenarios.
- **FR-013**: Release-facing version and changelog surfaces for the Canon workspace MUST be updated before this slice is published.

### Key Entities

- **Review Anchor**: A durable reference from a review finding to one changed surface and either one changed line or one contiguous changed span.
- **ConventionalCommentScope**: The existing explicit `pr`, `file`, or `surface` indicator that remains mandatory for every comment entry.
- **Conventional Comment Entry**: The reviewer-facing artifact unit that combines comment kind, title, rationale, explicit scope, optional anchor, and affected surfaces.
- **Review Finding**: The persisted review outcome whose evidence may or may not justify an optional inline anchor.

## Success Criteria

### Measurable Outcomes

- **SC-001**: 100% of Conventional Comment entries backed by durable inline evidence display a visible line or span anchor in rendered review artifacts.
- **SC-002**: 0% of Conventional Comment entries without durable inline evidence display fabricated inline anchors.
- **SC-003**: 100% of Conventional Comment entries continue to display explicit `pr`, `file`, or `surface` scope whether anchored or unanchored.
- **SC-004**: In independent validation using representative published packets, reviewers can identify the referenced surface and anchor bounds for anchored comments within 30 seconds per comment without consulting live code-host tooling.
- **SC-005**: Approval outcomes and primary-artifact designation remain unchanged for equivalent review findings before and after this feature.
- **SC-006**: In independent validation using a fixed sample set of at least 10 mixed anchored and scope-only comments, reviewers correctly distinguish anchored comments from broader scope-only comments in 100% of sampled cases.

## Validation Plan

- **Structural validation**: Review packet and published artifact contract checks confirm optional anchors remain well-formed, explicit scope remains present, and unchanged readiness surfaces stay intact.
- **Logical validation**: Scenario-based validation covers line anchors, span anchors, mixed anchored and unanchored packets, ambiguous multi-surface findings, stale coordinates, and no-surface fallback behavior.
- **Independent validation**: A second-reader packet review confirms the artifact remains understandable outside Canon runtime and that broader comments do not imply false precision.
- **Evidence artifacts**: Authored decisions in `specs/060-pr-review-anchors/`, governed validation evidence under `.canon/`, representative packet samples, coverage output, and release/documentation diffs captured during feature validation.

## Decision Log

- **D-001**: Treat optional inline anchors as additive to explicit scope rather than a replacement, **Rationale**: the current scope model is already delivered and remains the reliable fallback when inline precision is unavailable.
- **D-002**: Only emit an anchor when one finding can be tied to one concrete changed surface and one durable coordinate interval, **Rationale**: precise-looking output must never outrun the evidence Canon actually persists.
- **D-003**: Keep anchor rendering host-agnostic, **Rationale**: published packets must remain readable and portable outside any one code-host environment.

## Non-Goals

- Re-specifying or re-delivering the existing explicit `pr`/`file`/`surface` scope model.
- Creating host-specific review threads, inline comments, or deep links for external platforms.
- Recomputing anchor coordinates from live repository state after the governed review evidence has been recorded.
- Changing review finding categories, non-PR `review` mode behavior, or approval-gate semantics.

## Assumptions

- The delivered explicit scope behavior from `053-pr-review-scope` remains the baseline that this feature builds on.
- Future `pr-review` evidence can persist enough coordinate detail to support anchors for at least some findings without requiring host-specific APIs.
- Published review packets continue to serve as durable records that must be understandable both inside and outside Canon runtime context.
- Reviewers prefer honest scope-only degradation over approximate inline placement when evidence quality is incomplete.
