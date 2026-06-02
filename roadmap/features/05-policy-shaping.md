# 05 - Policy Shaping

## Problem
Canon effectively handles ubiquitous language changes via `domain-language` and `domain-model`. However, proposing changes to repo-wide rules, guidelines (like `.agents/skills`), or a core "constitution" currently falls under generic document `change` packets. Changing rules without structured checks is dangerous.

## Proposal
Create a `policy-shaping` governance mode.
Updating policies should be treated as fundamentally altering the behavior constraints for all future autonomous modes. A `policy-shaping` mode must enforce retroactive sanity checks (e.g., "Does this new policy break or contradict previously compliant behavior?") and demand explicit review of the downstream ripple effects.

## Risk Profile

**Governance Zone**: Red (systemic mutation of enforcement rules).
A policy change alters the behavioral constraints for every future autonomous
run. Miscalibrated rules can silently break compliant work or create unbounded
migration debt. Requires human Systemic Impact sign-off before the new rule is
enforceable.

## Why Existing Modes Are Not Enough
- `domain-language` and `domain-model` stabilize meaning, but they do not own
  repo-wide enforcement rules.
- `change` can describe a document edit, but it does not naturally quantify the
  retroactive compliance blast radius of a new policy.

## Dependencies

- **None upstream**: policy shaping is independent and can start in parallel.
- **Pairs with 06 (Observability Design)**: observability constraints are
  themselves policies; the two modes share a natural review surface.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `domain-language` | Adjacent: language stabilizes vocabulary; policy stabilizes rules. |
| `domain-model` | Adjacent: model stabilizes structure; policy stabilizes enforcement. |
| `change` | Downstream: the approved policy diff is eventually applied as a change packet. |
| `verification` | Consumed: retroactive conformance checking is a specialized verification. |
| `review` | Terminal: the completed policy packet requires formal review approval. |

## Entry Gates
- The draft rule must be specific enough to test or audit; vague guidance is not
  sufficient input.
- The packet must name the protected surface: code, prompts, skills, release
  workflow, approval flow, or repo-wide documentation.
- Known exceptions and legacy areas must be identified up front rather than
  discovered only after the rule is declared final.

## Operational Mechanics
- **Inputs**: A proposed governance rule change (`draft-policy.md`) and the repository's existing "constitution".
- **Workflow Steps**:
  1. **Drafting & Synthesis**: The language of the new policy is refined to be directly enforceable by downstream rules (e.g., turning a vague guideline into a specific assertion).
  2. **Impact Radius Assessment (Dry Run)**: A consuming runtime or operator-facing validator runs an exploratory validation pass against the *existing* codebase and returns evidence describing how many files or systems currently violate the new draft rule.
  3. **Migration Planning**: If the policy introduces widespread violations, the agent must generate a conformance migration strategy (e.g., "Apply exceptions to legacy modules and schedule tech debt resolution").
  *(Note: Canon defines the proposal, impact report, waiver, and sign-off semantics. Command surfaces that execute audits or mutate workspace behavior remain owned by the consuming runtime.)*
- **Required Artifacts**: A `policy-diff.md` showing the semantic change to the constitution, and a `conformance-impact-report.md` detailing retroactive violations and the required migration strategy. This explicitly requires human `Systemic Impact` sign-off.

## Exit Gates
- The proposed rule must be enforceable in language precise enough for humans and
  downstream automation to interpret consistently.
- The packet must quantify impact radius, not just assert that impact exists.
- If migration is required, the packet must name the waiver, exception, or
  staged conformance path before the rule can be accepted.

## Packet Shape
- `01-policy-context.md`: current rule, motivation, and protected surface.
- `02-proposed-rule.md`: canonical wording and examples.
- `03-conformance-impact.md`: current violations, edge cases, and ambiguities.
- `04-migration.md`: waiver policy, rollout phases, and debt created.
- `05-approval.md`: human sign-off boundary and downstream enforcement notes.

## Success Criteria

- Every accepted policy is enforceable: a downstream automation or human can
  unambiguously determine compliance without interpretation guesswork.
- Impact radius is quantified before the rule lands, not discovered
  retroactively through broken runs.
- Legacy exceptions are named and time-bounded rather than left as permanent
  silent waivers.