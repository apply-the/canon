# Architecture C4 Artifact Contract

This contract document captures the surface that the implementation must preserve. It mirrors the data model and is the source of truth for tests under `tests/contract/architecture_c4_contract.rs`.

## Required Artifacts

The architecture mode MUST emit the following artifacts for every architecture run:

```text
architecture-decisions.md   # legacy
invariants.md               # legacy
tradeoff-matrix.md          # legacy
boundary-map.md             # legacy
readiness-assessment.md     # legacy
system-context.md           # new (C4 Level 1)
container-view.md           # new (C4 Level 2)
component-view.md           # new (C4 Level 3)
```

Total: 8 artifacts.

## Gate Associations

| Artifact                  | Gates                                 |
|---------------------------|---------------------------------------|
| architecture-decisions.md | Architecture, Risk                    |
| invariants.md             | Architecture, ReleaseReadiness        |
| tradeoff-matrix.md        | Architecture, Risk                    |
| boundary-map.md           | Architecture, ReleaseReadiness        |
| readiness-assessment.md   | Architecture, ReleaseReadiness        |
| system-context.md         | Architecture, Exploration             |
| container-view.md         | Architecture                          |
| component-view.md         | Architecture, ReleaseReadiness        |

## Authored Headings

The renderer MUST extract these exact H2 headings from the supplied brief:

- `## System Context` → `system-context.md`
- `## Containers` → `container-view.md`
- `## Components` → `component-view.md`

## Missing-body Marker

When the brief omits or empties an authored heading, the renderer MUST produce:

```markdown
# <Artifact Title>

## Missing Authored Body

No `<canonical heading>` section was authored in the supplied brief.
Author this section in the architecture brief and rerun.
```

The literal string `## Missing Authored Body` MUST appear in the artifact body.

## Inspect Surface

`canon inspect artifacts --run <RUN_ID>` for an architecture run MUST list all 8 artifacts.

## Publish Surface

`canon publish <RUN_ID>` for an architecture run MUST land all 8 artifacts under the existing architecture publish destination, with no path layout change.
