# Contract: Follow-On Artifact Shapes

## Scope

This contract defines the intended packet shape for the 030 follow-on slice.

## Mode Mapping

| Mode | Shape | Primary Expectation | Honesty Guard |
|------|-------|---------------------|---------------|
| `discovery` | Opportunity Solution Tree seed plus JTBD-flavored discovery brief | Exploratory framing makes outcome, opportunities, options, pressure points, and downstream handoff readable without chat context | Missing authored discovery sections remain explicit and never become synthesized filler |
| `system-shaping` | domain-map plus structural-options packet | Boundary decisions, domain responsibilities, structural alternatives, and sequencing rationale remain explicit for downstream architects and implementers | Missing shaping sections remain visibly missing instead of being inferred from systems-language prose |
| `review` | findings-first review bundle | Severity- and evidence-oriented review findings, decision impact, and final disposition remain explicit and reusable outside chat | Missing review sections or unresolved evidence remain visible instead of being normalized into acceptance |

## Contract Rules

- Shape guidance must improve packet readability for the intended artifact
  audience.
- Shape guidance must not change Canon's approval, evidence, or risk posture.
- Follow-on shape rules apply only to `discovery`, `system-shaping`, and
  `review` in this slice.