---
name: canon-discovery
description: Use when you are exploring a problem space and want Canon mode context without pretending discovery is runnable end to end today.
---

# Canon Discovery

## Support State

- `modeled-only`
- `default visibility`: `discoverable-standard`

## Purpose

Keep the discovery mode visible in Codex without pretending Canon can run it
end to end yet.

## When To Trigger

- The user wants discovery-oriented framing before the work is bounded enough for requirements.

## When It Must Not Trigger

- The user is ready for a real runnable framing workflow; use `$canon-requirements` then.
- The user expects a real Canon run id or emitted evidence bundle.

## Required Inputs

- none required for support-state guidance

## Preflight Profile

- Optional taxonomy confirmation: `canon inspect modes`
- Do not require or start a Canon run.

## Canon Command Contract

- Support-state only.
- Explain that Canon knows `discovery` as a typed mode in its mode catalog and Codex taxonomy.
- Explain that Canon does not yet deliver discovery-specific execution, gates, or artifacts end to end.

## Expected Output Shape

- explicit modeled-only notice
- what Canon already knows about discovery
- what is missing before discovery becomes runnable
- nearest honest alternative

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- Do not emit any Canon run summary.

## Next-Step Guidance

- Use `$canon-requirements` when the user wants a runnable bounded framing workflow.
- Use `$canon-architecture` only for modeled mode context, not execution.

## Related Skills

- `$canon-requirements`
- `$canon-architecture`
