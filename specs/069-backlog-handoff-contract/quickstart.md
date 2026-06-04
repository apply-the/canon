# Quickstart: Backlog Handoff Contract

Use an isolated fixture workspace, not the Canon repository root.

## Scenario 1: Full backlog packet with handoff available

1. Prepare a bounded backlog brief and upstream source artifacts in a temporary
   workspace.
2. Run backlog mode against the fixture workspace.
3. Confirm the emitted packet contains:
   - the existing full backlog planning artifacts
   - stable `slice_id` values across slice, dependency, sequencing, and
     acceptance-anchor artifacts
   - `execution-handoff.md`
4. Publish the packet and verify an external reader can identify:
   - the selected first admissible slice
   - its implementation artifact references
   - its dependency prerequisites
   - its independent verification anchors

## Scenario 2: Full planning packet with handoff unavailable

1. Prepare a backlog brief whose slices are traceable but do not include
   explicit implementation refs or independent verification anchors.
2. Run backlog mode against the fixture workspace.
3. Confirm the emitted packet contains the full planning artifact set without
   `execution-handoff.md`.
4. Verify `backlog-overview.md` and inspect output explain why handoff is
   unavailable.

## Scenario 3: Closure-limited or risk-only packet

1. Prepare a backlog brief with unresolved closure boundaries or contradictory
   dependencies.
2. Run backlog mode against the fixture workspace.
3. Confirm Canon emits only the bounded risk-focused packet and does not emit
   `execution-handoff.md`.
4. Verify closure weaknesses remain explicit and no prose implies downstream
   execution readiness.
