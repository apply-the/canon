# Feature Specification: Stronger Architecture Outputs (C4 Model)

**Feature Branch**: `015-architecture-c4`
**Created**: 2026-04-25
**Status**: Draft
**Input**: User description: "Strengthen the existing architecture mode by adding C4 model artifacts (system context, container, component) alongside the existing decision, invariants, boundary, and tradeoff artifacts. The mode must remain critique-first and must not collapse into diagram-only output. Authored input must drive C4 content; renderer must preserve authored sections verbatim and emit explicit Missing Authored Body markers when sections are absent. Templates and examples for the new C4 sections must ship under docs/templates/canon-input/architecture/ and docs/examples/canon-input/architecture/. Existing architecture runs that do not author C4 sections must keep working with explicit honest blockers."

## Governance Context *(mandatory)*

**Mode**: change

**Risk Classification**: bounded-impact. The work extends an already-delivered mode (`architecture`) by adding new artifacts and renderer logic, plus templates and examples. It does not change run identity, persistence layout, publish flow, or other modes. The risk is bounded to the architecture artifact set, its renderer, its skill, and the docs that accompany it.

**Scope In**:

- Extend the `architecture` mode artifact contract to include C4 model artifacts: system context, container, and component views.
- Update the renderer for architecture artifacts to extract and preserve authored C4 H2 sections verbatim, and to emit explicit `## Missing Authored Body` markers when authored content is absent.
- Update the `canon-architecture` skill (both `defaults/embedded-skills/` and the materialized `.agents/skills/`) to require authored C4 body sections before invoking Canon.
- Add starter templates under `docs/templates/canon-input/architecture/` that describe the required H2 sections for each C4 view.
- Add realistic examples under `docs/examples/canon-input/architecture/` that demonstrate a credible C4 packet.
- Keep the existing decision, invariants, boundary, tradeoff, and readiness artifacts intact and emitted alongside the new C4 artifacts.

**Scope Out**:

- Adding C4 Level 4 (Code) views.
- Producing diagram files (PlantUML, Mermaid renders, SVG). C4 artifacts in this slice are textual and table-based.
- Reworking other modes (`system-shaping`, `change`, etc.) to emit C4 content.
- Changing approval gating, run identity, or publish layout for `architecture`.
- Introducing the broader Mode Authoring Specialization roadmap feature; this slice only applies the authored-body pattern to the architecture C4 sections.
- Reworking the existing critique-first artifact bodies.

**Invariants**:

- `architecture` MUST remain critique-first; the run MUST NOT collapse into a diagram-only output. Decision, invariants, boundary, and tradeoff artifacts MUST still be emitted.
- Authored content for C4 sections MUST be preserved verbatim by the renderer whenever present in the supplied brief.
- When an authored C4 section is absent, the renderer MUST emit an explicit `## Missing Authored Body` marker for that section instead of fabricating content.
- The C4 expansion MUST NOT regress existing architecture runs: a run that does not include the new C4 sections MUST still complete or block honestly, with the missing C4 content surfaced explicitly rather than silently filled.
- The new artifacts MUST remain inspectable, publishable, and reviewable through Canon's existing governed runtime model without changes to run identity or publish targets.

**Decision Traceability**: Decisions for this feature are recorded in `specs/015-architecture-c4/decision-log.md` and cross-linked from the Canon change run that implements the feature under `.canon/runs/<...>/decisions/`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Architect gets a C4-shaped architecture packet (Priority: P1)

An architect needs Canon to produce an architecture packet that is readable in a familiar industry shape (C4 model) so the output can be shared with engineers and reviewers outside Canon.

**Why this priority**: This is the primary outcome of the feature. The architecture mode already exists; what makes this slice valuable is the recognizable C4 shape that lets the packet be consumed by people who do not know Canon's internal artifact vocabulary.

**Independent Test**: Start an `architecture` run from an authored brief that includes C4 sections, then confirm the emitted packet contains explicit `system-context.md`, `container-view.md`, and `component-view.md` artifacts with the authored content preserved verbatim, alongside the existing decision, invariants, boundary, and tradeoff artifacts.

**Acceptance Scenarios**:

1. **Given** an architecture brief with authored C4 sections (System Context, Containers, Components), **When** the user starts an `architecture` run, **Then** Canon emits `system-context.md`, `container-view.md`, and `component-view.md` with the authored content preserved verbatim.
2. **Given** the same run, **When** the user inspects the artifact set, **Then** the existing `architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`, `boundary-map.md`, and `readiness-assessment.md` artifacts are still present and unchanged in shape.
3. **Given** the published architecture packet, **When** an engineer outside the run reads `system-context.md`, `container-view.md`, and `component-view.md`, **Then** they can identify the bounded system, its external actors and dependencies, its internal containers, and the components inside the most relevant container without consulting other files.

---

### User Story 2 - Honest blocker when C4 content is missing (Priority: P2)

A reviewer wants the architecture packet to be explicit when the AI did not author the C4 sections, so the packet does not silently look complete when it is not.

**Why this priority**: This is the truthfulness guarantee of the feature. Without it, the renderer would fabricate generic C4 placeholders that are worse than nothing.

**Independent Test**: Start an `architecture` run from a brief that omits one or more C4 sections, then confirm the corresponding artifact is still emitted but contains an explicit `## Missing Authored Body` marker instead of fabricated content, and the run state reflects the gap.

**Acceptance Scenarios**:

1. **Given** an architecture brief that omits the C4 System Context section, **When** the user runs `architecture`, **Then** `system-context.md` is emitted with an explicit `## Missing Authored Body` marker.
2. **Given** an architecture brief that omits all C4 sections, **When** the user runs `architecture`, **Then** all three C4 artifacts are emitted with explicit `## Missing Authored Body` markers, and the run does not present a false ready-to-share posture.
3. **Given** a brief that authored only `## System Context` and `## Containers` but not `## Components`, **When** the user inspects the packet, **Then** the first two C4 artifacts contain authored content verbatim and the third contains the explicit missing-body marker.

---

### User Story 3 - Templates and examples bootstrap the new shape (Priority: P3)

A new user wants to be able to start a credible C4-shaped architecture brief without already knowing the required section list, by copying a starter template or a realistic example.

**Why this priority**: Without templates and examples, the new authored-body contract is hidden behind the renderer and the skill text. Templates make the contract self-evident.

**Independent Test**: Confirm that `docs/templates/canon-input/architecture/brief.md` documents all required H2 sections (legacy + C4) and that `docs/examples/canon-input/architecture/brief.md` contains a complete and plausible authored brief that, if used as `--input`, would produce a fully authored C4 packet.

**Acceptance Scenarios**:

1. **Given** the new template at `docs/templates/canon-input/architecture/brief.md`, **When** the user reads it, **Then** every required H2 section for the legacy artifacts and the new C4 artifacts is present with explanatory placeholders.
2. **Given** the new example at `docs/examples/canon-input/architecture/brief.md`, **When** the user runs an `architecture` run with that file as `--input`, **Then** the packet completes with authored C4 content preserved verbatim and no missing-body markers.

### Edge Cases

- The brief authors the C4 sections but uses subtly different headings (for example "## C4 - System Context") rather than the exact required heading.
- The brief authors the C4 sections in a different order than the canonical order.
- The brief authors only diagram syntax (Mermaid/PlantUML) inside the C4 sections, with no prose; the renderer must preserve the authored content verbatim, not collapse it.
- The brief authors very long C4 sections that include explicit external actors, dependencies, and crossing rules.
- The brief authors C4 sections, but the legacy decision/invariants/tradeoff sections are still missing; the run must not skip the existing critique artifacts in favor of the new C4 ones.
- A repository runs the new architecture mode without updating templates; the runtime contract MUST still be self-describing through the skill.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The `architecture` mode MUST emit three additional artifacts named `system-context.md`, `container-view.md`, and `component-view.md` alongside the existing decision, invariants, tradeoff, boundary, and readiness artifacts.
- **FR-002**: The new C4 artifacts MUST be governed by the same artifact contract layer as the existing architecture artifacts (typed contract with required artifact names and gate associations).
- **FR-003**: The architecture renderer MUST extract the authored H2 sections for `## System Context`, `## Containers`, and `## Components` from the supplied brief and render them verbatim into the corresponding C4 artifacts.
- **FR-004**: When an authored C4 H2 section is absent or empty, the renderer MUST emit a `## Missing Authored Body` marker section in the corresponding artifact instead of fabricating content.
- **FR-005**: The `canon-architecture` skill MUST require the assistant to author the C4 H2 sections in the supplied brief before invoking Canon, and MUST list those required sections explicitly.
- **FR-006**: The materialized `.agents/skills/canon-architecture/SKILL.md` MUST stay synchronized with the embedded skill source.
- **FR-007**: A starter template MUST exist at `docs/templates/canon-input/architecture/brief.md` that documents the full required H2 section list for both legacy and C4 artifacts.
- **FR-008**: A realistic example MUST exist at `docs/examples/canon-input/architecture/brief.md` that, when supplied as `--input`, yields a fully authored C4 packet without missing-body markers.
- **FR-009**: The architecture mode MUST remain critique-first; the run MUST NOT skip emission of the existing decision, invariants, tradeoff, boundary, or readiness artifacts in favor of the new C4 artifacts.
- **FR-010**: The C4 artifacts MUST be inspectable through `canon inspect artifacts` and publishable through `canon publish` using the existing architecture publish destination, without changes to run identity or publish layout.
- **FR-011**: Existing architecture tests MUST continue to pass; the C4 expansion MUST NOT regress decisions, invariants, tradeoffs, boundary, or readiness artifact rendering.

### Key Entities *(include if feature involves data)*

- **C4 System Context Artifact**: A textual description of the bounded system, its external actors, and its external dependencies, persisted as `system-context.md`.
- **C4 Container View Artifact**: A textual description of the bounded internal containers (deployable units, services, applications, data stores), persisted as `container-view.md`.
- **C4 Component View Artifact**: A textual description of the components inside the most relevant container and how they interact, persisted as `component-view.md`.
- **Architecture Brief**: The authored input that drives the architecture packet. Now expected to include both the legacy critique-first sections (decisions, invariants, tradeoffs, boundary, readiness) and the new C4 sections (System Context, Containers, Components).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: An architecture run with the supplied example brief produces three C4 artifacts whose authored content matches the brief verbatim.
- **SC-002**: An architecture run with a brief that omits any C4 section produces a C4 artifact for that section with an explicit `## Missing Authored Body` marker.
- **SC-003**: All existing architecture runtime, contract, and renderer tests continue to pass after the C4 expansion.
- **SC-004**: The `canon inspect artifacts` output for an architecture run with the example brief lists eight artifacts (five legacy + three C4) under the architecture artifact set.
- **SC-005**: The starter template at `docs/templates/canon-input/architecture/brief.md` enumerates every required H2 section for both legacy and C4 artifacts.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: targeted Rust tests covering the architecture artifact contract, the renderer behavior for authored and missing C4 sections, the architecture run end-to-end with the new sections, and the inspect artifacts surface listing all eight artifacts; plus existing architecture-related test suites for non-regression.
- **Independent validation**: generate a representative architecture packet in a temporary repository using the new example brief, publish it, and read only the published markdown to confirm the C4 artifacts are credible and the legacy critique artifacts are preserved.
- **Evidence artifacts**: results recorded in `specs/015-architecture-c4/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Use the textual C4 model (system context, container, component) rather than diagram-syntax artifacts. **Rationale**: keeps the artifact set reviewable without requiring a diagram renderer and matches the authored-body contract used elsewhere in Canon.
- **D-002**: Emit the C4 artifacts alongside the existing critique artifacts rather than replacing any of them. **Rationale**: preserves the critique-first invariant and avoids regressing existing architecture consumers.
- **D-003**: When an authored C4 section is missing, emit the artifact with an explicit `## Missing Authored Body` marker rather than skipping the artifact. **Rationale**: keeps the artifact set predictable and makes the authoring gap visible to reviewers.

## Non-Goals

- C4 Level 4 (Code) views.
- Generated diagrams (Mermaid, PlantUML, SVG).
- Authored-body expansion of other modes (deferred to the broader Mode Authoring Specialization roadmap feature).
- Domain-Driven Design artifacts (deferred to the Domain Modeling And Boundary Design roadmap feature).
- Changes to run identity, publish destinations, or persistence layout.

## Assumptions

- The existing `canon-architecture` skill workflow (file-backed brief under `canon-input/architecture.md` or `canon-input/architecture/`) remains the primary entry point for authored C4 input.
- The existing publish destination for `architecture` continues to handle the expanded artifact set without layout changes.
- Authored C4 sections in the brief use the canonical headings `## System Context`, `## Containers`, and `## Components`.
- The architecture mode keeps its current gate profile; this slice does not add a new gate.
