# Contract: Persona Boundaries

## Scope

This contract defines what persona guidance may and may not do in the 031
remaining-rollout slice.

## Persona Mapping

| Mode | Persona | Intended Audience | Persona May Influence | Persona Must Not Influence |
|------|---------|-------------------|-----------------------|----------------------------|
| `implementation` | delivery lead | maintainers translating bounded plans into executable work | task framing, delivery sequencing emphasis, implementation-note readability, and bounded decision explanation | required sections, missing-gap handling, or risk and evidence truthfulness |
| `refactor` | preservation-focused maintainer | maintainers reviewing safe structural change | invariant emphasis, mechanism-versus-behavior framing, and rationale readability | feature authority, preserved-behavior honesty, or missing authored sections |
| `verification` | adversarial verifier | reviewers judging whether claims are actually supported | claim framing, evidence prioritization, independence language, and unresolved-finding readability | closure authority, unsupported evidence, or blocked-state honesty |

## Boundary Rules

- Persona guidance is advisory and presentational, not authoritative.
- Persona guidance cannot grant approval, close evidence gaps, or normalize a
  missing authored section into a complete packet.
- Any conflict between persona framing and Canon contract semantics resolves in
  favor of Canon contract semantics.