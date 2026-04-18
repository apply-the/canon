# Canon Skill Output Shapes

Use these shapes as the canonical response contract for Canon skills in Codex.

## Optional Frontend Affordance: Action Chips

- `Action Chips:` optional host-rendered actions that mirror already-valid textual next steps.
- Each chip must expose: `id`, `label`, `skill`, `prefilled_args`, `required_user_inputs`, `visibility_condition`, and `recommended`.
- Chips are progressive enhancement only. `Possible Actions:` and `Recommended Next Step:` remain mandatory text fallback in every host.
- Use only these initial labels for governed run actions: `Approve generation...`, `Resume run`, and `Inspect evidence`.
- Never label an approval chip `Proceed with generation` or any equivalent that hides the approval decision.
- Never prefill `RUN_ID`, `TARGET`, or other chip arguments unless Canon already returned those exact values for the active run.
- `Approve generation...` requires a real Canon-backed approval target and must still collect missing approval fields such as `BY`, `DECISION`, and `RATIONALE` before invocation.
- `Resume run` is valid only when Canon still allows continuation on the same run id.
- Prefer `Inspect evidence` over approval-oriented chips when the run is gated and no readable artifact packet exists yet.

## Runnable Skill: Init

- `Summary:` one sentence stating whether `.canon/` was initialized.
- `Repo Root:` current repository root.
- `Run ID:` never present.
- `State:` never present.
- `Possible Actions:` one or two concrete follow-ups only.
- `Recommended Next Step:` `$canon-requirements` or `$canon-status` when useful, but never automatic follow-up execution.

## Runnable Skill: Classification Confirmation

- `Summary:` one sentence stating that Canon inferred a provisional risk/zone pair and needs explicit confirmation before run start.
- `Inferred Risk:` the provisional Canon-backed risk value.
- `Inferred Zone:` the provisional Canon-backed zone value.
- `Confidence:` `low`, `moderate`, or `high` from Canon.
- `Why:` short Canon-backed rationale plus the strongest signals.
- `Possible Actions:` confirm the pair, override one field, or provide both values manually.
- `Recommended Next Step:` confirm or override the inferred classification.
- Never emit a run id, approval target, or resumed state from this shape.
- Never describe this step as Canon approval. It is intake confirmation only.

## Runnable Skill: Clarity Inspection

- `Summary:` one sentence stating whether the authored inputs still require clarification.
- `Mode:` the analyzed Canon mode.
- `Source Inputs:` the exact authored paths Canon inspected.
- `Missing Context:` the highest-priority authored gaps Canon found.
- `Reasoning Signals:` the strongest Canon-backed signals behind those gaps.
- `Clarification Questions:` the prioritized follow-up questions Canon wants answered.
- `Recommended Focus:` optional. Surface Canon's top follow-up area when one exists.
- `Possible Actions:` answer the top clarification questions, refine the authored note, or continue into the relevant run-start skill when the gaps are closed.
- `Recommended Next Step:` one action only, usually answer the top question first and start the governed run second.
- Never emit a run id, approval target, or evidence bundle from this pre-run inspection shape.

## Runnable Skill: Run Started

- `Summary:` one sentence naming the workflow and whether the run started.
- `Run ID:` real value from Canon output.
- `Owner:` persisted run owner when Canon resolved and recorded one.
- `State:` real Canon run state if available.
- `What Happened:` plain-language statement of the current Canon outcome.
- If Canon returns `mode_result`, surface that result directly with the primary artifact path and excerpt before any inspect-oriented drill-down.
- `Readable Evidence:` concrete `.canon/artifacts/<run-id>/...` paths when Canon emitted them.
- `Action Chips:` optional chips only when Canon already makes the corresponding next step valid for this run.
- `Possible Actions:` ordered next moves that are valid from the current state.
- `Recommended Next Step:` one action only, preserving the run context.
- If Canon already reports a gating reason, say it directly instead of routing through `$canon-status` first.
- Never claim an artifact count or say that a review packet was generated unless you can name concrete Canon-backed artifact paths.
- For fresh-start requests, do not reuse prior runs, artifact directories, or the latest `.canon` state unless the user explicitly asked to continue an existing run or provided a real `RUN_ID`.
- If the user supplied a fresh intent and did not ask to continue prior work, do not stop the flow to ask whether to continue an older run. Default to new intake.

## Runnable Skill: Status Completed

- `Summary:` one sentence stating the run is complete.
- `Run ID:` the real run id.
- `Owner:` persisted run owner when Canon recorded one.
- `State:` real Canon run state.
- `What Happened:` direct statement that the run is complete and what surface is now worth reading.
- If Canon returns `mode_result`, treat it as the happy-path result and keep inspection as optional drill-down.
- `Readable Evidence:` concrete artifact paths when available.
- `Action Chips:` optional inspection chips only; never show approval or resume chips for a completed ungated run.
- `Possible Actions:` inspection options that make sense for the completed run.
- `Recommended Next Step:` one inspection action only.
- Do not suggest `$canon-approve` or `$canon-resume` when Canon reports no pending approvals and no gated state.

## Runnable Skill: Status Gated

- `Summary:` the run is blocked or awaiting approval.
- `Run ID:` the real run id.
- `Owner:` persisted run owner when Canon recorded one.
- `State:` real Canon run state.
- `Target:` exact Canon-backed approval target when one exists.
- `What Is Blocking:` direct diagnosis of the blocked condition.
- `Why:` plain-language Canon-backed rationale for the gate or recommendation-only state.
- `Readable Evidence:` concrete `.canon/artifacts/<run-id>/...` files only.
- `Action Chips:` optional `Inspect evidence` chip when inspection is the next honest move, plus `Approve generation...` only when Canon returned a real approval target for this run.
- `Possible Actions:` ordered options such as inspect, approve, and resume.
- `Recommended Next Step:` one action only. Prefer inspection before approval when the user has not yet seen the packet they are being asked to accept.
- If `canon inspect artifacts` returns no entries, say that explicitly and do not recommend artifact review as the primary next step.

## Runnable Skill: Inspection

- `Summary:` one sentence naming the inspection surface.
- `Run ID:` the run being inspected.
- `What You Learned:` the most consequential Canon-backed takeaway from the inspection.
- `Readable Evidence:` point to `.canon/artifacts/<run-id>/...` only.
- `Action Chips:` optional chips that preserve the same run context, such as `Inspect evidence` for lineage-first follow-up or `Approve generation...` after review when Canon already exposed the target.
- `Possible Actions:` the valid follow-ups from the current run state.
- `Recommended Next Step:` one action only, keeping the same run context.
- Never infer missing artifact paths from run summaries or expected contracts; only report paths returned by Canon-backed output.

## Runnable Skill: Approval Recorded

- `Summary:` one sentence stating whether Canon recorded the approval.
- `Run ID:` the real run id.
- `Target:` exact gate or invocation target Canon acknowledged.
- `Approved By:` persisted Canon approver identity for the recorded decision.
- `Recorded At:` persisted Canon approval timestamp for the recorded decision.
- `What Changed:` whether the approval unblocked continuation or only updated review state.
- `Action Chips:` optional `Resume run` chip only when Canon still requires continuation on the same run.
- `Possible Actions:` the valid follow-ups from the new Canon state.
- `Recommended Next Step:` `$canon-resume` when Canon still requires continuation; otherwise the single most useful inspection or status action.

## Runnable Skill: Resumed

- `Summary:` one sentence naming the resumed workflow state.
- `Run ID:` the real run id.
- `Owner:` persisted run owner when Canon recorded one.
- `State:` real Canon state after resume.
- `What Happened:` whether Canon completed, remained gated, or produced new evidence/artifacts.
- `Readable Evidence:` the most relevant `.canon/artifacts/<run-id>/...` paths for the resumed state.
- `Action Chips:` optional chips for the newly valid follow-up only; reuse `Inspect evidence` before any new approval chip when resumed output is gated without a readable packet.
- `Possible Actions:` valid follow-ups only.
- `Recommended Next Step:` one action only, or none if the run is complete and self-explanatory.

## Runnable Skill: Artifact Inspection

- `Summary:` one sentence naming the artifact set.
- `Run ID:` the real run id.
- `Artifacts:` concrete `.canon/artifacts/<run-id>/...` paths only.
- `Artifact Boundary:` do not treat repository-root notes or ad-hoc analysis files as run artifacts.
- `What To Review:` identify the most important artifact in the set.
- `Evidence:` do not expose internal run-state files in standard user-facing output.
- `Action Chips:` optional `Approve generation...` chip only after the user has a real packet to review and Canon already exposed the exact approval target.
- `Possible Actions:` valid follow-ups from the current run state.
- `Recommended Next Step:` one action only.
- If the artifact list is empty, say so directly and recommend `$canon-inspect-evidence` or `$canon-status` instead of pretending a packet exists.

## Runnable Skill: Gated

- `Summary:` the run is blocked or awaiting approval.
- `Run ID:` real value from Canon output.
- `Gate:` or `Target:` exact Canon-backed approval target if available.
- `What Is Blocking:` the exact blocked condition.
- `Readable Evidence:` concrete artifact paths only.
- `Action Chips:` optional `Inspect evidence` first, with `Approve generation...` only after the gating target is known and the user has enough Canon-backed context to act.
- `Possible Actions:` ordered valid follow-ups.
- `Recommended Next Step:` one action only, usually inspect first and approve second unless the user has already reviewed the evidence.
- Do not treat stale repo state or previous runs as the active run for a new request.
- At most, mention a possibly related older run as optional context after the fresh path is already clear; never make it the primary decision point for a new request.

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
- `canon-inspect-clarity`
- `canon-approve`
- `canon-resume`
- `canon-brownfield`
- `canon-pr-review`
