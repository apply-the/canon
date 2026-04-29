# Validation Report: Supply Chain And Legacy Analysis Mode

## Validation Summary

- **Feature**: `024-supply-chain-legacy`
- **Release Target**: `0.24.0`
- **Status**: In Progress (final coverage gate pending)
- **Validation owner**: feature implementation plus separate review pass

## Structural Validation Plan

| Check | Purpose | Planned Evidence | Status |
|------|---------|------------------|--------|
| Version-surface sync | Ensure `0.24.0` is reported consistently | Cargo manifests, shared runtime compatibility references, release-facing docs | Passed |
| Skill validation | Ensure shared and mirrored skills stay aligned | `/bin/bash scripts/validate-canon-skills.sh` output | Passed |
| Formatting | Ensure repository formatting remains clean | `cargo fmt --check` output | Passed |
| Linting | Ensure no unresolved workspace warnings | `cargo clippy --workspace --all-targets --all-features -- -D warnings` output | Passed |

## Logical Validation Plan

| Check | Purpose | Planned Evidence | Status |
|------|---------|------------------|--------|
| Contract test | Validate artifact family and section rules | focused `cargo test --test supply_chain_analysis_contract` | Passed |
| Renderer test | Validate authored preservation and explicit gap markers | `cargo test --test supply_chain_analysis_authoring_renderer` | Passed |
| Direct runtime test | Cover `EngineService` orchestration, summaries, and publishability | `cargo test --test supply_chain_analysis_direct_runtime --test supply_chain_analysis_run` | Passed |
| Publish test | Validate docs publish destination and artifact set | `cargo test --test supply_chain_analysis_run --test release_024_docs` | Passed |
| Shared regression checks | Verify mode discovery and skill materialization | inspect-modes, mode-profiles, init/materialization tests | Passed |

## Coverage Goal

- Every Rust source file added or modified by this feature must show at least
  85% line coverage in the recorded coverage evidence.
- Coverage evidence must favor direct `EngineService` and adapter-oriented tests
  over CLI-only subprocess coverage where library code is involved.

## Independent Validation Plan

- Review the authored packet for one positive-path repository surface.
- Review one negative-path run where a required scanner or posture decision is
  missing and confirm the packet exposes an explicit gap marker.
- Re-check release-facing version surfaces and the final task ordering in
  `tasks.md` before implementation closeout.

## Evidence Log

| Evidence Item | Path Or Command | Result | Notes |
|--------------|------------------|--------|-------|
| Specification review | `specs/024-supply-chain-legacy/spec.md` | Planned | Confirm no unresolved clarification markers |
| Plan review | `specs/024-supply-chain-legacy/plan.md` | Planned | Confirm constitution alignment |
| Task review | `specs/024-supply-chain-legacy/tasks.md` | Planned | Confirm first and last task constraints |
| Coverage evidence | coverage command and recorded output | Planned | Must show >=85% line coverage per touched Rust file |
| Workspace closeout | fmt, clippy, focused tests, selected regression checks | Planned | Record final pass set here |

## Executed Evidence

| Evidence Item | Path Or Command | Result | Notes |
|--------------|------------------|--------|-------|
| Skill validation | `/bin/bash scripts/validate-canon-skills.sh` | Passed | Includes `canon-supply-chain-analysis` available-now validation and runtime-hint checks |
| Focused supply-chain + shared regression suite | `cargo test --test supply_chain_analysis_contract --test supply_chain_analysis_authoring_renderer --test supply_chain_analysis_direct_runtime --test supply_chain_analysis_run --test inspect_clarity --test release_024_docs --test inspect_modes --test mode_profiles --test init_creates_canon --test direct_runtime_coverage --test skills_bootstrap` | Passed | 58 tests across targeted binaries passed in this run |
| Added authoring/docs coverage | `cargo test --test supply_chain_analysis_authoring_docs --test skills_bootstrap` | Passed | Confirms docs+template+example sections and skill source/mirror parity |
| Formatting and lint gates | `cargo fmt && cargo fmt --check && cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed | No clippy warnings/errors after feature updates |
| Coverage export (focused) | `cargo llvm-cov --workspace --json --summary-only --output-path target/llvm-cov/summary-024.json --test ...` | Passed | Export produced machine-readable coverage summary |
| Coverage export (broad) | `cargo llvm-cov --workspace --all-targets --ignore-run-fail --json --summary-only --output-path target/llvm-cov/summary-024-full.json` | Completed with tolerated legacy failure | Legacy test `release_021_docs` failed on historical version assertion; report still emitted |

## Coverage Gate Snapshot (T037)

Coverage source: `target/llvm-cov/summary-024-full.json` (line coverage percent for modified Rust files).

| Rust File | Line Coverage | Meets 85% |
|-----------|---------------|-----------|
| `crates/canon-engine/src/artifacts/contract.rs` | 93.35% | Yes |
| `crates/canon-engine/src/artifacts/markdown.rs` | 93.43% | Yes |
| `crates/canon-engine/src/domain/mode.rs` | 99.02% | Yes |
| `crates/canon-engine/src/orchestrator/classifier.rs` | 71.53% | No |
| `crates/canon-engine/src/orchestrator/gatekeeper.rs` | 88.40% | Yes |
| `crates/canon-engine/src/orchestrator/publish.rs` | 81.76% | No |
| `crates/canon-engine/src/orchestrator/service.rs` | 79.45% | No |
| `crates/canon-engine/src/orchestrator/service/clarity.rs` | 56.67% | No |
| `crates/canon-engine/src/orchestrator/service/execution.rs` | 100.00% | Yes |
| `crates/canon-engine/src/orchestrator/service/inspect.rs` | 67.11% | No |
| `crates/canon-engine/src/orchestrator/service/summarizers.rs` | 92.49% | Yes |
| `crates/canon-engine/src/persistence/store.rs` | 91.28% | Yes |

Current closeout status:

- Recommendation-only posture, packet honesty, release docs, and validators are validated.
- `T037` remains open until shared orchestrator files below 85% line coverage are raised or the coverage gate definition is explicitly narrowed to feature-diff scope.