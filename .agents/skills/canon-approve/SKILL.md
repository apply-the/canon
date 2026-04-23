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
- `DECISION`
- `RATIONALE`

Optional:

- `BY` when the user wants to override Git-derived approver identity explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- If no run id was provided, resolve the latest run from Canon runtime state.
- Verify the selected run exists before invoking Canon.
- `BY` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit approver input.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon approve --run <RUN_ID> --target <TARGET> [--by <BY>] --decision <DECISION> --rationale <RATIONALE>`
- `TARGET` must come from real Canon output in `gate:<kind>` or `invocation:<request-id>` form.
- Return Canon-backed approval state only.

## Expected Output Shape

- concise approval summary
- real run id
- real approval target echoed from Canon-backed workflow context
- persisted Canon approver identity and approval timestamp
- direct statement of what changed after Canon recorded the approval
- optional `Resume run` action chip only when Canon still requires continuation on the same run id
- ordered possible actions from the new Canon state
- one recommended next step pointing to `$canon-resume` when Canon accepted the approval, or the single most useful inspection or status step when it did not

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight validates run-id only; `TARGET`, `DECISION`, and `RATIONALE` remain skill-local requirements, while `BY` may be resolved from Git identity.
- If the run id is missing or invalid, ask only for the exact run id and show the exact retry form `canon approve --run <RUN_ID> --target <TARGET> [--by <BY>] --decision <DECISION> --rationale <RATIONALE>` while preserving the other approval fields in the current interaction.
- If the target is missing, require the exact `gate:<kind>` or `invocation:<request-id>` form.
- If Canon cannot resolve an approver identity, ask for `--by <BY>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- If `DECISION` or `RATIONALE` are missing, require them explicitly before invoking Canon.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.
- Never invent approval success without Canon-backed output.

## Next-Step Guidance

- Recommend `$canon-resume` first when Canon accepted the approval and continuation is still required.
- For execution-gated `implementation` and `refactor`, expect Canon to remain in `AwaitingApproval` with no remaining approval targets until `$canon-resume` consumes the post-approval continuation.
- Use `$canon-status` only if Canon did not accept the approval or if the workflow no longer needs continuation.
- If a host renders chips, preserve `DECISION` and `RATIONALE` as explicit approval inputs, resolve `BY` from Git identity when available, and never rename the approval action to `Proceed with generation`.

## Related Skills

- `$canon-resume`
- `$canon-status`
