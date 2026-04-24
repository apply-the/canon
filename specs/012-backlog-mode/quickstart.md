# Quickstart: Backlog Mode

## Goal

Exercise the planned end-to-end user paths for backlog mode using the existing Canon CLI surfaces and the new backlog packet contract.

## Preconditions

- Canon CLI is installed for the workspace
- The repository contains bounded upstream artifacts or an authored backlog brief that explicitly explains bounded planning intent
- The user can author either `canon-input/backlog.md` or `canon-input/backlog/`

## Successful Bounded Backlog Flow

1. Author the backlog packet at `canon-input/backlog.md` or `canon-input/backlog/`.

   Preferred folder-backed shape:

   ```text
   canon-input/
     backlog/
       brief.md
       priorities.md
       context-links.md
   ```

   Example `brief.md` excerpt:

   ```md
# Backlog Brief

Delivery Intent: turn the approved auth modernization architecture into bounded epics and delivery slices for staged execution.
Desired Granularity: epic-plus-slice
Planning Horizon: next two delivery increments
Source References:
- docs/architecture/decisions/R-20260420-AUTHMOD/decision-summary.md
- docs/architecture/decisions/R-20260420-AUTHMOD/capability-map.md
Constraints:
- session rollout depends on shared identity adapter readiness
Out of Scope:
- UI polish for account settings remains deferred
   ```

   Example `priorities.md` excerpt:

   ```md
# Priorities

1. stabilize identity adapter foundation
2. sequence session migration before audit cleanup
3. keep admin reporting follow-on until dependencies are closed
   ```

   Example `context-links.md` excerpt:

   ```md
# Context Links

- docs/requirements/R-20260418-AUTHMOD/requirements.md
- docs/architecture/decisions/R-20260420-AUTHMOD/decision-summary.md
- docs/architecture/decisions/R-20260420-AUTHMOD/capability-map.md
   ```

2. Start the run:

```bash
canon run \
  --mode backlog \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-engineer \
  --input canon-input/backlog
```

3. Inspect lifecycle and outputs:

```bash
canon status --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
```

4. Verify the emitted bundle contains `backlog-overview.md`, `epic-tree.md`, `capability-to-epic-map.md`, `dependency-map.md`, `delivery-slices.md`, `sequencing-plan.md`, `acceptance-anchors.md`, and `planning-risks.md`.

5. Publish the packet:

```bash
canon publish <RUN_ID>
```

6. Confirm the published output lands under `docs/planning/<RUN_ID>/` and remains understandable without reading internal run state.

## Closure-Blocked Flow

1. Author a backlog brief whose upstream architecture is incomplete or contradictory.

   Example `brief.md` excerpt:

   ```md
# Backlog Brief

Delivery Intent: split the identity rework into executable delivery phases.
Desired Granularity: epic-plus-slice
Source References:
- docs/architecture/decisions/R-20260420-AUTHMOD/decision-summary.md
Constraints:
- capability ownership and shared adapter boundaries are still unsettled
   ```

2. Start the run with the same `canon run --mode backlog ...` command.

3. Verify the result is blocked or downgraded with an explicit closure finding instead of a misleading full packet.
   Expected runtime truth:
   - blocking findings keep the run in `Blocked`
   - warning-only findings complete the run in downgraded form
   - both outcomes emit only `backlog-overview.md` and `planning-risks.md`

4. Confirm that `canon status --run <RUN_ID> --output json` exposes `closure_status`, `decomposition_scope`, and `closure_findings`, and that `canon inspect evidence --run <RUN_ID> --output json` repeats the same closure reason with a two-entry `artifact_provenance_links` list.

5. Confirm that the artifacts center on `backlog-overview.md` plus `planning-risks.md`, not a fake complete backlog decomposition.

## Regression Checks

- Existing modes still auto-bind their canonical `canon-input/<mode>.md|/` paths without regression.
- `canon publish`, `canon inspect`, `canon status`, and run lookup remain backward-compatible for all existing modes.
- Backlog packets remain above task level and never emit ticket-shaped or sprint-shaped output.