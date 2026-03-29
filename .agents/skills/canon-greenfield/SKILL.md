---
name: canon-greenfield
description: Use when the user wants greenfield Canon mode context but the workflow is not runnable end to end yet.
---

# Canon Greenfield

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the greenfield mode honestly while it remains modeled-only.

## When To Trigger

- The user is discussing a new-system workflow and wants Canon mode guidance.

## When It Must Not Trigger

- The user expects a real Canon run or emitted evidence bundle.
- The user simply needs a runnable framing workflow; use `$canon-requirements`.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes --output json`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `greenfield` as a typed mode and keeps it discoverable.
- Explain that Canon does not yet run greenfield-specific execution and evidence end to end.

## Expected Output Shape

- explicit modeled-only notice
- known scope statement
- missing runtime delivery statement
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit a fabricated Canon run result.

## Next-Step Guidance

- Use `$canon-requirements` when the user needs a runnable starting point today.
- Use `$canon-implementation` only as modeled context, not execution.

## Related Skills

- `$canon-requirements`
- `$canon-implementation`
