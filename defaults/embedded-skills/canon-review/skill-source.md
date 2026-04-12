---
name: canon-review
description: Use when the user wants Canon review mode context without pretending the generic review workflow is runnable end to end today.
---

# Canon Review

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Keep generic review visible in the taxonomy while distinguishing it clearly
from the delivered `$canon-pr-review` workflow.

## When To Trigger

- The user asks for Canon review mode rather than a specific real diff review.

## When It Must Not Trigger

- The user has a real diff or base/head range; use `$canon-pr-review`.
- The user expects a real Canon run today.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon models `review` as a typed mode and keeps it discoverable.
- Explain that the delivered runnable diff-backed workflow today is `$canon-pr-review`, not generic `review`.

## Expected Output Shape

- explicit modeled-only notice
- distinction from `$canon-pr-review`
- what Canon knows today about review mode
- what is missing before generic review becomes runnable end to end

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never emit fabricated review packets or Canon run state.

## Next-Step Guidance

- Use `$canon-pr-review` for a real diff-backed review workflow.
- Use `$canon-inspect-evidence` if the user already has a pr-review run id.

## Related Skills

- `$canon-pr-review`
- `$canon-inspect-evidence`
