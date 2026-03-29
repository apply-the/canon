# Canon Skill Output Shapes

Use these shapes as the canonical response contract for Canon skills in Codex.

## Runnable Skill: Init

- `Summary:` one sentence stating whether `.canon/` was initialized.
- `Repo Root:` current repository root.
- `Run ID:` never present.
- `State:` never present.
- `Next:` `$canon-requirements` or `$canon-status` as suggestions only, not automatic follow-up execution.

## Runnable Skill: Run Started

- `Summary:` one sentence naming the workflow and whether the run started.
- `Run ID:` real value from Canon output.
- `State:` real Canon run state if available.
- `Next:` `$canon-status`, `$canon-inspect-invocations`, `$canon-inspect-evidence`.

## Runnable Skill: Status Completed

- `Summary:` one sentence stating the run is complete.
- `Run ID:` the real run id.
- `State:` real Canon run state.
- `Next:` `$canon-inspect-invocations`, `$canon-inspect-evidence`, or `$canon-inspect-artifacts`.
- Do not suggest `$canon-approve` or `$canon-resume` when Canon reports no pending approvals and no gated state.

## Runnable Skill: Status Gated

- `Summary:` the run is blocked or awaiting approval.
- `Run ID:` the real run id.
- `State:` real Canon run state.
- `Target:` exact Canon-backed approval target when one exists.
- `Next:` `$canon-approve`, then `$canon-status` or `$canon-resume` depending on whether Canon completes on approval or requires continuation.

## Runnable Skill: Inspection

- `Summary:` one sentence naming the inspection surface.
- `Run ID:` the run being inspected.
- `Evidence:` point to `.canon/runs/<run-id>/...` or `.canon/artifacts/<run-id>/...`.
- `Next:` the nearest inspect, approve, or resume skill.

## Runnable Skill: Approval Recorded

- `Summary:` one sentence stating whether Canon recorded the approval.
- `Run ID:` the real run id.
- `Target:` exact gate or invocation target Canon acknowledged.
- `Next:` `$canon-resume` when Canon still requires continuation; otherwise `$canon-status`.

## Runnable Skill: Resumed

- `Summary:` one sentence naming the resumed workflow state.
- `Run ID:` the real run id.
- `State:` real Canon state after resume.
- `Next:` `$canon-status`, `$canon-inspect-evidence`, or `$canon-inspect-artifacts` depending on Canon output.

## Runnable Skill: Artifact Inspection

- `Summary:` one sentence naming the artifact set.
- `Run ID:` the real run id.
- `Artifacts:` concrete `.canon/artifacts/<run-id>/...` paths only.
- `Evidence:` point to `.canon/runs/<run-id>/evidence.toml` when useful.
- `Next:` `$canon-inspect-evidence` or `$canon-status`.

## Runnable Skill: Gated

- `Summary:` the run is blocked or awaiting approval.
- `Run ID:` real value from Canon output.
- `Gate:` or `Target:` exact Canon-backed approval target if available.
- `Next:` `$canon-approve`, then `$canon-status` or `$canon-resume` based on Canon output.

## Failure

- `Summary:` state the failure clearly.
- `Failure Code:` deterministic helper-layer code.
- `Action:` exact corrective step.
- Never emit a fake run id or partial Canon success summary.

## Support-State

- `Support State:` exact label.
- `Known Today:` what Canon already knows about the mode.
- `Missing:` what is required before the mode becomes runnable end to end.
- `Nearest Runnable Skill:` only if honest and useful.
- Never emit a run id, approval result, or evidence summary.

## MVP Focus

In the first shipped slice, the primary runnable shapes are:

- `canon-init`
- `canon-requirements`
- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`

## Broadened Runnable Surface

Later slices extend the same Canon-backed shapes to:

- `canon-inspect-artifacts`
- `canon-approve`
- `canon-resume`
- `canon-brownfield`
- `canon-pr-review`
