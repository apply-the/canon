# Security Assessment Brief: Webhook Platform Integrity Review

System Surface: The inbound webhook verification path for the existing event platform.
Primary Upstream Mode: change
Upstream Sources:
- docs/changes/R-20260428-WEBHOOKS/change-surface.md
- src/webhooks/verifier.rs
- infra/proxy/webhook-gateway.yaml
Carried-Forward Decisions:
- Webhook verification remains bounded to the existing gateway and verifier pair.
- Secret storage remains in the managed store for this slice.
Excluded Upstream Scope: Downstream analytics processing, customer retry tooling, and unrelated API endpoints remain out of scope.

## Assessment Scope
- Assess request-integrity risk for inbound webhook verification and replay handling.

## In-Scope Assets
- Edge webhook gateway.
- Signature verification service.
- Managed signing secret store.

## Trust Boundaries
- Internet to edge gateway.
- Edge gateway to internal verification service.
- Verification service to managed secret store.

## Out Of Scope
- Customer-side sender hardening.
- Analytics fan-out after verification.

## Threat Inventory
- Forged webhook payloads.
- Replay attempts using stale but previously valid signatures.
- Proxy misconfiguration that drops or rewrites signature headers.

## Attacker Goals
- Inject unauthorized privileged events.
- Reuse captured payloads to trigger repeated processing.

## Boundary Threats
- Header preservation can fail at the proxy boundary before the verifier sees the request.
- Replay attempts cross the internet-to-edge boundary before internal timing checks can reject them.

## Risk Findings
- Replay-window enforcement is incomplete for privileged handlers.
- Proxy header preservation is assumed but not yet independently verified.

## Likelihood And Impact
- Replay exploitation likelihood is moderate with high impact for privileged actions.
- Proxy-header stripping likelihood is low to moderate with high impact because verification silently weakens.

## Proposed Owners
- Platform security owns replay-window recommendations.
- Infrastructure owns proxy-header preservation verification.

## Recommended Controls
- Enforce replay-window validation on all privileged webhook handlers.
- Add an explicit proxy-header preservation check to deployment validation.

## Tradeoffs
- Tighter replay windows increase operational sensitivity to clock skew.
- Extra proxy validation slightly slows deployment review.

## Sequencing Notes
1. Add replay-window telemetry and rejection counters.
2. Verify header preservation in staging.
3. Enforce stale-signature rejection for privileged handlers.

## Assumptions
- Signing secrets remain exclusively in the managed secret store.
- Gateway clock skew stays within the current operational tolerance.

## Evidence Gaps
- No packet capture currently proves every proxy hop preserves signature headers.
- No bounded replay simulation has been recorded after the latest verifier refactor.

## Unobservable Surfaces
- Third-party sender retry timing remains only partially observable.

## Applicable Standards
- OWASP ASVS request integrity guidance applies to the webhook entry point.
- Internal secure-event handling standards apply to privileged event types.

## Control Families
- Request validation.
- Secret handling.
- Secure configuration management.

## Scope Limits
- This packet informs control prioritization and does not certify compliance or replace a live penetration test.

## Source Inputs
- src/webhooks/verifier.rs
- infra/proxy/webhook-gateway.yaml
- docs/changes/R-20260428-WEBHOOKS/change-surface.md

## Independent Checks
- cargo test --test security_assessment_authoring_renderer
- cargo test --test security_assessment_run

## Deferred Verification
- Run a bounded replay simulation after replay-window enforcement lands.