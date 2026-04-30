---
name: canon-system-assessment
description: Use when you need a governed system-assessment packet for an existing system with ISO 42010 coverage, explicit observed findings, inferred findings, and assessment gaps, and publishable evidence.
---

# Canon System Assessment

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon system-assessment workflow as a governed as-is
architecture packet started from your AI assistant.

## When To Trigger

- The user needs an as-is system assessment for an existing system before
  choosing change, architecture, migration, or security follow-up work.
- The user already has a bounded assessment brief and wants Canon to persist
  coverage, views, dependencies, risks, and evidence gaps without inventing a
  future-state design.

## When It Must Not Trigger

- The user is making a future-state architecture decision rather than assessing
  an existing system as-is; use `$canon-architecture`.
- The user still lacks a bounded system surface or stakeholder concern set; use
  `$canon-discovery` first.
- The user really needs a security or supply-chain packet instead of a broader
  system assessment; use `$canon-security-assessment` or
  `$canon-supply-chain-analysis`.
- The user is explicitly asking to inspect, approve, or continue an existing
  run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one system-assessment brief file, one system-assessment input folder,
  or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Always bind `--system-context existing` for this skill.
- Verify risk, zone, and at least one authored input are present before
  invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/system-assessment.md` or
  `canon-input/system-assessment/` as the canonical authored-input locations
  for this mode.
- For a folder-backed packet under `canon-input/system-assessment/`, treat
  `brief.md` as the authoritative brief and any sibling notes as carried
  context; the current brief still needs to restate the bounded assessment
  directly.
- For an explicit inline note, pass it through `--input-text` instead of
  materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or published packets.
- If the user still lacks a bounded assessment objective, stakeholder set,
  primary concern list, or credible evidence surface, redirect to
  `$canon-discovery` or `$canon-architecture` instead of inventing structure.
- If risk is invalid, ask with guided fixed choices: `low-impact`,
  `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or
  `red`.

## Author System Assessment Body Before Invoking Canon

Canon does not invent the assessment body for you. Canon governs, validates,
and persists the packet. You (the assistant) MUST author the real system
assessment content from the bounded source material BEFORE calling
`canon run --mode system-assessment`.

Do this every time, even when the user only handed you a short note:

1. Read the source inputs the user pointed at. Identify the assessment
   objective, stakeholders, concerns, views, assets, dependencies, risks,
   evidence basis, and unresolved gaps from code, configs, docs, diagrams, and
   stated operating context. Do not guess.
2. Compose a single system-assessment brief file at
   `canon-input/system-assessment.md` or
   `canon-input/system-assessment/brief.md` (or use `--input-text` for a
   one-shot inline brief). The file MUST include all of the following H2
   sections, populated with concrete content tied to the source you just read:
   - `## Assessment Objective`
   - `## Stakeholders`
   - `## Primary Concerns`
   - `## Assessment Posture`
   - `## Stakeholder Concerns`
   - `## Assessed Views`
   - `## Partial Or Skipped Coverage`
   - `## Confidence By Surface`
   - `## Assessed Assets`
   - `## Critical Dependencies`
   - `## Boundary Notes`
   - `## Ownership Signals`
   - `## Responsibilities`
   - `## Primary Flows`
   - `## Observed Boundaries`
   - `## Components`
   - `## Interfaces`
   - `## Confidence Notes`
   - `## Execution Environments`
   - `## Network And Runtime Boundaries`
   - `## Deployment Signals`
   - `## Coverage Gaps`
   - `## Technology Stack`
   - `## Platform Dependencies`
   - `## Version Or Lifecycle Signals`
   - `## Evidence Gaps`
   - `## Integrations`
   - `## Data Exchanges`
   - `## Trust And Failure Boundaries`
   - `## Inference Notes`
   - `## Observed Risks`
   - `## Risk Triggers`
   - `## Impact Notes`
   - `## Likely Follow-On Modes`
  - `## Observed Findings`
  - `## Inferred Findings`
  - `## Assessment Gaps`
   - `## Evidence Sources`
3. Each section MUST be specific to the bounded system surface you actually
   read. Boilerplate architecture language or generic modernization slogans are
   a failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will redesign the
   system, prove complete coverage, or certify the architecture on the user's
   behalf.
5. Then invoke Canon. Canon will preserve the authored sections into the
   system-assessment packet, surface missing-context markers honestly, emit the
   exact `## Missing Authored Body` marker when required sections are absent,
   and gate the result through architecture, risk, and release-readiness
   checks.

If you cannot author a credible as-is assessment because the boundary or
evidence surface is still too vague, say so directly and redirect to
`$canon-discovery` or `$canon-architecture` instead of submitting an empty
brief.

### Packet Shape And Persona

Author the packet as a systems assessor explaining the current architecture to
maintainers, reviewers, and approvers using ISO 42010-style viewpoint coverage.

- Favor explicit scope, viewpoint coverage, assets, dependencies, evidence,
  risks, and documented gaps over solutioneering.
- Keep the packet bounded to the authored system surface; do not imply complete
  repository coverage, future-state approval, or operational certainty the
  source does not support.
- Persona guidance is presentation only. Missing authored sections remain
  explicit gaps and must not be backfilled with confident architecture prose.

## Canon Command Contract

- Canon command: `canon run --mode system-assessment --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval
  targets Canon emits.
- System assessment remains recommendation-only in this tranche.
- Systemic-impact or red-zone system-assessment runs surface `gate:risk`
  approval first; once approved, the packet can complete without a separate
  execution-resume path.
- Approval-gated and blocked system-assessment packets remain publishable when
  Canon emitted a readable artifact set.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the assessment objective, covered views, and explicit
  evidence gaps
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/system-assessment/` paths when Canon
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
- If the system boundary is still too ambiguous to assess credibly, redirect to
  `$canon-discovery` or `$canon-architecture` instead of guessing structure.
- If the real task is an incident, bounded change plan, or threat review,
  redirect to `$canon-incident`, `$canon-change`, or
  `$canon-security-assessment` instead of pretending the system packet answers
  it.
- If Canon returns `AwaitingApproval`, surface the exact approval target Canon
  produced and keep the packet recommendation-only.
- If Canon returns a blocked packet, point first to `assessment-overview.md`,
  `coverage-map.md`, `component-view.md`, `integration-view.md`,
  `risk-register.md`, and `assessment-evidence.md`.
- If Canon emits missing-context markers, say directly that the packet is
  readable but not yet credible enough to imply confident architectural
  coverage.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts`
  first.
- Use `$canon-inspect-evidence` when the user needs lineage, approvals, or
  policy rationale.
- Use `$canon-approve` when a systemic or red-zone system packet is ready for
  explicit risk approval.
- Use `$canon-architecture` when the next real step is future-state decision
  work.
- Use `$canon-change` when the next real step is bounded modification planning.
- Use `$canon-security-assessment` when the next real step is targeted threat
  and risk review rather than broader architecture coverage.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-discovery`
- `$canon-architecture`
- `$canon-change`
- `$canon-security-assessment`

## Canonical Authored Inputs

- `canon-input/system-assessment.md`
- `canon-input/system-assessment/`

## Output Intent

- durable as-is system assessment packet
- explicit viewpoint coverage, risks, evidence posture, and uncovered surfaces
- no autonomous redesign, remediation, or certification claims