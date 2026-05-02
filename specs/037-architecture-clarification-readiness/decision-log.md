# Decision Log: Architecture Clarification, Assumptions, And Readiness Reroute

## D-001: Extend the existing architecture clarity contract instead of creating a new clarification workflow

- **Status**: Accepted
- **Rationale**: Canon already has `inspect clarity` for file-backed modes, so
  the bounded next step is to improve that contract rather than create a second
  interview-style mode.

## D-002: Use `readiness-assessment.md` as the durable home for assumptions and unresolved questions

- **Status**: Accepted
- **Rationale**: Architecture mode already owns a readiness artifact, and this
  slice is about making that readiness output honest and reviewable.

## D-003: Reuse documented mode handoff boundaries for reroute behavior

- **Status**: Accepted
- **Rationale**: The mode guide already distinguishes discovery, requirements,
  system-shaping, and architecture. Reusing those boundaries keeps reroute
  semantics coherent with the rest of the product.

## D-004: Keep missing-authored-body behavior stronger than generated assumptions

- **Status**: Accepted
- **Rationale**: Omitted canonical sections are a stronger signal than any
  generated fallback. The product must remain honest about missing authored
  input.