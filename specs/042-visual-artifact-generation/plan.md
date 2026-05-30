# Implementation Plan: Pragmatic C4 Architecture Packets And Visual Artifacts

**Branch**: `042-visual-artifact-generation` | **Date**: 2026-05-08 | **Spec**: [specs/042-visual-artifact-generation/spec.md](specs/042-visual-artifact-generation/spec.md)
**Input**: Feature specification from `/specs/042-visual-artifact-generation/spec.md`

## Summary

Reshape Canon's `architecture` packet from a flat multi-file bundle into a pragmatic C4 handoff with one primary architecture document, one machine-readable manifest of included and omitted views, Mermaid diagram sources for each included visual view, and optional SVG or PNG renderings when the environment can support them honestly. The implementation stays additive and governed: existing supporting artifacts remain inspectable, but publish and review now center one human-readable packet instead of forcing reviewers to open every file independently.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact, because the feature changes packet contract and release-facing documentation surfaces without expanding destructive runtime authority.  
**Scope In**: architecture artifact contract updates, markdown and Mermaid rendering, publishable manifest shape, optional rendered asset materialization, release version bump to `0.42.0`, docs, and focused validation for architecture-mode packet changes.  
**Scope Out**: code-level diagram generation, mandatory binary rendering support in all environments, non-architecture mode redesign except where shared visual infrastructure is strictly required, and any governance bypass around omitted or weak evidence.

**Invariants**:

- Architecture packets must remain honest about unsupported, omitted, or low-confidence views and render targets.
- Existing architecture supporting artifacts must remain inspectable and traceable even if the primary handoff surface becomes a consolidated document.

**Decision Log**: `specs/042-visual-artifact-generation/decision-log.md`  
**Validation Ownership**: Generation happens through spec, code, and documentation changes in the workspace; validation happens through targeted Rust tests, publish-surface checks, coverage review of modified Rust files, lint or format checks, and independent human readback of the emitted packet.  
**Approval Gates**: No new human approval gate beyond bounded-impact review; existing run-state approvals for architecture mode remain authoritative.

## Technical Context

**Language/Version**: Rust 1.96.0 workspace plus Markdown documentation and Spec Kit feature artifacts.  
**Primary Dependencies**: existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`, plus `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`, and current release helper shell scripts.  
**Storage**: local filesystem under `.canon/`, published repository docs under `docs/` and `specs/`, and repo-local feature artifacts under `specs/042-visual-artifact-generation/`.  
**Testing**: focused Rust contract, renderer, run, and publish tests; `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo nextest run`; and coverage inspection for touched Rust files with `cargo llvm-cov`.  
**Target Platform**: local-first CLI usage on macOS, Linux, and Windows with publishable Markdown and text-based diagram artifacts.  
**Project Type**: Rust CLI workspace with file-backed governed runtime artifacts and published documentation packets.  
**Existing System Touchpoints**: `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, architecture-mode run services, publish docs, release-version surfaces, and architecture tests under `tests/architecture_*` and `tests/contract/`.  
**Performance Goals**: no meaningful change to run latency; artifact generation should stay linear in the number of emitted views and render targets.  
**Constraints**: Mermaid must be the default text-based diagram contract, rendered SVG or PNG assets must be optional and capability-dependent, coverage review must focus on modified Rust files, and the release surface must move to `0.42.0` before closeout.  
**Scale/Scope**: one feature slice touching the architecture packet family, shared publish metadata, version surfaces, and a bounded set of docs and tests.

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
specs/042-visual-artifact-generation/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── architecture-visual-packet.md
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml
Cargo.lock
CHANGELOG.md
README.md
ROADMAP.md
crates/
├── canon-cli/
│   └── src/commands/publish.rs
└── canon-engine/
    └── src/
        ├── artifacts/
        │   ├── contract.rs
        │   └── markdown.rs
        └── orchestrator/
            └── publish.rs
docs/
└── guides/
    └── modes.md
tests/
├── architecture_c4_contract.rs
├── architecture_c4_renderer.rs
├── architecture_c4_run.rs
├── integration/
│   └── skills_bootstrap.rs
└── release_*.rs
```

**Structure Decision**: Keep the implementation in the existing Rust workspace and additive architecture artifact pipeline. The feature lands as contract and renderer changes in `canon-engine`, publish-surface verification in existing CLI and integration tests, and release or documentation alignment in the same repository root.

## Complexity Tracking

No constitution deviations are required for this feature.
