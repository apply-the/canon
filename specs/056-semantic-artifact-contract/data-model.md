# Data Model: Semantic Artifact Contract

## Overview

This feature introduces a producer-owned semantic descriptor that extends the
existing Canon publication and artifact-indexing metadata surfaces. The model
does not create a new discovery path; it augments Canon-owned metadata already
carried through runtime packet metadata and adjacent publication sidecars.

## Entity: SemanticArtifactDescriptor

Represents the Canon-owned semantic metadata attached to a semantically
addressable published artifact.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `semantic_contract_line` | string | Yes | Major compatibility line for semantic parsing, initially `v1`. |
| `semantic_eligibility` | enum | Yes | Canon semantic posture for the artifact surface: `eligible` or `excluded`. |
| `semantic_provenance_boundary` | enum | Yes when eligible | Canon-owned unit of authored meaning consumers must preserve. |
| `semantic_provenance_ref` | string | Yes when eligible | Stable reference to the Canon-owned surface, managed block, or section. |
| `semantic_labels` | string[] | No | Additive descriptive labels that do not change the meaning of required fields. |
| `semantic_exclusion_reason` | string | No | Explicit reason for exclusion or rejection-compatible context. |

### Validation Rules

- `semantic_contract_line` must be recognized by the consumer or the artifact
  is rejected as unsupported.
- `semantic_eligibility = eligible` requires both
  `semantic_provenance_boundary` and `semantic_provenance_ref`.
- `semantic_eligibility = excluded` must not imply retrieval eligibility; an
  optional `semantic_exclusion_reason` should explain the exclusion.
- Additive optional fields are compatible within the current contract line.
- Removing or renaming required fields requires a new major contract line.

## Entity: SemanticEligibilityState

Defines whether Canon permits a published artifact class to participate in
semantic retrieval.

| Value | Meaning |
|-------|---------|
| `eligible` | The artifact may participate in downstream semantic retrieval if the required semantic descriptor fields are present. |
| `excluded` | The artifact remains visible or indexable for other purposes but is not part of the semantic retrieval surface. |

## Entity: SemanticProvenanceBoundary

Defines the producer-owned unit of meaning a consumer must preserve when
deriving local fragments.

| Value | Meaning |
|-------|---------|
| `surface` | The full published surface is the Canon semantic boundary. |
| `managed_block` | A Canon-managed block inside a mixed readable surface is the semantic boundary. |
| `section` | A named Canon-authored section within a readable artifact is the semantic boundary. |

### Boundary Rules

- Canon owns the declared boundary and provenance reference, not any derived
  consumer fragments.
- Consumers may derive smaller local fragments but must keep the Canon
  boundary reference attached to those fragments.
- Non-Canon content inside a mixed surface cannot inherit Canon semantic
  ownership by proximity.

## Entity: ArtifactClassSemanticPolicy

Maps an existing Canon artifact class to its semantic posture and carrier
expectations.

| Artifact Class | Carrier | Semantic Eligibility | Notes |
|----------------|---------|----------------------|-------|
| `managed-surface` | managed-surface envelope + adjacent sidecar | eligible | Uses Canon-managed blocks or the full surface as the provenance anchor. |
| `proposal-artifact` | packet-metadata sidecar | eligible | Eligible at the published proposal surface boundary. |
| `evidence-bundle` | packet-metadata sidecar | eligible | Eligible only when Canon can point to a stable evidence-facing boundary. |
| `index-surface` | packet-metadata sidecar | excluded | Visibility-only surface; consumers must not treat it as retrieval-ready content. |

## Entity: SemanticRejectionCondition

Canonical reasons a consumer must reject or decline semantic use of an
artifact.

| Condition | Trigger |
|-----------|---------|
| `excluded-artifact-class` | The artifact class is explicitly excluded from semantic retrieval. |
| `unsupported-contract-line` | `semantic_contract_line` is not recognized by the consumer. |
| `missing-required-semantic-field` | Required semantic descriptor fields are absent or empty. |
| `invalid-provenance-reference` | `semantic_provenance_ref` does not resolve to a Canon-owned boundary. |
| `non-canon-mixed-surface` | Semantic metadata is being inferred for non-Canon content inside a mixed-producer surface. |

## Relationships

- `RuntimePacketMetadata` optionally carries `artifact_indexing` and will carry
  the semantic descriptor through the same published metadata surface.
- `ArtifactIndexingMetadata` determines the artifact class and metadata carrier
  family; `SemanticArtifactDescriptor` adds retrieval-facing producer
  semantics without redefining that carrier.
- `ArtifactClassSemanticPolicy` constrains which provenance boundary values are
  valid for each artifact class.

## State And Compatibility Evolution

This feature does not introduce a runtime state machine. Compatibility evolves
through contract-line versioning:

- additive optional fields stay within the current major line
- removing or renaming required fields requires a new major line
- changing the meaning of an existing eligibility state or provenance boundary
  value is breaking and also requires a new major line