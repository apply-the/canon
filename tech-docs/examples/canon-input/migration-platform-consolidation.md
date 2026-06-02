# Migration Brief: Platform Consolidation

## Current State
legacy-build-runner and legacy-secrets-agent coordinate CI workloads on a
separate operational stack.

## Target State
the shared platform-runner stack owns CI execution, secrets brokering, and
artifact staging for the bounded build pipeline surface.

## Transition Boundaries
CI job execution, secrets brokering for builds, and artifact upload paths only.
Developer laptops, production deploys, and billing jobs stay out of scope.

## Guaranteed Compatibility
- existing CI job definitions continue to run without YAML rewrites
- current artifact paths remain readable during the transition window

## Temporary Incompatibilities
- build latency dashboards will split between the old and new stacks during
  phased cutover

## Coexistence Rules
- route a bounded canary set of build queues to the shared platform-runner
- keep legacy-build-runner available for immediate fallback during each phase

## Options Matrix
- Option 1 keeps both runners active during a phased queue-by-queue cutover.
- Option 2 performs a one-day hard cut and accepts a narrower rollback window.

## Ordered Steps
1. Mirror queue metadata into the shared platform-runner control plane.
2. Shift one non-critical build queue into the new runner stack.
3. Expand cutover queue by queue once artifact upload and secrets brokering stay stable.
4. Retire the legacy runner only after the bounded canary queues stay healthy for one week.

## Parallelizable Work
- update operator runbooks for the shared platform-runner
- build new latency and secrets-broker dashboards in parallel with the canary

## Cutover Criteria
- canary build success rate matches the legacy runner baseline
- secrets resolution errors stay below the agreed operational threshold
- artifact upload paths remain stable for the bounded CI queues

## Rollback Triggers
- secrets brokering failures exceed the queue-specific error budget
- artifact uploads diverge from the existing path contract
- queue latency regresses beyond the accepted threshold for the canary set

## Fallback Paths
- return the affected queues to legacy-build-runner while keeping mirrored metadata for diagnosis
- disable new secrets brokering and restore the legacy agent path immediately

## Re-Entry Criteria
- the failure mode is reproduced and bounded to a specific runner or broker issue
- the corrected canary path passes one full day of stable queue execution

## Adoption Implications
- keep the consolidation bounded to CI workloads first; production deploy
  orchestration and batch jobs must stay deferred to later packets

## Verification Checks
- compare build success, secrets lookup, and artifact upload metrics across both stacks
- replay a bounded set of historical CI jobs against the shared platform-runner

## Residual Risks
- long-tail custom build plugins may still assume legacy runner timing
- secrets broker caching could hide drift until less-common jobs execute

## Release Readiness
- recommendation-only until the platform owner and security owner both accept
  the bounded fallback posture

## Migration Decisions
- use phased queue cutover instead of a same-day hard switch
- keep legacy-build-runner warm until the bounded CI surface proves stable

## Tradeoff Analysis
- phased cutover costs extra operational overhead but preserves rollback
  credibility and keeps secrets-broker failures bounded

## Decision Evidence
- canary queue telemetry and mirrored metadata already support bounded
  comparison between the old and new runner stacks
- shared platform-runner ownership and secrets-broker support are healthy for
  CI, but downstream deploy tooling lacks equivalent readiness signals

## Recommendation
- proceed with phased queue cutover through the shared platform-runner and defer broader platform consolidation until CI stability is proven

## Why Not The Others
- a same-day hard switch would trade away rollback credibility before the
  bounded CI queues prove stable on the new stack

## Ecosystem Health
- shared platform-runner telemetry, ownership, and secrets-broker support are
  healthy enough for bounded CI adoption, but downstream deploy tooling is not
  yet ready for the same move

## Deferred Decisions
- whether deploy orchestration should move onto the same runner stack
- whether batch-job artifact uploads should share the new broker path

## Approval Notes
- platform owner and security owner approval are both required before the
  first canary queue moves