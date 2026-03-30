---
name: canon-resume
description: Use when a real Canon run has been unblocked and you need to continue it from recorded runtime state.
---

# Canon Resume

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Continue an existing Canon run after approval or after the blocked condition was addressed.

## When To Trigger

- A real Canon run exists and is ready to continue.

## When It Must Not Trigger

- No run id is available.
- The user is asking to start a new run rather than continue an existing one.

## Required Inputs

- `RUN_ID`

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command resume --repo-root "$PWD" --require-init --run-id <RUN_ID>` first.
- Treat the shared helper output as the source of truth for run-id validation and retry behavior.

## Canon Command Contract

- Canon command: `canon resume --run <RUN_ID>`
- Return the resumed Canon-backed state only.

## Expected Output Shape

- concise resume summary
- real run id
- resumed state from Canon-backed output
- next step pointing to `$canon-status`, `$canon-inspect-evidence`, or `$canon-inspect-artifacts` depending on the resumed outcome

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The helper-enforced flow asks only for the missing or invalid `RUN_ID` and keeps retry guidance scoped to that identifier.
- If the run id is missing or unknown, ask only for the exact run id and show the exact retry form `canon resume --run <RUN_ID>`.
- If Canon reports the run is still gated or stale after preflight succeeded, surface that as a Canon-execution outcome and point back to `$canon-status` or `$canon-approve`.
- Never report a resumed state that did not come from Canon.

## Next-Step Guidance

- Use `$canon-status` for the resumed run.
- Use `$canon-inspect-evidence` to review updated evidence.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
