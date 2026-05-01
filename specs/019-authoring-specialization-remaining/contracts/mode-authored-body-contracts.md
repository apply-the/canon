# Contract: Mode Authored-Body Contracts

## Purpose

Define the authored-body contract for the `system-shaping`, `implementation`, and `refactor` follow-on specialization delivered in feature `019-authoring-specialization-remaining`.

## Artifact Coverage

- `system-shape.md`
- `domain-model.md`
- `architecture-outline.md`
- `capability-map.md`
- `delivery-options.md`
- `risk-hotspots.md`
- `task-mapping.md`
- `mutation-bounds.md`
- `implementation-notes.md`
- `completion-evidence.md`
- `validation-hooks.md`
- `rollback-notes.md`
- `preserved-behavior.md`
- `refactor-scope.md`
- `structural-rationale.md`
- `regression-evidence.md`
- `contract-drift-check.md`
- `no-feature-addition.md`

## System-Shaping Required Sections

For the 030 artifact-shapes follow-on, `system-shaping` should read like a domain map plus structural-options brief authored by a bounded system designer.

- `system-shape.md`
  - `## System Shape`
  - `## Boundary Decisions`
  - `## Domain Responsibilities`
- `domain-model.md`
  - `## Candidate Bounded Contexts`
  - `## Core And Supporting Domain Hypotheses`
  - `## Ubiquitous Language`
  - `## Domain Invariants`
  - `## Boundary Risks And Open Questions`
- `architecture-outline.md`
  - `## Structural Options`
  - `## Selected Boundaries`
  - `## Rationale`
  - `## Why Not The Others`
- `capability-map.md`
  - `## Capabilities`
  - `## Dependencies`
  - `## Gaps`
- `delivery-options.md`
  - `## Delivery Phases`
  - `## Sequencing Rationale`
  - `## Risk per Phase`
- `risk-hotspots.md`
  - `## Hotspots`
  - `## Mitigation Status`
  - `## Unresolved Risks`

## Implementation Required Sections

- `task-mapping.md`
  - `## Task Mapping`
  - `## Bounded Changes`
- `mutation-bounds.md`
  - `## Mutation Bounds`
  - `## Allowed Paths`
- `implementation-notes.md`
  - `## Executed Changes`
  - `## Candidate Frameworks`
  - `## Options Matrix`
  - `## Decision Evidence`
  - `## Recommendation`
  - `## Task Linkage`
- `completion-evidence.md`
  - `## Completion Evidence`
  - `## Adoption Implications`
  - `## Remaining Risks`
- `validation-hooks.md`
  - `## Ecosystem Health`
  - `## Safety-Net Evidence`
  - `## Independent Checks`
- `rollback-notes.md`
  - `## Rollback Triggers`
  - `## Rollback Steps`

## Refactor Required Sections

- `preserved-behavior.md`
  - `## Preserved Behavior`
  - `## Approved Exceptions`
- `refactor-scope.md`
  - `## Refactor Scope`
  - `## Allowed Paths`
- `structural-rationale.md`
  - `## Structural Rationale`
  - `## Untouched Surface`
- `regression-evidence.md`
  - `## Safety-Net Evidence`
  - `## Regression Findings`
- `contract-drift-check.md`
  - `## Contract Drift`
  - `## Reviewer Notes`
- `no-feature-addition.md`
  - `## Feature Audit`
  - `## Decision`

## Rendering Rules

- When a canonical authored section exists and is non-empty, the renderer preserves that section body verbatim in the corresponding emitted artifact.
- When a required canonical section is absent or blank, the emitted artifact includes `## Missing Authored Body` naming the canonical heading.
- Near-match headings do not satisfy the contract unless an alias is explicitly documented.
- `system-shaping` continues to preserve the existing `domain-model.md` authored contract while extending the same behavior to the remaining packet artifacts.
- `implementation` and `refactor` must receive the original authored brief text for section extraction; evidence-mixed summaries are not a substitute for the authored source.

## Documentation Sync Rules

- Embedded skill sources and mirrored `.agents` skill files must enumerate the same authored sections as this contract.
- Starter templates must demonstrate the same canonical headings.
- Worked examples must exercise the same artifact-to-heading mapping.
- Guide and roadmap text may summarize the contract, but must not contradict or broaden it.

## Non-Goals

- No new artifact file names in this slice.
- No automatic fuzzy heading matching.
- No changes to publish destinations, risk gates, run identity, or execution posture.