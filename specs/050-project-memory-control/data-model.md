# Data Model: Project Memory And Delivery Control Contracts

## ContractBundle

- **Purpose**: Canon-owned package of stable and feature-local contract docs
  that defines producer semantics for project memory and delivery-control
  integration.
- **Fields**:
  - `owner`: `canon`
  - `stable_contract_path`
  - `feature_local_contract_paths[]`
  - `contract_major_line`
  - `known_consumers[]`
- **Relationships**:
  - contains `ManagedBlockDescriptor`, `LineageRecord`, `PromotionTarget`,
    `GovernedStageRef`, `PromotionEvent`, and `EvidenceRef` definitions

## ManagedBlockDescriptor

- **Purpose**: Defines the generated block wrapper used in repo-visible docs.
- **Fields**:
  - `marker_family`
  - `producer`
  - `source_ref`
  - `contract_version`
- **Constraints**:
  - marker family is producer-neutral
  - block boundaries must permit non-destructive refresh of generated content
  - producer-owned evidence text may vary by block producer, but Canon-only
    policy fields stay outside non-Canon blocks

## LineageRecord

- **Purpose**: Captures the minimum integrity and source metadata required for a
  promoted block or document.
- **Required fields**:
  - `contract_version`
  - `producer`
  - `source_ref`
  - `source_artifacts[]`
  - `promotion_state`
  - `promoted_at`
  - `content_digest`
- **Optional fields**:
  - `mode`
  - `stage`
  - `owner`
  - `risk`
  - `zone`
  - `approval_state`
  - `packet_readiness`
  - `promotion_profile`

## PromotionTarget

- **Purpose**: Declares where a Canon promotion outcome may land in a
  repository.
- **Fields**:
  - `path`
  - `target_class` (`stable`, `pending`, `proposal`, `evidence`, `index`)
  - `update_strategy`
  - `stable_allowed`
- **Relationships**:
  - selected by promotion policy
  - referenced by `PromotionEvent`

## GovernedStageRef

- **Purpose**: Canon-owned summary of a governed stage outcome consumed by
  Boundline.
- **Fields**:
  - `contract_version`
  - `source`
  - `run_ref`
  - `mode`
  - `state`
  - `approval_state`
  - `packet_readiness`
  - `primary_artifact`
  - `artifact_order[]`
  - `promotion_refs[]`
  - `risk`
  - `zone`

## PromotionEvent

- **Purpose**: Describes a project-memory or evidence promotion action.
- **Fields**:
  - `contract_version`
  - `source`
  - `event_type`
  - `run_ref`
  - `mode`
  - `target`
  - `strategy`
  - `promotion_state`
  - `lineage_ref`
  - `content_digest`

## EvidenceRef

- **Purpose**: Connects repo-visible evidence summaries to their authoritative
  runtime source.
- **Fields**:
  - `contract_version`
  - `source`
  - `source_ref`
  - `evidence_type`
  - `target`
  - `status`
  - `summary`
- **Constraints**:
  - attribution fields identify the producer of the evidence block
  - evidence refs do not transfer ownership of Canon promotion policy or stable
    target selection

## Relationship Notes

- `ContractBundle` is the owner-side source of truth for all other entities.
- `ManagedBlockDescriptor` and `LineageRecord` appear inside repo-visible
  documents.
- `PromotionTarget` determines where Canon may project governed knowledge.
- `GovernedStageRef`, `PromotionEvent`, and `EvidenceRef` are consumer-facing
  exchange shapes, not Canon orchestration logic.