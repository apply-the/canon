# Feature Specification: Industry-Standard Artifact Shapes Follow-On

**Feature Branch**: `030-artifact-shapes-follow-on`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Implement the next slice of industry-standard artifact shapes by extending high-leverage remaining modes with explicit authored personas and reviewer-native packet shapes. Target discovery, system-shaping, and review first. Include a task for the version bump, a task for impacted docs and changelog updates, coverage for modified or new Rust files, cargo clippy cleanup, and cargo fmt execution."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because this slice changes existing
skills, renderers, tests, docs, and release-facing surfaces across already
modeled Canon workflows without introducing a new runtime mode or changing
approval, evidence, or publish semantics  
**Scope In**:

- first follow-on artifact-shape and persona support for `discovery`,
  `system-shaping`, and `review`
- skill guidance, template/example alignment, and renderer preservation for the
  new shaped packet surfaces
- focused validation, release-surface updates, and `0.30.0` version alignment
- explicit tasking for version bump, impacted docs plus changelog updates,
  coverage for modified or new Rust files, `cargo clippy`, and `cargo fmt`

**Scope Out**:

- new Canon modes or changes to run lifecycle semantics
- protocol or distribution-channel work
- widening persona rollout to every remaining mode in one pass
- approval, evidence, trace, or publish-contract changes unrelated to shaped
  packet authoring

**Invariants**:

- Persona guidance MUST remain subordinate to canonical artifact contracts,
  missing-authored-body behavior, approval posture, and evidence requirements.
- Modes outside `discovery`, `system-shaping`, and `review` MUST keep their
  current observable behavior unless a later scoped change expands coverage.
- This slice MUST NOT change `.canon/` persistence, canonical `run_id`
  identity, or the structured external publish contract delivered in 029.

**Decision Traceability**: Decisions start in this specification and continue
in `specs/030-artifact-shapes-follow-on/decision-log.md`, with validation
evidence recorded in
`specs/030-artifact-shapes-follow-on/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Shape Discovery For Exploratory Work (Priority: P1)

As a maintainer using Canon to frame an ambiguous problem, I want `discovery`
to produce an exploratory packet shaped like an Opportunity Solution Tree seed
plus Jobs-To-Be-Done style brief so the output reads like credible product and
research framing instead of a generic Canon summary.

**Why this priority**: Discovery is the earliest high-leverage entry point for
uncertain work. If this slice lands well, downstream requirements and shaping
packets start from stronger artifacts instead of repackaged chat output.

**Independent Test**: With one representative discovery brief, the generated
packet can be reviewed on its own as an exploratory packet with explicit
outcome, opportunities, solution directions, assumption tests, and bounded
unknowns without additional prompt context.

**Acceptance Scenarios**:

1. **Given** an authored discovery brief with the canonical discovery headings,
   **When** the assistant follows the updated `discovery` skill, **Then** the
   emitted packet reads as an exploratory OST and JTBD-flavored artifact while
   preserving Canon's bounded unknowns, assumptions, and downstream handoff.
2. **Given** a discovery brief that omits a required authored section,
   **When** the packet is rendered, **Then** Canon still emits the explicit
   missing-body signal instead of improvising the absent section with
   persona-shaped prose.

---

### User Story 2 - Shape System-Shaping For Domain And Structure Work (Priority: P2)

As a maintainer using Canon to define a new capability boundary, I want
`system-shaping` to produce a packet shaped like a domain-map and structural
option brief from a system-design perspective so the output reads like a real
bounded design exploration rather than a flat packet dump.

**Why this priority**: `system-shaping` sits at the center of new-capability
work. Giving it an explicit domain-map plus structural-options shape improves
the quality of one of Canon's most strategic packets without adding a new mode.

**Independent Test**: With one representative shaping brief, the emitted packet
can be reviewed as a domain-and-structure artifact containing bounded contexts,
ubiquitous language, domain responsibilities, candidate structures, and
selected boundaries grounded in authored input.

**Acceptance Scenarios**:

1. **Given** an authored system-shaping brief with the canonical headings,
   **When** the assistant follows the updated `system-shaping` skill, **Then**
   the packet reads like a system-design artifact with explicit domain-map and
   structural-option sections while preserving Canon's critique-first posture.
2. **Given** a shaping brief whose authored sections are structurally weak,
   **When** the packet is rendered, **Then** Canon still records the weakness
   honestly with missing-body or blocked-packet behavior instead of treating
   the persona as permission to invent a stronger design.

---

### User Story 3 - Shape Review For Reviewer-Native Findings (Priority: P3)

As a maintainer reviewing a bounded packet, I want `review` to produce a
findings-first artifact shaped for reviewer workflows so severity, location,
rationale, and recommended change are easier to consume without losing Canon's
disposition and evidence discipline.

**Why this priority**: `review` is the natural follow-on after discovery and
shaping improvements because it turns the generated packet into something teams
can act on directly during review and acceptance.

**Independent Test**: With one representative review packet, the emitted output
can be judged as a reviewer-native findings bundle that still preserves gate
targets, disposition state, evidence gaps, and Canon's honesty guarantees.

**Acceptance Scenarios**:

1. **Given** an authored review packet with the canonical review headings,
   **When** the assistant follows the updated `review` skill, **Then** the
   packet reads like a reviewer findings bundle with severity, location,
   rationale, and recommended change framing.
2. **Given** a review packet that still has unresolved evidence gaps,
   **When** the shaped packet is emitted, **Then** Canon still preserves the
   existing disposition gate and missing-evidence posture rather than letting
   the reviewer persona imply closure.

---

### User Story 4 - Ship 0.30.0 With Aligned Docs And Validation (Priority: P4)

As a maintainer shipping this slice, I want version bump, impacted docs,
changelog, coverage, `cargo clippy`, and `cargo fmt` explicitly tracked in the
feature workflow so the delivered authoring contract is trustworthy outside the
code diff.

**Why this priority**: This repository treats release-facing documentation and
validation evidence as part of the shipped contract, not optional cleanup.

**Independent Test**: A maintainer can inspect the completed task graph and
validation artifacts and confirm explicit work exists for the `0.30.0` version
surfaces, impacted docs plus changelog updates, touched-Rust-file coverage,
`cargo clippy`, and `cargo fmt`.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects the
   release surfaces, **Then** `Cargo.toml`, `Cargo.lock`, shared runtime
   compatibility references, docs, and changelog consistently report `0.30.0`.
2. **Given** the generated task plan, **When** a maintainer reviews it,
   **Then** it includes an explicit version-bump task, an explicit impacted
   docs plus changelog task, a touched-Rust-file coverage task, and explicit
   `cargo clippy` plus `cargo fmt` tasks.

### Edge Cases

- What happens when a persona-native packet shape wants headings or framing
  that partially overlap but do not exactly match Canon's current authored H2
  contract?
- How does the system handle packets that are partially shaped but still miss a
  canonical authored section required for honest downstream gating?
- Which invariant is most likely to be stressed when targeted modes share
  renderer helpers or docs phrasing with non-targeted modes?
- How are deferred shapes for `implementation`, `refactor`, and `verification`
  documented so maintainers do not assume this slice completed the whole
  roadmap item?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST define an explicit authored persona for each
  in-scope follow-on mode: `discovery`, `system-shaping`, and `review`.
- **FR-002**: The system MUST define an explicit industry-standard packet shape
  for each in-scope follow-on mode.
- **FR-003**: The system MUST map `discovery` to an Opportunity Solution Tree
  seed plus Jobs-To-Be-Done style exploratory packet while preserving Canon's
  canonical discovery contract.
- **FR-004**: The system MUST map `system-shaping` to a domain-map plus
  structural-options packet while preserving Canon's bounded-context,
  invariants, and selected-boundaries surfaces.
- **FR-005**: The system MUST map `review` to a reviewer-native findings packet
  with severity, location, rationale, and recommended change framing while
  preserving Canon's review disposition and evidence semantics.
- **FR-006**: Persona guidance MUST remain guidance-only and MUST NOT override
  canonical artifact contracts, approval posture, evidence requirements, or
  explicit missing-authored-body behavior.
- **FR-007**: For every in-scope follow-on mode, the renderer MUST preserve the
  declared shaped authored sections when they are present and MUST emit the
  existing explicit gap signals when required authored sections are absent.
- **FR-008**: The system MUST provide focused validation evidence for positive
  and negative paths across `discovery`, `system-shaping`, and `review`.
- **FR-009**: Operator-facing docs, templates, examples, roadmap text, and
  skill guidance MUST make the in-scope persona and shape mappings discoverable
  and must document which roadmap follow-ons remain deferred.
- **FR-010**: Modes outside `discovery`, `system-shaping`, and `review` MUST
  remain behaviorally unchanged unless explicitly pulled into scope by a later
  feature.
- **FR-011**: Cargo manifests, lockfile surfaces, runtime compatibility
  references, and release-facing docs MUST align to `0.30.0` for this slice.
- **FR-012**: The generated task plan MUST include an explicit version-bump
  task, an explicit impacted-docs-and-changelog task, a coverage task for
  modified or new Rust files, a `cargo clippy` task, and a `cargo fmt` task.
- **FR-013**: Modified or newly created Rust files in this slice MUST receive
  focused automated validation coverage before the feature is complete.
- **FR-014**: This slice MUST NOT change run identity generation, `.canon/`
  storage layout, or the structured publish destination contract delivered in
  029.

### Key Entities *(include if feature involves data)*

- **Mode Persona Profile**: the authored counterpart for a mode, including
  intended audience, critique posture, and explicit limits on implied
  authority.
- **Artifact Shape Contract**: the packet shape a mode must follow, including
  required sections, canonical authored headings, and preservation behavior.
- **Generated Review Surface**: the bundle of emitted artifacts a maintainer or
  reviewer uses to judge whether the packet matches both the declared shape and
  Canon's governance rules.
- **Validation Evidence Record**: the durable artifact set that captures
  positive-path, negative-path, and non-regression checks for this follow-on
  slice.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For each in-scope mode, at least one representative authored
  brief can be turned into a packet that an independent reviewer judges as both
  contract-compliant and audience-appropriate without additional chat context.
- **SC-002**: Negative-path validation for each in-scope mode continues to
  surface explicit missing-content or blocked-packet markers whenever required
  authored sections are absent.
- **SC-003**: A maintainer unfamiliar with the slice can identify the intended
  persona and artifact shape for `discovery`, `system-shaping`, and `review`
  within two minutes by reading the produced planning artifacts and updated
  docs.
- **SC-004**: Existing validation for non-targeted modes shows no newly
  introduced behavioral regressions attributable to this follow-on slice.
- **SC-005**: Release-facing version surfaces and impacted docs consistently
  describe `0.30.0` and the shaped follow-on contract for this slice.

## Validation Plan *(mandatory)*

- **Structural validation**: Consistency checks across roadmap text, skill
  guidance, templates, examples, release-facing docs, and the declared artifact
  shape contracts for `discovery`, `system-shaping`, and `review`.
- **Logical validation**: Positive-path and negative-path packet walkthroughs,
  renderer checks, run/integration tests, and release-surface regression tests
  for `0.30.0`.
- **Independent validation**: A separate review pass over shaped packet outputs,
  validation evidence, and the final diff to confirm persona guidance did not
  weaken governance behavior or non-target mode stability.
- **Evidence artifacts**:
  `specs/030-artifact-shapes-follow-on/decision-log.md`,
  `specs/030-artifact-shapes-follow-on/validation-report.md`,
  `specs/030-artifact-shapes-follow-on/tasks.md`, focused test outputs,
  `lcov.info`, and release-facing doc diffs.

## Decision Log *(mandatory)*

- **D-001**: The follow-on slice covers `discovery`, `system-shaping`, and
  `review`, **Rationale**: these are the highest-leverage remaining modes for
  broadening industry-standard packet shapes without widening Canon into a new
  runtime domain.
- **D-002**: The slice keeps release alignment and validation closeout inside
  the feature scope, **Rationale**: this repository treats docs, compatibility
  anchors, and evidence as part of the shipped contract rather than post-merge
  cleanup.

## Non-Goals

- Add new runtime modes or change approval, evidence, or publish semantics.
- Complete the entire remaining artifact-shapes roadmap in one slice.
- Rework packaging or distribution channels as part of this feature.
- Revisit already-delivered `requirements`, `architecture`, `change`, or
  `pr-review` packet shapes except where docs or validation need alignment.

## Assumptions

- The 021 first slice remains the baseline and this feature extends the same
  artifact-shape plus persona contract instead of redefining it.
- Existing skill, renderer, template, and example surfaces for `discovery`,
  `system-shaping`, and `review` are sufficient foundations for a bounded
  follow-on slice.
- The repository version after 029 should move to `0.30.0` for this delivery.
- Maintainers still expect release-facing docs, coverage, `cargo clippy`, and
  `cargo fmt` to be explicit first-class tasks in the feature workflow.
