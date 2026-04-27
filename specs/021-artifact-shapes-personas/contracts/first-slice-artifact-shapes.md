# Contract: First-Slice Artifact Shapes

## Scope

This contract defines the intended packet shape for the first slice of the
feature.

## Mode Mapping

| Mode | Shape | Primary Expectation | Honesty Guard |
|------|-------|---------------------|---------------|
| `requirements` | PRD-style packet | Product-facing framing of problem, users, scope, requirements, success criteria, and open questions | Missing authored sections remain explicit and never become synthesized filler |
| `architecture` | C4 plus ADR-style packet | Architecture decision, option analysis, consequences, and C4 views remain explicit and reusable outside chat | Missing C4 or decision sections remain visibly missing rather than inferred |
| `change` | ADR-style bounded change packet | Context, bounded decision, preserved behavior, and validation boundaries remain explicit | Missing bounded-change sections remain blocked by visible gap markers |

## Contract Rules

- Shape guidance must improve packet readability for the intended artifact
  audience.
- Shape guidance must not change Canon's approval, evidence, or risk posture.
- First-slice shape rules apply only to the modes listed above.
