---
name: canon-incident
description: Use when the user wants incident mode context in Canon without pretending the incident workflow is runnable end to end today.
---

# Canon Incident

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Expose incident as a Canon mode while keeping support-state honesty explicit.

## When To Trigger

- The user asks for incident mode specifically.

## When It Must Not Trigger

- The user expects a runnable Canon run today.
- The user only needs bounded problem framing; use `$canon-requirements`.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes --output json`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `incident` as a typed mode and keeps it discoverable.
- Explain that Canon does not yet deliver incident-specific execution, escalation, and evidence end to end.

## Expected Output Shape

- explicit modeled-only notice
- known mode statement
- missing delivery statement
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated Canon run output.

## Next-Step Guidance

- Use `$canon-brownfield` when the incident requires bounded live-codebase change planning.
- Use `$canon-requirements` when the first need is problem framing.

## Related Skills

- `$canon-brownfield`
- `$canon-requirements`
