# CLI Contract: Review And Verification Runs

## Review Run

- Command shape:

```bash
canon run --mode review --risk <RISK> --zone <ZONE> [--owner <OWNER>] --input canon-input/review.md
```

- Expectations:
  - review accepts exactly one canonical authored input at `canon-input/review.md` or `canon-input/review/`
  - review does not accept arbitrary code folders or diff-style inputs; use `pr-review` for diffs or `WORKTREE`
  - output includes a real run id, state, artifact paths, and mode result
  - review may expose `gate:review-disposition` when unresolved must-fix findings remain

## Verification Run

- Command shape:

```bash
canon run --mode verification --risk <RISK> --zone <ZONE> [--owner <OWNER>] --input <INPUT_PATH> [<INPUT_PATH> ...]
```

- Expectations:
  - inputs refer to authored files or directories outside `.canon/`
  - output includes a real run id, state, artifact paths, and mode result
  - unresolved findings block readiness through Canon gate reporting rather than a fake success state

## Shared Inspection Expectations

- `canon status --run <RUN_ID>` exposes state, artifact paths, and mode result
- `canon inspect artifacts --run <RUN_ID>` resolves concrete review or verification artifact paths
- `canon inspect evidence --run <RUN_ID>` resolves evidence lineage for the run
