# Domain Language Brief: E-Commerce Order Fulfillment Vocabulary

System Surface: The order fulfillment domain for the existing e-commerce platform.
Primary Upstream Mode: discovery
Upstream Sources:
- docs/discovery/R-20260501-FULFILLMENT/01-problem-map.md
- src/orders/domain.rs
- docs/architecture/decisions/bounded-contexts.md
Carried-Forward Decisions:
- Order lifecycle terms remain consistent with the existing payment integration.
- Warehouse and logistics terms are bounded to internal fulfillment, not third-party carriers.
Excluded Upstream Scope: Customer-facing marketing terminology, returns processing language, and payment gateway terms remain out of scope.

## Domain Scope
- Stabilize the shared vocabulary for the order fulfillment bounded context, covering order creation through shipment confirmation.

## Language Maturity
- Stabilizing: core terms exist in code but differ across teams and documentation.

## Upstream Sources
- Discovery packet from fulfillment investigation.
- Existing `Order`, `Shipment`, and `FulfillmentRequest` types in `src/orders/domain.rs`.
- Architecture decision record on bounded contexts.

## Downstream Consumers
- Architecture mode for fulfillment boundary redesign.
- Change mode for fulfillment pipeline refactoring.
- Implementation mode for order status API.

## Glossary Entries
- **Order**: A customer's confirmed intent to purchase one or more items.
- **Line Item**: A single product entry within an order, including quantity and price.
- **Fulfillment Request**: An internal instruction to pick, pack, and ship a set of line items.
- **Shipment**: A physical package dispatched to the customer.
- **Carrier**: The logistics provider responsible for delivering a shipment.
- **Tracking Event**: A status update from the carrier about a shipment's location or state.
- **Allocation**: The reservation of inventory for a specific fulfillment request.

## Source References
- "Order" and "Line Item" originate from the payment integration contract.
- "Fulfillment Request" is defined in `src/orders/fulfillment.rs`.
- "Shipment" is used inconsistently: logistics team says "parcel," engineering says "shipment."

## Open Gaps
- "Backorder" is used informally but has no formal definition.
- "Partial fulfillment" vs. "split shipment" needs clarification.

## Canonical Terms
- Use "Fulfillment Request" (not "fulfillment order" or "pick request").
- Use "Shipment" (not "parcel" or "package").
- Use "Allocation" (not "reservation" or "hold").

## Deprecated Synonyms
- "Pick request" is deprecated; use "Fulfillment Request."
- "Parcel" is deprecated; use "Shipment."
- "Hold" is deprecated; use "Allocation."

## Migration Notes
- Rename `PickRequest` struct to `FulfillmentRequest` in the next refactor cycle.
- Update logistics team documentation to use "Shipment" consistently.

## Conflict Inventory
- "Order" means different things in payment (financial commitment) vs. fulfillment (physical processing instruction).
- "Status" is overloaded: order status, shipment status, and payment status all use the same word.

## Resolution Status
- "Order" conflict: resolved by qualifying as "Payment Order" vs. "Fulfillment Order" at the context boundary.
- "Status" conflict: open; needs bounded-context-specific status enums.

## Escalation Triggers
- Any new term that crosses the fulfillment/payment boundary must be reviewed by both domain leads.
- Terms used in external carrier APIs must be mapped to canonical internal terms.

## Context-Dependent Terms
- "Order" means a financial commitment in the payment context and a processing instruction in the fulfillment context.
- "Complete" means payment captured in the payment context and shipment delivered in the fulfillment context.

## Disambiguation Rules
- Always qualify "order" with its context when used near a boundary: "payment order" or "fulfillment order."
- Use "order lifecycle stage" instead of bare "status" when the context is ambiguous.

## Usage Examples
- Correct: "The fulfillment order transitions to allocated when inventory is reserved."
- Incorrect: "The order is complete" (ambiguous without context qualifier).

## Naming Conventions
- Domain types use PascalCase matching canonical terms: `FulfillmentRequest`, `Shipment`, `Allocation`.
- Event names use past-tense canonical terms: `OrderPlaced`, `ShipmentDispatched`, `AllocationConfirmed`.

## Domain Boundaries
- This language packet covers the fulfillment bounded context only.
- Payment, returns, and customer communication terms are out of scope.

## Enforcement Guidance
- Code review should flag any use of deprecated synonyms in new code.
- Architecture reviews should verify that boundary-crossing terms use the qualified form.

## Code Naming Patterns
- `FulfillmentRequest` (not `PickRequest` or `FulfillOrder`).
- `ShipmentStatus` (not `ParcelStatus` or `DeliveryState`).
- `AllocationResult` (not `ReservationOutcome`).

## API Surface Terms
- `POST /fulfillment-requests` (not `/pick-requests` or `/fulfillment-orders`).
- `GET /shipments/{id}/tracking-events` (not `/parcels/{id}/updates`).

## Alignment Gaps
- `src/orders/fulfillment.rs` still uses `PickRequest` struct name.
- The logistics dashboard uses "parcel" in all UI labels.

## Consumer Modes
- domain-model (to formalize relationships between fulfillment concepts).
- architecture (to redesign fulfillment boundary).
- change (to refactor fulfillment pipeline naming).

## Handoff Expectations
- Downstream modes should use the canonical terms from this packet.
- Any term not in the glossary should be escalated before use.

## Adoption Risks
- Logistics team may resist renaming "parcel" to "shipment" in their tooling.
- Legacy integrations may break if API parameter names are renamed without a migration period.
