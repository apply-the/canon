# Quickstart: Review Mode Completion

## Review a non-PR package

1. Author a bounded review brief at `canon-input/review.md` or under `canon-input/review/`.

   Minimal shape:

   ```md
   # Review Brief

   Review Target: The packet or proposal being reviewed.
   Evidence Basis: The artifacts, tests, decision notes, or checks in scope.
   Owner: reviewer
   Boundary Concern: The boundary or ownership edge that must remain explicit.
   Pending Decision: The decision this review is expected to accept, reject, or defer.
   Open Concern: The gap or concern that may still require explicit disposition.
   ```

   Use this for packet review after `requirements`, `architecture`, `brownfield-change`, or another non-PR proposal bundle. Do not point `review` at `src/` or a diff target.
2. Run:

```bash
canon run \
  --mode review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input canon-input/review.md
```

3. Inspect the result:

```bash
canon status --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
```

## Verify claims and invariants

1. Author the verification target under `canon-input/verification/`.

  Minimal shape:

  ```md
  # Verification Brief

  ## Claims Under Test
  - claim 1
  - claim 2

  ## Evidence Basis
  - artifact, test, or repository surface 1
  - artifact, test, or repository surface 2

  ## Contract Surface
  - the interface or invariant that must stay true

  ## Risk Boundary
  - the contradiction or proof gap that should block readiness

  ## Challenge Focus
  - the strongest claim Canon should try to falsify first
  ```

2. Run:

```bash
canon run \
  --mode verification \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input canon-input/verification/
```

3. Inspect the result:

```bash
canon status --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
```

## Expected first-slice behavior

- `review` produces a durable review packet and may require explicit disposition approval if the packet records unresolved must-fix findings.
- `verification` produces a durable challenge packet and blocks readiness when unresolved findings remain open.
- Both modes remain inspectable through the existing Canon surfaces.
