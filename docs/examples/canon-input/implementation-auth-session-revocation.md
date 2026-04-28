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
## Task Mapping
1. Add bounded auth session repository helpers.
2. Thread the helper through the revocation service without expanding the public API.
3. Record implementation notes for operator review and rollback.

## Bounded Changes
- Auth session repository helper wiring.
- Revocation service internal composition.

## Mutation Bounds
src/auth/session.rs and src/auth/repository.rs only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Executed Changes
- Add the bounded repository helper and thread it through the revocation service without widening the public API.

## Options Matrix
- Option 1 keeps the helper inside the auth session slice.
- Option 2 introduces a shared auth abstraction before the bounded slice proves reusable.

## Recommendation
- Start with the local helper and defer the broader abstraction until a later packet proves the reuse pressure.

## Task Linkage
- Step 1 adds the helper.
- Step 2 rewires the service behind the existing external contract.
- Step 3 records the resulting packet and rollback posture.

## Completion Evidence
- The emitted implementation packet and focused tests confirm the bounded slice is ready for operator review.

## Adoption Implications
- Keep the helper local to auth session revocation until adjacent auth flows demonstrate the same need.

## Remaining Risks
- Repository wiring could still drift into adjacent auth modules if the bounded paths expand during review.

## Ecosystem Health
- The surrounding auth subsystem is stable enough for a local helper, but shared abstraction pressure is still low.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before mutation.

## Independent Checks
- cargo test --test session_contract
- cargo test --test auth_audit_ordering

## Rollback Triggers
Revocation output drifts, audit ordering becomes unstable, or repository wiring expands beyond the declared auth-session slice.

## Rollback Steps
1. Revert the bounded auth-session patch.
2. Redeploy the previous build.
3. Restore the last known-good audit ordering snapshot.

Risk Level: bounded-impact
Zone: yellow