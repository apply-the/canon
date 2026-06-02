# Validation Report: Artifact Indexing Contract

## Status

- **Implementation status**: complete
- **Independent review**: complete
- **Coverage closeout**: complete

## Executed Validation

### 2026-05-17

- `cargo fmt --all`
  Result: passed
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  Result: passed
- `cargo test --no-run --all-targets`
  Result: passed
- `cargo nextest run --workspace --all-features --status-level fail --success-output never`
  Result: passed
  Notes: completed the full workspace suite with `958 / 958` tests passing.
- `cargo test -p canon-cli commands::governance::projection::tests`
  Result: passed
  Notes: validated governance projection fallback behavior, packet metadata
  loading, artifact ordering, and the new `RuntimePacketMetadata` fields on the
  CLI side.
- `cargo test -p canon-engine artifact_indexing_metadata_validate_rejects_misaligned_discovery_rule`
  Result: passed
- `cargo test -p canon-engine indexable_artifact_class_for_publication_rejects_pending_append_only_mapping`
  Result: passed
- `cargo test -p canon-engine artifact_indexing_metadata_maps_index_surfaces_from_publication`
  Result: passed
- `cargo test -p canon-engine engine_service_public_wrappers_cover_runtime_and_skill_entrypoints`
  Result: passed
  Notes: raised `service.rs` above the modified-file coverage threshold by exercising the public wrapper entrypoints and approval-state branches.
- `cargo test --test publish_runtime publish_run_with_profile_promotes_completed_requirements`
  Result: passed
- `cargo test --test assistant_plugin_packages metadata_paths_and_versions_are_aligned`
  Result: passed
  Notes: confirmed the assistant package manifests stayed aligned with the `0.55.0` version bump.

### Coverage Closeout

- Merged targeted LCOV reports from:
  - `cargo llvm-cov -p canon-engine --lib --lcov --output-path target/canon-engine-lib.lcov`
  - `cargo llvm-cov --workspace --test publish_runtime --lcov --output-path target/publish-runtime.lcov`
  - `cargo llvm-cov --workspace --test domain_analysis_direct_runtime --lcov --output-path target/domain-analysis-direct-runtime.lcov`
  - `cargo llvm-cov -p canon-cli --bin canon --lcov --output-path target/canon-cli-bin.lcov`
- Modified-file coverage:
  - `crates/canon-engine/src/domain/publish_profile.rs`: `96.28%` (`622 / 646`)
  - `crates/canon-engine/src/domain/artifact.rs`: `99.12%` (`225 / 227`)
  - `crates/canon-engine/src/orchestrator/publish.rs`: `95.56%` (`1744 / 1825`)
  - `crates/canon-engine/src/orchestrator/service.rs`: `99.09%` (`217 / 219`)
  - `crates/canon-cli/src/commands/governance/projection.rs`: `95.35%` (`328 / 344`)
- Changed-line coverage: `100%` across all modified production Rust files.

## Independent Review

- Compared the implemented artifact-indexing vocabulary, metadata carrier
  semantics, publish-path guards, and compatibility notes against
  `tech-docs/integration/project-memory-promotion-contract.md`, the feature-local
  derived contract docs, and neighboring Canon artifact-producing specs.
- Findings: no contract drift, no untyped publish-path expansion, and no scope
  creep beyond Canon’s producer-only boundary.