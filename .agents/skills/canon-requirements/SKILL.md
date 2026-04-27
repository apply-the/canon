---
name: canon-requirements
description: Use when you need a bounded requirements run in Canon before code, architecture, or execution drift starts.
---

# Canon Requirements

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Start a real Canon requirements run from your AI assistant without making the user memorize
the raw CLI.

## When To Trigger

- The user wants bounded framing before design or implementation moves.
- The user has a problem statement or input file and needs a governed requirements run.

## When It Must Not Trigger

- The user already has a run id and wants inspection or unblock actions.
- The user is asking specifically for change or pr-review behavior.

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
- For auto-binding only, treat `canon-input/requirements.md` or `canon-input/requirements/` as the canonical authored-input locations for this mode.
- When both canonical requirements locations exist, prefer `canon-input/requirements/` so preflight and clarity inspect the full authored requirements surface instead of a single file.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- If neither canonical location exists and the user did not provide an explicit input or inline note, ask explicitly for the authored input path or inline note.
- `OWNER` is optional. If omitted, Canon should try repository-local or global Git identity before asking for explicit owner input.
- If the selected file, folder, or inline note is empty, whitespace-only, or structurally insufficient, surface that as invalid authored input and retry only that slot.
- If the authored brief is present but underspecified, use `$canon-inspect-clarity` with `MODE=requirements` and the full authored input surface so Canon runs `canon inspect clarity --mode requirements --input <INPUT_PATH> [<INPUT_PATH> ...]` and surfaces Canon-backed missing-context findings and targeted clarification questions before starting the run.
- If risk and/or zone are missing after the authored input surface is known, use `canon inspect risk-zone --mode requirements --input <INPUT_PATH>` to infer a provisional pair, explain the Canon rationale and confidence, and ask the user to confirm or override before invoking Canon.
- If the inferred classification returns `low` confidence, present it as provisional and invite override rather than treating it as final.
- Classification confirmation is intake confirmation only, not Canon approval.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.
- Do not show preflight checks to the user. Report only the specific missing input.

## Author Requirements Body Before Invoking Canon

Canon does not invent the requirements body for you. Canon governs, validates, and persists the packet. You (the assistant) MUST author the real requirements body from the bounded source material BEFORE calling `canon run --mode requirements`.

Do this every time:

1. Read the source inputs the user pointed at. Identify the bounded problem, the intended outcome, the relevant constraints, and the still-open decisions. Do not guess.
2. Compose a single requirements brief at `canon-input/requirements.md` or a folder-backed packet under `canon-input/requirements/`. The authored body MUST include all of the following H2 sections with concrete content tied to the source you just read:
	- `## Problem` — the user-visible or operator-visible problem and why it matters now.
	- `## Outcome` — what should be true if this bounded slice succeeds.
	- `## Constraints` — hard delivery, compatibility, compliance, performance, or operational constraints.
	- `## Non-Negotiables` — explicit rules that cannot be relaxed in this slice.
	- `## Options` — the credible bounded options still on the table.
	- `## Recommended Path` — the currently recommended option and why it wins for this slice.
	- `## Tradeoffs` — the important costs or downsides attached to the recommendation.
	- `## Consequences` — what follows if the recommendation is accepted.
	- `## Scope Cuts` — explicit exclusions for this slice. Prefer this canonical heading.
	- `## Deferred Work` — follow-on work that remains intentionally outside this slice.
	- `## Decision Checklist` — the concrete decisions or confirmations required before downstream planning.
	- `## Open Questions` — unresolved items that still need explicit handling.
3. Prefer the canonical heading `## Scope Cuts`. `## Out of Scope` is accepted only as a compatibility alias for already-authored material and is not the preferred heading for new briefs.
4. Canon preserves those authored sections verbatim in the emitted packet. If a required section is missing or empty, Canon emits `## Missing Authored Body` naming the missing canonical heading instead of fabricating filler.
5. If you cannot author a credible requirements body from the available source, run `$canon-inspect-clarity` or ask targeted questions before invoking Canon rather than submitting an empty brief.

## Canon Command Contract

- Canon command: `canon run --mode requirements --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id and state, plus the run's final result summary when Canon emitted a readable requirements packet.

## AI Companion Operating Model

After preflight succeeds and the real Canon run exists, the assistant is
responsible for turning the requirements packet from templated stubs into a
grounded, reviewable artifact set.

### Run Boundary

- Read the full authored input surface before generation. If the input is a
	directory such as `canon-input/requirements/`, treat the directory as one
	authored packet and read it recursively instead of narrowing to one child
	file.
- Start the real Canon run before artifact writing so
	`.canon/artifacts/<RUN_ID>/requirements/` exists and the packet stays
	attached to a real run id.
- Treat `canon-input/` as read-only source material throughout generation,
	clarification, critique, and summary.
- Write generated content only into Canon-managed files under
	`.canon/artifacts/<RUN_ID>/requirements/`.
- Keep unanswered ambiguity explicit in the generated packet or provenance
	sidecar instead of quietly guessing.

### PRD Shape

The Canon requirements packet IS the Product Requirements Document (PRD) for
this bounded scope. The materialized artifact set under
`.canon/artifacts/<RUN_ID>/requirements/` must read end-to-end as a
self-contained PRD.

The PRD must cover, at minimum:

- Problem Statement: the user-visible problem and why it matters now.
- Target Users / Stakeholders: who is affected and who owns the outcome.
- Scope: explicit in-scope items and explicit out-of-scope items, both grounded
  in the authored input or in an explicit user clarification.
- Functional Requirements: bounded, testable behaviors the system must support.
- Non-Functional Requirements: constraints such as performance, security,
  compatibility, regulatory, and operability boundaries.
- Success Criteria / Acceptance: how the team will know the requirements are
  met, expressed as measurable outcomes when possible.
- Dependencies and Assumptions: external systems, teams, data, or facts the
  PRD relies on.
- Open Questions and Decisions: unresolved items, with the recorded default
  the PRD currently assumes.

Mapping rules:

- Map every PRD section above to one of the Canon-emitted artifact files. If
  the Canon artifact contract does not natively contain a PRD section, place
  it under the closest existing artifact and record the mapping in the
  provenance sidecar.
- A reader who opens only `.canon/artifacts/<RUN_ID>/requirements/` must be
  able to understand the product intent without reopening `canon-input/`.
- Do not invent PRD content that is not grounded in the authored input, in an
  explicit user clarification, or in an explicitly marked open question.
- Keep the PRD focused on this run's bounded scope; do not expand it into a
  product roadmap or a long-term vision document.

### Authoring Persona

Author the packet as a product lead writing a bounded PRD for the people who
must decide whether to proceed with this slice.

- Favor product-facing language about problem, outcomes, users or
	stakeholders, scope, success criteria, dependencies, and open decisions.
- Keep the packet tightly bounded to the current run; do not drift into a
	roadmap, implementation plan, or long-term vision memo.
- Persona guidance is presentation only. If the source is missing a required
	section, keep the gap explicit or let Canon emit `## Missing Authored Body`;
	never invent missing PRD content just to make the packet sound complete.

### Generation Loop

1. Read the authored input surface and extract the bounded problem, scope cuts,
   constraints, dependencies, decisions, and open questions already present in
   the source.
2. Start the Canon run and inspect the generated requirements stubs under
   `.canon/artifacts/<RUN_ID>/requirements/`.
3. If the source is sufficient, generate the PRD directly. If the source is
   structurally sufficient but materially ambiguous, run the clarification loop
   before final generation.
4. Draft each PRD section so every nontrivial claim is grounded either in the
   authored input, in an explicit user clarification, or in a clearly marked
   open question.
5. Run a critique pass against the draft and challenge unsupported scope
   expansion, hidden assumptions, contradictions, missing PRD sections, and
   invented certainty.
6. Overwrite the templated stubs with the revised PRD and write the provenance
   sidecar.

### Clarification Loop

Use clarification only when the missing information changes the packet
materially. Do not ask generic discovery questions once the input is already
bounded enough for requirements mode.

#### Question Granularity Rules

- Ask one decision per question.
- Every question must be self-contained enough that the user can answer it
	without reading the rest of the batch.
- Include the local context inside the question block; do not expect the user
	to infer which document fragment you are referring to.
- Never join multiple topics with commas, `and`, `plus`, or stacked subclauses
	inside one question.
- Never ask the user to do your extraction work with prompts such as `give
	source refs`, `extract from the docs`, or `should I infer this`.
- If a question cannot be asked atomically, rewrite it into smaller questions
	or drop it.

#### Question Format (mandatory)

Present questions as a numbered list. For each question render exactly:

```
N. <one-sentence question>
	 Affects: <artifact file> > <section name>
	 Why it matters: <one line explaining what this answer changes in the packet>
	 Context: <up to two lines quoting or paraphrasing the relevant input fragment, or "no input coverage" if the documents are silent>
	 Options:
		 a) <concrete option grounded in the input or in plausible domain practice>
		 b) <a different concrete option>
		 c) <a third option when the space is naturally trichotomous>
		 d) Other / free text
	 Default if skipped: <what you will write in the artifact if the user defers, e.g. "leave as Open Question">
	 Status: Required | Optional
```

Rules on options:

- Options must be concrete answers, not `tell me more` or `extract from docs`.
	If you cannot enumerate plausible answers, the question is underspecified -
	either rewrite it more narrowly or drop it.
- Provide between 2 and 4 options plus `Other / free text`.
- Order options from most to least likely given the input.
- Never use an option as a way to ask another question.

#### Use Host UI Affordances When Available

Some chat hosts render structured question prompts (Copilot Chat input forms,
Codex multi-question panels, Claude rich inputs). When the host exposes a
rich-input affordance, prefer it over plain prose:

- Use the host's structured-question or form widget when it can carry both the
	question text and helper or secondary text. Put the one-sentence question as
	the main prompt and put `Why it matters` plus `Context` as the secondary or
	hint text so the user sees them as inline guidance, tooltip, description, or
	subtitle.
- Use the host's option-picker widget for the `Options` block when one is
	available, so the user can click instead of typing.
- Use the host's multi-question pager when more than one question fits the same
	submit gesture, but never collapse multiple distinct topics into a single
	question just to fit one page.
- Keep the question text short enough to read at a glance: hard cap of one
	sentence and roughly 100 characters.
- Keep `Why it matters` to one short line and `Context` to at most two short
	lines so they fit a tooltip or hint area.
- When the host has no rich-input affordance, fall back to the plain
	numbered-list rendering above. Do not pretend to render UI the host does not
	support.
- If the host strips or truncates the helper text, the question itself must
	still be answerable.

#### Batch Size

- Ask 3 to 5 questions per batch when multiple ambiguities remain.
- Hard max: 7 questions in one batch.
- Ask fewer questions when only 1 or 2 ambiguities materially affect the
	packet.
- Stop asking once the remaining uncertainty can be honestly recorded as an
	open question.

#### Interaction Loop

- Ask the first batch.
- Wait for the user's answers before generating final artifact content for the
	affected sections.
- Revise the working packet after each batch.
- Ask a second batch only when unresolved ambiguity still materially changes
	the packet.
- If the user skips a required answer, apply the stated default, mark the
	uncertainty explicitly, and record it in the provenance sidecar.

#### Provenance Sidecar

Write `.canon/artifacts/<RUN_ID>/requirements/ai-provenance.md` as a compact
audit trail for the AI-authored packet. Include:

- source inputs actually used
- working approach and critique focus
- unresolved or deferred questions
- a `Clarification Loop` section with the counts: presented, Required
	answered, Required unresolved, Optional answered or deferred, and
	substantive clarifications by section

## Expected Output Shape

- concise run summary
- real run id
- real run state
- direct statement of the requirements result when a readable packet exists
- primary artifact path and short excerpt when available
- primary artifact action when Canon exposes a Canon-backed direct-open affordance
- generated content must be written only to Canon-managed run outputs such as `.canon/artifacts/<RUN_ID>/requirements/` and `.canon/artifacts/<RUN_ID>/requirements/ai-provenance.md`, never back into `canon-input/`
- one recommended next step that preserves the same run context, or no mandatory next step when the requirements result is already self-explanatory

## Failure Handling Guidance

- If `.canon/` is missing, point to `$canon-init`.
- The preflight asks only for the missing slot and must preserve valid ownership fields inside the current interaction.
- If the assistant has already edited or is about to edit an authored input file under `canon-input/`, stop, acknowledge the boundary error, restore the workflow to read-only input handling, and continue by writing only to Canon-managed run outputs.
- If risk, zone, or input are missing, name the missing input, keep the already valid fields, and show the exact Canon CLI retry form after the semantic prompt.
- If preflight returns classification confirmation instead of readiness, treat that as missing intake confirmation rather than as a Canon approval gate.
- If Canon fails because no owner could be resolved, ask for `--owner <OWNER>` explicitly or tell the user to configure `git user.name` and `git user.email`.
- For `RISK`, use guided fixed choices with the exact allowed values `low-impact`, `bounded-impact`, and `systemic-impact`.
- For `ZONE`, use guided fixed choices with the exact allowed values `green`, `yellow`, and `red`.
- If an input is invalid, tell the user which typed slot failed and retry only that slot.
- If the input file is missing, ask only for the missing path and do not restate already valid ownership metadata.
- If an explicit inline note is empty or whitespace-only, ask only for non-empty `--input-text` content and do not restate already valid ownership metadata.
- If Canon fails after preflight succeeds, state that the failure happened inside Canon execution rather than before Canon execution.
- Never simulate a successful run if Canon did not start one.

## Next-Step Guidance

- When the run completed and emitted a readable packet, treat the run summary itself as the happy-path result and recommend `$canon-inspect-artifacts` only as optional drill-down.
- If the authored input is still underspecified before run start, prefer `$canon-inspect-clarity` and its `canon inspect clarity --mode requirements --input <INPUT_PATH> [<INPUT_PATH> ...]` contract over generic follow-up questions.
- When Canon exposes a primary artifact action, surface that direct-open affordance before inspect detours.
- Use `$canon-inspect-evidence` when the user needs provenance, policy rationale, or denied invocation detail behind the packet.
- Use `$canon-status` to re-check the overall run state only after inspection or follow-up work.

## Related Skills

- `$canon-init`
- `$canon-inspect-clarity`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`

