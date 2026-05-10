# Feature Specification: Standard ADR Publish Artifacts

**Feature Branch**: `043-standard-adr-publish`  
**Created**: 2026-05-10  
**Status**: Draft  
**Input**: User description: "Add standard Nygard ADR support by making architecture the canonical ADR-producing mode, adding optional ADR export for change and migration during publish, keeping incident and non-decision modes out of ADR publication, bumping the Canon version first, and requiring final validation to include at least 95% coverage on all new or modified Rust files plus clean clippy and cargo fmt."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact, because this slice changes publish-time decision artifacts, documentation, and mode contracts for decision-carrying packets without changing approval semantics, destructive execution behavior, or the `.canon/` runtime model.  
**Scope In**:

- Add one standard Nygard-style ADR markdown artifact for `architecture` publishes by default.
- Add optional ADR export during publish for `change` and `migration` when the operator explicitly requests that the packet enter the durable ADR register.
- Define the ADR registry path, numbering rules, filename shape, section contract, and source-packet traceability.
- Update contracts, renderers, publish behavior, documentation, skills, templates, and focused tests for the supported modes.
- Bump Canon version surfaces before the implementation tasks that depend on the new behavior land.
- Close the feature with focused coverage work so every new or modified source file touched by this slice reaches at least 95% line coverage.

**Scope Out**:

- Introducing a new dedicated `adr` mode.
- Replacing the current runtime packet artifacts or `.canon/` evidence with ADR files as the system of record.
- Publishing ADR entries from `incident`, `requirements`, `discovery`, `review`, `verification`, `pr-review`, `implementation`, `refactor`, `backlog`, or `system-shaping` in this slice.
- Building a full ADR lifecycle management system for manual status transitions, bulk supersession workflows, or repository-wide ADR editing commands.
- Changing approval gates, run-state transitions, evidence persistence, or publish destinations outside the new ADR register.

**Invariants**:

- Runtime packet artifacts under `.canon/` and the existing publish outputs MUST remain authoritative; ADR files are a publish-time projection, not a replacement runtime model.
- `architecture` MUST be the canonical ADR-producing mode, while `change` and `migration` MUST remain opt-in for ADR export rather than always writing into the registry.
- Generated ADR content MUST stay critique-first and evidence-backed; missing or weak authored decision content cannot be replaced with fabricated context, alternatives, or consequences.
- Unsupported modes, especially `incident` and non-decision flows, MUST continue to publish without creating ADR entries.
- Existing approval behavior, run identity, evidence linking, and packet readiness semantics MUST remain unchanged.

**Decision Traceability**: Design and implementation decisions for this slice will be recorded in `specs/043-standard-adr-publish/decision-log.md`, with validation evidence and closeout notes recorded in `specs/043-standard-adr-publish/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish A Standard Architecture ADR (Priority: P1)

As an architect or reviewer, I want `architecture` publishes to emit one repository-local ADR in the standard Nygard shape so the accepted architectural decision is easy to find and understand without re-reading the entire packet.

**Why this priority**: This is the main value of the feature. If `architecture` does not become the canonical ADR-producing mode, the slice does not actually establish a durable standard ADR workflow.

**Independent Test**: Run a publishable `architecture` packet in a repository with no existing ADR register and verify that publish creates exactly one numbered ADR file under `docs/adr/` containing the required standard sections and links back to the packet artifacts.

**Acceptance Scenarios**:

1. **Given** a publishable `architecture` run with decision, constraints, and consequences recorded, **When** the operator publishes the packet, **Then** Canon writes one numbered ADR file under `docs/adr/` with title, date, status, context, decision, and consequences.
2. **Given** a repository with no prior ADR files, **When** the first `architecture` ADR is published, **Then** Canon creates the registry path and assigns the first non-conflicting ADR number using the canonical filename shape.

---

### User Story 2 - Opt Durable Change Or Migration Decisions Into The ADR Register (Priority: P2)

As a maintainer, I want `change` and `migration` publishes to support explicit ADR export only when the packet represents a durable repository decision, so the ADR register stays meaningful instead of absorbing every tactical packet by default.

**Why this priority**: The repository already carries strong decision records in `change` and `migration`, but not every packet should automatically become a durable ADR. Opt-in export preserves signal quality.

**Independent Test**: Publish one `change` packet and one `migration` packet with and without ADR export enabled and verify that the ADR register changes only for the opt-in cases.

**Acceptance Scenarios**:

1. **Given** a publishable `change` or `migration` run, **When** the operator publishes without ADR export enabled, **Then** Canon preserves the normal publish outputs and creates no new ADR file.
2. **Given** a publishable `change` or `migration` run, **When** the operator publishes with ADR export enabled, **Then** Canon writes one numbered ADR file in the same standard shape and keeps traceability back to the original packet.

---

### User Story 3 - Keep ADR Publication Honest, Bounded, And Documented (Priority: P3)

As a maintainer of Canon itself, I want the supported modes, numbering rules, status behavior, and unsupported-mode boundaries to be documented and regression-tested so the ADR register stays coherent and does not drift into a hidden side system.

**Why this priority**: Without consistent docs, contracts, and tests, the ADR register will either drift from the packet model or silently expand into unsupported modes.

**Independent Test**: Run the focused regression suite across supported and unsupported modes, inspect the generated docs and skills, and confirm that only the supported modes can create ADR files and that the documented behavior matches the implementation.

**Acceptance Scenarios**:

1. **Given** a publishable `incident` or other unsupported-mode packet, **When** the operator publishes it, **Then** Canon produces the normal packet outputs and does not create an ADR file.
2. **Given** existing ADR files already present in `docs/adr/`, **When** a new supported ADR export is published, **Then** Canon assigns a new non-conflicting number and leaves existing ADR files unchanged.

### Edge Cases

- `docs/adr/` does not exist yet and Canon must create the registry path without changing any unrelated publish destination.
- The registry already contains numbered ADR files and gaps in numbering; Canon must still assign the next non-conflicting identifier without overwriting prior records.
- A supported source packet is publishable but still carries explicit missing-body or missing-context markers in decision sections; the exported ADR must preserve honesty rather than inventing standard prose.
- A `change` or `migration` packet is published multiple times with and without ADR export; the opt-in boundary must remain explicit and predictable.
- An unsupported mode has decision-sounding text in its artifacts; Canon must still refuse ADR publication because the mode boundary, not keyword matching, controls eligibility.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `architecture` publish MUST generate one standard ADR markdown artifact by default for every publishable architecture packet.
- **FR-002**: Every generated ADR MUST include a unique ADR identifier, title, date, status, context, decision, and consequences in a stable Markdown shape aligned with the standard Nygard template.
- **FR-003**: The generated `architecture` ADR MUST derive its content from the existing architecture packet artifacts and include traceability back to the source packet and its publish location.
- **FR-004**: ADR files MUST be published under `docs/adr/` with a canonical numbered filename shape that preserves stable ordering and avoids collisions with existing ADR files.
- **FR-005**: ADR numbering MUST allocate the next non-conflicting identifier without mutating or renaming existing ADR files.
- **FR-006**: `change` and `migration` publish flows MUST support explicit opt-in ADR export and MUST NOT create ADR files when that export is not requested.
- **FR-007**: `incident` and all other unsupported modes MUST NOT create ADR files during publish.
- **FR-008**: Generated ADRs MUST preserve explicit missing-body, missing-context, or downgraded-evidence honesty from the source packet instead of fabricating canonical sections.
- **FR-009**: Existing runtime packet artifacts and normal publish outputs MUST remain present; ADR publication MUST be additive rather than replacing current publish behavior.
- **FR-010**: Documentation, examples, and skill guidance MUST explain the supported modes, default versus optional ADR export posture, registry path, numbering behavior, and unsupported-mode boundary.
- **FR-011**: Canon version surfaces MUST be bumped consistently as the first implementation task for this feature slice.
- **FR-012**: The final validation closeout for this feature MUST prove that every new or modified source file touched by the slice reaches at least 95% line coverage and that repository-required formatting and lint validation are clean.

### Key Entities *(include if feature involves data)*

- **ADR Registry Entry**: A durable Markdown document under `docs/adr/` containing the standard ADR sections plus traceability back to the Canon packet that produced it.
- **ADR Publish Policy**: The mode-specific rule set that decides whether ADR export is default, opt-in, or unsupported.
- **ADR Source Mapping**: The traceability data connecting a generated ADR to the source mode, source packet artifacts, and publish operation that created it.
- **ADR Identifier**: The sequential repository-local number and filename slug that give each ADR stable referenceability.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Publishing a valid `architecture` packet in a repository with no ADR history creates exactly one ADR file under `docs/adr/` containing all required standard sections.
- **SC-002**: Publishing valid `change` and `migration` packets without ADR export enabled creates zero ADR files, while publishing the same packet with ADR export enabled creates exactly one ADR file per publish operation.
- **SC-003**: Regression validation shows that unsupported modes create zero ADR files while preserving their existing publish outputs.
- **SC-004**: 100% of generated ADR files contain a unique non-conflicting identifier and an explicit trace back to the source Canon packet.
- **SC-005**: The final validation suite passes repository-required formatting, lint, and focused ADR publish regression validation.
- **SC-006**: Every new or modified source file touched by this feature achieves at least 95% line coverage in the final coverage closeout.

## Validation Plan *(mandatory)*

- **Structural validation**: Repository-required formatting and lint validation, focused contract validation, and any skill or doc validation scripts affected by the ADR publish surface.
- **Logical validation**: Focused publish, contract, renderer, and integration tests covering `architecture` default ADR export, `change` and `migration` opt-in ADR export, numbering behavior, unsupported-mode non-regression, and honesty preservation when source decision sections are weak or incomplete.
- **Independent validation**: Cross-artifact Speckit coherence analysis before implementation, then one manual readback comparing a generated ADR against its source packet to verify that the ADR explains the same decision without hidden chat context.
- **Evidence artifacts**: `specs/043-standard-adr-publish/validation-report.md`, generated coverage evidence for touched source files, and the focused regression outputs recorded during closeout.

## Decision Log *(mandatory)*

- **D-001**: Keep ADR generation as a publish-time projection rather than a new runtime mode, **Rationale**: Canon already has governed packet modes and should not fork a second decision workflow for the same work.
- **D-002**: Make `architecture` the canonical ADR-producing mode while keeping `change` and `migration` opt-in, **Rationale**: architecture decisions are durable by default while tactical changes should not automatically pollute the ADR register.
- **D-003**: Treat the initial ADR lifecycle state as publish-generated `Accepted` output unless later work adds richer lifecycle controls, **Rationale**: this keeps the first slice standard-shaped without expanding into full ADR management workflows.

## Non-Goals

- Introduce a new `adr` execution mode or parallel packet family.
- Add bulk ADR lifecycle editing, supersession orchestration, or manual status-management commands in this slice.
- Export ADRs from unsupported modes just because their artifacts mention decisions.
- Replace the existing architecture, change, or migration packet artifacts with ADR files.

## Assumptions

- `docs/adr/` is the correct repository-local destination for durable ADR publication in this project.
- The first implementation slice can treat publish-generated ADR status as `Accepted` while still remaining compatible with the standard ADR template.
- Existing `architecture`, `change`, and `migration` decision artifacts contain enough structured authored content to derive standard ADR sections without inventing new evidence.
- The requested version bump, coverage closeout, and required formatting or lint cleanups are part of this feature’s mandatory implementation definition of done rather than optional polish.
