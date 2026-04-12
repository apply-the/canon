# Contract: Canon Skill Contract

## Purpose

Define the minimum repo-local contract every Canon skill must satisfy.

## Skill Classes

### Executable Wrapper

- Drives a real Canon CLI command
- May create or mutate Canon runtime state
- Must return Canon-backed status and next steps

### Support-State Wrapper

- Does not start unsupported Canon modes
- May read shared references or use `canon inspect modes`
- Must return honest support-state messaging and no fabricated run id

## Required Fields in `SKILL.md`

- frontmatter `name`
- frontmatter `description`
- `name`
- `description`
- `support state`
- `default visibility`
- `purpose`
- `when to trigger`
- `when it must not trigger`
- `required inputs`
- `preflight profile`
- `Canon command contract`
- `expected output shape`
- `failure handling guidance`
- `next-step guidance`
- `related skills`

## Command Contract Rules

- Supported skills invoke `canon` directly or through shared deterministic
  helpers that do not replace Canon logic.
- Support-state skills must not invoke `canon run` for unsupported modes.
- Support-state skills may reference `canon inspect modes --output json` only
  to confirm Canon knows about the mode taxonomy; this does not make the mode
  runnable.
- Any displayed run id must come from real Canon runtime output.
- Any approval guidance must refer to the real Canon approval target contract.
- Any action-chip prefilled argument must come from real Canon runtime output
  for the active run.

## Output Rules

- Supported skills must return a concise summary plus run id and next steps.
- Supported skills may expose optional structured `Action Chips`, but only as
  progressive enhancement over the required textual next-step contract.
- Inspection skills must point back to `.canon/` evidence surfaces.
- Approval and resume skills must show the exact next operation.
- Support-state skills must say the mode is not runnable when that is true.
- Skills must not paraphrase Canon runtime state when an exact Canon-backed
  summary, run id, or evidence pointer is available.
- `Possible Actions:` and `Recommended Next Step:` remain mandatory even when a
  host can render chips.
- `Approve generation...` may appear only when Canon returned a real approval
  target for the active run, and it must still preserve required approval
  fields such as `BY`, `DECISION`, and `RATIONALE`.
- `Resume run` may appear only when Canon still allows continuation on the
  same run id.
- `Inspect evidence` should be preferred over approval-oriented chips when the
  run is gated and no readable artifact packet exists yet.
- Skills must never label a governed approval action as `Proceed with
  generation` or any wording that hides the decision boundary.
