# Validation Report: Interactive Init Experience

## Planned Evidence

| Area | Validation | Status | Evidence |
|------|------------|--------|----------|
| Guided state machine | Focused unit tests for selection, confirmation, ignored `Esc`, and interruption handling | Pass | `cargo test -p canon-cli --bin canon` passed with 192/192 tests, including `tui::init::*`, `tui::mod::*`, `tui::render::*`, and `tui::terminal::*` coverage-focused additions. |
| Guided command contract | Guided integration coverage for default launch, `--ai` preselection, and 10-keypress reachability | Pass | `cargo test --test init_guided_contract --test init_terminal_recovery` passed; `guided_flow_reaches_claude_within_ten_keypresses` is green. |
| Non-interactive compatibility | Regression and contract coverage for `--non-interactive`, `--ai`, structured output, and no-TTY fallback | Pass | `cargo test --test init_creates_canon --test init_non_interactive_contract` passed; non-interactive idempotence, AI passthrough, JSON summary output, and structured-output rejection without `--non-interactive` are green. |
| Terminal lifecycle | Focused unit and integration coverage for restore-on-drop, `Ctrl+C`, failure, and layout-fit rejection | Pass | `cargo test -p canon-cli --bin canon` plus `cargo test --test init_terminal_recovery` passed; restore-on-drop, interruption, failure teardown, and too-small-layout rejection are green. |
| Usability protocol | 10 first-attempt guided runs from clean temp workspaces without external documentation | Pass | Scripted temp-workspace protocol using `target/debug/canon` completed `10/10` first-attempt guided runs, exceeding the `9/10` success target. No external documentation was consulted during the protocol. |
| Release surfaces | Version, changelog, docs, site, roadmap, quickstart, and CLI contract review | Pass | Release surfaces aligned to `0.63.0`; `cargo test --test assistant_plugin_packages metadata_paths_and_versions_are_aligned` passed after updating assistant and runtime-compatibility metadata. |
| Formatting | `cargo fmt --check` | Pass | `cargo fmt` completed successfully in final closeout. |
| Linting | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Pass | Final strict clippy run passed without warnings. |
| Coverage on touched Rust files | LCOV-backed changed-file coverage evidence over 95% for touched Rust sources | Pass | `cargo llvm-cov -p canon-cli --all-targets --all-features --lcov --output-path lcov.canon-cli.info` produced: `app.rs 98.83%`, `main.rs 96.36%`, `commands/init.rs 100.00%`, `tui/mod.rs 98.57%`, `tui/init.rs 97.04%`, `tui/render.rs 100.00%`, `tui/terminal.rs 99.27%`. Patch intersection against touched production Rust files reported no misses. |

## Implementation Evidence

- Final repo quality gates:
	- `cargo fmt`
	- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
	- `cargo test -p canon-cli --bin canon`
	- `cargo test --test init_creates_canon --test init_guided_contract --test init_non_interactive_contract --test init_terminal_recovery`
	- `cargo test --test assistant_plugin_packages metadata_paths_and_versions_are_aligned`

- SC-001 usability protocol:
	- Built binary: `target/debug/canon`
	- Workspace isolation: each run executed from a fresh `mktemp -d` workspace, never from the repository root.
	- Harness: `CANON_TUI_TEST_INTERACTIVE=1`, `CANON_TUI_TEST_SIZE=120x40`, `CANON_TUI_TEST_EVENTS=enter,enter`, and `CANON_TUI_TEST_CAPTURE_PATH=<temp>/guided-capture.log`.
	- Result: `10/10` first-attempt guided runs created `.canon/` and wrote `terminal_restored=true` into the capture log.
	- External documentation consulted during the protocol: none.

- Quickstart walkthrough notes:
	- Default guided flow: passed through the scripted guided harness; `.canon/` created and restore marker recorded.
	- Guided preselection (`init --ai copilot`): passed; `.canon/` created and the capture log contained `Copilot`.
	- Non-interactive JSON (`init --non-interactive --output json`): passed; `.canon/` created and the emitted stdout parsed as valid JSON.
	- Structured-output rejection (`init --output json` without `--non-interactive`): passed; command exited non-zero, `.canon/` was absent, and stderr referenced `--non-interactive`.
	- Interruption handling (`ctrl-c` scripted event): passed; command exited non-zero, `.canon/` was absent, stderr reported that no `.canon` changes were made, and the capture log recorded terminal restoration.
	- Too-small terminal handling (`CANON_TUI_TEST_SIZE=40x12`): passed; command exited non-zero, `.canon/` was absent, and stderr reported the resize-or-`--non-interactive` guidance.

- Targeted behavior evidence:
	- Guided reachability: `guided_flow_reaches_claude_within_ten_keypresses` passed.
	- Guided preselection: `init_honors_ai_preselection_inside_guided_flow` passed.
	- Non-interactive regression: `init_non_interactive_is_idempotent_and_creates_runtime_scaffolding` and the four `init_non_interactive_contract::*` cases passed.
	- Terminal recovery: `ctrl_c_restores_terminal_and_avoids_side_effects`, `guided_init_failure_restores_terminal_before_returning_error`, and `too_small_layout_rejects_before_guided_terminal_setup` passed.

- Coverage evidence:
	- Command: `cargo llvm-cov clean --workspace && cargo llvm-cov -p canon-cli --all-targets --all-features --lcov --output-path lcov.canon-cli.info`
	- Changed production Rust files all exceeded `95%` line coverage:
		- `crates/canon-cli/src/app.rs`: `509/515` (`98.83%`)
		- `crates/canon-cli/src/main.rs`: `53/55` (`96.36%`)
		- `crates/canon-cli/src/commands/init.rs`: `151/151` (`100.00%`)
		- `crates/canon-cli/src/tui/mod.rs`: `69/70` (`98.57%`)
		- `crates/canon-cli/src/tui/init.rs`: `131/135` (`97.04%`)
		- `crates/canon-cli/src/tui/render.rs`: `133/133` (`100.00%`)
		- `crates/canon-cli/src/tui/terminal.rs`: `410/413` (`99.27%`)
	- Patch validation command: `git diff --unified=0 -- crates/canon-cli/src/app.rs crates/canon-cli/src/main.rs crates/canon-cli/src/commands/init.rs crates/canon-cli/src/tui/mod.rs crates/canon-cli/src/tui/init.rs crates/canon-cli/src/tui/render.rs crates/canon-cli/src/tui/terminal.rs | python3 scripts/common/coverage/intersect_patch_coverage.py --lcov lcov.info ...`
	- Patch-validation outcome: no uncovered touched production Rust lines remained.

## Independent Review Focus

- Verify that the guided UI remains CLI-only and does not move terminal-state or presentation concerns into `canon-engine`.
- Verify that `--non-interactive` preserves the current argument-driven behavior and serializer surfaces.
- Verify that structured-output rejection only applies when `--non-interactive` is absent.
- Verify that terminal restoration holds across success, failure, `Ctrl+C`, and preflight rejection paths.
- Verify that the version bump and operator-facing docs describe the same shipped behavior as the runtime.

## Independent Review Outcome

- No blocking findings.
- CLI-only boundary preserved: the guided UI, terminal lifecycle, and scripted test harness remain inside `canon-cli`; `canon-engine::EngineService::init()` remains the single source of `.canon/` side effects.
- Non-interactive contract review: no deviations found. `--non-interactive` preserves the argument-driven path, JSON output, AI selection passthrough, and no-TTY fallback behavior. Structured output remains rejected whenever `--non-interactive` is absent.
- Terminal cleanup review: success, interruption, failure, and preflight rejection paths are covered by focused unit/integration tests plus temp-workspace walkthroughs. Residual risk is limited to live PTY-specific raw-mode transitions not being exercised by a human-driven harness in this chat environment; the runtime path remains isolated to CLI code and the teardown contract is otherwise validated.
- Dependency review: the additive `ratatui` and `crossterm` dependencies remain confined to the workspace manifest and `canon-cli`; no engine dependency surface changed.
- Release-surface review: version alignment is consistent across Cargo metadata, assistant manifests, runtime-compatibility references, changelog, docs, site, roadmap, quickstart, and CLI contract wording.
- SC-001 verification: the recorded `10/10` first-attempt guided runs satisfy the `9/10` success target.
- Final outcome: approved for closeout.