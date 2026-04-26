# System Shaping Brief: Billing

Intent: Build a bounded capability for invoicing, automated payment collection, and Stripe integration.
Constraint: Keep the first release limited to credit card integrations (no ACH or wire transfers yet).

## Goal
The new billing subsystem must handle recurring subscription payments, create PDF invoices for compliance, and synchronize payment states back to the core user product.

## Users or Stakeholders
- Internal Finance team (needs reporting dashboards).
- Active customers (needs invoice PDFs and subscription management UI).

## Domain Responsibilities
- Synchronize customer profiles to Stripe.
- Manage billing state transitions (active, past-due, canceled).
- Create PDF statements monthly based on product usage.
- Securely collect and process payment methods without persisting raw PANs internally.

## Constraints
- All PCI data stays with the payment processor (Stripe). No raw cards stored in our database.
- Must run as an asynchronous process; if Stripe goes down, the core application registration must remain active.

## Risks
- Data anomalies between our database and the payment provider (Stripe drift).
- Double billing charges due to concurrent job runners.
- Accidental account deactivation off cycle if webhooks are lost.

## Open Questions
- Do we integrate a separate PDF generation engine or have Stripe handle receipts entirely?
- How to handle retries for webhook events across multiple nodes gracefully?
- Should the billing service be its own microservice bounded externally with gRPC or a monolith library?

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