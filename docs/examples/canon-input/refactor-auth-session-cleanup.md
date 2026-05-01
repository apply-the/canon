# Refactor Brief: Auth Session Composition Cleanup

Use this brief to author a preserved-behavior matrix plus
structural-rationale packet that proves no new feature semantics are being
introduced.

Feature Slice: Auth session boundary and repository composition inside the existing login subsystem.
Primary Upstream Mode: implementation
Upstream Sources:
- docs/implementation/R-20260422-AUTHREVOC/task-mapping.md
- docs/implementation/R-20260422-AUTHREVOC/mutation-bounds.md
- docs/implementation/R-20260422-AUTHREVOC/rollback-notes.md
Carried-Forward Invariants:
- Session revocation formatting remains externally unchanged.
- Audit ordering remains externally unchanged.
- No new feature behavior is introduced in the auth session surface.
Excluded Upstream Scope: Public auth API, login UI, deployment wiring, and operator dashboards remain outside this structural cleanup.
## Preserved Behavior
Session revocation formatting and audit ordering remain externally unchanged.

## Approved Exceptions
None.

## Refactor Scope
Auth session boundary and repository composition only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Structural Rationale
Isolate persistence concerns and internal composition without changing externally meaningful behavior.

## Untouched Surface
Public auth API, tests/session.md, deployment wiring, and analytics consumers stay unchanged.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before structural cleanup.

## Regression Findings
No regression findings are accepted in this bounded packet.

## Contract Drift
No public contract drift is allowed.

## Reviewer Notes
Reviewer confirmation is required before any drift or feature semantics are accepted.

## Feature Audit
No new feature behavior is introduced in this refactor packet.

## Decision
Preserve behavior and stop immediately if the surface expands or the packet starts to add feature semantics.

Risk Level: bounded-impact
Zone: yellow