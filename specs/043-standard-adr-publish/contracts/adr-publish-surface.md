# Contract: ADR Publish Surface

## Purpose

Define the externally visible CLI and publish behavior for standard ADR generation.

## Command Surface

```text
canon publish <RUN_ID> [--to <PATH>] [--adr]
```

- `RUN_ID`: existing publish target.
- `--to <PATH>`: existing packet publish destination override.
- `--adr`: explicit ADR export request for opt-in modes.

## Mode Behavior Matrix

| Mode | Default ADR behavior | `--adr` behavior | Result |
|------|----------------------|------------------|--------|
| `architecture` | default-on | allowed but redundant | publish packet and create one ADR in `tech-docs/adr/` |
| `change` | off | enable ADR export | publish packet and create one ADR in `tech-docs/adr/` |
| `migration` | off | enable ADR export | publish packet and create one ADR in `tech-docs/adr/` |
| unsupported modes | off | invalid | return validation error; do not create ADR |

## ADR Output Contract

When ADR export occurs, Canon writes one Markdown file to:

```text
tech-docs/adr/ADR-XXXX-<slug>.md
```

The ADR body MUST contain:

- `# ADR XXXX: <Title>`
- `**Date:** YYYY-MM-DD`
- `**Status:** Accepted`
- `## Context`
- `## Decision`
- `## Consequences`

Optional sections MAY be included when supported by the packet evidence:

- `## Alternatives Considered`
- `## Source Packet`

## Numbering Rules

- Parse existing `tech-docs/adr/ADR-XXXX-*.md` files.
- Use the next non-conflicting numeric identifier.
- Do not rename, rewrite, or compress gaps in existing ADR numbering.

## Source Mapping Rules

- `architecture`: derive from `architecture-overview.md`, `architecture-decisions.md`, `tradeoff-matrix.md`, `readiness-assessment.md`, and related packet metadata when present.
- `change`: derive from `decision-record.md`, `implementation-plan.md`, `change-surface.md`, and related packet metadata when ADR export is requested.
- `migration`: derive from `decision-record.md`, `source-target-map.md`, `compatibility-matrix.md`, and related packet metadata when ADR export is requested.

## Honesty Rules

- If source packet sections contain explicit missing-body, missing-context, or downgraded-evidence markers, the generated ADR MUST preserve those signals instead of fabricating canonical prose.
- Unsupported modes MUST not silently ignore `--adr`; they must reject the request clearly.
- `--to` changes the packet publish location only; ADR registry output remains under `tech-docs/adr/`.

## Summary Surface

`canon publish` output and `PublishSummary` must surface ADR publication when it occurs by including the generated ADR file in the published files list.