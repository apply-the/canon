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
- at least one input file, input folder, or note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify risk, zone, and --input <INPUT_PATH> are present before invoking Canon.
- For auto-binding only, treat `canon-input/requirements.md` or `canon-input/requirements/` as the canonical authored-input locations for this mode.
- When both canonical requirements locations exist, prefer `canon-input/requirements/` so preflight and clarity inspect the full authored requirements surface instead of a single file.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input, ask explicitly for the input path.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the authored brief is present but underspecified, use `$canon-inspect-clarity` with `MODE=requirements` and the full authored input surface so Canon runs `canon inspect clarity --mode requirements --input <INPUT_PATH> [<INPUT_PATH> ...]` and surfaces Canon-backed missing-context findings and targeted clarification questions before starting the run.
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode requirements --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode requirements --risk <RISK> --zone <ZONE> [--owner <OWNER>] --input <INPUT_PATH>`
- Return the real Canon run id and state, plus the run's final result summary when Canon emitted a readable requirements packet.

## Expected Output Shape

- concise run summary
- real run id
- real run state
- direct statement of the requirements result when a readable packet exists
- primary artifact path and short excerpt when available
- primary artifact action when Canon exposes a Canon-backed direct-open affordance
- one recommended next step that preserves the same run context, or no mandatory next step when the requirements result is already self-explanatory

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing slot and must preserve valid ownership fields inside the current interaction.
- If risk, zone, or input are missing, name the missing input, keep the already valid fields, and show the exact Canon CLI retry form after the semantic prompt.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- Never simulate a successful run if Canon did not start one.

## Next-Step Guidance

- When the run completed and emitted a readable packet, treat the run summary itself as the happy-path result and recommend `$canon-inspect-artifacts` only as optional drill-down.
- If the authored input is still underspecified before run start, prefer `$canon-inspect-clarity` and its `canon inspect clarity --mode requirements --input <INPUT_PATH> [<INPUT_PATH> ...]` contract over generic follow-up questions.
- When Canon exposes a primary artifact action, surface that direct-open affordance before inspect detours.
- Use `$canon-inspect-evidence` when the user needs provenance, policy rationale, or denied invocation detail behind the packet.
- Use `$canon-status` to re-check the overall run state only after inspection or follow-up work.

## Related Skills

- `$canon-init`
- `$canon-inspect-clarity`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`

