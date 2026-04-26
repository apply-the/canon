# Feature Specification: Domain Modeling And Boundary Design

**Feature Branch**: `017-domain-boundary-design`  
**Created**: 2026-04-26  
**Status**: Draft  
**Input**: User description: "Domain Modeling And Boundary Design as the next delivered feature, making bounded contexts, ubiquitous language, context relationships, and domain invariants first-class across system-shaping, architecture, and change without weakening Canon's critique-first posture."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This feature deepens already-delivered modes and artifacts without introducing a new runtime domain, changing approval semantics, or altering persistence and publish contracts. The blast radius is bounded to artifact contracts, renderers, skill guidance, examples/templates, and the related tests and docs for `system-shaping`, `architecture`, and `change`.  
**Scope In**:

- Extend `system-shaping` so it can emit a candidate domain map, core-domain hypotheses, ubiquitous-language seeds, and explicit domain invariants.
- Extend `architecture` so it can emit bounded-context definitions, context relationships, ownership boundaries, integration seams, and anti-corruption candidates when warranted by the source material.
- Extend `change` so a bounded change must identify the domain slice it touches, the invariants it must preserve, and the ownership/context boundaries it must not silently cross.
- Update the corresponding skill guidance, templates, examples, docs, and tests so the new domain-modeling surfaces are explicit and reviewable.
- Preserve Canon's critique-first posture so weak domain boundaries are challenged rather than accepted as authored truth.

**Scope Out**:

- Introducing a separate new Canon mode for domain modeling.
- Replacing existing mode families or artifact families outside `system-shaping`, `architecture`, and `change`.
- Reworking runtime identity, approval gates, evidence layout, or publish destinations.
- Expanding this slice into packaging, protocol, distribution, or hosted workflow features.
- Completing industry-standard artifact-shape work for unrelated modes.
- Turning domain modeling into an implementation-planning engine or code-generation feature.

**Invariants**:

- `system-shaping`, `architecture`, and `change` MUST remain critique-first and evidence-backed; domain outputs cannot become unchallenged restatements of the brief.
- Existing run identity, state transitions, approval behavior, evidence linking, and publish destinations MUST remain unchanged.
- Domain outputs MUST stay explicitly bounded to authored source material and surfaced assumptions; Canon MUST not invent authoritative system boundaries that are unsupported by the input.
- `change` MUST continue to describe bounded existing-system modification rather than broad greenfield redesign.
- Non-target modes MUST remain behaviorally unchanged in this slice.

**Decision Traceability**: Decisions for this feature will be recorded in `specs/017-domain-boundary-design/decision-log.md`, with validation evidence captured in `specs/017-domain-boundary-design/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Shape A System Around Real Domain Boundaries (Priority: P1)

A user running `system-shaping` wants Canon to surface candidate bounded contexts, ubiquitous language, and core-versus-supporting domain hypotheses so the system can be shaped around business boundaries instead of generic component lists.

**Why this priority**: This is the earliest point where weak boundaries can still be corrected cheaply. If Canon does not make domain boundaries explicit here, downstream architecture and change work inherits ambiguity.

**Independent Test**: Run `system-shaping` on a realistic capability brief and confirm the resulting packet contains a candidate domain map, domain vocabulary, and explicit invariants that can be reviewed without reading source code.

**Acceptance Scenarios**:

1. **Given** a `system-shaping` brief describing a business capability with multiple responsibilities, **When** Canon completes the run, **Then** the packet identifies candidate bounded contexts and names their distinct responsibilities.
2. **Given** a `system-shaping` brief with ambiguous terminology, **When** Canon completes the run, **Then** the packet surfaces a ubiquitous-language seed and highlights conflicting or overloaded terms rather than collapsing them silently.

---

### User Story 2 - Formalize Context Boundaries In Architecture (Priority: P2)

An architect wants `architecture` packets to show bounded contexts, context relationships, and integration seams so architecture review can reason about ownership and coupling instead of only structure and components.

**Why this priority**: Once a system is moving from shaping into architecture, context boundaries become review-critical. Without this slice, architecture can stay structurally neat while remaining domain-blurry.

**Independent Test**: Run `architecture` on a context-rich brief and confirm the packet identifies bounded contexts, relationships between them, ownership boundaries, and anti-corruption candidates where crossings are risky.

**Acceptance Scenarios**:

1. **Given** an `architecture` brief that spans multiple business capabilities, **When** Canon emits the architecture packet, **Then** it distinguishes bounded contexts and shows their relationships instead of presenting one undifferentiated system.
2. **Given** an `architecture` brief with integration between contexts, **When** Canon emits the packet, **Then** it names the seam and whether anti-corruption or translation boundaries should be considered.

---

### User Story 3 - Keep Changes Honest About Domain Impact (Priority: P3)

A maintainer wants `change` runs to identify the domain slice they affect and the invariants they must preserve so a bounded change does not accidentally cross ownership or business-rule boundaries.

**Why this priority**: This protects the most failure-prone surface: a "small" change that is actually crossing domain seams. It is lower priority than shaping and architecture, but essential for downstream safety.

**Independent Test**: Run `change` on an existing-system modification and confirm the packet names the affected domain slice, preserved invariants, and ownership boundaries that must remain intact.

**Acceptance Scenarios**:

1. **Given** a `change` brief scoped to one subsystem, **When** Canon emits the change packet, **Then** it identifies the domain slice and the invariants that the change must preserve.
2. **Given** a `change` brief whose requested modification would cross context ownership boundaries, **When** Canon emits the packet, **Then** it explicitly surfaces that boundary crossing as a decision or risk rather than hiding it inside generic implementation language.

### Edge Cases

- The input describes responsibilities that overlap heavily, so the "right" context boundary is uncertain and must be surfaced as an explicit tradeoff rather than a false certainty.
- The authored brief uses inconsistent names for the same business concept across sections.
- A `change` brief is technically narrow but domain-wide in impact because it touches a shared invariant or integration seam.
- An `architecture` brief proposes structural boundaries that conflict with the domain boundaries inferred from responsibilities and ownership.
- A user provides too little domain detail to support confident context boundaries; Canon must surface the uncertainty explicitly instead of fabricating clean partitions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `system-shaping` MUST be able to emit a candidate domain map that identifies proposed bounded contexts and their primary responsibilities.
- **FR-002**: `system-shaping` MUST emit a ubiquitous-language seed that captures key domain terms, overloaded terms, and terminology that requires alignment.
- **FR-003**: `system-shaping` MUST surface core-domain, supporting-domain, or generic-domain hypotheses when the source material supports that distinction.
- **FR-004**: `system-shaping` MUST surface explicit domain invariants or business rules that later modes are expected to preserve.
- **FR-005**: `architecture` MUST be able to emit bounded-context definitions for the architecture under review.
- **FR-006**: `architecture` MUST be able to emit context relationships, including collaboration, dependency, and integration seams where the source material supports them.
- **FR-007**: `architecture` MUST surface ownership boundaries and anti-corruption candidates when context crossings present risk or ambiguity.
- **FR-008**: `change` MUST require the authored packet to identify the domain slice affected by the proposed change.
- **FR-009**: `change` MUST surface the domain invariants and ownership boundaries that the proposed change must preserve.
- **FR-010**: `change` MUST explicitly identify when a requested bounded change crosses or stresses a context boundary.
- **FR-011**: Skills, templates, and worked examples for `system-shaping`, `architecture`, and `change` MUST document the new domain-modeling sections consistently.
- **FR-012**: The markdown renderer and artifact contracts for the target modes MUST preserve the authored domain-modeling sections verbatim when provided and surface explicit missing-body honesty markers when required sections are absent.
- **FR-013**: Mode guidance and roadmap documentation MUST explain the delivered first slice and the remaining deferred scope without implying that all domain-modeling work is complete.
- **FR-014**: Focused contract, renderer, run, and docs tests MUST exist for the new domain-modeling surfaces across the three target modes.
- **FR-015**: Non-target modes MUST retain their current behavior and artifact contracts unchanged.

### Key Entities *(include if feature involves data)*

- **Bounded Context**: A coherent domain slice with its own responsibilities, terminology, and ownership boundary.
- **Context Relationship**: A defined interaction or dependency between two bounded contexts, including translation or anti-corruption needs where relevant.
- **Ubiquitous Language Seed**: A structured set of domain terms, meanings, conflicts, and alignment needs derived from the authored brief.
- **Domain Invariant**: A business rule or boundary condition that shaping, architecture, and change outputs must preserve explicitly.
- **Domain Slice**: The specific bounded context or cross-context surface affected by a proposed change.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `system-shaping`, `architecture`, and `change` each have at least one focused test proving their new domain-modeling sections appear in emitted artifacts when authored.
- **SC-002**: Each target mode has at least one focused negative test proving a required domain-modeling section surfaces an explicit missing-body marker when absent.
- **SC-003**: The target modes each ship synchronized skill guidance, template content, and example content describing the same domain-modeling contract.
- **SC-004**: A reviewer can inspect one packet per target mode and identify bounded contexts, domain vocabulary, and preserved invariants without needing to infer them from unrelated prose.
- **SC-005**: The targeted validation suite passes without changing approval semantics, publish destinations, or non-target mode behavior.
- **SC-006**: The roadmap and mode guidance clearly separate the delivered first slice from deferred follow-on work such as industry-standard artifact shapes or broader authoring specialization.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: Focused contract, renderer, run, and docs tests for `system-shaping`, `architecture`, and `change`, plus targeted non-regression coverage for adjacent existing mode behavior.
- **Independent validation**: Separate review of `spec.md`, `plan.md`, and `tasks.md` after planning, followed by one walkthrough packet per target mode using realistic authored examples and one negative fixture per mode with a required domain section removed.
- **Evidence artifacts**: Validation results and findings recorded in `specs/017-domain-boundary-design/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Keep the first slice bounded to `system-shaping`, `architecture`, and `change`, **Rationale**: these are the modes where domain boundaries create the highest leverage without widening runtime scope.
- **D-002**: Preserve critique-first behavior instead of treating authored domain boundaries as authoritative truth, **Rationale**: Canon's value is challenging weak structure, not rubber-stamping it.
- **D-003**: Treat industry-standard artifact shapes as a separate follow-on concern, **Rationale**: domain-boundary fidelity should land before broader shape standardization expands scope.

## Non-Goals

- Adding a standalone domain-modeling mode.
- Replacing existing architecture or change artifacts with an entirely new artifact family.
- Introducing packaging, distribution, deployment, or hosted workflow work.
- Extending the slice to `implementation`, `refactor`, `review`, `verification`, `incident`, or `migration`.
- Turning this feature into implementation planning, code generation, or execution automation.

## Assumptions

- Existing `system-shaping`, `architecture`, and `change` artifacts already have enough structure to absorb a first slice of domain-modeling sections without requiring a new runtime model.
- The current skill/template/example sync pattern used by recent slices is sufficient for keeping authored contracts aligned across docs and runtime behavior.
- Reviewers of Canon packets value explicit uncertainty about weak or ambiguous domain boundaries more than false precision.
- The repository will continue to treat roadmap text, mode guidance, and worked examples as first-class evidence of what a delivered slice actually covers.

