# Contract: Refactor Execution

## Summary

`refactor` becomes a governed execution mode for structural improvement that preserves externally meaningful behavior and forbids undeclared feature addition.

## Authored Inputs

- Canonical authored input: `canon-input/refactor.md` or `canon-input/refactor/`
- Inputs are read-only and must be snapshotted immutably into `.canon/runs/<RUN_ID>/inputs/` and referenced from `context.toml`
- Required authored content:
  - preserved behavior statement
  - structural rationale
  - explicit refactor scope / mutation bounds
  - validation intent for touched surfaces

## CLI Surface

- Start path remains `canon run --mode refactor ...`
- Existing lifecycle surfaces remain authoritative:
  - `canon status --run <RUN_ID>`
  - `canon inspect artifacts --run <RUN_ID>`
  - `canon inspect invocations --run <RUN_ID>`
  - `canon inspect evidence --run <RUN_ID>`
  - `canon publish <RUN_ID>`
- No new top-level refactor command is introduced

## Persisted Runtime Contract

- `.canon/runs/<RUN_ID>/context.toml` includes refactor-specific preservation metadata
- When the authored packet is folder-backed and declares lineage markers, `.canon/runs/<RUN_ID>/context.toml` also includes provenance-only `upstream_context` metadata derived from `brief.md` and `source-map.md`
- `.canon/runs/<RUN_ID>/state.toml` continues to use existing `RunState` values
- `.canon/runs/<RUN_ID>/artifacts/` must contain:
  - `preserved-behavior.md`
  - `refactor-scope.md`
  - `structural-rationale.md`
  - `regression-evidence.md`
  - `contract-drift-check.md`
  - `no-feature-addition.md`
- Invocation constraints must project machine-checkable scope bounds into `allowed_paths`

## Gate and Failure Semantics

- Blocking failures:
  - missing preserved behavior statement
  - missing structural rationale
  - missing focused safety-net evidence for touched surfaces
  - undeclared semantic drift
  - undeclared contract drift
  - feature addition or public behavior expansion without explicit approved exception
  - out-of-bounds adapter request
- Recommendation-only posture is mandatory for:
  - red-zone or systemic-impact refactor work
  - preservation evidence gaps without approved exception
  - any policy path where mutation cannot be safely recommended

## Publish and Inspect Expectations

- Default publish destination remains `docs/refactors/<RUN_ID>/`
- Published summaries and inspect/status outputs must label recommendation-only posture explicitly when present
- `canon inspect evidence --run <RUN_ID>` should expose upstream lineage fields when the current packet provides `Feature Slice`, `Primary Upstream Mode`, `Upstream Sources`, carried-forward invariants, or excluded upstream scope
- Run lookup by display id, UUID, short id, slug, and `@last` remains unchanged
