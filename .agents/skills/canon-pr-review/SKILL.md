---
name: canon-pr-review
description: Use when you need a governed Canon review of a real diff or pull-request range instead of a loose chat summary.
---

# Canon PR Review

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon pr-review workflow through a named Codex skill.

## When To Trigger

- The user wants a governed review of a real diff or base/head range.

## When It Must Not Trigger

- The user wants generic review discussion without a real diff.
- The user is asking for the modeled-only `canon-review` workflow.

## Required Inputs

- `OWNER`
- `RISK`
- `ZONE`
- base ref
- head ref

## Preflight Profile

- Run `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command pr-review --repo-root "$PWD" --require-init --owner <OWNER> --risk <RISK> --zone <ZONE> --input <BASE_REF> --input <HEAD_REF>` first.

## Canon Command Contract

- Canon command: `canon run --mode pr-review --risk <RISK> --zone <ZONE> --owner <OWNER> --input <BASE_REF> --input <HEAD_REF> --output json`
- Return the real Canon run id, state, and any review-disposition gate Canon emits.

## Expected Output Shape

- concise run-start or gated review summary
- Canon-backed run state
- guidance to `$canon-inspect-evidence` or `$canon-inspect-artifacts` when the run completes
- guidance to `$canon-approve` when Canon requires explicit review disposition
- follow-up guidance to `$canon-status`, and only `$canon-resume` if Canon still leaves the run incomplete after approval

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- If base or head ref is missing, require them explicitly.
- If Canon returns `AwaitingApproval`, surface the exact gate target, typically `gate:review-disposition`, and do not simulate a review packet beyond Canon output.

## Next-Step Guidance

- Use `$canon-status` after the run starts.
- Use `$canon-inspect-evidence` for review evidence.
- Use `$canon-inspect-artifacts` for emitted review packet paths.
- Use `$canon-approve` if review disposition is gated.
- After approval, use `$canon-status` and only `$canon-resume` if Canon still leaves the run incomplete.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
