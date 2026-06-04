# Feature Specification: Authority Zone Contract

**Feature Branch**: `054-authority-zone-contract`  
**Created**: 2026-05-15  
**Status**: Draft  
**Input**: User description: "create specs in canon and boundline and the branches. Define the contracts between the two repositories"

## Governance Context *(mandatory)*

**Mode**: architecture  
**Risk Classification**: systemic-impact, because this slice defines a Canon-owned semantic contract that downstream runtimes such as Boundline will rely on for control, compatibility, and fail-closed behavior across repository boundaries.  
**Scope In**: a stable Canon authority-governance contract line; authority zones and change classes; governed personas and anti-behaviors; approval and readiness semantics; advisory stage-role hints; compatibility rules for downstream consumers; and Canon documentation that explains the contract boundary.  
**Scope Out**: Boundline runtime roles, council algorithms, adjudication, provider or model routing, retry policy, stop-transition policy, and runtime human-gate orchestration.

**Invariants**:

- Canon remains the semantic authority for governed posture and MUST NOT become the runtime orchestrator for external delivery systems.
- `stage_role_hints` remain advisory metadata and MUST NOT become executable assignments for runtime roles, reviewers, providers, or models.
- Human CLI output and machine-facing contract output MUST describe the same governed semantics rather than diverging into separate truth sources.

**Decision Traceability**: Decisions for this feature are recorded in `specs/054-authority-zone-contract/`, with the downstream-facing contract documented in Canon governance and integration docs.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Authority Contract For Consumers (Priority: P1)

As a Boundline maintainer, I want one Canon contract line that explains the governed posture of a packet without requiring source-code inference, so I can consume Canon semantics safely across repository boundaries.

**Why this priority**: Without one stable contract line, every downstream consumer will infer Canon semantics differently and the cross-repo boundary will drift immediately.

**Independent Test**: Inspect representative Canon governed packet metadata and Canon integration docs, and verify that a maintainer can determine the required `authority-governance-v1` fields without reading Canon implementation code, and can identify optional provenance fields when they are present.

**Acceptance Scenarios**:

1. **Given** a governed Canon packet produced by a supported mode, **When** a consumer inspects the documented contract and packet metadata, **Then** the consumer can determine the packet's required `authority-governance-v1` fields and, when present, its optional persona anti-behaviors, primary artifact, artifact order, promotion references, and stage role hints.
2. **Given** a packet whose required authority metadata is missing, **When** a consumer inspects it, **Then** the contract requires the consumer to treat the authority semantics as unavailable instead of guessing them from prose or filenames.
3. **Given** a packet using an incompatible or unknown contract line, **When** a consumer inspects it, **Then** the contract makes it possible to fail closed without reinterpreting the packet as if it were compatible.

---

### User Story 2 - Preserve The Semantic And Runtime Boundary (Priority: P2)

As a Canon maintainer, I want Canon to publish governed authority semantics without choosing runtime councils or reviewer assignments, so downstream runtimes can consume the contract without Canon drifting into orchestration ownership.

**Why this priority**: The boundary is the product. If Canon begins assigning runtime behavior directly, the contract will stop being portable and downstream runtimes will lose local control.

**Independent Test**: Compare the final Canon contract against the paired Boundline council spec and verify that Canon publishes advisory semantics and hints only, while Boundline remains responsible for reviewer choice, routing, and stop behavior.

**Acceptance Scenarios**:

1. **Given** Canon publishes `stage_role_hints`, **When** a downstream consumer reads them, **Then** the documentation makes clear that the hints are advisory and do not choose runtime roles or councils.
2. **Given** a future proposal that tries to add council profile, provider route, or model route directives to the Canon contract, **When** the proposal is evaluated against this feature, **Then** it is explicitly out of scope.
3. **Given** Canon publishes approval state or restricted authority semantics, **When** a downstream runtime consumes them, **Then** the contract explains the governed posture while leaving the runtime stop or human-gate behavior to the consumer.

---

### User Story 3 - Evolve The Contract Safely (Priority: P3)

As a cross-repo integrator, I want additive evolution rules for the authority contract, so new Canon metadata can be introduced without breaking older consumers or silently changing the meaning of existing semantics.

**Why this priority**: Cross-repo contracts fail in practice when versioning rules are implicit. Safe additive change and explicit incompatible change rules are required from the first slice.

**Independent Test**: Present a consumer with one supported contract example, one unsupported contract line, and one example with extra optional metadata, and verify that the consumer can classify the supported packet, reject the incompatible one, and ignore additive extras safely.

**Acceptance Scenarios**:

1. **Given** a packet with supported authority semantics and additional optional metadata, **When** an older consumer reads it, **Then** the consumer can still classify the supported semantics without inventing meaning for the new fields.
2. **Given** a packet with a new incompatible contract line, **When** an older consumer reads it, **Then** the consumer treats the authority contract as unavailable and does not guess fallback meaning.
3. **Given** a packet with a known contract line but unknown optional stage role hints, **When** a consumer reads it, **Then** the consumer may ignore those hints while still using the rest of the compatible authority metadata.

### Edge Cases

- A packet declares `restricted` authority semantics while approval is still missing, so the contract must express the governed posture without implying how the runtime should enforce the stop.
- A packet declares an intended persona but omits persona anti-behaviors, so the contract must allow consumers to keep the required persona semantics while treating the optional anti-behavior metadata as unavailable rather than inventing defaults.
- A packet includes `stage_role_hints` that refer to capabilities a downstream runtime does not recognize, so those hints must remain ignorable without invalidating the rest of the contract.
- A packet lists promotion references or artifact order metadata that are readable but incomplete, so the consumer must be able to preserve readability while rejecting the authority semantics as insufficient for strict governance use.
- Canon adds a future optional metadata field that older consumers do not know, so compatibility must remain additive by default unless the contract line changes.

## Requirements *(mandatory)*

### Functional Requirements

**`authority-governance-v1` field profile**:

- **Required for first-slice consumer conformance**: `authority_zone`, `change_class`, `intended_persona`, `approval_state`, `packet_readiness`, and `risk`
- **Optional additive metadata in the same contract line**: `persona_anti_behaviors`, `primary_artifact`, `artifact_order`, `promotion_refs`, and `stage_role_hints`

- **FR-001**: Canon MUST define one stable machine-readable authority-governance contract line for this slice, named `authority-governance-v1`.
- **FR-002**: The `authority-governance-v1` contract MUST publish Canon-owned `authority_zone` semantics limited in the first slice to `green`, `yellow`, `red`, and `restricted`.
- **FR-003**: The `authority-governance-v1` contract MUST publish Canon-owned `change_class` semantics limited in the first slice to `low-impact`, `bounded-impact`, `systemic-impact`, and `critical-operations`.
- **FR-004**: Every governed Canon mode covered by this slice MUST publish an intended persona as required metadata, and MAY publish persona anti-behaviors as optional metadata that further describe the mode's semantic posture without assigning downstream runtime behavior.
- **FR-005**: The contract MUST publish approval state, packet readiness, and risk as required Canon-owned governed semantics for every `authority-governance-v1` packet, and MAY publish primary artifact, artifact order, and promotion references as optional governed provenance when that packet family supports them.
- **FR-006**: The contract MUST publish `stage_role_hints` only as optional advisory capability or posture hints and MUST NOT encode runtime role assignment, council profile, provider route, model route, retry policy, stop semantics, or final decision authority.
- **FR-007**: Canon MUST document which governed modes and published artifacts are expected to carry `authority-governance-v1` semantics in this first slice.
- **FR-008**: Canon MUST preserve the rule that missing required `authority-governance-v1` metadata or an unsupported contract line causes the authority semantics to become unavailable to consumers without making the underlying packet unreadable, while missing optional metadata leaves the compatible remainder of the contract usable.
- **FR-009**: Canon MUST document additive compatibility rules so older consumers may ignore new optional metadata, while incompatible vocabulary or meaning changes require a new contract line.
- **FR-010**: Canon MUST keep the machine-facing contract and human-facing governance documentation aligned so downstream consumers do not need to choose between conflicting semantic descriptions.
- **FR-011**: Canon MUST publish documentation for governed personas and authority zones that downstream maintainers can use without reading Canon source code.
- **FR-012**: Canon MUST remain authoritative for project memory, evidence publication, approval state, packet readiness, and governed semantic meaning, rather than delegating those semantics to an external orchestrator.
- **FR-013**: Canon MUST NOT choose downstream runtime roles, domain experts, councils, stop transitions, adjudication outcomes, or human-gate execution behavior.

### Key Entities *(include if feature involves data)*

- **Authority Governance Contract**: The Canon-owned machine-readable semantic envelope that tells downstream consumers how to interpret governed posture for a packet.
- **Authority Zone**: The Canon-owned classification that expresses the minimum governance posture for a packet or action boundary.
- **Governed Persona**: The Canon-owned semantic description of the intended authoring posture for a mode, together with any optional anti-behaviors that further define what the mode is not supposed to do.
- **Stage Role Hint**: Advisory metadata that suggests downstream review or expertise posture without assigning executable runtime behavior.
- **Governance Envelope**: The combined set of authority, readiness, approval, artifact-order, and promotion semantics published with a governed packet.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A downstream maintainer can determine the required `authority-governance-v1` fields for a supported governed packet, and any optional provenance fields that are present, in under 10 minutes using Canon docs and packet metadata alone.
- **SC-002**: In representative missing-metadata and unsupported-contract scenarios, 100% of consumers can fail closed on the authority semantics without guessing fallback meaning.
- **SC-003**: In representative contract reviews, 100% of stage role hints remain clearly advisory and none require Canon to choose councils, runtime roles, providers, or models.
- **SC-004**: Canon's governance docs and machine-facing contract describe one coherent semantic boundary and do not present Canon as a runtime orchestrator.

## Validation Plan *(mandatory)*

- **Structural validation**: Review the final contract surface against roadmap S3 vocabulary, inspect representative governed packet metadata, and verify that required authority fields and compatibility rules are present and internally consistent.
- **Logical validation**: Walk through one supported packet example, one missing-metadata example, one unsupported contract-line example, one restricted-without-approval example, and one advisory-hint example.
- **Independent validation**: Perform a separate cross-repo review comparing the Canon contract against the paired Boundline consumer spec to confirm the semantic and runtime ownership boundary holds.
- **Evidence artifacts**: `specs/054-authority-zone-contract/` plus the delivered Canon governance documentation for personas and authority zones.

## Decision Log *(mandatory)*

- **D-001**: Canon will introduce a named first-slice contract line, `authority-governance-v1`, **Rationale**: a named contract line makes compatibility explicit and lets consumers fail closed when incompatible semantic changes arrive later.
- **D-002**: `stage_role_hints` remain advisory rather than executable, **Rationale**: preserving advisory-only hints keeps Canon as the semantic source of truth while leaving runtime choice to downstream systems such as Boundline.

## Non-Goals

- Define runtime council composition, reviewer quorums, adjudication rules, or stop-transition logic for external runtimes.
- Assign provider routes, model routes, or retry behavior to downstream consumers.
- Turn Canon into the operator-facing runtime controller for Boundline or any other external delivery system.

## Assumptions

- Boundline is the primary first consumer of `authority-governance-v1`, but the contract should remain reusable by other downstream governed runtimes.
- Canon can attach the authority-governance contract to the governed packet and documentation surfaces it already owns without inventing a second publication channel.
- The first slice uses the closed S3 vocabulary for authority zones and change classes, and later vocabulary expansion can follow the additive or new-contract-line rules defined here.
- Downstream runtimes may ignore unknown optional metadata safely, but they should reject unknown required vocabulary or unsupported contract lines.
