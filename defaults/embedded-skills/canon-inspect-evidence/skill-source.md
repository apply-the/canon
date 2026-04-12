---
name: canon-inspect-evidence
description: Use when you need the evidence bundle, lineage, and linked runtime surfaces for an existing Canon run.
---

# Canon Inspect Evidence

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Inspect the evidence Canon recorded for an existing run.

## When To Trigger

- The user wants evidence lineage rather than only artifacts.
- The user wants to inspect how generation, validation, and decisions are linked.

## When It Must Not Trigger

- No run id is available.
- The user only needs a simple completion state.

## Required Inputs

- `RUN_ID`

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- If no run id was provided, resolve the latest run from Canon runtime state.
- Verify the selected run exists before invoking Canon.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon inspect evidence --run <RUN_ID>`

## Expected Output Shape

- concise evidence summary
- real run id
- readable artifact links only
- direct statement of the most important Canon-backed takeaway from the inspection
- optional action chips that preserve the same run context, especially `Approve generation...` only when the target is already known and `Inspect evidence` remains the primary inspection affordance elsewhere
- ordered possible actions from the current run state
- one recommended next step that preserves the same run context

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight requests only the missing or invalid `RUN_ID` and does not broaden into generic conversational recovery.
- If the run id is missing or invalid, ask only for the exact run id and show the exact retry form `canon inspect evidence --run <RUN_ID>`.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.

## Next-Step Guidance

- If the run is approval-gated and Canon already emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- If the user already reviewed the packet and the exact approval target is known, recommend `$canon-approve` next.
- Use `$canon-status` only when the user needs to re-check overall run state after inspection or resume.
- If a host renders chips, do not expose `Approve generation...` until Canon already exposed the exact target for this run.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-invocations`

