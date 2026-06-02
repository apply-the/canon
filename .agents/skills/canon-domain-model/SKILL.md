---
name: canon-domain-model
description: Use when you need a governed domain-model packet that formalizes domain concepts, relationships, invariants, and feature-impact rules before architecture or backlog decomposition.
---

# Canon Domain Model

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon domain-model workflow as a governed run
started from your AI assistant.

## When To Trigger

- The user needs to formalize domain concepts, relationships, and invariants.
- The user already has a bounded model brief and wants Canon to persist
  concept catalogs, relationship maps, invariants, and feature-impact rules.

## When It Must Not Trigger

- The user still needs to stabilize the shared vocabulary; use
  `$canon-domain-language` first.
- The user needs architecture boundary decisions; use `$canon-architecture`
  after domain-model.
- The user is explicitly asking to inspect, approve, or continue an existing
  run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one domain-model brief file, one domain-model input
  folder, or one inline note

Optional:

- `OWNER` when the user wants to override Git-derived ownership explicitly

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Verify `.canon/` exists. If missing, point to `$canon-init`.
- `--system-context` is optional for this mode.
- Verify risk, zone, and at least one authored input are present before
  invoking Canon.
- Treat authored inputs under `canon-input/` as read-only source material.
- For auto-binding only, treat `canon-input/domain-model.md` or
  `canon-input/domain-model/` as the canonical authored-input locations
  for this mode.
- For a folder-backed packet under `canon-input/domain-model/`, treat
  `brief.md` as the authoritative brief and any sibling notes as carried
  context.
- For an explicit inline note, pass it through `--input-text` instead of
  materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or published packets.

## Author Domain Model Body Before Invoking Canon

Canon does not invent the model body for you. Canon governs, validates, and
persists the packet. You (the assistant) MUST author the real model content
from the bounded source material BEFORE calling
`canon run --mode domain-model`.

Do this every time, even when the user only handed you a short domain note:

1. Read the source inputs the user pointed at. Identify the domain scope,
   concepts, relationships, bounded contexts, lifecycles, invariants,
   policies, feature-impact rules, and code/data mappings from the existing
   codebase, upstream language packet, and stated domain context.
2. Compose a single domain-model brief file at
   `canon-input/domain-model.md` or `canon-input/domain-model/brief.md`
   (or use `--input-text` for a one-shot inline brief). The file MUST include
   the following H2 sections populated with concrete content:
   - `## Domain Scope`
   - `## Model Maturity`
   - `## Upstream Sources`
   - `## Downstream Consumers`
   - `## Concepts`
   - `## Ownership Boundaries`
   - `## Open Gaps`
   - `## Relationships`
   - `## Cardinality Rules`
   - `## Boundary Crossings`
   - `## Bounded Contexts`
   - `## Context Relationships`
   - `## Integration Seams`
   - `## Entity Lifecycles`
   - `## State Transitions`
   - `## Invariant Guards`
   - `## Invariants`
   - `## Enforcement Points`
   - `## Violation Consequences`
   - `## Business Policies`
   - `## Constraint Rules`
   - `## Exception Handling`
   - `## Impact Rules`
   - `## Affected Concepts`
   - `## Downstream Effects`
   - `## Code Mapping`
   - `## Data Store Mapping`
   - `## Alignment Gaps`
   - `## Model Gaps`
   - `## Risk Signals`
   - `## Recommended Follow-Ups`
   - `## Consumer Modes`
   - `## Handoff Expectations`
   - `## Adoption Risks`
3. Each section MUST be specific to the bounded domain you actually read.
   Generic modeling advice is a failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will implement
   model changes or enforce invariants on the user's behalf.

### Packet Shape And Persona

Author the packet as a domain architect structuring bounded concept guidance
for developers, architects, and delivery teams.

- Favor explicit concepts, relationships, invariants, and enforcement points
  over abstract modeling frameworks.
- Keep the packet bounded to the authored domain scope.
- Persona guidance is presentation only. Missing authored sections remain
  explicit gaps and must not be backfilled with confident modeling claims.

## Canon Command Contract

- Canon command: `canon run --mode domain-model --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval targets.
- Domain-model remains recommendation-only in this tranche.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the bounded model result and any explicit gaps
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/domain-model/` paths when Canon
  emitted them
- blocked gates or approval targets when present
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon
  already returned in `mode_result.action_chips`

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- Use `$canon-architecture` when the next real step is structural decisions informed by the model.
- Use `$canon-backlog` when the model is stable enough for epic decomposition.
- Use `$canon-change` when model gaps reveal bounded modifications needed.
- Use `$canon-publish` to promote the packet to `tech-docs/domain/model/`.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-domain-language`
- `$canon-architecture`
- `$canon-backlog`
- `$canon-publish`

## Canonical Authored Inputs

- `canon-input/domain-model.md`
- `canon-input/domain-model/`

## Output Intent

- durable domain-model packet
- explicit concepts, relationships, invariants, and feature-impact rules
