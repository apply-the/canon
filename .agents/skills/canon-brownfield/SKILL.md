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

## When It Must Not Trigger

- The user only needs requirements framing.
- The user wants a refactor discussion for a mode that is not runnable yet.

## Required Inputs

- `OWNER`
- `RISK`
- `ZONE`
- one brownfield brief file that names the system slice, legacy invariants, change surface, validation strategy, and decision record

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command brownfield-change --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <INPUT_PATH>` first.

## Canon Command Contract

- Canon command: `canon run --mode brownfield-change --risk <RISK> --zone <ZONE> --owner <OWNER> --input <INPUT_PATH> --output json`
- Return the real Canon run id, state, and any approval target Canon emits.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- guidance to inspection skills when the run starts cleanly
- invocation-scoped approval and resume guidance when Canon enters `AwaitingApproval`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- If required brownfield inputs are missing, name them explicitly.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced and do not imply the run is complete.

## Next-Step Guidance

- Use `$canon-status` after the run starts.
- Use `$canon-inspect-evidence` and `$canon-inspect-artifacts` after the run starts.
- Use `$canon-approve` and `$canon-resume` only if Canon gates the run.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
