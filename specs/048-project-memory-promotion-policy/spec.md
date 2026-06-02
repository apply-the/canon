# Feature Specification: Project Memory Promotion Policy

**Feature Branch**: `048-project-memory-promotion-policy`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: User description: "Establish a Canon-owned project-memory promotion contract and owner-side feature slice for publish profiles, promotion policy, lineage metadata, and non-destructive update strategies that Boundline can consume without turning Canon into the orchestrator."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because the slice introduces a new
cross-repo publication contract, project-visible promotion policy, and lineage
surface without changing Canon's role as the governed runtime or moving
delivery orchestration into Canon  
**Scope In**:

- project-memory publish profile definition
- Canon-owned promotion policy states and meanings
- Canon-owned lineage metadata for promoted outputs
- Canon-owned non-destructive update strategies for project-visible documents
- pending, audit, and evidence publication surfaces
- the feature-local shared contract brief under `contracts/`
- an explicit requirement to promote the accepted contract into a stable Canon
  documentation path such as `tech-docs/integration/project-memory-promotion-contract.md`
  or `tech-docs/contracts/project-memory-promotion-contract.md`

**Scope Out**:

- Boundline delivery-path logic
- Boundline stage-planner logic
- Boundline assurance-profile logic
- Boundline governed-stage orchestration
- Canon as a delivery orchestrator
- updating existing docs or code in this first pass

**Invariants**:

- `.canon/` remains the governed runtime and evidence storage surface.
- Project-visible output is a promoted projection of governed results, not a
  replacement for Canon runtime artifacts.
- Canon owns publish profiles, promotion policy, lineage, update strategies,
  and promotion states.
- Canon MUST NOT become the orchestrator for bounded delivery.
- Consumers MUST NOT redefine Canon promotion semantics.

**Decision Traceability**: The authoritative feature-local shared contract for
this slice is
`specs/048-project-memory-promotion-policy/contracts/boundline-project-memory-promotion-contract.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Promote Project Memory With Policy Boundaries (Priority: P1)

As a Canon operator publishing governed output, I want project-visible memory to
update only when Canon policy says the output is stable, approved, or clearly
marked pending, so visible project documents do not silently absorb unapproved
or blocked content as durable truth.

**Why this priority**: This is the core producer-side capability. Without an
explicit promotion policy, project memory becomes either a manual-only afterthought
or an unsafe blind overwrite surface.

**Independent Test**: Exercise representative completed, approval-gated, and
blocked runs against the `project-memory` publish profile and verify Canon
updates stable memory, pending surfaces, or evidence-only outputs according to
policy without changing orchestration ownership.

**Acceptance Scenarios**:

1. **Given** a completed or reusable project-shaping output that Canon policy
   marks `auto`, **When** the operator publishes with the `project-memory`
   profile, **Then** Canon updates the configured stable project-memory target
   and emits lineage metadata.
2. **Given** an architecture or other governed output that is still awaiting
   approval and Canon policy marks `auto-if-approved` or `pending-index`,
   **When** publish runs, **Then** Canon updates pending or audit surfaces and
   does not silently overwrite the stable project-memory target.
3. **Given** a blocked or evidence-bearing output that Canon policy marks
   `evidence-only` or `index-only`, **When** publish runs, **Then** Canon
   updates evidence or index surfaces only and preserves the stable
   project-memory surface unchanged.

---

### User Story 2 - Preserve Lineage And Curated Documents (Priority: P1)

As a reviewer of promoted project knowledge, I want every promoted document to
preserve source lineage and to use a non-destructive update strategy, so I can
trust where the content came from and avoid losing curated human-authored text.

**Why this priority**: Project-visible memory becomes dangerous if lineage is
lost or if Canon overwrites curated repository knowledge with no managed
boundary.

**Independent Test**: Publish representative packets into managed-block,
proposal-file, and append-only index targets and verify the emitted surfaces
preserve required lineage fields and human-authored content outside Canon-owned
ranges.

**Acceptance Scenarios**:

1. **Given** a managed project-memory target, **When** Canon promotes output,
   **Then** it updates only the Canon-managed range and preserves curated text
   outside that range.
2. **Given** a target Canon cannot safely rewrite in place, **When** promotion
   runs, **Then** Canon emits a proposal file or other configured non-destructive
   output rather than forcing an unsafe overwrite.
3. **Given** any promoted output, **When** a reviewer inspects it or its sidecar,
   **Then** they can recover `contract_version`, source run, mode, profile,
   promotion state, approval state, readiness, publish time, update strategy,
   and source artifacts.

---

### User Story 3 - Publish A Stable Consumer Contract (Priority: P2)

As a maintainer shipping Canon for Boundline consumption, I want a versioned
contract brief inside the feature slice and an explicit path to promote that
accepted contract into a stable documentation location, so Canon can own the
producer contract without leaving consumers tied forever to a feature-local
draft path.

**Why this priority**: The feature-local contract is correct for initial design,
but cross-repo consumers need an eventual stable path once the contract is
accepted.

**Independent Test**: Inspect the feature artifacts and verify the feature-local
contract brief includes explicit versioning and policy fields, while the owner-side
spec requires later promotion into a stable documentation path.

**Acceptance Scenarios**:

1. **Given** the feature-local contract brief, **When** a maintainer reads it,
   **Then** it includes `contract_version`, owner, known consumers,
  compatibility rules and pre-1.0 change policy.
2. **Given** the owner-side Canon spec, **When** implementation planning begins,
   **Then** the spec requires promotion of the accepted contract into a stable
   Canon documentation path such as `tech-docs/integration/...` or `tech-docs/contracts/...`.
3. **Given** a future breaking contract change, **When** Canon updates the
  contract, **Then** the documented pre-1.0 change policy governs the update
  rather than an undocumented ad hoc process.

### Edge Cases

- A packet is readable but still blocked, and policy allows only evidence or
  index publication.
- A curated project-memory document contains a Canon-managed range plus large
  human-authored context outside the range.
- A consumer sees a newer major `contract_version` than it supports.
- A promotion candidate cannot safely merge into the target stable document.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST define a `project-memory` publish profile for promoting
  governed output into project-visible knowledge surfaces.
- **FR-002**: Canon MUST own the promotion-state vocabulary and its semantics.
- **FR-003**: Canon MUST support policy states that distinguish stable
  project-memory promotion from pending, evidence-only, index-only, or manual
  publication outcomes.
- **FR-004**: Canon MUST preserve governed runtime artifacts under `.canon/`
  and MUST treat project-visible publication as a projection rather than the
  authoritative runtime record.
- **FR-005**: Canon MUST emit lineage metadata sufficient to recover
  `contract_version`, source run, mode, profile, promotion state, approval
  state, readiness, publish time, update strategy, and source artifacts.
- **FR-006**: Canon MUST own and document non-destructive update strategies for
  project-visible documents, including managed updates, proposal emission, and
  append-only index behavior where applicable.
- **FR-007**: Canon MUST preserve curated human-authored content outside
  Canon-managed ranges when using managed document updates.
- **FR-008**: Canon MUST define which outputs may update stable project memory,
  which update pending or audit surfaces, and which remain evidence-only.
- **FR-009**: Canon MUST publish the shared contract brief under the feature
  slice `contracts/` directory during this first pass.
- **FR-010**: Canon MUST require later promotion of the accepted shared contract
  into a stable Canon documentation path such as
  `tech-docs/integration/project-memory-promotion-contract.md` or
  `tech-docs/contracts/project-memory-promotion-contract.md`.
- **FR-011**: Canon MUST version the shared contract explicitly through
  `contract_version`, owner, known consumers, compatibility rules,
  and pre-1.0 change policy.
- **FR-012**: Canon MUST NOT specify, implement, or assume Boundline delivery
  paths, stage-planner logic, assurance-profile policy, or governed-stage
  orchestration behavior in this slice.
- **FR-013**: Canon MUST remain the governed producer and MUST NOT become the
  delivery orchestrator.

### Key Entities *(include if feature involves data)*

- **Project-Memory Publish Profile**: The Canon-owned profile that projects
  governed output into stable project-memory, evidence, or index surfaces.
- **Promotion State**: The Canon-owned policy outcome that determines whether a
  packet updates stable memory, pending surfaces, evidence-only surfaces, or
  requires manual handling.
- **Lineage Metadata**: The durable producer-owned metadata that lets consumers
  recover source identity and publication semantics from promoted output.
- **Update Strategy**: The Canon-owned mechanism for changing a project-visible
  document without destructive overwrite.
- **Stable Consumer Contract**: The versioned producer contract that starts as a
  feature-local contract brief and later moves to a stable Canon documentation
  path.

## Shared Contract Alignment *(mandatory)*

The Canon owner-side spec and the Boundline integration-side spec must remain
aligned on:

- stage taxonomy and mode mapping
- project-memory, evidence, and index target surfaces
- promotion-state vocabulary and meanings
- lineage metadata field names and meanings
- update-strategy vocabulary and meanings
- compatibility rules and pre-1.0 change policy

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Canon can express stable, pending, index-only, evidence-only, and
  manual promotion outcomes without requiring consumers to invent their own
  promotion semantics.
- **SC-002**: Every promoted output type covered by the slice has a documented
  lineage contract that preserves canonical run traceability.
- **SC-003**: The feature-local contract brief is sufficient for a consumer to
  determine ownership, versioning, compatibility, and change policy without
  consulting source code.
- **SC-004**: The owner-side spec explicitly requires later promotion of the
  accepted contract into a stable Canon documentation path.

## Validation Plan *(mandatory)*

- **Structural validation**: review the feature-local contract brief and
  owner-side spec for explicit ownership boundaries, version fields, and stable
  documentation-promotion requirements.
- **Logical validation**: verify acceptance scenarios for stable promotion,
  pending publication, evidence-only publication, lineage preservation, and
  non-destructive updates once implementation begins.
- **Independent validation**: confirm a Boundline maintainer could consume the
  contract without finding Canon orchestration behavior or missing producer-side
  semantics.
- **Evidence artifacts**: this `spec.md`, the feature-local contract brief, and
  later planning and validation artifacts created under the same feature folder.

## Decision Log *(mandatory)*

- **D-001**: Canon owns the shared contract because it owns producer-side
  publish, promotion, lineage, and document-update semantics, **Rationale**:
  consumers may rely on those semantics, but they must not define them.
- **D-002**: The first pass keeps the contract brief under the feature-local
  `contracts/` folder, **Rationale**: design work should remain versioned with
  the feature slice before the accepted contract is promoted to a stable docs path.

## Non-Goals

- Turning Canon into the project or delivery orchestrator.
- Specifying Boundline delivery-path or stage-planner behavior.
- Updating existing docs or implementation in this first artifact-writing pass.
- Replacing `.canon/` runtime storage with project-visible documentation.

## Assumptions

- Boundline is the initial known consumer of this contract, but the contract
  should be versioned as if additional consumers may exist later.
- The accepted contract will later need a stable Canon documentation home so
  consumers are not pinned forever to a feature-local draft path.
- Project-visible knowledge must be policy-driven rather than blindly automatic
  or permanently manual.