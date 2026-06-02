# Feature Specification: Governed Reasoning Posture v2

**Feature Branch**: `065-reasoning-posture-v2`

**Created**: 2026-06-02

**Status**: Draft

**Input**: User description: "Create a hard, contract-line-evolving redesign of Canon's governed reasoning posture producer contract that introduces a new Canon-owned contract line, strengthens the producer shape with typed subcontracts, defines fail-closed migration and incompatibility rules, publishes machine-checkable examples, and aligns the release on Canon 0.64.0 with the required validation and documentation updates."

## Governance Context

**Execution Mode**: architecture

**Risk Classification**: systemic-impact because this feature introduces a new
Canon-owned stable contract line consumed by Boundline and changes the semantic
shape, rejection behavior, compatibility surface, and migration boundary of a
governed cross-repository protocol.

**Scope Boundaries**:

- In scope: a successor stable reasoning-posture contract under
  `governed_reasoning_posture_v2`; typed producer subcontracts for profile
  selection, independence, confidence handoff, provenance, and compatibility;
  explicit coexistence, migration, incompatibility, and rejection rules between
  `v1` and `v2`; machine-checkable valid and invalid examples; executable
  contract validation for the stable doc, feature-local contract brief,
  vocabulary, compatibility windows, migration rules, and release-facing
  metadata; Canon `0.64.0` release alignment; README, CHANGELOG, and impacted
  `tech-docs` updates required to publish the new contract truthfully.
- Out of scope: Canon-owned execution loops for debate, reflexion,
  self-consistency, or other reasoning runtimes; Canon-owned participant
  routing, provider selection, runtime prompt orchestration, confidence
  synthesis, trace emission, or final acceptance authority; hidden changes to
  Boundline runtime behavior disguised as contract evolution; any silent
  reinterpretation of `governed_reasoning_posture_v1`.

**Invariants**:

- Canon remains the semantic owner of reasoning posture authoring,
  compatibility, provenance requirements, and contract-line evolution.
- Boundline remains the runtime owner of activation, routing, provider choice,
  runtime prompting, confidence synthesis, trace emission, and final acceptance
  authority.
- `governed_reasoning_posture_v1` keeps its current meaning and cannot be
  broadened, weakened, or retrofitted to carry `v2` semantics.
- Unsupported contract lines, incomplete payloads, contradictory selector data,
  invalid provenance, incomplete confidence handoff data, impossible or
  contradictory independence requirements, and incompatible version windows
  fail closed.
- There is no implicit fallback from `v2` to `v1`, from stronger selector
  semantics to weaker selector semantics, or from strict independence
  requirements to advisory prose.
- Machine-checkable examples and executable validation are part of the contract
  surface, not optional explanatory material.
- Any modified Rust source file in the implementing change set must retain at
  least 95% coverage, and lint validation must complete with no unresolved
  Clippy findings.

**Decision Traceability Expectations**: The stable Canon contract remains the
normative source, while the feature-local contract brief mirrors it for branch
planning and review. This specification, its follow-on design artifacts, the
stable contract document, and the validation report must make the semantic
delta from `v1` explicit enough that later work does not rely on chat-only
context.

## Clarifications

### Session 2026-06-02

- Q: In `governed_reasoning_posture_v2`, how should profile selection work? → A: `v2` requires exactly one selector kind per payload: either `required_profile_family` or `required_profile_id`, never both; if both are present, or neither is present, validation fails closed.
- Q: What coexistence rule should `governed_reasoning_posture_v2` enforce for `v1` and `v2` publication? → A: A release may publish both `v1` and `v2` only when exactly one line is marked active and the other is explicitly marked legacy; there is no implicit fallback, and consumers must reject mixed or ambiguous posture inputs.
- Q: In `governed_reasoning_posture_v2`, how should confidence handoff be modeled? → A: Every `v2` payload must include a typed `confidence_handoff` block with an explicit state such as `none` or `required`; absence is invalid, `required` must carry its required fields, validation rules, evidence or provenance references, and fail-closed behavior, and `none` must still be present explicitly.
- Q: In `governed_reasoning_posture_v2`, how should provenance be modeled? → A: Every `v2` payload must include a typed `provenance` block with an explicit state and reference-kind contract; absence fails validation, provenance cannot be inferred only from contract line or release metadata, and when `confidence_handoff.state = "required"`, provenance must provide the evidence or provenance references needed to validate that handoff.
- Q: In `governed_reasoning_posture_v2`, how should independence be modeled? → A: Every `v2` payload must include a typed `minimum_independence` block with required hard-minimum fields and a separate optional guidance sub-block; hard minima are mandatory validation requirements, guidance is advisory only and cannot weaken or replace the minima, and absent, contradictory, impossible, or overriding independence data fails validation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish A Typed v2 Contract (Priority: P1)

As a Boundline maintainer, I want Canon to publish a typed
`governed_reasoning_posture_v2` contract so that I can implement consumer
validation from repository artifacts alone without reverse-engineering Canon
source code.

**Why this priority**: This is the core value of the feature. If the successor
contract still depends on consumer inference, the redesign has not solved the
actual integration problem.

**Independent Test**: Review the stable Canon contract, the feature-local
contract brief, and the machine-checkable examples, then validate that a
consumer can determine required fields, selector resolution, independence
obligations, confidence handoff rules, provenance rules, and fail-closed
behavior without reading Canon implementation code.

**Acceptance Scenarios**:

1. **Given** the published `governed_reasoning_posture_v2` contract, **When** a
   maintainer reads the contract and examples, **Then** they can identify the
   required typed subcontracts, supported vocabulary, selector conflict rules,
   and rejection behavior from repository artifacts alone.
2. **Given** a valid `governed_reasoning_posture_v2` payload, **When** contract
   validation runs, **Then** the payload is accepted and the validation result
   states why it is valid.
3. **Given** a payload that names both a profile family and a profile id with
   contradictory meaning, **When** contract validation runs, **Then** the
   payload is rejected with an explicit selector-conflict reason.

---

### User Story 2 - Fail Closed On Drift And Malformed Data (Priority: P2)

As a cross-repo integrator, I want malformed posture data, stale release
metadata, and incompatible compatibility claims to be rejected before runtime
execution begins so that drift cannot pass as a valid integration state.

**Why this priority**: A stronger contract that still tolerates contradictory or
stale signals would remain operationally ambiguous and unsafe.

**Independent Test**: Run the executable contract validation against the stable
doc, feature-local brief, release-facing metadata, and the invalid example set,
then confirm that each invalid case fails with the expected rejection reason.

**Acceptance Scenarios**:

1. **Given** a payload with incomplete or invalid confidence handoff and
   provenance data, **When** validation runs, **Then** the payload is rejected
   before it is treated as consumable posture input.
2. **Given** release metadata that claims support Canon and Boundline windows
   the contract payload itself does not justify, **When** validation runs,
   **Then** the release is rejected as contract drift.
3. **Given** unsupported vocabulary, contradictory typed data, or omitted
   required fields, **When** validation runs, **Then** each case fails closed
   with a specific rejection reason.

---

### User Story 3 - Governed Migration Between v1 And v2 (Priority: P3)

As a migration owner, I want explicit coexistence and incompatibility rules
between `governed_reasoning_posture_v1` and `governed_reasoning_posture_v2` so
that I know when a workspace may remain on `v1`, when it must move to `v2`, and
when mixed inputs must be rejected.

**Why this priority**: Contract evolution without a clear migration boundary is
just another form of ambiguity. Consumers need deterministic rules for mixed and
transitional ecosystems.

**Independent Test**: Review the migration rules and run the coexistence and
migration-rejection examples to confirm that the allowed and rejected cases are
machine-checkable and unambiguous.

**Acceptance Scenarios**:

1. **Given** a consumer that only supports `governed_reasoning_posture_v1`,
   **When** it is presented with a `v2` payload, **Then** the input is rejected
   rather than downgraded or reinterpreted.
2. **Given** a release that publishes both `v1` legacy context and `v2` active
   context, **When** the coexistence rules are reviewed, **Then** the release
   makes the active line, legacy line, and allowed interpretation boundary
   explicit.
3. **Given** a release publishes both lines without naming exactly one active
  line and one legacy line, **When** migration validation runs, **Then** the
  release is rejected as ambiguous dual-line publication.
4. **Given** a workflow that requires `v2`, **When** it receives only `v1`
   posture data, **Then** the migration policy rejects the input with an
   explicit version-line reason.

---

### User Story 4 - Publish The New Contract Truthfully (Priority: P4)

As a release reviewer, I want Canon `0.64.0` release surfaces and user-facing
documentation to describe the new contract line truthfully so that I can verify
what changed from `v1`, what remained invariant, and what Canon still does not
own.

**Why this priority**: Publishing a new contract line without aligned release
surfaces would create immediate cross-repo drift and weaken trust in the new
boundary.

**Independent Test**: Review the stable contract, release-facing metadata,
README, CHANGELOG, and impacted `tech-docs`, then confirm that a reviewer can
identify the semantic delta from `v1`, the unchanged Canon/Boundline ownership
boundary, and the Canon `0.64.0` release alignment in under 10 minutes.

**Acceptance Scenarios**:

1. **Given** the release-facing artifacts for Canon `0.64.0`, **When** a
   reviewer inspects them, **Then** they clearly identify `v2` as the new
   contract line and do not describe it as a minor extension of `v1`.
2. **Given** the updated documentation set, **When** a reviewer compares it to
   the contract, **Then** the semantic delta from `v1`, the migration boundary,
   and the unchanged runtime ownership split remain consistent.
3. **Given** a release candidate with stale README, CHANGELOG, or impacted
   `tech-docs`, **When** release validation runs, **Then** the candidate is not
   considered ready.

### Edge Cases

- A payload publishes both `required_profile_family` and `required_profile_id`,
  and the two selectors point to incompatible consumer obligations.
- A payload publishes both selector kinds, or neither selector kind, and claims
  to be a valid `governed_reasoning_posture_v2` posture payload.
- A payload declares confidence handoff semantics without the provenance and
  evidence required to justify that handoff.
- A `governed_reasoning_posture_v2` payload omits the `confidence_handoff`
  block entirely.
- A payload sets `confidence_handoff.state = "required"` but omits the
  required handoff fields, validation rules, or evidence and provenance
  references needed to justify the handoff.
- A payload sets `confidence_handoff.state = "none"` but still carries fields
  that imply an active handoff obligation.
- A `governed_reasoning_posture_v2` payload omits the `provenance` block
  entirely.
- A payload includes `provenance`, but the block omits its explicit state or
  reference-kind contract.
- A payload sets `confidence_handoff.state = "required"`, but the provenance
  block is missing, invalid, or incompatible with the evidence needed to
  validate that handoff.
- A payload satisfies field presence but weakens the published meaning of the
  contract by encoding advisory semantics where `v2` requires hard rejection.
- A `governed_reasoning_posture_v2` payload is presented to a consumer that only
  supports `v1`.
- A `governed_reasoning_posture_v1` payload is presented to a workflow that
  requires `v2`.
- A release publishes both `v1` and `v2` but marks neither line as legacy, or
  marks both lines as active.
- A payload or release surface mixes `v1` and `v2` semantics in a way that
  prevents the consumer from determining one unambiguous active contract line.
- A `governed_reasoning_posture_v2` payload omits the `minimum_independence`
  block entirely.
- Independence requirements are partially present, internally contradictory, or
  impossible to satisfy.
- A `minimum_independence.guidance` block attempts to weaken, replace, or
  override a declared hard-minimum independence requirement.
- Additive optional fields attempt to smuggle in weaker fallback semantics that
  contradict the declared contract line.
- Release metadata claims support that the contract payload and examples do not
  justify.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST publish a new stable contract line,
  `governed_reasoning_posture_v2`, rather than stretching
  `governed_reasoning_posture_v1` to carry materially stronger semantics.
- **FR-002**: Canon MUST explicitly document why `v1` is insufficient and why
  `v2` is intentional protocol evolution rather than optional refinement.
- **FR-003**: Canon MUST preserve the existing meaning of
  `governed_reasoning_posture_v1` and MUST NOT silently reinterpret `v1` data as
  `v2` data.
- **FR-004**: Canon MUST define whether `v1` and `v2` may coexist, the exact
  conditions under which coexistence is allowed, and the cases that mixed-line
  inputs must reject.
- **FR-005**: When Canon publishes both `v1` and `v2` in the same release, it
  MUST mark exactly one contract line as active and the other as legacy.
- **FR-006**: `governed_reasoning_posture_v2` MUST define a typed profile
  requirement model that makes selector kind, selector value, conflict handling,
  and consumer obligation explicit.
- **FR-007**: Every `governed_reasoning_posture_v2` payload MUST declare exactly
  one selector kind: either `required_profile_family` or
  `required_profile_id`.
- **FR-008**: `governed_reasoning_posture_v2` MUST fail closed when both
  selector kinds are present or when neither selector kind is present; it MUST
  NOT define a precedence rule.
- **FR-009**: `governed_reasoning_posture_v2` MUST define a typed independence
  requirement model that separates hard minima from optional guidance without
  turning Canon into a runtime router.
- **FR-010**: Every `governed_reasoning_posture_v2` payload MUST include a
  typed `minimum_independence` block.
- **FR-011**: The `minimum_independence` block MUST include required
  hard-minimum fields and MAY include a separate optional guidance sub-block.
- **FR-012**: Hard-minimum independence fields are mandatory validation
  requirements; guidance is advisory only and MUST NOT weaken, replace, or
  override the hard minima.
- **FR-013**: `governed_reasoning_posture_v2` MUST reject absent, partial,
  contradictory, or impossible independence requirements.
- **FR-014**: `governed_reasoning_posture_v2` MUST replace the lone
  `confidence_handoff_required` boolean with a structured confidence-handoff
  contract.
- **FR-015**: Every `governed_reasoning_posture_v2` payload MUST include a
  typed `confidence_handoff` block.
- **FR-016**: The `confidence_handoff` block MUST carry an explicit state such
  as `none` or `required`; omission of the block or omission of its state MUST
  fail closed.
- **FR-017**: When `confidence_handoff.state` is `required`, the block MUST
  include the required handoff fields, validation rules, evidence or provenance
  references, and explicit fail-closed behavior.
- **FR-018**: When `confidence_handoff.state` is `none`, the block MUST still
  be present and MUST explicitly state that no confidence handoff is required
  for the posture.
- **FR-019**: The confidence-handoff contract MUST define what Canon is
  asserting, what evidence backs the assertion, what the consumer must do with
  the handoff, and what rejection behavior applies when the contract is
  incomplete or invalid.
- **FR-020**: `governed_reasoning_posture_v2` MUST replace opaque provenance
  signaling with a typed provenance contract.
- **FR-021**: Every `governed_reasoning_posture_v2` payload MUST include a
  typed `provenance` block.
- **FR-022**: The `provenance` block MUST carry an explicit state and a stable
  reference-kind contract; omission of the block or omission of its state or
  reference kind MUST fail closed.
- **FR-023**: Provenance MUST NOT be inferred only from the contract line or
  release metadata.
- **FR-024**: Even minimal provenance MUST be explicit, machine-checkable, and
  tied to stable reference kinds published by the contract.
- **FR-025**: When `confidence_handoff.state` is `required`, the provenance
  block MUST provide the evidence or provenance references needed to validate
  that handoff; missing, invalid, or incompatible provenance MUST fail closed.
- **FR-026**: The provenance contract MUST define supported reference kinds,
  source and evidence expectations, and rejection behavior for missing, invalid,
  stale, or contradictory provenance.
- **FR-027**: `governed_reasoning_posture_v2` MUST define a compatibility-window
  contract that makes supported version ranges, incompatibility, and rejection
  behavior explicit.
- **FR-028**: Unsupported contract lines, omitted required fields, unsupported
  vocabulary, contradictory typed data, invalid migration claims, invalid
  provenance, incomplete confidence handoff data, and stale release metadata
  MUST fail closed.
- **FR-029**: Canon MUST publish concrete machine-checkable examples covering at
  least one valid `governed_reasoning_posture_v2` payload, invalid profile
  selector cases, invalid independence cases, invalid confidence handoff cases,
  invalid provenance cases, invalid compatibility-window cases, a `v1`/`v2`
  coexistence case, and a migration-rejection case.
- **FR-030**: Every published example MUST state the expected validation result
  and the expected acceptance or rejection reason.
- **FR-031**: Canon MUST keep the stable integration contract as the normative
  source and MUST keep the feature-local contract brief aligned as a review and
  planning mirror.
- **FR-032**: Executable contract validation MUST verify alignment between the
  stable contract, the feature-local contract brief, the supported vocabulary,
  the compatibility windows, the migration rules, and the release-facing
  metadata the downstream validation path depends on.
- **FR-033**: Canon MUST define migration expectations for `v1`-only consumers,
  `v2`-capable consumers, and mixed ecosystems so that each state has explicit
  acceptance and rejection rules.
- **FR-034**: Canon MUST define rejection behavior for a `v2` payload presented
  to a `v1` consumer and for a `v1` payload presented to a workflow that
  requires `v2`.
- **FR-035**: Canon MUST update README, CHANGELOG, and any impacted `tech-docs`
  whenever the new contract line changes release-facing or user-facing meaning.
- **FR-036**: Canon MUST align the published contract, executable validation,
  and release-facing metadata to Canon `0.64.0` for this feature.
- **FR-037**: Any modified Rust source file in the implementation of this
  feature MUST demonstrate at least 95% coverage.
- **FR-038**: The implementation of this feature MUST complete Clippy validation
  with no unresolved warnings for the touched code.
- **FR-039**: Canon MUST NOT gain ownership of downstream activation, routing,
  provider choice, runtime prompting, confidence synthesis, trace emission, or
  final acceptance authority as a side effect of publishing `v2`.

### Key Entities *(include if feature involves data)*

- **Governed Reasoning Posture Contract v2**: The Canon-owned stable contract
  line that defines the successor producer boundary, required typed subcontracts,
  supported vocabulary, compatibility rules, and fail-closed semantics.
- **Profile Requirement Contract**: The typed selector surface that states
  whether the consumer is being asked to satisfy a profile family or an exact
  profile id, including the rule that exactly one selector kind must be present
  per payload, with no precedence fallback.
- **Independence Requirement Contract**: The typed posture surface that states
  the hard minimum independence constraints, the optional guidance sub-block,
  and the rule that guidance cannot weaken or override the hard minima, without
  assigning runtime routes or providers.
- **Confidence Handoff Contract**: The structured producer assertion describing
  whether Canon is handing a confidence requirement to the consumer, the
  evidence that supports it, the explicit `state` for every payload, and the
  rejection rules when the handoff is incomplete, invalid, or omitted.
- **Provenance Contract**: The typed reference surface that identifies the
  source or evidence Canon is citing for the posture, the explicit state and
  reference-kind contract for every payload, and the rules for valid, invalid,
  stale, omitted, or contradictory provenance.
- **Compatibility And Migration Contract**: The set of rules that define the
  supported version windows, the relationship between `v1` and `v2`, allowed
  coexistence, and migration rejection behavior.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In validation recorded for this feature, a maintainer can
  implement `governed_reasoning_posture_v2` consumer validation from repository
  artifacts alone, without reading Canon implementation code, in under 60
  minutes.
- **SC-002**: 100% of the published machine-checkable examples produce the
  expected validation result and the expected acceptance or rejection reason.
- **SC-003**: 100% of validation runs for the invalid example set reject omitted
  required fields, contradictory selector data, invalid provenance, incomplete
  confidence handoff data, unsupported vocabulary, invalid migration claims, and
  stale release metadata.
- **SC-004**: In release review evidence, a reviewer can identify in under 10
  minutes what changed semantically from `v1`, what remained invariant, and why
  Canon still does not own downstream runtime orchestration.
- **SC-005**: For the implementing change set, all modified Rust source files
  meet the 95% minimum coverage target and all Clippy validation for touched
  code completes without unresolved warnings.

## Assumptions

- Canon `0.64.0` is the target release line for publishing the new contract.
- Boundline will remain the runtime owner of execution behavior; the contract
  redesign clarifies producer semantics rather than moving orchestration into
  Canon.
- During migration, `v1` may remain as a legacy supported line only when the
  release explicitly names it as legacy and does not allow implicit
  reinterpretation of `v1` payloads under `v2` rules.
- Consumer validation is allowed to depend on the stable contract, the
  feature-local contract brief, the machine-checkable examples, executable
  contract tests, and the release-facing metadata that the documented validation
  path reads.