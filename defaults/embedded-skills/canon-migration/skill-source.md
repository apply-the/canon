---
name: canon-migration
description: Use when you need a governed migration packet for an existing system with explicit compatibility, sequencing, and fallback posture.
---

# Canon Migration

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon migration workflow as a governed run started from
your AI assistant.

## When To Trigger

- The user needs a governed migration packet for an existing system.
- The user already has a bounded migration brief and wants Canon to persist
	compatibility, sequencing, fallback, and residual-risk evidence.

## When It Must Not Trigger

- The user still needs to shape a target architecture or migration boundary;
	use `$canon-architecture` or `$canon-system-shaping` first.
- The user really needs bounded change planning rather than a compatibility or
	cutover packet; use `$canon-change`.
- The user is explicitly asking to inspect, approve, or continue an existing
	run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one migration brief file, one migration input folder, or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- Always bind `--system-context existing` for this skill.
- Verify risk, zone, and at least one authored input are present before invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/migration.md` or `canon-input/migration/` as the canonical authored-input locations for this mode.
- For a folder-backed packet under `canon-input/migration/`, treat `brief.md` as the authoritative migration brief and any sibling notes as carried-forward context; the current brief still needs to restate the bounded transition surface directly.
- For an explicit inline note, pass it through `--input-text` instead of materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent `.canon/` artifacts, or published packets.
- If the user still lacks a bounded source state, target state, compatibility expectation, or fallback path, redirect to `$canon-architecture`, `$canon-system-shaping`, or `$canon-change` instead of inventing migration detail.
- If risk is invalid, ask with guided fixed choices: `low-impact`, `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or `red`.

## Author Migration Body Before Invoking Canon

Canon does not invent the migration body for you. Canon governs, validates,
and persists the packet. You (the assistant) MUST author the real migration
content from the bounded source material BEFORE calling
`canon run --mode migration`.

Do this every time, even when the user only handed you a short rollout note:

1. Read the source inputs the user pointed at. Identify the real source state, target state, compatibility contract, decision alternatives, ecosystem health, sequencing pressure, adoption implications, and fallback credibility from code, configs, docs, and the stated rollout context. Do not guess.
2. Compose a single migration brief file at `canon-input/migration/brief.md` (or use `--input-text` for a one-shot inline brief). The file MUST include all of the following H2 sections, populated with concrete content tied to the source you just read:
	 - `## Current State`
	 - `## Target State`
	 - `## Transition Boundaries`
	 - `## Guaranteed Compatibility`
	 - `## Temporary Incompatibilities`
	 - `## Coexistence Rules`
	 - `## Options Matrix`
	 - `## Ordered Steps`
	 - `## Parallelizable Work`
	 - `## Cutover Criteria`
	 - `## Rollback Triggers`
	 - `## Fallback Paths`
	 - `## Re-Entry Criteria`
	 - `## Adoption Implications`
	 - `## Verification Checks`
	 - `## Residual Risks`
	 - `## Release Readiness`
	 - `## Migration Decisions`
	 - `## Tradeoff Analysis`
	 - `## Recommendation`
	 - `## Ecosystem Health`
	 - `## Deferred Decisions`
	 - `## Approval Notes`
3. Each section MUST be specific to the bounded migration surface you actually read. Boilerplate rollout text or vague coexistence claims are a failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will execute cutover, rollback, or deployment steps on the user's behalf.
5. Then invoke Canon. Canon will preserve the authored sections into the migration packet, surface missing-context markers honestly, emit the exact `## Missing Authored Body` marker when required sections are absent, and gate the result through exploration, architecture, migration-safety, risk, and readiness checks.

If you cannot author a credible migration body because compatibility or fallback is still too vague, say so directly and redirect to `$canon-architecture`, `$canon-system-shaping`, or `$canon-change` instead of submitting an empty brief.

### Packet Shape And Persona

Author the packet as a migration lead comparing rollout paths for operators,
reviewers, and approvers.

- Favor compatibility tradeoffs, options matrix decisions, ecosystem health,
	adoption implications, and rollback credibility over generic rollout
	optimism.
- Keep recommendations bounded to the authored transition surface; do not
	imply cutover approval or operational certainty the source does not support.
- Persona guidance is presentation only. Missing authored sections remain
	explicit gaps and must not be backfilled with migration-sounding prose.

## Canon Command Contract

- Canon command: `canon run --mode migration --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval targets Canon emits.
- Migration remains recommendation-only in this tranche.
- Systemic-impact or red-zone migration runs surface `gate:risk` approval first; once approved, the packet can complete without a separate execution-resume path.
- Blocked migration packets remain publishable when Canon emitted a readable artifact set.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the compatibility, sequencing, and fallback result
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/migration/` paths when Canon emitted them
- blocked gates or approval targets when present
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon already returned in `mode_result.action_chips`; do not collapse them to label-only bullets. In text-only hosts, render each chip's `text_fallback` instead. Must be the last element of the response; do not place any text after this section.

## Failure Handling Guidance

- If `canon` is missing, show the supported install path from README.
- If `.canon/` is missing, point to `$canon-init`.
- If the migration boundary is still structurally unclear, redirect to `$canon-architecture` or `$canon-system-shaping` instead of guessing compatibility or fallback detail.
- If the real task is bounded code-change planning, redirect to `$canon-change` instead of pretending the migration packet is the execution plan.
- If Canon returns `AwaitingApproval`, surface the exact approval target Canon produced and keep the packet recommendation-only.
- If Canon returns a blocked packet, point first to `source-target-map.md`, `compatibility-matrix.md`, `sequencing-plan.md`, `fallback-plan.md`, and `migration-verification-report.md`.
- If Canon emits missing-context markers, say directly that the packet is readable but not yet credible enough to imply rollout readiness.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- Use `$canon-inspect-evidence` when the user needs lineage, approvals, or policy rationale.
- Use `$canon-approve` when a systemic or red-zone migration packet is ready for explicit risk approval.
- Use `$canon-change` when the next real step is bounded live-codebase change planning.
- Use `$canon-architecture` when the real gap is still compatibility or boundary design rather than rollout sequencing.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-change`
- `$canon-architecture`

## Canonical Authored Inputs

- `canon-input/migration.md`
- `canon-input/migration/`

## Output Intent

- durable migration compatibility packet
- explicit sequencing, fallback, and residual-risk posture
- no autonomous operational execution
