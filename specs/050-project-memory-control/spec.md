# Feature Specification: Project Memory And Delivery Control Contracts

**Feature Branch**: `050-project-memory-control`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: User description: "Stabilize Canon owned project-memory and delivery-control contracts, promotion policy clarifications, producer-neutral managed blocks, and compatibility rules for Boundline consumers without changing the Canon orchestration boundary."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because this slice clarifies and expands
the canonical cross-repo contract surface while preserving Canon as the
governed producer and keeping delivery orchestration outside Canon  
**Scope In**:

- Canon-owned canonical contract publication under a stable documentation path
- feature-local contract briefs for project memory, promotion events, governed
  stage refs, and evidence refs
- producer-neutral managed block format for repo-visible project memory and
  evidence documents
- default target mapping for `docs/project/` and `docs/evidence/`
- minimum required V1 lineage fields and optional metadata set
- stronger compatibility policy for additive versus breaking contract changes
- explicit authorship rules for Canon-produced versus Boundline-produced
  repo-visible evidence blocks

**Scope Out**:

- Boundline workflow registry design or planner logic
- Boundline project-index semantics and cluster-topology modeling
- Backstage or TechDocs integration
- provider-runtime readiness or voting implementation
- turning Canon into a delivery orchestrator

**Invariants**:

- `.canon/` remains the authoritative governed runtime and evidence store.
- Canon owns producer-side contract semantics, promotion policy, lineage
  generation, and publish behavior.
- Consumers may rely on Canon contracts but may not redefine Canon promotion
  semantics.
- Shared repo-visible docs may include Boundline-produced evidence blocks, but
  that does not transfer contract ownership away from Canon.
- Large work remains decomposed into bounded stages and work units rather than
  unbounded autonomy.

**Decision Traceability**: Canon publishes the stable contract under
`docs/integration/`, while the detailed feature-local contract set for this
slice lives under `specs/050-project-memory-control/contracts/`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Canonical Contract Bundle (Priority: P1)

As a Canon maintainer, I want one stable Canon-owned contract bundle for project
memory and delivery-control integration so Boundline and future consumers can
pin a single source of truth instead of reverse-engineering producer behavior
from runtime code or stale feature notes.

**Why this priority**: Until the owner-side contract is stable and discoverable,
every downstream implementation risks drift or duplicate ownership.

**Independent Test**: Inspect the stable contract path and the feature-local
contract bundle and verify that a consumer can identify the owner, supported
contract line, required fields, and out-of-scope behavior without reading Canon
source code.

**Acceptance Scenarios**:

1. **Given** the accepted Canon contract bundle, **When** a maintainer reads
   `docs/integration/project-memory-promotion-contract.md`, **Then** they can
   identify the owner, canonical managed-block format, compatibility policy,
   and required lineage fields from that stable path.
2. **Given** the feature-local contract set, **When** a maintainer compares it
   with the stable Canon docs path, **Then** the required producer semantics,
   field names, and ownership boundaries remain aligned.
3. **Given** a downstream consumer that needs governed stage refs, promotion
   events, or evidence refs, **When** it reads the feature-local contracts,
   **Then** it can find those shapes without Canon specifying consumer-side
   orchestration behavior.

---

### User Story 2 - Define Safe Repo-Visible Publication Surfaces (Priority: P1)

As a Canon operator, I want the contract to state which repo-visible targets can
be updated, when stable managed blocks are allowed, and when proposal or
pending-only outputs are required, so curated project docs stay readable without
unsafe overwrites.

**Why this priority**: Repo-visible project memory is only credible if stable
knowledge, pending knowledge, and evidence-only output remain distinguishable.

**Independent Test**: Walk through completed, approval-gated, blocked, and
conflicting promotion scenarios and verify that the contract always yields an
explicit target and update strategy without implying Canon owns all authorship
inside `docs/evidence/`.

**Acceptance Scenarios**:

1. **Given** completed and approved stable knowledge, **When** Canon promotes
   it, **Then** the contract directs Canon to a stable target under
   `docs/project/` or `docs/evidence/` using producer-neutral managed blocks.
2. **Given** a pending, blocked, or conflicting promotion candidate, **When**
   Canon evaluates it, **Then** the contract directs Canon to proposal files,
   pending indexes, or evidence-only targets instead of overwriting stable
   project memory.
3. **Given** a repo-visible evidence document that contains Canon and Boundline
   contributions, **When** a reviewer inspects its managed blocks, **Then**
  each block declares its producer and source ref, and only Canon-owned
  blocks can define promotion policy fields such as `promotion_state`,
  `approval_state`, `packet_readiness`, or target-routing semantics.

---

### User Story 3 - Freeze Minimum V1 Lineage And Compatibility (Priority: P2)

As a consumer integrator, I want a lean required lineage set and a strong
compatibility policy so a V1 contract is implementable now and future breaking
changes are explicit instead of being smuggled in as ambiguous field drift.

**Why this priority**: The control layer is not implementable if every
promotion requires heavyweight metadata or if the compatibility rules remain
soft.

**Independent Test**: Inspect the contract and verify that required versus
optional lineage fields are clearly separated and that additive versus breaking
changes have different documented handling.

**Acceptance Scenarios**:

1. **Given** a V1 promoted block or document, **When** a consumer inspects its
   lineage, **Then** it can always recover `contract_version`, `producer`,
   `source_ref`, `source_artifacts`, `promotion_state`, `promoted_at`, and
   `content_digest`.
2. **Given** an additive V1-compatible contract revision, **When** a consumer
   encounters extra optional fields, **Then** the contract allows the consumer
   to ignore them without reinterpretation of required semantics.
3. **Given** a breaking contract change, **When** Canon publishes it, **Then**
   the contract requires a new major contract line rather than reusing the V1
   semantics silently.

### Edge Cases

- A stable project-memory document contains curated human-authored content
  around multiple managed blocks from different producers.
- Canon can publish evidence for a run, but the stable project-memory target is
  not eligible because approval or readiness is missing.
- A consumer receives a V1-compatible document with omitted optional fields and
  must not treat the omission as a breaking change.
- A future producer wants to write repo-visible evidence blocks without taking
  ownership of Canon promotion policy or Canon-managed content.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST publish the canonical project-memory and
  delivery-control contract bundle under a stable Canon documentation path.
- **FR-002**: Canon MUST mirror the stable contract semantics in feature-local
  contract documents for this slice so design, planning, and implementation
  work stay versioned with the feature.
- **FR-003**: Canon MUST define default repo-visible targets under
  `docs/project/` and `docs/evidence/` and map Canon-owned promotion semantics
  to those targets.
- **FR-004**: Canon MUST define a producer-neutral managed block format using a
  `project-memory:managed` marker family that includes `producer`,
  `source_ref`, and `contract_version`.
- **FR-005**: Canon MUST allow Canon and Boundline to contribute blocks inside
  `docs/evidence/` through the shared managed-block format while preserving
  Canon ownership of the contract and Canon-owned promotion rules; Boundline
  blocks may contribute evidence text and attribution metadata but MUST NOT
  redefine Canon promotion-state, approval, readiness, or target-routing
  semantics.
- **FR-006**: Canon MUST define the required V1 lineage fields as
  `contract_version`, `producer`, `source_ref`, `source_artifacts`,
  `promotion_state`, `promoted_at`, and `content_digest`.
- **FR-007**: Canon MUST define `mode`, `stage`, `owner`, `risk`, `zone`,
  `approval_state`, `packet_readiness`, and `promotion_profile` as optional V1
  lineage fields rather than required ones.
- **FR-008**: Canon MUST document when stable managed-block updates are allowed
  and when proposal files, pending indexes, or evidence-only outputs are
  required.
- **FR-009**: Canon MUST define shared contract shapes for governed stage refs,
  promotion events, and evidence refs as Canon-owned consumer-facing contracts.
- **FR-010**: Canon MUST define compatibility rules such that additive V1
  contract changes remain backward-compatible, removing or renaming required
  fields is breaking, and breaking changes require a new major contract line.
- **FR-011**: Canon MUST document support for the previous minor published
  contract revision for at least one full minor release cycle.
- **FR-012**: Canon MUST preserve `.canon/` as the authoritative governed
  runtime and evidence store and MUST treat repo-visible docs as projections of
  governed state.
- **FR-013**: Canon MUST NOT define Boundline workflow-registry semantics,
  project-index semantics, assurance-profile behavior, or consumer stop
  precedence in this slice.
- **FR-014**: Canon MUST remain the governed producer and MUST NOT become the
  delivery orchestrator.

### Key Entities *(include if feature involves data)*

- **Contract Bundle**: The Canon-owned stable and feature-local contract set for
  project memory, evidence refs, promotion events, and governed stage refs.
- **Managed Block**: A producer-neutral repo-visible block bounded by explicit
  markers and source metadata so generated content can be refreshed safely.
- **Lineage Record**: The metadata envelope that lets consumers recover source,
  promotion state, and integrity details from a promoted block or document.
- **Promotion Target**: A stable, pending, proposal, or evidence-only repo path
  selected by Canon policy.
- **Evidence Ref**: A contract shape that links readable repo-visible evidence
  back to Canon runs or Boundline traces without changing producer ownership.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A consumer can determine owner, supported contract line,
  required lineage fields, and breaking-change rules from the stable Canon docs
  path in under 10 minutes.
- **SC-002**: The contract bundle leaves zero unresolved clarification markers
  around target mapping, managed-block authorship, or minimum V1 lineage.
- **SC-003**: Reviewers can distinguish required versus optional V1 lineage
  fields and Canon-managed versus Boundline-managed evidence blocks from the
  published contract text alone.
- **SC-004**: Breaking versus additive contract changes are documented clearly
  enough that a Boundline consumer can decide whether to proceed, warn, or stop
  without reading Canon runtime code.

## Validation Plan *(mandatory)*

- **Structural validation**: compare the stable Canon contract path and the
  feature-local contracts for field-name, marker-format, and ownership
  alignment.
- **Logical validation**: walk through completed, approval-gated, blocked,
  conflicting, and mixed-producer evidence scenarios against the contract text.
- **Independent validation**: confirm a Boundline maintainer can consume the
  contract bundle without finding Canon-side orchestration behavior or missing
  producer semantics.
- **Evidence artifacts**: this `spec.md`, the feature-local contracts, the
  stable `docs/integration/` contract doc, and later planning and validation
  artifacts under the same feature folder.

## Decision Log *(mandatory)*

- **D-001**: Keep the canonical consumer-discovery path under
  `docs/integration/`, **Rationale**: Canon already uses that stable surface and
  consumers should not be forced to hunt through numbered specs for the current
  contract.
- **D-002**: Use producer-neutral managed blocks rather than Canon-specific
  markers, **Rationale**: repo-visible evidence can be contributed by Canon and
  Boundline while contract ownership still stays centralized in Canon.
- **D-003**: Keep V1 lineage required fields intentionally small, **Rationale**:
  the first control-layer implementation needs a credible minimum contract more
  than exhaustive metadata.

## Non-Goals

- Turning Canon into the project or delivery orchestrator.
- Defining Boundline workflow registries, project indexes, or assurance
  profiles.
- Shipping Backstage, TechDocs, or lifecycle-hook engines in this slice.
- Replacing `.canon/` runtime storage with repo-visible documentation.

## Assumptions

- Boundline is the first consumer and will pin the Canon-owned contract line
  rather than vendoring a second source-of-truth contract.
- `docs/project/` and `docs/evidence/` remain the default repo-visible targets
  for V1, while any future overrides can be layered later without changing
  producer ownership.
- Consumers need readable contracts and examples more urgently than they need a
  fully generalized publication platform.
- The stable Canon docs path and the feature-local contracts will be updated in
  the same feature slice to avoid drift during implementation.
