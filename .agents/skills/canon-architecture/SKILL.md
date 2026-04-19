---
name: canon-architecture
description: Use when you need a governed Canon architecture run to record decisions, tradeoffs, and risk-gated approvals.
---

# Canon Architecture

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Start a real Canon architecture run from Codex without making the user
memorize the raw CLI.

## When To Trigger

- The user is asking for architecture mode specifically.

## When It Must Not Trigger

- The user already has a run id and wants inspection, approval, or continuation actions.
- The architecture question is actually a brownfield change-planning request tied to a live bounded slice; use `$canon-brownfield`.

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
- For auto-binding only, treat `canon-input/architecture.md` or `canon-input/architecture/` as the canonical authored-input locations for this mode.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, ask explicitly for the input path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the user is starting from a short note instead of a structured brief, guide them to include explicit architectural decision focus and constraints before invoking Canon.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode architecture --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode architecture --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any approval target Canon emits.

## Expected Output Shape

- concise run-start or gated summary
- real run id
- real run state
- direct statement of the architecture result when a readable packet exists
- primary artifact path and short excerpt when available
- concrete `.canon/artifacts/...` paths when Canon emitted them
- next steps pointing to `$canon-status`, `$canon-inspect-artifacts`, and `$canon-inspect-evidence`

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
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and do not restate already valid ownership metadata.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced, typically `gate:risk`, and do not imply the run is complete.
- Never emit fabricated Canon runtime state.

## Next-Step Guidance

- When Canon emitted readable artifacts, recommend `$canon-inspect-artifacts` first.
- When Canon emitted a readable architecture result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as drill-down.
- Use `$canon-inspect-evidence` when the user needs lineage, critique rationale, or approval context behind the packet.
- Use `$canon-approve` only after the user has reviewed the packet or explicitly wants to record approval.
- After approval, recommend `$canon-status` first and use `$canon-resume` only if Canon still leaves the run incomplete.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-resume`
