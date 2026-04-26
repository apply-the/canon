# Feature Specification: Mode Authoring Specialization

**Feature Branch**: `016-mode-authoring-specialization`  
**Created**: 2026-04-25  
**Status**: Draft  
**Input**: User description: "Mode Authoring Specialization for requirements discovery and change with authored H2 sections, verbatim renderer preservation, missing authored body markers, docs templates, examples, roadmap updates, and tests."

## Governance Context *(mandatory)*

**Mode**: change
**Risk Classification**: bounded-impact. This slice extends already-delivered modes and authoring skills without changing Canon run identity, approval semantics, persistence layout, or publish destinations. The blast radius is bounded to authored-body extraction, mode-specific skill guidance, templates/examples, and the corresponding tests and docs.
**Scope In**:

- Extend `requirements`, `discovery`, and `change` so they require explicit authored H2 sections for their emitted artifacts.
- Preserve authored artifact body sections verbatim in emitted markdown artifacts for those three modes.
- Emit an explicit `## Missing Authored Body` block when a required authored section is absent rather than fabricating content.
- Ensure the relevant orchestrator paths pass authored brief content through to the renderer so the renderer sees the real body rather than only derived summary text.
- Ship updated skill guidance, user-facing templates, realistic examples, docs guidance, roadmap updates, and validation coverage for the first-slice modes.

**Scope Out**:

- Expanding the pattern to `architecture`, `backlog`, or `pr-review`, which already act as delivered reference implementations.
- Expanding the pattern to `system-shaping`, `implementation`, `refactor`, `review`, `verification`, `incident`, or `migration` in this slice.
- Changing mode approval rules, runtime state layout, artifact publish destinations, or evidence bundle structure.
- Replacing critique-first artifacts with industry-standard shapes; that remains a separate roadmap direction.
- Introducing new Canon modes or widening this slice into Domain Modeling and Boundary Design.

**Invariants**:

- `requirements`, `discovery`, and `change` MUST remain critique-first and evidence-backed; authored-body preservation cannot remove or bypass existing artifact, critique, or evidence behavior.
- When authored content is missing, Canon MUST say so explicitly with `## Missing Authored Body` and must name the missing canonical heading; it MUST NOT fabricate plausible-looking artifact sections.
- Canonical authored headings are exact-match contracts unless an alias is explicitly documented in the mode contract; near-match headings are treated as missing.
- Existing delivered reference modes (`backlog`, `architecture`, `pr-review`) MUST keep their current behavior unchanged.
- Run identity, gate evaluation, publish destinations, and persistence layout MUST remain unchanged for the updated modes.

**Decision Traceability**: Decisions for this feature will be recorded in `specs/016-mode-authoring-specialization/decision-log.md`, with validation evidence captured in `specs/016-mode-authoring-specialization/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Authoring Contract Is Explicit Before Run (Priority: P1)

A Canon user wants the mode skill, template, and example to state exactly which authored H2 sections must exist before invoking `requirements`, `discovery`, or `change`, so they can author a real packet instead of guessing what Canon expects.

**Why this priority**: If the authoring contract stays implicit, the renderer and runtime fixes remain undiscoverable and users will keep submitting thin briefs that degrade into placeholders.

**Independent Test**: Read the updated skill text, template, and example for one first-slice mode and confirm the required authored H2 sections are explicit, consistent, and sufficient to prepare a valid authored packet without reading source code.

**Acceptance Scenarios**:

1. **Given** a user opens the updated `requirements` skill and template, **When** they prepare a brief, **Then** they can identify every required authored H2 section for the emitted artifacts.
2. **Given** a user opens the updated `discovery` example, **When** they compare it with the skill guidance, **Then** the example uses the same authored section contract and demonstrates a realistic non-placeholder packet.

---

### User Story 2 - Renderer Preserves Authored Sections Honestly (Priority: P2)

A reviewer wants emitted artifacts for `requirements`, `discovery`, and `change` to preserve authored sections verbatim when they exist, and to surface explicit missing-body blocks when they do not, so Canon stops producing generic filler that looks complete.

**Why this priority**: This is the core behavioral change. Without renderer honesty, the documentation and skill changes do not materially improve the runtime output.

**Independent Test**: Run each updated mode with a complete authored brief and confirm the emitted artifacts preserve the authored body verbatim; run the mode again with an incomplete brief and confirm the affected artifact emits `## Missing Authored Body` instead of fabricated prose.

**Acceptance Scenarios**:

1. **Given** an authored `change` brief that includes all required H2 sections, **When** the run completes, **Then** the emitted `change` artifacts preserve those authored sections verbatim.
2. **Given** an authored `requirements` brief that omits one required artifact body section, **When** the run completes, **Then** the affected artifact contains an explicit `## Missing Authored Body` block naming the missing canonical heading.
3. **Given** an authored `discovery` brief that uses non-canonical heading variants, **When** the run completes, **Then** Canon treats the section as missing rather than silently accepting or rewriting the variant.

---

### User Story 3 - Maintainers Can Review and Ship the Slice Safely (Priority: P3)

A maintainer wants roadmap, docs, examples, and tests to reflect the delivered first slice honestly, so the new behavior is auditable, reviewable, and safe to extend later to the remaining modes.

**Why this priority**: This feature changes authoring expectations across three modes; without docs, roadmap updates, and test evidence, later slices will drift or duplicate work.

**Independent Test**: Read the updated roadmap and mode guidance, then run the targeted test suites and confirm the repository documents the first-slice scope, the tests cover skills/docs/runtime behavior, and the remaining roadmap scope is still explicit.

**Acceptance Scenarios**:

1. **Given** a maintainer reads the roadmap after the first slice lands, **When** they inspect the relevant entry, **Then** they can see what was delivered now and what remains deferred to later slices.
2. **Given** a maintainer runs the targeted validation for the first slice, **When** the tests pass, **Then** they have evidence covering skill guidance, docs/templates/examples, renderer behavior, and end-to-end runs for the updated modes.

### Edge Cases

- A brief provides near-match headings such as `## Decision Options` instead of the canonical authored heading the renderer expects.
- A brief provides some required authored sections but leaves one section blank or whitespace-only.
- An updated mode still depends on summary-derived prose for some legacy sections; the authored-body changes must not break those sections while the first slice stays bounded.
- Templates, examples, and skill guidance for one mode drift out of sync with each other even if the runtime passes.
- A targeted mode already has partial authored-body handling in one artifact but not the rest of the packet; the first slice must normalize the full packet contract instead of leaving mixed behavior.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `requirements`, `discovery`, and `change` MUST each define an explicit authored-body contract that names the canonical H2 sections users must author for each emitted artifact.
- **FR-002**: The corresponding mode skills MUST include an `Author <Mode> Body Before Invoking Canon` section that enumerates those required authored H2 sections.
- **FR-003**: The materialized `.agents/skills/` copies for the updated modes MUST remain synchronized with the embedded skill sources.
- **FR-004**: The markdown renderer MUST preserve authored body sections verbatim for the updated modes whenever the canonical authored heading is present and non-empty.
- **FR-005**: When a required authored body section is absent or empty, the renderer MUST emit an explicit `## Missing Authored Body` block that names the missing canonical heading rather than generating generic filler.
- **FR-006**: The relevant mode orchestrator paths MUST pass authored brief content through to the renderer so the renderer can operate on the real authored body.
- **FR-007**: Each updated mode MUST ship a starter template under `docs/templates/canon-input/` that lists the required authored H2 sections for that mode's packet.
- **FR-008**: Each updated mode MUST ship a realistic example under `docs/examples/canon-input/` that exercises the required authored sections and can act as a credible starting packet.
- **FR-009**: `docs/guides/modes.md` MUST be updated so the new authored-body contract is discoverable without reading code.
- **FR-010**: The roadmap MUST be updated to record the delivered first slice honestly and preserve the remaining scope for later slices.
- **FR-011**: Focused contract, renderer, run, and docs tests MUST exist for the updated modes, and existing non-target modes MUST continue to pass unchanged.
- **FR-012**: The first slice MUST NOT modify the runtime behavior of `architecture`, `backlog`, or `pr-review` beyond necessary non-functional non-behavioral references in docs or roadmap text.

### Key Entities *(include if feature involves data)*

- **Mode Authored-Body Contract**: The mapping between an emitted artifact and the canonical authored H2 section or sections that must exist in the user-provided brief.
- **Missing Authored Body Marker**: The explicit honesty block emitted when a required authored artifact section is absent or empty.
- **Mode Authoring Template**: A starter brief that enumerates the required H2 sections for a mode and teaches the authoring contract.
- **Mode Authoring Example**: A realistic brief that demonstrates a fully authored packet for a mode and can be used to verify the contract end to end.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each first-slice mode (`requirements`, `discovery`, `change`) has one focused renderer test that proves authored sections are preserved verbatim.
- **SC-002**: Each first-slice mode has one focused negative test proving a missing or empty authored section emits `## Missing Authored Body`.
- **SC-003**: Each first-slice mode has synchronized skill guidance, template, and example that enumerate the same authored section contract.
- **SC-004**: The targeted validation suite for this feature passes without regressing existing reference-mode tests.
- **SC-005**: The roadmap and mode guidance describe the delivered first slice and remaining roadmap scope without implying that all remaining modes are already specialized.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: Focused contract, renderer, run, and docs tests for `requirements`, `discovery`, and `change`, plus non-regression coverage for already-delivered reference modes and existing affected tests.
- **Independent validation**: A separate review pass over `spec.md`, `plan.md`, and `tasks.md` after `/speckit.tasks`, followed by a temp-repo or isolated-workspace walkthrough using at least one updated example packet per updated mode and one derived negative fixture per mode created by removing a required H2 section from the updated example.
- **Evidence artifacts**: Results and findings recorded in `specs/016-mode-authoring-specialization/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Keep the first slice bounded to `requirements`, `discovery`, and `change`, **Rationale**: these modes are still generic enough to benefit materially from specialization while keeping risk bounded and leaving reference modes untouched.
- **D-002**: Use `## Missing Authored Body` as the honesty fallback instead of generic filler, **Rationale**: explicit incompleteness is safer than plausible-looking fabricated content.
- **D-003**: Treat `architecture`, `backlog`, and `pr-review` as delivered reference implementations rather than widening their scope again here, **Rationale**: avoids regressions and keeps the first slice additive.

## Non-Goals

- Updating all remaining Canon modes in this slice.
- Replacing critique-first packets with new industry-standard artifact shapes.
- Changing approval gates, publish destinations, or runtime persistence.
- Introducing new modes or broadening this feature into domain-modeling work.

## Assumptions

- The current authored-body patterns in `backlog`, `architecture`, and `pr-review` are sufficient references for the first slice.
- The updated modes already have enough artifact structure that specializing their authored-body contracts is additive rather than architectural.
- Existing skill validation scripts will remain valid unless the new skill sections introduce validator-sensitive wording; when validator logic changes, both the shell and PowerShell validators must stay logically equivalent even if only the shell script is executed in local validation on macOS.
- The repository will continue to use repo-local docs templates and examples as the primary discoverability path for authored mode inputs.
