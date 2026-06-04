# Contract: Backlog Execution Handoff

## Summary

The backlog execution handoff contract extends Canon's existing backlog packet
without changing its planning-only boundary. It introduces stable slice
identity and an additive handoff artifact for downstream runtimes that need
explicit, reviewable execution-readiness signals.

## Packet Shapes

### Full Planning Packet With Handoff Available

A successful full backlog packet with at least one execution-admissible slice
must emit the existing backlog artifacts plus:

- `execution-handoff.md`

The full artifact set is therefore:

- `backlog-overview.md`
- `epic-tree.md`
- `capability-to-epic-map.md`
- `dependency-map.md`
- `delivery-slices.md`
- `sequencing-plan.md`
- `acceptance-anchors.md`
- `planning-risks.md`
- `execution-handoff.md`

### Full Planning Packet With Handoff Unavailable

When backlog decomposition is credible as a planning artifact but no slice is
credible for downstream execution handoff, Canon must emit the existing full
planning packet without `execution-handoff.md`.

In that case:

- `backlog-overview.md` must state that downstream handoff is unavailable
- the packet must explain the highest-impact reason, such as missing
  implementation refs or independent verification anchors
- inspect and publish summaries must preserve the same truth

### Risk-Only or Closure-Limited Packet

Risk-only or closure-limited packets keep the existing bounded contract:

- `backlog-overview.md`
- `planning-risks.md`

These packets must not emit `execution-handoff.md`.

## Stable Slice Identity

For every successful full planning packet:

- each delivery slice must include a stable `slice_id`
- the same `slice_id` must be reused in `delivery-slices.md`,
  `dependency-map.md`, `sequencing-plan.md`, and `acceptance-anchors.md`
- duplicate or contradictory `slice_id` usage invalidates handoff availability

## `execution-handoff.md` Content Expectations

When present, `execution-handoff.md` must include:

- the selected first execution-admissible `slice_id`
- the reason this slice is first
- explicit implementation artifact references for the selected slice
- dependency prerequisites and blocked assumptions
- independent verification anchors or proof targets for downstream validation
- a clear statement that downstream runtimes still own execution admission

## Non-Goals Within the Contract

- no implementation task checklist
- no sprint plan or staffing forecast
- no direct execution authorization from Canon
- no downstream-runtime-specific policy language

## Publish and Inspect Expectations

- Published packets must remain understandable without access to hidden runtime
  state
- Readers must be able to distinguish:
  - full packet with handoff available
  - full packet with handoff unavailable
  - risk-only or closure-limited packet
- inspect surfaces must preserve the same distinction without requiring packet
  consumers to infer it from missing files alone
