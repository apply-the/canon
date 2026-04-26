# Change Domain Slice Contract

## Strengthened Existing Artifact Surface

| Artifact | Required sections after this slice | Purpose |
|----------|------------------------------------|---------|
| `system-slice.md` | `System Slice`, `Domain Slice`, `Excluded Areas` | Make the bounded change explicit in both technical and domain terms |
| `legacy-invariants.md` | `Legacy Invariants`, `Domain Invariants`, `Forbidden Normalization` | Preserve both implementation and business-rule continuity |
| `change-surface.md` | `Change Surface`, `Ownership`, `Cross-Context Risks` | Show the allowed mutation surface, owners, and boundary stress |
| `decision-record.md` | `Decision Record`, `Boundary Tradeoffs`, `Consequences`, `Unresolved Questions` | Capture why this bounded domain slice is preferable and what risk it carries |

## Unchanged Supporting Artifacts

- `implementation-plan.md` and `validation-strategy.md` remain in the packet and should reference the chosen domain slice where relevant, but they do not become separate domain-model artifacts.

## Skill and Docs Expectation

- The `canon-change` skill, template, and example must teach the new domain-slice and cross-context headings as part of the authored change packet.
- Missing authored sections continue to surface the explicit honesty marker instead of generic bounded-change filler.