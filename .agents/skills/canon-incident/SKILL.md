---
name: canon-incident
description: Use when the user wants incident or outage mode context in Canon without pretending the incident workflow is runnable end to end today.
---

# Canon Incident

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Keep incident visible as a Canon mode while preserving honest support-state
messaging for outage and response requests.

## When To Trigger

- The user asks for incident or outage workflow context specifically.
- The user wants to know what Canon supports today for incident-mode work.

## When It Must Not Trigger

- The user wants runnable live-codebase change planning; use `$canon-change`.
- The user only needs bounded problem framing; use `$canon-requirements`.
- The user expects a real Canon run, approval target, or evidence bundle today.

## Required Inputs

- incident context and scope when the user wants a more precise handoff;
	otherwise none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes`
- If incident context is provided, use it only to choose the nearest honest
	alternative.
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `incident` as a typed mode and keeps it discoverable.
- Explain that Canon does not yet deliver incident-specific execution, escalation, and evidence end to end.
- If the request has stabilized into bounded live-codebase change work, direct the
	user to `$canon-change`.
- If the incident is still too ambiguous to bound safely, direct the user to
	`$canon-requirements`.

## Expected Output Shape

- explicit modeled-only notice
- what Canon knows today about incident mode
- what is missing before incident becomes runnable end to end
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated Canon run output, approval state, or evidence bundle.

## Next-Step Guidance

- Use `$canon-change` when the incident requires bounded live-codebase change planning.
- Use `$canon-requirements` when the incident first needs bounded problem framing.

## Related Skills

- `$canon-change`
- `$canon-requirements`
