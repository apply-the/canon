---
name: canon-inspect-artifacts
description: Use when a Canon run already exists and you need the emitted artifact paths rather than run state or evidence lineage.
---

# Canon Inspect Artifacts

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose artifact paths from a real Canon run without inventing runtime state.

## When To Trigger

- The user wants emitted artifact paths for a run.

## When It Must Not Trigger

- No run id is available.
- The user is asking for request-level decisions or evidence lineage instead.

## Required Inputs

- `RUN_ID`

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- If no run id was provided, resolve the latest run from Canon runtime state.
- Verify the selected run exists before invoking Canon.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon inspect artifacts --run <RUN_ID>`
- Return emitted artifact paths from Canon output only.
- Treat only Canon-backed paths under `.canon/artifacts/<RUN_ID>/...` as valid run artifacts.

## Expected Output Shape

- concise artifact summary
- real run id
- artifact path pointers
- clear callout of the most important artifact to review first
- no run-level TOML pointers in standard user-facing output
- optional action chips that preserve the same run context, with `Approve generation...` only after packet review and `Inspect evidence` when rationale is the real next need
- ordered possible actions from the current run state
- one recommended next step that preserves the same run context, or no mandatory next step when the packet already answers the user request

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight keeps the retry surface limited to `RUN_ID`.
- If the run id is missing or invalid, ask only for the exact run id and show the exact retry form `canon inspect artifacts --run <RUN_ID>`.
- If Canon fails after preflight succeeds, report it as a Canon-execution outcome rather than a preflight failure.
- Never summarize artifacts that were not returned by Canon-backed output.
- Never elevate ad-hoc root-level notes or analysis files into the artifact summary.
- If Canon returns an empty artifact list, say that explicitly and do not imply that a packet exists somewhere else.

## Next-Step Guidance

- If the packet already answers the user request, treat inspection as complete and keep further drill-down optional.
- If the artifact packet explains an approval-gated or recommendation-only block, recommend `$canon-approve` next only after the user has reviewed it.
- Use `$canon-inspect-evidence` when the user needs the policy rationale or request lineage behind the artifact packet.
- Use `$canon-status` only to re-check overall run state after inspection, approval, or resume.
- If a host renders chips, keep `Inspect evidence` available when rationale is still missing and never render an approval chip without the real Canon target.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
