# Canon Stable Profiles

Profiles define the structural requirements, authority, and expected evidence
for governed packets. Canon has exactly nine stable profiles, in this order:

1. `discovery`
2. `requirements`
3. `architecture`
4. `backlog`
5. `change`
6. `refactor`
7. `verification`
8. `pr-review`
9. `incident`

The same registry drives stable parsing, `canon inspect modes`, and machine
capabilities. Identifiers are exact; aliases, case changes, surrounding
whitespace, and unknown values fail closed.

## Change and implementation

`change` governs intent, scope, risks, invariants, acceptance criteria,
authority, and required evidence. It does not execute implementation.
`implementation` is not a stable Canon profile and is not mapped to `change`
or `refactor`. Boundline or a bounded adapter owns execution. Canon can still
read historical records whose persisted mode predates this boundary.

## Verification

`verification` governs evidence requirements. Canon can run deterministic
checks and validate the structure, binding, freshness, terminality, challenge
tier, and lineage of externally supplied semantic evidence. Structural
acceptance does not assert that the external judgment is true.

Canon does not execute a semantic reviewer, invoke Copilot or another model to
manufacture review, read provider credentials, use the network for review, or
turn authored status labels into proof. A verification packet remains blocked
until its required external evidence is supplied through a qualified boundary.
`canon verify --run <RUN_ID>` validates only the persisted deterministic
evidence projection and exits nonzero while required external semantic
evidence is missing.
