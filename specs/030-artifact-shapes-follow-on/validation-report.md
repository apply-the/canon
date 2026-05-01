# Validation Report: Industry-Standard Artifact Shapes Follow-On

## Status

Completed

- The follow-on slice for `discovery`, `system-shaping`, `review`, and
  `0.30.0` release alignment is implemented and validated in the current tree.
- `.canon/` runtime storage, canonical `run_id` identity, and the structured
  publish contract from 029 remained unchanged throughout the slice.

## Structural Validation

- Confirm `spec.md`, `plan.md`, `research.md`, `data-model.md`, and
  `contracts/` describe the same follow-on shape, persona, and release
  alignment contract.
- Confirm target skill source files and mirrored skill files remain aligned.
- Confirm release-facing version surfaces align on `0.30.0`.
- Confirm non-targeted modes remain explicitly out of scope in docs and
  planning artifacts.

### Completed Structural Evidence

- Planning review completed for `spec.md`, `plan.md`, `research.md`,
  `data-model.md`, `tasks.md`, and the 030 contracts.
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, and
  the mirrored runtime-compatibility references align on `0.30.0`.
- Embedded and mirrored skill content remains synchronized; `scripts/validate-canon-skills.sh`
  passed.
- The current tree does not retain dedicated README or changelog release tests;
  delivered-slice wording is enforced by the surviving doc regressions in
  `tests/discovery_authoring_docs.rs`,
  `tests/system_shaping_domain_modeling_docs.rs`,
  `tests/review_authoring_docs.rs`, `tests/requirements_authoring_docs.rs`,
  `tests/change_authoring_docs.rs`, `tests/architecture_c4_docs.rs`,
  `tests/mode_authoring_follow_on_docs.rs`, and `tests/skills_bootstrap.rs`.

## Logical Validation

- Run focused discovery docs, renderer, and run tests.
- Run focused system-shaping docs, renderer, and run tests.
- Run focused review docs, renderer, and run tests.
- Run skill mirror validation and release-surface checks.
- Run `cargo llvm-cov`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo nextest run --workspace --all-features`.

### Completed Logical Evidence

- Focused current-tree authoring and compatibility suite passed:
  - `cargo test --test discovery_authoring_docs --test system_shaping_domain_modeling_docs --test review_authoring_docs --test mode_authoring_follow_on_docs --test skills_bootstrap` -> 24 passed.
  - `cargo test --test requirements_authoring_docs` -> 4 passed after relaxing one stale README persona assertion to match the shipped `0.30.0` wording.
- `scripts/validate-canon-skills.sh` passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- `cargo nextest run --workspace --all-features` passed with 615/615 tests green.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` passed and saved `lcov.info`.

### Touched Rust-File Coverage

- No production Rust source files changed in the final 030 tree; the Rust-file
  delta is test-only.
- Workspace `lcov.info` was regenerated successfully for the current tree.
- Modified test-only Rust files validated through the passing targeted test
  binaries above plus the full `cargo nextest` run:
  - `tests/architecture_c4_docs.rs`
  - `tests/change_authoring_docs.rs`
  - `tests/discovery_authoring_docs.rs`
  - `tests/integration/skills_bootstrap.rs`
  - `tests/requirements_authoring_docs.rs`
  - `tests/review_authoring_docs.rs`
  - `tests/system_shaping_domain_modeling_docs.rs`
- Removed test-only files `tests/release_028_docs.rs` and
  `tests/release_029_publish.rs` are intentionally outside the final validation
  surface because they are no longer part of the branch.

## Independent Validation

- Review the final diff to confirm persona guidance never overrides missing-body,
  evidence, or disposition semantics.
- Review updated docs and roadmap text to confirm only the delivered slice is
  described as complete.
- Review `lcov.info` and command outputs to confirm every modified or new Rust
  file has explicit coverage evidence.

### Completed Independent Evidence

- Final diff review confirmed the 030 slice changes authoring guidance, docs,
  mirrors, and test expectations only; it does not widen Canon runtime,
  approval, evidence, or publish behavior.
- Final doc review confirmed `README.md`, `ROADMAP.md`, `docs/guides/modes.md`,
  and the in-scope templates/examples describe only the delivered follow-on
  modes and keep deferred rollout scope explicit.
- Final validation review confirmed the surviving current-tree test surface is
  sufficient to guard the shipped contract after the dedicated README and
  changelog release tests were removed from the branch.

## Evidence Paths

- `specs/030-artifact-shapes-follow-on/decision-log.md`
- `specs/030-artifact-shapes-follow-on/tasks.md`
- `specs/030-artifact-shapes-follow-on/contracts/follow-on-artifact-shapes.md`
- `specs/030-artifact-shapes-follow-on/contracts/persona-boundaries.md`
- `specs/030-artifact-shapes-follow-on/contracts/release-alignment.md`
- focused test outputs
- `lcov.info`
- final release-facing doc diffs