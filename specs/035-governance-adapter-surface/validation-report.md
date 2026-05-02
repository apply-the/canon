# Validation Report: Governance Adapter Surface For External Orchestrators

## Status

Completed

- Feature 035 is closed on one `0.35.0` machine-facing compatibility story
  across code, contract docs, release surfaces, and validation evidence.
- Post-Synod closeout is complete, including the refresh corruption fix and
  the `missing_fields` versus `missing_sections` contract split.
- Touched Rust source files in this slice exceed the 95% line-coverage gate:
  `crates/canon-cli/src/app.rs` 99.17%,
  `crates/canon-cli/src/commands.rs` 100.00%, and
  `crates/canon-cli/src/commands/governance.rs` 96.79%.

## Structural Validation

- Confirmed `spec.md`, `plan.md`, `tasks.md`, `research.md`, `data-model.md`,
  `decision-log.md`, and `contracts/governance-adapter-contract.md` agree on
  the flat `v1` governance adapter contract, additive compatibility rules,
  `missing_fields`, packet-only `missing_sections`, and refresh corruption
  failure semantics.
- Confirmed release-facing version anchors report `0.35.0` consistently in
  manifests, runtime-compatibility references, docs, roadmap text, and
  `CHANGELOG.md`.
- Confirmed `.canon/` layout, publish semantics, approval targets, and
  existing human-oriented CLI behavior remain unchanged; the implementation
  stays bounded to `canon-cli` and existing public engine store surfaces.
- Ran `cargo fmt` after the final governance changes, then
  `cargo fmt --check` with no diff.
- Ran `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  passed.

## Logical Validation

- Ran `cargo test --test governance_cli --test governance_adapter_surface`:
  8 passed, 0 failed.
- Ran `cargo test --test skills_bootstrap
  skills_install_for_codex_carries_current_runtime_compatibility_reference`:
  1 passed, 0 failed.
- Verified blocked request-precondition outcomes now surface
  `missing_fields` and do not reuse `missing_sections`.
- Verified `governed_ready` appears only with reusable packet state and
  canonical workspace-relative refs.
- Verified refresh returns `status: failed` with
  `reason_code: artifact_contract_unreadable` when
  `artifact-contract.toml` is unreadable.
- Added focused in-process guard coverage for unreadable and missing artifact
  contract scenarios in `crates/canon-cli/src/commands/governance.rs` tests.
- Ran `cargo nextest run`: 319 tests run, 319 passed, 0 skipped.

## Coverage Evidence

- Produced `lcov.info` via
  `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`.
- Final touched Rust source-file coverage:
  - `crates/canon-cli/src/app.rs`: 477/481 lines, 99.17%
  - `crates/canon-cli/src/commands.rs`: 11/11 lines, 100.00%
  - `crates/canon-cli/src/commands/governance.rs`: 1359/1404 lines, 96.79%
- No touched Rust source file remained at or below the 95% blocking threshold.

## Independent Validation

- Ran one live consumer-driven smoke in an initialized temporary Canon
  workspace via
  `cargo run --manifest-path /Users/rt/workspace/apply-the/canon/crates/canon-cli/Cargo.toml --bin canon -- governance ...`.
- Smoke result: a supported requirements start request returned
  `status: governed_ready` with reusable packet refs; after overwriting the
  run's `artifact-contract.toml` with invalid TOML, refresh returned
  `status: failed` and `reason_code: artifact_contract_unreadable`.
- Reviewed blocked, failed, and approval-gated paths against Synod feedback to
  confirm consumers can branch on `status`, `approval_state`, `reason_code`,
  `missing_fields`, and packet-readiness fields without parsing prose.
- Recorded and fixed a validation hazard in the test harness: governance
  end-to-end helpers must not fall back to `target/debug/canon` because a
  stale binary can mask current-source regressions.
- Reviewed release-facing docs and runtime-compatibility references after the
  contract update to confirm the public `0.35.0` story is coherent outside the
  code diff.

## Evidence Paths

- `specs/035-governance-adapter-surface/spec.md`
- `specs/035-governance-adapter-surface/plan.md`
- `specs/035-governance-adapter-surface/research.md`
- `specs/035-governance-adapter-surface/data-model.md`
- `specs/035-governance-adapter-surface/contracts/`
- `specs/035-governance-adapter-surface/decision-log.md`
- `specs/035-governance-adapter-surface/tasks.md`
- `specs/035-governance-adapter-surface/validation-report.md`
- `tests/contract/governance_cli.rs`
- `tests/integration/governance_adapter_surface.rs`
- `tests/integration/skills_bootstrap.rs`
- `lcov.info`
- `cargo fmt --check` output
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  output
- `cargo nextest run` output
- independent consumer-driven smoke evidence captured from the temporary
  workspace repro