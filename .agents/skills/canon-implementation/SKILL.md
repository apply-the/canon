---
name: canon-implementation
description: Use when the user wants implementation mode context in Canon without pretending the implementation workflow is runnable end to end today.
---

# Canon Implementation

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Keep implementation visible as a Canon mode while staying honest about the
current runtime boundary.

## When To Trigger

- The user explicitly wants Canon implementation mode context.

## When It Must Not Trigger

- The user expects a real Canon run id or emitted evidence bundle today.
- The user actually needs a live-codebase workflow; use `$canon-brownfield`.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes --output json`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `implementation` as a typed mode and keeps it discoverable.
- Explain that Canon does not yet deliver implementation-specific execution, approval, and evidence loops end to end.

## Expected Output Shape

- explicit modeled-only notice
- known mode statement
- missing delivery statement
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated Canon runtime behavior.

## Next-Step Guidance

- Use `$canon-brownfield` for runnable live-codebase work.
- Use `$canon-pr-review` if the work is already on a real diff.

## Related Skills

- `$canon-brownfield`
- `$canon-pr-review`
