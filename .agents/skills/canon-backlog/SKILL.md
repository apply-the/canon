---
name: canon-backlog
description: Use when you need a governed backlog run that decomposes bounded upstream decisions into delivery epics and slices.
---

# Canon Backlog

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon backlog workflow as a governed run started from your AI assistant.

## When To Trigger

- The user needs a governed backlog packet that turns bounded upstream decisions into epics, slices, dependencies, and sequencing.
- The user already has a bounded backlog brief or packet and wants Canon to preserve traceability and closure findings.

## When It Must Not Trigger

- The upstream structure is still unsettled and the user really needs `$canon-system-shaping` or `$canon-architecture` first.
- The user already has a bounded execution slice and now needs task mapping or mutation guidance; use `$canon-implementation`.
- The user is explicitly asking to inspect, approve, resume, or continue an existing run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one backlog brief file, one backlog input folder, or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Always bind `--system-context existing` for this skill.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/backlog.md` or `canon-input/backlog/` as the canonical authored-input locations for this mode.
- For a folder-backed packet under `canon-input/backlog/`, treat `brief.md` as the authoritative backlog brief, `priorities.md` as explicit decomposition priority input, and `context-links.md` as carried-forward source references.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or published packets.
- If the user only has a vague change intent or unresolved structural question, redirect to `$canon-system-shaping` or `$canon-architecture` instead of fabricating decomposition detail.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.

## Author Backlog Body Before Invoking Canon

Canon does not invent the backlog body for you. Canon governs, validates, and persists the packet. You (the assistant) MUST author the real backlog content from the bounded source material BEFORE calling `canon run --mode backlog`. This mirrors how `/speckit.tasks` produces an actual `tasks.md`.

Do this every time, even when the user only handed you a one-line delivery intent:

1. Read the source inputs the user pointed at (files, folders, repo). Identify the real capability surface from code, configs, and docs. Do not guess.
2. Compose a single backlog brief file at `canon-input/backlog/brief.md` (or use `--input-text` for a one-shot inline brief). The file MUST include all of the following H2 sections, populated with concrete content tied to the source you just read:
   - `## Delivery Intent` — one paragraph naming the bounded outcome.
   - `## Desired Granularity` — one of `epic-plus-slice`, `epic-plus-slice-plus-story-candidate`.
   - `## Planning Horizon` — concrete release window or quarter.
   - `## Source References` — bullet list of real files/dirs/links you actually read.
   - `## Priorities` — bullet list of decomposition priorities.
   - `## Constraints` — bullet list of non-negotiables (perf, contract, ownership, rollback).
   - `## Out of Scope` — bullet list of explicit exclusions.
   - `## Epic Tree` — 3 to 7 named epics, each with a one-line scope statement and the source area it touches. No placeholders.
   - `## Capability To Epic Map` — bullets mapping concrete capabilities/modules to epic ids.
   - `## Dependency Map` — bullets naming inter-epic and external dependencies with direction.
   - `## Delivery Slices` — for each epic, 1-3 named slices that are user- or contract-visible, above task level.
   - `## Sequencing Plan` — ordered list of slices with rationale.
   - `## Acceptance Anchors` — bullets describing the observable signal that proves each slice is done.
   - `## Planning Risks` — bullets naming sequencing, dependency, and granularity risks tied to this body.
3. Each section MUST be specific to the source you read. Generic boilerplate ("Establish a bounded foundation", "Deliver visible slices") is a failure and the user will reject it.
4. Stop at slice level. Do NOT invent task-level breakdown, sprint tickets, or executable test plans.
5. Then invoke Canon. Canon will preserve your authored sections verbatim into the eight-artifact packet, attach closure findings if the brief is weak, and govern the run.

If you cannot author a credible body because the upstream structure is genuinely unsettled, say so directly and redirect to `$canon-system-shaping` or `$canon-architecture` instead of submitting an empty brief.

## Canon Command Contract

- Canon command: `canon run --mode backlog --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any closure findings Canon emits.
- A sufficiently bounded backlog run completes with a full eight-artifact planning packet.
- A closure-limited backlog run may complete in downgraded form or stop blocked, but it must surface explicit closure findings and limit outputs to `backlog-overview.md` and `planning-risks.md`.

## Expected Output Shape

- concise run-start summary
- Canon-backed run state
- direct statement of whether the backlog packet is full or closure-limited
- primary artifact path and short excerpt when available
- concrete `.canon/artifacts/<RUN_ID>/backlog/` paths when Canon emitted them
- closure findings, decomposition scope, and closure notes when the run is downgraded or blocked
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon already returned in `mode_result.action_chips`; do not collapse them to label-only bullets. In text-only hosts, render each chip's `text_fallback` instead. Must be the last element of the response; do not place any text after this section.

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- If `.canon/` is missing, point to `$canon-init`.
- If the user is still shaping the capability or debating boundaries, redirect to `$canon-system-shaping` or `$canon-architecture` instead of inventing backlog decomposition.
- If Canon returns a blocked or downgraded packet, point first to `planning-risks.md`, the closure findings Canon surfaced, and the limited artifact set rather than implying the full packet exists.
- If Canon emits only the risk packet, say directly that Canon did not claim credible decomposition yet.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- Use `$canon-inspect-evidence` when the user needs lineage, closure rationale, or policy decisions.
- Use `$canon-implementation` only after a bounded slice is selected from the packet.
- Return to `$canon-system-shaping` or `$canon-architecture` when the real blocker is unresolved structure, ownership, or dependency closure.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-system-shaping`
- `$canon-architecture`
- `$canon-implementation`

## Canonical Authored Inputs

- `canon-input/backlog.md`
- `canon-input/backlog/`

## Output Intent

- durable backlog planning packet
- explicit closure findings when source architecture is not sufficiently closed
- no task-level decomposition