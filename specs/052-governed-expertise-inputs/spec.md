# Feature Specification: Governed Expertise Inputs

**Feature Branch**: `052-governed-expertise-inputs`  
**Created**: 2026-05-14  
**Status**: Draft  
**Input**: User description: "Create a Canon-owned contract for governed expertise inputs that Boundline may consume for expert-pack selection without turning Canon into a runtime selector, provider router, or delivery orchestrator."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because this slice clarifies and
stabilizes how Canon labels domain expertise outputs that downstream consumers
may reuse, without changing Canon orchestration or letting Canon choose runtime
roles  
**Scope In**:

- Canon-owned semantics for which governed outputs count as expertise inputs
- a stable way for consumers to classify those expertise inputs from Canon-owned
  mode and publication semantics
- compatibility and ownership rules for expertise-input evolution
- alignment between Canon integration documentation and source-level expertise
  classification

**Scope Out**:

- runtime role selection or expert-pack activation
- provider or model routing
- a Canon-managed runtime registry or plugin marketplace
- new publish destinations beyond existing project-memory and evidence-facing
  surfaces
- Boundline planning, orchestration, or inspection behavior

**Invariants**:

- Canon remains the semantic producer of governed expertise inputs.
- Boundline remains the owner of runtime role selection and expert-pack
  activation.
- Expertise inputs reuse Canon's existing publication and lineage semantics;
  this slice does not invent a second publication channel.
- Expertise inputs remain human-readable governed artifacts, not opaque runtime
  bundles.

**Decision Traceability**: Decisions for this slice are recorded in
`specs/052-governed-expertise-inputs/` and promoted Canon integration
documentation under `docs/integration/`, with the expertise-input contract kept
consistent with the existing project-memory promotion contract.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Canon-Owned Expertise Input Contract (Priority: P1)

As a Boundline maintainer, I want one Canon contract that says which governed
outputs count as expertise inputs and how to identify them, so I do not have to
reverse-engineer Canon packets or source code to consume governed domain
knowledge safely.

**Why this priority**: Without a stable contract, every downstream consumer
will infer different Canon semantics and cross-repo drift becomes likely.

**Independent Test**: A maintainer can read the stable Canon integration
contract and determine which outputs qualify as expertise inputs, which modes
produce them, and what metadata is required to classify them.

**Acceptance Scenarios**:

1. **Given** a Canon artifact produced from `domain-language` or
   `domain-model`, **When** a consumer inspects the documented contract and the
   published lineage, **Then** the consumer can determine that the artifact is
   a governed expertise input and which expertise kind it represents.
2. **Given** a Canon artifact produced from a non-expertise mode such as
   `review` or `verification`, **When** a consumer inspects the contract,
   **Then** the contract makes clear that the artifact is outside the governed
   expertise-input surface for this slice.
3. **Given** an expertise input with missing required classification metadata,
   **When** a consumer inspects it, **Then** the contract requires the consumer
   to treat it as unavailable rather than guessing its meaning.

---

### User Story 2 - Preserve The Canon And Boundline Ownership Boundary (Priority: P1)

As a Canon maintainer, I want expertise inputs to stop at governed knowledge
publication, so Canon can inform Boundline without drifting into runtime role
selection, pack activation, or provider-routing policy.

**Why this priority**: The cross-repo design fails if Canon begins carrying
runtime directives that belong to Boundline's delivery layer.

**Independent Test**: Compare the resulting contract against the new Boundline
selection spec and verify that Canon publishes expertise inputs only, while
Boundline remains the runtime selector.

**Acceptance Scenarios**:

1. **Given** the final expertise-input contract, **When** a maintainer reads
   the ownership section, **Then** it is explicit that Canon publishes
   expertise inputs but does not choose expert packs, runtime roles, or models.
2. **Given** a future proposal that adds runtime-role directives to Canon
   expertise artifacts, **When** it is evaluated against the contract,
   **Then** the proposal is clearly out of scope for this contract line.
3. **Given** an existing Canon publish flow, **When** expertise-input support is
   added, **Then** it continues to use Canon's existing publication and lineage
   semantics instead of introducing a new runtime-specific channel.

---

### User Story 3 - Ignore Unknown Or Incompatible Expertise Inputs Safely (Priority: P2)

As a cross-repo integrator, I want unsupported expertise kinds, missing
required metadata, or incompatible contract lines to fail closed, so older
consumers do not invent behavior from partially understood Canon output.

**Why this priority**: Cross-repo compatibility only stays safe when consumers
can ignore or reject unknown expertise inputs explicitly.

**Independent Test**: Present a consumer with a supported expertise input, an
unknown expertise kind, and a mismatched contract line, and verify that the
consumer can classify the supported case while rejecting the others without
reinterpreting Canon semantics.

**Acceptance Scenarios**:

1. **Given** an expertise input with a supported expertise kind and contract
   line, **When** a consumer reads it, **Then** the consumer can classify it
   without reading Canon source code.
2. **Given** an expertise input with an unknown expertise kind, **When** an
   older consumer reads it, **Then** the consumer may ignore the new kind while
   still respecting the rest of the stable lineage envelope.
3. **Given** an expertise input published under an unsupported contract line,
   **When** a consumer reads it, **Then** the consumer treats that input as
   unavailable and does not infer fallback semantics.

### Edge Cases

- A `domain-language` or `domain-model` artifact is published with pending,
  blocked, or evidence-only promotion state and remains visible but not stable.
- A non-expertise mode publishes an artifact family that resembles domain
  guidance, so the contract must still exclude it from the expertise-input
  surface.
- Two expertise inputs for the same subject disagree, so Canon must publish
  both traceably without trying to resolve Boundline runtime choice.
- A consumer sees valid lineage metadata but no supported expertise-kind
  classification, so the input must be treated as readable Canon output but not
  as a governed expertise input.
- A future additive expertise kind is introduced and older consumers must ignore
  it without treating the older contract line as broken.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST define a stable governed expertise-input surface for
  this contract line and limit the initial expertise kinds to outputs produced
  from the existing `domain-language` and `domain-model` modes.
- **FR-002**: Canon MUST provide a stable machine-readable expertise
  classification through an `expertise_input` metadata object that lets
  consumers determine whether a published artifact is a governed expertise
  input without inferring that fact from narrative prose.
- **FR-003**: Canon MUST require expertise inputs to carry enough lineage and
  mode information that a consumer can identify the governing mode, contract
  line, expertise kind, and applicable domain families.
- **FR-004**: Canon MUST document which publication surfaces and target classes
  expertise inputs may use under the existing project-memory promotion and
  indexing contracts.
- **FR-005**: Canon MUST preserve the rule that expertise inputs extend current
  Canon publication semantics rather than introducing a second runtime-specific
  publication channel.
- **FR-006**: Canon MUST allow consumers to treat absent, blocked, pending,
  incompatible, or unknown expertise inputs as unavailable without breaking the
  readability of the underlying Canon artifact.
- **FR-007**: Canon MUST document compatibility rules for adding new expertise
  kinds or additive expertise metadata without silently changing the meaning of
  existing kinds.
- **FR-008**: Canon MUST make explicit that expertise inputs do not encode
  runtime role selection, expert-pack activation, provider routing, or delivery
  control-flow decisions.
- **FR-009**: Canon MUST keep expertise inputs traceable to source packet
  lineage and readable to human maintainers.
- **FR-010**: Canon MUST keep the expertise-input contract aligned with a
  stable source-level classification surface so contract validation and source
  behavior do not drift apart.

### Key Entities *(include if feature involves data)*

- **Governed Expertise Input**: A Canon-published artifact whose stable
  classification says it is reusable domain expertise rather than a runtime
  directive.
- **Expertise Kind**: The Canon-owned category that names what kind of governed
  expertise an input carries for this contract line.
- **Expertise Classification Surface**: The stable machine-readable Canon-owned
  way consumers use to decide whether an artifact qualifies as a governed
  expertise input.
- **Expertise Input Lineage**: The packet lineage and publication metadata that
  keeps the expertise input traceable to its originating Canon run.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can determine which Canon outputs are valid governed
  expertise inputs and which are excluded in under 10 minutes using Canon
  integration documentation alone.
- **SC-002**: Consumers can classify `domain-language` and `domain-model`
  expertise inputs without reading Canon implementation code.
- **SC-003**: 100% of validation scenarios involving unsupported expertise
  kinds, missing required classification metadata, or unsupported contract lines
  fail closed instead of inventing fallback meaning.
- **SC-004**: The final contract makes the Canon and Boundline ownership
  boundary explicit without adding Canon runtime-selection responsibilities.

## Validation Plan *(mandatory)*

- **Structural validation**: Contract review against existing integration docs,
  source-level classification review, and metadata-shape inspection for
  expertise inputs.
- **Logical validation**: Walk through one supported expertise input, one
  excluded non-expertise artifact, one unknown expertise kind, one blocked or
  pending publication-state scenario, and one incompatible contract-line
  scenario.
- **Independent validation**: Review by a separate maintainer comparing the
  new expertise-input contract against the Boundline expert-pack-selection spec
  and existing Canon publication contracts.
- **Evidence artifacts**: Contract notes, classification tests, and review
  findings recorded under `specs/052-governed-expertise-inputs/`.

## Decision Log *(mandatory)*

- **D-001**: Initial expertise kinds are limited to `domain-language` and
  `domain-model`, **Rationale**: these are the existing Canon modes most clearly
  aligned with reusable governed expertise and least likely to leak runtime
  orchestration semantics.
- **D-002**: Expertise inputs extend existing project-memory promotion and
  indexing semantics, **Rationale**: a second publication channel would create
  duplicate Canon contracts for the same knowledge.
- **D-003**: Boundline remains the runtime selector, **Rationale**: Canon must
  stay artifact-first and must not become the runtime role engine.

## Non-Goals

- Selecting runtime roles, expert packs, or models for Boundline
- Creating a Canon runtime registry, plugin distribution system, or marketplace
- Replacing existing project-memory or evidence publication semantics
- Expanding the initial expertise-input surface beyond `domain-language` and
  `domain-model` in this contract line

## Assumptions

- Existing Canon publication contracts already provide enough lineage structure
  to anchor expertise-input semantics without inventing a new packet format.
- Boundline will treat Canon expertise inputs as optional inputs rather than a
  hard dependency for local runtime behavior.
- The initial value comes from stable classification and boundary-setting, not
  from publishing every possible Canon mode as an expertise input.
- Canon source-level classification can be kept aligned with the integration
  contract through focused code and test updates in this slice.
