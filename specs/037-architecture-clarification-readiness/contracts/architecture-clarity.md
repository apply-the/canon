# Contract: Architecture Clarity

## Purpose

Define how `canon inspect clarity --mode architecture` reports bounded
decision-changing questions, materially closed posture, and reroute guidance.

## Clarification Question Shape

Each architecture clarification question summary MUST include:

- `id`
- `prompt`
- `rationale`
- `evidence`
- `affects`
- `default_if_skipped`
- `status`

`status` is limited to `required` or `optional`.

## Behavioral Requirements

- Architecture clarity MUST ask only questions whose answers can materially
  change the structural recommendation, readiness posture, or next-mode
  recommendation.
- Architecture clarity MUST cap the returned question set.
- Architecture clarity MUST preserve materially closed decisions and MUST NOT
  synthesize clarification churn just to force balance.
- When the brief is under-bounded for architecture mode, architecture clarity
  MUST recommend reroute to an existing earlier mode and explain the trigger.

## Reroute Expectations

- Use `discovery` when the problem space itself is still blurry.
- Use `requirements` when the problem is bounded but framing, constraints, or
  intended outcome remain the main unresolved issue.
- Use `system-shaping` when the bounded problem is known but the capability
  structure is not ready for architecture tradeoff selection.

## Output Quality Coupling

- Missing-context findings continue to drive a `structurally-complete` posture.
- Clarification questions without missing-context findings may still yield a
  `materially-useful` posture.
- Defaulted assumptions or reroute guidance must not silently upgrade the
  posture to `publishable`.

## Non-Goals

- This contract does not define a live interview session.
- This contract does not add answer-option matrices to inspect output.
- This contract does not create a new governed mode.