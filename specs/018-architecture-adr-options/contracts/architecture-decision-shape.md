# Contract: Architecture Decision Shape

## Purpose

Define the authored-body contract for the strengthened `architecture` decision surfaces in feature `018-architecture-adr-options`.

## Artifact Coverage

- `architecture-decisions.md`
- `tradeoff-matrix.md`
- `readiness-assessment.md`
- Existing C4 artifacts remain additive and unchanged in contract.

## Required Decision-Facing Sections Introduced By This Feature

These sections are the new authored contract this feature adds for the
decision-facing architecture slice. Missing-body tests in this feature must
target this set.

- `## Decision Drivers`
- `## Options Considered`
- `## Pros`
- `## Cons`
- `## Recommendation`
- `## Why Not The Others`

## Existing Architecture Sections Still In Scope

These sections already exist in the architecture brief and remain part of the
overall authored packet, but they are not the new missing-body focus of this
feature slice:

- `## Decision`
- `## Constraints`
- `## Evaluation Criteria`

## ADR Output Compatibility

`architecture-decisions.md` now emits an ADR-like `## Consequences` section.
This feature keeps authored-input compatibility explicit:

- `## Consequences` is the canonical heading for new authored architecture briefs.
- `## Risks` remains an accepted backward-compatible input alias for existing briefs and is rendered as `## Consequences` in the emitted artifact.

## Existing C4 Sections That Remain Unchanged

These sections keep their current authored contract and current missing-body
behavior. This feature must not weaken or reinterpret them.

- `## System Context`
- `## Containers`
- `## Components`

## Full Authored Surface Reference

The `architecture` authored brief must provide canonical H2 sections for the decision-facing slice:

- `## Decision`
- `## Constraints`
- `## Evaluation Criteria`
- `## Decision Drivers`
- `## Options Considered`
- `## Pros`
- `## Cons`
- `## Recommendation`
- `## Why Not The Others`
- `## Consequences`
- `## Risks` (legacy alias accepted)
- `## System Context`
- `## Containers`
- `## Components`

## Rendering Rules

- When a canonical authored section exists, the renderer preserves that section body verbatim in the appropriate architecture artifact.
- When the authored brief uses `## Risks`, the renderer treats it as a backward-compatible alias and emits that body under `## Consequences` in `architecture-decisions.md`.
- When one of the required decision-facing sections introduced by this feature is absent, the emitted decision-facing artifact includes `## Missing Authored Body` naming the canonical heading.
- Existing C4 missing-body behavior stays governed by the current C4 contract and tests; this feature does not expand or narrow that behavior.
- Decision-facing sections must not overwrite or weaken the behavior of the existing C4 artifacts.
- Option-analysis sections may explicitly state that only one viable option remains, but must not fabricate alternatives when the choice is materially closed.

## Non-Goals

- No new artifact file names in this slice.
- No live ecosystem evidence lookup in this slice.
- No changes to publish destinations, risk gates, or run identity.