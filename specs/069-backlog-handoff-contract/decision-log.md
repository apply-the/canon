# Design Decision Log: Backlog Handoff Contract

## D-001: Stable downstream identity is slice-level, not task-level

- **Status**: Proposed
- **Context**: Boundline and other downstream runtimes need explicit identity
  to validate handoff evidence, but Canon backlog mode must stay above task
  decomposition.
- **Decision**: Introduce stable `slice_id` values for delivery slices.
- **Consequences**: Canon gains durable cross-artifact references without
  turning backlog packets into implementation task lists.

## D-002: Handoff uses a dedicated additive artifact

- **Status**: Proposed
- **Context**: Existing backlog artifacts are planning-first and should stay
  readable as such.
- **Decision**: Emit a separate `execution-handoff.md` artifact only when a
  slice is credible for downstream execution handoff.
- **Consequences**: Planning-packet completeness and handoff availability can
  stay distinct and auditable.

## D-003: Full planning success does not imply handoff availability

- **Status**: Proposed
- **Context**: A packet may be useful planning output even when no slice has
  sufficient implementation refs or independent verification anchors.
- **Decision**: Preserve full planning packets while explicitly surfacing
  handoff unavailability.
- **Consequences**: Canon avoids false readiness while still publishing honest
  planning artifacts.

## D-004: Handoff evidence must be independently checkable

- **Status**: Proposed
- **Context**: Downstream runtimes need proof targets rather than planner
  self-confidence.
- **Decision**: Require implementation artifact refs, dependency prerequisites,
  and independent verification anchors for any handoff-capable slice.
- **Consequences**: Downstream validators can challenge readiness without
  reverse-engineering the packet.

## D-005: Downstream runtimes keep execution authority

- **Status**: Proposed
- **Context**: Canon governs artifacts, while execution admission may depend on
  local runtime policy and environment knowledge.
- **Decision**: State explicitly that Canon emits governed handoff signals but
  does not grant execution authority.
- **Consequences**: The contract remains consumer-agnostic and respects runtime
  ownership boundaries.

## D-006: Independent review must challenge both overreach and under-specification

- **Status**: Proposed
- **Context**: This feature can fail in two different directions: Canon could
  overreach into task generation, or it could emit a handoff artifact whose
  evidence is too vague for downstream validation.
- **Decision**: The independent review pass must explicitly check that
  `execution-handoff.md` stays above task level and that its implementation
  refs and verification anchors are concrete enough for downstream use.
- **Consequences**: Validation evidence cannot stop at file existence; it must
  review the semantic quality of the handoff payload.
