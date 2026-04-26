# Change Authoring Contract

## Artifact Mapping

| Artifact | Canonical authored headings | Fallback policy |
|----------|-----------------------------|-----------------|
| `system-slice.md` | `## System Slice`, `## Excluded Areas` | Emit `## Missing Authored Body` naming the missing canonical heading if a required section is absent or empty |
| `legacy-invariants.md` | `## Legacy Invariants`, `## Forbidden Normalization` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `change-surface.md` | `## Change Surface`, `## Ownership` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `implementation-plan.md` | `## Implementation Plan`, `## Sequencing` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `validation-strategy.md` | `## Validation Strategy`, `## Independent Checks` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `decision-record.md` | `## Decision Record`, `## Consequences`, `## Unresolved Questions` | Emit marker naming the missing canonical heading if a required section is absent or empty |

## Skill and Docs Expectation

- The change skill, template, and example must move from the current inline-label format to canonical H2-authored sections.
- Existing metadata headings such as owner, risk level, and zone may remain in the brief, but they are outside authored-body extraction unless a later contract adds an explicit alias.