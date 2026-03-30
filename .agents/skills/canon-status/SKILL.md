---
name: canon-status
description: Use when a Canon run already exists and you need its current state, pending approvals, and next steps.
---

# Canon Status

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Show the current Canon-backed state of an existing run.

## When To Trigger

- The user has a run id and wants the latest run state.
- The user wants to know whether a run is blocked, completed, or waiting for approval.

## When It Must Not Trigger

- No run id is available yet.
- The user is asking to start a new Canon workflow.

## Required Inputs

- `RUN_ID`

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command status --repo-root "$PWD" --require-init --run-id <RUN_ID>` first.
- Treat the shared helper output as the source of truth for run-id validation and retry behavior.

## Canon Command Contract

- Canon command: `canon status --run <RUN_ID> --output json`
- Return Canon-backed state only.

## Expected Output Shape

- concise run summary
- real run id
- real run state
- next steps pointing to `$canon-inspect-invocations` and `$canon-inspect-evidence`
- `$canon-approve` and `$canon-resume` only when Canon reports a gated state, pending approvals, or an explicit approval target

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The helper-enforced flow asks only for the missing or invalid `RUN_ID` and must not invent any other missing state.
- If the run id is missing or unknown, ask only for the exact run id and show the exact retry form `canon status --run <RUN_ID> --output json`.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.

## Next-Step Guidance

- Use `$canon-inspect-invocations` for request-level decisions.
- Use `$canon-inspect-evidence` for evidence lineage.
- If the run is completed and not gated, do not suggest `$canon-approve` or `$canon-resume`.
- If the run is gated or awaiting approval, use `$canon-approve` and then `$canon-resume`.

## Related Skills

- `$canon-inspect-invocations`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-resume`
