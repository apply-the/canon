# Quickstart: Controlled Execution Modes

## Goal

Exercise the planned end-to-end user paths for `implementation` and `refactor` using the existing Canon CLI surfaces and the promoted runtime contracts.

## Preconditions

- Canon CLI is installed for the workspace
- The repository contains a bounded authored input under the canonical mode-specific path
- The repository includes focused validation coverage for the touched surface, or the run is expected to remain recommendation-only

## Implementation Flow

1. Author the implementation packet at `canon-input/implementation.md` or `canon-input/implementation/`.

   Folder-backed carry-forward packets should prefer this shape when continuing prior Canon work:

   ```text
   canon-input/
     implementation/
       brief.md
       source-map.md
       selected-context.md  # optional
   ```

   Example `brief.md` excerpt:

   ```md
# Implementation Brief

Feature Slice: auth session revocation
Primary Upstream Mode: change
Task Mapping:
- Thread the revocation helper through the existing service boundary.
Mutation Bounds: src/auth/session.rs; src/auth/repository.rs
Allowed Paths:
- src/auth/session.rs
- src/auth/repository.rs
Safety-Net Evidence:
- cargo test --test session_contract
Independent Checks:
- cargo test --test session_contract
Rollback Triggers: revocation output formatting drifts.
Rollback Steps: revert the bounded auth-session patch.
   ```

   Example `source-map.md` excerpt:

   ```md
   # Source Map

   ## Upstream Sources

   - docs/changes/R-20260422-AUTHREVOC/change-surface.md
   - docs/changes/R-20260422-AUTHREVOC/implementation-plan.md

   ## Carried-Forward Decisions

   - Revocation output formatting stays stable.
   - Contract coverage must pass before and after mutation.

   ## Excluded Upstream Scope

   Login UI flow and token issuance remain out of scope.
   ```

2. Start the run:

```bash
canon run \
  --mode implementation \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-engineer \
  --input canon-input/implementation
```

3. Record the returned run id and inspect the state:

```bash
canon status --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
```

4. Verify the emitted bundle contains `task-mapping.md`, `mutation-bounds.md`, `implementation-notes.md`, `completion-evidence.md`, `validation-hooks.md`, and `rollback-notes.md`.
5. Confirm `canon inspect evidence --run <RUN_ID> --output json` exposes the folder-backed lineage under `upstream_feature_slice`, `primary_upstream_mode`, `upstream_source_refs`, `carried_forward_items`, and `excluded_upstream_scope` when those markers are authored.
6. Publish the completed packet:

```bash
canon publish <RUN_ID>
```

7. Confirm the published output lands under `docs/implementation/<RUN_ID>/` and that recommendation-only posture is explicitly labeled if the run was not allowed to mutate.

## Refactor Flow

1. Author the refactor packet at `canon-input/refactor.md` or `canon-input/refactor/`.

   Folder-backed carry-forward packets should prefer this shape when continuing prior bounded work:

   ```text
   canon-input/
     refactor/
       brief.md
       source-map.md
       selected-context.md  # optional
   ```

   Example `brief.md` excerpt:

   ```md
# Refactor Brief

Feature Slice: auth session cleanup
Primary Upstream Mode: implementation
Preserved Behavior: session revocation formatting and audit ordering remain unchanged.
Approved Exceptions: none.
Refactor Scope: auth session boundary and repository composition only.
Allowed Paths:
- src/auth/session.rs
- src/auth/repository.rs
Structural Rationale: isolate persistence concerns without changing externally meaningful behavior.
Untouched Surface: public auth API, tests/session.md, and deployment wiring stay unchanged.
Safety-Net Evidence:
- cargo test --test session_contract
Regression Findings: none.
Contract Drift: no public contract drift is allowed.
Feature Audit: no new feature behavior is introduced in this refactor packet.
Decision: preserve behavior and stop if the surface expands.
   ```

   Example `source-map.md` excerpt:

   ```md
   # Source Map

   ## Upstream Sources

   - docs/implementation/R-20260422-AUTHREVOC/implementation-notes.md
   - docs/implementation/R-20260422-AUTHREVOC/task-mapping.md

   ## Carried-Forward Invariants

   - Revocation output formatting stays stable.
   - Audit ordering remains externally unchanged.

   ## Excluded Upstream Scope

   Any session-token issuance changes remain out of scope.
   ```

2. Start the run:

```bash
canon run \
  --mode refactor \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-engineer \
  --input canon-input/refactor
```

3. Inspect lifecycle and evidence surfaces:

```bash
canon status --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
```

4. Verify the emitted bundle contains `preserved-behavior.md`, `refactor-scope.md`, `structural-rationale.md`, `regression-evidence.md`, `contract-drift-check.md`, and `no-feature-addition.md`.
5. Confirm `canon inspect evidence --run <RUN_ID> --output json` carries the same upstream lineage fields when the folder packet provides them.
6. Publish the completed packet:

```bash
canon publish <RUN_ID>
```

7. Confirm the published output lands under `docs/refactors/<RUN_ID>/` and that any recommendation-only posture or blocking drift reason is visible in summaries.

## Recommendation-Only Checks

- Run the same flows with a red-zone or systemic-impact classification and verify:
  - `status` still resolves the run normally
  - invocation inspection shows `recommendation_only = true`
  - emitted artifacts are proposals, not enacted changes
  - publish output remains visible without creating a second publish surface

## Regression Checks

- Existing modes (`requirements`, `discovery`, `system-shaping`, `architecture`, `change`, `review`, `verification`, `pr-review`) still parse, run, inspect, and publish with no identity or artifact-regression side effects.
- `mode_profiles` no longer classifies `implementation` and `refactor` as staged once the promoted runtime behavior ships.

## Walkthrough Validation Notes

- Validated with real folder-backed `implementation` and `refactor` packets against the CLI on 2026-04-23.
- `canon inspect evidence --output json` surfaced `upstream_feature_slice`, `primary_upstream_mode`, and `excluded_upstream_scope` for both flows when the packet authored those markers.
- `canon publish <RUN_ID>` wrote the existing destinations under `docs/implementation/<RUN_ID>/` and `docs/refactors/<RUN_ID>/` without requiring a parallel publish surface.
