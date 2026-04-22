---
name: canon-architecture
description: Use when you need a governed Canon architecture run to record decisions, tradeoffs, and risk-gated approvals.
---

# Canon Architecture

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Start a real Canon architecture run from your AI assistant without making the
user memorize the raw CLI.

## When To Trigger

- The user is asking for architecture mode specifically.
- The user needs a governed architecture decision packet with critique and approval-aware risk handling.

## When It Must Not Trigger

- The user already has a run id and wants inspection, approval, or continuation actions.
- The architecture question is actually a bounded change-planning request tied to a live slice of an existing system; use `$canon-change`.

## Required Inputs

- `RISK`
- `ZONE`
- `SYSTEM_CONTEXT`
- at least one input file, input folder, or inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Verify `SYSTEM_CONTEXT` is present before invoking Canon.
- Ask for `SYSTEM_CONTEXT` explicitly with guided fixed choices `new` or `existing`; do not infer it from the mode label.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- For auto-binding only, treat `canon-input/architecture.md` or `canon-input/architecture/` as the canonical authored-input locations for this mode.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, ask explicitly for the input path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the user is starting from a short note instead of a structured brief, guide them to include explicit architectural decision focus and constraints before invoking Canon.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode architecture --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Canon Command Contract

- Canon command: `canon run --mode architecture --system-context <SYSTEM_CONTEXT> --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any approval target Canon emits.

## AI Companion Operating Model

After preflight succeeds and the real Canon run exists, the assistant is
responsible for turning the architecture packet from templated stubs into a
grounded, reviewable artifact set.

### Run Boundary

- Read the full authored input surface before generation. If the input is a directory such as `canon-input/architecture/`, treat the directory as one authored packet and read it recursively before generation.
- Start the real Canon run before artifact writing so `.canon/artifacts/<RUN_ID>/architecture/` exists and the packet stays attached to a real run id.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- Write generated content only into Canon-managed files under `.canon/artifacts/<RUN_ID>/architecture/`.
- Keep unanswered ambiguity explicit in the generated packet or provenance sidecar instead of quietly guessing.

### Generation Loop

1. Read the authored input surface and extract the architectural decisions in scope, candidate alternatives, tradeoffs already cited, declared constraints, and the explicit open questions already present in the source.
2. Start the Canon run and inspect the generated architecture stubs under `.canon/artifacts/<RUN_ID>/architecture/`.
3. If the source is sufficient, generate the packet directly. If it is structurally sufficient but materially ambiguous, run the clarification loop before final generation.
4. Draft each artifact so every nontrivial decision is grounded in the authored input, in an explicit user clarification, or in a clearly marked open question.
5. Run a critique pass that challenges unsupported decisions, hidden assumptions about runtime or organizational context, missing alternatives, and unaddressed risk or zone constraints.
6. Overwrite the templated stubs with the revised packet and write the provenance sidecar.

### Clarification Loop

When the authored input is structurally present but materially ambiguous on
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

Write `.canon/artifacts/<RUN_ID>/architecture/ai-provenance.md` describing how
the packet was produced. It must include:

- The authored input files that were read.
- The clarification questions that were asked, the user answers received, and any defaults that were applied because the user skipped or deferred a question.
- A `Clarification Loop` section with: number of questions presented, number of Required questions answered, number of Required questions left unresolved, number of Optional questions answered or deferred, and the number of substantive clarifications that drove material changes per artifact section.
- The critique findings that were addressed and any that were intentionally deferred.
- The generation model or assistant identity that produced the artifact set.

## Expected Output Shape

- concise run-start or gated summary
- real run id
- real run state
- direct statement of the architecture result when a readable packet exists
- primary artifact path and short excerpt when available
- concrete `.canon/artifacts/...` paths when Canon emitted them
- next steps pointing to `$canon-status`, `$canon-inspect-artifacts`, and `$canon-inspect-evidence`
- generated content must be written only to Canon-managed run outputs such as `.canon/artifacts/<RUN_ID>/architecture/` and `.canon/artifacts/<RUN_ID>/architecture/ai-provenance.md`, never back into `canon-input/`

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing slot and must preserve valid ownership fields inside the current interaction.
- If risk, zone, or input are missing, name the missing input, keep the already valid fields, and show the exact Canon CLI retry form after the semantic prompt.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `SYSTEM_CONTEXT`, use guided fixed choices with the exact allowed values `new` and `existing`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and do not restate already valid ownership metadata.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced, typically `gate:risk`, and do not imply the run is complete.
- Never emit fabricated Canon runtime state.
- If the assistant has already edited or is about to edit an authored input file under `canon-input/`, stop, restore the file from the user's last saved version, and report the rollback before continuing.

## Next-Step Guidance

- When Canon emitted readable artifacts, recommend `$canon-inspect-artifacts` first.
- When Canon emitted a readable architecture result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as drill-down.
- Use `$canon-inspect-evidence` when the user needs lineage, critique rationale, or approval context behind the packet.
- Use `$canon-approve` only after the user has reviewed the packet or explicitly wants to record approval.
- After approval, recommend `$canon-status` first and use `$canon-resume` only if Canon still leaves the run incomplete.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-resume`
