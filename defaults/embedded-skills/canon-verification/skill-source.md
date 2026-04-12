---
name: canon-verification
description: Use when the user asks for Canon verification mode and needs an explicit explanation of the current intentionally limited boundary.
---

# Canon Verification

## Support State

- `intentionally-limited`
- `default visibility`: `discoverable-standard`

## Purpose

Keep verification discoverable while making the current product boundary
obvious.

## When To Trigger

- The user asks for Canon verification mode specifically.

## When It Must Not Trigger

- The user expects a runnable verification workflow today.
- The user already has a run id and simply needs evidence inspection; use `$canon-inspect-evidence`.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes`
- Do not start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that the CLI surface exists but the verification workflow remains intentionally limited in the current release.
- Explain that Canon can still expose evidence inspection today, but not a full `verify` workflow.

## Expected Output Shape

- explicit intentionally-limited notice
- current boundary statement
- what is missing before full verification becomes runnable
- nearest useful alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Never fabricate verification output or Canon runtime state.

## Next-Step Guidance

- Use `$canon-inspect-evidence` to inspect current evidence.
- Use `$canon-pr-review` for the delivered review workflow.

## Related Skills

- `$canon-inspect-evidence`
- `$canon-pr-review`
