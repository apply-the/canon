# Feature 011: Tasks

Numbered, dependency-ordered tasks. `[X]` marks completed work.

## Phase A — Spec realignment

- [X] T001 Append D-013, D-014, D-015 to
  `specs/010-controlled-execution-modes/decision-log.md` and supersede
  D-011.
- [X] T002 Open `specs/011-execution-mode-approval-and-chips/` with
  `spec.md`, `plan.md`, `tasks.md`, `validation-report.md`.
- [ ] T003 Update T044 entry in
  `specs/010-controlled-execution-modes/validation-report.md` to
  "Closed with carry-forward to 011".

## Phase B — BUG-1 owner cleanup (independent of Phase C/D)

- [X] T004 Remove `Owner: maintainer` from
  `docs/templates/canon-input/implementation.md`,
  `docs/templates/canon-input/refactor.md`,
  `docs/examples/canon-input/implementation-auth-session-revocation.md`,
  `docs/examples/canon-input/refactor-auth-session-cleanup.md`,
  `/Users/rt/workspace/java-html-sanitizer/canon-input/implementation/brief.md`,
  `/Users/rt/workspace/java-html-sanitizer/canon-input/refactor/brief.md`.
- [X] T005 Thread `default_owner: &str` through
  `render_change_artifact`, `render_implementation_artifact`,
  `render_refactor_artifact`. Update the three call sites in
  `service.rs` to pass `self.resolve_owner("")`. Keep
  `bounded-system-maintainer` as the ultimate fallback.

## Phase C — BUG-2 ExecutionApproval gate (depends on T001)

- [ ] T006 Rewrite `implementation_risk_gate()` and
  `refactor_risk_gate()` in
  `crates/canon-engine/src/orchestrator/gatekeeper.rs` to emit
  `GateStatus::NeedsApproval { target: "gate:execution" }` when no
  matching approval has been recorded.
- [ ] T007 Split mutation profiles in `defaults/policies/adapters.toml`:
  add `implementation-mutation-recommendation`,
  `implementation-mutation-bounded`,
  `refactor-mutation-recommendation`, `refactor-mutation-bounded`.
- [ ] T008 Mirror the split in
  `crates/canon-engine/src/orchestrator/invocation.rs`. Make profile
  selection a function of approval state for the active run.
- [ ] T009 Reorder `policy_decision_attempt()` in
  `crates/canon-engine/src/orchestrator/service.rs` so
  `NeedsApproval` short-circuits before the recommendation-only
  short-circuit.
- [ ] T010 Surface `gate:execution` in `RunSummary.approval_targets`.
- [ ] T011 Wire `canon resume` to re-evaluate the gate, pick the
  bounded profile, re-emit the artifact bundle, and report
  `execution_posture = approved-recommendation`.
- [ ] T012 Add contract tests in
  `tests/contract/implementation_contract.rs` and
  `tests/contract/refactor_contract.rs` for awaiting-approval and
  post-resume states.
- [ ] T013 Update existing integration tests in
  `tests/integration/implementation_run.rs`,
  `tests/integration/refactor_run.rs`, and any other suite that
  asserts immediate `Completed` for these modes.

## Phase D — BUG-3 Action Chips (depends on T010 for the Approve chip)

- [ ] T014 Add `ActionChip` type and
  `ModeResultSummary.action_chips: Vec<ActionChip>` in
  `crates/canon-engine/src/orchestrator/service.rs`.
- [ ] T015 Implement `build_action_chips_for(run_state,
  approval_targets, primary_artifact_path, mode)` and call it from
  `summarize_implementation_mode_result()` and
  `summarize_refactor_mode_result()` first; then from every other
  available-now summarizer.
- [ ] T016 Render chips in
  `crates/canon-cli/src/output.rs::render_mode_result()` and confirm
  Serde JSON output exposes the field with no
  `skip_serializing_if` blocking it.
- [ ] T017 Add snapshot tests for `implementation` and `refactor`
  covering run-started, awaiting-approval, completed-after-approval.

## Phase E — Skills and documentation

- [ ] T018 Update `.agents/skills/canon-implementation/SKILL.md`,
  `.agents/skills/canon-refactor/SKILL.md`,
  `defaults/embedded-skills/canon-implementation/skill-source.md`,
  `defaults/embedded-skills/canon-refactor/skill-source.md` to describe
  the approve+resume cycle and the `Action Chips:` line.
- [ ] T019 Update the `Implementation Flow` and `Refactor Flow`
  walkthroughs in
  `specs/010-controlled-execution-modes/quickstart.md` and the
  matching sections of `MODE_GUIDE.md`.

## Phase F — Validation

- [ ] T020 Run `cargo fmt --check`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  `bash scripts/validate-canon-skills.sh`.
- [ ] T021 Run `cargo test` and `cargo nextest run`.
- [ ] T022 End-to-end smoke against a temporary workspace covering
  both modes with `canon run` -> `canon approve` -> `canon resume`.
- [ ] T023 Re-run T044 walkthrough on
  `/Users/rt/workspace/java-html-sanitizer/canon-input/{implementation,refactor}`
  and capture chip and approval evidence in `validation-report.md`.
