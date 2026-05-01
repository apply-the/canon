# Implementation Plan: Structured External Publish Destinations

**Branch**: `029-publish-destinations` | **Date**: 2026-05-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/029-publish-destinations/spec.md`

## Summary

Deliver the 029 publish-structure slice by replacing run-id-only default
publish destinations with readable date-prefixed descriptor paths, adding
durable published-packet metadata for run traceability, preserving explicit
publish overrides and approval-gated operational publishing behavior, and
aligning the `0.29.0` release surface with focused Rust coverage,
`cargo clippy`, and `cargo fmt` closeout.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature changes the shared
external publish contract, metadata materialization, docs, tests, and release
surfaces across already-modeled modes without changing `.canon/` persistence,
run identity, approval semantics, or the publish CLI interface  
**Scope In**: 029 planning artifacts; default publish destination structure;
published metadata materialization; explicit override preservation; focused
publish-path tests; release-surface version alignment to `0.29.0`; and final
coverage, `cargo clippy`, and `cargo fmt` closeout  
**Scope Out**: remote publishing; `.canon/` runtime layout changes; new modes;
CLI redesign; approval-policy changes; and unrelated artifact-shape rewrites

**Invariants**:

- `.canon/` remains runtime and evidence storage only; external publish stays a
  materialization step rather than a new runtime surface.
- Canonical `run_id` identity remains unchanged and recoverable even when the
  external path is descriptor-based.
- Explicit `publish --to` overrides remain authoritative and are not silently
  normalized into the default structured destination.
- Existing publish eligibility for completed and approval-gated operational
  packets remains unchanged.

**Decision Log**: `specs/029-publish-destinations/decision-log.md`  
**Validation Ownership**: Generation happens through planning artifacts,
publish-layer code changes, tests, and docs on the implementation branch;
validation happens through focused automated publish-path tests,
release-surface checks, coverage review for touched Rust files, and a final
independent diff and output review before merge.  
**Approval Gates**: Standard maintainer review plus repository quality gates
(`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, focused publish-path regression coverage, full-workspace regression, and coverage evidence for touched Rust files); no new runtime approval gate is introduced by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown documentation and Spec Kit artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; Spec Kit scripts under `.specify/scripts/bash/`  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run --workspace --all-features`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused publish unit and integration tests, and release-surface checks  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with orchestration, persistence, publish logic, and repository documentation artifacts  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/persistence/manifests.rs`, `crates/canon-cli/src/commands/publish.rs`, `tests/contract/runtime_filesystem.rs`, `tests/integration/*publish.rs`, `tests/integration/run_lookup.rs`, `ROADMAP.md`, `README.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `docs/guides/modes.md`, and shared runtime compatibility references under `.agents/skills/` and `defaults/embedded-skills/`  
**Performance Goals**: Preserve current local publish responsiveness and add no mandatory network activity, no new approval round-trip, and no duplicate artifact rendering pass  
**Constraints**: Keep `.canon/` layout unchanged; preserve the `publish` CLI contract and override semantics; keep default roots under current external families such as `specs/` and `docs/`; maintain approval-gated operational publish behavior; and provide focused coverage for every modified or newly added Rust file  
**Scale/Scope**: One cross-mode publish-layer slice spanning destination resolution, metadata materialization, release-surface docs, and focused regression coverage across the existing Rust workspace

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
specs/029-publish-destinations/
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
├── canon-cli/
│   └── src/
│       ├── app.rs
│       └── commands/publish.rs
└── canon-engine/
    └── src/
        ├── orchestrator/publish.rs
        └── persistence/
            ├── manifests.rs
            └── slug.rs

docs/
├── guides/
└── architecture/

tests/
├── contract/
├── integration/
└── *_publish.rs
```

**Structure Decision**: Keep the existing Rust workspace and documentation
layout. Implement the feature by updating the shared publish orchestrator,
using existing persisted run metadata for descriptor derivation, extending
publish-focused tests, and aligning release-facing docs and version surfaces
instead of adding a new crate or persistence layer.

## Complexity Tracking

No constitution violations are currently expected for this feature.
