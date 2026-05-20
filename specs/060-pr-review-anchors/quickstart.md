# Quickstart: pr-review Optional Inline Anchors

## Goal

Implement optional line/span anchors for `pr-review` Conventional Comments
without changing readiness semantics or the mandatory explicit scope contract.

## Prerequisites

- Work on branch `060-pr-review-anchors`.
- Keep `spec.md`, `plan.md`, `research.md`, and the contract document open while
  implementing.
- Treat the persisted `git diff --unified=0` patch as the only authority for
  anchor precision.

## Implementation Sequence

1. Extend the review domain in `crates/canon-engine/src/review/findings.rs`.
   - Add a typed optional anchor owned by each `ReviewFinding`.
   - Keep explicit `scope` mandatory for every finding.
   - Reject ambiguous, cross-surface, stale, or disjoint anchor candidates.

2. Reuse the existing diff-backed input path.
   - Confirm the `pr-review` flow still relies on `crates/canon-adapters/src/shell.rs`
     for changed surfaces and zero-context diff hunks.
   - Do not introduce host-specific APIs or live-repo recomputation.

3. Update the Conventional Comments renderer in
   `crates/canon-engine/src/artifacts/markdown/governance.rs`.
   - Preserve current comment kinds and explicit scope output.
   - Render anchor text only when a finding carries a valid anchor.
   - Use host-agnostic text of the form `surface:start` or `surface:start-end`.

4. Keep readiness and review summary behavior unchanged.
   - Confirm `review-summary.md` stays the primary artifact.
   - Confirm must-fix disposition and approval outcomes do not change.

5. Update reviewer guidance mirrors.
   - Align `.agents/skills/canon-pr-review/SKILL.md` and
     `defaults/embedded-skills/canon-pr-review/skill-source.md` with the new
     anchored versus scope-only behavior.

## Validation Sequence

1. Run focused unit and integration tests for `pr-review` anchor derivation and rendering.
    - Include `tests/contract/pr_review_anchor_contract.rs` and the anchored/degraded
       cases in `tests/integration/pr_review_run.rs`.
2. Run broader workspace validation expected for bounded-impact Rust changes.
3. Inspect representative rendered packet samples with anchored and degraded comments.
4. Record outcomes in `validation-report.md`.

## Suggested Commands

```bash
cargo fmt --check
cargo test -p canon-engine pr_review --lib
cargo test --test integration pr_review
cargo nextest run
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Done Criteria

- Every comment still has explicit scope.
- Valid single-line evidence produces a line anchor.
- Valid contiguous multi-line evidence produces a span anchor.
- Ambiguous or stale evidence produces no anchor.
- `review-summary.md` and readiness outcomes remain unchanged.