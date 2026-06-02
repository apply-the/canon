# Domain Model Brief: E-Commerce Order Fulfillment Concept Model

System Surface: The order fulfillment domain for the existing e-commerce platform.
Primary Upstream Mode: domain-language
Upstream Sources:
- tech-docs/domain/language/2026-05-01-order-fulfillment/02-domain-glossary.md
- tech-docs/domain/language/2026-05-01-order-fulfillment/03-preferred-language.md
- src/orders/domain.rs
Carried-Forward Decisions:
- Canonical terms from the domain-language packet are adopted without modification.
- Fulfillment boundary remains internal; third-party carrier integration is a separate bounded context.
Excluded Upstream Scope: Payment domain model, returns processing model, and customer communication model remain out of scope.

## Domain Scope
- Formalize the concept model for the order fulfillment bounded context, covering order creation through shipment delivery confirmation.

## Model Maturity
- Evolving: core concepts are identified from the language packet but relationships and invariants need formal specification.

## Upstream Sources
- Domain-language packet for order fulfillment vocabulary.
- Existing `Order`, `FulfillmentRequest`, `Shipment`, and `Allocation` types in `src/orders/domain.rs`.

## Downstream Consumers
- Architecture mode for fulfillment boundary redesign.
- Backlog mode for fulfillment epic decomposition.
- Implementation mode for order status API.

## Concepts
- **Order**: A customer's confirmed intent to purchase one or more items. Owned by the Order Management context.
- **Line Item**: A single product entry within an order. Owned by the Order Management context.
- **Fulfillment Request**: An internal instruction to pick, pack, and ship a set of line items. Owned by the Fulfillment context.
- **Shipment**: A physical package dispatched to the customer. Owned by the Fulfillment context.
- **Allocation**: The reservation of inventory for a specific fulfillment request. Owned by the Inventory context.
- **Tracking Event**: A status update from the carrier about a shipment. Owned by the Carrier Integration context.
- **Carrier**: The logistics provider responsible for delivering a shipment. Owned by the Carrier Integration context.

## Ownership Boundaries
- Order Management owns Order and Line Item lifecycle.
- Fulfillment owns Fulfillment Request and Shipment lifecycle.
- Inventory owns Allocation lifecycle.
- Carrier Integration owns Tracking Event and Carrier reference data.

## Open Gaps
- "Backorder" concept is referenced but not yet formalized.
- "Partial Fulfillment" needs a concept definition and lifecycle model.

## Relationships
- Order --contains--> Line Item (1:N).
- Order --triggers--> Fulfillment Request (1:N, one per fulfillment batch).
- Fulfillment Request --produces--> Shipment (1:1).
- Fulfillment Request --requires--> Allocation (1:N, one per line item).
- Shipment --emits--> Tracking Event (1:N).
- Shipment --uses--> Carrier (N:1).

## Cardinality Rules
- An Order must contain at least one Line Item.
- A Fulfillment Request references exactly one Order but may cover a subset of its Line Items.
- An Allocation references exactly one Line Item and one inventory location.

## Boundary Crossings
- Order to Fulfillment Request crosses the Order Management / Fulfillment boundary.
- Allocation crosses the Fulfillment / Inventory boundary.
- Tracking Event crosses the Carrier Integration / Fulfillment boundary.

## Bounded Contexts
- **Order Management**: Owns order creation, modification, and cancellation.
- **Fulfillment**: Owns pick, pack, ship workflow.
- **Inventory**: Owns stock levels, allocations, and replenishment signals.
- **Carrier Integration**: Owns carrier selection, label generation, and tracking ingestion.

## Context Relationships
- Order Management --> Fulfillment: upstream/downstream (Order Management publishes, Fulfillment subscribes).
- Fulfillment --> Inventory: partnership (Fulfillment requests allocation, Inventory confirms).
- Carrier Integration --> Fulfillment: conformist (Fulfillment adapts to carrier API contracts).

## Integration Seams
- Order Management to Fulfillment: `OrderPlaced` domain event.
- Fulfillment to Inventory: `AllocationRequested` / `AllocationConfirmed` commands.
- Carrier Integration to Fulfillment: `TrackingEventReceived` domain event.

## Entity Lifecycles
- **Order**: Created -> Confirmed -> Fulfilling -> Shipped -> Delivered -> Closed.
- **Fulfillment Request**: Pending -> Allocated -> Picking -> Packed -> Shipped.
- **Allocation**: Requested -> Confirmed -> Released.

## State Transitions
- Order: Created -> Confirmed (payment captured), Confirmed -> Fulfilling (fulfillment request created).
- Fulfillment Request: Pending -> Allocated (all allocations confirmed), Allocated -> Picking (warehouse picks).
- Allocation: Requested -> Confirmed (inventory reserved), Confirmed -> Released (shipment dispatched or cancelled).

## Invariant Guards
- An Order cannot transition to Fulfilling unless at least one Fulfillment Request exists.
- A Fulfillment Request cannot transition to Picking unless all Allocations are Confirmed.
- An Allocation cannot be Released unless the Fulfillment Request is either Shipped or Cancelled.

## Invariants
- An Order's total must equal the sum of its Line Item prices.
- A Fulfillment Request must reference only Line Items from a single Order.
- Allocated quantity for a Line Item must not exceed the ordered quantity.
- A Shipment must have exactly one Carrier assigned before dispatch.

## Enforcement Points
- Order total invariant: enforced at Order creation and Line Item modification.
- Fulfillment Request scope invariant: enforced at Fulfillment Request creation.
- Allocation quantity invariant: enforced at Allocation creation.
- Carrier assignment invariant: enforced at Shipment dispatch transition.

## Violation Consequences
- Order total mismatch: reject the mutation and log a domain violation event.
- Cross-order Fulfillment Request: reject creation with a validation error.
- Over-allocation: reject the allocation request and notify the fulfillment operator.
- Missing carrier: block shipment dispatch and raise an operational alert.

## Business Policies
- Orders above a configurable threshold require manual fulfillment approval.
- Allocations expire after 24 hours if not confirmed by the fulfillment process.
- Carriers must be re-validated weekly against the approved carrier list.

## Constraint Rules
- Maximum 50 Line Items per Order.
- Maximum 3 Fulfillment Requests per Order (to limit split shipments).
- Allocation expiry window: 24 hours (configurable per warehouse).

## Exception Handling
- Expired allocations are automatically released and the fulfillment request transitions to Pending.
- Manual fulfillment approval can be bypassed by a designated operations lead with audit logging.

## Impact Rules
- Adding a new payment method does not affect the fulfillment model (boundary isolation).
- Adding a new carrier requires extending the Carrier Integration context and updating the carrier validation policy.
- Changing the allocation expiry window affects Inventory and Fulfillment contexts.

## Affected Concepts
- New carrier: Carrier, Shipment, Tracking Event.
- Allocation expiry change: Allocation, Fulfillment Request.
- New payment method: none in this bounded context.

## Downstream Effects
- New carrier: carrier validation policy must be updated; fulfillment operators must be trained.
- Allocation expiry change: warehouse SLAs must be reviewed; monitoring thresholds must be adjusted.

## Code Mapping
- `Order` -> `src/orders/domain.rs::Order`
- `LineItem` -> `src/orders/domain.rs::LineItem`
- `FulfillmentRequest` -> `src/orders/fulfillment.rs::FulfillmentRequest`
- `Shipment` -> `src/shipping/domain.rs::Shipment`
- `Allocation` -> `src/inventory/allocation.rs::Allocation`

## Data Store Mapping
- `Order` -> `orders` table, `order_id` primary key.
- `LineItem` -> `line_items` table, foreign key to `orders`.
- `FulfillmentRequest` -> `fulfillment_requests` table, foreign key to `orders`.
- `Allocation` -> `allocations` table, foreign keys to `fulfillment_requests` and `inventory_locations`.

## Alignment Gaps
- `Shipment` struct is in `src/shipping/` but the fulfillment context owns it conceptually.
- `Allocation` uses `reservation_id` column name instead of `allocation_id`.

## Model Gaps
- "Backorder" concept needs lifecycle definition.
- "Partial Fulfillment" policy needs formalization.
- Return-to-stock flow after cancellation is not yet modeled.

## Risk Signals
- Allocation expiry logic is implemented in two places (application and database trigger), creating a consistency risk.
- Carrier validation policy is hardcoded; it should be configuration-driven.

## Recommended Follow-Ups
- Formalize "Backorder" and "Partial Fulfillment" concepts in a follow-up domain-model run.
- Architecture mode run to redesign the fulfillment/inventory boundary.
- Change mode run to unify the allocation expiry implementation.

## Consumer Modes
- architecture (to redesign fulfillment boundary based on this model).
- backlog (to decompose fulfillment improvements into delivery slices).
- change (to refactor allocation and naming alignment gaps).

## Handoff Expectations
- Downstream modes should reference this model's concept catalog and invariants.
- Any new concept should be added through a follow-up domain-model run.

## Adoption Risks
- Teams may resist renaming `reservation_id` to `allocation_id` in the database.
- Carrier Integration context ownership may be contested by the logistics team.
