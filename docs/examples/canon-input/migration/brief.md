# Migration Brief

## Current State
auth-v1 serves login and token refresh traffic for the monolith.

## Target State
auth-v2 serves the same bounded traffic surface using the centralized identity service.

## Transition Boundaries
login endpoint and token signing only. Background syncing stays in the monolith.

## Guaranteed Compatibility
- existing user tokens continue to validate
- auth-v1 continues to handle signature verification during the transition window

## Temporary Incompatibilities
- admin reporting metrics for auth success rates will diverge between systems

## Coexistence Rules
- write session state to both v1 and v2 during the test window
- reads default to v1 first, falling back to v2 if missing

## Ordered Steps
- Phase 1: Deploy auth-v2 with shadow writes enabled for new tokens.
- Phase 2: Verify signature compatibility using sampled test requests.
- Phase 3: Roll over load balancer traffic.
- Phase 4: Decommission auth-v1 token generation.

## Parallelizable Work
- update the internal ID-provider documentation
- create telemetry dashboards for auth-v2 error rates

## Cutover Criteria
- auth-v2 signatures are validated 100% correctly by endpoints during the shadow phase
- p99 latency does not increase beyond 100ms

## Rollback Triggers
- signature verification failures exceed the agreed error budget during shadow traffic
- login success rate regresses for older mobile clients after load balancer cutover

## Fallback Paths
- return login traffic to auth-v1 while keeping auth-v2 shadow writes enabled for diagnosis
- stop new token generation on auth-v2 and resume auth-v1 signing immediately

## Re-Entry Criteria
- auth-v2 signature validation passes on sampled traffic with no mobile-client regressions
- the on-call owner confirms the fallback cause is understood and bounded

## Verification Checks
- manual QA login testing across web and mobile
- compare error metrics on `POST /login`

## Residual Risks
- rare edge cases in older mobile app integrations might fail the v2 token parsing

## Release Readiness
- explicit fallback credibility is not yet established for the database rollback

## Migration Decisions
- retain dual-write logic for at least 72 hours
- do not migrate legacy admin API routes

## Deferred Decisions
- defer deleting the auth-v1 DB tables to next quarter

## Approval Notes
- wait for Security Team sign-off on the new token signing key rotation process before starting Phase 1