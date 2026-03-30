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

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command inspect-invocations --repo-root "$PWD" --require-init --run-id <RUN_ID>` first.
- Treat the shared helper output as the source of truth for run-id validation and retry behavior.

## Canon Command Contract

- Canon command: `canon inspect invocations --run <RUN_ID> --output json`

## Expected Output Shape

- concise inspection summary
- real run id
- request-level decision pointers from Canon output
- next steps pointing to `$canon-status`, `$canon-inspect-evidence`, or `$canon-approve`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The helper-enforced flow keeps the retry surface limited to `RUN_ID`.
- If the run id is missing or invalid, ask only for the exact run id and show the exact retry form `canon inspect invocations --run <RUN_ID> --output json`.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.

## Next-Step Guidance

- Use `$canon-status` for the run headline.
- Use `$canon-inspect-evidence` when you need lineage across generation and validation paths.
- Use `$canon-approve` if the inspected invocation is approval-gated.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-approve`

