---
name: canon-init
description: Use when a repository does not have Canon runtime state yet and you need to initialize .canon before any governed workflow.
---

# Canon Init

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Initialize `.canon/` in the current repository so Canon workflows can run and
persist durable evidence locally.

## When To Trigger

- The current repository has no `.canon/` directory yet.
- A later Canon skill needs initialization before it can proceed.

## When It Must Not Trigger

- `.canon/` already exists and the user is asking for run status or inspection.
- The user is asking for a specific workflow such as requirements or pr-review.
- The user wants initialization plus another Canon workflow in the same turn;
  initialize first and stop.

## Required Inputs

- current repository context

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command init --repo-root "$PWD"` first.
- Do not require `.canon/` to exist before initialization.

## Canon Command Contract

- Canon command: `canon init --output json`
- This skill is Canon-backed immediately and should not invent setup state.
- Execute only `canon init --output json`.
- Do not automatically start another Canon skill or `canon run` in the same turn.

## Expected Output Shape

- concise initialization summary
- whether `.canon/` was created or already present
- no run id
- no run state
- optional next-step suggestions pointing to `$canon-requirements` or `$canon-status`

## Failure Handling Guidance

- If `canon` is missing, show the install command from the shared compatibility reference.
- If the user is outside a Git repo, tell them to switch into the intended repository root before retrying.

## Next-Step Guidance

- After reporting the init result, suggest `$canon-requirements` or
  `$canon-status` only as manual next steps.
- Do not execute the follow-up skill automatically.

## Related Skills

- `$canon-requirements`
- `$canon-status`
