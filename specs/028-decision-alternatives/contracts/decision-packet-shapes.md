# Contract: Decision Packet Shapes

## Scope

This contract defines the authored packet families introduced or aligned by the
028 decision-support feature.

## Structural Decision Family

| Mode | Required Decision Shape | Reviewer Outcome |
|------|-------------------------|------------------|
| `system-shaping` | decision summary, decision drivers, options considered, tradeoff analysis, recommended direction, why-not rationale | Reviewer can compare structural patterns before the capability shape is fixed |
| `architecture` | existing ADR plus options shape aligned to the broader feature vocabulary | Reviewer can identify winning and rejected architecture options without losing C4 context |
| `change` | bounded decision record, decision drivers, options considered, tradeoff analysis, recommendation, why-not rationale | Reviewer can see why one bounded change path was chosen over another |

## Framework Evaluation Family

| Mode | Required Evaluation Shape | Reviewer Outcome |
|------|---------------------------|------------------|
| `implementation` | decision summary, candidate options, tradeoff analysis, evidence references, ecosystem-health notes, adoption implications | Reviewer can judge concrete stack choices against execution constraints |
| `migration` | decision summary, candidate options, tradeoff analysis, evidence references, ecosystem-health notes, migration or rollback implications | Reviewer can compare coexistence and migration paths with explicit compatibility cost |

## Honesty Rules

- If the authored input materially closes the decision to one viable option,
  the packet must say so explicitly instead of fabricating a balanced matrix.
- If a required authored decision or evidence section is missing, Canon must
  surface the explicit gap rather than inventing analysis.
- If an evidence-backed comparison claim lacks authored support, the packet
  must preserve the evidence gap instead of presenting the claim as settled
  truth.