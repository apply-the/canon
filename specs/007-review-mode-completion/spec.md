# Feature Specification: Review Mode Completion

**Feature Branch**: `007-review-mode-completion`  
**Created**: 2026-04-19  
**Status**: Draft  
**Input**: User description: "Complete Review Mode Completion for Canon: promote review and verification to full-depth governed workflows with durable artifacts, real run state, inspection compatibility, approval-aware behavior, and 0.7.0 documentation updates."

## Governance Context

**Mode**: brownfield  
**Risk Classification**: systemic-impact because this feature promotes two currently non-runnable review-heavy modes into truthful end-to-end product behavior across runtime, evidence, approval, inspection, documentation, and release surfaces.  
**Scope In**:
- Full-depth review mode for non-PR change packages and artifact bundles
- Full-depth verification mode for adversarial challenge of claims, invariants, contracts, and evidence
- Durable artifacts, real run state, approval-aware behavior, and inspection compatibility for both modes
- Reuse of existing pr-review findings, disposition, and evidence patterns where that reuse preserves clear semantics
- User-facing documentation, mode guidance, and roadmap alignment for the 0.7.0 release

**Scope Out**:
- Completion of implementation or refactor modes
- Delivery of incident or migration workflows
- Distribution work such as Homebrew, winget, or Scoop
- Protocol interoperability such as MCP server exposure, A2A, or other external surfaces
- Unrelated CLI polish beyond what is required for truthful review and verification delivery

**Invariants**:

- Review MUST remain distinct from diff-backed pr-review in intent, inputs, and user guidance
- Review MUST accept exactly one canonical authored input at `canon-input/review.md` or `canon-input/review/` and MUST reject arbitrary code folders as review packets
- Verification MUST remain challenge-oriented and MUST NOT become an implementation workflow
- Outputs for both modes MUST remain durable, inspectable, and approval-aware through Canon-backed surfaces
- The design MUST prefer extending existing review and evidence machinery over inventing a separate subsystem when reuse does not blur semantics
- Canon MUST never fabricate run state, evidence, approvals, findings, or packets it cannot name concretely

**Decision Traceability**: Decisions will be recorded in this specification, the follow-on planning artifacts, and the feature decision log. Validation evidence will be recorded through Canon-backed runtime artifacts, automated validation results, and release documentation review artifacts.

## User Scenarios & Testing

### User Story 1 - Review Non-PR Package (Priority: P1)

A reviewer wants Canon to assess a non-PR change package or artifact bundle and produce a durable review packet with explicit disposition, missing evidence, and decision impact.

**Why this priority**: This is the primary missing review workflow on the roadmap and the most direct extension of the already-delivered pr-review capability.

**Independent Test**: A user can run review mode against a bounded package outside PR semantics and receive a real run, a durable packet, and a clear outcome or gate state without relying on support-state messaging.

**Acceptance Scenarios**:

1. **Given** a bounded change package or artifact bundle authored under `canon-input/review.md` or `canon-input/review/` outside pull request semantics, **When** the user runs review mode, **Then** Canon creates a real run and emits a durable review packet with boundary assessment, missing evidence, decision impact, and disposition.
2. **Given** a review input that lacks supporting evidence for some conclusions, **When** review mode completes, **Then** Canon records the missing evidence explicitly instead of implying unsupported certainty.
3. **Given** a review result that requires explicit acceptance of remaining risk, **When** the user inspects the run, **Then** the disposition and any required approval are surfaced through the existing Canon-backed follow-up surfaces.

---

### User Story 2 - Challenge Claims And Invariants (Priority: P2)

An auditor or reviewer wants Canon to challenge claims, invariants, contracts, or evidence directly and receive an adversarial verification packet with unresolved findings.

**Why this priority**: Verification is the second unfinished review-heavy mode and raises trust in Canon by making challenge and contradiction first-class product behavior.

**Independent Test**: A user can run verification mode against a bounded claim or evidence bundle and receive a real verification packet with explicit challenges, supported claims, and unresolved findings.

**Acceptance Scenarios**:

1. **Given** a bounded set of claims, invariants, or evidence, **When** the user runs verification mode, **Then** Canon emits a real verification packet including an invariants checklist, contract matrix, adversarial review, verification report, and unresolved findings.
2. **Given** a claim that is not supported by the available evidence, **When** verification completes, **Then** Canon records the mismatch explicitly and preserves it as an inspectable unresolved finding.
3. **Given** a verification result with unresolved concerns, **When** the user follows up through Canon inspection surfaces, **Then** the outstanding issues remain visible and tied to the same run context.

---

### User Story 3 - Inspect And Continue Through Existing Surfaces (Priority: P3)

A user wants review and verification runs to behave like first-class Canon runs that work cleanly with the existing status, inspection, approval, and resume flows.

**Why this priority**: The feature is incomplete if the modes run but cannot be inspected and continued through the same product surfaces as other Canon workflows.

**Independent Test**: A user can inspect, approve, or resume review and verification runs using the current Canon-backed surfaces without undocumented mode-specific detours.

**Acceptance Scenarios**:

1. **Given** a completed or gated review run, **When** the user uses the existing status and inspection surfaces, **Then** Canon returns coherent summaries and real artifact references without requiring a special-case workflow.
2. **Given** a completed or gated verification run, **When** the user uses the existing status and inspection surfaces, **Then** Canon returns coherent summaries and real artifact references without requiring a special-case workflow.
3. **Given** a run that needs explicit acceptance or continuation, **When** the user follows the Canon-backed next step, **Then** the approval or resume action preserves the same run context.

### Edge Cases

- What happens when a review package requests disposition but omits critical supporting evidence?
- How does the system handle verification inputs that combine conflicting claims or invariants from multiple sources?
- Which invariant is most likely to be stressed when a user provides a non-PR package that resembles a diff review and risks collapsing into pr-review semantics?
- What happens when a run completes structurally but still carries unresolved findings that require explicit downstream acceptance?
- How does the system handle documentation or roadmap surfaces that still describe review and verification as unfinished after runtime support ships?

## Requirements

### Functional Requirements

- **FR-001**: System MUST allow users to start review mode for a non-PR change package or artifact bundle and receive a real Canon-backed run.
- **FR-002**: Review mode MUST emit a durable packet that records boundary assessment, missing evidence, decision impact, and final disposition.
- **FR-003**: Review mode MUST preserve a clear distinction from pr-review in semantics, expected inputs, and user guidance, including a single canonical authored review packet under `canon-input/review.md` or `canon-input/review/`.
- **FR-004**: System MUST allow users to start verification mode against claims, invariants, contracts, or evidence bundles and receive a real Canon-backed run.
- **FR-005**: Verification mode MUST emit a durable packet that records adversarial challenge results, supported claims, and unresolved findings.
- **FR-006**: Review and verification MUST reuse existing review-heavy findings, disposition, and evidence patterns where that reuse does not collapse their distinct semantics.
- **FR-007**: Both modes MUST preserve durable evidence and artifacts that remain inspectable through the existing status, inspect artifacts, inspect evidence, inspect invocations, approve, and resume surfaces.
- **FR-008**: System MUST never fabricate run state, evidence, approvals, findings, or packets when inputs are missing, contradictory, or unsupported.
- **FR-009**: When evidence is missing or contradictory, the emitted outputs MUST make that gap explicit rather than implying completeness.
- **FR-010**: Any approval or disposition flow required by review or verification MUST be surfaced through the same Canon-backed run context rather than a special-case side path.
- **FR-011**: User-facing product guidance MUST explain when to use review, verification, and pr-review and MUST reflect their true support state in the 0.7.0 release.
- **FR-012**: Roadmap and mode guidance materials MUST stop describing review and verification as unfinished once this feature ships.
- **FR-013**: The minimum delivered slice MUST make both modes genuinely runnable end to end rather than only modeled, partially wired, or documented.
- **FR-014**: Review and verification MUST preserve separation between generation and validation so they remain challenge-oriented workflows rather than implementation workflows.
- **FR-015**: Result summaries for review and verification MUST allow users to understand whether a run completed, is gated, or is blocked without relying on undocumented follow-up behavior.
- **FR-016**: Evidence lineage for review and verification MUST remain compatible with the current Canon inspection model and downstream approval decisions.

### Key Entities

- **Review Request**: A governed request to assess a non-PR change package or artifact bundle.
- **Verification Request**: A governed request to challenge claims, invariants, contracts, or evidence directly.
- **Review Packet**: The durable artifact set that records review findings, boundary assessment, missing evidence, decision impact, and disposition.
- **Verification Packet**: The durable artifact set that records invariant checks, contract challenges, adversarial findings, verification outcome, and unresolved findings.
- **Evidence Bundle**: The Canon-backed record linking inputs, invocations, validation evidence, and emitted artifacts for a run.
- **Review Disposition**: The recorded outcome or gate state that determines whether a reviewed package is acceptable, blocked, or accepted with explicit risk.
- **Unresolved Finding**: A challenge result that remains open and must be inspected or explicitly accepted before downstream use.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Users can complete at least one end-to-end review workflow and one end-to-end verification workflow and receive real Canon-backed artifacts for both.
- **SC-002**: For completed or gated review and verification runs, the initial Canon result or status summary makes the outcome understandable without requiring undocumented, mode-specific follow-up steps.
- **SC-003**: All artifact and evidence references surfaced for review and verification resolve to real Canon-backed paths and records.
- **SC-004**: The 0.7.0 product guidance distinguishes review, verification, and pr-review across representative scenarios and no longer describes review or verification as support-state-only surfaces.
- **SC-005**: The first delivered slice preserves inspection, approval, and resume continuity so users can continue the same run context without mode-specific hacks.
- **SC-006**: The shipped feature never claims evidence or a runnable result that it cannot identify concretely through Canon artifact or evidence surfaces.

## Validation Plan

- **Structural validation**: Validate mode contracts, manifests, CLI and help surfaces, skill contracts, documentation surfaces, and roadmap language for internal consistency.
- **Logical validation**: Execute end-to-end contract or integration scenarios for review mode, verification mode, and cross-surface inspection, approval, and resume behavior.
- **Independent validation**: Perform a separate adversarial review of the emitted packets and a separate review of the updated documentation and mode guidance before release.
- **Evidence artifacts**: Preserve Canon-backed runtime artifacts under `.canon`, automated validation results in repository test evidence, and release documentation review artifacts that show guidance and roadmap alignment.

## Decision Log

- **D-001**: Review and verification will be delivered as one roadmap feature, **Rationale**: both are review-heavy, evidence-heavy, and should reuse the pr-review substrate where that reuse preserves clear semantics.
- **D-002**: The feature is classified as systemic-impact, **Rationale**: it changes product truthfulness across runtime, evidence, approval, inspection, documentation, and roadmap surfaces.
- **D-003**: The first slice must make both modes runnable end to end, **Rationale**: modeled-only or intentionally-limited behavior is already present and does not satisfy the roadmap goal.
- **D-004**: The 0.7.0 release includes documentation and roadmap alignment, **Rationale**: the product cannot ship truthful runnable support while leaving public guidance in a pre-delivery state.

## Non-Goals

- Completing implementation or refactor modes
- Delivering incident or migration workflows
- Adding distribution channels beyond the current release flow
- Expanding into protocol interoperability or remote server exposure
- Reworking unrelated CLI UX beyond what is necessary for truthful review and verification delivery
- Collapsing review into pr-review or turning verification into an implementation helper

## Assumptions

- This feature targets the 0.7.0 release.
- Existing pr-review findings, disposition, and evidence patterns are reusable enough to support the first slice without inventing a parallel subsystem.
- Existing inspection and approval surfaces remain the canonical follow-up path for review-heavy runs.
- The first release can define bounded, canonical input shapes for review and verification without solving every future workflow variant.
- User-facing documentation, mode guidance, and roadmap artifacts must align with shipped runtime truth as part of this feature.

