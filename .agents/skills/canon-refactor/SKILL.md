---
name: canon-refactor
description: Use when the user wants refactor mode context in Canon without pretending refactor is runnable end to end today.
---

# Canon Refactor

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Keep refactor visible as a Canon mode while protecting users from fake
runnable behavior.

## When To Trigger

- The user is asking for refactor mode specifically.

## When It Must Not Trigger

- The user wants a real runnable change workflow in a live codebase; use `$canon-change`.
- The user expects a real Canon run today.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `refactor` as a typed mode and keeps it discoverable.
- Explain that Canon does not yet deliver refactor-specific execution, invariants, and evidence end to end.

## Expected Output Shape

- explicit modeled-only notice
- known mode statement
- missing delivery statement
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated Canon state.

## Next-Step Guidance

- Use `$canon-change` for runnable change planning.
- Use `$canon-review` for runnable non-PR review packets, not for refactor execution.

## Related Skills

- `$canon-change`
- `$canon-review`
