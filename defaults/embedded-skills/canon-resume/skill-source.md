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

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- If no run id was provided, resolve the latest run from Canon runtime state.
- Verify the selected run exists before invoking Canon.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon resume --run <RUN_ID>`
- Return the resumed Canon-backed state only.

## Expected Output Shape

- concise resume summary
- real run id
- resumed state from Canon-backed output
- direct statement of whether the run completed, remained gated, or produced new material to review
- readable artifact paths when the resumed state emitted them
- optional action chips for the newly valid follow-up only, typically `Inspect evidence` before any new approval chip when no readable packet exists yet
- ordered possible actions from the resumed state
- one recommended next step, or none if the run is complete and self-explanatory

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing or invalid `RUN_ID` and keeps retry guidance scoped to that identifier.
- If the run id is missing or unknown, ask only for the exact run id and show the exact retry form `canon resume --run <RUN_ID>`.
- If Canon reports the run is still gated or stale after preflight succeeded, surface that as a Canon-execution outcome and point back to `$canon-status` or `$canon-approve`.
- Never report a resumed state that did not come from Canon.

## Next-Step Guidance

- If Canon produced new artifacts or a new review packet, recommend the most relevant inspection surface first.
- If the run is still gated, recommend the exact next blocking action rather than routing through a generic status step.
- Use `$canon-status` only when the user needs a fresh overall summary after the resumed transition.
- If a host renders chips, show only the follow-up that Canon made newly valid for the same run id.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
