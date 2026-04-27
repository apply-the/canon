# Contract: Mode Authored-Body Contracts

## Purpose

Define the authored-body contract for the completion slice delivered in feature `020-authoring-specialization-completion`.

## Authoring Scope Note

- `## Summary` remains a renderer-generated section because it is required by the runtime artifact contract in `crates/canon-engine/src/artifacts/contract.rs`.
- The authored contract in this document covers only the non-summary packet body sections that must be supplied explicitly by the author.

## Artifact Coverage

- `review-brief.md`
- `boundary-assessment.md`
- `missing-evidence.md`
- `decision-impact.md`
- `review-disposition.md`
- `invariants-checklist.md`
- `contract-matrix.md`
- `adversarial-review.md`
- `verification-report.md`
- `unresolved-findings.md`
- `incident-frame.md`
- `hypothesis-log.md`
- `blast-radius-map.md`
- `containment-plan.md`
- `incident-decision-record.md`
- `follow-up-verification.md`
- `source-target-map.md`
- `compatibility-matrix.md`
- `sequencing-plan.md`
- `fallback-plan.md`
- `migration-verification-report.md`
- `decision-record.md`

## Review Required Sections

- `review-brief.md`
  - `## Review Target`
  - `## Evidence Basis`
- `boundary-assessment.md`
  - `## Boundary Findings`
  - `## Ownership Notes`
- `missing-evidence.md`
  - `## Missing Evidence`
  - `## Collection Priorities`
- `decision-impact.md`
  - `## Decision Impact`
  - `## Reversibility Concerns`
- `review-disposition.md`
  - `## Final Disposition`
  - `## Accepted Risks`

## Verification Required Sections

- `invariants-checklist.md`
  - `## Claims Under Test`
  - `## Invariant Checks`
- `contract-matrix.md`
  - `## Contract Assumptions`
  - `## Verification Outcome`
- `adversarial-review.md`
  - `## Challenge Findings`
  - `## Contradictions`
- `verification-report.md`
  - `## Verified Claims`
  - `## Rejected Claims`
  - `## Overall Verdict`
- `unresolved-findings.md`
  - `## Open Findings`
  - `## Required Follow-Up`

## Incident Required Sections

- `incident-frame.md`
  - `## Incident Scope`
  - `## Trigger And Current State`
  - `## Operational Constraints`
- `hypothesis-log.md`
  - `## Known Facts`
  - `## Working Hypotheses`
  - `## Evidence Gaps`
- `blast-radius-map.md`
  - `## Impacted Surfaces`
  - `## Propagation Paths`
  - `## Confidence And Unknowns`
- `containment-plan.md`
  - `## Immediate Actions`
  - `## Ordered Sequence`
  - `## Stop Conditions`
- `incident-decision-record.md`
  - `## Decision Points`
  - `## Approved Actions`
  - `## Deferred Actions`
- `follow-up-verification.md`
  - `## Verification Checks`
  - `## Release Readiness`
  - `## Follow-Up Work`

## Migration Required Sections

- `source-target-map.md`
  - `## Current State`
  - `## Target State`
  - `## Transition Boundaries`
- `compatibility-matrix.md`
  - `## Guaranteed Compatibility`
  - `## Temporary Incompatibilities`
  - `## Coexistence Rules`
- `sequencing-plan.md`
  - `## Ordered Steps`
  - `## Parallelizable Work`
  - `## Cutover Criteria`
- `fallback-plan.md`
  - `## Rollback Triggers`
  - `## Fallback Paths`
  - `## Re-Entry Criteria`
- `migration-verification-report.md`
  - `## Verification Checks`
  - `## Residual Risks`
  - `## Release Readiness`
- `decision-record.md`
  - `## Migration Decisions`
  - `## Deferred Decisions`
  - `## Approval Notes`

## Rendering Rules

- When a canonical authored section exists and is non-empty, the renderer preserves that section body verbatim in the corresponding emitted artifact.
- When a required canonical section is absent or blank, the emitted artifact includes `## Missing Authored Body` naming the missing canonical heading.
- Near-match headings do not satisfy the contract unless an alias is explicitly documented and tested.
- Summary sections remain renderer-generated, but the required authored sections above are the source of truth for the packet body.
- Mode-specific result semantics such as review disposition, verification verdicts, and incident/migration recommendation-only posture must remain compatible with the current gate evaluations.

## Documentation Sync Rules

- Embedded skill sources and mirrored `.agents` skill files must enumerate the same authored sections as this contract.
- Starter templates must demonstrate the same canonical headings.
- Worked examples must exercise the same artifact-to-heading mapping.
- Guide, roadmap, changelog, and compatibility references may summarize the contract, but must not contradict the rollout state or broaden scope.

## Non-Goals

- No new artifact file names in this slice.
- No fuzzy heading matching beyond any explicitly documented alias.
- No changes to publish destinations, risk gates, run identity, or execution posture.