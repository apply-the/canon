# Discovery Authoring Contract

## Artifact Mapping

| Artifact | Canonical authored headings | Fallback policy |
|----------|-----------------------------|-----------------|
| `problem-map.md` | `## Problem Domain`, `## Repo Surface`, `## Immediate Tensions`, `## Downstream Handoff` | Emit `## Missing Authored Body` naming the missing canonical heading if a required section is absent or empty |
| `unknowns-and-assumptions.md` | `## Unknowns`, `## Assumptions`, `## Validation Targets`, `## Confidence Levels` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `context-boundary.md` | `## In-Scope Context`, `## Repo Surface`, `## Out-of-Scope Context`, `## Translation Trigger` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `exploration-options.md` | `## Options`, `## Constraints`, `## Recommended Direction`, `## Next-Phase Shape` | Emit marker naming the missing canonical heading if a required section is absent or empty |
| `decision-pressure-points.md` | `## Pressure Points`, `## Blocking Decisions`, `## Open Questions`, `## Recommended Owner` | Emit marker naming the missing canonical heading if a required section is absent or empty |

## Skill and Docs Expectation

- The discovery skill, template, and example must teach exploratory framing without falling back to generic filler.
- Near-match headings are treated as missing unless explicitly listed as compatibility aliases in a later slice.