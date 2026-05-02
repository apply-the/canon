# Validation Report: Guided Run Operations And Review Experience

## Planned Evidence

| Area | Validation | Status | Evidence |
|------|------------|--------|----------|
| Engine guidance derivation | focused unit tests for `recommend_next_action`, action ordering, and chip alignment | Passed | `cargo test -p canon-engine build_possible_actions`; `cargo test -p canon-engine build_action_chips_for` |
| CLI markdown rendering | focused renderer tests in `crates/canon-cli/src/output.rs` | Passed | `cargo test -p canon-cli run_summary_markdown` |
| Shared next-step helpers | `tests/render_next_steps.rs` | Passed | `cargo test --test render_next_steps` |
| Gated/approved/resumed lifecycle | focused integration regression | Passed | `cargo test --test implementation_run run_implementation_completes_with_recommendation_only_execution_posture` |
| Release/docs alignment | release-surface guardrail tests | Passed | `cargo test --test release_036_release_provenance_integrity release_docs_and_version_surfaces_align_on_0_37_0_delivery` |
| Coverage on touched Rust files | workspace coverage run | Passed | `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` |
| Formatting | `cargo fmt --check` | Passed | `cargo fmt`; `cargo fmt --check` |
| Linting | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed | workspace task `clippy-all-warnings` |
| Full regression sweep | `cargo nextest run` | Passed | workspace task `nextest-full-suite-green` |

## Independent Review Focus

- Verify that approval-gated, blocked, and resumed runs never suggest invalid
  follow-up actions.
- Verify that `Possible Actions:` and `Recommended Next Step:` stay aligned with
  chip fallback text.
- Verify that completed readable results remain result-first and do not gain a
  fake mandatory next step.
- Verify that docs, roadmap, and changelog describe the same shipped operator
  story as the runtime.

## Findings

- The runtime now emits structured `possible_actions` beside the existing
  `recommended_next_action`, so run and status JSON can preserve ordered
  follow-up guidance without relying on host inference.
- CLI markdown now renders `## Possible Actions` while keeping completed
  result-first runs free of fake mandatory next steps.
- Action-chip recommendation selection now stays aligned with readable-packet
  review for gated or blocked runs that already emitted a primary artifact.
- `README.md`, `CHANGELOG.md`, `ROADMAP.md`, and the release-surface guardrail
  now describe feature `038` as delivered under the existing `0.37.0` line.

## Independent Review Outcome

- Review-first guidance remains explicit: approval is never suggested before a
  readable packet when one exists.
- Resume guidance appears only after approval targets are cleared.
- Completed readable results still avoid a fake mandatory next step while
  preserving drill-down actions.
- No new run-state family, hidden planner loop, or persistence layout was
  introduced.