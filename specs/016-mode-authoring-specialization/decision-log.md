# Decision Log: Mode Authoring Specialization

- **D-001**: Limit the first implementation slice to `requirements`, `discovery`, and `change`.
  - **Rationale**: Highest value among still-generic modes while keeping risk bounded.
  - **Alternatives considered**: full multi-mode rollout; single-mode pilot.

- **D-002**: Reuse a single `## Missing Authored Body` honesty marker across authored-body-specialized modes.
  - **Rationale**: One reviewer-visible fallback contract is easier to test and explain.
  - **Alternatives considered**: per-mode marker strings; architecture-only constant reuse with no generalization.

- **D-003**: Update the existing docs template/example filenames instead of creating a second hierarchy.
  - **Rationale**: Preserve the repository's current discoverability path and avoid parallel conventions.
  - **Alternatives considered**: nested `docs/templates/canon-input/<mode>/brief.md` and parallel examples.

- **D-004**: Treat near-match authored headings as missing unless a compatibility alias is explicitly documented in the contract.
  - **Rationale**: Keeps the contract strict, deterministic, and honest.
  - **Alternatives considered**: permissive fuzzy heading matching.