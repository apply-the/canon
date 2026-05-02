# Quickstart: Guided Run Operations And Review Experience

## Goal

Validate that Canon now presents one coherent operator story for completed,
blocked, approval-gated, approved, and resumed runs.

## 1. Run focused engine and CLI guidance tests

```bash
cargo test -p canon-engine build_action_chips_for
cargo test -p canon-engine recommend_next_action
cargo test -p canon-cli run_summary_markdown
```

## 2. Run shared next-step renderer regressions

```bash
cargo test --test render_next_steps
```

## 3. Run a focused governed-flow integration check

```bash
cargo test --test integration/implementation_run implementation_run_requires_governed_review_before_execution
```

If the exact test name changes during implementation, keep the focused target on
the integration scenario that covers gated, approved, and resumed operator
guidance in one run lifecycle.

## 4. Validate release, docs, and roadmap alignment

```bash
cargo test --test release_036_release_provenance_integrity
```

## 5. Capture coverage for touched Rust files

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
```

## 6. Finish hygiene checks

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run
```

## Review Checklist

- Completed runs keep result-first readability and do not force a fake next
  step.
- Blocked runs distinguish remediation from approval.
- Gated runs prefer inspection before approval when a readable packet exists.
- Approved runs replace stale approval actions with resume guidance.
- CLI markdown, helper scripts, and action-chip fallback text all tell the same
  story.
- `ROADMAP.md`, `CHANGELOG.md`, and other release surfaces describe the shipped
  `0.37.0` operator-guidance behavior.