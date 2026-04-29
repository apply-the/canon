---
name: canon-security-assessment
description: Use when you need a governed security-assessment packet for an existing system with explicit threats, risks, mitigations, and evidence gaps.
---

# Canon Security Assessment

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon security-assessment workflow as a governed run
started from your AI assistant.

## When To Trigger

- The user needs a governed security packet for an existing system.
- The user already has a bounded security brief and wants Canon to persist
  threats, risks, mitigations, assumptions, and evidence gaps.

## When It Must Not Trigger

- The user still needs to shape the system boundary or define the target
  surface; use `$canon-architecture`, `$canon-system-shaping`, or
  `$canon-change` first.
- The user really needs incident containment rather than a bounded threat and
  risk packet; use `$canon-incident`.
- The user is explicitly asking to inspect, approve, or continue an existing
  run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one security-assessment brief file, one security-assessment input
  folder, or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Always bind `--system-context existing` for this skill.
- Verify risk, zone, and at least one authored input are present before
  invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/security-assessment.md` or
  `canon-input/security-assessment/` as the canonical authored-input locations
  for this mode.
- For a folder-backed packet under `canon-input/security-assessment/`, treat
  `brief.md` as the authoritative brief and any sibling notes as carried
  context; the current brief still needs to restate the bounded surface
  directly.
- For an explicit inline note, pass it through `--input-text` instead of
  materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or published packets.
- If the user still lacks a bounded in-scope asset inventory, trust boundary,
  or evidence basis, redirect to `$canon-architecture` or `$canon-change`
  instead of inventing security detail.
- If risk is invalid, ask with guided fixed choices: `low-impact`,
  `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or
  `red`.

## Author Security Assessment Body Before Invoking Canon

Canon does not invent the security body for you. Canon governs, validates, and
persists the packet. You (the assistant) MUST author the real security content
from the bounded source material BEFORE calling
`canon run --mode security-assessment`.

Do this every time, even when the user only handed you a short security note:

1. Read the source inputs the user pointed at. Identify the bounded assessment
   scope, in-scope assets, trust boundaries, threats, rated risks, recommended
   controls, evidence gaps, compliance anchors, and deferred checks from code,
   configs, docs, and the stated operating context. Do not guess.
2. Compose a single security-assessment brief file at
   `canon-input/security-assessment.md` or `canon-input/security-assessment/brief.md`
   (or use `--input-text` for a one-shot inline brief). The file MUST include
   all of the following H2 sections, populated with concrete content tied to
   the source you just read:
   - `## Assessment Scope`
   - `## In-Scope Assets`
   - `## Trust Boundaries`
   - `## Out Of Scope`
   - `## Threat Inventory`
   - `## Attacker Goals`
   - `## Boundary Threats`
   - `## Risk Findings`
   - `## Likelihood And Impact`
   - `## Proposed Owners`
   - `## Recommended Controls`
   - `## Tradeoffs`
   - `## Sequencing Notes`
   - `## Assumptions`
   - `## Evidence Gaps`
   - `## Unobservable Surfaces`
   - `## Applicable Standards`
   - `## Control Families`
   - `## Scope Limits`
   - `## Source Inputs`
   - `## Independent Checks`
   - `## Deferred Verification`
3. Each section MUST be specific to the bounded system surface you actually
   read. Boilerplate threat-model language or generic security slogans are a
   failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will apply
   mitigations, enforce policy, or certify compliance on the user's behalf.
5. Then invoke Canon. Canon will preserve the authored sections into the
   security packet, surface missing-context markers honestly, emit the exact
   `## Missing Authored Body` marker when required sections are absent, and
   gate the result through architecture, risk, and release-readiness checks.

If you cannot author a credible security body because the boundary or evidence
surface is still too vague, say so directly and redirect to
`$canon-architecture` or `$canon-change` instead of submitting an empty brief.

### Packet Shape And Persona

Author the packet as a security reviewer structuring bounded threat and risk
guidance for maintainers, operators, and approvers.

- Favor explicit scope, trust boundaries, threat reasoning, risk ratings,
  mitigation tradeoffs, and evidence gaps over compliance theater.
- Keep the packet bounded to the authored system surface; do not imply audit
  completion, remediation authority, or live scanner certainty the source does
  not support.
- Persona guidance is presentation only. Missing authored sections remain
  explicit gaps and must not be backfilled with confident security prose.

## Canon Command Contract

- Canon command: `canon run --mode security-assessment --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval targets
  Canon emits.
- Security assessment remains recommendation-only in this tranche.
- Systemic-impact or red-zone security-assessment runs surface `gate:risk`
  approval first; once approved, the packet can complete without a separate
  execution-resume path.
- Approval-gated and blocked security-assessment packets remain publishable
  when Canon emitted a readable artifact set.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the bounded security result and any explicit evidence gaps
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/security-assessment/` paths when Canon
  emitted them
- blocked gates or approval targets when present
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon
  already returned in `mode_result.action_chips`; do not collapse them to
  label-only bullets. In text-only hosts, render each chip's `text_fallback`
  instead. Must be the last element of the response; do not place any text
  after this section.

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- If `.canon/` is missing, point to `$canon-init`.
- If the security boundary is still too ambiguous to assess credibly, redirect
  to `$canon-architecture` or `$canon-change` instead of guessing threats or
  mitigations.
- If the real task is incident containment, redirect to `$canon-incident`
  instead of pretending the security packet is an outage workflow.
- If Canon returns `AwaitingApproval`, surface the exact approval target Canon
  produced and keep the packet recommendation-only.
- If Canon returns a blocked packet, point first to `assessment-overview.md`,
  `threat-model.md`, `risk-register.md`, `mitigations.md`,
  `assumptions-and-gaps.md`, and `assessment-evidence.md`.
- If Canon emits missing-context markers, say directly that the packet is
  readable but not yet credible enough to imply a confident security posture.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts`
  first.
- Use `$canon-inspect-evidence` when the user needs lineage, approvals, or
  policy rationale.
- Use `$canon-approve` when a systemic or red-zone security packet is ready for
  explicit risk approval.
- Use `$canon-change` when the next real step is bounded live-codebase change
  planning for a mitigation.
- Use `$canon-architecture` when the real gap is still boundary definition
  rather than threat review.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-change`
- `$canon-architecture`
- `$canon-incident`

## Canonical Authored Inputs

- `canon-input/security-assessment.md`
- `canon-input/security-assessment/`

## Output Intent

- durable security assessment packet
- explicit scope, threats, risks, mitigations, and evidence-gap posture
- no autonomous remediation or compliance certification