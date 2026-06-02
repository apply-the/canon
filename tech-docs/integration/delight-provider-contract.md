# Canon Delight Provider Contract

## Contract Identity

- `owner`: `canon`
- `current_contract_line`: `delight-provider-v1`
- `schema_version`: `v1`
- `stable_doc`: `tech-docs/integration/delight-provider-contract.md`
- `primary_consumer`: `boundline`
- `feature_brief`: `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`

## Purpose

Define the Canon-owned producer semantics for governed artifacts, metadata, and
compatibility signals that Boundline S7 may consume for delight surfaces.
This contract makes the Canon-to-S7 boundary explicit, stable, and extensible.

## Authority And Sync Rules

- This stable document is the normative source for which artifact classes are
  authorized, what metadata they must carry, and how compatibility signaling works.
- Feature-local contracts (under `specs/`) may elaborate with examples and
  supporting shapes but do not supersede this document.
- If a conflict appears between this stable document and a feature-local brief,
  this stable document wins and the feature-local brief must be realigned before
  merge.
- Consumers may rely on Canon contracts but may not redefine Canon promotion
  semantics, governance authority, or artifact eligibility rules.

## Contract Constraints

- Canon MUST NOT become responsible for Boundline's UX, assistant command
  behavior, explanation vocabulary, or delight orchestration.
- S7 consumption MUST be explicitly contracted; implicit or ambient Canon
  semantics MUST NOT become available to Boundline delight surfaces.
- Every artifact class listed here MUST support a degraded-state outcome in
  Boundline when that artifact is missing, stale, incompatible, or contradicted.
- Contract extension requires explicit amendment acknowledged by both Canon and
  Boundline before any new artifact class becomes authorized.

## Authorized Artifact Classes

The six classes below are exhaustive for `delight-provider-v1`. Underlying
Canon envelopes, packet families, and metadata carriers remain source anchors;
they do not create additional S7 contract classes unless this contract is
amended.

### `packets`

- **Source anchors**:
  - `AuthorityGovernanceV1Envelope` and `AdaptiveGovernanceV1Envelope` in `crates/canon-engine/src/domain/publish_profile.rs`
  - packet metadata sidecars described in `tech-docs/integration/governance-adapter.md`
  - promoted managed surfaces governed by `tech-docs/integration/project-memory-promotion-contract.md`
- **Required metadata fields**:
  - `delight_provider_contract_version` — `delight-provider-v1`
  - `contract_line` — the packet family contract line, such as `authority-governance-v1` or `adaptive-governance-v1`
  - `schema_version` — packet family schema version
  - `promotion_state` — serialized Canon promotion state
  - `source_ref` — the run, packet, or promoted surface lineage reference
  - `degradation_state` — one of the `CompatibilitySignal` values
  - `mode` — Canon mode that produced the packet
- **Eligibility rules**:
  - packets MUST remain Canon-owned artifacts with Canon-managed lineage
  - packets MUST NOT be consumed when the governing packet state is rejected or outside the contracted line
- **Degradation conditions**:
  - `absent` — packet or promoted surface not present in the workspace
  - `stale` — packet exists but the promoted or cached copy predates the current governed state
  - `incompatible` — packet contract line or schema version is outside the contracted range
  - `contradicted` — packet content conflicts with a co-present Canon governance signal

---

### `approval-states`

- **Source anchors**:
  - `AuthorityApprovalState` in `crates/canon-engine/src/domain/publish_profile.rs`
  - approval-state projections documented in `tech-docs/integration/governance-adapter.md`
  - approval semantics referenced by `tech-docs/integration/project-memory-promotion-contract.md`
- **Required metadata fields**:
  - `delight_provider_contract_version`
  - `contract_line`
  - `schema_version`
  - `source_ref`
  - `promotion_state`
  - `degradation_state`
  - `approval_state` — `not-needed`, `requested`, `granted`, `rejected`, or `expired`
- **Eligibility rules**:
  - the value MUST come from Canon-owned approval state, not downstream inference
- **Degradation conditions**:
  - `absent` — no Canon approval signal is present for the requested packet or surface
  - `stale` — approval signal exists but predates the current packet lineage or is still pending human action
  - `incompatible` — approval signal is emitted under an unsupported contract line or schema
  - `contradicted` — approval posture conflicts with a co-present Canon governance or review signal

---

### `readiness-signals`

- **Source anchors**:
  - `AuthorityPacketReadiness` in `crates/canon-engine/src/domain/publish_profile.rs`
  - readiness projections documented in `tech-docs/integration/governance-adapter.md`
  - packet routing semantics in `tech-docs/integration/project-memory-promotion-contract.md`
- **Required metadata fields**:
  - `delight_provider_contract_version`
  - `contract_line`
  - `schema_version`
  - `source_ref`
  - `promotion_state`
  - `degradation_state`
  - `packet_readiness` — `pending`, `incomplete`, `reusable`, or `rejected`
- **Eligibility rules**:
  - readiness values MUST remain Canon-owned packet posture, not downstream synthesis
- **Degradation conditions**:
  - `absent` — no Canon readiness signal is available
  - `stale` — readiness signal exists but no longer matches the current packet or promoted surface
  - `incompatible` — readiness signal is emitted under an unsupported contract line or schema
  - `contradicted` — readiness signal conflicts with co-present governance or review posture

---

### `security-findings`

- **Source anchors**:
  - `security-assessment` packet families emitted by Canon runtime modes
  - promoted `tech-docs/security-assessments` surfaces routed by `crates/canon-engine/src/orchestrator/publish.rs`
  - packet lineage and promotion semantics from `tech-docs/integration/project-memory-promotion-contract.md`
- **Required metadata fields**:
  - `delight_provider_contract_version`
  - `contract_line`
  - `schema_version`
  - `source_ref`
  - `promotion_state`
  - `degradation_state`
  - `finding_kind` — Canon-owned security finding family or category
  - `severity` — Canon-owned severity vocabulary carried by the packet or promoted artifact
- **Eligibility rules**:
  - security findings MUST remain Canon-authored or Canon-promoted packet content
  - downstream runtimes MUST NOT invent additional Canon security findings when the class is absent
- **Degradation conditions**:
  - `absent` — no Canon security finding artifact is present
  - `stale` — security finding exists but was produced for an older governed state
  - `incompatible` — finding artifact lacks the required contract line or schema version
  - `contradicted` — finding content conflicts with a co-present Canon approval or readiness signal

---

### `audit-findings`

- **Source anchors**:
  - review findings emitted by `review` and `pr-review` packet families
  - verification findings emitted by `verification` packet families
  - Canon-managed review finding surfaces such as `review-summary.md` and conventional-comments artifacts
- **Required metadata fields**:
  - `delight_provider_contract_version`
  - `contract_line`
  - `schema_version`
  - `source_ref`
  - `promotion_state`
  - `degradation_state`
  - `finding_kind` — review, verification, or audit-oriented Canon finding category
  - `disposition` — Canon-owned disposition or closure state when present
- **Eligibility rules**:
  - audit findings MUST originate from Canon-managed review or verification artifacts
- **Degradation conditions**:
  - `absent` — no Canon audit or review finding is present
  - `stale` — audit finding refers to superseded packet lineage or an outdated diff/evidence set
  - `incompatible` — finding artifact lacks the required contract line or schema version
  - `contradicted` — finding posture conflicts with a co-present approval, readiness, or packet signal

---

### `promotion-references`

- **Source anchors**:
  - `promotion_refs` in `AuthorityGovernanceV1Envelope`
  - promotion targets and routing semantics in `tech-docs/integration/project-memory-promotion-contract.md`
- **Required metadata fields**:
  - `delight_provider_contract_version`
  - `contract_line`
  - `schema_version`
  - `source_ref`
  - `promotion_state`
  - `degradation_state`
  - `promotion_ref` — Canon-owned promotion reference identifier or path
  - `target_class` — Canon target class for the promoted artifact
- **Eligibility rules**:
  - promotion references MUST remain Canon-owned publication outputs
  - references to `evidence` and `index` target classes remain outside the delight-provider surface
- **Degradation conditions**:
  - `absent` — no Canon promotion reference is available for the requested surface
  - `stale` — promotion reference points to an older promoted surface than the current governed state
  - `incompatible` — promotion reference or target class falls outside the contracted publication rules
  - `contradicted` — promotion reference conflicts with co-present packet or governance state

## Compatibility Signaling

Canon MUST emit a `degradation_state` field alongside every contracted artifact.
Boundline MUST evaluate this field before consuming the artifact.

| Signal | Meaning | Required Boundline Action |
|---|---|---|
| `available` | Present, promoted, within contracted schema | Consume normally |
| `stale` | Present but promotion epoch is outdated or gate is pending | Surface staleness; do not fabricate certainty |
| `incompatible` | Contract line or schema version outside contracted range | Do not consume; surface incompatibility |
| `absent` | Artifact class not present in workspace | Continue with Boundline-only evidence |
| `contradicted` | Artifact contradicts a co-present canonical signal | Surface contradiction; do not merge conflicting signals |

## Schema Versioning

- The delight-provider contract uses semantic versioning for its contract line
  identifier (`delight-provider-v1`, `delight-provider-v2`, ...).
- Individual artifact class schema versions track the Rust envelope type
  version and are pinned in this document.
- A change that alters required metadata fields, `CompatibilitySignal` values,
  or eligibility rules for any authorized class constitutes a breaking change
  and requires a new contract line.
- Additive changes (new optional fields, new authorized classes) MAY be
  accommodated within the current contract line with a documented amendment.

## Amendment Procedure

1. Canon proposes the amendment by updating
   `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`
   and opening a cross-repo review request to Boundline.
2. Once both teams acknowledge, the stable document at
   `tech-docs/integration/delight-provider-contract.md` is updated with the new
   contract line.
3. The prior artifact class remains listed under `Deprecated Classes` with its
   `removal_epoch` until that epoch is reached.
4. Canon MUST provide advance notice and fallback guidance when deprecating any
   authorized artifact class.

## Deprecated Classes

*None at contract-line `delight-provider-v1`.*

## Out Of Scope

The following are explicitly NOT part of this contract:

- Boundline's S7 UX, CLI rendering, or explanation vocabulary.
- How Boundline prioritizes or combines inputs into explanations.
- Runtime governance for S7 behavior; S7 remains Boundline-owned.
- Ambient or undeclared Canon semantics not listed in Authorized Artifact Classes.
- Canon `evidence` and `index` class promotion artifacts (not eligible for S7 delight).
