# Feature Specification: Requirements PRD Publishing And Chat Publish Skill

**Feature Branch**: `041-prd-publish-chat`  
**Created**: 2026-05-07  
**Status**: Draft  
**Input**: User description: "Improve Canon requirements publishing by emitting consolidated prd markdown artifacts, adding a chat-first publish skill for Copilot/Codex, clarifying publish UX in docs, and covering the new behavior with tests, lint-clean changes, and a version bump."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because the change touches publishable artifact contracts, repo-local AI skills, and release-facing documentation, but it does not alter approval policy, execution modes, or destructive runtime behavior.  
**Scope In**: requirements artifact generation and publish surfaces, repo-local publish skill authoring for chat-first usage, documentation and examples for PRD and publish flows, release/version metadata, and validation coverage for the touched surfaces.  
**Scope Out**: new approval behavior, non-requirements consolidated packets beyond light documentation touch-ups, automatic diagram rendering, remote publish destinations, and broader redesign of Canon mode semantics.

**Invariants**:

- Requirements publish MUST continue to emit the existing sectional packet files and packet metadata; any new PRD artifact is additive rather than a replacement.
- Publish MUST remain gated by existing run completion rules and MUST NOT bypass approval, critique, or publish destination boundaries.

**Decision Traceability**: Decisions and tradeoffs will be recorded in `specs/041-prd-publish-chat/spec.md`, `specs/041-prd-publish-chat/plan.md`, and `specs/041-prd-publish-chat/decision-log.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish A Readable PRD Packet (Priority: P1)

As a product or architecture user running `requirements`, I want publish output to include at least one consolidated PRD markdown document so I can read and share a single packet without reconstructing it from fragmented files.

**Why this priority**: The missing consolidated PRD is the direct usability gap that makes successful runs feel empty even when artifacts were generated correctly.

**Independent Test**: Run a completed `requirements` packet and publish it to the default or override destination, then verify the output includes the existing section files plus one or more readable `prd*.md` artifacts that combine the authoritative content.

**Acceptance Scenarios**:

1. **Given** a completed requirements run with the canonical authored brief, **When** Canon publishes the run, **Then** the destination contains the existing sectional packet files and a consolidated `prd.md` built from the same packet content.
2. **Given** a completed requirements run published to an override path, **When** publish succeeds, **Then** the override path receives the consolidated PRD artifact alongside the existing packet metadata and section files.

---

### User Story 2 - Invoke Publish From Chat (Priority: P2)

As a chat-first Canon user working through repo-local skills, I want an explicit publish skill so I can drive the publish step from Copilot or Codex without having to infer the raw CLI contract from memory.

**Why this priority**: Chat users already rely on repo-local skills for governed mode entry; leaving publish as an undocumented manual command breaks the workflow at the final step.

**Independent Test**: Initialize Canon skills, inspect the repo-local skill set, and confirm there is a publish-oriented skill with instructions that route the agent to `canon publish <RUN_ID>` and explain destination behavior.

**Acceptance Scenarios**:

1. **Given** a repository initialized with Copilot or Codex skills, **When** a user asks to publish a completed run from chat, **Then** the available skill guidance explicitly covers the publish command, required run identifier, and optional destination override.
2. **Given** a run that is not publishable yet, **When** a user follows the chat publish guidance, **Then** the surfaced behavior still respects the existing CLI error and does not imply that chat can bypass governance gates.

---

### User Story 3 - Understand The Publish UX Up Front (Priority: P3)

As a new Canon user, I want the docs and release notes to describe where generated packet files live before and after publish so I do not mistake a successful run for an empty result.

**Why this priority**: The runtime is working, but the mental model is weak; better docs reduce avoidable support friction and make the new PRD artifact discoverable.

**Independent Test**: Read the updated README and mode guidance and confirm they explicitly distinguish `.canon/artifacts` from published destinations while naming the consolidated requirements PRD artifact.

**Acceptance Scenarios**:

1. **Given** the updated docs, **When** a user reads the requirements or publish flow, **Then** the docs explain that generated artifacts first land under `.canon/` and only appear in `docs/` or `specs/` after publish.
2. **Given** the updated docs and version bump, **When** release-facing surfaces are reviewed, **Then** the new PRD artifact and chat publish workflow are discoverable without reading source code.

### Edge Cases

- What happens when a completed requirements run is missing one or more optional authored sections and the consolidated PRD must preserve the same missing-body honesty markers as the sectional artifacts?
- How does the system handle publish to a custom destination without regressing the default `specs/<date>-requirements` leaf naming?
- Which invariant is most likely to be stressed by this case? The additive-artifact invariant is stressed most by any attempt to replace or rename existing published requirement files.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST extend the `requirements` artifact contract and renderer so a completed requirements packet can produce at least one consolidated PRD markdown artifact in addition to the existing sectional files.
- **FR-002**: The consolidated PRD artifact MUST be derived from the same authoritative requirements packet content used for the sectional artifacts and MUST preserve missing-body honesty instead of inventing filler.
- **FR-003**: The publish flow MUST copy the consolidated PRD artifact to the default publish destination and to any explicit `--to` override destination without changing current destination semantics.
- **FR-004**: The system MUST keep the existing sectional requirements files and `packet-metadata.json` publish behavior intact so downstream users relying on file-level outputs are not broken.
- **FR-005**: The repository MUST ship a chat-first publish skill under `.agents/skills/` that explains how to publish completed runs from Copilot or Codex, including run-id input, default destination behavior, and override-path usage.
- **FR-006**: Documentation MUST explicitly distinguish generated runtime artifacts under `.canon/artifacts` from published packet destinations and MUST mention the consolidated requirements PRD output.
- **FR-007**: Release/version metadata MUST be updated to reflect the shipped feature once implementation and validation are complete.
- **FR-008**: The implementation MUST add automated coverage for consolidated requirements PRD generation, publish output, and chat publish skill validation behavior.

### Key Entities *(include if feature involves data)*

- **Requirements Packet Publication**: The publishable set of requirements artifacts, including sectional markdown files, the consolidated PRD markdown, and packet metadata.
- **Chat Publish Skill**: The repo-local skill package that maps a chat publish request onto the Canon CLI publish contract without changing runtime governance semantics.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A completed requirements run publishes at least one consolidated `prd.md` artifact in 100% of automated publish tests covering default and override destinations.
- **SC-002**: The published requirements directory still contains the pre-existing sectional files and packet metadata in 100% of compatibility tests after the change.
- **SC-003**: Repo-local skill validation passes with the new publish skill present and no broken skill references.
- **SC-004**: README or mode guidance contains an explicit statement of the `.canon/artifacts` versus published-destination distinction and names the consolidated PRD output.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and repo skill validation scripts.
- **Logical validation**: Focused Rust tests for requirements rendering and publish output, plus any targeted skill or documentation checks needed for the new surfaces.
- **Independent validation**: Review the published packet and chat skill wording from the user perspective to confirm the PRD is discoverable and publish remains clearly gated.
- **Evidence artifacts**: `specs/041-prd-publish-chat/validation-report.md`, test output captured during implementation, and updated publish-facing docs.

## Decision Log *(mandatory)*

- **D-001**: Canon will add a consolidated requirements PRD as an additive artifact rather than replacing the existing file set, **Rationale**: users need a readable single document, but downstream tooling or reviewers may still depend on the sectional packet files.

## Non-Goals

- Introduce consolidated PRD artifacts for every Canon mode in this slice.
- Add Mermaid or image rendering pipelines to published packets as part of this feature.

## Assumptions

- Requirements remains the highest-value starting point for consolidated PRD output because it already maps cleanly onto product-facing packet sections.
- A repo-local skill is the appropriate chat-first surface for publish because Canon already exposes run lifecycle steps through skills instead of adding chat-only runtime APIs.
- Existing publish semantics, including completed-run checks and destination override behavior, remain correct and should be preserved.
- The version bump for this slice can follow the repository's normal release-line practice without changing the broader release process.
