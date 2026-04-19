---
name: canon-brownfield
description: Use when you need a governed brownfield change run in a live codebase where invariants and existing behavior matter.
---

# Canon Brownfield

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon brownfield-change workflow through a named Codex skill.

## When To Trigger

- The user needs a governed change plan in a live codebase.
- The user is starting a new brownfield intake from an intent, a note, or a brief.

## When It Must Not Trigger

- The user only needs requirements framing.
- The user wants a refactor discussion for a mode that is not runnable yet.
- The user is explicitly asking to inspect, approve, resume, or continue an existing run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one brownfield brief file, one brownfield input folder, one inline note, or enough starting intent to complete one through guided intake

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Treat a fresh brownfield request as a new intake. Do not infer that the latest run, latest artifact directory, or latest brief is the active request unless the user explicitly says to continue or provides a real `RUN_ID`.
- If the user gives a new change intent after guided intake questions, default to starting a new brownfield run for that intent. Do not pause to ask whether to continue an older blocked run unless the user explicitly asked to recover prior work.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- For auto-binding only, treat `canon-input/brownfield-change.md` or `canon-input/brownfield-change/` as the canonical authored-input locations for this mode.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, continue guided intake or ask explicitly for the brief path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the user already has a brownfield brief file, use it directly.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If the user has only a change intent, guide them to fill the minimum missing slots: system slice, intended change, legacy invariants, allowed or excluded change surface, and validation strategy.
- The declared change surface must stay closed enough to name affected modules, interfaces, or jobs; open-ended scope such as adjacent modules, whole-repo work, or workspace-wide edits should be treated as escalation, not normal bounded intake.
- If the intent is still too ambiguous to bound safely after guided intake, stop and redirect to `$canon-requirements` rather than guessing.
- If risk and/or zone are missing after the authored brief or guided-intake surface is known, use `canon inspect risk-zone --mode brownfield-change --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode brownfield-change --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- The skill may guide the user to complete a minimal brownfield brief before invoking Canon; do not require a fully authored brief up front.
- For a fresh request, do not summarize prior Canon runs as if they were the current attempt. Start from intake, then invoke Canon once the bounded brief is ready.
- For a fresh request, do not present `continue existing run` versus `start fresh` as a primary choice. New intake is the default path unless the user explicitly requests continuation or provides a `RUN_ID`.
- Return the real Canon run id, state, and any approval target Canon emits.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the brownfield result when a readable packet exists
- primary artifact path and short excerpt when available
- direct statement of what happened or what is blocking progress
- concrete `.canon/artifacts/...` paths when Canon emitted them
- optional action chips for the same valid next steps, typically `Inspect evidence` before `Approve generation...` when no readable packet exists yet
- ordered possible actions
- one recommended next step that keeps the run context intact

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- When the user already has a file-backed brief flow, the preflight must preserve valid ownership fields and asks only for the missing brief path or missing ownership slot.
- The preflight must preserve valid ownership fields and ask only for the missing intake slot or missing brief path.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- If the user starts from intent instead of a brief, request only the minimum missing brownfield slots rather than demanding a full document rewrite.
- If the user says they are starting from scratch, do not inspect old runs or report stale artifact state unless they explicitly ask for recovery of a previous run.
- If the user provides a fresh intent and the repo also contains older blocked runs, do not interrupt the intake with a continuation choice. Mention older runs only as optional context after the new-intake path is established.
- If a file-backed retry is required, name the missing typed slot explicitly and show the exact Canon CLI retry form after the semantic prompt.
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and keep already valid ownership or intake fields.
- If the change is still too ambiguous to bound safely, say that directly and recommend `$canon-requirements` as the honest next step.
- If the declared change surface broadens beyond a closed bounded slice, say that Canon escalated the mutation request for explicit approval instead of treating it as ordinary recommendation-only guidance.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If the input file is missing, request only the missing brief path and do not restate already valid ownership metadata.
- If an input is invalid, ask only for the failing slot rather than resetting the whole request.
- If Canon returns a failure after preflight succeeded, report it as a Canon-execution outcome, not as a preflight failure.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced and do not imply the run is complete.
- If Canon returns recommendation-only transformation guidance, say that workspace mutation is still gated in this tranche and point first to the review packet that explains the recommendation.

## Next-Step Guidance

- For a fresh request, the recommended next step is guided intake, not inspection of prior runs.
- For a fresh request, proceed through intake and start the new run once the bounded brief is ready. Do not require the user to choose between a new run and an older blocked run unless they explicitly asked for recovery.
- When Canon emitted a readable brownfield result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as drill-down.
- When the run starts cleanly, recommend `$canon-inspect-artifacts` first if Canon emitted a reviewable packet; otherwise recommend `$canon-inspect-evidence`.
- When brownfield mutation is recommendation-only or approval-gated, recommend `$canon-inspect-artifacts` first only if Canon emitted a readable packet that explains the block.
- If scope broadening triggered approval on the mutation request, recommend `$canon-inspect-artifacts` first so the user can review the bounded packet before approving the broader surface.
- If no concrete artifact paths are available yet, recommend `$canon-inspect-evidence` first, then `$canon-approve`, then `$canon-resume`.
- Use `$canon-inspect-evidence` when the user needs the invocation rationale, policy decision, or evidence lineage behind the packet, especially before generation has emitted readable artifacts.
- Use `$canon-status` to re-check the overall run state only after inspection or resume, not as a generic first stop.
- If a host renders chips, the chip order must follow the same logic and the approval chip label must remain `Approve generation...`.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
