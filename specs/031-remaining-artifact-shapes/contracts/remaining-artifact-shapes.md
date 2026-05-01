# Contract: Remaining Artifact Shapes

## Scope

This contract defines the intended packet shape for the 031 remaining-rollout
slice.

## Mode Mapping

| Mode | Shape | Primary Expectation | Honesty Guard |
|------|-------|---------------------|---------------|
| `implementation` | task-mapped implementation plan plus bounded framework-evaluation dossier | delivery framing makes tasks, validation intent, implementation notes, and any real stack choice readable without chat context | missing authored implementation sections remain explicit and no fake option set appears when the decision is already closed |
| `refactor` | preserved-behavior matrix plus structural-rationale record | invariants, preserved behavior, mechanism changes, and structural rationale remain explicit for maintainers reviewing safe change | missing refactor sections remain visibly missing instead of being inferred from maintenance-language prose |
| `verification` | claims-evidence-independence matrix | claim status, supporting evidence, independence posture, and unresolved findings remain explicit and reusable outside chat | missing verification sections or unsupported evidence remain visible instead of being normalized into closure |

## Contract Rules

- Shape guidance must improve packet readability for the intended artifact
  audience.
- Shape guidance must not change Canon's approval, evidence, blocked-state, or
  risk posture.
- Remaining shape rules apply only to `implementation`, `refactor`, and
  `verification` in this slice.