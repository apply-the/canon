# Data Model: Domain Modeling And Boundary Design

## Entity: Mode Domain Modeling Profile

- **Purpose**: captures how one target mode expresses domain-modeling outputs in this feature slice.
- **Fields**:
  - `mode_name`: one of `system-shaping`, `architecture`, `change`
  - `artifact_contract_path`: feature-level contract document for the mode
  - `method_config_path`: mode metadata path under `defaults/methods/`
  - `skill_source_path`: embedded skill source
  - `materialized_skill_path`: `.agents/skills/` mirror
  - `template_path`: starter input template
  - `example_path`: worked example input
  - `orchestrator_path`: mode-specific runtime entrypoint
  - `renderer_entrypoint`: artifact renderer branch responsible for the packet

## Entity: Bounded Context Candidate

- **Purpose**: describes a candidate business boundary surfaced by `system-shaping` or `architecture`.
- **Fields**:
  - `name`: reviewer-visible context name
  - `responsibilities`: core behaviors owned by the context
  - `owned_terms`: domain terms primarily defined by this context
  - `owners_or_stakeholders`: people or teams responsible for the context
  - `confidence_notes`: uncertainty or ambiguity attached to the proposed split
  - `core_supporting_generic_classification`: optional hypothesis about strategic importance

## Entity: Context Relationship

- **Purpose**: captures how two bounded contexts interact.
- **Fields**:
  - `source_context`: originating context
  - `target_context`: dependent or collaborating context
  - `relationship_type`: collaboration, dependency, upstream/downstream, or shared boundary
  - `integration_seam`: explicit interaction seam or handoff surface
  - `translation_need`: whether term or model translation is required
  - `anti_corruption_candidate`: whether a protective boundary is warranted
  - `risk_note`: coupling or ownership concern associated with the relationship

## Entity: Ubiquitous Language Entry

- **Purpose**: records domain vocabulary that must stay coherent across artifacts.
- **Fields**:
  - `term`: canonical domain term
  - `meaning`: intended meaning in the bounded context
  - `owning_context`: context responsible for the term
  - `conflicting_terms`: alternative or overloaded labels found in the brief
  - `alignment_question`: unresolved terminology decision, if any

## Entity: Domain Invariant Record

- **Purpose**: captures a domain rule that shaping, architecture, or change must preserve explicitly.
- **Fields**:
  - `invariant`: business rule or non-negotiable truth
  - `protected_contexts`: contexts affected by the rule
  - `violation_risk`: what fails if the invariant is broken
  - `validation_anchor`: reviewer-visible test, check, or walkthrough that confirms the invariant still holds

## Entity: Domain Slice Reference

- **Purpose**: identifies the bounded portion of the system affected by a `change` request.
- **Fields**:
  - `change_request`: the requested modification being bounded
  - `affected_contexts`: contexts directly touched by the change
  - `excluded_contexts`: nearby contexts that must remain untouched
  - `preserved_invariants`: domain invariants the change must not violate
  - `ownership_boundary`: named owner or reviewer boundary for the slice
  - `cross_context_risk`: any explicit boundary crossing or seam stress caused by the change

## Relationships

- One `Mode Domain Modeling Profile` defines the artifact surfaces for many `Bounded Context Candidate` records.
- One `Bounded Context Candidate` participates in zero or more `Context Relationship` records.
- One `Bounded Context Candidate` owns zero or more `Ubiquitous Language Entry` records.
- One `Domain Invariant Record` may protect one or many `Bounded Context Candidate` records.
- One `Domain Slice Reference` links the `change` packet to the relevant `Bounded Context Candidate`, `Context Relationship`, and `Domain Invariant Record` entries.