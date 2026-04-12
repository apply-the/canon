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

- `RISK`
- `ZONE`
- at least one input file or note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify risk, zone, and --input <INPUT_PATH> are present before invoking Canon.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If risk is missing or invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is missing or invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode requirements --risk <RISK> --zone <ZONE> [--owner <OWNER>] --input <INPUT_PATH>`
- Return the real Canon run id and state only.

## Expected Output Shape

- concise run-start summary
- real run id
- real run state
- next steps pointing to `$canon-status`, `$canon-inspect-invocations`, and `$canon-inspect-evidence`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing slot and must preserve valid ownership fields inside the current interaction.
- If risk, zone, or input are missing, name the missing input, keep the already valid fields, and show the exact Canon CLI retry form after the semantic prompt.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
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

