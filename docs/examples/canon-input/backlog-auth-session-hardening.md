# Backlog Brief: Auth Session Hardening

## Delivery Intent
Turn the approved auth-session hardening decision into bounded epics and delivery slices for staged rollout.

## Desired Granularity
epic-plus-slice

## Planning Horizon
next two delivery increments

## Source References
- docs/architecture/decisions/R-20260422-AUTHREVOC/decision-summary.md
- docs/changes/R-20260422-AUTHREVOC/change-surface.md
- docs/changes/R-20260422-AUTHREVOC/implementation-plan.md

## Priorities
- Ship the rollback-safe revocation foundation before expanding audit-side cleanup.
- Keep cross-module dependency blockers explicit in the packet.

## Constraints
- The packet must stay above task level.
- Revocation output formatting must remain traceable to the approved upstream packet.
- Shared auth repository wiring is the only allowed cross-module dependency surface.

## Out of Scope
- Login UI redesign
- Admin analytics cleanup
- Sprint or ticket-level task breakdown