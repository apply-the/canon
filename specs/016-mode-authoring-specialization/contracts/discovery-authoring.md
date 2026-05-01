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

- The discovery skill, template, example, and mode guidance must teach an
	Opportunity Solution Tree seed plus Jobs-To-Be-Done flavored packet without
	falling back to generic filler.
- Discovery should read as work authored by an exploratory research lead for
	downstream product and engineering decision makers.
- `## Problem Domain` and `## Immediate Tensions` anchor the desired outcome
	and blocked job; `## Options`, `## Recommended Direction`, and
	`## Next-Phase Shape` express opportunity or solution branches; and
	`## Validation Targets`, `## Confidence Levels`, and `## Assumptions`
	express the assumption-test spine of the packet.
- Near-match headings are treated as missing unless explicitly listed as compatibility aliases in a later slice.