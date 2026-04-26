# System-Shaping Domain Modeling Contract

## Additive Artifact Surface

| Artifact | Required sections | Gate impact | Purpose |
|----------|-------------------|-------------|---------|
| `domain-model.md` | `Summary`, `Candidate Bounded Contexts`, `Core And Supporting Domain Hypotheses`, `Ubiquitous Language`, `Domain Invariants`, `Boundary Risks And Open Questions` | `Exploration`, `Architecture` | Make domain boundaries, vocabulary, preserved business rules, and uncertainty first-class in the system-shaping packet |

## Existing Artifact Expectations

- `system-shape.md` keeps structural framing, with `Domain Responsibilities` staying aligned to the new domain-model artifact.
- `architecture-outline.md` remains the downstream bridge into architecture mode and should reference the proposed domain boundaries when they materially shape the next step.
- `capability-map.md`, `delivery-options.md`, and `risk-hotspots.md` remain in the packet and may reference domain contexts, but they do not replace `domain-model.md`.

## Runtime Contract Expectation

- `domain-model.md` must be recorded in the mode artifact contract so inspect/publish surfaces treat it like the rest of the packet.
- The feature must explicitly decide which gates block on a missing or incomplete `domain-model.md`; the initial design expectation is that both `Exploration` and `Architecture` require it.

## Skill and Docs Expectation

- The `canon-system-shaping` skill, template, and example must explicitly teach the authored sections for `domain-model.md`.
- Missing authored sections follow the established honesty pattern: the artifact is emitted with an explicit missing-body marker rather than fabricated domain prose.