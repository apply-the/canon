# Feature Specification: Mode Authoring Specialization Follow-On

**Feature Branch**: `019-authoring-specialization-remaining`  
**Created**: 2026-04-26  
**Status**: Draft  
**Input**: User description: "Continue Mode Authoring Specialization for system-shaping, implementation, and refactor so their packets use explicit authored H2 contracts, verbatim renderer preservation, honest missing-body markers, synchronized skills/templates/examples, and focused validation."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice deepens three already-delivered modes without introducing a new runtime domain, changing Canon run identity, approval semantics, publish destinations, or persistence layout. The blast radius is bounded to authored-body contracts, renderer behavior, skill guidance, templates, examples, docs, and focused tests for `system-shaping`, `implementation`, and `refactor`.  
**Scope In**:

- Extend `system-shaping` so its remaining packet artifacts require explicit authored H2 sections instead of leaning on summary-derived prose.
- Extend `implementation` so all emitted artifacts can preserve authored H2 sections verbatim and surface explicit missing-body markers when required sections are absent.
- Extend `refactor` so all emitted artifacts can preserve authored H2 sections verbatim and surface explicit missing-body markers when required sections are absent.
- Keep the current first-slice authored-body patterns as the reference behavior and apply them consistently to the targeted modes.
- Update skill guidance, starter templates, worked examples, mode docs, roadmap text, and focused validation coverage for the targeted modes.
- Ensure the relevant orchestrator paths pass the real authored brief content to the renderer so preservation logic operates on the authored body, not only on derived summaries.

**Scope Out**:

- Reopening `requirements`, `discovery`, `change`, or `architecture`, which already received the delivered first slice.
- Extending the same pattern in this slice to `review`, `verification`, `incident`, or `migration`.
- Replacing critique-first packet shapes with broader industry-standard artifacts such as PRDs, full ADR suites, or external framework-evaluation dossiers.
- Changing run identity, approval gates, recommendation-only posture, persistence layout, evidence bundle semantics, or publish destinations.
- Introducing new Canon modes or external evidence collectors.

**Invariants**:

- `system-shaping`, `implementation`, and `refactor` MUST remain critique-first and evidence-backed; authored-body preservation cannot silently suppress evidence gaps or critical findings.
- Missing required authored sections MUST surface `## Missing Authored Body` naming the canonical heading; Canon MUST NOT fabricate plausible filler for absent authored sections.
- `implementation` and `refactor` MUST retain their existing recommendation-only and approval-aware execution posture in v0.x.
- Non-target modes and already-delivered reference behaviors MUST remain unchanged.
- Canonical authored headings MUST remain explicit contracts; near-match headings count as missing unless an alias is intentionally documented.

**Decision Traceability**: Decisions for this feature will be recorded in `specs/019-authoring-specialization-remaining/decision-log.md`, with validation evidence recorded in `specs/019-authoring-specialization-remaining/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Authors Know Exactly What To Write (Priority: P1)

A Canon user wants the skills, templates, and examples for `system-shaping`, `implementation`, and `refactor` to state exactly which authored H2 sections are required for each emitted artifact, so they can prepare a real packet instead of guessing the mode contract.

**Why this priority**: If the authoring contract stays implicit, the runtime continues to accept thin briefs that degrade into placeholders, and the feature does not improve real usage.

**Independent Test**: Read the updated skill, template, and example for any targeted mode and confirm they enumerate the same required authored sections without needing source-code inspection.

**Acceptance Scenarios**:

1. **Given** a user opens the updated `implementation` skill and template, **When** they prepare a bounded execution brief, **Then** they can identify every required authored H2 section for the emitted packet.
2. **Given** a user opens the updated `system-shaping` example, **When** they compare it with the skill guidance, **Then** the example demonstrates the same authored-body contract and reads like a real packet instead of scaffold text.

---

### User Story 2 - Renderers Preserve Authored Body Honestly (Priority: P2)

A reviewer wants the emitted artifacts for `system-shaping`, `implementation`, and `refactor` to preserve authored sections verbatim when they exist, and to emit explicit missing-body markers when they do not, so Canon stops masking thin input with generic filler.

**Why this priority**: This is the core runtime behavior change. Without honest preservation and explicit gaps, the documentation and skill changes do not materially improve the outputs.

**Independent Test**: Run each updated mode with one complete authored brief and one incomplete brief, then confirm that complete packets preserve authored sections verbatim and incomplete packets name the missing canonical section with `## Missing Authored Body`.

**Acceptance Scenarios**:

1. **Given** a complete authored `refactor` brief, **When** Canon emits the packet, **Then** the refactor artifacts preserve the authored section bodies verbatim.
2. **Given** an `implementation` brief missing one required rollback section, **When** Canon emits the packet, **Then** `rollback-notes.md` contains `## Missing Authored Body` and names the missing canonical heading.
3. **Given** a `system-shaping` brief with a near-match heading instead of the documented canonical heading, **When** Canon emits the packet, **Then** Canon treats that section as missing rather than silently accepting or rewriting it.

---

### User Story 3 - Maintainers Can Ship The Slice Safely (Priority: P3)

A maintainer wants the roadmap, docs, tests, and examples to describe the delivered second slice honestly, so later slices can extend the pattern without confusion or behavior drift.

**Why this priority**: This slice touches three modes across runtime and docs. Without synchronized validation and roadmap language, the next slice will either duplicate work or misstate what is already shipped.

**Independent Test**: Read the updated roadmap and mode guide, then run the targeted validation for the three modes and confirm the repository documents the delivered slice, keeps remaining scope explicit, and proves the runtime behavior.

**Acceptance Scenarios**:

1. **Given** a maintainer reads the roadmap after this slice lands, **When** they inspect the relevant section, **Then** they can see what the second slice delivered and which modes still remain for later rollout.
2. **Given** a maintainer runs the targeted validation suite, **When** the tests pass, **Then** they have evidence covering skill guidance, templates, examples, renderer behavior, and run-level behavior for the targeted modes.

### Edge Cases

- A brief includes a near-match heading such as `## Rollback Plan` instead of the canonical heading `## Rollback Steps`.
- A required authored section is present but blank or whitespace-only.
- A `system-shaping` packet provides a strong `domain-model.md` body but omits authored content for the rest of the packet.
- An `implementation` brief authors validation sections but omits mutation bounds; the packet must remain honest without altering execution posture.
- A `refactor` brief authors `Feature Audit` but omits `Decision`; the no-feature-addition packet must not look complete.
- A targeted mode mixes authored sections with legacy summary-driven content; authored required sections must remain verbatim and must not be replaced by generated scaffolding.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `system-shaping` MUST define an explicit authored-body contract for every emitted artifact in its packet, including canonical H2 sections for `system-shape.md`, `architecture-outline.md`, `capability-map.md`, `delivery-options.md`, and `risk-hotspots.md`, while preserving the existing explicit contract for `domain-model.md`.
- **FR-002**: `implementation` MUST define an explicit authored-body contract for every emitted artifact in its packet, including canonical H2 sections for `task-mapping.md`, `mutation-bounds.md`, `implementation-notes.md`, `completion-evidence.md`, `validation-hooks.md`, and `rollback-notes.md`.
- **FR-003**: `refactor` MUST define an explicit authored-body contract for every emitted artifact in its packet, including canonical H2 sections for `preserved-behavior.md`, `refactor-scope.md`, `structural-rationale.md`, `regression-evidence.md`, `contract-drift-check.md`, and `no-feature-addition.md`.
- **FR-004**: The embedded skill source for each targeted mode MUST include an `Author <Mode> Body Before Invoking Canon` section that enumerates the required authored H2 sections per artifact.
- **FR-005**: The materialized `.agents/skills/` copies for the targeted modes MUST remain synchronized with the embedded skill sources.
- **FR-006**: The markdown renderer MUST preserve targeted-mode authored sections verbatim whenever the canonical heading is present and non-empty.
- **FR-007**: When a required authored section is absent or empty, the renderer MUST emit `## Missing Authored Body` naming the missing canonical heading instead of generating fallback filler for that section.
- **FR-008**: The relevant orchestrator paths MUST pass the authored brief body through to the renderer so authored-section extraction operates on the real packet text.
- **FR-009**: Each targeted mode MUST ship a starter template under `docs/templates/canon-input/` that uses the same authored H2 contract documented in its skill.
- **FR-010**: Each targeted mode MUST ship a realistic worked example under `docs/examples/canon-input/` that exercises the authored-body contract across the full emitted packet.
- **FR-011**: `docs/guides/modes.md` MUST describe the new authored-body contract for the targeted modes without implying broader rollout completion.
- **FR-012**: `ROADMAP.md` MUST record this slice as delivered follow-on scope for Mode Authoring Specialization and MUST keep remaining rollout scope explicit.
- **FR-013**: `implementation` and `refactor` MUST keep their existing recommendation-only posture, gate behavior, and approval semantics unchanged.
- **FR-014**: Focused renderer, contract, run, and docs tests MUST exist for each targeted mode, covering both successful authored preservation and honest missing-body behavior.
- **FR-015**: Non-target modes MUST retain their current behavior unchanged except for non-behavioral roadmap or documentation references required to describe this slice.

### Key Entities *(include if feature involves data)*

- **Mode Authored-Body Contract**: The per-artifact mapping between an emitted packet artifact and the canonical authored H2 sections required in the source brief.
- **Missing Authored Body Marker**: The explicit honesty block that names a required authored section Canon did not receive.
- **System-Shaping Packet**: The bounded shaping packet whose remaining artifacts move from summary-heavy rendering to explicit authored-body preservation.
- **Execution Guidance Packet**: The `implementation` packet whose authored sections define task mapping, mutation bounds, validation, and rollback boundaries without changing execution posture.
- **Refactor Preservation Packet**: The `refactor` packet whose authored sections define preserved behavior, scope, regression evidence, drift review, and no-feature-addition posture.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each targeted mode has at least one focused positive test proving authored sections are preserved verbatim in emitted artifacts.
- **SC-002**: Each targeted mode has at least one focused negative test proving a missing required authored section emits `## Missing Authored Body` naming the canonical heading.
- **SC-003**: The skill guidance, template, and worked example for each targeted mode describe the same authored-body contract with no unresolved drift.
- **SC-004**: The targeted validation suite passes without changing run identity, approval semantics, recommendation-only posture, publish destinations, or non-target mode behavior.
- **SC-005**: The roadmap and mode guide describe this slice as delivered while still naming the remaining unspecialized modes as follow-on scope.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: Focused renderer, contract, run, and docs tests for `system-shaping`, `implementation`, and `refactor`, plus targeted non-regression validation for already-delivered reference modes.
- **Independent validation**: Review of `spec.md`, `plan.md`, and `tasks.md` before implementation, followed by one realistic example walkthrough and one negative authored-body fixture per targeted mode with a required H2 section removed.
- **Evidence artifacts**: Validation results and findings recorded in `specs/019-authoring-specialization-remaining/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Keep the second slice bounded to `system-shaping`, `implementation`, and `refactor`, **Rationale**: these modes still show the largest remaining authored-body gap while keeping the change surface smaller than a full remaining-mode rollout.
- **D-002**: Preserve recommendation-only and approval-aware behavior for execution-heavy modes, **Rationale**: authored-body specialization should improve packet honesty without reopening execution governance.
- **D-003**: Use `## Missing Authored Body` rather than templated filler for absent authored sections, **Rationale**: explicit incompleteness is safer and more reviewable than plausible-looking generated prose.
- **D-004**: Reuse the existing artifact families instead of introducing new packet shapes in this slice, **Rationale**: the current value is authored-body honesty, not artifact-family expansion.

## Non-Goals

- Extending the same authored-body rollout in this slice to `review`, `verification`, `incident`, or `migration`.
- Reopening `requirements`, `discovery`, `change`, or `architecture`, which already received the delivered first slice.
- Introducing broader industry-standard artifact shapes or live framework/ecosystem comparison collectors.
- Changing runtime state transitions, publish destinations, risk gating, or approval mechanics.
- Adding new Canon modes.

## Assumptions

- The current packets for `system-shaping`, `implementation`, and `refactor` are stable enough to accept explicit authored-body contracts without needing a new runtime model.
- The authored-section preservation helper already present in the renderer can be reused for the targeted modes with bounded additional mapping logic.
- Existing docs and examples remain the primary discoverability path for how users author Canon input packets.
- Maintaining identical behavior in already-delivered reference modes is more important than widening this slice further.
- The next available sequential feature slot is 019, consistent with the existing spec directories and project numbering configuration.
