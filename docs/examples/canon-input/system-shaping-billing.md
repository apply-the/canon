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