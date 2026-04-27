# Feature Specification: Industry-Standard Artifact Shapes With Personas

**Feature Branch**: `021-artifact-shapes-personas`  
**Created**: 2026-04-27  
**Status**: Draft  
**Input**: User description: "Implement industry-standard artifact shapes with explicit mode-specific personas in Canon skills and renderers"

## Governance Context *(mandatory)*

**Mode**: change
**Risk Classification**: bounded-impact because this modifies existing skill,
renderer, validation, and documentation behavior across already-modeled Canon
modes without introducing a new runtime domain
**Scope In**: first-slice support for `requirements`, `architecture`, and
`change` to gain explicit industry-standard artifact shapes plus bounded
authoring personas; corresponding renderer preservation behavior; validation
evidence; and operator-facing documentation that explains the mapping
**Scope Out**: new execution or analysis modes; package-manager distribution;
protocol interoperability; full persona rollout for every remaining mode;
loosening approval, evidence, or missing-authored-body behavior

**Invariants**:

- Persona guidance MUST remain subordinate to Canon's artifact contracts,
  missing-authored-body markers, risk gates, and evidence requirements.
- In-scope artifact shapes MUST remain explicitly reviewable and must not rely
  on chat history or hidden prompting to explain structure or intent.
- Modes outside the first slice MUST keep their current observable behavior
  unless a later scoped change explicitly expands coverage.

**Decision Traceability**: Decisions start in this specification and continue in
`specs/021-artifact-shapes-personas/decision-log.md`, with validation evidence
recorded in `specs/021-artifact-shapes-personas/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ship Shaped Authoring For High-Leverage Modes (Priority: P1)

As a Canon maintainer, I want the `requirements`, `architecture`, and `change`
skills to guide the assistant with both an industry-standard packet shape and a
bounded authored persona so the generated packets read like credible working
artifacts instead of generic AI summaries.

**Why this priority**: This is the smallest slice that proves the roadmap item
the user selected and it directly improves the highest-leverage packet types for
planning and structural decision-making.

**Independent Test**: With one representative authored brief per in-scope mode,
the resulting packet can be reviewed on its own and judged as both
contract-compliant and audience-appropriate without extra prompt context.

**Acceptance Scenarios**:

1. **Given** an authored requirements brief with the expected canonical
   sections, **When** the assistant follows Canon's `requirements` skill,
   **Then** the emitted packet reads as a PRD shaped for a product-facing
   audience and still preserves Canon's explicit scope, tradeoff, and open
   question structure.
2. **Given** an authored architecture brief with the expected decision and C4
   sections, **When** the assistant follows Canon's `architecture` skill,
   **Then** the emitted packet reads as a C4 plus ADR-style architecture packet
   authored from an architecture-decision perspective without losing explicit
   constraints, drivers, consequences, or rejected alternatives.
3. **Given** an authored change brief with the expected bounded-change sections,
   **When** the assistant follows Canon's `change` skill, **Then** the emitted
   packet reads as a bounded ADR-style change record authored from a change
   owner perspective while preserving invariants, sequencing, and validation
   boundaries.

---

### User Story 2 - Keep Personas Bounded By Canon Governance (Priority: P2)

As a reviewer of Canon outputs, I want persona guidance to improve voice and
audience fit without hiding missing authored content, weakening evidence, or
inventing authority so that shaped packets stay trustworthy.

**Why this priority**: Persona support is only valuable if it does not break
Canon's core honesty guarantees. This is the highest-risk failure mode of the
feature and must be validated as a separate slice.

**Independent Test**: A negative-path walkthrough that intentionally omits
required authored sections still produces explicit gap markers and unchanged
approval or evidence posture even when persona instructions are active.

**Acceptance Scenarios**:

1. **Given** an in-scope brief that omits a required authored section,
   **When** the assistant follows the persona-aware skill, **Then** the emitted
   artifact still surfaces `## Missing Authored Body` or the equivalent explicit
   gap signal for the canonical section instead of filling the gap with
   persona-shaped prose.
2. **Given** a persona instruction that could imply stronger authority than the
   evidence supports, **When** the packet is generated, **Then** Canon still
   presents recommendations, evidence gaps, and risk posture according to the
   existing mode contract rather than the persona's implied seniority.

---

### User Story 3 - Make Persona And Shape Mapping Discoverable (Priority: P3)

As a maintainer or operator adopting the feature, I want the mode-to-shape and
mode-to-persona mapping to be explicit in planning artifacts and docs so I can
understand which modes changed now, which are deferred, and how the guidance is
supposed to be used.

**Why this priority**: Discoverability reduces future drift. It also keeps the
first slice bounded by making deferrals explicit rather than tacit.

**Independent Test**: A new maintainer can identify the targeted modes, their
intended artifact shapes, and their intended personas by reading the produced
specification, planning artifacts, and updated roadmap without reading source
code first.

**Acceptance Scenarios**:

1. **Given** the completed planning and documentation artifacts, **When** a
   maintainer reviews the feature package, **Then** the in-scope modes, persona
   mappings, and deferred follow-on coverage are explicit.
2. **Given** a mode outside the first slice, **When** a maintainer reviews the
   same package, **Then** it is clear that the mode remains out of scope for the
   current feature rather than silently inheriting incomplete persona guidance.

### Edge Cases

- What happens when the persona guidance pushes toward a tone or structure that
  conflicts with a canonical heading contract?
- How does the system handle first-slice packets where the authored brief is
  structurally incomplete but the persona could plausibly improvise missing
  narrative?
- Which invariant is most likely to be stressed when a non-targeted mode shares
  renderer helpers with a targeted mode?
- How are deferred modes documented so operators do not assume persona support
  exists everywhere after the first slice ships?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST define an explicit authored persona for each
  first-slice mode in scope for this feature.
- **FR-002**: The system MUST define an explicit industry-standard artifact
  shape for each first-slice mode in scope for this feature.
- **FR-003**: The system MUST map `requirements` to a PRD-style packet,
  `architecture` to a C4 plus ADR-style packet, and `change` to an ADR-style
  bounded change packet for the first slice.
- **FR-004**: The system MUST make the intended audience and authored persona
  discoverable in the relevant skill guidance and operator-facing feature
  artifacts.
- **FR-005**: Persona guidance MUST remain subordinate to canonical artifact
  contracts, evidence rules, approval posture, and explicit missing-content
  markers.
- **FR-006**: For every first-slice mode, the system MUST preserve authored
  sections in the declared artifact shape and MUST emit explicit missing-content
  markers when canonical authored sections are absent.
- **FR-007**: The system MUST provide validation evidence that persona-aware
  shaping improves packet fit without masking missing authored content or
  changing governance behavior.
- **FR-008**: The system MUST record which modes are in scope now and which are
  deferred to later slices.
- **FR-009**: The system MUST leave non-targeted modes behaviorally unchanged
  for the current slice unless they are explicitly pulled into scope by a later
  feature.
- **FR-010**: Maintainers and reviewers MUST be able to evaluate whether a
  generated packet matches both the declared shape and the declared persona
  using durable artifacts rather than chat-only context.

### Key Entities *(include if feature involves data)*

- **Mode Persona Profile**: the declared authored counterpart for a mode,
  including intended audience, critique posture, and boundaries on authority.
- **Artifact Shape Contract**: the named packet shape a mode must follow,
  including required sections and the relationship between authored input and
  emitted artifacts.
- **Generated Packet Review Surface**: the bundle of emitted artifacts a
  maintainer or reviewer uses to judge whether the packet matches the declared
  shape and persona while preserving Canon's governance semantics.
- **Validation Evidence Record**: the durable artifact set that captures
  positive-path and negative-path checks for shape preservation, missing-content
  honesty, and deferred-scope behavior.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For each first-slice mode, at least one representative authored
  brief can be turned into a packet that an independent reviewer judges as both
  contract-compliant and audience-appropriate without additional chat context.
- **SC-002**: In negative-path validation for each first-slice mode, omitted
  required authored sections continue to surface explicit missing-content
  markers in every reviewed packet.
- **SC-003**: A maintainer unfamiliar with the feature can identify the
  intended persona and artifact shape for each first-slice mode within two
  minutes by reading the produced planning artifacts and docs.
- **SC-004**: Existing validation for non-targeted modes shows no newly
  introduced behavioral regressions attributable to this first slice.
- **SC-005**: All first-slice persona and shape mappings, plus all deferrals,
  are recorded in durable decision and validation artifacts before the feature
  is considered complete.

## Validation Plan *(mandatory)*

- **Structural validation**: Consistency checks across roadmap, skill guidance,
  feature artifacts, and packet-shape contracts for the first-slice modes.
- **Logical validation**: Positive-path and negative-path packet walkthroughs
  for `requirements`, `architecture`, and `change`, including preserved-shape
  checks and missing-content honesty checks.
- **Independent validation**: A separate review pass over the emitted packet
  examples and validation evidence to confirm that persona guidance did not
  change governance behavior.
- **Evidence artifacts**: `specs/021-artifact-shapes-personas/decision-log.md`,
  `specs/021-artifact-shapes-personas/validation-report.md`,
  `specs/021-artifact-shapes-personas/tasks.md`, and any generated contract or
  walkthrough artifacts created during planning and implementation.

## Decision Log *(mandatory)*

- **D-001**: The first slice covers `requirements`, `architecture`, and
  `change`, **Rationale**: these modes have the highest leverage for proving the
  combined artifact-shape plus persona contract without widening the runtime
  surface.
- **D-002**: Persona guidance is bounded and may never override explicit
  contract, evidence, or gap-reporting behavior, **Rationale**: Canon's core
  value depends on visible truthfulness when authored input is incomplete.

## Non-Goals

- Add new Canon modes as part of this slice.
- Roll out personas to every remaining mode before the first-slice contract is
  proven.
- Change package-manager distribution, release channels, or protocol
  interoperability as part of this feature.
- Replace Canon's existing approval, evidence, or missing-authored-body
  semantics with persona-driven defaults.

## Assumptions

- Canon's existing roadmap direction toward industry-standard artifact shapes is
  the authoritative product context for this feature.
- Repo-local skills remain the primary surface through which assistants learn
  how to author Canon packets.
- Existing renderer behavior can be extended to preserve persona-aware shapes
  without changing the underlying governance model.
- The first slice may defer persona rollout for `discovery`, `system-shaping`,
  `implementation`, `refactor`, `review`, `verification`, `incident`, and
  `migration` as long as the deferral is explicit.
