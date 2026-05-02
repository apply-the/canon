# Research: Guided Run Operations And Review Experience

## Decision 1: Extend the existing run or status summary pipeline

- **Decision**: Keep feature `038` inside the current `EngineService`
  runtime-details path and reuse `RunSummary`, `StatusSummary`,
  `ModeResultSummary`, `RecommendedActionSummary`, and `ActionChip` rather than
  creating a separate operator workflow subsystem.
- **Why**: The controlling code path already exists. `service.rs` computes
  `blocking_classification`, `approval_targets`, `blocked_gates`, `mode_result`,
  and `recommended_next_action`, while `output.rs` already renders result,
  blockers, and next-step sections. The current gap is coherence, not missing
  infrastructure.
- **Alternatives considered**:
  - Create a new operator-guidance service. Rejected because it would duplicate
    runtime facts already assembled in `service.rs`.
  - Push all logic into skill-layer scripts. Rejected because it would force the
    CLI and JSON/runtime surfaces to drift from the canonical runtime state.

## Decision 2: Derive recommended next step and possible actions from the same facts

- **Decision**: Introduce one shared operator-guidance derivation that uses the
  active run state, approval targets, blocked gates, readable artifact paths,
  evidence availability, and `mode_result` to produce:
  - one optional recommended next action
  - an ordered list of valid possible actions
  - progressive-enhancement action chips that mirror the same text contract
- **Why**: The current split is the main source of drift. Today,
  `recommend_next_action()` can recommend `inspect-artifacts` while
  `build_action_chips_for()` only knows about `open-primary-artifact`,
  `inspect-evidence`, `approve`, and `resume`. `output.rs` then prints action
  chips but omits `Possible Actions:` entirely.
- **Alternatives considered**:
  - Keep the current split and patch strings in each renderer. Rejected because
    it would preserve the root cause: multiple places deciding the same next
    move.
  - Overload chips to replace the text contract. Rejected because the skill and
    frontend contract explicitly require `Possible Actions:` and
    `Recommended Next Step:` as the mandatory fallback.

## Decision 3: Prefer readable packet review before approval when possible

- **Decision**: When Canon has already emitted a readable packet for a gated or
  blocked run, the recommended next step should point to packet review before
  approval or other continuation actions.
- **Why**: The shared output-shape references and status-skill guidance already
  establish this rule. The operator should review the packet that explains the
  gate before taking a governed action.
- **Implications**:
  - Approval-gated runs with readable artifacts should prioritize
    `inspect-artifacts` or the direct primary-artifact open path over immediate
    approval.
  - Blocked runs with readable artifacts should point to artifact review rather
    than evidence-first or approval-oriented flows.
  - Resume should appear only after approvals are recorded and no approval
    targets remain.

## Decision 4: Keep chips as progressive enhancement, not the canonical action list

- **Decision**: Preserve the existing chip posture: chips are optional host
  affordances that mirror Canon-backed text next steps. They do not replace the
  ordered textual action list.
- **Why**: The repository already codifies this in the shared output-shape
  reference and the codex-skills frontend contract. Hosts that cannot render
  chips must still receive a complete text contract.
- **Alternatives considered**:
  - Expand chips until they become the only guided action surface. Rejected
    because it would break the mandatory fallback contract.
  - Remove chips from the runtime summary. Rejected because chip-capable hosts
    already use them as high-signal progressive enhancement.

## Decision 5: Validate through focused summary and script regressions before full-suite closeout

- **Decision**: Use narrow, behavior-scoped tests first, then finish with
  workspace-wide hygiene checks.
- **Focused validation targets**:
  - engine helper tests in `crates/canon-engine/src/orchestrator/service/tests.rs`
  - next-action and action-chip unit tests in `next_action.rs` and
    `summarizers.rs`
  - CLI markdown rendering tests in `crates/canon-cli/src/output.rs`
  - shared script regression tests in `tests/render_next_steps.rs`
  - one focused integration regression for gated, approved, and resumed flows in
    `tests/integration/implementation_run.rs`
- **Closeout validation**:
  - release/docs alignment test(s)
  - `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo nextest run`