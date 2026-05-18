# Research: Canon S7 Delight Provider Contracts

**Feature**: 057-s7-delight-provider
**Date**: 2026-05-17
**Status**: Complete — all NEEDS CLARIFICATION resolved

---

## Research Question 1 — Which artifact classes already exist and are S7-eligible?

**Decision**: The following governed artifact classes are authorized for
delight-provider contract inclusion in `v1`. Each is grounded in existing
Rust types and published metadata shapes.

| Artifact Class | Rust Anchor | Contract Line | S7 Eligibility |
|---|---|---|---|
| `authority-governance` | `AuthorityGovernanceV1Envelope` | `authority-governance-v1` | Eligible |
| `adaptive-governance` | `AdaptiveGovernanceV1Envelope` | `adaptive-governance-v1` | Eligible |
| `semantic-artifact` | `SemanticArtifactDescriptor` | `v1` (`SEMANTIC_ARTIFACT_CONTRACT_LINE_V1`) | Eligible when `eligibility == Eligible` |
| `expertise-input` | `ExpertiseInputMetadata` | governed by `governed-expertise-input-contract.md v1` | Eligible |
| `project-memory-promotion` | promotion target class | governed by `project-memory-promotion-contract.md v1` | Eligible |

**Rationale**: These five classes are already modeled with versioned contract
lines, typed serde structs, and published integration documents. No new Rust
types are needed to contract them for S7.

**Alternatives considered**: Including raw runmanifest fields or
`packet-metadata.json` keys directly — rejected because they are not stable
typed shapes and would couple S7 to internal Canon serialization details.

---

## Research Question 2 — What is the contract line identifier for the delight-provider contract?

**Decision**: `delight-provider-v1`

**Rationale**: Follows the existing pattern (`authority-governance-v1`,
`adaptive-governance-v1`, `v1` for semantic artifacts). Qualifies the owner
(`canon`), the consumer intent (`delight-provider`), and the schema generation
(`v1`) in one stable identifier.

**Alternatives considered**: `s7-input-v1` — rejected because it names the
consumer technology rather than the Canon provision role; `canon-delight-v1` —
rejected because it implies Canon owns the delight behavior.

---

## Research Question 3 — What metadata fields must each artifact class carry?

**Decision**: Every artifact class in the delight-provider contract MUST carry
the following metadata when consumed by S7:

| Field | Source | Required |
|---|---|---|
| `contract_line` | From the artifact envelope's own `contract_line` field | Yes |
| `delight_provider_contract_version` | `delight-provider-v1` | Yes |
| `promotion_state` | `PromotionState` enum serialized from Canon run | Yes |
| `schema_version` | Per-class schema version (e.g., `v1`) | Yes |
| `degradation_conditions` | Listed per class in the contract | Yes |
| `lineage_ref` | Canon run ID that produced the artifact | Recommended |

**Rationale**: These fields allow S7 to determine whether a given Canon artifact
is within the contracted schema, currently promoted, and whether degradation
signaling applies — without requiring S7 to understand Canon internals.

**Alternatives considered**: Embedding all fields in `packet-metadata.json` only —
rejected because not all artifact classes are packet-backed; a per-class
envelope makes the boundary cleaner.

---

## Research Question 4 — How does compatibility and degradation signaling work?

**Decision**: Compatibility signaling follows the governance-adapter.md pattern:
Canon emits a structured metadata field (`degradation_state`) alongside each
contracted artifact. Possible values:

| Signal | Meaning | Boundline Action |
|---|---|---|
| `available` | Artifact is present, promoted, and within contracted schema | Consume normally |
| `stale` | Artifact is present but its `promotion_state` predates the current run or governance epoch | Surface staleness explicitly; do not fabricate certainty |
| `incompatible` | Artifact contract line or schema version is outside the contracted range | Do not consume; surface incompatibility |
| `absent` | Artifact class was requested but is not present in workspace | Continue with Boundline-owned evidence only |
| `contradicted` | Artifact content contradicts a co-present canonical signal (e.g., approval granted but risk class elevated) | Surface contradiction; do not merge conflicting signals |

**Rationale**: Five discrete states cover the full degradation surface described
in the spec (US2). Discrete values are safer than free-form rationale strings
because Boundline can map them to UX state without parsing Canon prose.

**Alternatives considered**: Boolean `is_available` + free-form `reason` —
rejected because it pushes interpretation logic into S7; a typed enum-equivalent
gives Boundline deterministic branching.

---

## Research Question 5 — What schema versioning approach applies?

**Decision**: The delight-provider contract uses semantic versioning for the
contract line identifier (`delight-provider-v1`, `delight-provider-v2`, …).
Individual artifact class schema versions track the Rust type version and are
pinned in the contract document. The contract document itself carries a
`current_contract_line` field following the `project-memory-promotion-contract.md`
pattern.

**Rationale**: Semantic version of the contract line makes cross-team amendment
coordination unambiguous. Per-class schema versions allow individual artifact
classes to evolve independently without forcing a full contract line increment.

**Alternatives considered**: Date-based versioning — rejected because it does
not communicate breaking vs. additive changes; single global schema version for
all classes — rejected because it creates unnecessary coupling between artifact
evolution cycles.

---

## Research Question 6 — What is the amendment procedure?

**Decision**: Contract amendments follow a two-step procedure:

1. Canon proposes a new artifact class or metadata field change by updating
   `specs/057-s7-delight-provider/contracts/delight-provider-contract.md` and
   opening a cross-repo review request to Boundline.
2. Once both teams acknowledge, the stable document at
   `docs/integration/delight-provider-contract.md` is updated with the new
   contract line and the prior class remains documented under `deprecated_classes`
   until the agreed deprecation epoch passes.

**Rationale**: Two-step process prevents unilateral Canon updates from silently
breaking S7; the stable `docs/integration/` document is the normative source per
the authority rules established by `project-memory-promotion-contract.md`.

---

## Resolved Clarifications Summary

| Item | Resolution |
|---|---|
| Which artifact classes? | 5 classes enumerated above, all grounded in existing Rust types |
| Contract line identifier | `delight-provider-v1` |
| Required metadata fields | 6 required fields + 1 recommended |
| Compatibility signaling | 5-state `degradation_state` enum |
| Schema versioning | Semantic contract line + per-class schema version |
| Amendment procedure | Two-step: feature-local brief → stable doc after cross-team acknowledgment |
