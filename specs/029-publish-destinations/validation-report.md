# Validation Report: Structured External Publish Destinations

## Status

Completed

- Structured default publish destinations, published metadata sidecars, and
  `0.29.0` release-surface alignment are implemented and validated.
- `.canon/` remains runtime and evidence storage only; explicit `--to`
  overrides and existing approval-gated operational publish allowances remain
  unchanged.

## Structural Validation

- Confirm `spec.md`, `plan.md`, `contracts/`, and release-facing docs describe
  the same structured destination and metadata contract.
- Confirm the `0.29.0` version bump is aligned across manifests, lockfile,
  shared runtime compatibility references, and release-facing docs.
- Confirm `.canon/` runtime storage, publish eligibility rules, and explicit
  override behavior remain unchanged by the feature.

### Completed Structural Evidence

- Planning review completed for `spec.md`, `plan.md`, `research.md`,
  `data-model.md`, and the publish contracts.
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, and
  the mirrored runtime-compatibility references now align on `0.29.0`.
- `tests/release_029_publish.rs` passed with 2 assertions covering the shipped
  version surfaces and the structured publish-doc contract.
- `tests/skills_bootstrap.rs` passed with 15 assertions, including the new
  runtime-compatibility mirror check for Codex skills installation.

## Logical Validation

- Run focused publish-path unit tests for destination resolution and metadata
  materialization.
- Run integration tests covering default publish destinations, explicit
  overrides, and approval-gated operational publishing.
- Run release-surface regression checks for `0.29.0` docs and compatibility
  references.
- Run final formatting, linting, coverage, and full-workspace regression
  commands.

### Completed Logical Evidence

- Focused publish suite passed:
  - `cargo test -p canon-engine orchestrator::publish::tests` -> 6 passed.
  - `cargo test -p canon-cli --bin canon commands::publish::tests` -> 3 passed.
  - `cargo test --test runtime_filesystem --test run_lookup --test pr_review_publish --test incident_publish --test migration_publish --test release_029_publish --test skills_bootstrap` -> 32 passed.
- Expanded publish regression passed:
  - `cargo test --test backlog_run --test implementation_run --test system_assessment_run --test security_assessment_run --test refactor_run --test incident_run --test migration_run --test supply_chain_analysis_run --test supply_chain_analysis_direct_runtime --test security_assessment_direct_runtime` -> 28 passed.
  - `cargo test --test requirements_run --test discovery_run --test system_shaping_run --test review_run --test pr_review_run --test architecture_run --test change_run --test verification_run --test refactor_preservation_run` -> 21 passed.
  - `cargo test --test supply_chain_analysis_authoring_docs` -> 3 passed after updating the last stale publish-path doc assertion.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- `cargo nextest run --workspace --all-features` passed with 619/619 tests green.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` passed and emitted `lcov.info`.

### Touched Rust-File Coverage

- Source files with line coverage recorded in `lcov.info`:
  - `crates/canon-engine/src/orchestrator/publish.rs` -> 257/300 lines, 85.67%.
  - `crates/canon-cli/src/commands/publish.rs` -> 83/84 lines, 98.81%.
- Modified test-only Rust files were exercised during the `cargo llvm-cov`
  run but are emitted as `0/0` line counters in workspace `lcov.info`; their
  validation evidence is the passing test-target execution above:
  - `tests/contract/runtime_filesystem.rs`
  - `tests/integration/pr_review_publish.rs`
  - `tests/integration/incident_publish.rs`
  - `tests/integration/migration_publish.rs`
  - `tests/integration/run_lookup.rs`
  - `tests/backlog_run.rs`
  - `tests/integration/implementation_run.rs`
  - `tests/integration/system_assessment_run.rs`
  - `tests/integration/security_assessment_run.rs`
  - `tests/integration/refactor_run.rs`
  - `tests/integration/incident_run.rs`
  - `tests/integration/migration_run.rs`
  - `tests/integration/supply_chain_analysis_run.rs`
  - `tests/supply_chain_analysis_direct_runtime.rs`
  - `tests/security_assessment_direct_runtime.rs`
  - `tests/release_028_docs.rs`
  - `tests/release_029_publish.rs`
  - `tests/integration/skills_bootstrap.rs`
  - `tests/supply_chain_analysis_authoring_docs.rs`

## Independent Validation

- Review the final diff to confirm `.canon/` remains runtime-only and run-id
  traceability remains explicit.
- Review published packet outputs to confirm descriptor-based paths remain
  readable while metadata keeps canonical identity recoverable.
- Review release-facing docs and changelog to confirm the shipped slice is
  described accurately.

### Completed Independent Evidence

- Planning artifact review completed before task generation.
- Final publish diff review confirmed `resolve_destination()` still honors
  explicit absolute and relative overrides before any default-path logic.
- Final publish diff review confirmed the new external contract only changes
  structured default materialization plus `packet-metadata.json`; it does not
  move runtime state out of `.canon/` or redefine canonical run identity.
- Final release-surface review confirmed `README.md`, `ROADMAP.md`,
  `docs/guides/getting-started.md`, `docs/guides/modes.md`, and
  `CHANGELOG.md` describe the shipped `0.29.0` publish contract accurately and
  keep the delivered `028` history visible.

## Evidence Paths

- `specs/029-publish-destinations/decision-log.md`
- `specs/029-publish-destinations/tasks.md`
- `specs/029-publish-destinations/contracts/publish-destination-contract.md`
- `specs/029-publish-destinations/contracts/published-packet-metadata.md`
- `specs/029-publish-destinations/contracts/release-alignment.md`
- `Cargo.toml`
- `Cargo.lock`
- `README.md`
- `docs/guides/getting-started.md`
- `docs/guides/modes.md`
- `ROADMAP.md`
- `CHANGELOG.md`
- focused publish-path test outputs
- `lcov.info`
- full `cargo nextest run` output