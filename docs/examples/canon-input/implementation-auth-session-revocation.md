# Implementation Brief: Bounded Session Revocation Hardening

Feature Slice: Auth session revocation repository wiring inside the existing login subsystem.
Primary Upstream Mode: change
Upstream Sources:
- docs/changes/R-20260422-AUTHREVOC/change-surface.md
- docs/changes/R-20260422-AUTHREVOC/legacy-invariants.md
- docs/changes/R-20260422-AUTHREVOC/implementation-plan.md
Carried-Forward Decisions:
- Revocation output formatting stays stable for downstream audit consumers.
- The allowed mutation surface remains inside the auth session modules.
- Contract coverage must pass before and after mutation.
Excluded Upstream Scope: Login UI flow, token issuance, deployment manifests, and analytics consumers remain out of scope for this packet.
Task Mapping: 1. Add bounded auth session repository helpers. 2. Thread the helper through the revocation service without expanding the public API. 3. Record implementation notes for operator review and rollback.
Mutation Bounds: src/auth/session.rs and src/auth/repository.rs only.
Allowed Paths:
- src/auth/session.rs
- src/auth/repository.rs
Safety-Net Evidence: Contract coverage protects revocation formatting and audit ordering before mutation.
Independent Checks:
- cargo test --test session_contract
- cargo test --test auth_audit_ordering
Rollback Triggers: Revocation output drifts, audit ordering becomes unstable, or repository wiring expands beyond the declared auth-session slice.
Rollback Steps: Revert the bounded auth-session patch, redeploy the previous build, and restore the last known-good audit ordering snapshot.
Risk Level: bounded-impact
Zone: yellow