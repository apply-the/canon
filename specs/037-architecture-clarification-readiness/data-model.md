# Data Model: Architecture Clarification, Assumptions, And Readiness Reroute

## Entity: Architecture Clarification Question Summary

- **Purpose**: Represents one inspect-facing clarification prompt that can
  materially change an architecture recommendation, readiness posture, or
  reroute outcome.
- **Fields**:
  - `id`: stable identifier for the question
  - `prompt`: the exact question Canon surfaces
  - `rationale`: why the answer matters
  - `evidence`: authored evidence or gap context behind the question
  - `affects`: packet section or decision surface that changes if the question
    is answered differently
  - `default_if_skipped`: explicit fallback if the user defers the question
  - `status`: `required` or `optional`
- **Validation Rules**:
  - A clarification question must only exist if the answer could materially
    change the architecture decision, readiness posture, or next-mode
    recommendation.
  - Duplicate prompts must collapse into one summary.
  - Questions must remain bounded; the architecture inspect surface must cap
    the returned set.

## Entity: Working Assumption Entry

- **Purpose**: Records an explicit temporary assumption or default that keeps an
  architecture recommendation usable while remaining honest about uncertainty.
- **Fields**:
  - `statement`: the assumption itself
  - `source`: authored input, answered clarification, or defaulted fallback
  - `bounded_surface`: decision area constrained by the assumption
  - `readiness_effect`: how the assumption limits publishability or readiness
- **Validation Rules**:
  - Working assumptions must not backfill omitted canonical authored sections.
  - Assumptions derived from skipped clarification questions must remain
    visibly provisional.

## Entity: Unresolved Architecture Question

- **Purpose**: Captures a remaining architecture issue that still affects
  readiness after authored input review and any applied defaults.
- **Fields**:
  - `question`: unresolved issue text
  - `impact`: what decision or boundary remains unstable
  - `recommended_owner`: who should resolve it when known
  - `reroute_trigger`: whether the unresolved issue implies a different mode
- **Validation Rules**:
  - The unresolved question must remain visible in readiness output until it is
    actually answered or explicitly downgraded.
  - A question that triggers reroute must point to an existing Canon mode.

## Entity: Architecture Readiness Record

- **Purpose**: Represents the durable readiness output for an architecture
  packet.
- **Fields**:
  - `summary`: compact explanation of the architecture decision surface
  - `readiness_status`: current posture and explanation
  - `working_assumptions`: list of active Working Assumption Entries
  - `unresolved_questions`: list of unresolved questions still bounding the
    packet
  - `blockers`: remaining blockers before the packet is publishable or safe to
    consume downstream
  - `accepted_risks`: explicit risks carried forward
  - `recommended_next_mode`: suggested follow-up mode when the brief is not
    architecture-ready or when the packet should hand off downstream
- **Validation Rules**:
  - The readiness record must stay coherent with the inspect clarity posture.
  - `recommended_next_mode` must never name a mode Canon does not support.
  - Readiness must not be upgraded to publishable when unresolved questions or
    defaulted assumptions still materially bound the decision.

## Entity: Mode Reroute Recommendation

- **Purpose**: Encodes the reason an architecture input should return to
  `discovery`, `requirements`, or `system-shaping` before architecture mode is
  treated as ready.
- **Fields**:
  - `mode`: recommended existing downstream or upstream mode
  - `trigger`: the missing boundary or ambiguity driving the reroute
  - `rationale`: why that mode is a better fit than architecture right now
- **Validation Rules**:
  - The reroute recommendation must be derived from documented Canon mode
    boundaries.
  - The recommendation remains advisory and must not mutate run state by
    itself.

## Relationships

- One **Architecture Readiness Record** may reference many
  **Architecture Clarification Question Summaries** indirectly through
  `working_assumptions` and `unresolved_questions`.
- One **Architecture Readiness Record** owns many **Working Assumption
  Entries**.
- One **Architecture Readiness Record** owns many **Unresolved Architecture
  Questions**.
- One **Architecture Readiness Record** may own one
  **Mode Reroute Recommendation**.

## State Semantics

- A materially closed architecture brief may produce zero clarification
  questions while still emitting a readiness record.
- A structurally sufficient but ambiguous brief may remain usable if its
  assumptions and defaults are explicit.
- A non-architecture-ready brief must keep reroute guidance explicit instead of
  pretending to be architecture-ready.