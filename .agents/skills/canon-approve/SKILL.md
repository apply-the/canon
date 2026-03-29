---
name: canon-approve
description: Use when Canon has blocked or gated a real run and you need to record an explicit approval against the actual runtime target.
---

# Canon Approve

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Record an explicit approval against a real Canon gate or invocation target.

## When To Trigger

- The user has a real run id and approval target from Canon output.

## When It Must Not Trigger

- No run id is available.
- No real Canon-backed target is available yet.

## Required Inputs

- `RUN_ID`
- `TARGET`
- `BY`
- `DECISION`
- `RATIONALE`

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command approve --repo-root "$PWD" --require-init --run-id <RUN_ID>` first.

## Canon Command Contract

- Canon command: `canon approve --run <RUN_ID> --target <TARGET> --by <BY> --decision <DECISION> --rationale <RATIONALE>`
- `TARGET` must come from real Canon output in `gate:<kind>` or `invocation:<request-id>` form.
- Return Canon-backed approval state only.

## Expected Output Shape

- concise approval summary
- real run id
- real approval target echoed from Canon-backed workflow context
- next step pointing to `$canon-resume` when Canon accepted the approval, or `$canon-status` when the run still needs inspection

## Failure Handling Guidance

- If the target is missing, require the exact `gate:<kind>` or `invocation:<request-id>` form.
- If `BY`, `DECISION`, or `RATIONALE` are missing, require them explicitly before invoking Canon.
- Never invent approval success without Canon-backed output.

## Next-Step Guidance

- Use `$canon-resume` after Canon records the approval successfully.
- Use `$canon-status` if the user needs the latest run state first or if Canon did not accept the approval.

## Related Skills

- `$canon-resume`
- `$canon-status`
