# Validation Report: Governance Runtime Framing

## Structural Validation

- `cargo fmt --check`: passed after one `cargo fmt` normalization pass on the new Rust guardrail tests.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed on the final branch state.
- Focused Rust guardrail tests: `cargo test --test governance_runtime_framing_docs` passed.
- Focused Rust guardrail tests: `cargo test --test release_040_governance_runtime_framing` passed.
- Focused Rust guardrail tests: `cargo test --test release_036_release_provenance_integrity` passed.
- Focused Rust guardrail tests: `cargo test --test skills_bootstrap skills_install_for_codex_carries_current_runtime_compatibility_reference` passed.
- Focused Rust guardrail tests: `cargo test --test governance_cli governance_capabilities_reports_v1_machine_contract` passed.
- `cargo nextest run`: passed on the final branch state with `348` tests run and `348` passed.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`: passed on the final branch state.

## Logical Validation

- README and getting-started now describe Canon as the governed packet runtime for AI-assisted engineering, keep the non-goal around generic agent-framework framing explicit, and insert `inspect clarity` into the human happy path before `run`.
- The new governance adapter guide documents the three adapter commands, stable response fields, the same-runtime boundary, and the current explicit `pr-review` limitation rather than fabricating unsupported request semantics.
- Release surfaces now align on `0.40.0` across `Cargo.toml`, `Cargo.lock`, runtime compatibility references, `README.md`, `CHANGELOG.md`, `ROADMAP.md`, and the publishing guides checked by the existing release provenance guardrail.
- The CLI machine-facing surface now reinforces the same boundary through `compatibility_notes` in `crates/canon-cli/src/commands/governance.rs`, and the contract test locks that wording.

## Independent Validation

- Manual readback against `spec.md` confirmed that Canon remains framed as the governed runtime rather than the higher-level orchestrator.
- Manual readback confirmed that no Synod-specific stage mapping leaked into Canon core docs.
- The adapter guide explicitly names the current `pr-review` boundary, which keeps the documentation honest relative to the current `v1` request envelope.

## Coverage Evidence

- Final workspace `lcov.info` exports the touched source file `crates/canon-cli/src/commands/governance.rs` at `1389/1451` covered lines, which is `95.73%` line coverage.
- The other touched Rust files in this feature are integration or contract test crates under `tests/`. The workspace `lcov.info` export does not emit them as `SF:` entries, so file-level percentages are not available from the final workspace report.
- To compensate, a focused `cargo llvm-cov --workspace --all-features --test governance_runtime_framing_docs --test release_040_governance_runtime_framing --test release_036_release_provenance_integrity --test skills_bootstrap --lcov --output-path lcov-touched-tests.info` run was executed successfully. It exercised the touched test targets directly but still produced an empty `lcov-touched-tests.info`, confirming the exporter limitation rather than a missing execution path.

## File Diff Summary

- Public framing: `README.md`, `docs/guides/getting-started.md`, and `docs/guides/modes.md`
- New integration surface: `docs/integration/governance-adapter.md`
- Release alignment: `Cargo.toml`, `Cargo.lock`, runtime compatibility references, `CHANGELOG.md`, `ROADMAP.md`, `docs/guides/publishing-to-winget.md`, and `docs/guides/publishing-to-scoop.md`
- Rust guardrails: `tests/governance_runtime_framing_docs.rs`, `tests/release_040_governance_runtime_framing.rs`, `tests/release_036_release_provenance_integrity.rs`, `tests/integration/skills_bootstrap.rs`, `tests/contract/governance_cli.rs`, and `crates/canon-cli/src/commands/governance.rs`

## Final Closeout

- All planned tasks in `tasks.md` are complete.
- The feature invariants still hold: Canon remains local-first and governed, the human CLI and governance adapter are documented as one runtime, and no governance adapter schema or lifecycle semantics were changed.
- Proposed commit message: `feat: deliver governance runtime framing as Canon 0.40.0`