---
name: canon-supply-chain-analysis
description: Use when you need a governed supply-chain-analysis packet for an existing repository with explicit SBOM, vulnerability, license, and legacy posture evidence.
---

# Canon Supply Chain Analysis

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon supply-chain-analysis workflow as a governed run
started from your AI assistant.

## When To Trigger

- The user needs a governed packet for dependency posture, SBOM, license
  compatibility, vulnerability triage, or legacy modernization pressure.
- The user already has a bounded repository surface and wants Canon to persist
  scanner-backed evidence plus explicit coverage gaps.

## When It Must Not Trigger

- The user still needs to define the system or repository boundary; use
  `$canon-discovery`, `$canon-architecture`, or `$canon-change` first.
- The user really needs a bounded threat-and-mitigation packet; use
  `$canon-security-assessment`.
- The user is explicitly asking to inspect, approve, or continue an existing
  run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one supply-chain-analysis brief file, one supply-chain-analysis input
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
- For auto-binding only, treat `canon-input/supply-chain-analysis.md` or
  `canon-input/supply-chain-analysis/` as the canonical authored-input
  locations for this mode.
- For a folder-backed packet under `canon-input/supply-chain-analysis/`, treat
  `brief.md` as the authoritative brief and any sibling notes as carried
  context; the brief still needs to restate the bounded manifest surface and
  policy posture directly.
- For an explicit inline note, pass it through `--input-text` instead of
  materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or published packets.
- If the user has not declared licensing posture or non-OSS tool policy, ask
  before invoking Canon instead of guessing.
- If the repo surface is still too broad, ask the user to narrow the in-scope
  manifests or directories before starting the run.
- If risk is invalid, ask with guided fixed choices: `low-impact`,
  `bounded-impact`, or `systemic-impact`.
- If zone is invalid, ask with guided fixed choices: `green`, `yellow`, or
  `red`.

## Author Supply Chain Analysis Body Before Invoking Canon

Canon does not invent the supply-chain body for you. Canon governs, validates,
and persists the packet. You (the assistant) MUST author the real dependency,
license, and legacy posture content from the bounded source material BEFORE
calling `canon run --mode supply-chain-analysis`.

Do this every time, even when the user only handed you a short dependency note:

1. Read the source inputs the user pointed at. Identify the bounded scan scope,
   licensing posture, distribution model, in-scope ecosystems, excluded paths,
   scanner policy, known findings, coverage gaps, modernization pressure, and
   deferred checks from manifests, lockfiles, docs, and the stated operating
   context. Do not guess.
2. Compose a single supply-chain-analysis brief file at
   `canon-input/supply-chain-analysis.md` or
   `canon-input/supply-chain-analysis/brief.md` (or use `--input-text` for a
   one-shot inline brief). The file MUST include all of the following H2
   sections, populated with concrete content tied to the source you just read:
   - `## Declared Scope`
   - `## Licensing Posture`
   - `## Distribution Model`
   - `## Ecosystems In Scope`
   - `## Out Of Scope Components`
   - `## Scanner Selection Rationale`
   - `## SBOM Outputs`
   - `## Findings By Severity`
   - `## Exploitability Notes`
   - `## Triage Decisions`
   - `## Compatibility Classes`
   - `## Flagged Incompatibilities`
   - `## Obligations`
   - `## Outdated Dependencies`
   - `## End Of Life Signals`
   - `## Abandonment Signals`
   - `## Modernization Slices`
   - `## Scanner Decisions`
   - `## Coverage Gaps`
   - `## Source Inputs`
   - `## Independent Checks`
   - `## Deferred Verification`
3. Each section MUST be specific to the bounded repository surface you actually
   read. Boilerplate dependency-risk language or generic legal slogans are a
   failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will install tools,
   upgrade dependencies, certify compliance, or accept risk on the user's
   behalf.
5. Then invoke Canon. Canon will preserve the authored sections into the
   supply-chain packet, surface missing-context markers honestly, emit explicit
   missing-body or coverage-gap markers when required sections are absent, and
   gate the result through risk and release-readiness checks.

If you cannot author a credible supply-chain body because the repo boundary,
licensing posture, or ecosystem scope is still too vague, say so directly and
redirect to `$canon-discovery`, `$canon-architecture`, or `$canon-change`
instead of submitting an empty brief.

### Packet Shape And Persona

Author the packet as a dependency and release-posture reviewer structuring
bounded guidance for maintainers, operators, and approvers.

- Favor explicit scope, evidence sources, coverage gaps, compatibility posture,
  modernization pressure, and deferred validation over false certainty.
- Keep the packet bounded to the authored repository surface; do not imply
  legal sign-off, scanner completeness, or remediation authority the source
  does not support.
- Persona guidance is presentation only. Missing authored sections remain
  explicit gaps and must not be backfilled with confident prose.

## Canon Command Contract

- Canon command: `canon run --mode supply-chain-analysis --system-context existing --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval
  targets Canon emits.
- Supply-chain analysis remains recommendation-only in this tranche.
- Systemic-impact or red-zone supply-chain-analysis runs surface `gate:risk`
  approval first; once approved, the packet can complete without a separate
  execution-resume path.
- Approval-gated and blocked supply-chain-analysis packets remain publishable
  when Canon emitted a readable artifact set.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the bounded supply-chain result and any explicit coverage
  or decision gaps
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/supply-chain-analysis/` paths when Canon
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
- If licensing posture or non-OSS tool policy is unresolved, ask before
  invoking Canon instead of guessing.
- If the real task is broader threat review rather than dependency posture,
  redirect to `$canon-security-assessment`.
- If Canon returns `AwaitingApproval`, surface the exact approval target Canon
  produced and keep the packet recommendation-only.
- If Canon returns a blocked packet, point first to `analysis-overview.md`,
  `sbom-bundle.md`, `vulnerability-triage.md`, `license-compliance.md`,
  `legacy-posture.md`, and `analysis-evidence.md`.
- If Canon emits coverage gaps, say directly that the packet is readable but
  still incomplete for the uncovered scanner or ecosystem surface.

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts`
  first.
- Use `$canon-inspect-evidence` when the user needs lineage, tool decisions,
  approvals, or policy rationale.
- Use `$canon-approve` when a systemic or red-zone supply-chain packet is ready
  for explicit risk approval.
- Use `$canon-change` or `$canon-migration` when the next real step is a
  bounded modernization or dependency-replacement packet.
- Use `$canon-security-assessment` when supply-chain findings need to feed a
  threat or risk review.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-change`
- `$canon-migration`
- `$canon-security-assessment`

## Canonical Authored Inputs

- `canon-input/supply-chain-analysis.md`
- `canon-input/supply-chain-analysis/`

## Output Intent

- durable supply-chain packet
- explicit SBOM, vulnerability, license, and legacy posture with coverage-gap honesty
- no autonomous installation, remediation, or compliance certification