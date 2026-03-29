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

## Canon Command Contract

- Canon command: `canon resume --run <RUN_ID>`
- Return the resumed Canon-backed state only.

## Expected Output Shape

- concise resume summary
- real run id
- resumed state from Canon-backed output
- next step pointing to `$canon-status`, `$canon-inspect-evidence`, or `$canon-inspect-artifacts` depending on the resumed outcome

## Failure Handling Guidance

- If the run id is missing or unknown, ask for the exact run id.
- If Canon reports the run is still gated or stale, surface that state and point back to `$canon-status` or `$canon-approve`.
- Never report a resumed state that did not come from Canon.

## Next-Step Guidance

- Use `$canon-status` for the resumed run.
- Use `$canon-inspect-evidence` to review updated evidence.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
