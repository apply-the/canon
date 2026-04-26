# Feature Specification: Architecture ADR And Options

**Feature Branch**: `018-architecture-adr-options`  
**Created**: 2026-04-26  
**Status**: Draft  
**Input**: User description: "Deliver a narrow architecture authoring bundle that upgrades architecture mode only: preserve authored ADR-style decision sections, add explicit option-analysis sections (Decision Drivers, Options Considered, Pros, Cons, Recommendation, Why Not The Others), and keep existing C4 outputs unchanged."

## Governance Context *(mandatory)*

**Mode**: architecture  
**Risk Classification**: bounded-impact. This feature deepens one already-delivered mode without adding a new runtime domain, changing publish destinations, or altering approval semantics. The blast radius is bounded to the `architecture` authored contract, renderer behavior, skill/template/example guidance, and focused tests and documentation for that mode.  
**Scope In**:

- Extend `architecture` so authored packets can carry ADR-like decision content and explicit option-analysis sections in a stable, reviewable shape.
- Preserve those authored decision and comparison sections verbatim in emitted architecture artifacts when they are present.
- Keep the existing C4 packet (`system-context.md`, `container-view.md`, `component-view.md`) intact while improving the decision-facing architecture artifacts.
- Update the corresponding skill guidance, templates, worked examples, documentation, and focused tests for `architecture` only.
- Preserve critique-first honesty so missing authored sections still surface explicit missing-body markers instead of fabricated certainty.

**Scope Out**:

- Extending the same package to `requirements`, `change`, or any other mode in this slice.
- Introducing a new artifact family outside the existing `architecture` packet.
- Changing run identity, state transitions, approval gates, evidence layout, publish paths, or non-architecture mode behavior.
- Implementing broader ecosystem-evaluation collectors, live package-registry lookups, or new external adapters.
- Turning this slice into a general roadmap-wide rollout of all industry-standard shapes or all decision-comparison behavior.

**Invariants**:

- `architecture` MUST remain critique-first and evidence-backed; authored decision content cannot become unchallenged restatement of the brief.
- Existing C4 artifacts and their authored-section preservation rules MUST remain behaviorally intact.
- Missing required authored decision sections MUST still surface explicit missing-body honesty markers instead of fabricated prose.
- Existing run identity, state transitions, approval behavior, evidence linking, and publish destinations MUST remain unchanged.
- Non-target modes MUST remain behaviorally unchanged in this slice.

**Decision Traceability**: Decisions for this feature will be recorded in `specs/018-architecture-adr-options/decision-log.md`, with validation evidence captured in `specs/018-architecture-adr-options/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Author A Real Architecture Decision Packet (Priority: P1)

An architect wants the `architecture` mode to emit a decision packet that reads like a real ADR-backed architecture review, with explicit decision drivers, options considered, recommendation, and consequences instead of a thin summary.

**Why this priority**: This is the core user value of the slice. Without a stronger decision artifact, the package is just cosmetic wording change rather than a real improvement in how architecture decisions are captured and reviewed.

**Independent Test**: Run `architecture` on a brief that includes multiple viable options and confirm the resulting packet exposes decision drivers, explicit alternatives, the recommendation, and the rationale for rejecting the other options without needing chat history.

**Acceptance Scenarios**:

1. **Given** an `architecture` brief with multiple structural options, **When** Canon emits the packet, **Then** the decision-facing artifacts preserve authored sections for decision drivers, options considered, recommendation, and rejected alternatives.
2. **Given** an `architecture` brief whose authored decision content is already ADR-shaped, **When** Canon emits the packet, **Then** the resulting architecture decision artifact preserves that authored structure rather than flattening it into generic prose.

---

### User Story 2 - Review Tradeoffs Without Losing C4 Context (Priority: P2)

A reviewer wants to inspect architecture tradeoffs and the chosen option while still receiving the existing C4 context artifacts unchanged, so decision analysis and structural communication remain aligned in one packet.

**Why this priority**: The package is only useful if it improves decision review without regressing the already-delivered C4 slice.

**Independent Test**: Run `architecture` on a brief containing both C4 sections and explicit options, then confirm the packet includes the improved decision artifacts and the unchanged C4 artifacts in the same run.

**Acceptance Scenarios**:

1. **Given** an `architecture` brief with authored `## System Context`, `## Containers`, `## Components`, and option-analysis sections, **When** Canon emits the packet, **Then** the C4 artifacts continue to preserve the authored C4 sections while the decision artifacts preserve the option-analysis sections.
2. **Given** an `architecture` packet under review, **When** a reviewer inspects it, **Then** they can identify what won, what lost, and why, without losing visibility into the system context and component boundaries.

---

### User Story 3 - Keep Missing Context Honest And Synchronized (Priority: P3)

A maintainer wants the architecture skill guidance, template, worked example, renderer, and tests to agree on the new authored decision contract so the mode stays honest when authors omit required sections.

**Why this priority**: This protects the quality bar of the first two stories. Without synchronized authoring guidance and missing-body behavior, the stronger artifact shape will drift or silently degrade.

**Independent Test**: Remove one required authored decision section from a focused architecture fixture and confirm the emitted artifact names the missing section explicitly while the docs and skill guidance still describe the same contract.

**Acceptance Scenarios**:

1. **Given** an `architecture` brief missing one required option-analysis section, **When** Canon emits the packet, **Then** the affected artifact contains an explicit missing-body marker naming the absent section.
2. **Given** the `architecture` template, worked example, and skill guidance, **When** a maintainer compares them, **Then** they describe the same authored decision sections and review expectations.

### Edge Cases

- The architecture decision is already materially closed, so the packet must say that only one viable option remains instead of inventing fake alternatives.
- The authored brief includes options and recommendation text but omits the reasoning for rejecting alternatives.
- The authored ADR-style content and the C4 sections disagree about the chosen system boundary.
- The authored brief uses non-canonical wording for decision sections, stressing the missing-body fallback rather than verbatim preservation.
- The architecture brief contains rich C4 context but almost no decision analysis, so the packet must improve honesty rather than pretend the comparison work already exists.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `architecture` MUST support authored decision content that expresses the architecture decision in an ADR-like shape, including context, decision, status, and consequences.
- **FR-002**: `architecture` MUST support authored option-analysis sections for decision drivers, options considered, recommendation, and explicit rejection rationale for alternatives.
- **FR-003**: The emitted architecture decision-facing artifacts MUST preserve authored ADR-like and option-analysis sections verbatim when those sections use the canonical authored contract.
- **FR-004**: When one or more required authored decision sections are absent, the affected emitted artifact MUST surface an explicit missing-body marker naming the missing section instead of fabricating content.
- **FR-005**: Existing `system-context.md`, `container-view.md`, and `component-view.md` behavior MUST remain unchanged by this slice.
- **FR-006**: The improved decision-facing architecture packet MUST allow a reviewer to identify the winning option, the rejected options, and the decision rationale from the emitted artifacts alone.
- **FR-007**: The `architecture` skill guidance, template, and worked example MUST describe the same authored decision and option-analysis contract.
- **FR-008**: The architecture documentation MUST explain this package as a bounded first slice of the broader roadmap work rather than implying the full roadmap item is complete.
- **FR-009**: Focused contract, renderer, run, and docs tests MUST exist for the new authored decision sections and their missing-body behavior.
- **FR-010**: Non-target modes and non-decision architecture artifacts MUST retain their current behavior unchanged.

### Key Entities *(include if feature involves data)*

- **Architecture Decision Record Shape**: The authored decision structure that captures context, selected decision, status, and consequences in a stable reviewable form.
- **Option Analysis Section**: An authored comparison section that identifies drivers, alternatives, recommendation, and rejection rationale.
- **Decision-Facing Artifact**: The subset of the architecture packet that carries decision, tradeoff, and readiness reasoning for reviewers.
- **Missing-Body Marker**: The explicit honesty block naming an authored decision section that was required but absent.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The `architecture` mode has at least one focused positive test proving authored ADR-like and option-analysis sections appear in emitted decision artifacts when present.
- **SC-002**: The `architecture` mode has at least one focused negative test proving a missing required decision section surfaces an explicit missing-body marker.
- **SC-003**: The architecture template, worked example, and skill guidance all describe the same authored decision-section contract with no unresolved drift.
- **SC-004**: A reviewer can inspect one emitted architecture packet and identify the decision drivers, viable options, recommendation, and rejection rationale without consulting chat history.
- **SC-005**: The targeted validation suite passes without changing C4 artifact behavior, approval semantics, publish destinations, or non-target mode behavior.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: Focused contract, renderer, run, and docs tests for `architecture`, plus targeted non-regression coverage for existing C4 artifact behavior.
- **Independent validation**: Review of `spec.md`, `plan.md`, and `tasks.md` before implementation, followed by one realistic authored architecture packet walkthrough and one negative fixture with a required decision section removed.
- **Evidence artifacts**: Validation results and findings recorded in `specs/018-architecture-adr-options/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Keep this slice architecture-only, **Rationale**: it combines three roadmap threads in one place without reopening multiple modes at once.
- **D-002**: Preserve existing C4 artifacts unchanged while improving decision-facing artifacts, **Rationale**: this avoids regressing already-delivered architecture communication surfaces.
- **D-003**: Treat missing authored decision content as an honesty problem, not a generation problem, **Rationale**: Canon should expose weak decision input rather than inventing completeness.

## Non-Goals

- Extending the same authored-decision package to `requirements`, `change`, `implementation`, or any other mode in this slice.
- Introducing live framework or ecosystem evidence collection.
- Changing runtime risk gating, publish behavior, run state transitions, or evidence storage.
- Replacing the current C4 artifact family with a new modeling system.

## Assumptions

- The existing `architecture` artifact family has enough structure to absorb a stronger authored decision contract without introducing a new runtime model.
- Reviewers value explicit rejected alternatives and consequences in the architecture packet more than generic decision summaries.
- The existing authored-section preservation pattern used in recent slices can be reused for architecture decision sections without changing Canon's governance model.
- The first implementation slice can stay bounded to repository docs, templates, skill guidance, renderer logic, and focused tests for `architecture`.
