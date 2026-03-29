---
name: canon-migration
description: Use when the user wants migration mode context in Canon without pretending the migration workflow is runnable end to end today.
---

# Canon Migration

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Expose migration as a Canon mode while preserving honest support-state
messaging.

## When To Trigger

- The user asks for migration mode specifically.

## When It Must Not Trigger

- The user expects a real Canon run today.
- The user only needs runnable change planning; use `$canon-brownfield`.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes --output json`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `migration` as a typed mode and keeps it discoverable.
- Explain that Canon does not yet deliver migration-specific execution, gates, and evidence end to end.

## Expected Output Shape

- explicit modeled-only notice
- known mode statement
- missing delivery statement
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated Canon state.

## Next-Step Guidance

- Use `$canon-brownfield` for runnable live-codebase change planning.
- Use `$canon-architecture` for modeled architecture context.

## Related Skills

- `$canon-brownfield`
- `$canon-architecture`
