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

## Output Rules

- Supported skills must return a concise summary plus run id and next steps.
- Inspection skills must point back to `.canon/` evidence surfaces.
- Approval and resume skills must show the exact next operation.
- Support-state skills must say the mode is not runnable when that is true.
- Skills must not paraphrase Canon runtime state when an exact Canon-backed
  summary, run id, or evidence pointer is available.
