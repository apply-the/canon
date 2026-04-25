# Data Model: Stronger Architecture Outputs (C4 Model)

This feature does not introduce a new persistent runtime entity. It extends the architecture artifact contract and the architecture renderer's authored-section extraction.

## Architecture Artifact Contract (extended)

Existing artifacts in the architecture set:

- `architecture-decisions.md` — preserved, unchanged.
- `invariants.md` — preserved, unchanged.
- `tradeoff-matrix.md` — preserved, unchanged.
- `boundary-map.md` — preserved, unchanged.
- `readiness-assessment.md` — preserved, unchanged.

New artifacts in the architecture set:

| Artifact name           | Authored brief H2     | Gates                                            | Notes                                          |
|-------------------------|-----------------------|--------------------------------------------------|------------------------------------------------|
| `system-context.md`     | `## System Context`   | `Architecture`, `Exploration`                    | C4 Level 1 view: system + actors + dependencies |
| `container-view.md`     | `## Containers`       | `Architecture`                                   | C4 Level 2 view: bounded containers             |
| `component-view.md`     | `## Components`       | `Architecture`, `ReleaseReadiness`               | C4 Level 3 view: components in primary container |

## Authored Section Extraction Rules

- The renderer reads the architecture brief and extracts each canonical H2 section by exact heading match (case-sensitive on the literal text, ignoring leading/trailing whitespace).
- The body of each authored H2 section is everything until the next H2 or end of document, preserved verbatim, including fenced code blocks (Mermaid, PlantUML), tables, and nested H3+ headings.
- When a section is absent or its body is empty after trimming whitespace, the renderer emits the artifact with the structure:
  ```
  # <Title derived from artifact>

  ## Missing Authored Body

  No `<canonical heading>` section was authored in the supplied brief.
  Author this section in the architecture brief and rerun.
  ```

## Brief Shape (informational)

The new authored-section extraction in this slice is load-bearing only for the C4 sections. The legacy critique-first artifacts continue to be rendered from the existing context/generation/critique summaries the orchestrator already supplies.

A complete authored architecture brief is therefore expected to include the following H2 sections, with the C4 ones being the ones the new renderer actively extracts:

- `## Decisions` (informational)
- `## Invariants` (informational)
- `## Tradeoffs` (informational)
- `## Boundaries` (informational)
- `## Readiness` (informational)
- `## System Context` (load-bearing for `system-context.md`)
- `## Containers` (load-bearing for `container-view.md`)
- `## Components` (load-bearing for `component-view.md`)

Order in the brief is informational; only canonical-heading exact match drives extraction. The renderer reads the C4 sections from the run's `context_summary` argument, which the orchestrator already populates from the supplied authored brief.

## State Transitions

No new runtime states. The architecture run continues to follow its existing lifecycle (Pending → Running → Completed/Blocked/AwaitingApproval).

## Validation Rules

- Each new artifact MUST be present in the architecture artifact contract list.
- Each new artifact MUST be either authored (verbatim body present) or marked with `## Missing Authored Body`.
- The renderer MUST NOT alter the existing five legacy architecture artifact bodies.
