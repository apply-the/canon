# Validation Report: PR Review Conventional Comments

## Summary

Validation proved that `pr-review` now emits a Conventional Comments shaped
artifact while preserving existing approval, summary, and publish semantics.

## Validation Ownership

- **Generation owners**: runtime contract, rendering, summary, docs, and skill
  changes
- **Structural validators**: formatter, linter, and repository consistency
  checks
- **Logical validators**: contract and integration tests for artifact shape,
  mapping, approval-state preservation, and publish readability
- **Independent validators**: separate read-through of the emitted packet to
  confirm the new artifact reads like real review comments and does not hide
  must-fix severity

## Structural Validation

- **Status**: Passed
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `/bin/bash scripts/validate-canon-skills.sh`
- Result: formatting and lint validation passed after the additive artifact,
  tests, method metadata, user-facing guidance, and skill mirrors were updated.

## Logical Validation

- **Status**: Passed
- `cargo test --test pr_review_contract --test pr_review_run --test pr_review_publish`
- `cargo test -p canon-engine pr_review`
- Result: contract coverage now requires `conventional-comments.md`; run and
  publish flows emit the new artifact; deterministic kind mapping is validated
  in `canon-engine`; approval-gated runs keep `review-summary.md` as the
  primary artifact and still require explicit disposition for unresolved
  must-fix findings.

## Independent Validation

- **Status**: Passed
- Generated a real `pr-review` run in a temporary Git repository and reviewed
  the emitted `conventional-comments.md` packet directly.
- Confirmed the artifact is readable as standalone markdown with `Summary`,
  `Evidence Posture`, `Conventional Comments`, and `Traceability` sections.
- Confirmed note-only packets render severity-safe kinds such as `praise` and
  the artifact remains surface-scoped without fabricated line anchors.

## Exit Criteria

- **Passed**: `pr-review` emits `conventional-comments.md`
- **Passed**: existing disposition semantics remain unchanged
- **Passed**: publish output remains readable and traceable
- **Passed**: docs and skill surfaces describe delivered behavior
