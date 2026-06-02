# Validation Report: Mode Publish Alignment

## Status

- Structural validation: Complete
- Logical validation: Complete
- Independent validation: Complete
- Coverage closeout: Complete

## Structural Validation Results

- `cargo fmt --check` passed after formatting the new runtime and test assertions.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed on the final `0.45.0` tree.
- Focused version-governed validation passed through:
	- `cargo test --test skills_bootstrap`
	- release-surface `rg` checks confirming no stale `0.44.0` references remained in touched version-governed files outside historical changelog entries

## Logical Validation Results

- Pre-fix failing checks confirmed the real drift:
	- `cargo test --test security_assessment_direct_runtime security_assessment_direct_run_exercises_service_summary_and_publish_paths -- --exact` failed because `AwaitingApproval` `security-assessment` packets were rejected by `publish`.
	- `cargo test --test assistant_plugin_packages publish_command_surfaces_match_the_positional_cli_contract -- --exact` failed because assistant metadata still used `canon publish --run <RUN_ID>`.
- Post-fix focused checks passed:
	- `cargo test --test security_assessment_direct_runtime`
	- `cargo test --test assistant_plugin_packages`
	- `cargo test --test skills_bootstrap`
- Final regression closeout:
	- `cargo nextest run --workspace --all-features` completed successfully on the final tree after the slice landed.

## Independent Validation Results

- Readback against `tech-docs/guides/modes.md` confirmed the runtime now matches the documented operational publish posture for readable `security-assessment` packets in `AwaitingApproval` and `Blocked` states.
- Readback against assistant package surfaces confirmed `assistant/commands/governed-methods.json` and `assistant/prompts/copilot-command-pack.md` both now use positional `canon publish <RUN_ID>` syntax, matching the CLI contract.
- Readback of touched version-governed surfaces confirmed the active release line is `0.45.0` across workspace manifests, assistant package metadata, runtime compatibility references, README, tests, and the new changelog entries.

## Coverage Closeout

- Initial `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` left `crates/canon-engine/src/orchestrator/publish.rs` at `774/822 = 94.2%` whole-file coverage.
- Following the repository guidance for suspicious one-off coverage gaps, `cargo llvm-cov clean --workspace` was run before repeating the exact coverage command.
- Final touched-file coverage after the clean rerun:
	- `crates/canon-engine/src/orchestrator/publish.rs`: `797/836 = 95.3%`
	- `tests/security_assessment_direct_runtime.rs`: not materialized in `lcov.info` (test source file, treated as not-applicable)
	- `tests/assistant_plugin_packages.rs`: not materialized in `lcov.info` (test source file, treated as not-applicable)
	- `tests/integration/skills_bootstrap.rs`: not materialized in `lcov.info` (test source file, treated as not-applicable)
- Coverage requirement satisfied for the modified Rust source file.

## Notes

- No additional publish destination, PRD, C4, or ADR behavior changed in this slice.
- The final branch is ready for commit and review.