---
name: canon-verification
description: Use when you need a governed Canon verification run to challenge claims, invariants, contracts, or evidence directly.
---

# Canon Verification

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon verification workflow for file-backed challenge
packets, started from your AI assistant.

## When To Trigger

- The user wants Canon to challenge claims, invariants, contracts, or evidence directly.
- The user wants a governed verification packet rather than a diff review or a generic review packet.

## When It Must Not Trigger

- The user wants a diff-backed review or worktree review; use `$canon-pr-review`.
- The user wants a file-backed non-PR review packet rather than a challenge workflow; use `$canon-review`.
- The user already has a run id and simply needs status, artifacts, or evidence inspection; use the run-scoped skills.

## Required Inputs

- `RISK`
- `ZONE`
- at least one input file, input folder, or inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- For auto-binding only, treat `canon-input/verification.md` or `canon-input/verification/` as the canonical authored-input locations for this mode.
- When both canonical verification locations exist, prefer `canon-input/verification/` so Canon reads the full authored packet instead of a single file.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, ask explicitly for the input path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode verification --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode verification --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked readiness finding Canon emits.

## Author Verification Body Before Invoking Canon

Canon does not invent the verification body for you. Canon governs, validates,
and persists the packet. You (the assistant) MUST author the real verification
content from the bounded source material BEFORE calling
`canon run --mode verification`.

Do this every time, even when the user only handed you a short challenge note:

1. Read the source inputs the user pointed at. Identify the actual claims under test, the invariants and contract assumptions that matter, the strongest contradictions, the currently supported or rejected claims, and any unresolved follow-up. Do not guess.
2. If the packet is materially ambiguous, ask targeted clarification questions before writing the verification brief rather than inventing content.
3. Compose the authored verification packet at `canon-input/verification.md` or inside `canon-input/verification/`. The authored packet MUST include all of the following H2 sections, populated with concrete content tied to the source you just read:
   - `## Claims Under Test`
   - `## Invariant Checks`
   - `## Contract Assumptions`
   - `## Verification Outcome`
   - `## Challenge Findings`
   - `## Contradictions`
   - `## Verified Claims`
   - `## Rejected Claims`
   - `## Overall Verdict`
   - `## Open Findings`
   - `## Required Follow-Up`
4. Keep gate-critical status lines inside the authored sections when they apply. For example, `Status: unsupported` belongs in `## Overall Verdict`, and `Status: unresolved-findings-open` belongs in `## Open Findings`.
5. Keep the packet challenge-focused. Do NOT turn it into a generic review memo, a diff review, or a speculative remediation plan.
6. Then invoke Canon. Canon will preserve the authored sections into the verification packet, emit `## Missing Authored Body` when a required heading is absent, and keep unsupported or unresolved packets honestly blocked instead of fabricating confidence.

If you cannot author a credible verification body because the source is really a generic review packet, a diff/worktree review, or an unbounded claim set, say so directly and redirect to `$canon-review`, `$canon-pr-review`, or an upstream planning mode instead of submitting an empty packet.

### Packet Shape And Persona

Author the packet as an adversarial verifier challenging claims for decision
makers.

- Favor contradictions, unsupported assumptions, and clear verdict language
   over generic review prose.
- Keep conclusions bounded to the authored claims and evidence under test; do
   not drift into design or implementation planning.
- Persona guidance is presentation only. Missing authored sections or
   unresolved findings remain explicit gaps and must not be backfilled with
   certainty.

## Expected Output Shape

- concise result-first verification summary or blocked-readiness summary
- Canon-backed run state
- direct statement of what happened or what is blocking verification
- concrete `.canon/artifacts/...` verification packet paths when available
- when Canon emitted a readable verification result in the run summary, treat that summary as the happy path and keep artifact inspection as drill-down
- ordered possible actions
- one recommended next step that preserves the run context
- generated content must be written only to Canon-managed run outputs such as `.canon/artifacts/<RUN_ID>/verification/` and `.canon/artifacts/<RUN_ID>/verification/ai-provenance.md`, never back into `canon-input/`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing slot and must preserve valid ownership fields inside the current interaction.
- If risk, zone, or input are missing, name the missing input, keep the already valid fields, and show the exact Canon CLI retry form after the semantic prompt.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and do not restate already valid ownership metadata.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- If Canon returns `Blocked`, surface the unresolved findings or readiness blockers directly and do not imply that verification succeeded.
- If the assistant has already edited or is about to edit an authored input file under `canon-input/`, stop, restore the file from the user's last saved version, and report the rollback before continuing.

## Next-Step Guidance

- When Canon emitted a readable verification result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as optional drill-down.
- If the packet is blocked by unresolved findings, recommend `$canon-inspect-artifacts` first so the user can read the verification report and unresolved findings together.
- Use `$canon-inspect-evidence` when the user needs provenance, policy rationale, or validation lineage behind the verification packet.
- After external follow-up or evidence changes, recommend `$canon-status` first and use `$canon-resume` only if Canon still leaves the run incomplete.
- Use `$canon-review` for non-PR review packets and `$canon-pr-review` for diff-backed review rather than forcing verification to stand in for those workflows.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-resume`
- `$canon-review`
- `$canon-pr-review`
