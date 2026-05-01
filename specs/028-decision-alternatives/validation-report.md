# Validation Report: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Status

Completed

- The 028 decision-support slice is implemented for `system-shaping`,
  `change`, `implementation`, and `migration`, with `architecture` retained as
  the regression anchor.
- Release surfaces, shared runtime-compatibility references, impacted docs,
  templates, examples, and changelog are aligned to `0.28.0`.
- Final validation is green across skill synchronization, targeted tests,
  release-surface regression, formatting, linting, full-workspace regression,
  and LCOV generation.

## Structural Validation

- Confirm `spec.md`, `plan.md`, `contracts/`, and repository-facing docs
  describe the same in-scope modes, packet families, evidence posture, and
  release surface.
- Confirm embedded skill sources and mirrored `.agents/skills/` files remain
  synchronized via `/bin/bash scripts/validate-canon-skills.sh`.
- Confirm version references report `0.28.0` consistently in `Cargo.toml`,
  `CHANGELOG.md`, `README.md`, `ROADMAP.md`, and shared runtime-compatibility
  references.
- Confirm non-target runtime surfaces and `.canon/` layout remain unchanged.

### Completed Structural Evidence

- Planning review completed for `spec.md`, `plan.md`, `research.md`,
  `data-model.md`, and the feature contracts.
- `/bin/bash scripts/validate-canon-skills.sh` passed after the skill-source,
  mirror, and runtime-compatibility updates.
- `cargo test --test release_028_docs` passed and now locks the `0.28.0`
  version surfaces in `Cargo.toml`, `Cargo.lock`, `README.md`,
  `CHANGELOG.md`, `ROADMAP.md`, `AGENTS.md`, and the shared runtime
  compatibility references.
- Release-facing docs and examples were reviewed after the final docs sweep;
  non-target runtime semantics, `.canon/` layout, approval targets, and
  publish destinations remain unchanged.

## Logical Validation

- Run focused docs, renderer, contract, and run tests for `system-shaping`,
  `change`, `implementation`, and `migration`.
- Run focused regression validation for `architecture` to confirm the existing
  option-analysis baseline remains intact.
- Run one positive-path walkthrough and one missing-section or missing-evidence
  walkthrough for each runtime-targeted behavior group.
- Run release-surface checks for versioned docs and compatibility references.
- Run final formatting, lint, coverage, and full workspace regression commands.

### Completed Logical Evidence

- Focused framework validation passed with
  `cargo test --test implementation_authoring_renderer --test implementation_authoring_docs --test implementation_contract --test implementation_run --test migration_authoring_renderer --test migration_authoring_docs --test migration_contract --test migration_run`.
- Additional targeted regressions passed while updating stale fixtures and
  final release coverage: `cargo test --test change_authoring_contract`,
  `cargo test --test change_governed_execution`,
  `cargo test --test change_invocation_contract`,
  `cargo test --test approve_resolution`,
  `cargo test --test direct_runtime_coverage <targeted-test>`,
  `cargo test --test run_lookup recommendation_only_implementation_runs_remain_resolvable_via_last_alias`,
  `cargo test --test policy_and_traces implementation_run_persists_recommendation_only_mutation_traces`,
  and `cargo test --test release_028_docs`.
- `cargo fmt --check` passed on the final workspace state.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  passed on the final workspace state.
- `cargo nextest run --workspace --all-features` passed with `614/614` tests
  green.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
  completed successfully and produced the final LCOV artifact.
- Touched runtime-file coverage from `lcov.info`:
  - `crates/canon-engine/src/artifacts/contract.rs` -> `LF:742`, `LH:700`
  - `crates/canon-engine/src/artifacts/markdown.rs` -> `LF:2271`, `LH:2080`
- Modified and newly added Rust test files were exercised by the targeted
  `cargo test` runs above plus the final `cargo nextest run` and
  `cargo llvm-cov` executions.

## Independent Validation

- Review `spec.md`, `plan.md`, and `tasks.md` before implementation to confirm
  scope, invariants, and validation separation remain coherent.
- Perform a read-only review of the final diff to confirm decision-support
  wording stays evidence-backed and recommendation-only.
- Perform one release-surface review across roadmap, mode guide, changelog,
  README, manifests, and compatibility references to confirm the delivered
  slice and remaining roadmap candidates are accurately described.

### Completed Independent Evidence

- Planning artifact review completed before task generation.
- Final diff review confirmed invariant preservation: no `.canon/` schema or
  publish-destination changes, no new approval semantics, and no widening of
  recommendation-only posture beyond the existing model.
- Final release-surface review confirmed `0.28.0` alignment across manifests,
  lockfile, shared runtime-compatibility references, `README.md`,
  `CHANGELOG.md`, `ROADMAP.md`, `AGENTS.md`, `docs/guides/modes.md`, and the
  impacted templates/examples.
- Evidence-honesty review confirmed the new sections only preserve authored
  decision drivers, candidate frameworks, decision evidence, and rejection
  rationale; missing-section handling still emits explicit honesty markers.

## Evidence Paths

- `specs/028-decision-alternatives/decision-log.md`
- `specs/028-decision-alternatives/tasks.md`
- `specs/028-decision-alternatives/contracts/decision-packet-shapes.md`
- `specs/028-decision-alternatives/contracts/release-alignment.md`
- `ROADMAP.md`
- `README.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `Cargo.lock`
- `AGENTS.md`
- `tests/release_028_docs.rs`
- `tests/direct_runtime_coverage.rs`
- `defaults/embedded-skills/`
- `.agents/skills/`
- `lcov.info`
- focused test outputs for the targeted mode surfaces
- `cargo llvm-cov` output for touched Rust files
- full `cargo nextest run` output for final regression coverage