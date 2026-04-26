# Quickstart: Architecture ADR And Options

## Positive Scenario

1. Author an architecture brief at `canon-input/architecture.md` that includes:
   - the existing decision and C4 sections
   - explicit authored sections for decision drivers, options considered, pros, cons, recommendation, and why-not rationale
2. Run Canon in `architecture` mode with the usual risk, zone, and `--system-context` inputs.
3. Inspect the emitted packet and confirm:
   - `architecture-decisions.md` reads like a real ADR-backed decision artifact
   - `tradeoff-matrix.md` shows the authored option analysis
   - `system-context.md`, `container-view.md`, and `component-view.md` remain unchanged in behavior

## Negative Scenario

1. Remove one required authored decision section, such as `## Why Not The Others`, from the same brief.
2. Run the same `architecture` flow again.
3. Inspect the emitted decision-facing artifact and confirm it includes `## Missing Authored Body` naming the missing section.

## Review Expectations

- A reviewer can identify the winning option, rejected options, and rationale from the packet alone.
- Missing authored context stays explicit instead of being silently filled.
- C4 context remains inspectable in the same packet.