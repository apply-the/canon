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

- `RISK`
- `ZONE`
- base ref
- head ref (or `WORKTREE` to review uncommitted changes)

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify risk and zone are present.
- Never infer base/head refs from the active editor file, recent `.canon/` artifacts, or any file-backed input path.
- `pr-review` does not auto-bind from `canon-input/`; it requires explicit base/head refs or `WORKTREE`.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- Verify both --ref <BASE_REF> --ref <HEAD_REF> resolve in the local Git repository.
- Canon accepts local refs plus resolved remote-tracking refs such as `origin/main`.
- If base and head refs resolve to the same commit, check for uncommitted changes with `git status --porcelain`. If uncommitted changes exist, ask with a guided choice whether to review them by using `WORKTREE` as the head ref or to provide a different head ref. If no uncommitted changes exist, report that the ref pair has no diff.
- `WORKTREE` is a valid head ref value — it tells Canon to diff the working tree against the base ref.
- If risk and/or zone are missing after the base/head pair is known, use `canon inspect risk-zone --mode pr-review --input <BASE_REF> --input <HEAD_REF>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode pr-review --risk <RISK> --zone <ZONE> [--owner <OWNER>] --input <BASE_REF> --input <HEAD_REF>`
- When reviewing uncommitted changes, use `WORKTREE` as the head ref: `canon run --mode pr-review --risk <RISK> --zone <ZONE> [--owner <OWNER>] --input <BASE_REF> --input WORKTREE`
- Return the real Canon run id, state, and any review-disposition gate Canon emits.

## Expected Output Shape

- concise result-first review summary or gated review-disposition summary
- Canon-backed run state
- direct statement of what happened or what is blocking the review
- concrete `.canon/artifacts/...` review packet paths when available
- when Canon emitted a readable review result in the run summary, treat that summary as the happy path and keep artifact inspection as drill-down
- ordered possible actions
- one recommended next step that preserves the run context

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- The ref pair flow preserves the valid side of the pair when only one ref is missing or invalid.
- If base or head ref is missing, require only the missing ref explicitly and show the exact Canon CLI form after the semantic prompt.
- If a ref is invalid, keep ref wording specific to refs and never blur it into file-path guidance.
- Canon accepts local refs plus resolved remote-tracking refs such as `origin/main` or `refs/remotes/origin/main`, and normalizes them before invocation.
- If the ref pair is malformed, ask for a distinct base/head pair and keep any normalized valid side visible in the retry guidance.
- If the ref pair collapses to the same commit and the working tree is dirty, use a guided choice between `WORKTREE` and providing a different head ref.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- If Canon returns `AwaitingApproval`, surface the exact gate target, typically `gate:review-disposition`, and do not simulate a review packet beyond Canon output.

## Next-Step Guidance

- When Canon emitted a readable review result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as drill-down into `review-summary.md` and the detailed findings.
- Use `$canon-inspect-evidence` when the user needs the lineage, request history, or policy rationale behind the review packet.
- Use `$canon-approve` only after the user has reviewed the packet or explicitly wants to record disposition.
- After approval, recommend `$canon-status` first and use `$canon-resume` only if Canon still leaves the run incomplete.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
