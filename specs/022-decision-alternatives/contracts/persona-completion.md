# Contract: Persona Completion

## Scope

This contract defines which personas are runtime-targeted and which are
guidance-only in the `022` slice.

## Runtime-Targeted Persona Mapping

| Mode | Persona | Intended Audience | Persona May Influence | Persona Must Not Influence |
|------|---------|-------------------|-----------------------|----------------------------|
| `system-shaping` | System-design counterpart | Reviewers shaping a capability before structure is fixed | Framing of structural options, boundary clarity, and tradeoff emphasis | Approval posture, evidence truthfulness, or missing-section honesty |
| `architecture` | Architecture-decision counterpart | Reviewers evaluating architecture boundaries and decisions | Decision framing, option rationale, and architecture readability | Approval posture, missing authored sections, or risk semantics |
| `change` | Change-planning counterpart | Maintainers reviewing bounded design alternatives | Preservation emphasis, sequencing clarity, and bounded ownership language | Allowed change surface, invariant honesty, or validation requirements |
| `implementation` | Delivery-lead counterpart | Engineers choosing a concrete stack for execution | Constraint framing, evaluation criteria, and delivery impact language | Runtime authority, evidence fabrication, or approval semantics |
| `migration` | Migration-lead counterpart | Reviewers evaluating coexistence or replacement paths | Compatibility framing, rollback clarity, and modernization tradeoffs | Recommendation-only posture, evidence truthfulness, or release gates |

## Guidance-Only Persona Completion

| Mode | Persona | Intended Audience |
|------|---------|-------------------|
| `review` | Lead or staff software engineer reviewer | Maintainers evaluating code or packet quality |
| `pr-review` | Pull-request reviewer counterpart | Engineers consuming diff-level review comments |
| `verification` | Adversarial verifier counterpart | Reviewers testing claims against evidence |
| `incident` | Incident commander or operational lead counterpart | Operators reviewing containment credibility |

## Boundary Rules

- Persona guidance is advisory and presentational, not authoritative.
- Guidance-only persona completion in this slice must not imply new runtime
  packet behavior.
- Any conflict between persona framing and Canon contract semantics resolves in
  favor of Canon contract semantics.
