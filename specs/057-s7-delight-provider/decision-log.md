# Decision Log: Canon S7 Delight Provider Contracts

**Feature**: 057-s7-delight-provider
**Date**: 2026-05-17
**Spec decisions**: `specs/057-s7-delight-provider/spec.md`

---

## Spec-Stage Decisions

### D-001 — Canon remains the authoritative owner of governance semantics

**Decision**: Canon MUST remain the authoritative owner of governance semantics;
S7 is a consumer, not a co-designer of Canon concepts.

**Rationale**: Prevents divergence and keeps governance authority in Canon's
domain. If S7 could author or redefine Canon semantics, the governance boundary
would become ambiguous and unenforceable.

**Alternatives considered**: Collaborative co-ownership — rejected because it
blurs the boundary and creates merge conflicts in governance authority.

**Status**: Accepted. Recorded in spec.md D-001.

---

### D-002 — S7 consumption from Canon MUST be explicitly contracted

**Decision**: S7 consumption from Canon MUST be explicitly contracted, not
ambient. No unlisted artifact class may be consumed regardless of availability.

**Rationale**: Prevents creeping scope and makes boundary maintenance feasible.
Implicit consumption cannot be audited, versioned, or deprecated safely.

**Alternatives considered**: Implicit availability of all promoted Canon
artifacts — rejected because it removes the ability to deprecate, version, or
restrict artifact classes independently.

**Status**: Accepted. Recorded in spec.md D-002.

---

### D-003 — Every Canon artifact consumed by S7 MUST support degraded-state outcomes

**Decision**: Every governed artifact class in the delight-provider contract MUST
carry degradation conditions and Canon MUST emit a `degradation_state` signal.
Boundline MUST act on this signal and MUST NOT fabricate certainty.

**Rationale**: Keeps Boundline usable and honest when Canon governance is
unavailable or incompatible. Delight that hides degraded governance is more
dangerous than no delight.

**Alternatives considered**: Optional degradation signaling — rejected because
optional signals become ignored under operational pressure.

**Status**: Accepted. Recorded in spec.md D-003.

---

## Design-Stage Decisions

### D-004 — Contract line identifier is `delight-provider-v1`

**Decision**: The delight-provider contract uses `delight-provider-v1` as its
stable contract line identifier.

**Rationale**: Follows the existing pattern (`authority-governance-v1`,
`adaptive-governance-v1`). Qualifies the Canon role (`delight-provider`) and
schema generation (`v1`) without naming the consumer technology.

**Alternatives considered**: `s7-input-v1` — rejected (names consumer, not Canon
role); `canon-delight-v1` — rejected (implies Canon owns delight behavior).

**Status**: Accepted. Recorded in research.md Q2 and contracts/delight-provider-contract.md.

---

### D-005 — Five artifact classes authorized in v1

**Decision**: The initial `delight-provider-v1` contract authorizes exactly five
governed artifact classes: `authority-governance`, `adaptive-governance`,
`semantic-artifact`, `expertise-input`, and `project-memory-promotion`. No
additional classes are authorized without explicit amendment.

**Rationale**: These five classes are already modeled with versioned contract
lines, typed serde structs, and published integration documents. They cover the
S7 delight surface without requiring new Rust types or runtime changes.

**Alternatives considered**: Including raw `packet-metadata.json` keys or
runmanifest fields — rejected (not stable typed shapes; would couple S7 to
internal Canon serialization details).

**Status**: Accepted. Recorded in research.md Q1 and data-model.md.

---

### D-006 — Five-state `CompatibilitySignal` enum

**Decision**: Compatibility and degradation signaling uses five discrete states:
`available`, `stale`, `incompatible`, `absent`, `contradicted`.

**Rationale**: Five states cover the full degradation surface from the spec (US2
acceptance scenarios). Discrete enum values are safer than free-form rationale
strings because Boundline can map them to UX state without parsing Canon prose.

**Alternatives considered**: Boolean `is_available` + free-form `reason` —
rejected (pushes interpretation logic into S7); three-state — rejected
(insufficient to distinguish `stale` from `absent` from `contradicted`).

**Status**: Accepted. Recorded in research.md Q4, data-model.md, and contracts/delight-provider-contract.md.

---

### D-007 — Semantic contract line versioning + per-class schema versions

**Decision**: The delight-provider contract uses semantic versioning for the
contract line (`delight-provider-v1`, `delight-provider-v2`, …) with individual
artifact class schema versions pinned separately in the contract document.

**Rationale**: Per-class schema versions allow independent evolution of artifact
types without forcing a full contract line increment. Semantic contract line
versioning makes cross-team amendment coordination unambiguous.

**Alternatives considered**: Date-based versioning — rejected (does not
communicate breaking vs. additive); single global schema version — rejected
(creates unnecessary coupling between artifact evolution cycles).

**Status**: Accepted. Recorded in research.md Q5 and contracts/delight-provider-contract.md.

---

### D-008 — No new Rust types required for v1

**Decision**: The `delight-provider-v1` contract is a documentation-only artifact.
Existing Rust types in `publish_profile.rs` serve as normative schema anchors
without modification.

**Rationale**: The existing types (`AuthorityGovernanceV1Envelope`,
`AdaptiveGovernanceV1Envelope`, `SemanticArtifactDescriptor`,
`ExpertiseInputMetadata`) are already versioned, serde-derived, and stable.
Adding new Rust types for documentation purposes would be unnecessary complexity.

**Alternatives considered**: New `DelightProviderContractLine` Rust struct —
rejected (adds runtime coupling where only a Markdown contract document is needed).

**Status**: Accepted. Recorded in plan.md Technical Context.

---

### D-009 — `evidence` and `index` target classes excluded from S7 eligibility

**Decision**: The `project-memory-promotion` artifact class excludes `evidence`
and `index` target classes from S7 delight eligibility. Only `stable` and
`pending` target classes are authorized.

**Rationale**: `evidence` artifacts are audit trails, not governed knowledge
surfaces. `index` artifacts are structural append-only summaries. Neither is
appropriate for Boundline delight consumption.

**Alternatives considered**: Allowing `evidence` with degradation — rejected
(evidence bundles are voluminous and not structured for explanation use).

**Status**: Accepted. Recorded in data-model.md and contracts/delight-provider-contract.md.

---

## Implementation-Stage Decisions

### D-010 — Stable S7 contract vocabulary is expressed as six consumer-facing classes

**Decision**: The implementation surface for `delight-provider-v1` is expressed
as six consumer-facing artifact classes: `packets`, `approval-states`,
`readiness-signals`, `security-findings`, `audit-findings`, and
`promotion-references`.

**Rationale**: This wording matches the feature specification and keeps the
Canon-to-Boundline boundary understandable to downstream reviewers without
forcing them to reverse-map raw Canon Rust type names.

**Alternatives considered**: Exposing only the underlying Rust-type families —
rejected because that would optimize for Canon implementation detail instead of
the consumer-facing contract boundary.

**Status**: Accepted for implementation. The stable doc, feature brief,
data model, and contract tests must all use the same six-class vocabulary.

---

### D-011 — Stable-doc publication remains gated even after local implementation completes

**Decision**: Completing the local Markdown contract and contract test harness
does not by itself authorize downstream consumption. The stable integration
document remains non-authoritative for release purposes until the cross-team
review and human Systemic-Impact approval evidence are recorded.

**Rationale**: This preserves the constitutionally required separation between
generation and independent validation for a systemic-impact boundary contract.

**Alternatives considered**: Treating merge readiness as equivalent to release
readiness — rejected because it would collapse generation and approval into one
step.

**Status**: Accepted for implementation and reflected in validation-report.md.

---

### D-012 — Approval and readiness remain distinct contract classes even when emitted by one envelope

**Decision**: `approval-states` and `readiness-signals` remain separate
consumer-facing contract classes even though both may be emitted by the same
authority-governance metadata carrier.

**Rationale**: S7 consumes approval posture and readiness posture for different
questions. Collapsing them into one class would reintroduce Canon implementation
detail at the contract boundary and weaken degraded-state handling.

**Alternatives considered**: Treating them as one combined governance-status
class — rejected because it would blur independent validation and drift checks.

**Status**: Accepted for implementation. The contract tests must verify both
classes independently.

---

### D-013 — The six-class inventory is exhaustive for `delight-provider-v1`

**Decision**: `delight-provider-v1` authorizes exactly six consumer-facing
classes: `packets`, `approval-states`, `readiness-signals`,
`security-findings`, `audit-findings`, and `promotion-references`.

**Rationale**: The S7 consumer boundary must be explicit and auditably finite.
If older type-centric labels such as `authority-governance` or
`project-memory-promotion` remained first-class alongside the six classes, the
contract would reintroduce ambient Canon semantics by duplication.

**Alternatives considered**: Keeping both the six consumer-facing classes and
the earlier type-centric labels as co-equal contract entries — rejected because
it would make boundary validation ambiguous.

**Status**: Accepted for implementation. The stable doc and contract tests must
reject any additional first-class class headings for this contract line.

---

### D-014 — Every contracted class must surface Canon-owned degradation and versioning rules

**Decision**: Every authorized class in `delight-provider-v1` must surface the
Canon-owned compatibility states `available`, `stale`, `incompatible`,
`absent`, and `contradicted`, and the contract must continue to distinguish
breaking changes (new contract line required) from additive changes (amendment
within the current line allowed).

**Rationale**: Degraded-state handling and schema-versioning rules are part of
the contract boundary, not optional implementation detail. Without them,
downstream S7 surfaces would need to infer when Canon data is safe to consume.

**Alternatives considered**: Leaving degradation and versioning to prose-only
consumer interpretation — rejected because it would make validation non-deterministic.

**Status**: Accepted for implementation. The contract tests must assert both the
five-state compatibility table and the schema-versioning rules.

---

### D-015 — Stable doc and feature brief must fail closed on drift

**Decision**: The stable integration document and the feature-local brief for
`delight-provider-v1` must remain synchronized in the same change. If they
diverge, the contract validation target must fail rather than leaving one as an
informal or stale shadow of the other.

**Rationale**: Boundary contracts only remain reviewable if the stable doc and
the feature-local elaboration evolve together. Drift would create two competing
sources of truth exactly where the contract is meant to remove ambiguity.

**Alternatives considered**: Allowing temporary divergence until closeout —
rejected because it weakens reviewability and makes independent validation less
reliable.

**Status**: Accepted for implementation. The contract test target must compare
the stable doc and feature brief directly.
