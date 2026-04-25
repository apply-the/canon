# Contract: Migration Artifact Packet

## Summary

The `migration` mode emits a governed operational packet for compatibility-aware
transition work. The packet must preserve explicit sequencing, fallback, and
compatibility posture while remaining readable outside the runtime.

## Artifact Names

- `source-target-map.md`
- `compatibility-matrix.md`
- `sequencing-plan.md`
- `fallback-plan.md`
- `migration-verification-report.md`
- `decision-record.md`

## Required Sections

### `source-target-map.md`

- `## Summary`
- `## Current State`
- `## Target State`
- `## Transition Boundaries`

### `compatibility-matrix.md`

- `## Summary`
- `## Guaranteed Compatibility`
- `## Temporary Incompatibilities`
- `## Coexistence Rules`

### `sequencing-plan.md`

- `## Summary`
- `## Ordered Steps`
- `## Parallelizable Work`
- `## Cutover Criteria`

### `fallback-plan.md`

- `## Summary`
- `## Rollback Triggers`
- `## Fallback Paths`
- `## Re-Entry Criteria`

### `migration-verification-report.md`

- `## Summary`
- `## Verification Checks`
- `## Residual Risks`
- `## Release Readiness`

### `decision-record.md`

- `## Summary`
- `## Migration Decisions`
- `## Deferred Decisions`
- `## Approval Notes`

## Validation Gates

- `Exploration`
- `Architecture`
- `MigrationSafety`
- `Risk`
- `ReleaseReadiness`

## Content Expectations

The packet must:

- identify the current and target states plus the bounded transition surface
- make compatibility guarantees and temporary incompatibilities explicit
- show sequencing and fallback as operational plans rather than generic prose
- surface when rollback, coexistence, or verification credibility is missing
- remain readable after publish under `docs/migrations/<RUN_ID>/`

## Compatibility Expectations

- The packet must stay recommendation-only in the first full-depth slice.
- Compatibility claims must be distinguishable from assumptions or temporary
  exceptions.
- The packet summary and verification artifacts must preserve gate outcomes
  and evidence posture for status and inspect surfaces.

## Non-Goals For This Contract

- automatic cutover orchestration
- tool-specific deployment workflow export
- hiding compatibility debt behind a single go/no-go statement