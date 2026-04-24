# Contract: Backlog Packet

## Summary

A backlog packet is the durable output of `backlog` mode. It must bridge bounded upstream decisions into delivery decomposition while staying above implementation-task detail and remaining credible as a standalone planning artifact.

## Successful Packet Contract

A successful backlog run must emit these artifacts under `.canon/runs/<RUN_ID>/artifacts/`:

- `backlog-overview.md`
- `epic-tree.md`
- `capability-to-epic-map.md`
- `dependency-map.md`
- `delivery-slices.md`
- `sequencing-plan.md`
- `acceptance-anchors.md`
- `planning-risks.md`

Each emitted epic, slice, dependency, and sequencing statement must remain traceable to a bounded source artifact, an explicit authored priority, or a named planning gap.

## Risk-Only Packet Contract

When closure is insufficient for credible full decomposition, the runtime must not emit a misleading full packet. In that case it may emit only a bounded risk-focused packet centered on:

- `backlog-overview.md`
- `planning-risks.md`

The overview and planning-risks outputs must make the closure weakness explicit and must not pretend that missing epics, slices, or sequencing were confidently resolved.

## Artifact Content Expectations

- `backlog-overview.md`: scope, planning horizon, source inputs, delivery intent, and decomposition posture
- `epic-tree.md`: initiative/epic/sub-epic hierarchy with clear boundaries
- `capability-to-epic-map.md`: traceability from source capabilities and decisions into backlog structure
- `dependency-map.md`: cross-epic, intra-epic, and external dependencies relevant to delivery planning
- `delivery-slices.md`: bounded vertical slices and their delivery intent without task-level decomposition
- `sequencing-plan.md`: proposed order, critical path, and safe parallelism
- `acceptance-anchors.md`: bounded planning-level signals that an epic or slice should count as complete
- `planning-risks.md`: closure gaps, oversized work items, hidden dependencies, sequencing uncertainty, and contradictory source inputs

## Publish and Inspect Expectations

- Default publish destination is `docs/planning/<RUN_ID>/`
- Published packets must remain understandable without access to internal runtime state
- `canon inspect artifacts --run <RUN_ID>` and `canon inspect evidence --run <RUN_ID>` must expose enough information to distinguish a successful full packet from a closure-limited risk-only packet
- Run lookup by display id, UUID, short id, slug, and `@last` remains unchanged

## Non-Goals Within the Packet

- no fine-grained implementation task lists
- no story points or sprint plans
- no team-capacity-based sequencing guesses
- no tool-specific ticket formatting