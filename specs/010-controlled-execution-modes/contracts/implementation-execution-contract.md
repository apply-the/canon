# Contract: Implementation Execution

## Summary

`implementation` becomes a governed execution mode that turns an approved bounded plan into controlled execution using existing Canon run, inspect, and publish surfaces.

## Authored Inputs

- Canonical authored input: `canon-input/implementation.md` or `canon-input/implementation/`
- Inputs are read-only and must be snapshotted immutably into `.canon/runs/<RUN_ID>/inputs/` and referenced from `context.toml`
- Required authored content:
  - explicit mutation bounds
  - plan/task linkage to prior bounded work or an authored implementation brief
  - validation intent for touched surfaces

## CLI Surface

- Start path remains `canon run --mode implementation ...`
- Existing surfaces remain authoritative for lifecycle visibility:
  - `canon status --run <RUN_ID>`
  - `canon inspect artifacts --run <RUN_ID>`
  - `canon inspect invocations --run <RUN_ID>`
  - `canon inspect evidence --run <RUN_ID>`
  - `canon publish <RUN_ID>`
- No new top-level command is introduced for basic lifecycle visibility

## Persisted Runtime Contract

- `.canon/runs/<RUN_ID>/context.toml` includes implementation-specific execution metadata
- When the authored packet is folder-backed and declares lineage markers, `.canon/runs/<RUN_ID>/context.toml` also includes provenance-only `upstream_context` metadata derived from `brief.md` and `source-map.md`
- `.canon/runs/<RUN_ID>/state.toml` continues to use existing `RunState` values
- `.canon/runs/<RUN_ID>/artifacts/` must contain:
  - `task-mapping.md`
  - `mutation-bounds.md`
  - `implementation-notes.md`
  - `completion-evidence.md`
  - `validation-hooks.md`
  - `rollback-notes.md`
- Invocation constraints must project machine-checkable mutation bounds into `allowed_paths`

## Gate and Failure Semantics

- Blocking failures:
  - missing explicit mutation bounds
  - missing plan/task linkage
  - missing focused safety-net evidence for touched existing surfaces
  - out-of-bounds adapter request
  - emitted change not mapped to bounded task/plan intent
  - missing completion evidence or rollback notes after mutation
- Recommendation-only posture is mandatory for:
  - red-zone or systemic-impact work
  - missing safety-net evidence without approved exception
  - other policy-constrained mutation cases surfaced by invocation policy

## Publish and Inspect Expectations

- Default publish destination remains `docs/implementation/<RUN_ID>/`
- Published summaries and inspect/status outputs must label recommendation-only posture explicitly when present
- `canon inspect evidence --run <RUN_ID>` should expose upstream lineage fields when the current packet provides `Feature Slice`, `Primary Upstream Mode`, `Upstream Sources`, carried-forward decisions, or excluded upstream scope
- Run lookup by display id, UUID, short id, slug, and `@last` remains unchanged
