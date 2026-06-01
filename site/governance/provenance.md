# AI Provenance

As repositories absorb more AI-generated code, a new risk emerges: **mystery code**. When complex logic is committed, reviewers need to know exactly how it was generated, what prompt was used, and what context the model was looking at.

Canon solves this through strict **AI Provenance** tracking.

## The Provenance Chain

Every piece of output generated during a Canon session carries metadata that answers critical questions:
- **Which Host/Model** generated this payload? (e.g., `gpt-4o`, `claude-3-5-sonnet`)
- **What Reasoning Profile** was active? (e.g., `bounded_self_consistency`)
- **What Context** was injected into the prompt?

## Immutable Traces

Provenance is not a guess; it is mathematically derived from the execution traces. 

When a session reaches the `publish` stage, Canon attaches a lightweight provenance signature to the governed packet. This ensures that any artifact entering your repository's permanent **Project Memory** has a clear, auditable lineage back to its AI originator.

## Codebase Integrity

By enforcing AI Provenance, Canon ensures that:
1. You can audit the exact prompts that led to security regressions.
2. You can reproduce agentic workflows by feeding identical context into the same model profile.
3. Your repository retains a clear boundary between human-authored intent and machine-generated implementation.