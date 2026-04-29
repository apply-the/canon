# Contract: Security Assessment Packet Shape

## Scope

This contract defines the artifact family and authored-body expectations for the
new `security-assessment` mode.

## Artifact Family

| Artifact | Required Reviewer Outcome |
|----------|---------------------------|
| `assessment-overview.md` | Reviewer can identify the bounded assessment scope, in-scope assets, trust boundaries, and out-of-scope areas |
| `threat-model.md` | Reviewer can inspect the explicit threats or attacker goals mapped to the named surfaces |
| `risk-register.md` | Reviewer can inspect rated findings with likelihood, impact, proposed owner, and status |
| `mitigations.md` | Reviewer can inspect recommendation-only controls and their tradeoffs |
| `assumptions-and-gaps.md` | Reviewer can identify unknowns, missing telemetry, and confidence limits |
| `compliance-anchors.md` | Reviewer can see which standards or control families inform the packet without mistaking it for an audit |
| `assessment-evidence.md` | Reviewer can trace what source material grounded the packet and where evidence is still missing |

## Canonical Authored Sections

The authored brief must include concrete content for the canonical H2 sections
required by the mode skill. The renderer preserves those sections verbatim in
the corresponding artifacts.

## Honesty Rules

- If a required authored section is absent or empty, the corresponding artifact
  must surface `## Missing Authored Body` rather than inventing threats,
  findings, or mitigations.
- If evidence is stale, weak, or unavailable, the packet must preserve the gap
  explicitly instead of presenting confidence it does not have.
- Compliance anchors remain contextual references only and must not be phrased
  as certification or audit completion.

## Posture Rules

- The packet remains recommendation-only.
- The packet must not imply Canon executed remediations or verified live
  security posture beyond the authored and repository-grounded inputs.