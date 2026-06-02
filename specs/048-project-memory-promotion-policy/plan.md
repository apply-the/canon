# Implementation Plan: Project Memory Promotion Policy

**Branch**: `048-project-memory-promotion-policy` | **Date**: 2026-05-13 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/048-project-memory-promotion-policy/spec.md`

## Summary

Establish a Canon-owned project-memory promotion contract: publish profiles,
promotion policy states, lineage metadata, and non-destructive update strategies.
This slice adds the domain types, promotion policy engine, profile-aware publish
path, lineage metadata emission, non-destructive update helpers, and a stable
consumer contract document without turning Canon into the delivery orchestrator.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact; the slice introduces a new publication
contract and promotion policy surface without moving delivery orchestration into
Canon or changing existing runtime storage semantics.
**Scope In**:
- `PublishProfile` and `PromotionState` domain types
- Lineage metadata struct with required fields
- `UpdateStrategy` enum for non-destructive updates
- Promotion policy evaluation logic
- Profile-aware publish path in `publish.rs` and `service.rs`
- `--profile` CLI argument for `canon publish`
- Managed-block, proposal-file, and append-only-index update helpers
- Stable contract document at `tech-docs/integration/project-memory-promotion-contract.md`
- Default promotion policy TOML
- Tests for each promotion state and update strategy
- Version bump to 0.48.0

**Scope Out**:
- Boundline delivery-path or stage-planner logic
- Boundline assurance-profile logic
- Boundline governed-stage orchestration
- Canon as a delivery orchestrator
- Runtime storage schema changes to `.canon/`

**Invariants**:
- `.canon/` remains the governed runtime and evidence storage surface
- Project-visible output is a promoted projection of governed results
- Canon owns publish profiles, promotion policy, lineage, update strategies
- Canon MUST NOT become the delivery orchestrator
- Consumers MUST NOT redefine Canon promotion semantics
- Existing publish behavior without an explicit profile remains covered by
    regression tests during this pre-stable contract line

**Decision Log**: `specs/048-project-memory-promotion-policy/decision-log.md`
**Validation Ownership**: Agent generates implementation; `cargo test`, `cargo clippy`, `cargo fmt --check`, and coverage tooling validate independently.
**Approval Gates**: Human review of contract document before merge.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `toml`, `thiserror`, `tracing`, `uuid`, `time`
**Storage**: Local filesystem under `.canon/` (TOML manifests, Markdown artifacts)
**Testing**: `cargo test` / `cargo nextest run` / `cargo llvm-cov`
**Target Platform**: macOS, Linux, Windows (cross-platform CLI)
**Project Type**: CLI + library workspace
**Existing System Touchpoints**:
- `crates/canon-engine/src/orchestrator/publish.rs` (core publish logic)
- `crates/canon-engine/src/orchestrator/service.rs` (publish dispatch)
- `crates/canon-engine/src/domain/mode.rs` (Mode enum, ModeProfile)
- `crates/canon-cli/src/` (CLI argument parsing)
- `PublishMetadata` struct (sidecar JSON)
- `PublishSummary` struct (return type)
**Performance Goals**: N/A (CLI tool, no latency constraints)
**Constraints**: Existing `canon publish` invocation paths remain regression-tested during this slice
**Scale/Scope**: 18 modes, ~1000 lines in publish.rs

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
specs/048-project-memory-promotion-policy/
├── spec.md
├── plan.md
├── decision-log.md
├── contracts/
│   └── boundline-project-memory-promotion-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── mode.rs           (existing; no changes needed)
│       │   └── publish_profile.rs (NEW: PublishProfile, PromotionState, LineageMetadata, UpdateStrategy)
│       ├── orchestrator/
│       │   ├── publish.rs        (profile-aware publish, lineage emission, non-destructive updates)
│       │   └── service.rs        (publish dispatch with optional profile)
│       └── lib.rs                (re-export new domain types)
├── canon-cli/
│   └── src/
│       └── main.rs or commands/  (--profile argument on publish subcommand)
└── canon-adapters/               (no changes expected)
defaults/
└── policies/
    └── publish-profiles.toml     (NEW: default promotion policy per mode)
tech-docs/
└── integration/
    └── project-memory-promotion-contract.md (NEW: stable consumer contract)
```

**Structure Decision**: Existing Rust workspace layout with a new domain module
`publish_profile.rs`, a new default policy file, and a new stable contract doc.
No structural changes to the crate graph.

## Complexity Tracking

No constitution violations identified.
