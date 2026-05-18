# Data Model: Canon S7 Delight Provider Contracts

**Feature**: 057-s7-delight-provider
**Date**: 2026-05-17
**Grounded in**: `research.md` (all NEEDS CLARIFICATION resolved)

---

## Primary Entities

### 1. DelightProviderContractLine

The stable contract identity document that governs which Canon artifact classes
S7 may consume.

| Field | Type | Required | Notes |
|---|---|---|---|
| `owner` | string | Yes | Always `canon` |
| `current_contract_line` | string | Yes | `delight-provider-v1` |
| `schema_version` | string | Yes | `v1` |
| `stable_doc` | string | Yes | `docs/integration/delight-provider-contract.md` |
| `primary_consumer` | string | Yes | `boundline` |
| `authorized_artifact_classes` | `DelightArtifactClass[]` | Yes | List of contracted classes |
| `deprecated_classes` | `DeprecatedArtifactClass[]` | No | Classes pending removal |
| `amendment_log` | string | Yes | Path to amendment history |

**Invariants**:
- `current_contract_line` MUST match the `delight_provider_contract_version`
  field emitted on every contracted artifact.
- `authorized_artifact_classes` is the exhaustive list; unlisted classes are NOT
  authorized regardless of their promotion state.
- `authorized_artifact_classes` contains exactly six classes in `delight-provider-v1`.

---

### 2. DelightArtifactClass

A single governed artifact class authorized for S7 consumption within this
contract line.

| Field | Type | Required | Notes |
|---|---|---|---|
| `class_id` | string | Yes | Stable identifier, e.g., `packets` |
| `contract_line` | string | Yes | The canonical contract line or source contract that governs the class |
| `schema_version` | string | Yes | Schema version of the source Canon shape or document family |
| `source_anchors` | `string[]` | Yes | Canonical Rust type names, packet families, or integration docs that ground the class |
| `required_metadata` | `RequiredMetadataField[]` | Yes | Fields that MUST be present for S7 consumption |
| `degradation_conditions` | `DegradationCondition[]` | Yes | When each `CompatibilitySignal` applies |
| `eligibility_rules` | string[] | No | Extra conditions beyond class membership |

**State Transitions**:

```
Authorized → Deprecated (via amendment) → Removed (after epoch passes)
```

**Invariants**:
- A class MUST NOT be consumed by S7 before it appears in `authorized_artifact_classes`.
- A deprecated class MUST continue to emit `stale` or `incompatible` signals
  until the removal epoch; it MUST NOT be silently dropped.

---

### 3. RequiredMetadataField

A metadata field that MUST be present and valid on a contracted Canon artifact
for S7 to consume it without degradation.

| Field | Type | Required | Notes |
|---|---|---|---|
| `field_name` | string | Yes | Stable field key name |
| `description` | string | Yes | Human-readable purpose |
| `example_value` | string | No | Illustrative example |

**Shared required fields for all artifact classes**:

| `field_name` | Description |
|---|---|
| `contract_line` | The governing Canon contract line or packet family line |
| `delight_provider_contract_version` | Always `delight-provider-v1` for this contract |
| `source_ref` | Canon lineage reference for the packet, finding, or promoted surface |
| `promotion_state` | Canon promotion state at time of S7 consumption |
| `schema_version` | Per-class schema version |
| `degradation_state` | One of the `CompatibilitySignal` values |

---

### 4. CompatibilitySignal

A discrete signal emitted by Canon to communicate the consumption safety state
of a governed artifact. S7 MUST act on this signal before consuming the artifact.

| Value | Meaning | Required S7 Action |
|---|---|---|
| `available` | Present, promoted, within contracted schema | Consume normally |
| `stale` | Present but promotion epoch is outdated | Surface staleness; do not fabricate certainty |
| `incompatible` | Contract line or schema version outside contracted range | Do not consume; surface incompatibility |
| `absent` | Artifact class not present in workspace | Continue with Boundline-only evidence |
| `contradicted` | Artifact contradicts a co-present canonical signal | Surface contradiction; do not merge conflicting signals |

**Invariants**:
- S7 MUST NOT treat `stale`, `incompatible`, or `contradicted` as `available`.
- S7 MUST NOT fabricate a Canon governance signal when `absent` is received.

---

### 5. ContractAmendment

A record of a proposed or accepted change to the delight-provider contract.

| Field | Type | Required | Notes |
|---|---|---|---|
| `amendment_id` | string | Yes | Sequential, e.g., `A-001` |
| `type` | enum | Yes | `add_class`, `deprecate_class`, `update_metadata`, `update_signaling` |
| `proposed_by` | string | Yes | `canon` or `boundline` |
| `status` | enum | Yes | `proposed`, `acknowledged`, `active`, `rejected` |
| `effective_contract_line` | string | Yes | Contract line in which this takes effect |
| `boundline_spec_ref` | string | No | Boundline spec location that mirrors this amendment |
| `rationale` | string | Yes | Why this amendment is needed |
| `canon_spec_ref` | string | Yes | Canon spec or decision-log entry that records this change |

**Invariants**:
- A new artifact class MUST NOT be consumed by S7 until the amendment reaches
  `active` status in both Canon and Boundline specifications.
- `rejected` amendments MUST be retained for audit; they MUST NOT be deleted.

---

### 6. DeprecatedArtifactClass

A governed artifact class that has been removed from the authorized set but
remains in the contract document pending its removal epoch.

| Field | Type | Required | Notes |
|---|---|---|---|
| `class_id` | string | Yes | Same as the `DelightArtifactClass` it replaces |
| `deprecated_in` | string | Yes | Contract line in which deprecation was declared |
| `removal_epoch` | string | Yes | Contract line at or after which the class is removed |
| `migration_guidance` | string | Yes | What S7 should use instead |
| `fallback_signal` | `CompatibilitySignal` | Yes | Signal to emit when this class is requested after deprecation |

---

## Entity Relationships

```
DelightProviderContractLine
  ├── authorized_artifact_classes: DelightArtifactClass[]
  │     ├── required_metadata: RequiredMetadataField[]
  │     └── degradation_conditions → CompatibilitySignal (enum)
  ├── deprecated_classes: DeprecatedArtifactClass[]
  └── amendments: ContractAmendment[]
```

---

## Per-Class Artifact Inventory (v1)

### `packets`

- **Source anchors**:
  - `AuthorityGovernanceV1Envelope`
  - `AdaptiveGovernanceV1Envelope`
  - `docs/integration/project-memory-promotion-contract.md`
- **Contract line**: packet family contract line, such as `authority-governance-v1` or `adaptive-governance-v1`
- **Schema version**: packet family version, initially `v1`
- **Additional required metadata**: `mode`, `source_ref`, `promotion_state`
- **Eligibility rules**: packets remain Canon-owned and MUST NOT be consumed when the governing packet is rejected or outside the contracted line.

### `approval-states`

- **Source anchors**:
  - `AuthorityApprovalState`
  - `docs/integration/governance-adapter.md`
- **Contract line**: authority-governance contract line that carries the approval state
- **Schema version**: `v1`
- **Additional required metadata**: `approval_state`, `source_ref`, `promotion_state`
- **Eligibility rules**: approval state MUST come from Canon-owned governance metadata, not downstream inference.

### `readiness-signals`

- **Source anchors**:
  - `AuthorityPacketReadiness`
  - `docs/integration/governance-adapter.md`
- **Contract line**: authority-governance contract line that carries readiness posture
- **Schema version**: `v1`
- **Additional required metadata**: `packet_readiness`, `source_ref`, `promotion_state`
- **Eligibility rules**: readiness remains Canon-owned packet posture and cannot be synthesized by S7.

### `security-findings`

- **Source anchors**:
  - `security-assessment` packet families
  - promoted `docs/security-assessments` surfaces
- **Contract line**: Canon packet family or promoted-surface contract line carrying the finding
- **Schema version**: packet family version or promoted-surface schema version
- **Additional required metadata**: `finding_kind`, `severity`, `source_ref`, `promotion_state`
- **Eligibility rules**: only Canon-authored or Canon-promoted security findings are eligible.

### `audit-findings`

- **Source anchors**:
  - `review` and `pr-review` packet families
  - `verification` packet families
  - Canon-managed review finding surfaces such as `review-summary.md`
- **Contract line**: Canon packet family or promoted-surface contract line carrying the finding
- **Schema version**: packet family version or promoted-surface schema version
- **Additional required metadata**: `finding_kind`, `disposition`, `source_ref`, `promotion_state`
- **Eligibility rules**: findings MUST come from Canon-managed review or verification artifacts.

### `promotion-references`

- **Source anchors**:
  - `promotion_refs` in `AuthorityGovernanceV1Envelope`
  - `docs/integration/project-memory-promotion-contract.md`
- **Contract line**: project-memory promotion contract line or source packet line that emits the reference
- **Schema version**: `v1`
- **Additional required metadata**: `promotion_ref`, `target_class`, `source_ref`, `promotion_state`
- **Eligibility rules**: references to `evidence` and `index` target classes remain out of scope for S7 delight.
