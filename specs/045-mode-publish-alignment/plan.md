# Implementation Plan: Mode Publish Alignment

**Branch**: `045-mode-publish-alignment` | **Date**: 2026-05-11 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/045-mode-publish-alignment/spec.md`

## Summary

Align the only confirmed publish-behavior mismatch between runtime and documentation by allowing readable `security-assessment` packets to publish from the same non-`Completed` operational states already documented, fix assistant publish examples that still drift to a synthetic `--run` form, bump the repository release line to `0.45.0`, and close the slice with focused tests plus formatter, linter, nextest, and touched-file coverage evidence.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the slice changes published operator behavior for one existing mode and updates release-governed repository surfaces without altering Canon's broader artifact model  
**Scope In**: `security-assessment` publish-state alignment, assistant publish command-surface correction, `0.45.0` release-line updates, focused tech-docs/tests/release assertions, and validation evidence capture  
**Scope Out**: new projected artifact families, new publish destination roots, new CLI flags, broad mode-policy redesign, and any behavior changes for already aligned PRD/C4/ADR slices

**Invariants**:

- Default publish destinations by mode remain unchanged.
- Requirements `prd.md`, architecture C4 outputs, and ADR projection rules remain unchanged.
- Publish continues to copy from `.canon/artifacts/` into visible destinations without mutating governed runtime artifacts.
- Assistant command guidance stays a documentation/metadata consumer of the CLI contract and does not redefine the CLI.

**Decision Log**: `specs/045-mode-publish-alignment/decision-log.md`  
**Validation Ownership**: Generation happens through repository code and documentation changes; validation happens through focused tests, formatter/linter/test commands, coverage review, and independent doc/runtime readback recorded in the validation report.  
**Approval Gates**: No additional human approval gate beyond normal bounded-impact review; independent validation evidence is still mandatory before merge.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024, plus Markdown/JSON/YAML repository docs and metadata  
**Primary Dependencies**: Existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time` surfaces  
**Storage**: Local filesystem under `.canon/` for runtime artifacts plus repository files under `tech-docs/`, `assistant/`, `.agents/`, `defaults/`, `tests/`, and release metadata surfaces  
**Testing**: Focused `cargo test` targets, `cargo nextest run --workspace --all-features`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`  
**Target Platform**: Local-first CLI on macOS, Linux, Windows, and CI runners  
**Project Type**: Rust workspace CLI and library with repository-published documentation and assistant package metadata  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/publish.rs`, assistant package metadata and prompt pack files, version-governed release surfaces, `README.md`, `tech-docs/guides/modes.md`, and directly affected tests  
**Performance Goals**: No material performance change; publish gate evaluation and assistant metadata validation remain negligible overhead  
**Constraints**: Keep the slice bounded to the confirmed mismatches, preserve release-surface consistency, keep touched Rust files at or above 95% line coverage unless an explicit exception is justified  
**Scale/Scope**: One operational publish-policy adjustment, one assistant-syntax cleanup, one repository-wide version-line advance, and associated test/doc closeout

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/045-mode-publish-alignment/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/orchestrator/publish.rs
├── canon-cli/
│   └── src/
└── canon-adapters/

assistant/
├── commands/governed-methods.json
└── prompts/copilot-command-pack.md

tech-docs/
└── guides/modes.md

tests/
├── assistant_plugin_packages.rs
├── security_assessment_direct_runtime.rs
└── integration/

specs/
└── 045-mode-publish-alignment/
```

**Structure Decision**: This is an existing single Rust workspace with repository-published docs and assistant metadata. The feature stays inside the existing engine publish orchestration, assistant metadata/docs, release-line surfaces, and targeted tests.

## Complexity Tracking

No constitution deviations are expected for this slice.
