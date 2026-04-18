# Validation Report: Analysis Mode Expansion

**Feature**: 006-analysis-expansion  
**Date**: 2026-04-14  
**Status**: Completed

## Structural Validation

| Check | Target | Method | Status |
|-------|--------|--------|--------|
| SV-01 | Artifact contracts compile | `cargo check --workspace` — new match arms in `contract_for_mode()` | Pass |
| SV-02 | Gate evaluation functions compile | `cargo check --workspace` — new functions in `gatekeeper.rs` | Pass |
| SV-03 | Mode dispatch compiles | `cargo check --workspace` — new match arms in `service.rs run()` | Pass |
| SV-04 | Mode constants defined | `cargo check --workspace` — STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES | Pass |
| SV-05 | ModeProfile updates consistent | `cargo test` — `all_mode_profiles()` returns `Full` depth for discovery, system-shaping, architecture | Pass |
| SV-06 | Methods TOML updated | Manual cross-check — `implementation_depth = "full"`, spec-authoritative artifact names | Pass |
| SV-07 | Clippy clean | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Pass |

## Logical Validation

| Check | Target | Method | Status |
|-------|--------|--------|--------|
| LV-01 | Discovery end-to-end run | `tests/discovery_run.rs` | Pass |
| LV-02 | System-Shaping end-to-end run | `tests/greenfield_run.rs` | Pass |
| LV-03 | Architecture end-to-end run | `tests/architecture_run.rs` | Pass |
| LV-04 | Discovery gate blocking | `tests/discovery_contract.rs` — missing `context-boundary.md` blocks Exploration | Pass |
| LV-05 | System-Shaping gate blocking | `tests/greenfield_contract.rs` and `tests/greenfield_run.rs` — missing architecture evidence or insufficient context blocks | Pass |
| LV-06 | Architecture gate blocking | `tests/architecture_contract.rs` — missing `tradeoff-matrix.md` blocks Architecture | Pass |
| LV-07 | SystemicImpact approval | `tests/architecture_contract.rs`, `tests/architecture_run.rs`, `tests/discovery_contract.rs` | Pass |
| LV-08 | Artifact section validation | `crates/canon-engine/src/artifacts/markdown.rs` unit test plus contract-backed gate tests | Pass |
| LV-09 | Evidence bundle linkage | `tests/discovery_run.rs`, `tests/greenfield_contract.rs`, `tests/pr_review_evidence_contract.rs` regression sweep | Pass |
| LV-10 | Discovery without critique | `tests/discovery_contract.rs` and `tests/discovery_run.rs` — only 2 governed invocations persisted | Pass |
| LV-11 | System-Shaping with critique | `tests/greenfield_contract.rs` and `tests/greenfield_run.rs` | Pass |

## Independent Validation

| Check | Target | Method | Status |
|-------|--------|--------|--------|
| IV-01 | Artifact contracts match spec | Manual review plus direct contract tests for all 3 modes | Pass |
| IV-02 | Gate profiles match spec | Manual review of `mode.rs`, `gatekeeper.rs`, and mode constants | Pass |
| IV-03 | Persistence layout correct | Integration runs persist `.canon/runs/<id>/` and `.canon/artifacts/<id>/...` for all 3 modes | Pass |
| IV-04 | Inspection surfaces work | `inspect artifacts`, `inspect evidence`, and `inspect invocations` exercised by new tests | Pass |
| IV-05 | Embedded skills truthful | `scripts/validate-canon-skills.sh` and `pwsh -File scripts/validate-canon-skills.ps1` | Pass |

## Consistency Checks

| Check | Target | Method | Status |
|-------|--------|--------|--------|
| CC-01 | Mode profiles ↔ contracts | Manual cross-check of artifact families against `contract_for_mode()` | Pass |
| CC-02 | Methods TOML ↔ contracts | Manual cross-check of `defaults/methods/*.toml` against `contract_for_mode()` | Pass |
| CC-03 | Gate profiles ↔ gatekeeper | Manual cross-check of `mode.rs`, `modes/*.rs`, and `gatekeeper.rs` | Pass |
| CC-04 | Spec artifacts ↔ contract.rs | Manual review of all 15 new artifact names against spec tables | Pass |
| CC-05 | STEP_SEQUENCE ↔ orchestration | Manual review of `service.rs` against mode sequence declarations | Pass |

## Execution Evidence

- `cargo check --workspace`: passed
- `cargo fmt --check`: passed after applying `cargo fmt`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed
- `cargo test`: passed, including new direct runtime coverage assertions for `EngineService`, artifact contracts, and shell adapter worktree flows
- `cargo nextest run`: passed, 80/80 tests green
- `scripts/validate-canon-skills.sh`: passed
- `pwsh -File scripts/validate-canon-skills.ps1`: passed
- `cargo deny check licenses advisories bans sources`: passed
- `cargo llvm-cov --workspace --all-features --json --output-path target/llvm-cov-summary.json`: completed, overall line coverage `91.95%`

## Artifact Review Notes

- Discovery sample artifact review showed `problem-map.md` retaining the supplied problem statement and constraints in the summary and boundary sections.
- System-shaping sample artifact review showed `system-shape.md` carrying explicit `Intent` and `Constraint` anchors into emitted content, and insufficient-evidence runs now block instead of silently passing.
- Architecture sample artifact review showed `architecture-decisions.md` carrying decision-focus and constraint context through generated decisions and critique-backed tradeoffs.

## Open Items

- None. Coverage now exceeds the `>= 85%` target, and post-change regression sweeps remained green for `requirements`, `brownfield-change`, and `pr-review`.

## Evidence Artifacts

- Feature validation notes: `specs/006-analysis-expansion/validation-report.md` (this file, updated after execution)
- Runtime evidence: `.canon/artifacts/<RUN_ID>/{discovery|system-shaping|architecture}/`
- Test results: `cargo test` and `cargo nextest run` output
- Coverage report: `target/llvm-cov-summary.json`
