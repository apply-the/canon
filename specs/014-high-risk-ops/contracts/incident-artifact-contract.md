# Contract: Incident Artifact Packet

## Summary

The `incident` mode emits a governed operational packet for bounded failure
investigation and containment. The packet must remain readable as standalone
markdown while preserving explicit containment, evidence, and readiness
posture.

## Artifact Names

- `incident-frame.md`
- `hypothesis-log.md`
- `blast-radius-map.md`
- `containment-plan.md`
- `incident-decision-record.md`
- `follow-up-verification.md`

## Required Sections

### `incident-frame.md`

- `## Summary`
- `## Incident Scope`
- `## Trigger And Current State`
- `## Operational Constraints`

### `hypothesis-log.md`

- `## Summary`
- `## Known Facts`
- `## Working Hypotheses`
- `## Evidence Gaps`

### `blast-radius-map.md`

- `## Summary`
- `## Impacted Surfaces`
- `## Propagation Paths`
- `## Confidence And Unknowns`

### `containment-plan.md`

- `## Summary`
- `## Immediate Actions`
- `## Ordered Sequence`
- `## Stop Conditions`

### `incident-decision-record.md`

- `## Summary`
- `## Decision Points`
- `## Approved Actions`
- `## Deferred Actions`

### `follow-up-verification.md`

- `## Summary`
- `## Verification Checks`
- `## Release Readiness`
- `## Follow-Up Work`

## Validation Gates

- `Risk`
- `IncidentContainment`
- `Architecture`
- `ReleaseReadiness`

## Content Expectations

The packet must:

- identify the bounded incident surface and the current operational state
- separate known facts from working hypotheses and evidence gaps
- make blast radius and containment logic visible without hidden runtime state
- surface when evidence is insufficient to justify progression
- remain readable after publish under `docs/incidents/<RUN_ID>/`

## Compatibility Expectations

- The packet must stay recommendation-only in the first full-depth slice.
- Blocked or downgraded runs must still publish honest artifact content rather
  than polished but unsupported readiness language.
- The packet summary and follow-up verification artifacts must preserve gate
  outcomes and evidence posture for status and inspect surfaces.

## Non-Goals For This Contract

- live operational automation
- hidden escalation rules outside the packet artifacts
- replacing broader Canon approval semantics with incident-specific shortcuts