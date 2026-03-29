---
name: canon-architecture
description: Use when the user wants architecture mode context in Canon without pretending the architecture workflow is runnable end to end today.
---

# Canon Architecture

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Keep architecture visible as a first-class Canon mode while preserving honest
support-state messaging.

## When To Trigger

- The user is asking for architecture mode specifically.

## When It Must Not Trigger

- The user expects a real Canon run or evidence bundle today.
- The user is ready to start a runnable workflow; use `$canon-requirements` or `$canon-brownfield`.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes --output json`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon already models `architecture` as a typed mode with risk and zone semantics.
- Explain that Canon does not yet deliver architecture-specific execution, gates, or evidence end to end.

## Expected Output Shape

- explicit modeled-only notice
- what Canon knows today about architecture mode
- what is missing before runnable delivery
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated Canon runtime state.

## Next-Step Guidance

- Use `$canon-requirements` for runnable bounded framing.
- Use `$canon-brownfield` when the architecture question is attached to a live-codebase change.

## Related Skills

- `$canon-requirements`
- `$canon-brownfield`
