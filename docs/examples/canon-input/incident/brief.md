# Incident Brief

## Incident Scope
payments-api and checkout flow only.

## Trigger And Current State
elevated 5xx responses after the last deploy.

## Operational Constraints
no autonomous remediation and no schema changes.

## Known Facts
- errors started after the deploy
- rollback remains available

## Working Hypotheses
- retry amplification is exhausting the service

## Evidence Gaps
- database saturation is not yet confirmed

## Impacted Surfaces
- payments-api
- checkout flow

## Propagation Paths
- checkout request path

## Confidence And Unknowns
- medium confidence until saturation evidence is collected

## Immediate Actions
- disable async retries

## Ordered Sequence
- capture blast radius
- disable retries
- reassess error rate

## Stop Conditions
- error rate stabilizes below the alert threshold

## Decision Points
- decide whether rollback is still required

## Approved Actions
- disable retries within the bounded surface

## Deferred Actions
- schema-level changes remain out of scope

## Verification Checks
- confirm 5xx rate drops

## Release Readiness
- keep recommendation-only posture until the owner accepts the packet

## Follow-Up Work
- add a saturation dashboard and post-incident review item