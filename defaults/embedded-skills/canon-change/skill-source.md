---
name: canon-change
description: Use when you need a governed change run in a live codebase where invariants and existing behavior matter.
---

# Canon Change

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon change workflow as a governed run started from your
AI assistant.

## When To Trigger

- The user needs a governed change plan in a live codebase.
- The user is starting a new change intake from an intent, a note, or a brief.

## When It Must Not Trigger

- The user only needs requirements framing.
- The user wants a refactor discussion for a mode that is not runnable yet.
- The user is explicitly asking to inspect, approve, resume, or continue an existing run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one change brief file, one change input folder, one inline note, or enough starting intent to complete one through guided intake

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Treat a fresh change request as a new intake. Do not infer that the latest run, latest artifact directory, or latest brief is the active request unless the user explicitly says to continue or provides a real `RUN_ID`.
- If the user gives a new change intent after guided intake questions, default to starting a new change run for that intent. Do not pause to ask whether to continue an older blocked run unless the user explicitly asked to recover prior work.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- This skill always binds `--system-context existing`; if the user is actually shaping a new system or a new capability, redirect to `$canon-system-shaping` or `$canon-architecture` instead of forcing `change`.
- For auto-binding only, treat `canon-input/change.md` or `canon-input/change/` as the canonical authored-input locations for this mode.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, continue guided intake or ask explicitly for the brief path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the user already has a change brief file, use it directly.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If the user has only a change intent, guide them to fill the minimum missing slots: system slice, intended change, legacy invariants, allowed or excluded change surface, and validation strategy.
- The declared change surface must stay closed enough to name affected modules, interfaces, or jobs; open-ended scope such as adjacent modules, whole-repo work, or workspace-wide edits should be treated as escalation, not normal bounded intake.
- If the intent is still too ambiguous to bound safely after guided intake, stop and redirect to `$canon-requirements` rather than guessing.
- If risk and/or zone are missing after the authored brief or guided-intake surface is known, use `canon inspect risk-zone --mode change --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Author Change Body Before Invoking Canon

Canon does not invent the change body for you. Canon governs, validates, and persists the packet. You (the assistant) MUST author the real change body from the bounded source material BEFORE calling `canon run --mode change`.

Do this every time, even when the user starts from change intent rather than a finished brief:

1. Read the source inputs the user pointed at. Identify the existing system slice, the intended modification, the preserved invariants, the allowed change surface, and the validation evidence that must exist. Do not guess.
2. Compose a single change brief at `canon-input/change.md` or a folder-backed packet under `canon-input/change/`. The authored body MUST include all of the following H2 sections with concrete content tied to the source you just read:
  - `## System Slice`
  - `## Domain Slice`
  - `## Excluded Areas`
  - `## Intended Change`
  - `## Legacy Invariants`
  - `## Domain Invariants`
  - `## Forbidden Normalization`
  - `## Change Surface`
  - `## Ownership`
  - `## Cross-Context Risks`
  - `## Implementation Plan`
  - `## Sequencing`
  - `## Validation Strategy`
  - `## Independent Checks`
  - `## Decision Record`
  - `## Decision Drivers`
  - `## Options Considered`
  - `## Decision Evidence`
  - `## Boundary Tradeoffs`
  - `## Recommendation`
  - `## Why Not The Others`
  - `## Consequences`
  - `## Unresolved Questions`
3. Metadata such as `Owner:`, `Risk Level:`, and `Zone:` may remain in the brief, but they are outside authored-body extraction in this first slice.
4. Inline labels such as `Change Surface:` do not satisfy this first-slice contract. Use the canonical H2 headings above.
5. Canon preserves those authored sections verbatim in the emitted packet. If a required section is missing or empty, Canon emits `## Missing Authored Body` naming the missing canonical heading instead of fabricating filler.
6. If you cannot author a credible change body from the available source, stop and redirect to `$canon-requirements` or ask targeted intake questions before invoking Canon rather than submitting an empty brief.

### Packet Shape And Persona

Author the packet as a change owner writing an ADR-style bounded change plan
for maintainers and reviewers of the live system.

- Favor preserved invariants, the closed change surface, sequencing,
  validation evidence, explicit decision drivers, and rejection rationale.
- Keep the packet bounded to the declared system slice and excluded areas; do
  not widen scope with persona voice.
- Persona guidance is presentation only. If authored sections are missing,
  keep the gap explicit and let Canon preserve `## Missing Authored Body`
  rather than fabricating completeness.

## Canon Command Contract

- Canon command: `canon run --mode change --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- The skill may guide the user to complete a minimal change brief before invoking Canon; do not require a fully authored brief up front.
- For a fresh request, do not summarize prior Canon runs as if they were the current attempt. Start from intake, then invoke Canon once the bounded brief is ready.
- For a fresh request, do not present `continue existing run` versus `start fresh` as a primary choice. New intake is the default path unless the user explicitly requests continuation or provides a `RUN_ID`.
- Return the real Canon run id, state, and any approval target Canon emits.

## AI Companion Operating Model

After preflight succeeds and the real Canon run exists, the assistant is
responsible for turning the change packet from templated stubs into a
grounded, reviewable artifact set.

### Run Boundary

- Read the full authored input surface before generation. If the input is a directory such as `canon-input/change/`, treat the directory as one authored packet and read it recursively before generation.
- Start the real Canon run before artifact writing so `.canon/artifacts/<RUN_ID>/change/` exists and the packet stays attached to a real run id.
- Treat authored inputs under `canon-input/` as read-only source material. Do not rewrite, normalize, append to, or otherwise modify the user's input files during preflight, clarification, generation, critique, or summary.
- Do not mutate source files outside `.canon/artifacts/<RUN_ID>/change/` during generation. Surface any proposed code changes as patch text or annotated excerpts inside the change packet, not as in-place edits to the working tree.
- Write generated content only into Canon-managed files under `.canon/artifacts/<RUN_ID>/change/`.
- Keep unanswered ambiguity explicit in the generated packet or provenance sidecar instead of quietly guessing.

### Generation Loop

1. Read the authored input surface and extract the targeted system slice, intended change, legacy invariants the change must preserve, declared change surface, validation strategy, and the explicit open questions already present in the source.
2. Start the Canon run and inspect the generated change stubs under `.canon/artifacts/<RUN_ID>/change/`.
3. If the source is sufficient, generate the packet directly. If it is structurally sufficient but materially ambiguous, run the clarification loop before final generation.
4. Draft each artifact so every nontrivial claim, proposed mutation, and risk call is grounded in the authored input, in an explicit user clarification, or in a clearly marked open question.
5. Run a critique pass that challenges scope drift beyond the declared change surface, broken or invented legacy invariants, unjustified mutations to the change surface, and unaddressed validation gaps.
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

Write `.canon/artifacts/<RUN_ID>/change/ai-provenance.md` describing how the
packet was produced. It must include:

- The authored input files that were read.
- The clarification questions that were asked, the user answers received, and any defaults that were applied because the user skipped or deferred a question.
- A `Clarification Loop` section with: number of questions presented, number of Required questions answered, number of Required questions left unresolved, number of Optional questions answered or deferred, and the number of substantive clarifications that drove material changes per artifact section.
- The critique findings that were addressed and any that were intentionally deferred.
- The generation model or assistant identity that produced the artifact set.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the change result when a readable packet exists
- primary artifact path and short excerpt when available
- direct statement of what happened or what is blocking progress
- concrete `.canon/artifacts/...` paths when Canon emitted them
- optional action chips for the same valid next steps, typically `Inspect evidence` before `Approve generation...` when no readable packet exists yet
- ordered possible actions
- one recommended next step that keeps the run context intact
- generated content must be written only to Canon-managed run outputs such as `.canon/artifacts/<RUN_ID>/change/` and `.canon/artifacts/<RUN_ID>/change/ai-provenance.md`, never back into `canon-input/` and never as in-place edits to the working tree

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- When the user already has a file-backed brief flow, the preflight must preserve valid ownership fields and asks only for the missing brief path or missing ownership slot.
- The preflight must preserve valid ownership fields and ask only for the missing intake slot or missing brief path.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- If the user starts from intent instead of a brief, request only the minimum missing change slots rather than demanding a full document rewrite.
- If the user says they are starting from scratch, do not inspect old runs or report stale artifact state unless they explicitly ask for recovery of a previous run.
- If the user provides a fresh intent and the repo also contains older blocked runs, do not interrupt the intake with a continuation choice. Mention older runs only as optional context after the new-intake path is established.
- If the user is really targeting a new system, say directly that `change` currently supports only `--system-context existing` and redirect them to `$canon-system-shaping` or `$canon-architecture`.
- If a file-backed retry is required, name the missing typed slot explicitly and show the exact Canon CLI retry form after the semantic prompt.
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and keep already valid ownership or intake fields.
- If the change is still too ambiguous to bound safely, say that directly and recommend `$canon-requirements` as the honest next step.
- If the declared change surface broadens beyond a closed bounded slice, say that Canon escalated the mutation request for explicit approval instead of treating it as ordinary recommendation-only guidance.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If the input file is missing, request only the missing brief path and do not restate already valid ownership metadata.
- If an input is invalid, ask only for the failing slot rather than resetting the whole request.
- If Canon returns a failure after preflight succeeded, report it as a Canon-execution outcome, not as a preflight failure.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced and do not imply the run is complete.
- If Canon returns recommendation-only transformation guidance, say that workspace mutation is still gated in this tranche and point first to the review packet that explains the recommendation.
- If the assistant has already edited or is about to edit an authored input file under `canon-input/`, or any source file outside `.canon/artifacts/<RUN_ID>/change/`, stop, restore the file from the user's last saved version or from Git, and report the rollback before continuing.

## Next-Step Guidance

- For a fresh request, the recommended next step is guided intake, not inspection of prior runs.
- For a fresh request, proceed through intake and start the new run once the bounded brief is ready. Do not require the user to choose between a new run and an older blocked run unless they explicitly asked for recovery.
- When Canon emitted a readable change result in the run summary, treat that summary as the happy path and keep `$canon-inspect-artifacts` as drill-down.
- When the run starts cleanly, recommend `$canon-inspect-artifacts` first if Canon emitted a reviewable packet; otherwise recommend `$canon-inspect-evidence`.
- When change mutation is recommendation-only or approval-gated, recommend `$canon-inspect-artifacts` first only if Canon emitted a readable packet that explains the block.
- If scope broadening triggered approval on the mutation request, recommend `$canon-inspect-artifacts` first so the user can review the bounded packet before approving the broader surface.
- If no concrete artifact paths are available yet, recommend `$canon-inspect-evidence` first, then `$canon-approve`, then `$canon-resume`.
- Use `$canon-inspect-evidence` when the user needs the invocation rationale, policy decision, or evidence lineage behind the packet, especially before generation has emitted readable artifacts.
- Use `$canon-status` to re-check the overall run state only after inspection or resume, not as a generic first stop.
- If a host renders chips, the chip order must follow the same logic and the approval chip label must remain `Approve generation...`.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
