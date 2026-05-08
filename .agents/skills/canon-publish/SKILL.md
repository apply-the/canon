---
name: canon-publish
description: Use when a Canon run is complete and the user wants to publish the packet into docs or specs from chat.
---

# Canon Publish

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Publish a completed Canon run from chat without making the user remember the raw CLI contract.

## When To Trigger

- The user explicitly wants to publish a completed Canon run.
- The user wants the durable packet copied from `.canon/artifacts/` into `docs/`, `specs/`, or another visible destination.

## When It Must Not Trigger

- The user is still trying to inspect or understand a run rather than publish it.
- No run id is available and the user did not explicitly ask for the latest run.
- The user is asking to start, approve, or resume a run.

## Required Inputs

- `RUN_ID`

Optional:

- `DESTINATION` for `--to <PATH>` when the user wants an override instead of the default publish destination

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Require an explicit run identifier or an explicit latest-run intent such as `@last`; do not silently publish the latest run on the user's behalf.
- Verify the selected run exists before invoking Canon.
- If the user supplied an override path, pass it through exactly as `--to <PATH>` instead of rewriting it.
- Do not show preflight checks to the user. Report only the missing or invalid publish input.

## Canon Command Contract

- Canon command: `canon publish <RUN_ID> [--to <DESTINATION>]`
- Return Canon-backed publish results only.

## Expected Output Shape

- concise publish summary
- real run id
- real mode
- concrete publish destination
- published file paths copied by Canon
- when the mode is `requirements`, surface that the destination includes the additive `prd.md` alongside the sectional packet files

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- If `RUN_ID` is missing or unknown, ask only for the exact run id and show the exact retry form `canon publish <RUN_ID>`.
- If Canon reports that the run cannot be published yet, surface that as a Canon-backed execution outcome rather than implying that chat can bypass the gate.
- If publish fails because the run is blocked, gated, or incomplete, recommend `$canon-status` or `$canon-inspect-artifacts` instead of retrying blindly.

## Next-Step Guidance

- After a successful publish, recommend opening the published destination or reviewing the primary published packet file.
- For a published requirements run, mention `prd.md` first when the user wants one readable product-facing document.
- If the user still needs lineage or runtime context after publish, point to `$canon-inspect-artifacts` or `$canon-status`.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`