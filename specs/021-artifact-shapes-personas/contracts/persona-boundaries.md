# Contract: Persona Boundaries

## Scope

This contract defines what persona guidance may and may not do in the first
slice.

## First-Slice Persona Mapping

| Mode | Persona | Intended Audience | Persona May Influence | Persona Must Not Influence |
|------|---------|-------------------|-----------------------|----------------------------|
| `requirements` | Product lead | Stakeholders reviewing bounded product intent | Packet voice, prioritization framing, product-facing clarity | Required sections, missing-gap handling, or evidence truthfulness |
| `architecture` | Architect | Reviewers evaluating boundaries and tradeoffs | Structural rationale, option framing, and architecture readability | Approval posture, missing authored sections, or risk semantics |
| `change` | Change owner | Maintainers reviewing a bounded modification plan | Preservation emphasis, sequencing clarity, and bounded ownership language | Allowed change surface, invariant honesty, or validation requirements |

## Boundary Rules

- Persona guidance is advisory and presentational, not authoritative.
- Persona guidance cannot grant approval, close evidence gaps, or normalize a
  missing authored section into a complete packet.
- Any conflict between persona framing and Canon contract semantics resolves in
  favor of Canon contract semantics.
