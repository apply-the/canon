# Decision Log: Mode Authoring Specialization Completion

- **D-001**: Complete the remaining four modes in one feature slice.
  - Rationale: The renderer/doc/test pattern is now established, and finishing the rollout in one slice avoids a misleading half-complete roadmap and release state.
  - Consequence: Planning and validation must cover both critique-oriented and operational modes together.

- **D-002**: Reuse the shared authored-body extraction and missing-body marker pattern.
  - Rationale: Existing specialization slices already define the required honesty model.
  - Consequence: The implementation should converge on shared helpers instead of proliferating new renderer-specific rules.

- **D-003**: Treat canonical H2 headings as the contract for all remaining targeted artifacts.
  - Rationale: Explicit contracts are easier to review, document, and validate than fuzzy heading recovery.
  - Consequence: Tests must cover missing, blank, and near-match headings.

- **D-004**: Preserve current mode posture and gate semantics while improving authored fidelity.
  - Rationale: This feature is about packet honesty and discoverability, not governance redesign.
  - Consequence: Validation must prove that review disposition, verification blockers, and incident/migration recommendation-only posture are unchanged.

- **D-005**: Land release/docs/version surfaces in the same slice as the runtime changes.
  - Rationale: The repository uses versioned docs and compatibility references to describe shipped behavior.
  - Consequence: `Cargo.toml`, `CHANGELOG.md`, `ROADMAP.md`, `docs/guides/modes.md`, and runtime-compatibility references are first-class implementation surfaces.

- **D-006**: Use `incident` and `migration` as the first local runtime probe inside the shared foundation work, while keeping `review` and `verification` as the MVP story.
  - Rationale: The operational modes still offer the smallest falsifiable renderer/handoff slice, but the feature priority remains highest for `review` and `verification` because they deliver the strongest user-facing authoring value.
  - Consequence: Early focused validation can probe the authored-body conversion path on incident/migration, but story completion and release readiness still follow the US1 → US2 → US3 ordering in `tasks.md`.