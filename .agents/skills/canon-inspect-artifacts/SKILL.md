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

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command inspect-artifacts --repo-root "$PWD" --require-init --run-id <RUN_ID>` first.

## Canon Command Contract

- Canon command: `canon inspect artifacts --run <RUN_ID> --output json`
- Return emitted artifact paths from Canon output only.

## Expected Output Shape

- concise artifact summary
- real run id
- artifact path pointers
- evidence pointer back to `.canon/runs/<run-id>/evidence.toml` when useful

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- If the run id is missing, show the canonical retry form.
- Never summarize artifacts that were not returned by Canon-backed output.

## Next-Step Guidance

- Use `$canon-inspect-evidence` for linked evidence surfaces.
- Use `$canon-status` for current run state.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
