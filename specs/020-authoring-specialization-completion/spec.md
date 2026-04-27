# Feature Specification: Mode Authoring Specialization Completion

**Feature Branch**: `020-authoring-specialization-completion`  
**Created**: 2026-04-27  
**Status**: Draft  
**Input**: User description: "Complete Mode Authoring Specialization for review, verification, incident, and migration with canonical authored H2 contracts, honest missing-body markers, renderer preservation, skill/template/example alignment, version bump to 0.20.0, and roadmap/docs updates."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice completes an existing rollout across already-shipped modes without adding a new runtime domain, changing Canon persistence layout, or altering approval models. The blast radius is bounded to authored-body contracts, renderers, docs, examples, skills, tests, and version/documentation surfaces for `review`, `verification`, `incident`, and `migration`.  
**Scope In**:

- Complete the authored-body specialization rollout for `review`, `verification`, `incident`, and `migration`.
- Define canonical authored H2 contracts for every emitted artifact in those four modes.
- Preserve authored sections verbatim in emitted markdown artifacts and emit explicit `## Missing Authored Body` markers for missing required sections.
- Keep run-state semantics honest when authored input is incomplete, including any gate-blocked or approval-gated behavior already defined for these modes.
- Synchronize embedded skills, materialized skills, templates, worked examples, roadmap text, guide text, changelog entries, and compatibility references.
- Bump Canon version references from `0.19.0` to `0.20.0` where the repo treats the release as part of the delivered feature contract.

**Scope Out**:

- Reopening already-completed authored-body slices for `requirements`, `discovery`, `system-shaping`, `architecture`, `change`, `implementation`, or `refactor`, except for non-behavioral references needed for docs/version sync.
- Introducing new modes, new publish destinations, new runtime persistence schema, or new adapter capabilities.
- Changing the critique-first posture of `review` and `verification`, or the recommendation-only operational posture of `incident` and `migration`.
- Expanding into industry-standard artifact shape work beyond the authored-body contract needed to complete this rollout.
- Solving unrelated local hook/test harness hangs unless directly required to validate this feature slice.

**Invariants**:

- Missing required authored sections MUST surface `## Missing Authored Body` naming the canonical heading; Canon MUST NOT synthesize plausible filler for absent authored bodies.
- `review` and `verification` MUST remain critique-first and evidence-oriented; authoring specialization cannot weaken independent validation or flatten findings severity.
- `incident` and `migration` MUST remain recommendation-only operational modes with their current containment, fallback, and release-readiness posture preserved.
- Non-target modes, run identity, approval target semantics, publish destinations, and `.canon/` persistence layout MUST remain unchanged.
- Skills, templates, examples, renderer expectations, and focused validation MUST describe the same canonical H2 contract for each targeted mode.

**Decision Traceability**: Decisions for this feature will be recorded in `specs/020-authoring-specialization-completion/decision-log.md`, with validation evidence recorded in `specs/020-authoring-specialization-completion/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reviewers And Validators Author Real Bodies (Priority: P1)

A Canon author working in `review` or `verification` wants the skill, template, and runtime artifact contract to state exactly which authored H2 sections are required, so emitted packets preserve real findings, challenge evidence, and verdict material instead of drifting into boilerplate.

**Why this priority**: `review` and `verification` are the highest-leverage remaining generative quality modes. If these packets still accept vague input and emit generic prose, Canon remains weak at the exact point where users expect judgment and evidence.

**Independent Test**: Read the updated skill/template/example for `review` or `verification`, run one complete packet and one incomplete packet, and confirm authored sections are preserved verbatim while missing required sections are named explicitly with `## Missing Authored Body`.

**Acceptance Scenarios**:

1. **Given** a complete authored `review` brief, **When** Canon emits the review packet, **Then** the packet preserves authored bodies for review-specific sections verbatim and keeps findings-oriented structure intact.
2. **Given** a `verification` brief missing a required challenge or evidence section, **When** Canon emits the verification packet, **Then** the affected artifact names the missing canonical H2 section with `## Missing Authored Body` and does not replace it with templated filler.

---

### User Story 2 - Operational Authors Get Honest Incident And Migration Packets (Priority: P2)

An operator authoring `incident` or `migration` input wants containment, compatibility, rollback, and follow-up artifacts to preserve real authored content and surface explicit gaps, so operational packets remain trustworthy in higher-stakes workflows.

**Why this priority**: These are the last remaining modeled modes in the specialization rollout, and operational work is where fake completeness is most dangerous.

**Independent Test**: Run `incident` and `migration` with complete and incomplete authored packets, then confirm complete packets preserve authored bodies and incomplete packets emit honest missing-body markers without changing recommendation-only posture.

**Acceptance Scenarios**:

1. **Given** a complete authored `incident` packet, **When** Canon emits containment artifacts, **Then** the containment and decision artifacts preserve the authored operational sections verbatim.
2. **Given** a `migration` brief missing a required fallback or rollback section, **When** Canon emits the packet, **Then** the corresponding artifact explicitly names the missing canonical heading and the packet remains governed under the existing posture.

---

### User Story 3 - Maintainers Can Ship 0.20.0 With Clear Documentation (Priority: P3)

A maintainer wants the remaining rollout, docs, and release surfaces to describe the full completion of Mode Authoring Specialization accurately, so future work can move on to the next roadmap item without ambiguity.

**Why this priority**: Once the remaining four modes are finished, the roadmap and user-facing docs need to say the rollout is complete, otherwise the next increment will either duplicate work or understate delivered capability.

**Independent Test**: Read roadmap, mode guide, compatibility references, and changelog after the slice lands, then run targeted validations proving the four remaining modes now match the specialization contract and the repo reports `0.20.0` consistently.

**Acceptance Scenarios**:

1. **Given** a maintainer reads the roadmap after this slice lands, **When** they inspect the authoring-specialization section, **Then** it states the rollout is complete and no longer lists the four remaining modes as open follow-on scope.
2. **Given** a maintainer inspects versioned docs and manifests, **When** they compare release-facing references, **Then** they all report `0.20.0` consistently for this delivery slice.

### Edge Cases

- A brief uses a near-match heading such as `## Follow Up` instead of the canonical heading required by the targeted artifact.
- A required authored section exists but contains only whitespace or a placeholder bullet.
- A `review` packet includes strong findings text but omits the authored evidence basis that keeps the packet reviewable.
- A `verification` packet includes claims under test but omits the authored contradiction or required follow-up material.
- An `incident` packet authors containment steps but omits release-readiness or follow-up verification sections.
- A `migration` packet authors sequencing but omits fallback or rollback credibility sections.
- The docs claim rollout completion while one of the four targeted modes still lacks template/example/skill parity.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `review` MUST define an explicit canonical authored H2 contract for every emitted artifact in its packet.
- **FR-002**: `verification` MUST define an explicit canonical authored H2 contract for every emitted artifact in its packet.
- **FR-003**: `incident` MUST define an explicit canonical authored H2 contract for every emitted artifact in its packet.
- **FR-004**: `migration` MUST define an explicit canonical authored H2 contract for every emitted artifact in its packet.
- **FR-005**: Each targeted mode MUST ship an `Author <Mode> Body Before Invoking Canon` skill section that enumerates the required authored H2 sections per artifact.
- **FR-006**: Materialized `.agents/skills/` copies for the targeted modes MUST remain synchronized with the embedded skill sources.
- **FR-007**: The markdown renderer MUST preserve authored H2 bodies verbatim for the targeted modes whenever the canonical heading is present and non-empty.
- **FR-008**: When a required authored section for a targeted mode is absent or empty, the renderer MUST emit `## Missing Authored Body` naming the missing canonical heading.
- **FR-009**: The relevant orchestrator paths for the targeted modes MUST provide the renderer access to the authored packet text needed for section extraction.
- **FR-010**: Each targeted mode MUST ship starter templates under `docs/templates/canon-input/` that use the same canonical authored H2 contract documented in the skill.
- **FR-011**: Each targeted mode MUST ship worked examples under `docs/examples/canon-input/` that demonstrate complete authored packets aligned to the canonical contract.
- **FR-012**: Focused renderer, contract, run, and docs tests MUST exist for each targeted mode, covering both verbatim preservation and honest missing-body behavior.
- **FR-013**: `review` and `verification` MUST keep their existing critique-first, evidence-backed, and independent-validation posture unchanged.
- **FR-014**: `incident` and `migration` MUST keep their existing recommendation-only operational posture, gate semantics, and publish behavior unchanged.
- **FR-015**: `ROADMAP.md`, `docs/guides/modes.md`, `CHANGELOG.md`, and runtime-compatibility references MUST describe this slice as the completion of Mode Authoring Specialization.
- **FR-016**: Cargo manifests, shared compatibility references, and release-facing documentation MUST report Canon version `0.20.0` for this delivery.
- **FR-017**: Non-target modes MUST retain their current runtime behavior unchanged except for documentation or version references required by this feature.

### Key Entities *(include if feature involves data)*

- **Targeted Mode Authored Contract**: The per-artifact mapping from an emitted artifact to the canonical authored H2 sections required in the source brief for `review`, `verification`, `incident`, or `migration`.
- **Missing Authored Body Marker**: The explicit honesty block emitted when a required authored section was not supplied.
- **Review Packet**: The bounded artifact family for review findings, evidence, decision impact, and disposition.
- **Verification Packet**: The bounded artifact family for claims under test, evidence basis, challenge findings, contradictions, verdicts, and follow-up.
- **Incident Packet**: The bounded operational artifact family for containment, blast radius, decisions, and follow-up verification.
- **Migration Packet**: The bounded operational artifact family for source-target mapping, compatibility, sequencing, fallback, and migration verification.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each targeted mode has at least one focused positive test proving authored sections are preserved verbatim in emitted artifacts.
- **SC-002**: Each targeted mode has at least one focused negative test proving a missing required authored section emits `## Missing Authored Body` naming the canonical heading.
- **SC-003**: Skills, templates, and worked examples for all four targeted modes describe the same authored-body contract with no unresolved drift.
- **SC-004**: The targeted validation suite passes without changing review/verification independence semantics, incident/migration recommendation-only posture, or non-target mode behavior.
- **SC-005**: `ROADMAP.md`, `docs/guides/modes.md`, `CHANGELOG.md`, and compatibility references all reflect `0.20.0` and describe Mode Authoring Specialization as completed.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: Focused renderer, contract, run, and docs tests for `review`, `verification`, `incident`, and `migration`, plus targeted non-regression checks for previously completed specialization modes.
- **Independent validation**: Cross-artifact review of `spec.md`, `plan.md`, and `tasks.md`, followed by one complete-packet walkthrough and one missing-section walkthrough per targeted mode.
- **Evidence artifacts**: Validation results and findings recorded in `specs/020-authoring-specialization-completion/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Complete the remaining four modes inside one bounded slice, **Rationale**: finishing the rollout in one increment avoids a half-complete roadmap state and reuses the same renderer/skill/template pattern while context is fresh.
- **D-002**: Treat `0.20.0` as part of the feature contract, **Rationale**: the repo uses versioned compatibility/docs surfaces to describe delivered capability, so rollout completion should ship with a coherent release marker.
- **D-003**: Preserve each mode's current governance posture while improving authored-body fidelity, **Rationale**: the value of this slice is output honesty and discoverability, not policy churn.

## Non-Goals

- Introducing new modes, new packet families, or new external evidence collectors.
- Reopening already-delivered authored-body specialization work for non-target modes beyond sync-level references.
- Expanding into industry-standard artifact shape work beyond what is required to preserve authored H2 bodies.
- Changing `.canon/` persistence layout, run identity semantics, or publish destinations.
- Solving unrelated local tooling issues except where a fix is required to validate this feature.

## Assumptions

- `review`, `verification`, `incident`, and `migration` already have stable enough artifact families that authored-body specialization can be layered on without a new domain model.
- Existing renderer helpers for authored-section extraction and missing-body markers can be reused for the remaining modes with bounded additions.
- The next available sequential feature slot is `020`, and the release/version references can move to `0.20.0` as part of this slice.
- Users primarily discover authoring requirements through skills, templates, examples, and docs rather than source-code inspection.
- Completing this rollout is more valuable for the next increment than starting a new artifact-shape or decision-analysis feature before the current pattern is finished.
