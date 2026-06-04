# Data Model: Backlog Handoff Contract

## Entities

### DeliverySliceIdentifier

- **Purpose**: Provide a stable identifier for one delivery slice across all
  backlog packet artifacts.
- **Fields**:
  - `slice_id`: canonical stable identifier, unique within one backlog packet
  - `display_name`: human-readable slice title
- **Validation Rules**:
  - `slice_id` must be present for every slice in a successful full packet
  - `slice_id` values must be unique within the packet
  - `slice_id` must remain unchanged across every artifact that references the
    same slice

### DeliverySliceEntry

- **Purpose**: Represent one bounded delivery slice together with the evidence
  needed to determine whether downstream execution handoff is credible.
- **Fields**:
  - `slice_id`: `DeliverySliceIdentifier`
  - `delivery_intent`: short description of the slice outcome
  - `source_trace_links`: upstream evidence references
  - `dependency_prerequisites`: prerequisite slice IDs or external constraints
  - `implementation_artifact_refs`: zero or more bounded implementation refs
  - `verification_anchors`: zero or more independent verification anchors
  - `handoff_availability`: derived state for this slice
- **Validation Rules**:
  - a slice may appear in a successful planning packet with
    `handoff_availability = unavailable`
  - a slice may be `available` only when implementation refs and independent
    verification anchors are both present
  - contradictory dependency prerequisites invalidate handoff availability

### ImplementationArtifactReference

- **Purpose**: Bound the implementation surface a downstream runtime is
  expected to touch for a slice.
- **Fields**:
  - `reference_label`: concise human-readable label
  - `reference_scope`: file group, component surface, artifact, or bounded code
    area
  - `reference_basis`: why this reference belongs to the slice
- **Validation Rules**:
  - references must be specific enough for a reviewer to distinguish them from
    generic areas like "backend" or "frontend"
  - references remain advisory planning evidence, not an exhaustive task list

### IndependentVerificationAnchor

- **Purpose**: Capture a slice-scoped proof target that can be checked
  independently downstream.
- **Fields**:
  - `anchor_label`: concise validation target
  - `anchor_scope`: the behavior or artifact the anchor applies to
  - `anchor_independence_basis`: why this anchor is independently checkable
- **Validation Rules**:
  - anchors must identify proof targets rather than generic encouragement to
    "review" or "test"
  - anchors must be slice-scoped and traceable to the slice's delivery intent

### ExecutionHandoffArtifact

- **Purpose**: Summarize the first execution-admissible slice and the evidence
  a downstream runtime can use to gate execution.
- **Fields**:
  - `handoff_status`: `available`, `unavailable`, or `withheld_for_closure`
  - `selected_slice_id`: present only when `handoff_status = available`
  - `selection_rationale`: why this slice is first
  - `prerequisites`: slice IDs or constraints that must already hold
  - `blocked_assumptions`: assumptions that still require downstream care
  - `verification_targets`: extracted independent verification anchors
- **Validation Rules**:
  - the artifact exists only when `handoff_status = available`
  - if closure is risk-only or blocked, the artifact must not exist
  - every `selected_slice_id` must resolve to a valid `DeliverySliceEntry`

### HandoffAvailabilityFinding

- **Purpose**: Explain why handoff is available, unavailable, or withheld.
- **Fields**:
  - `status`: `available`, `unavailable`, or `withheld_for_closure`
  - `reason_code`: stable explanation identifier
  - `summary`: concise human-readable explanation
- **Validation Rules**:
  - every full planning packet must surface at least one handoff-availability
    finding in overview or inspect output
  - withheld-for-closure findings must align with the closure assessment path

## Relationships

- One `DeliverySliceEntry` owns exactly one `DeliverySliceIdentifier`.
- One backlog packet contains many `DeliverySliceEntry` values.
- One `DeliverySliceEntry` may own zero or more `ImplementationArtifactReference`
  values.
- One `DeliverySliceEntry` may own zero or more `IndependentVerificationAnchor`
  values.
- `ExecutionHandoffArtifact` selects zero or one `DeliverySliceEntry` as the
  first downstream-admissible slice.
- `HandoffAvailabilityFinding` summarizes packet-level handoff posture and may
  cite one or more slice-level gaps.

## State Model

### Handoff Status

1. `withheld_for_closure`
   - The backlog run is risk-only or closure-limited
   - `execution-handoff.md` does not exist
2. `unavailable`
   - A full planning packet exists, but no slice has sufficient evidence for
     downstream execution handoff
   - `execution-handoff.md` does not exist
3. `available`
   - A full planning packet exists and at least one slice has stable identity,
     explicit implementation refs, dependency prerequisites, and independent
     verification anchors
   - `execution-handoff.md` exists and names the first admissible slice

## Derived Rules

- Handoff availability is stricter than successful planning-packet generation.
- Slice identity is mandatory for successful full packets, even when handoff is
  unavailable.
- A packet may be publishable and readable while still being downstream
  handoff-unavailable.
