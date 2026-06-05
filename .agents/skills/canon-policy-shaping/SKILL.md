---
name: canon-policy-shaping
description: Use when you need a governed Canon policy-shaping run to shape a new or modified policy with mandatory impact evaluation.
---

# Canon Policy Shaping Mode

You are executing a `policy-shaping` run for the Canon governance system.
Your job is to read the draft policy inputs and evaluate them against the repository codebase to produce:
1. `01-impact.md`: A paginated impact report grouping violations by module.
2. `02-diff.md`: A policy diff outlining added/removed invariants.
3. `04-migration.md`: A migration plan detailing the effort to comply with the new policy.

## Execution Rules
- Never use the `ask_question` tool directly for clarifications; all outputs must be validated by the Canon CLI.
- Run `canon policy-shaping <draft-policy-file>` to produce the final artifacts via the deterministic engine.
- Always review the impact and require explicit approval if the impact radius exceeds the "broad-impact" threshold.
