# Feature Specification: Adaptive Governance Semantics

**Feature Branch**: `055-adaptive-governance`  
**Created**: 2026-05-16  
**Status**: Draft  
**Input**: User description: "su canon e boundline con due nuovi feature branch per implementare ../boundline/roadmap/S4 - control-graduation-and-adaptive-governance-spec.md. Definisci contratti tra i due se necessario"

## Governance Context *(mandatory)*

**Mode**: architecture  
**Risk Classification**: systemic-impact, because this slice defines Canon-owned adaptive-governance semantics that downstream runtimes will consume for delivery-critical governance behavior.  
**Scope In**: Canon-owned vocabulary for `advisory`, `catch`, `rule`, and `hook`; rollout-profile semantics for `minimal`, `guided`, `governed`, and `strict`; compatibility rules with `authority-governance-v1`; optional companion-contract semantics for downstream consumers; preserved approval, readiness, project-memory, lineage, and promotion-state semantics; and operator-facing documentation that explains the semantic boundary.  
**Scope Out**: runtime confidence computation, trust evolution, degradation selection, escalation targets, council assembly, stop-transition behavior, provider or model routing, and operator override execution.

**Invariants**:

- Canon remains the semantic authority for governed posture and MUST NOT become the runtime orchestrator for Boundline or other downstream delivery systems.
- `authority-governance-v1` remains the required S3 posture contract and MUST NOT silently change meaning as part of S4.
- Any S4 companion semantics remain advisory and portable, and MUST NOT assign runtime councils, reviewers, providers, models, stop transitions, or override outcomes.

**Decision Traceability**: Decisions for this feature are recorded in `specs/055-adaptive-governance/decision-log.md`, with the downstream semantic boundary documented in Canon governance and integration docs.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Adaptive Governance Vocabulary (Priority: P1)

As a Boundline maintainer, I want Canon to define adaptive-governance vocabulary and compatibility rules without requiring source-code inference, so downstream runtimes can interpret S4 semantics consistently across repository boundaries.

**Why this priority**: S4 is cross-repo. If Canon does not define the shared semantic vocabulary clearly, every consumer will invent its own meaning for the same governance terms.

**Independent Test**: Review Canon docs plus representative governed metadata and verify that a downstream maintainer can determine the meaning of `advisory`, `catch`, `rule`, `hook`, and the rollout profiles `minimal`, `guided`, `governed`, and `strict` without reading Canon implementation code.

**Acceptance Scenarios**:

1. **Given** a downstream maintainer reading Canon adaptive-governance documentation, **When** they inspect the S4 vocabulary, **Then** they can determine the semantic meaning of the governance states and rollout profiles without reading Canon source code.
2. **Given** a downstream maintainer comparing S3 and S4 documents, **When** they inspect the contract boundary, **Then** they can tell that `authority-governance-v1` remains the required posture baseline while adaptive-governance semantics are additive.
3. **Given** a missing or unsupported adaptive companion contract, **When** a downstream runtime inspects a governed packet, **Then** the consumer can treat the adaptive semantics as unavailable without guessing their meaning from filenames or prose fragments.

---

### User Story 2 - Preserve The Semantic And Runtime Boundary (Priority: P2)

As a Canon maintainer, I want Canon to publish adaptive-governance semantics without choosing runtime confidence, councils, degradation, or stop transitions, so downstream runtimes can stay locally authoritative for operational governance.

**Why this priority**: The semantic and runtime boundary is the key cross-repo constraint. If Canon begins assigning operational runtime behavior directly, the contract stops being portable.

**Independent Test**: Compare the final Canon semantic contract against the paired Boundline S4 spec and verify that Canon defines meaning and compatibility only, while Boundline remains responsible for runtime behavior and authority transfer.

**Acceptance Scenarios**:

1. **Given** Canon publishes adaptive-governance vocabulary, **When** a downstream runtime reads it, **Then** the documentation makes clear that the terms describe semantic posture rather than executable runtime decisions.
2. **Given** a proposal that tries to place councils, model routes, runtime confidence scores, or stop transitions inside Canon adaptive metadata, **When** it is evaluated against this feature, **Then** it is explicitly out of scope.
3. **Given** Canon publishes approval or readiness semantics together with adaptive governance semantics, **When** a consumer reads them, **Then** the semantic meaning remains Canon-owned while the runtime action stays consumer-owned.

---

### User Story 3 - Evolve The Contract Safely (Priority: P3)

As a cross-repo integrator, I want additive compatibility rules for adaptive-governance semantics, so Canon can evolve the semantic layer without breaking older consumers or silently changing the meaning of existing posture contracts.

**Why this priority**: Cross-repo governance contracts fail when versioning rules are implicit. Safe additive change rules are required from the first S4 slice.

**Independent Test**: Present one supported S3 posture contract, one packet with optional adaptive-governance companion semantics, and one packet with an unsupported adaptive companion contract, and verify that consumers can classify the required baseline, use or ignore the optional companion safely, and fail closed on incompatible semantics.

**Acceptance Scenarios**:

1. **Given** a packet with supported `authority-governance-v1` semantics and additional optional adaptive-governance metadata, **When** an older consumer reads it, **Then** the consumer can still use the required posture semantics without inventing meaning for the optional companion metadata.
2. **Given** a packet with an unsupported adaptive-governance contract line, **When** a downstream consumer reads it, **Then** the consumer can reject the adaptive semantics while still determining whether the required S3 posture contract remains compatible.
3. **Given** Canon introduces new optional adaptive-governance fields in a compatible contract line, **When** older consumers read them, **Then** they may ignore the new fields without changing the meaning of the existing adaptive vocabulary.

### Edge Cases

- A consumer sees `authority-governance-v1` but no adaptive-governance companion metadata, so Canon must make clear that the required posture contract remains usable on its own.
- A packet includes adaptive-governance metadata that names stronger maturity semantics than the downstream runtime supports, so the consumer must be able to reject the companion contract without guessing fallback runtime behavior.
- Canon documents rollout profiles using language that could be mistaken for council profiles, so the docs must keep governance maturity separate from runtime council size.
- A packet is missing required S3 posture metadata but includes optional adaptive semantics, so Canon must make clear that the optional companion cannot repair a missing required baseline.
- A future additive adaptive field appears in the same compatible contract line, so older consumers must remain able to ignore it safely.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST define S4 semantic vocabulary for these governance states: `advisory`, `catch`, `rule`, and `hook`.
- **FR-002**: Canon MUST define S4 rollout-profile vocabulary for these maturity labels: `minimal`, `guided`, `governed`, and `strict`, and MUST keep them distinct from S3 council profiles.
- **FR-003**: Canon MUST keep `authority-governance-v1` as the required S3 posture contract for downstream S4 consumers and MUST NOT silently change the existing meaning of its fields.
- **FR-004**: If Canon publishes machine-readable S4 companion semantics in this slice, it MUST do so under a separate named contract line, `adaptive-governance-v1`, rather than overloading `authority-governance-v1`.
- **FR-005**: Canon MUST define `adaptive-governance-v1`, if emitted, as optional companion semantics rather than a required baseline for all downstream governed consumers.
- **FR-006**: Canon MUST keep any `adaptive-governance-v1` companion semantics advisory and semantic only, and MUST NOT use that contract to assign runtime confidence scores, trust levels, councils, reviewers, providers, model routes, override outcomes, or stop transitions.
- **FR-007**: Canon MUST document the relationship between `authority-governance-v1` and optional `adaptive-governance-v1`, including which contract is required, which is optional, and how downstream consumers should behave when the companion contract is absent or unsupported.
- **FR-008**: Canon MUST preserve approval state, readiness semantics, governance metadata, lineage, project-memory meaning, and promotion state as Canon-owned semantics available to downstream runtimes.
- **FR-009**: Canon MUST document that downstream runtimes such as Boundline own runtime confidence evaluation, trust evolution, degradation selection, escalation selection, council assembly, and stop semantics.
- **FR-010**: Canon MUST require incompatible adaptive-governance vocabulary or meaning changes to use a new contract line instead of silently repurposing an existing one.
- **FR-011**: Canon MUST make missing required S3 posture metadata distinguishable from missing optional adaptive companion metadata so consumers can fail closed only on the required baseline and not invent adaptive meaning.
- **FR-012**: Canon MUST keep machine-facing contract semantics and human-facing governance documentation aligned so downstream maintainers do not need to choose between conflicting truth sources.
- **FR-013**: Canon MUST publish operator-facing documentation for governance semantics and authority zones that downstream maintainers can use without inspecting Canon source code.

### Key Entities *(include if feature involves data)*

- **Authority Posture Contract**: The required Canon-owned `authority-governance-v1` semantic baseline that describes governed posture for downstream consumers.
- **Adaptive Governance Companion Contract**: The optional Canon-owned `adaptive-governance-v1` semantic layer that describes governance maturity terms without assigning runtime execution behavior.
- **Governance State Vocabulary**: The Canon-defined meaning of `advisory`, `catch`, `rule`, and `hook` as semantic governance states.
- **Rollout Profile Vocabulary**: The Canon-defined meaning of `minimal`, `guided`, `governed`, and `strict` as governance maturity labels rather than council-shape labels.
- **Semantic Compatibility Rule**: The additive or incompatible change rule that tells consumers when they may ignore new optional metadata and when they must reject a new contract line.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A downstream maintainer can determine the meaning of the S4 governance states, rollout profiles, and cross-contract boundary in under 10 minutes using Canon documentation and representative governed metadata alone.
- **SC-002**: In representative contract reviews, 100% of adaptive-governance semantics remain clearly semantic and advisory, with no Canon-owned assignment of runtime councils, confidence, routes, or stop transitions.
- **SC-003**: In representative supported, missing-companion, and unsupported-companion scenarios, 100% of consumers can distinguish the required posture baseline from the optional adaptive companion behavior.
- **SC-004**: Canon human-facing docs and machine-facing contract output describe one coherent adaptive-governance boundary without conflicting ownership claims.

## Validation Plan *(mandatory)*

- **Structural validation**: Review the final semantics against the S4 roadmap, existing S3 posture vocabulary, and representative governed packet metadata to verify consistency and compatibility rules.
- **Logical validation**: Walk through one required-baseline-only packet, one packet with compatible optional adaptive companion semantics, one packet with an unsupported adaptive companion contract, and one packet with missing required posture semantics.
- **Independent validation**: Perform a separate cross-repo review against the paired Boundline S4 spec to confirm Canon remains the semantic authority and Boundline remains the runtime authority.
- **Evidence artifacts**: `specs/055-adaptive-governance/` plus the delivered Canon governance documentation and integration guidance for downstream consumers.

## Decision Log *(mandatory)*

- **D-001**: `authority-governance-v1` remains the required S3 posture baseline for S4 consumers, **Rationale**: preserving the existing required contract avoids forcing every downstream runtime to wait for a second machine contract before adopting S4 behavior.
- **D-002**: If Canon emits machine-readable S4 companion semantics, it does so as `adaptive-governance-v1`, **Rationale**: a separate contract line preserves additive evolution and prevents silent meaning drift in the existing posture contract.
- **D-003**: Runtime confidence, trust, councils, and stop behavior stay out of Canon semantic contracts, **Rationale**: keeping operational decisions downstream preserves the semantic and runtime boundary required by S4.

## Non-Goals

- Compute runtime confidence, trust evolution, degradation choice, escalation targets, council assembly, or stop transitions for downstream systems.
- Assign provider routes, model routes, reviewer sets, or override outcomes to external runtimes.
- Turn Canon into the operator-facing runtime controller for Boundline or another downstream delivery system.

## Assumptions

- Boundline is the primary first downstream consumer of S4 semantics, but Canon semantics should remain reusable by other downstream runtimes.
- The first S4 slice may deliver documentation and compatibility rules before every governed packet family emits optional adaptive companion metadata.
- Older consumers can ignore unknown optional adaptive metadata safely, but they should reject unsupported contract lines when stage policy requires a compatible companion contract.
- Canon continues to own project memory, approval semantics, readiness semantics, and governed semantic meaning even when downstream runtimes execute stronger or weaker governance behavior.
