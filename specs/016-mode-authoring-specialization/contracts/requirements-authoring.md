# Requirements Authoring Contract

## Artifact Mapping

| Artifact | Canonical authored headings | Fallback policy |
|----------|-----------------------------|-----------------|
| `problem-statement.md` | `## Problem`, `## Outcome` | Emit `## Missing Authored Body` naming the missing canonical heading if either required section is absent or empty |
| `constraints.md` | `## Constraints`, `## Non-Negotiables` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `options.md` | `## Options`, `## Recommended Path` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `tradeoffs.md` | `## Tradeoffs`, `## Consequences` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `scope-cuts.md` | `## Scope Cuts`, `## Deferred Work` | `## Out of Scope` may be accepted as a compatibility alias for `## Scope Cuts`; otherwise emit marker naming the missing canonical heading |
| `decision-checklist.md` | `## Decision Checklist`, `## Open Questions` | Emit marker naming the missing canonical heading if a required section is absent or empty |

## Skill and Docs Expectation

- The requirements skill, template, and example must enumerate the same canonical headings.
- The packet remains PRD-shaped, but the authored-body contract is expressed in Canon's emitted artifact vocabulary.