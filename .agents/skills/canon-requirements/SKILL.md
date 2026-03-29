---
name: canon-requirements
description: Use when you need a bounded requirements run in Canon before code, architecture, or execution drift starts.
---

# Canon Requirements

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Start a real Canon requirements run from Codex without making the user memorize
the raw CLI.

## When To Trigger

- The user wants bounded framing before design or implementation moves.
- The user has a problem statement or input file and needs a governed requirements run.

## When It Must Not Trigger

- The user already has a run id and wants inspection or unblock actions.
- The user is asking specifically for brownfield change or pr-review behavior.

## Required Inputs

- `OWNER`
- `RISK`
- `ZONE`
- at least one input file or note

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command requirements --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <INPUT_PATH>` first.

## Canon Command Contract

- Canon command: `canon run --mode requirements --risk <RISK> --zone <ZONE> --owner <OWNER> --input <INPUT_PATH> --output json`
- Return the real Canon run id and state only.

## Expected Output Shape

- concise run-start summary
- real run id
- real run state
- next steps pointing to `$canon-status`, `$canon-inspect-invocations`, and `$canon-inspect-evidence`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- If owner, risk, zone, or input are missing, name the missing input and show the retry form.
- Never simulate a successful run if Canon did not start one.

## Next-Step Guidance

- Use `$canon-status` for the latest state.
- Use `$canon-inspect-invocations` to inspect policy decisions.
- Use `$canon-inspect-evidence` to inspect evidence lineage.

## Related Skills

- `$canon-init`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`

