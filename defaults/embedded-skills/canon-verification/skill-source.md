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

## AI Companion Operating Model

After preflight succeeds and the real Canon run exists, the assistant is
responsible for turning the verification packet from templated stubs into a
grounded, reviewable artifact set.

### Run Boundary

- Read the full authored challenge packet before generation. If the input is the directory `canon-input/verification/`, treat the directory as one authored packet and read it recursively before generation.
- Start the real Canon run before artifact writing so `.canon/artifacts/<RUN_ID>/verification/` exists and the packet stays attached to a real run id.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- Write generated content only into Canon-managed files under `.canon/artifacts/<RUN_ID>/verification/`.
- Keep unanswered ambiguity explicit in the generated packet or provenance sidecar instead of quietly guessing.

### Generation Loop

1. Read the authored challenge packet and extract the claims, invariants, contracts, or evidence to be challenged, the supporting evidence already cited, the threat model or failure modes, and the explicit open questions already present in the source.
2. Start the Canon run and inspect the generated verification stubs under `.canon/artifacts/<RUN_ID>/verification/`.
3. If the packet is sufficient, generate the verification directly. If it is structurally sufficient but materially ambiguous, run the clarification loop before final generation.
4. Draft each verdict, counterexample, and unresolved finding so it is grounded in the authored packet, in an explicit user clarification, or in a clearly marked open question.
5. Run a critique pass that challenges unjustified verification verdicts, missed counterexamples, unstated dependence on prior assumptions, and silent acceptance of evidence gaps.
6. Overwrite the templated stubs with the revised verification packet and write the provenance sidecar.

### Clarification Loop

When the authored packet is structurally present but materially ambiguous on
points that would force the assistant to invent content, ask targeted
clarification questions before generation rather than papering over the gap.

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
  - Context: <≤2 lines from authored input, or `no input coverage`>
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
- Keep the visible question short, ideally under ~100 characters; move background, citations, and rationale into the secondary or tooltip slot.
- Fall back to the plain bulleted block above only when the host has no rich question UI or when the user explicitly asks for it inline.

#### Batch Size

- Ask 3 to 5 questions per round, never more than 7. If more questions exist, defer the lower-impact ones to a later round.

#### Interaction Loop

- Ask the batch.
- Wait for user answers.
- Apply answers, regenerate the affected sections, and surface any remaining open questions still required to finish the packet.
- Stop the loop when no Required question is unanswered.

#### Provenance Sidecar

Write `.canon/artifacts/<RUN_ID>/verification/ai-provenance.md` describing how
the verification packet was produced. It must include:

- The authored input files that were read.
- The clarification questions that were asked, the user answers received, and any defaults that were applied because the user skipped or deferred a question.
- A `Clarification Loop` section with: number of questions presented, number of Required questions answered, number of Required questions left unresolved, number of Optional questions answered or deferred, and the number of substantive clarifications that drove material changes per artifact section.
- The critique findings that were addressed and any that were intentionally deferred.
- The generation model or assistant identity that produced the artifact set.

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
