# Implementation Plan: System Assessment Mode

**Branch**: `027-system-assessment-mode` | **Date**: 2026-04-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/027-system-assessment-mode/spec.md`

## Summary

Deliver a new first-class governed mode named `system-assessment` as part of
the `0.26.0` release by extending Canon's mode model, system-context
validation, orchestration, artifact contracts, markdown rendering, publishing,
skills, templates, examples, and regression coverage. The mode will stay
read-only, require `system-context existing`, use ISO 42010 coverage language,
emit explicit observed findings, inferred findings, and assessment gaps with
confidence, and
publish a bounded as-is architecture packet without changing the existing
decision-shaped `architecture` mode.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact because the feature adds a new
first-class mode, changes shared runtime registries and validation rules,
touches publishing and documentation surfaces, and requires repository-wide
release synchronization for `0.26.0`  
**Scope In**: feature `027` planning and release artifacts; the new
`system-assessment` mode across domain, orchestration, contract, markdown,
publish, summary, and compatibility layers; new embedded and mirrored skills;
templates and examples; focused contract, run, publish, docs, and shared mode
coverage; version and compatibility synchronization for `0.26.0`; and
validation evidence for tests, formatting, skill sync, and linting  
**Scope Out**: reworking `architecture` into an assessment mode; structured
external publish destination redesign; live external evidence collectors;
cross-mode large-repo traversal changes; generated diagrams; and new `.canon/`
persistence schemas

**Invariants**:

- `system-assessment` remains read-only and does not imply approved design or
  implementation decisions.
- `architecture` remains the decision-shaped mode for tradeoffs,
  recommendations, and ADR-oriented packets.
- Assessment claims stay grounded in authored context or explicit repository
  evidence, otherwise the packet records an assessment gap or reduced
  confidence.
- The first slice requires `system-context=existing`.
- Existing modes, run identity, `.canon/` runtime layout, and approval
  semantics remain unchanged except for shared lists and docs that now include
  `system-assessment`.

**Decision Log**: `specs/027-system-assessment-mode/decision-log.md`  
**Validation Ownership**: Generation happens through spec, design artifacts,
  code, skill, and documentation changes on the feature branch; validation
  happens through focused automated tests, skill validation, workspace quality
  gates, and a separate read-only walkthrough of the resulting packet and
  publish path before closeout  
**Approval Gates**: explicit human ownership is required by the
systemic-impact risk class; runtime risk approval for `system-assessment`
follows Canon's governed packet gates during execution rather than adding a
separate release-only approval flow

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: focused `cargo test --test ...` coverage for `system-assessment`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with governed mode orchestration, markdown artifact rendering, embedded skill sources, mirrored AI-facing skills, and integration-heavy regression tests  
**Existing System Touchpoints**: `Cargo.toml`; `Cargo.lock`; `CHANGELOG.md`; `README.md`; `ROADMAP.md`; `AGENTS.md`; `defaults/methods/`; `defaults/embedded-skills/canon-shared/`; `defaults/embedded-skills/`; `.agents/skills/`; `docs/guides/modes.md`; `docs/templates/canon-input/`; `docs/examples/canon-input/`; `crates/canon-engine/src/domain/mode.rs`; `crates/canon-engine/src/artifacts/contract.rs`; `crates/canon-engine/src/artifacts/markdown.rs`; `crates/canon-engine/src/orchestrator/classifier.rs`; `crates/canon-engine/src/orchestrator/gatekeeper.rs`; `crates/canon-engine/src/orchestrator/publish.rs`; `crates/canon-engine/src/orchestrator/service.rs`; `crates/canon-engine/src/orchestrator/service/`; `crates/canon-cli/src/output.rs`; and `tests/`  
**Performance Goals**: preserve current CLI responsiveness, remain offline-capable, and add no mandatory I/O beyond repository and `.canon/` filesystem reads and writes  
**Constraints**: keep skill-source and mirrored-skill surfaces synchronized; keep the packet as-is rather than decision-shaped; keep existing mode behavior stable; and finish with clean formatting, lint, and focused regression evidence  
**Scale/Scope**: one new runtime mode, one new skill family, one new publish destination under the architecture docs family, several shared runtime edits, and a focused but repo-wide validation closeout

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
specs/027-system-assessment-mode/
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
defaults/
├── methods/
│   └── system-assessment.toml
└── embedded-skills/
    ├── canon-system-assessment/
    │   └── skill-source.md
    └── canon-shared/
        ├── references/
        └── scripts/

.agents/skills/
├── canon-system-assessment/
│   └── SKILL.md
└── canon-shared/
    ├── references/
    └── scripts/

crates/canon-engine/src/
├── artifacts/
│   ├── contract.rs
│   └── markdown.rs
├── domain/
│   └── mode.rs
└── orchestrator/
    ├── classifier.rs
    ├── gatekeeper.rs
    ├── publish.rs
    └── service/
        ├── mode_security_assessment.rs
        ├── mode_supply_chain_analysis.rs
        ├── mode_system_assessment.rs
        └── tests.rs

crates/canon-cli/src/
└── output.rs

docs/
├── guides/modes.md
├── templates/canon-input/
└── examples/canon-input/

tests/
├── contract/
├── integration/
└── system_assessment_*.rs
```

**Structure Decision**: Keep the existing Rust workspace and governed mode
layout. Implement `system-assessment` by extending shared registries and adding
a mode-specific service module, method file, skill files, docs, and focused
tests instead of introducing a new crate, storage backend, or publish system.

## Complexity Tracking

No constitution violations are currently expected for this feature.
