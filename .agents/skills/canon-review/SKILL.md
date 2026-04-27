---
name: canon-review
description: Use when you need a governed Canon review of a bounded non-PR change package or artifact bundle.
---

# Canon Review

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon review workflow for file-backed, non-PR review
packets, started from your AI assistant.

## When To Trigger

- The user wants a governed review of a bounded non-PR change package or artifact bundle.
- The user wants Canon to assess a review packet, not a diff range.
- The review packet usually comes downstream of `requirements`, `architecture`, `change`, or another non-PR proposal bundle when the next question is whether to accept the packet before downstream work.

## When It Must Not Trigger

- The user has a real diff, base/head range, or worktree review; use `$canon-pr-review`.
- The user already has a run id and wants status, evidence, artifacts, approval, or resume follow-up; use the run-scoped skills.

## Required Inputs

- `RISK`
- `ZONE`
- exactly one authored review packet at `canon-input/review.md` or `canon-input/review/`, or exactly one explicit inline review note via `--input-text`

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify risk, zone, and exactly one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- This mode accepts only `canon-input/review.md` or `canon-input/review/` as the runnable authored-input locations.
- When both canonical review locations exist, prefer `canon-input/review/` so Canon reads the full authored packet instead of a single file.
- For an explicit inline review note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an inline note, ask the user to author or move the review brief there, or provide exactly one inline review note; do not accept arbitrary code folders such as `src/`.
- If the user points review at a diff target, `WORKTREE`, the repo root, or another arbitrary code folder, redirect them to `$canon-pr-review` or ask for a proper review packet.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the selected review packet or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode review --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode review --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any `gate:review-disposition` target Canon emits.

## Author Review Body Before Invoking Canon

Canon does not invent the review body for you. Canon governs, validates, and
persists the packet. You (the assistant) MUST author the real review content
from the bounded source material BEFORE calling `canon run --mode review`.

Do this every time, even when the user only handed you a short approval note:

1. Read the source inputs the user pointed at. Identify the actual packet under review, the evidence already in scope, the boundary or ownership concerns, the decision impact, and the missing evidence that still matters. Do not guess.
2. If the packet is materially ambiguous, ask targeted clarification questions before writing the review brief rather than inventing content.
3. Compose the authored review packet at `canon-input/review.md` or inside `canon-input/review/`. The authored packet MUST include all of the following H2 sections, populated with concrete content tied to the source you just read:
   - `## Review Target`
   - `## Evidence Basis`
   - `## Boundary Findings`
   - `## Ownership Notes`
   - `## Missing Evidence`
   - `## Collection Priorities`
   - `## Decision Impact`
   - `## Reversibility Concerns`
   - `## Final Disposition`
   - `## Accepted Risks`
4. Keep gate-critical status lines inside the authored sections when they apply. For example, `Status: missing-evidence-open` belongs in `## Missing Evidence`, and `Status: awaiting-disposition` belongs in `## Final Disposition`.
5. Keep the packet review-focused and packet-backed. Do NOT turn it into a diff review, implementation plan, or speculative design memo.
6. Then invoke Canon. Canon will preserve the authored sections into the review packet, emit `## Missing Authored Body` when a required heading is absent, and keep `gate:review-disposition` honest instead of fabricating acceptance.

If you cannot author a credible review body because the user really has a diff, worktree, or an unbounded proposal rather than a review packet, say so directly and redirect to `$canon-pr-review` or an upstream planning mode instead of submitting an empty packet.

## Expected Output Shape

- concise result-first review summary or gated review-disposition summary
- Canon-backed run state
- direct statement of what happened or what is blocking the review
- concrete `.canon/artifacts/...` review packet paths when available
- when Canon emitted a readable review result in the run summary, treat that summary as the happy path and keep artifact inspection as drill-down
- ordered possible actions
- one recommended next step that preserves the run context
- generated content must be written only to Canon-managed run outputs such as `.canon/artifacts/<RUN_ID>/review/` and `.canon/artifacts/<RUN_ID>/review/ai-provenance.md`, never back into `canon-input/`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing slot and must preserve valid ownership fields inside the current interaction.
- If risk, zone, or input are missing, name the missing input, keep the already valid fields, and show the exact Canon CLI retry form after the semantic prompt.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If the supplied path is not `canon-input/review.md` or `canon-input/review/`, say that review accepts only the canonical review packet locations or exactly one explicit `--input-text` note and retry only that slot.
- If an explicit inline review note is empty or whitespace-only, ask only for non-empty `--input-text` content and do not restate already valid ownership metadata.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- If Canon returns `AwaitingApproval`, surface the exact gate target, typically `gate:review-disposition`, and do not simulate a completed disposition.
- If the assistant has already edited or is about to edit an authored input file under `canon-input/`, stop, restore the file from the user's last saved version, and report the rollback before continuing.

## Next-Step Guidance

- When Canon emitted a readable review result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as optional drill-down.
- Use `$canon-inspect-evidence` when the user needs provenance, policy rationale, or validation lineage behind the review packet.
- Use `$canon-approve` only after the user has inspected the packet or explicitly wants to record disposition.
- After approval, recommend `$canon-status` first and use `$canon-resume` only if Canon still leaves the run incomplete.
- Use `$canon-pr-review` instead when the real target is a diff or `WORKTREE`, not a file-backed review packet.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
- `$canon-pr-review`
