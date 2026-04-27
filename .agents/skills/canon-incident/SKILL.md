---
name: canon-incident
description: Use when you need a governed incident packet for an existing system with explicit blast-radius, containment, and follow-up readiness.
---

# Canon Incident

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon incident workflow as a governed run started from
your AI assistant.

## When To Trigger

- The user needs a governed incident or outage packet for an existing system.
- The user already has a bounded incident brief and wants Canon to persist
	blast radius, containment, sequencing, and follow-up readiness.

## When It Must Not Trigger

- The user still needs to bound the problem or separate incident language from
	ordinary change intent; use `$canon-requirements` first.
- The user really needs bounded live-codebase change planning rather than an
	operational packet; use `$canon-change`.
- The user is explicitly asking to inspect, approve, or continue an existing
	run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one incident brief file, one incident input folder, or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Always bind `--system-context existing` for this skill.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/incident.md` or `canon-input/incident/` as the canonical authored-input locations for this mode.
- For a folder-backed packet under `canon-input/incident/`, treat `brief.md` as the authoritative incident brief and any sibling notes as carried-forward context; the current brief still needs to restate the bounded operational surface directly.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or published packets.
- If the user still lacks a bounded impacted surface, containment target, or operator-owned follow-up path, redirect to `$canon-requirements` or `$canon-change` instead of inventing incident detail.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.

## Author Incident Body Before Invoking Canon

Canon does not invent the incident body for you. Canon governs, validates, and
persists the packet. You (the assistant) MUST author the real incident content
from the bounded source material BEFORE calling `canon run --mode incident`.

Do this every time, even when the user only handed you a short outage note:

1. Read the source inputs the user pointed at. Identify the real impacted surface, constraints, and evidence gaps from code, configs, docs, and the stated operational context. Do not guess.
2. Compose a single incident brief file at `canon-input/incident/brief.md` (or use `--input-text` for a one-shot inline brief). The file MUST include all of the following H2 sections, populated with concrete content tied to the source you just read:
   - `## Incident Scope`
   - `## Trigger And Current State`
   - `## Operational Constraints`
   - `## Known Facts`
   - `## Working Hypotheses`
   - `## Evidence Gaps`
   - `## Impacted Surfaces`
   - `## Propagation Paths`
   - `## Confidence And Unknowns`
   - `## Immediate Actions`
   - `## Ordered Sequence`
   - `## Stop Conditions`
   - `## Decision Points`
   - `## Approved Actions`
   - `## Deferred Actions`
   - `## Verification Checks`
   - `## Release Readiness`
   - `## Follow-Up Work`
3. Each section MUST be specific to the bounded incident surface you actually read. Boilerplate or generic remediation slogans are a failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will execute containment, rollback, or escalation on the user's behalf.
5. Then invoke Canon. Canon will preserve the authored sections into the incident packet, surface missing-context markers honestly, emit the exact `## Missing Authored Body` marker when required sections are absent, and gate the result through risk, containment, architecture, and readiness checks.

If you cannot author a credible incident body because the impacted surface is still too vague, say so directly and redirect to `$canon-requirements` or `$canon-change` instead of submitting an empty brief.

## Canon Command Contract

- Canon command: `canon run --mode incident --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval targets Canon emits.
- Incident remains recommendation-only in this tranche.
- Systemic-impact or red-zone incident runs surface `gate:risk` approval first; once approved, the packet can complete without a separate execution-resume path.
- Approval-gated and blocked incident packets remain publishable when Canon emitted a readable artifact set.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the containment result and any explicit missing evidence
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/incident/` paths when Canon emitted them
- blocked gates or approval targets when present
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon already returned in `mode_result.action_chips`; do not collapse them to label-only bullets. In text-only hosts, render each chip's `text_fallback` instead. Must be the last element of the response; do not place any text after this section.

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- If `.canon/` is missing, point to `$canon-init`.
- If the incident is still too ambiguous to bound safely, redirect to `$canon-requirements` instead of guessing blast radius or containment detail.
- If the real task is bounded code-change planning, redirect to `$canon-change` instead of pretending the incident packet is the implementation plan.
- If Canon returns `AwaitingApproval`, surface the exact approval target Canon produced and keep the packet recommendation-only.
- If Canon returns a blocked packet, point first to `incident-frame.md`, `blast-radius-map.md`, `containment-plan.md`, and `follow-up-verification.md`.
- If Canon emits missing-context markers, say directly that the packet is readable but not yet ready to imply operational confidence.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- Use `$canon-inspect-evidence` when the user needs lineage, approvals, or policy rationale.
- Use `$canon-approve` when a systemic or red-zone incident packet is ready for explicit risk approval.
- Use `$canon-change` when the next real step is bounded live-codebase change planning.
- Use `$canon-review` when the user wants a governed challenge of the incident packet itself.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-change`
- `$canon-requirements`

## Canonical Authored Inputs

- `canon-input/incident.md`
- `canon-input/incident/`

## Output Intent

- durable incident containment packet
- explicit blast radius, containment, sequencing, and follow-up posture
- no autonomous operational execution
