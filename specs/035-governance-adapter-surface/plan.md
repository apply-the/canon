# Implementation Plan: Governance Adapter Surface For External Orchestrators

**Branch**: `035-governance-adapter-surface` | **Date**: 2026-05-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/035-governance-adapter-surface/spec.md`

## Summary

Deliver feature 035 as one end-to-end slice by adding a first-class
machine-facing `canon governance` surface for external orchestrators,
projecting Canon's existing runtime state into a flat and versioned `v1`
contract, normalizing strict readiness semantics so `governed_ready` only
appears with a reusable packet, publishing capabilities and schema-version
discovery, preserving the human CLI surface, and shipping `0.35.0` with
contract docs, validation evidence, `cargo fmt`, `cargo clippy`, and more
than 95% coverage for every modified or newly created Rust source file.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact because the feature introduces Canon's
first public machine-facing governance contract for external orchestrators,
changes downstream control flow expectations, and creates a new compatibility
promise that must remain stable across releases  
**Scope In**: 035 planning artifacts; `canon governance` CLI parsing and
command execution; machine-facing request and response models; response
normalization for lifecycle, approval, packet readiness, and canonical
workspace-relative refs; capabilities reporting; contract and integration
tests; `0.35.0` version alignment; impacted docs plus changelog; focused Rust
coverage above 95% for touched files; and `cargo clippy` plus `cargo fmt`
closeout  
**Scope Out**: session orchestration, replanning, retry loops, cluster or
assistant routing concerns; new persistence schema or publish semantics;
consumer-specific workflow taxonomies; HTTP or daemon transport; and changes to
existing human CLI behavior outside the additive governance namespace

**Invariants**:

- Canon MUST remain the downstream governed runtime and MUST NOT absorb orchestration-brain responsibilities.
- `status: governed_ready` MUST only escape the machine-facing surface when the packet is reusable, inspectable, and backed by a non-empty packet projection.
- `status: awaiting_approval` MUST only escape with `approval_state: requested`.
- `packet_ref`, `expected_document_refs`, and `document_refs` MUST be canonical workspace-relative refs, never absolute machine-local paths.
- Well-formed requests missing domain-required context MUST yield deterministic blocked domain outcomes rather than protocol failure.
- Existing `run`, `resume`, `status`, `approve`, `verify`, `inspect`, `skills`, `list`, and `publish` flows MUST remain intact.

**Decision Log**: `specs/035-governance-adapter-surface/decision-log.md`  
**Validation Ownership**: Generation happens through planning artifacts, CLI and
engine code changes, contract documents, tests, and release-surface updates on
the feature branch; validation happens through focused contract and integration
tests, coverage review for touched Rust files, lint and format commands, and a
separate live consumer-driven smoke against a real Canon binary.  
**Approval Gates**: Explicit maintainer ownership because this is
systemic-impact work, plus repository quality gates (`cargo fmt --check`,
`cargo clippy --workspace --all-targets --all-features -- -D warnings`,
targeted contract and integration tests, `cargo nextest run`, touched-file
coverage above 95%, and independent consumer-driven smoke evidence); no new
runtime approval gate is introduced by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown and JSON-facing contract artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused CLI contract and engine integration tests, and one live consumer-driven smoke against the current Synod adapter expectations  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with public command parsing, orchestrator services, filesystem-backed runtime state, embedded skill references, and repository documentation artifacts  
**Existing System Touchpoints**: `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands.rs`, `crates/canon-cli/src/commands/`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/execution.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/persistence/store.rs`, `tests/contract/`, `tests/integration/`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, and shared runtime-compatibility references under `defaults/embedded-skills/` and `.agents/skills/`  
**Performance Goals**: Preserve current local CLI responsiveness for blocked and capabilities responses, add no network dependency, and avoid new multi-pass persistence or background runtime flows  
**Constraints**: Keep the `v1` response flat; default omitted `adapter_schema_version` to `v1`; emit exact published vocabularies; keep response refs workspace-relative; do not widen orchestration scope; keep existing human CLI behavior stable; and achieve greater than 95% coverage for every modified or newly created Rust source file in this slice  
**Scale/Scope**: One cross-cutting feature spanning CLI parsing, command execution, runtime-state projection, packet-readiness normalization, contract docs, release alignment, and regression validation across the current governed mode surface

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
specs/035-governance-adapter-surface/
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
crates/canon-cli/src/
├── app.rs
├── commands.rs
├── output.rs
└── commands/
    ├── inspect.rs
    ├── governance.rs
    ├── run.rs
    └── status.rs

crates/canon-engine/src/
├── orchestrator/
│   ├── publish.rs
│   ├── service.rs
│   └── service/
│       ├── execution.rs
│       └── inspect.rs
└── persistence/
    └── store.rs

defaults/embedded-skills/
└── canon-shared/references/runtime-compatibility.toml

.agents/skills/
└── canon-shared/references/runtime-compatibility.toml

tests/
├── contract/
│   └── governance_cli.rs
└── integration/
    └── governance_adapter_surface.rs
```

**Structure Decision**: Keep the existing Rust workspace and current CLI plus
engine layering. Implement 035 by adding an additive governance command module,
machine-facing request or response normalization near the CLI boundary, and
runtime-backed packet projection using the existing orchestrator and
persistence seams rather than introducing a new crate, service, or schema.

## Complexity Tracking

No constitution violations are currently expected for this feature.
