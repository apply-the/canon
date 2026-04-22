---
name: canon-pr-review
description: Use when you need a governed Canon review of a real diff or pull-request range instead of a loose chat summary.
---

# Canon PR Review

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon pr-review workflow as a governed run started from
your AI assistant.

## When To Trigger

- The user wants a governed review of a real diff or base/head range.

## When It Must Not Trigger

- The user wants generic review discussion without a real diff.
- The user is asking for the file-backed `canon-review` workflow instead of a diff-backed review.

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

## AI Companion Operating Model

After preflight succeeds and the real Canon run exists, the assistant is
responsible for turning the pr-review packet from templated stubs into a
grounded, reviewable artifact set.

### Run Boundary

- Read the full diff between `BASE_REF` and `HEAD_REF` (or between `BASE_REF` and the working tree when `WORKTREE` is the head ref) before generation.
- Start the real Canon run before artifact writing so `.canon/artifacts/<RUN_ID>/pr-review/` exists and the packet stays attached to a real run id.
- Treat the source files in the diff and the working tree as read-only during the review. Do not modify, refactor, format, or rewrite any source file as part of producing the review.
- Write generated content only into Canon-managed files under `.canon/artifacts/<RUN_ID>/pr-review/`.
- Keep unanswered ambiguity explicit in the generated packet or provenance sidecar instead of quietly guessing.

### Generation Loop

1. Read the diff and the touched files in their post-change state, then extract the changed surfaces, behavioral changes, declared invariants, and any TODO, FIXME, or open questions present in the diff.
2. Start the Canon run and inspect the generated pr-review stubs under `.canon/artifacts/<RUN_ID>/pr-review/`.
3. If the diff is self-explanatory, generate the review directly. If it is structurally bounded but materially ambiguous, run the clarification loop before final generation.
4. Draft each finding, verdict, and recommendation so it is grounded in the actual diff content, in an explicit user clarification, or in a clearly marked open question.
5. Run a critique pass that challenges uncritical acceptance, hallucinated bugs not present in the diff, missing review of nontrivial change surfaces, and unsupported verdicts on changed code.
6. Overwrite the templated stubs with the revised review packet and write the provenance sidecar.

### Clarification Loop

When the diff is structurally bounded but materially ambiguous on points that
would force the assistant to invent content, ask targeted clarification
questions before generation rather than papering over the gap.

#### Question Granularity Rules

- One question per question. Never bundle multiple decisions into a single prompt with `and`, `or`, or comma-separated sub-questions.
- Each question must isolate exactly one decision the user can answer with one short response.
- If a topic naturally needs several decisions, split it into separate atomic questions and label them as a short series.
- Prefer fixed-choice options when the answer space is small and known.

#### Question Format (mandatory)

Each question must follow this shape:

```
- Question: <one-sentence question>
  - Affects: <artifact or section it changes>
  - Why it matters: <one short line on what changes if unanswered>
  - Context: <≤2 lines from the diff or touched file, or `no diff coverage`>
  - Options:
    a) <concrete option>
    b) <concrete option>
    c) <concrete option>
    d) Other (free-form)
  - Default if skipped: <explicit default>
  - Status: Required | Optional
```

#### Use Host UI Affordances When Available

- When the host surface supports rich question UI (selectable options, secondary text, tooltips, hover hints), put the `Question` line as the main prompt and surface `Why it matters` and `Context` as secondary or tooltip text rather than dumping the full block inline.
- Keep the visible question short, ideally under ~100 characters; move background, diff citations, and rationale into the secondary or tooltip slot.
- Fall back to the plain bulleted block above only when the host has no rich question UI or when the user explicitly asks for it inline.

#### Batch Size

- Ask 3 to 5 questions per round, never more than 7. If more questions exist, defer the lower-impact ones to a later round.

#### Interaction Loop

- Ask the batch.
- Wait for user answers.
- Apply answers, regenerate the affected sections, and surface any remaining open questions still required to finish the review.
- Stop the loop when no Required question is unanswered.

#### Provenance Sidecar

Write `.canon/artifacts/<RUN_ID>/pr-review/ai-provenance.md` describing how the
review packet was produced. It must include:

- The base ref, head ref (or `WORKTREE`), and the touched files that were read.
- The clarification questions that were asked, the user answers received, and any defaults that were applied because the user skipped or deferred a question.
- A `Clarification Loop` section with: number of questions presented, number of Required questions answered, number of Required questions left unresolved, number of Optional questions answered or deferred, and the number of substantive clarifications that drove material changes per artifact section.
- The critique findings that were addressed and any that were intentionally deferred.
- The generation model or assistant identity that produced the artifact set.

## Expected Output Shape

- concise result-first review summary or gated review-disposition summary
- Canon-backed run state
- direct statement of what happened or what is blocking the review
- concrete `.canon/artifacts/...` review packet paths when available
- when Canon emitted a readable review result in the run summary, treat that summary as the happy path and keep artifact inspection as drill-down
- ordered possible actions
- one recommended next step that preserves the run context
- generated content must be written only to Canon-managed run outputs such as `.canon/artifacts/<RUN_ID>/pr-review/` and `.canon/artifacts/<RUN_ID>/pr-review/ai-provenance.md`, never as in-place edits to the working tree or to any source file in the diff

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
- If the assistant has already edited or is about to edit any source file in the diff or the working tree, stop, restore the file from Git, and report the rollback before continuing.

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
