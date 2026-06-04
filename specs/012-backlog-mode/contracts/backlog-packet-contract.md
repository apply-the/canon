# Contract: Backlog Packet

## Summary

A backlog packet is the durable output of `backlog` mode. It must bridge bounded upstream decisions into delivery decomposition while staying above implementation-task detail, remaining credible as a standalone planning artifact, and truthfully distinguishing planning completeness from downstream execution readiness.

## Successful Packet Contract

A successful backlog run must emit this full planning packet under `.canon/runs/<RUN_ID>/artifacts/`:

- `backlog-overview.md`
- `epic-tree.md`
- `capability-to-epic-map.md`
- `dependency-map.md`
- `delivery-slices.md`
- `sequencing-plan.md`
- `acceptance-anchors.md`
- `planning-risks.md`

When one slice is credibly bounded for downstream implementation, the runtime must also emit:

- `execution-handoff.md`

Each emitted epic, slice, dependency, sequencing statement, and handoff reference must remain traceable to a bounded source artifact, an explicit authored priority, or a named planning gap.

## Risk-Only Packet Contract

When closure is insufficient for credible full decomposition, the runtime must not emit a misleading full packet. In that case it may emit only a bounded risk-focused packet centered on:

- `backlog-overview.md`
- `planning-risks.md`

The overview and planning-risks outputs must make the closure weakness explicit and must not pretend that missing epics, slices, or sequencing were confidently resolved.

## Artifact Content Expectations

- `backlog-overview.md`: scope, planning horizon, source inputs, delivery intent, and decomposition posture
- `epic-tree.md`: initiative/epic/sub-epic hierarchy with clear boundaries
- `capability-to-epic-map.md`: traceability from source capabilities and decisions into backlog structure
- `dependency-map.md`: cross-epic, intra-epic, and external dependencies relevant to delivery planning, with stable `slice_id` references where available
- `delivery-slices.md`: bounded vertical slices and their delivery intent without task-level decomposition, including stable `slice_id` values
- `sequencing-plan.md`: proposed order, critical path, and safe parallelism, including stable `slice_id` values
- `acceptance-anchors.md`: bounded planning-level signals that an epic or slice should count as complete, including stable `slice_id` values
- `planning-risks.md`: closure gaps, oversized work items, hidden dependencies, sequencing uncertainty, and contradictory source inputs
- `execution-handoff.md`: the selected bounded slice, implementation artifact references, dependency prerequisites, independent verification anchors, and the explicit execution boundary when a governed handoff is credible

## Publish and Inspect Expectations

- Default publish destination is `tech-docs/planning/<RUN_ID>/`
- Published packets must remain understandable without access to internal runtime state
- `canon inspect artifacts --run <RUN_ID>` and `canon inspect evidence --run <RUN_ID>` must expose enough information to distinguish a full packet with governed execution handoff, a full packet with handoff unavailable, and a closure-limited risk-only packet
- Run lookup by display id, UUID, short id, slug, and `@last` remains unchanged

## Non-Goals Within the Packet

- no fine-grained implementation task lists
- no story points or sprint plans
- no team-capacity-based sequencing guesses
- no tool-specific ticket formatting
