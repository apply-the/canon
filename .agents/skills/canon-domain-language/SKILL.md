---
name: canon-domain-language
description: Use when you need a governed domain-language packet that stabilizes the shared vocabulary of a product area before downstream design or change work.
---

# Canon Domain Language

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Expose the delivered Canon domain-language workflow as a governed run
started from your AI assistant.

## When To Trigger

- The user needs to stabilize the shared vocabulary of a domain or product area.
- The user already has a bounded language brief and wants Canon to persist
  glossary, conflicts, preferred terms, and downstream guidance.

## When It Must Not Trigger

- The user still needs to explore the problem space; use `$canon-discovery` first.
- The user needs to formalize concept relationships and invariants; use
  `$canon-domain-model` after domain-language.
- The user is explicitly asking to inspect, approve, or continue an existing
  run; use the run-scoped skills instead.

## Required Inputs

- `RISK`
- `ZONE`
- either one domain-language brief file, one domain-language input
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
- For auto-binding only, treat `canon-input/domain-language.md` or
  `canon-input/domain-language/` as the canonical authored-input locations
  for this mode.
- For a folder-backed packet under `canon-input/domain-language/`, treat
  `brief.md` as the authoritative brief and any sibling notes as carried
  context.
- For an explicit inline note, pass it through `--input-text` instead of
  materializing a repo file automatically.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or published packets.

## Author Domain Language Body Before Invoking Canon

Canon does not invent the language body for you. Canon governs, validates, and
persists the packet. You (the assistant) MUST author the real language content
from the bounded source material BEFORE calling
`canon run --mode domain-language`.

Do this every time, even when the user only handed you a short domain note:

1. Read the source inputs the user pointed at. Identify the domain scope,
   candidate terms, ambiguities, conflicts, preferred language, and downstream
   consumers from code, docs, conversations, and the stated domain context.
2. Compose a single domain-language brief file at
   `canon-input/domain-language.md` or `canon-input/domain-language/brief.md`
   (or use `--input-text` for a one-shot inline brief). The file MUST include
   the following H2 sections populated with concrete content:
   - `## Domain Scope`
   - `## Language Maturity`
   - `## Upstream Sources`
   - `## Downstream Consumers`
   - `## Glossary Entries`
   - `## Source References`
   - `## Open Gaps`
   - `## Canonical Terms`
   - `## Deprecated Synonyms`
   - `## Migration Notes`
   - `## Conflict Inventory`
   - `## Resolution Status`
   - `## Escalation Triggers`
   - `## Context-Dependent Terms`
   - `## Disambiguation Rules`
   - `## Usage Examples`
   - `## Naming Conventions`
   - `## Domain Boundaries`
   - `## Enforcement Guidance`
   - `## Code Naming Patterns`
   - `## API Surface Terms`
   - `## Alignment Gaps`
   - `## Consumer Modes`
   - `## Handoff Expectations`
   - `## Adoption Risks`
3. Each section MUST be specific to the bounded domain you actually read.
   Generic language advice is a failure.
4. Keep the packet recommendation-only. Do NOT imply Canon will enforce
   naming conventions or rename code on the user's behalf.

### Packet Shape And Persona

Author the packet as a domain analyst structuring bounded vocabulary guidance
for developers, architects, and product teams.

- Favor explicit terms, conflicts, and disambiguation rules over vague
  language directives.
- Keep the packet bounded to the authored domain scope.
- Persona guidance is presentation only. Missing authored sections remain
  explicit gaps and must not be backfilled with confident language claims.

## Canon Command Contract

- Canon command: `canon run --mode domain-language --risk <RISK> --zone <ZONE> [--owner <OWNER>] (--input <INPUT_PATH> | --input-text <INPUT_TEXT>)`
- Return the real Canon run id, state, and any blocked gates or approval targets.
- Domain-language remains recommendation-only in this tranche.

## Expected Output Shape

- concise run-start or gated summary
- Canon-backed run state
- direct statement of the bounded language result and any explicit gaps
- primary artifact path and short excerpt when available
- direct statement of the active execution posture (`recommendation-only`)
- concrete `.canon/artifacts/<RUN_ID>/domain-language/` paths when Canon
  emitted them
- blocked gates or approval targets when present
- one recommended next step that keeps the run context intact
- `Action Chips:` when the host supports chips, preserve the full objects Canon
  already returned in `mode_result.action_chips`

## Next-Step Guidance

- When Canon emitted a readable packet, recommend `$canon-inspect-artifacts` first.
- Use `$canon-domain-model` when the next real step is formalizing concepts and relationships.
- Use `$canon-architecture` when the vocabulary is stable enough for boundary decisions.
- Use `$canon-change` when the next step is renaming or alignment work.
- Use `$canon-publish` to promote the packet to `tech-docs/domain/language/`.

## Related Skills

- `$canon-status`
- `$canon-inspect-artifacts`
- `$canon-inspect-evidence`
- `$canon-approve`
- `$canon-domain-model`
- `$canon-architecture`
- `$canon-publish`

## Canonical Authored Inputs

- `canon-input/domain-language.md`
- `canon-input/domain-language/`

## Output Intent

- durable domain-language packet
- explicit vocabulary, conflicts, preferred terms, and downstream guidance
