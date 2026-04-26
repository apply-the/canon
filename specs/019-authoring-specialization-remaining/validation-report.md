# Validation Report: Mode Authoring Specialization Follow-On

## Planning Status

- This artifact records the planned validation layers for feature `019-authoring-specialization-remaining` during the planning phase.
- Implementation-phase results will be appended after tasks execute.

## Checklist Status

| Checklist | Total | Completed | Incomplete | Status |
|-----------|-------|-----------|------------|--------|
| `requirements.md` | 20 | 20 | 0 | PASS |

## Governance And Setup Confirmation

- Confirmed `spec.md`, `plan.md`, `contracts/mode-authored-body-contracts.md`, `decision-log.md`, and this validation report carry the declared mode, risk, scope, invariants, and validation split for feature 019.
- Verified the repository is a git worktree and that `.gitignore` already contains the required Rust and universal ignore patterns for this slice.
- Confirmed the current authored-guidance surfaces for `system-shaping`, `implementation`, and `refactor` are the expected targets for the docs-sync work.
- Confirmed the current runtime and example surfaces for the three target modes are the expected code and documentation touchpoints for implementation.

## Baseline Status

- Completed T005: captured the focused pre-implementation baseline for the three targeted modes before authored-body changes land.
- Baseline suite:
	- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test system_shaping_contract --test implementation_contract --test refactor_contract --test system_shaping_run --test implementation_run --test refactor_run`
- Baseline result:
	- `system_shaping_contract`: 3 passed
	- `system_shaping_run`: 2 passed
	- `implementation_contract`: 2 passed
	- `implementation_run`: 3 passed
	- `refactor_contract`: 2 passed
	- `refactor_run`: 3 passed
	- Total: 15 passed, 0 failed

## Foundational Runtime Progress

- Completed T008 by confirming `crates/canon-engine/src/artifacts/contract.rs` already matched the authored-body contract captured in `contracts/mode-authored-body-contracts.md`; no contract schema edit was required.
- Completed T009 by extending `crates/canon-engine/src/artifacts/markdown.rs` so `system-shaping`, `implementation`, and `refactor` artifacts all render canonical authored H2 sections through the shared authored-body helper and emit `## Missing Authored Body` for absent headings.
- Completed T010 by restoring the original authored brief as renderer input for `implementation` and `refactor` in `crates/canon-engine/src/orchestrator/service/mode_change.rs` and verifying `mode_shaping.rs` already feeds the authored context directly.

### Focused Validation Evidence

- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test implementation_run --test refactor_run`
	- Result: 6 passed, 0 failed after restoring authored-brief handoff in `mode_change.rs`.
- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test system_shaping_authoring_renderer --test implementation_authoring_renderer --test refactor_authoring_renderer`
	- Result: 9 passed, 0 failed after the authored-body renderer refactor in `markdown.rs`.

## User-Facing Authored Guidance Progress

- Completed T011 by extending `tests/system_shaping_domain_modeling_docs.rs` to treat the feature 019 contract as the source of truth for the full `system-shaping` authored packet.
- Completed T012 by adding `tests/implementation_authoring_docs.rs` and `tests/refactor_authoring_docs.rs` to enforce canonical heading parity across contract docs, skill sources, mirrored skills, starter templates, and worked examples.
- Completed T013 by updating the embedded skills and mirrored `.agents` skills for `system-shaping`, `implementation`, and `refactor` so they enumerate the canonical authored H2 sections and document the `## Missing Authored Body` fallback.
- Completed T014 by converting the starter inputs under `docs/templates/canon-input/` to the canonical H2 contract for all three targeted modes.
- Completed T015 by rewriting the worked examples under `docs/examples/canon-input/` to exercise the full authored packet contract for the targeted modes.
- Completed T016 by recording the docs-sync decisions and focused evidence in this report and `decision-log.md`.

### Focused Docs-Sync Evidence

- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test system_shaping_domain_modeling_docs --test implementation_authoring_docs --test refactor_authoring_docs`
	- Result: 6 passed, 0 failed after synchronizing the skills, templates, and examples with the canonical authored H2 contract.

## Runtime Preservation Progress

- Completed T017 by extending `tests/contract/system_shaping_contract.rs`, `tests/contract/implementation_contract.rs`, and `tests/contract/refactor_contract.rs` to assert the full canonical authored-body section requirements and gate assignments for the targeted artifact families.
- Completed T018 by adding focused renderer coverage in `tests/system_shaping_authoring_renderer.rs`, `tests/implementation_authoring_renderer.rs`, and `tests/refactor_authoring_renderer.rs` for verbatim preservation, missing-body markers, and near-match heading rejection.
- Completed T019 by extending `tests/integration/system_shaping_run.rs`, `tests/integration/implementation_run.rs`, and `tests/integration/refactor_run.rs` so complete packets prove verbatim preservation while incomplete packets prove both emitted missing-body markers and blocked gate behavior.
- Completed T020 by extending `crates/canon-engine/src/artifacts/markdown.rs` across all targeted packet artifacts so canonical authored H2 sections are preserved verbatim and absent sections emit `## Missing Authored Body`.
- Completed T021 by restoring and validating authored-source handoff in `crates/canon-engine/src/orchestrator/service/mode_change.rs`, confirming `mode_shaping.rs` already feeds authored context directly, and updating `tests/direct_runtime_coverage.rs` to use canonical H2 fixtures for the targeted modes.
- Completed T022 by recording the runtime-preservation decisions and focused evidence in this report and `decision-log.md`.

### Focused Runtime Evidence

- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test system_shaping_authoring_renderer --test implementation_authoring_renderer --test refactor_authoring_renderer`
	- Result: 9 passed, 0 failed. Verbatim preservation, missing-body markers, and near-match rejection are covered at the renderer layer.
- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test system_shaping_contract --test implementation_contract --test refactor_contract --test system_shaping_run --test implementation_run --test refactor_run`
	- Result: 17 passed, 0 failed. Complete packets preserve authored sections; incomplete packets emit `## Missing Authored Body` and remain gate-blocked with `artifact-blocked` classification.
- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test direct_runtime_coverage`
	- Result: 17 passed, 0 failed after switching the engine-level fixtures to canonical H2 authored briefs for `system-shaping`, `implementation`, and `refactor`.

## Rollout And Non-Regression Progress

- Completed T023 by adding `tests/mode_authoring_follow_on_docs.rs` so roadmap and mode-guide wording stay synchronized with the delivered follow-on slice and the explicit remaining scope.
- Completed T024 by extending non-regression coverage in `tests/policy_and_traces.rs`, `tests/integration/refactor_preservation_run.rs`, and `tests/direct_runtime_coverage.rs` so canonical authored fixtures and recommendation-only posture remain explicit across trace, blocked-run, and engine-direct surfaces.
- Completed T025 by updating `docs/guides/modes.md`, `ROADMAP.md`, and confirming `AGENTS.md` already reflected the feature 019 plan context without further edits.
- Completed T026 by updating the shared execution and preservation fixtures in `tests/policy_and_traces.rs`, `tests/integration/refactor_preservation_run.rs`, and `tests/direct_runtime_coverage.rs` to use canonical H2 authored briefs.
- Completed T027 by recording rollout, roadmap, and non-regression evidence here and in `decision-log.md`.
- Synchronized the release surfaces by bumping the workspace version to `0.19.0`, updating `CHANGELOG.md`, and updating the shared runtime-compatibility references used by the embedded and mirrored skills.

### Focused Rollout Evidence

- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test mode_authoring_follow_on_docs`
	- Result: 2 passed, 0 failed after synchronizing `ROADMAP.md` and `docs/guides/modes.md` with the delivered follow-on slice.
- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test policy_and_traces --test refactor_preservation_run --test direct_runtime_coverage`
	- Result: 25 passed, 0 failed after converting the shared execution and preservation fixtures to canonical H2 authored briefs and reaffirming recommendation-only posture.

## Planned Structural Validation

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `/bin/bash scripts/validate-canon-skills.sh`

## Planned Logical Validation

- Focused contract tests for the `system-shaping`, `implementation`, and `refactor` artifact contracts
- Focused renderer tests proving verbatim authored-body preservation for each targeted mode
- Focused negative renderer or run tests proving `## Missing Authored Body` for absent or blank required sections
- Focused near-match heading tests proving canonical-heading enforcement, such as `Rollback Plan` not satisfying `Rollback Steps`
- Focused run tests proving the targeted packets emit correctly while `implementation` and `refactor` posture remains unchanged
- Docs-sync tests covering embedded skills, mirrored `.agents` skill files, templates, examples, and mode-guide wording for the targeted contract surfaces
- Focused non-regression checks for already-specialized reference modes and for unaffected execution semantics

## Planned Independent Validation

- Review `spec.md`, `plan.md`, and `tasks.md` before implementation begins
- Walk one realistic positive brief through each targeted mode and compare the authored source with the emitted packet artifacts
- Walk one negative brief per targeted mode with a required heading removed and verify the emitted missing-body marker names the canonical heading
- Perform an independent artifact review confirming roadmap and guide language do not overstate rollout completion

## Evidence To Capture

- Structural validation command output
- Focused targeted test command output
- Skill validation output
- Notes from positive and negative authored-brief walkthroughs
- Any deviations, compatibility findings, or follow-on work captured in `decision-log.md`

## Validation Ownership Split

- Generation changes: artifact contracts, renderer logic, orchestrator handoff, skill sources, mirrored skills, templates, examples, guide text, roadmap text, and focused tests
- Validation owners: structural commands, focused logical test runs, skill validator, and independent artifact review recorded separately from generation edits

## Baseline Risks To Watch

- `implementation` and `refactor` renderers currently depend on evidence-mixed summaries, which can hide authored H2 sections from extraction.
- `system-shaping` already partially uses authored preservation, so validation must prove the slice extends existing behavior without regressing `domain-model.md`.
- Docs drift across embedded skills, mirrored skills, templates, and examples is a direct feature failure for this slice.

## Final Structural Validation Results

- `cargo fmt --check`
	- Result: PASS after applying repository-standard formatting to the touched Rust test and renderer files.
- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo clippy --workspace --all-targets --all-features -- -D warnings`
	- Result: PASS.
- `/bin/bash scripts/validate-canon-skills.sh`
	- Result: PASS. Canon skill structure, support-state labels, overlap boundaries, and fake-run protections remain valid.

## Final Logical Validation Results

- `CARGO_TARGET_DIR=/Users/rt/workspace/apply-the/canon/target-agent cargo test --test system_shaping_authoring_renderer --test implementation_authoring_renderer --test refactor_authoring_renderer --test system_shaping_contract --test implementation_contract --test refactor_contract --test system_shaping_run --test implementation_run --test refactor_run --test system_shaping_domain_modeling_docs --test implementation_authoring_docs --test refactor_authoring_docs --test mode_authoring_follow_on_docs --test policy_and_traces --test refactor_preservation_run --test direct_runtime_coverage`
	- Result: 60 passed, 0 failed.
	- Coverage summary: renderer preservation and near-miss rejection, contract sections and gates, complete and incomplete run behavior, docs synchronization, trace posture, preservation blocking, and engine-direct fixtures all passed under the canonical H2 contract.

## Independent Review Findings

- Reviewed `spec.md`, `plan.md`, `tasks.md`, and `quickstart.md` against the delivered implementation and validation evidence.
- Result: no unresolved contradictions or requirement gaps remain after updating `quickstart.md` to state that incomplete targeted packets remain gate-blocked.
- Residual follow-on scope remains explicit and bounded to `review`, `verification`, `incident`, and `migration`.

## Closeout Confirmation

- Recommendation-only posture for `implementation` and `refactor` remains unchanged across run, status, trace, and direct-runtime surfaces.
- `system-shaping`, `implementation`, and `refactor` now share canonical authored H2 contracts across runtime, docs, skills, templates, examples, and validation.
- Missing canonical authored sections now produce both explicit `## Missing Authored Body` markers and honest blocked-gate outcomes where the packet is incomplete.
- Non-target modes remained behaviorally stable in the focused direct-runtime and trace validation surfaces exercised during closeout.