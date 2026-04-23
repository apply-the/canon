# Feature 011: Implementation Plan

Implementation guidance for `specs/011-execution-mode-approval-and-chips/spec.md`.

## Phasing

The work splits into six phases. Phase B (cosmetic owner cleanup) and
Phase A (spec realignment) are independent and land first. Phase C
(approval gate plumbing) is the largest code change and unlocks Phase D
(action chip emission) for the gated chip variants. Phase E (skills and
docs) and Phase F (full validation) close the feature.

### Phase A — Spec realignment

- Append D-013, D-014, D-015 to
  `specs/010-controlled-execution-modes/decision-log.md` and mark D-011
  as superseded by D-013. (Done.)
- Open this folder with `spec.md`, `plan.md`, `tasks.md`,
  `validation-report.md`. Reference back to 010.
- Update `specs/010-controlled-execution-modes/validation-report.md`
  T044 entry to "Closed with carry-forward to 011".

### Phase B — BUG-1 owner cosmetic cleanup

- Remove the literal `Owner: maintainer` line from
  `docs/templates/canon-input/{implementation,refactor}.md`,
  `docs/examples/canon-input/{implementation-auth-session-revocation,refactor-auth-session-cleanup}.md`,
  and the two java-html-sanitizer briefs created during T044.
- Thread `default_owner: &str` through
  `render_change_artifact`, `render_implementation_artifact`,
  `render_refactor_artifact` in
  `crates/canon-engine/src/artifacts/markdown.rs`. Use the
  git-derived `resolve_owner("")` value at the call sites in
  `crates/canon-engine/src/orchestrator/service.rs`. Keep
  `bounded-system-maintainer` as the ultimate fallback only when both
  the brief and git config are silent.

### Phase C — BUG-2 ExecutionApproval gate

- In `crates/canon-engine/src/orchestrator/gatekeeper.rs`, replace the
  stub `implementation_risk_gate()` and `refactor_risk_gate()` with
  evaluators that always emit
  `GateStatus::NeedsApproval { target: "gate:execution" }` when no
  matching approval has been recorded. The gate is unconditional in
  v0.1 per D-013.
- Split mutation profiles in
  `defaults/policies/adapters.toml` and
  `crates/canon-engine/src/orchestrator/invocation.rs`:
  - `implementation-mutation-recommendation` /
    `implementation-mutation-bounded`
  - `refactor-mutation-recommendation` /
    `refactor-mutation-bounded`
- In `crates/canon-engine/src/orchestrator/service.rs`
  `policy_decision_attempt()`, reorder branches so
  `PolicyDecisionKind::NeedsApproval` short-circuits before the
  recommendation-only short-circuit, and surface the `gate:execution`
  target on `RunSummary.approval_targets`.
- Wire `canon resume` so that, post-approval, the orchestrator
  re-evaluates the gate, picks the bounded constraint profile,
  re-emits the artifact bundle with approval lineage, and reports
  `execution_posture = approved-recommendation`.
- Add contract tests in
  `tests/contract/implementation_contract.rs` and
  `tests/contract/refactor_contract.rs` for the awaiting-approval and
  post-resume states. Update existing integration tests that asserted
  immediate `Completed` to add the approve+resume step.

### Phase D — BUG-3 Action Chips emission

- Add `ActionChip` next to `ResultActionSummary` in
  `crates/canon-engine/src/orchestrator/service.rs` and extend
  `ModeResultSummary` with `action_chips: Vec<ActionChip>` (always
  present, may be empty).
- Implement `build_action_chips_for(run_state, approval_targets,
  primary_artifact_path, mode)` and call it from the
  implementation/refactor summarizers first, then from every other
  available-now summarizer.
- Render the chips in the CLI text output via
  `crates/canon-cli/src/output.rs::render_mode_result()`. The JSON
  output gets the field automatically through Serde derive.
- Add snapshot tests for `implementation` and `refactor` covering
  run-started, awaiting-approval, completed-after-approval.

### Phase E — Skills and documentation

- Update `.agents/skills/canon-implementation/SKILL.md`,
  `.agents/skills/canon-refactor/SKILL.md`, and the matching
  `defaults/embedded-skills/.../skill-source.md` files to describe the
  approve+resume cycle and the `Action Chips:` line.
- Update the `Implementation Flow` and `Refactor Flow` walkthroughs in
  `specs/010-controlled-execution-modes/quickstart.md` and the
  matching sections of `MODE_GUIDE.md`.

### Phase F — Validation

- `cargo fmt --check`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  `bash scripts/validate-canon-skills.sh`,
  `cargo test`, `cargo nextest run`.
- End-to-end smoke against a temporary workspace covering both modes
  with `canon run` -> `canon approve` -> `canon resume`.
- Re-run T044 walkthrough on
  `/Users/rt/workspace/java-html-sanitizer/canon-input/{implementation,refactor}`
  and capture chip and approval evidence in
  `validation-report.md`.

## Risks and Mitigations

- **Risk**: Reordering `policy_decision_attempt()` branches changes
  recommendation-only outcomes for unrelated modes.
  **Mitigation**: Scope the reorder to the implementation/refactor
  branch only, or guard the reorder with a mode check; rely on the
  full integration suite to catch drift.
- **Risk**: New constraint profiles invalidate existing manifest
  fixtures.
  **Mitigation**: Update fixture profiles inside this feature; keep the
  pre-approval profile name aligned with prior behavior so old
  manifests deserialize cleanly.
- **Risk**: Hosts that already render `primary_artifact_action`
  duplicate the `Open primary artifact` chip.
  **Mitigation**: Document the duplication in the skill update and the
  shape doc; a future feature can deprecate `primary_artifact_action`.
