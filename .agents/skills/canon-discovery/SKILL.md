---
name: canon-discovery
description: Use when you need a governed Canon discovery run to bound a problem space before requirements or delivery planning.
---

# Canon Discovery

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Start a real Canon discovery run from Codex without making the user memorize
the raw CLI.

## When To Trigger

- The user wants discovery-oriented framing before the work is bounded enough for requirements.

## When It Must Not Trigger

- The user already has a run id and wants inspection, approval, or continuation actions.
- The user is asking specifically for system-shaping or architecture decisions.
- The user wants immediate bounded framing rather than exploratory mapping; use `$canon-requirements`.

## Required Inputs

- `RISK`
- `ZONE`
- at least one input file, input folder, or inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- For auto-binding only, treat `canon-input/discovery.md` or `canon-input/discovery/` as the canonical authored-input locations for this mode.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, ask explicitly for the input path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If the authored discovery brief is present but underspecified, use `$canon-inspect-clarity` with `MODE=discovery` so Canon runs `canon inspect clarity --mode discovery --input <INPUT_PATH>` and surfaces Canon-backed missing-context findings and targeted clarification questions before starting the run.
- Before collecting missing inputs, briefly explain the intake step, for example: `To start the discovery run I need the Canon risk level, the Canon zone, and the path to the discovery brief.`
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode discovery --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- If only the input path is missing, ask only for the input path and explain that Canon will use that note or brief as `--input` for the discovery run.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode discovery --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any approval target Canon emits.

## Expected Output Shape

- concise run-start or gated summary
- real run id
- real run state
- direct statement of the discovery result when a readable packet exists
- primary artifact path and short excerpt when available
- concrete `.canon/artifacts/...` paths when Canon emitted them
- one recommended next step by default, not a menu of multiple choices
- mention `$canon-status`, `$canon-inspect-artifacts`, or `$canon-inspect-evidence` only when they are directly relevant to the current run outcome

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight must preserve already valid fields inside the current interaction.
- If risk, zone, or input are missing, explain the missing step briefly, keep the already valid fields, and show the exact Canon CLI retry form after the prompt.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `RISK`, use the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use the exact allowed values `green`, `yellow`, and `red`.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and do not restate already valid ownership metadata.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced and do not imply the run is complete.
- Never simulate a successful run if Canon did not start one.

## Next-Step Guidance

- If the authored input is still underspecified before run start, prefer `$canon-inspect-clarity` and its `canon inspect clarity --mode discovery --input <INPUT_PATH>` contract over generic follow-up questions.
- When Canon emitted readable artifacts, recommend `$canon-inspect-artifacts` first.
- When Canon emitted a readable discovery result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as drill-down.
- Use `$canon-inspect-evidence` only when the user needs invocation lineage, approval rationale, or deeper runtime detail.
- Use `$canon-approve` only after the user has reviewed the emitted packet or explicitly wants to record approval.
- After approval, recommend `$canon-status` first and use `$canon-resume` only if Canon still leaves the run incomplete.
- Do not end with a multi-option menu by default. Give one primary recommended next action and keep any secondary actions brief.

## Related Skills

- `$canon-requirements`
- `$canon-inspect-clarity`
- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-resume`
