# Feature Specification: Artifact Indexing Contract

**Feature Branch**: `051-artifact-indexing-contract`  
**Created**: 2026-05-14  
**Status**: Draft  
**Input**: User description: "Stabilize a Canon-owned artifact indexing contract for Boundline runtime indexing by unifying indexable metadata across published artifacts and evidence, clarifying or removing undefined safety-net packets, and versioning those fields without turning Canon into a runtime registry or orchestrator."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because this slice clarifies and
stabilizes Canon-owned producer metadata across existing published artifacts
without changing Canon orchestration or delegating runtime behavior to Canon  
**Scope In**:

- a Canon-owned indexing contract for repo-visible published artifacts and
  evidence blocks
- a unified minimum metadata shape for artifacts that Boundline may index
- clarification or removal of the undefined `safety-net packets` term
- compatibility and versioning rules for additive versus breaking metadata
  changes
- explicit ownership boundaries between Canon semantic contracts and Boundline
  runtime indexing

**Scope Out**:

- Canon runtime orchestration
- Canon-managed local indexes or registries
- Boundline context assembly or runtime state behavior
- Boundline selection, council, or reasoning policies
- new Canon publish destinations outside current repo-visible documentation
  surfaces

**Invariants**:

- Canon remains the semantic owner of published artifact metadata.
- Canon MUST NOT become a runtime registry or runtime orchestrator.
- Boundline may consume Canon metadata, but Canon MUST NOT define Boundline
  control flow.
- Existing published artifacts remain readable to human maintainers even when
  machine-readable indexing metadata is added or clarified.

**Decision Traceability**: Decisions for this slice are recorded in
`specs/051-artifact-indexing-contract/` and promoted Canon integration
documentation under `docs/integration/project-memory-promotion-contract.md`,
which remains the single normative stable contract surface for this slice.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Indexable Contract Surface (Priority: P1)

As a Canon maintainer, I want one stable contract surface that says which
published artifacts are indexable and which metadata they carry, so downstream
consumers do not reverse-engineer Canon output from scattered specs.

**Why this priority**: Without a stable contract, every downstream consumer
reconstructs Canon semantics differently and contract drift becomes likely.

**Independent Test**: A maintainer can read the stable Canon contract path and
determine which published artifacts are indexable, which metadata fields are
required, which are optional, and where that metadata lives for each supported
artifact class without reading Canon source code.

**Acceptance Scenarios**:

1. **Given** the Canon integration documentation path
  `docs/integration/project-memory-promotion-contract.md`, **When** a maintainer
  inspects the artifact indexing contract, **Then** they can identify the
  indexable artifact classes and the minimum required metadata for each.
2. **Given** an artifact published under an indexable Canon surface, **When** a
  consumer reads its documented metadata carrier, **Then** the consumer can
  recover owner, contract line, source reference, and artifact class without
  inferring those facts from surrounding prose.
3. **Given** an artifact outside the supported indexing contract,
   **When** a consumer inspects it, **Then** the contract makes clear that the
   artifact is not part of the stable indexing surface.

---

### User Story 2 - Clarify Safety-Net And Evidence Semantics (Priority: P1)

As a cross-repo integrator, I want ambiguous artifact names removed or defined,
especially `safety-net packets`, so Boundline does not implement a contract for
an artifact class that Canon never actually publishes.

**Why this priority**: Undefined artifact terms create false dependencies and
force downstream runtime behavior to guess at Canon semantics.

**Independent Test**: Compare the final contract against current Canon publish
surfaces and verify that every named artifact class maps to a real Canon output
or is explicitly rejected as unsupported vocabulary.

**Acceptance Scenarios**:

1. **Given** the final contract text, **When** a maintainer searches for
   `safety-net packets`, **Then** the term is either explicitly defined as a
   real published artifact class or removed from the stable contract vocabulary.
2. **Given** Canon evidence documents with managed blocks, **When** a consumer
   inspects those blocks, **Then** the producer, source reference, and artifact
   class are explicit and machine-readable.
3. **Given** a Canon artifact type that is intentionally not indexable,
   **When** a maintainer reads the contract, **Then** the exclusion is stated
   explicitly rather than implied by omission.

---

### User Story 3 - Version Metadata Without Expanding Canon Scope (Priority: P2)

As a Boundline maintainer, I want additive metadata changes to remain compatible
and breaking changes to become explicit new contract lines, so I can pin Canon
artifact indexing safely without Canon taking over runtime behavior.

**Why this priority**: Stable versioning reduces downstream integration cost and
prevents silent semantic breakage.

**Independent Test**: A maintainer can review the compatibility section and
determine whether an additive field, removed field, or renamed field is allowed
for the current contract line.

**Acceptance Scenarios**:

1. **Given** an additive metadata field on an already supported artifact,
   **When** a consumer reads the contract, **Then** the consumer can ignore the
   new optional field without reinterpretation of required semantics.
2. **Given** a removed or renamed required metadata field, **When** Canon
   proposes the change, **Then** the contract requires a new major line rather
   than silently reusing the old line.
3. **Given** the final contract, **When** a consumer reads the ownership
   section, **Then** it is clear that Canon owns artifact semantics but not
   Boundline runtime behavior.

### Edge Cases

- A human-curated artifact contains surrounding prose plus Canon managed blocks
  that must remain readable after metadata normalization.
- An existing Canon artifact is useful to humans but intentionally excluded from
  the stable indexing surface.
- A consumer encounters a supported artifact class with missing optional
  metadata but complete required metadata.
- A future Canon feature wants to add new metadata to evidence blocks without
  changing the current major contract line.
- A downstream consumer relies on the old ambiguous term while the new contract
  removes it.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST publish a stable artifact-indexing contract that names
  the repo-visible artifact classes Boundline or other consumers may index.
- **FR-002**: Canon MUST define the minimum required metadata fields for every
  artifact class in the stable indexing contract.
- **FR-003**: Canon MUST distinguish required metadata from optional metadata so
  additive fields can evolve without changing the meaning of the current
  contract line.
- **FR-004**: Canon MUST explicitly define or remove the term `safety-net
  packets` from the stable contract vocabulary.
- **FR-005**: Canon MUST define machine-readable producer attribution and source
  reference metadata for indexable evidence blocks.
- **FR-006**: Canon MUST map each supported artifact class to one normative
  metadata carrier and discovery rule, using the managed-block lineage envelope
  for repo-visible managed documents or `packet-metadata.json` sidecars for
  packet-style artifact publications unless a future contract line documents a
  different carrier explicitly.
- **FR-007**: Canon MUST state which artifact classes are outside the stable
  indexing surface instead of leaving exclusion implicit.
- **FR-008**: Canon MUST document compatibility rules for additive, removed,
  and renamed metadata fields.
- **FR-009**: Canon MUST require a new major contract line for any change that
  removes or renames required metadata or changes the meaning of an existing
  required field.
- **FR-010**: Canon MUST preserve the boundary that artifact indexing metadata
  is semantic producer output, not Boundline runtime policy.
- **FR-011**: Canon MUST NOT define local runtime indexing, context assembly,
  council activation, or stop-semantics behavior in this slice.

### Key Entities *(include if feature involves data)*

- **Artifact Class**: A named family of Canon-published repo-visible outputs
  that may be indexed by consumers.
- **Artifact Indexing Metadata**: The minimum semantic metadata attached to an
  indexable Canon artifact or evidence block.
- **Evidence Managed Block**: A readable evidence block with explicit producer
  and source-reference metadata.
- **Metadata Carrier**: The normative location where a consumer discovers
  indexing metadata for an artifact class, such as a managed-block lineage
  envelope or `packet-metadata.json` sidecar.
- **Contract Line**: The major compatibility line that defines whether a
  consumer can safely parse Canon artifact metadata.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can determine the indexable Canon artifact classes
  and their required metadata from the stable contract documentation in under 10
  minutes.
- **SC-002**: The stable contract leaves zero ambiguous artifact names in the
  supported indexing vocabulary.
- **SC-003**: A consumer can distinguish additive-compatible metadata from
  breaking metadata changes without reading Canon implementation code.
- **SC-004**: The contract clarifies producer semantics without adding Canon
  runtime-orchestration responsibilities.

## Validation Plan *(mandatory)*

- **Structural validation**: Markdown contract review, schema-shape review for
  required versus optional fields, and compatibility-rule inspection.
- **Logical validation**: Walk through at least one supported artifact class,
  one excluded artifact class, one additive-change scenario, and one
  breaking-change scenario.
- **Independent validation**: Review by a separate maintainer comparing the new
  contract text against existing Canon artifact-producing specs.
- **Evidence artifacts**: Contract examples, comparison notes, and review
  findings recorded under `specs/051-artifact-indexing-contract/`.

## Decision Log *(mandatory)*

- **D-001**: Canon remains a semantic producer only, **Rationale**: the
  indexing contract must reduce ambiguity without turning Canon into a runtime
  registry.
- **D-002**: Artifact indexing versioning follows additive-compatible optional
  fields plus major-line breaks for required-field changes, **Rationale**: this
  gives consumers a stable pinning strategy.
- **D-003**: Undefined artifact vocabulary must be explicitly resolved, not
  tolerated, **Rationale**: ambiguous names create fake downstream
  dependencies.

## Non-Goals

- Defining Boundline runtime indexes or context packs
- Defining Boundline council or selection behavior
- Adding a Canon runtime registry or local indexing store
- Introducing new repo-visible publish destinations beyond current Canon
  documentation and evidence surfaces

## Assumptions

- Existing Canon artifact-producing specs already describe enough concrete
  artifact families that this slice can unify them without inventing new
  runtime behavior.
- Boundline will remain able to function without Canon, and this contract is
  optional enrichment rather than a hard runtime dependency.
- `docs/integration/project-memory-promotion-contract.md` remains the stable
  consumer-facing contract path for this slice, with 051 extending that
  contract rather than creating a second normative document.
