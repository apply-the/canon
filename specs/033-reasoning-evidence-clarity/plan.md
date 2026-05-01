# Implementation Plan: Cross-Mode Reasoning Evidence And Clarity Expansion

**Branch**: `033-reasoning-evidence-clarity` | **Date**: 2026-05-01 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/033-reasoning-evidence-clarity/spec.md`

## Summary

Deliver feature 033 end to end by expanding `inspect clarity` and
`reasoning_signals` across the remaining file-backed governed modes,
introducing a shared runtime posture for reasoning evidence versus shallow
output, tightening placeholder-heavy fallback packet sections so they read as
honest gaps rather than template filler, and synchronizing mirrored skills,
templates, examples, docs, changelog, version surfaces, coverage, `cargo
clippy`, and `cargo fmt` around the final `0.33.0` contract.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact because the feature changes shared
clarity inspection, reasoning-signal reporting, summarizer posture, renderer
fallback behavior, skill guidance, and release-facing surfaces across most of
the currently modeled governed modes without introducing a new runtime domain
or storage schema  
**Scope In**: 033 planning artifacts; `inspect clarity` expansion for
file-backed governed modes; reasoning-evidence posture for summaries and
affected packets; fallback tightening for placeholder-heavy artifacts;
review-family honesty alignment; mirrored skill, template, example, and docs
updates; `0.33.0` version alignment; explicit docs and changelog closeout; and
focused coverage, `cargo clippy`, and `cargo fmt` validation for touched Rust
files  
**Scope Out**: new runtime modes; new adapters or external integrations;
changes to `.canon/` persistence, publish destinations, run identity, or
approval targets; generic wording-only rewrites outside impacted surfaces; and
splitting runtime and authoring work into separate feature deliveries

**Invariants**:

- Canon MUST preserve or strengthen explicit honesty markers such as `## Missing Authored Body`, `## Missing Evidence`, blocked posture, unsupported verdicts, and unresolved findings.
- Canon MUST NOT fabricate contradictions, alternatives, or tradeoffs when the authored input materially closes the decision or cannot support the claim.
- Existing `.canon/` storage, run identity, publish semantics, approval flows, and recommendation-only posture MUST remain unchanged.

**Decision Log**: `specs/033-reasoning-evidence-clarity/decision-log.md`  
**Validation Ownership**: Generation happens through planning artifacts, runtime code changes, skill and template updates, docs alignment, and test additions on the feature branch; validation happens through focused automated tests, coverage review for touched Rust files, skill-sync checks, lint and format commands, and a separate final artifact and diff review before merge.  
**Approval Gates**: Explicit maintainer ownership because the feature is systemic-impact, plus repository quality gates (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, focused tests, coverage evidence for touched Rust files, and `/bin/bash scripts/validate-canon-skills.sh`); no new runtime approval gate is introduced by this feature.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: `cargo test`, `cargo nextest run`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, focused contract and run tests, docs or skill-sync tests, and `/bin/bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with embedded skill sources, mirrored AI-facing skills, contract-driven markdown rendering, and repository documentation artifacts  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-engine/src/orchestrator/service/clarity.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/gatekeeper.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-cli/src/output.rs`, `defaults/embedded-skills/`, `.agents/skills/`, `docs/templates/canon-input/`, `docs/examples/canon-input/`, `docs/guides/modes.md`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `AGENTS.md`, `tests/contract/inspect_clarity.rs`, targeted mode tests under `tests/`, and release/docs regression tests  
**Performance Goals**: Preserve current local CLI responsiveness, add no required network round trip, and avoid any new persistence or execution latency path beyond the existing mode flows  
**Constraints**: Keep non-target mode semantics stable unless reasoning-evidence posture must be shared; preserve explicit honesty markers; avoid placeholder prose that reads like authored reasoning; keep the feature unsliced across runtime and authoring surfaces; align release-facing surfaces to `0.33.0`; and provide focused automated coverage for every modified or newly created Rust file  
**Scale/Scope**: One cross-cutting feature spanning shared engine services, renderer helpers, mode-facing docs and skill mirrors, release surfaces, and focused regression coverage across the current governed mode set

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
specs/033-reasoning-evidence-clarity/
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
crates/canon-engine/src/
├── artifacts/
│   ├── contract.rs
│   └── markdown.rs
└── orchestrator/service/
    ├── clarity.rs
    ├── inspect.rs
    └── summarizers.rs

crates/canon-engine/src/orchestrator/
└── gatekeeper.rs

crates/canon-cli/src/
└── output.rs

defaults/embedded-skills/
├── canon-inspect-clarity/skill-source.md
├── canon-*/skill-source.md
└── canon-shared/references/runtime-compatibility.toml

.agents/skills/
├── canon-inspect-clarity/SKILL.md
├── canon-*/SKILL.md
└── canon-shared/references/runtime-compatibility.toml

docs/
├── guides/modes.md
├── templates/canon-input/
└── examples/canon-input/

tests/
├── contract/inspect_clarity.rs
├── backlog_*.rs
├── *_authoring_docs.rs
├── *_contract.rs
├── *_run.rs
└── release_*.rs
```

**Structure Decision**: Keep the existing Rust workspace, shared artifact
helpers, embedded skill sources, mirrored `.agents` skills, and repository doc
layout. Implement the feature by extending the existing clarity, summarizer,
gatekeeper, and renderer surfaces plus synchronized docs and validation rather
than introducing a new crate or storage path.

## Complexity Tracking

No constitution violations are currently expected for this feature.
