# Quickstart: Project Memory And Delivery Control Contracts

## Goal

Review and validate the Canon-owned contract bundle before producer-side code or
docs updates are considered complete.

## Scenario 1: Stable contract discovery

1. Open `tech-docs/integration/project-memory-promotion-contract.md`.
2. Confirm the document names Canon as the owner, identifies the current major
   contract line, and points to the feature-local contract set for detailed
   examples.
3. Verify the stable contract path describes the producer-neutral managed-block
   marker, required V1 lineage, and compatibility policy.

## Scenario 2: Feature-local contract bundle review

1. Open each contract file under
   `specs/050-project-memory-control/contracts/`.
2. Verify that project-memory promotion, governed stage refs, promotion events,
   and evidence refs are all defined without consumer-side orchestration logic.
3. Confirm the field names and semantics align with the stable contract path,
   and treat the feature-local contracts as elaborations rather than a second
   source of authority.

## Scenario 3: Promotion policy walkthrough

1. Review the documented target mapping for `tech-docs/project/` and `tech-docs/evidence/`.
2. Verify that stable managed-block updates are separated from pending,
   proposal, index-only, blocked, conflicting, and evidence-only outcomes.
3. Confirm mixed-producer evidence blocks stay attributable to their producer.
4. Confirm only Canon-owned shapes define promotion-state, approval, readiness,
   and target-routing semantics.

## Scenario 5: Independent consumer review

1. Ask a Boundline maintainer to review only the stable Canon contract path and
   the linked feature-local contracts.
2. Confirm they can identify the owner, managed-block rules, required lineage,
   and stop or proceed boundaries without reading Canon implementation code.
3. Record the outcome in `specs/050-project-memory-control/decision-log.md`.

## Scenario 4: Compatibility review

1. Review the compatibility section in the contract bundle.
2. Confirm additive V1 changes are allowed, while removing or renaming required
   fields is explicitly breaking.
3. Confirm the previous minor published contract revision is supported for one
   full minor release cycle.

## Validation Commands After Implementation

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo nextest run`