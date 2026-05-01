---
name: canon-refactor
description: Use when you need a governed refactor run for an existing system with preserved behavior and explicit no-feature-addition evidence.
---

# Canon Refactor

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon refactor workflow as a governed run started from
your AI assistant.

## When To Trigger

- The user needs a governed refactor packet for an existing system.
- The user already has a bounded preservation brief and wants Canon to persist the packet and evidence.

## When It Must Not Trigger

- The user still needs to decide the change boundary or is actually adding new behavior; use `$canon-change`.
- The user is explicitly asking to inspect, approve, resume, or continue an existing run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one refactor brief file, one refactor input folder, or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Always bind `--system-context existing` for this skill.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/refactor.md` or `canon-input/refactor/` as the canonical authored-input locations for this mode.
- For a folder-backed packet under `canon-input/refactor/`, treat `brief.md` as the authoritative readiness brief and `source-map.md` as explicit provenance for carried-forward `change` or `implementation` context.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or any other path under `.canon/`.
- Do not infer readiness by reading prior `.canon/` artifacts or published packets directly; the current brief must restate preserved behavior, scope, safety-net evidence, and no-feature-addition proof.
- If the user still needs to decide the preserved behavior boundary or the work adds new behavior, redirect to `$canon-change` instead of guessing.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.

## Author Refactor Body Before Invoking Canon

Canon does not invent the refactor body for you. Canon governs, validates, and
persists the packet. You (the assistant) MUST author the real refactor body
from the bounded source material BEFORE calling `canon run --mode refactor`.

Do this every time, even when the user starts from a short brief rather than a
finished packet:

1. Read the source inputs the user pointed at. Identify the preserved behavior,
bounded refactor scope, allowed paths, safety-net evidence, and no-feature-
addition proof. Do not guess.
2. Compose a single refactor brief at `canon-input/refactor.md` or a
folder-backed packet under `canon-input/refactor/`. The authored body MUST
include all of the following canonical H2 sections with concrete content:
	- `## Preserved Behavior`
	- `## Approved Exceptions`
	- `## Refactor Scope`
	- `## Allowed Paths`
	- `## Structural Rationale`
	- `## Untouched Surface`
	- `## Safety-Net Evidence`
	- `## Regression Findings`
	- `## Contract Drift`
	- `## Reviewer Notes`
	- `## Feature Audit`
	- `## Decision`
3. Inline labels or near-miss headings such as `Decisions` do not satisfy this
slice. Use the canonical H2 headings above.
4. Canon preserves those authored sections verbatim in the emitted packet. If a
required section is missing or empty, Canon emits `## Missing Authored Body`
naming the missing canonical heading instead of fabricating filler.
5. If you cannot author a credible refactor body from the available source,
stop and redirect to `$canon-change` or ask targeted clarification questions
before invoking Canon rather than submitting an empty brief.

### Packet Shape And Persona

Author the packet as a preservation-focused maintainer proving that structure
changes remain behavior-safe for reviewers and approvers.

- Favor preserved behavior, structural rationale, untouched surface, feature
	audit, and safety-net evidence over generic cleanup language.
- Keep conclusions bounded to the declared refactor scope and approved
	exceptions; do not drift into feature planning or broader change design.
- Persona guidance is presentation only. Missing authored sections,
	preservation gaps, or no-feature-addition concerns remain explicit and must
	not be backfilled with confident refactor prose.

## Canon Command Contract

- Canon command: `canon run --mode refactor --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any approval target Canon emits.
- Refactor always emits a `gate:execution` approval target on the first run; the `state` is `AwaitingApproval` and the posture stays `recommendation-only` until that gate is approved.
- After `$canon-approve` records the `gate:execution` approval, Canon still remains `AwaitingApproval` until `$canon-resume` runs the post-approval continuation.
- After `$canon-resume`, surface `approved-recommendation` only when the packet has no executable local patch payload; real workspace mutation currently requires a bounded local payload such as `patch.diff` inside `canon-input/refactor/`.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the refactor result when a readable packet exists
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only` before resume, `approved-recommendation` only after approved resume without a local patch payload, `mutating` after approved resume with a valid local patch payload)
- concrete `.canon/artifacts/<RUN_ID>/refactor/` paths when Canon emitted them
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon already returned in `mode_result.action_chips`; do not collapse them to label-only bullets. In text-only hosts, render each chip's `text_fallback` instead. Typical gated set: `Inspect evidence`, `Approve generation...`, `Open primary artifact`; after approval but before continuation: `Open primary artifact`, `Inspect evidence`, `Resume run`. Must be the last element of the response; do not place any text after this section.

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- If `.canon/` is missing, point to `$canon-init`.
- If the user is really still deciding the change boundary or is adding feature behavior, redirect to `$canon-change` instead of guessing preserved behavior or no-feature-addition proof.
- If Canon returns `AwaitingApproval`, surface the exact target Canon produced and do not imply the run is complete.
- If approval is already recorded and Canon still returns `AwaitingApproval`, say directly that the run now needs `$canon-resume`, not another approval.
- If Canon returns a blocked packet, point first to the emitted refactor artifacts that name the missing preserved behavior, scope, drift review, or no-feature-addition evidence.
- If Canon returns recommendation-only or approved-recommendation guidance without a local patch payload, say directly that Canon did not mutate the workspace in this tranche.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- Use `$canon-inspect-evidence` when the user needs invocation rationale or policy decisions.
- Use `$canon-change` when the real gap is still bounded planning or feature-delivery scoping.
- Use `$canon-review` when the packet needs explicit non-PR disposition.

## Related Skills

- `$canon-status`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
- `$canon-change`
- `$canon-review`
