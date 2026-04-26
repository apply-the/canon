# Decision Log: Architecture ADR And Options

- **D-001**: Keep the package architecture-only.
  - Rationale: This feature is intentionally the smallest coherent bundle spanning authored-body specialization, standard artifact shape, and option analysis.
  - Consequence: Follow-on work for `requirements` and `change` stays on the roadmap.

- **D-002**: Reuse existing architecture artifact file names.
  - Rationale: Strengthening current artifacts is lower risk than adding a new packet family.
  - Consequence: Contract and renderer changes must fit the existing packet layout.

- **D-003**: Preserve C4 behavior unchanged.
  - Rationale: The slice should improve decision review without reopening the already-delivered C4 feature.
  - Consequence: Validation must include explicit non-regression checks for the three C4 artifacts.

- **D-004**: Keep missing authored decision sections explicit.
  - Rationale: Critique-first honesty is a core Canon invariant.
  - Consequence: Tests and docs must describe missing-body behavior clearly.

- **D-005**: Treat the six option-analysis headings as the new required decision-facing contract for this slice.
  - Rationale: This isolates the new behavior under test from the already-existing architecture and C4 authored sections.
  - Consequence: Missing-body validation for feature 018 targets `Decision Drivers`, `Options Considered`, `Pros`, `Cons`, `Recommendation`, and `Why Not The Others`.

- **D-006**: Emit ADR-style `Consequences` while accepting legacy `Risks` authored input.
  - Rationale: The strengthened architecture packet should read like a real decision record without breaking existing authored briefs immediately.
  - Consequence: Docs and tests must describe and validate the `Risks` to `Consequences` compatibility path explicitly.