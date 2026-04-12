---
name: canon-inspect-invocations
description: Use when you need request-level Canon decisions, attempts, and policy outcomes for an existing run.
---

# Canon Inspect Invocations

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Inspect request-level invocation records for a Canon run.

## When To Trigger

- The user wants to inspect what Canon allowed, denied, or constrained.
- The user wants request-level traceability rather than a high-level run summary.

## When It Must Not Trigger

- No run id is available.
- The user wants artifact file paths rather than invocation decisions.

## Required Inputs

- `RUN_ID`

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- If no run id was provided, resolve the latest run from Canon runtime state.
- Verify the selected run exists before invoking Canon.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon inspect invocations --run <RUN_ID>`

## Expected Output Shape

- concise inspection summary
- real run id
- request-level decision pointers from Canon output
- next steps pointing to `$canon-status`, `$canon-inspect-evidence`, or `$canon-approve`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight keeps the retry surface limited to `RUN_ID`.
- If the run id is missing or invalid, ask only for the exact run id and show the exact retry form `canon inspect invocations --run <RUN_ID>`.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.

## Next-Step Guidance

- Use `$canon-status` for the run headline.
- Use `$canon-inspect-evidence` when you need lineage across generation and validation paths.
- Use `$canon-approve` if the inspected invocation is approval-gated.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-approve`

