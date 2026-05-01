# System Shaping Brief: Billing

_Authored as the system shaper for reviewers and downstream implementers
evaluating bounded billing structure._

Intent: Build a bounded capability for invoicing, automated payment collection, and Stripe integration.
Constraint: Keep the first release limited to credit card integrations (no ACH or wire transfers yet).

## System Shape
The billing packet should separate subscription lifecycle decisions, invoice obligations, and payment-processor interactions into explicit bounded responsibilities while keeping the first release inside one repository-local delivery slice.

## Boundary Decisions
- Keep entitlement and subscription state inside our product boundary even though payment collection delegates to Stripe.
- Treat invoice generation and retention as an internal billing responsibility rather than as a Stripe concern.
- Keep the first slice inside the existing application boundary rather than forcing an immediate microservice split.

## Domain Responsibilities
- Synchronize customer profiles to Stripe.
- Manage billing state transitions (active, past-due, canceled).
- Create PDF statements monthly based on product usage.
- Securely collect and process payment methods without persisting raw PANs internally.

## Candidate Bounded Contexts
- Subscription Lifecycle: owns plan selection, subscription state, renewals, and cancellation policy.
- Invoice Management: owns invoice generation, invoice delivery, and compliance retention.
- Payment Collection: owns Stripe payment intents, payment-method setup, retries, and settlement signals.

## Core And Supporting Domain Hypotheses
- Subscription Lifecycle is the core domain because it directly determines product entitlement and revenue posture.
- Payment Collection is supporting because it enables monetization but is partially delegated to Stripe.
- Invoice Management is supporting because it satisfies compliance and reporting needs around the core subscription flow.

## Ubiquitous Language
- Subscription: the active commercial agreement that entitles a customer to product access.
- Invoice: the compliance record of charges for a billing period, not the same thing as a Stripe charge.
- Payment State: the state of collection (`pending`, `succeeded`, `failed`, `past-due`) and its downstream entitlement effect.

## Domain Invariants
- Product access cannot be revoked solely because Stripe is temporarily unavailable; entitlement changes require a durable billing-state transition.
- The system must never store raw card data internally; PCI-sensitive payment data stays with Stripe.

## Boundary Risks And Open Questions
- Subscription Lifecycle and Payment Collection may drift if webhook timing and internal job retries disagree on final payment state.
- Finance reporting may need Invoice Management to own terminology that differs from Stripe's object model, which suggests an anti-corruption seam later.

## Structural Options
- Option 1: keep billing as one bounded module inside the existing application for the first release.
- Option 2: split payment collection into a separate service immediately and accept higher operational overhead.
- Option 3: let Stripe receipts stand in for invoice management and defer compliance-heavy retention work.

## Selected Boundaries
- Keep Subscription Lifecycle, Invoice Management, and Payment Collection as explicit bounded contexts inside one application slice for the first release.

## Rationale
- This keeps operational complexity low while still making the seams reviewable and ready for later extraction.

## Why Not The Others
- Splitting payment collection into a separate service immediately would add operational cost before the billing seams and webhook timing are proven.
- Letting Stripe receipts stand in for invoice management would postpone a compliance requirement that the billing domain already knows it must own.

## Capabilities
- Subscription state management.
- Invoice generation and retention.
- Payment method setup and retry orchestration.

## Dependencies
- Stripe APIs for payment-method setup, webhooks, and settlement status.
- Internal product account and entitlement data.
- PDF generation or receipt-rendering support for invoice output.

## Gaps
- The webhook retry strategy across multiple nodes still needs an explicit convergence design.
- The invoice PDF engine versus Stripe-receipt decision remains open.

## Delivery Phases
1. Land subscription lifecycle and payment-collection seams with explicit entitlement invariants.
2. Add invoice generation and retention once lifecycle state transitions are stable.
3. Harden webhook retries, reconciliation, and reporting exports.

## Sequencing Rationale
- Subscription and payment-state correctness must come first because downstream invoices and reporting depend on them.

## Risk per Phase
- Phase 1: Stripe drift could desynchronize entitlement state.
- Phase 2: compliance retention could lag behind invoice generation needs.
- Phase 3: webhook retries and reconciliation could still double-charge or miss settlement signals.

## Hotspots
- Stripe webhook ordering versus our internal entitlement transitions.
- Invoice terminology drift between finance expectations and Stripe objects.
- Concurrent retry workers issuing duplicate collection actions.

## Mitigation Status
- PCI scope mitigation is explicit and accepted because raw card data never enters our database.
- Operational isolation from Stripe outages is partially mitigated by asynchronous processing, but retry design is still incomplete.

## Unresolved Risks
- Finance reporting may still force a stronger anti-corruption layer than the first release includes.
- Lost or duplicated webhook events could still leave subscriptions in the wrong state until reconciliation hardening lands.