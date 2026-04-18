---
name: canon-status
description: Use when a Canon run already exists and you need its current state, pending approvals, and next steps.
---

# Canon Status

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Show the current Canon-backed state of an existing run.

## When To Trigger

- The user has a run id and wants the latest run state.
- The user wants to know whether a run is blocked, completed, or waiting for approval.

## When It Must Not Trigger

- No run id is available yet.
- The user is asking to start a new Canon workflow.

## Required Inputs

- `RUN_ID`

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- If no run id was provided, resolve the latest run from Canon runtime state.
- Verify the selected run exists before invoking Canon.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon status --run <RUN_ID>`
- Return Canon-backed state only.

## Expected Output Shape

- concise run summary
- real run id
- persisted run owner when Canon already recorded one
- real run state
- direct statement of what is wrong when the run is blocked or gated
- direct result summary when Canon already returned `mode_result`
- primary artifact action or path when Canon exposed a Canon-backed primary artifact
- concrete readable artifact paths when Canon emitted them
- optional action chips mirroring the same Canon-backed next step: `Inspect evidence`, `Approve generation...`, or `Resume run` only when each is valid for the active run
- ordered possible actions, not a flat menu of skills
- one recommended next step that preserves the run context, or no mandatory next step when a completed `mode_result` is already self-explanatory
- `$canon-approve` and `$canon-resume` only when Canon reports a gated state, pending approvals, or an explicit approval target

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing or invalid `RUN_ID` and must not invent any other missing state.
- If the run id is missing or unknown, ask only for the exact run id and show the exact retry form `canon status --run <RUN_ID>`.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.

## Next-Step Guidance

- If the run is completed and Canon already returned `mode_result`, treat that result as the happy path and keep inspection optional.
- If a Canon-backed primary artifact path is available, surface opening that artifact before inspect detours.
- Use `$canon-inspect-artifacts` as the recommended next step when Canon already emitted a reviewable packet that explains the block.
- Use `$canon-inspect-evidence` when the primary need is the invocation rationale, policy decision, or evidence lineage.
- Use `$canon-inspect-invocations` only when request-level detail is the next useful step, not as a default detour.
- If the run is completed and not gated, do not suggest `$canon-approve` or `$canon-resume`.
- If the run is gated or awaiting approval, recommend inspection first unless the user has already reviewed the evidence; then point to `$canon-approve`, followed by `$canon-resume` only when Canon still requires continuation.
- If no concrete artifact paths are available from Canon-backed output, do not claim that artifacts were generated and do not recommend `$canon-inspect-artifacts` as the primary next step.
- If a host renders chips, keep them aligned to the same ordered next steps and never use `Proceed with generation` as the approval label.

## Related Skills

- `$canon-inspect-invocations`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-resume`
