# Change Brief

## System Slice
Describe the bounded subsystem or module to change.

## Domain Slice
Describe the business or ownership boundary inside the system slice.

## Excluded Areas
- Explicit exclusion 1
- Explicit exclusion 2

## Intended Change
Describe the intended modification.

## Legacy Invariants
- Behavior that must remain true 1
- Behavior that must remain true 2

## Domain Invariants
- Domain rule 1 that the change must preserve
- Domain rule 2 that the change must preserve

## Forbidden Normalization
- Shortcut or simplification that this change must not perform

## Change Surface
- Files, modules, APIs, and boundaries allowed to change

## Ownership
- Primary owner and any explicit reviewers

## Cross-Context Risks
- Boundary crossing or seam risk 1
- Boundary crossing or seam risk 2

## Implementation Plan
Describe the high-level change approach.

## Sequencing
1. Ordered step 1
2. Ordered step 2

## Validation Strategy
- Test or check 1
- Test or check 2

## Independent Checks
- Review or verification that should happen outside the implementing path

## Decision Record
Explain why this change is preferable.

## Boundary Tradeoffs
- Tradeoff 1 created by keeping the change bounded
- Tradeoff 2 created by keeping the change bounded

## Consequences
- Consequence 1
- Consequence 2

## Unresolved Questions
- Question 1
- Question 2

Owner: staff-engineer
Risk Level: bounded-impact
Zone: yellow