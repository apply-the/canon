# Validation Report Plan: pr-review Optional Inline Anchors

## Validation Ownership

- Generation owner: feature implementer authoring the review-domain changes,
  rendered artifact updates, and guidance alignment.
- Independent validation owner: maintainer review of rendered packet samples,
  contract adherence, and regression evidence after implementation.
- Automated validation owner: Rust test, formatting, and linting commands run
  after code changes are complete.

## Planned Validation Layers

### Structural Validation

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- Contract review against `contracts/conventional-comment-anchor-contract.md`
- Artifact-shape inspection confirming explicit scope remains present in all
  Conventional Comment entries

### Logical Validation

- Unit tests for anchor derivation covering:
  - single-line anchor
  - contiguous multi-line span anchor
  - no changed surfaces
  - cross-surface findings
  - disjoint or ambiguous intervals
  - stale or invalidated coordinate candidates
- Integration tests covering:
  - rendered Conventional Comments output with line anchors
  - rendered output with span anchors
  - mixed anchored and unanchored findings in the same packet
  - unchanged `review-summary.md` disposition behavior

### Independent Validation

- Maintainer packet-sample review using at least one anchored packet and one
  degraded scope-only packet
- Reader check that anchor text is understandable without host-specific tooling
- Regression review confirming readiness posture and must-fix behavior did not change

## Evidence To Record

- command transcripts or summarized results for formatting, linting, and tests
- representative rendered artifact excerpts for line-anchor, span-anchor, and
  degraded scope-only cases
- independent maintainer sign-off notes on contract adherence and packet readability

## Exit Criteria

- All structural checks pass.
- Logical tests cover the scenarios listed in the feature spec.
- Independent review confirms no fabricated precision and no readiness regression.
- Validation evidence is linked back to the implementing Canon run.

## Current Status

- Implementation complete for the review-domain anchor model, Conventional
  Comment rendering, fixture coverage, and guidance/doc updates.
- Non-linker structural validation passed for Rust formatting, skill mirrors,
  and assistant package metadata after the implementation and release-alignment
  updates.
- Focused workspace-local diagnostics report no editor errors in the touched
  Rust and test files.
- Full executable validation now passes locally for focused tests, workspace
  tests, `nextest`, `clippy`, and LCOV generation.
- Independent maintainer review is recorded complete for the representative
  anchored and degraded packet samples.

## Structural Validation Evidence

- `cargo fmt --check`: passed after formatting the touched Rust sources and
  tests.
- `/bin/bash scripts/validate-canon-skills.sh`: passed, confirming the repo and
  embedded `canon-pr-review` skill mirrors remain aligned after the anchored
  comment guidance updates.
- `/bin/bash scripts/validate-assistant-plugins.sh`: passed, confirming the
  assistant package manifests and metadata stayed aligned after the `0.60.0`
  release-line bump.
- Reasoning-posture release drift inspection: passed after aligning
  `tech-docs/integration/governed-reasoning-posture-contract.md`, the feature-local
  contract brief, and `tests/contract/governed_reasoning_posture_contract.rs`
  to Canon `0.60.x` while preserving the repo's Boundline `0.62.x` window.
- `cargo test -p canon-engine derive_anchor --lib`: passed with the focused
  anchor derivation cases.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.
- VS Code language-service diagnostics: no errors in
  `crates/canon-engine/src/review/findings.rs`,
  `crates/canon-engine/src/artifacts/markdown/governance.rs`,
  `tests/contract/pr_review_anchor_contract.rs`, and
  `tests/integration/pr_review_run.rs` after implementation.

## Logical Validation Evidence

- Added focused unit coverage in `crates/canon-engine/src/review/findings.rs`
  for line anchors, span anchors, multiple surfaces, disjoint intervals, and
  missing durable interval evidence.
- Added contract coverage in `tests/contract/pr_review_anchor_contract.rs` for
  line, span, cross-surface, stale, and imported no-evidence packets.
- Added integration coverage in `tests/integration/pr_review_run.rs` for
  rendered line anchors, rendered span anchors, and degraded cross-surface
  scope-only comments.
- `cargo test --test pr_review_anchor_contract`: passed.
- `cargo test --test pr_review_run`: passed.
- `cargo test --workspace`: passed.
- `cargo nextest run`: passed.

## Coverage Validation Evidence

- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`:
  passed and produced `lcov.info` in the repository root.
- `lcov.info` includes `SF:` entries for:
  - `crates/canon-engine/src/review/findings.rs`
  - `crates/canon-engine/src/artifacts/markdown/governance.rs`
- In this workspace LCOV output, standalone test harness files such as
  `tests/pr_review_run.rs` and `tests/pr_review_anchor_contract.rs` are not
  emitted as `SF:` entries; executable coverage evidence for this feature is
  therefore recorded through the touched source files plus the passing focused
  and workspace test runs.

## Representative Artifact Evidence

- Anchored line form: `Anchor: tests/reviewer.md:3`
- Anchored span form: `Anchor: tests/reviewer.md:3-4`
- Degraded scope-only form: comments keep `scope:file` or `scope:surface` and
  omit `Anchor:` entirely when precision is not durable.

## Independent Review Checkpoints

- Maintainer packet-sample review completed against one anchored packet sample
  (`Anchor: tests/reviewer.md:3`) and one degraded scope-only packet sample
  with no `Anchor:` line.
- Maintainer confirmed the anchor text remains understandable without
  host-specific tooling and that explicit derived scope remains authoritative.
- Maintainer confirmed that `review-summary.md` remains the primary artifact
  and that readiness behavior stays unchanged.