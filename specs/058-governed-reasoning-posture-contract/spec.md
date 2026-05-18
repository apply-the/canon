# Feature Specification: Governed Reasoning Posture Contract

**Feature Branch**: `058-governed-reasoning-posture-contract`  
**Created**: 2026-05-18  
**Status**: Implemented  
**Input**: User description: "Create the Canon feature spec aligned to the governed reasoning posture contract branch and separate the contract scope from the gatekeeper refactor"

## Governance Context *(mandatory)*

**Mode**: architecture  
**Risk Classification**: systemic-impact, because this slice defines Canon-owned reasoning-posture semantics and compatibility windows that directly gate downstream Boundline reasoning activation and release-pair alignment.  
**Scope In**: Canon-owned `governed_reasoning_posture_v1` contract identity, producer shape, compatibility windows, supported vocabulary, executable contract checks, release-surface alignment needed by downstream consumers, and the bounded gatekeeper maintainability follow-through required to keep the touched runtime surface reviewable without changing policy meaning.  
**Scope Out**: Boundline runtime activation, participant routing, trace emission, confidence synthesis, or operator-facing delivery logic; Canon-owned execution loops for debate, reflexion, or self-consistency; and any new gate policy semantics beyond behavior-preserving code organization.

**Invariants**:

- Canon remains the producer and semantic owner of governed reasoning posture, while Boundline remains the runtime owner of reasoning activation, participants, confidence, and trace behavior.
- Unsupported contract lines, incomplete posture payloads, or incompatible release windows MUST fail closed rather than degrade into guessed compatibility.
- The gatekeeper refactor in this branch MUST preserve gate evaluation behavior, approval semantics, and material blocker meaning.
- This slice MUST NOT use the reasoning-posture contract as cover for introducing new Canon runtime ownership over downstream orchestration.

**Decision Traceability**: Decisions for this feature are recorded in `specs/058-governed-reasoning-posture-contract/decision-log.md`, with Canon-owned stable semantics published from `docs/integration/governed-reasoning-posture-contract.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Stable Reasoning Posture Contract (Priority: P1)

As a Boundline maintainer, I want Canon to publish one stable governed reasoning posture contract, so downstream reasoning activation can rely on explicit producer fields and compatibility windows instead of reverse-engineering Canon state.

**Why this priority**: Without the stable Canon contract, Boundline has no trustworthy producer boundary for reasoning posture and every downstream consumer is forced to infer semantics from implementation details.

**Independent Test**: Review the stable Canon contract and the paired executable contract test and confirm that a maintainer can identify the contract line, required fields, supported vocabulary, and supported release pair without reading Canon implementation code.

**Acceptance Scenarios**:

1. **Given** the stable Canon reasoning posture contract, **When** a maintainer reads it, **Then** they can identify the contract line, required fields, supported vocabulary, and supported Boundline and Canon compatibility windows.
2. **Given** the paired Boundline consumer brief, **When** the Canon contract is compared against it, **Then** the two repositories agree on the same contract line, required fields, and active compatibility window.
3. **Given** a proposal to extend the reasoning posture shape, **When** it changes required fields or vocabulary meaning, **Then** the feature requires an explicit contract-line update instead of silently repurposing `governed_reasoning_posture_v1`.

---

### User Story 2 - Fail Closed On Drift And Version Mismatch (Priority: P2)

As a cross-repo integrator, I want Canon contract drift and release-surface drift to be executable and visible, so stale manifests, stale compatibility metadata, and incompatible posture inputs are caught before downstream execution begins.

**Why this priority**: A stable doc alone is not enough. The contract must stay aligned with workspace versioning, package metadata, and the Boundline consumer boundary or the integration will rot silently.

**Independent Test**: Run the Canon reasoning-posture contract test and the release-surface validation that checks workspace version, plugin metadata, and runtime-compatibility references, and confirm they fail when the branch drifts.

**Acceptance Scenarios**:

1. **Given** a stale Canon plugin manifest or runtime-compatibility reference, **When** validation runs, **Then** the drift is reported before the branch is considered aligned.
2. **Given** an unsupported contract line or incompatible Boundline and Canon window, **When** the contract test runs, **Then** the branch fails closed instead of accepting the mismatch.
3. **Given** a valid Canon release surface and a matching Boundline consumer brief, **When** validation runs, **Then** the integration reports the contract line and release pair as aligned.

---

### User Story 3 - Preserve Gatekeeper Behavior While Restoring Maintainability (Priority: P3)

As a Canon maintainer, I want the oversized gatekeeper surface in this branch decomposed into bounded sibling modules without changing gate behavior, so contract-adjacent runtime checks stay reviewable and future policy work can remain scoped.

**Why this priority**: The branch already touches gate evaluation code. If that surface stays monolithic, review of the reasoning-posture work becomes harder and future gate work will continue to accumulate avoidable complexity.

**Independent Test**: Run the gatekeeper-focused tests and confirm the public gate evaluation entrypoints, statuses, and representative blocker conditions remain unchanged after the module split.

**Acceptance Scenarios**:

1. **Given** the gatekeeper module split, **When** Canon evaluates requirements, change, implementation, and incident gates, **Then** the public entrypoints still return the same gate outcomes for the same representative inputs.
2. **Given** the new sibling modules, **When** a maintainer reviews the branch, **Then** they can distinguish context shapes, public evaluation entrypoints, private rule helpers, and gatekeeper tests without reading one oversized file.
3. **Given** a proposal to change gate policy semantics during this refactor, **When** it is evaluated against this feature, **Then** it is treated as out of scope unless separately documented and validated.

### Edge Cases

- Canon publishes a posture payload that includes the right contract line but omits one required field.
- Canon and Boundline agree on the contract line but disagree on the supported release window.
- One host manifest or runtime-compatibility reference stays on an older Canon version even though the stable contract doc is current.
- The gatekeeper split accidentally drops a public evaluation function or materially changes blocker wording used by downstream validation.
- A future proposal tries to add Canon-owned route or participant semantics to the reasoning-posture contract without changing the contract line.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST publish one stable reasoning-posture contract for `governed_reasoning_posture_v1` that identifies the owner, stable doc path, primary consumer, and supported Boundline and Canon release windows.
- **FR-002**: Canon MUST publish the required producer fields for `governed_reasoning_posture_v1`, including contract identity, compatibility window, one of `required_profile_family` or `required_profile_id`, `minimum_independence`, `admission_priority`, `confidence_handoff_required`, and `provenance_ref`.
- **FR-003**: Canon MUST publish the supported reasoning-posture vocabulary for profile families, explicit profile ids, and admission priorities used by the active Boundline consumer.
- **FR-004**: Canon MUST define compatibility rules that fail closed on unsupported contract lines, incomplete posture payloads, or incompatible release windows.
- **FR-005**: Canon MUST keep the stable contract document and the feature-local contract brief synchronized so downstream consumers do not face conflicting truth sources.
- **FR-006**: Canon MUST provide executable validation that checks the stable contract against Canon-owned contract artifacts and the active Canon workspace version, while Boundline consumer alignment is recorded through independent cross-repo review.
- **FR-007**: Canon MUST keep release-facing metadata used by downstream validation, including workspace version references, plugin manifests, and runtime-compatibility references, aligned with the contract surface this branch declares.
- **FR-008**: Canon MUST keep reasoning-posture ownership bounded to posture authoring, provenance, and compatibility semantics, and MUST NOT assume downstream activation, participant, confidence, or trace ownership.
- **FR-009**: Canon MUST preserve the public gatekeeper evaluation surface and material gate behavior while the touched runtime code is reorganized for maintainability.
- **FR-010**: Canon MUST NOT introduce new gate policy semantics, new runtime governance behavior, or hidden cross-feature changes under the reasoning-posture contract slice without explicit additional specification and validation.
- **FR-011**: Canon MUST record cross-repo review and validation evidence for the reasoning-posture contract in a durable feature-local validation artifact.
- **FR-012**: Canon MUST update operator-facing and maintainer-facing docs needed to explain the active contract line and downstream release pairing.

### Key Entities *(include if feature involves data)*

- **Governed Reasoning Posture Contract**: The Canon-owned stable contract that defines the reasoning posture producer shape, contract line, supported vocabulary, and consumer boundary.
- **Reasoning Compatibility Window**: The explicit Boundline and Canon version range under which the published posture contract is supported.
- **Minimum Independence Requirement**: The Canon-owned posture sub-shape that describes route, provider, context, prompt-pattern, and participant distinctness expectations.
- **Release Alignment Surface**: The set of Canon-owned versioned references, including workspace version, plugin manifests, and runtime-compatibility metadata, that downstream consumers use to determine whether the branch is aligned.
- **Gatekeeper Evaluation Surface**: The public Canon gate evaluation entrypoints and supporting context and rule layers that must remain behaviorally stable while their code organization is improved.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can identify the active Canon reasoning-posture contract line, required fields, and supported release window in under 10 minutes using repository artifacts alone.
- **SC-002**: Executable contract validation fails on every tested unsupported contract line, missing required field, or incompatible release-window scenario and passes on the supported release pair.
- **SC-003**: Canon release-alignment validation reports one consistent version across workspace metadata, plugin manifests, and runtime-compatibility references for the active branch state.
- **SC-004**: Representative gatekeeper tests for the touched modes continue to return the same gate outcomes after the module decomposition.

## Validation Plan *(mandatory)*

- **Structural validation**: Review the stable contract, feature-local brief, release metadata, and Boundline consumer brief for one coherent contract identity and release window.
- **Logical validation**: Run the governed reasoning posture contract tests, release-surface alignment tests, and gatekeeper-focused behavior checks.
- **Independent validation**: Perform a cross-repo review against Boundline `061-reasoning-profile-contracts` to confirm Canon remains the posture producer while Boundline remains the runtime owner.
- **Evidence artifacts**: `specs/058-governed-reasoning-posture-contract/`, `docs/integration/governed-reasoning-posture-contract.md`, `tests/contract/governed_reasoning_posture_contract.rs`, and the touched Canon validation surfaces.

## Decision Log *(mandatory)*

- **D-001**: Canon remains the producer and semantic owner of `governed_reasoning_posture_v1`, **Rationale**: this keeps posture authoring and compatibility in Canon while downstream execution remains in Boundline.
- **D-002**: Unsupported contract lines and incompatible release windows fail closed, **Rationale**: silent degradation would make downstream reasoning activation unsafe and opaque.
- **D-003**: The gatekeeper work in this branch is maintainability follow-through, not a new policy feature, **Rationale**: the module split is justified only if behavior stays stable and review scope becomes clearer.

## Non-Goals

- Defining or implementing Boundline reasoning activation, participant routing, confidence synthesis, trace emission, or operator-facing execution summaries.
- Creating Canon-owned execution loops for self-consistency, debate, reflexion, or adjudication.
- Introducing new gate policies, new approval rules, or new operational governance semantics under cover of the gatekeeper refactor.

## Assumptions

- Boundline `061-reasoning-profile-contracts` is the active downstream consumer for this first reasoning-posture contract slice.
- The active supported release pair in the current branch remains Boundline `0.61.x` and Canon `0.57.x` unless a later coordinated release explicitly changes that window.
- The gatekeeper runtime changes already present in the branch are structural follow-through and are intended to preserve existing gate behavior.
- Existing Canon release metadata and assistant package surfaces are part of the downstream compatibility story and therefore belong in this feature boundary when they drift.
