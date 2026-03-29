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

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command inspect-evidence --repo-root "$PWD" --require-init --run-id <RUN_ID>` first.

## Canon Command Contract

- Canon command: `canon inspect evidence --run <RUN_ID> --output json`

## Expected Output Shape

- concise evidence summary
- real run id
- evidence pointers back to `.canon/runs/<run-id>/evidence.toml` and linked surfaces
- next steps pointing to `$canon-status` or `$canon-inspect-artifacts`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- If the run id is missing or invalid, ask for the exact run id and retry form.

## Next-Step Guidance

- Use `$canon-status` for the current state of the run.
- Use `$canon-inspect-artifacts` if you need emitted file paths alongside evidence.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-invocations`

