# Implementation Plan: Cybersecurity Risk Assessment Mode

**Branch**: `023-cybersecurity-risk-assessment` | **Date**: 2026-04-28 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/023-cybersecurity-risk-assessment/spec.md`

## Summary

Deliver a new first-class governed mode named `security-assessment` as part of
the `0.22.0` release by extending Canon's mode model, orchestration, artifact
contracts, markdown rendering, publishing, skill surfaces, examples, and tests.
The mode will be authored-input driven, recommendation-only, publishable to a
dedicated documentation path, and validated through focused mode coverage plus
full workspace formatting, linting, and regression checks.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact because the feature adds a new
first-class mode to the runtime, touches the central mode registry,
classification rules, orchestration dispatch, publishing, shared compatibility
surfaces, and release validation across the workspace  
**Scope In**: feature `023` planning and release artifacts; the new
`security-assessment` mode across domain, orchestration, contract, markdown,
publish, summary, and compatibility layers; new embedded and mirrored skill
surfaces; new templates and examples; focused contract, renderer, run, publish,
and docs tests; version and release-surface synchronization for `0.22.0`; and
validation evidence for coverage, formatting, linting, and full tests  
**Scope Out**: live scanner integrations; external network evidence collection;
`supply-chain-analysis`; compliance audit claims; autonomous remediation; new
`.canon/` persistence schemas; and unrelated mode-family redesigns

**Invariants**:

- `security-assessment` remains recommendation-only and never implies Canon can
  apply mitigations or enforce policy automatically.
- Existing modes, publish destinations, run identities, approval semantics, and
  `.canon/` persistence layout remain unchanged except for the addition of the
  new mode.
- Security findings and compliance anchors stay grounded in authored source
  material or surface explicit evidence gaps rather than fabricated certainty.

**Decision Log**: `specs/023-cybersecurity-risk-assessment/decision-log.md`  
**Validation Ownership**: Generation happens through spec, plan, tasks, code,
skills, docs, and test changes on the feature branch; validation happens
through focused automated tests, skill-sync validation, full workspace quality
gates, and a separate read-only review of the resulting packet and release
surfaces before closeout  
**Approval Gates**: explicit human ownership is required by the systemic-impact
risk class; normal repository review remains required; runtime risk approval for
the mode follows the governed packet gates during execution rather than adding a
new release-only approval path

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; no new persistent schema  
**Testing**: focused `cargo test --test ...` coverage for the new mode, `cargo nextest run`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with governed mode orchestration, markdown artifact rendering, embedded skill sources, mirrored AI-facing skills, and integration-heavy regression tests  
**Existing System Touchpoints**: `Cargo.toml`; `AGENTS.md`; `CHANGELOG.md`; `README.md`; `docs/guides/modes.md`; `defaults/methods/`; `defaults/embedded-skills/canon-shared/`; `crates/canon-engine/src/domain/mode.rs`; `crates/canon-engine/src/domain/gate.rs`; `crates/canon-engine/src/artifacts/contract.rs`; `crates/canon-engine/src/artifacts/markdown.rs`; `crates/canon-engine/src/orchestrator/classifier.rs`; `crates/canon-engine/src/orchestrator/gatekeeper.rs`; `crates/canon-engine/src/orchestrator/publish.rs`; `crates/canon-engine/src/orchestrator/service.rs`; `crates/canon-engine/src/orchestrator/service/`; `crates/canon-cli/src/output.rs`; `docs/templates/canon-input/`; `docs/examples/canon-input/`; `tests/`; and mirrored `.agents/skills/` surfaces  
**Performance Goals**: preserve current CLI responsiveness and add no mandatory external I/O beyond repository and `.canon/` filesystem access  
**Constraints**: stay offline-capable; keep skill-source and mirrored-skill surfaces synchronized; keep recommendation-only posture explicit; keep existing mode behavior stable; finish with clean formatting, lint, and workspace test results  
**Scale/Scope**: one new runtime mode, one new skill family, one new publish destination, several shared runtime edits, and a focused but repo-wide validation closeout

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
specs/023-cybersecurity-risk-assessment/
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
│   └── security-assessment.toml
└── embedded-skills/
    ├── canon-security-assessment/
    │   └── skill-source.md
    └── canon-shared/
        ├── references/
        └── scripts/

.agents/skills/
├── canon-security-assessment/
│   └── SKILL.md
└── canon-shared/
    ├── references/
    └── scripts/

crates/canon-engine/src/
├── artifacts/
│   ├── contract.rs
│   └── markdown.rs
├── domain/
│   ├── gate.rs
│   └── mode.rs
└── orchestrator/
    ├── classifier.rs
    ├── gatekeeper.rs
    ├── publish.rs
    └── service/
        ├── mode_security_assessment.rs
        └── tests.rs

crates/canon-cli/src/
└── output.rs

docs/
├── guides/modes.md
├── templates/canon-input/
└── examples/canon-input/

tests/
├── integration/
└── security_assessment_*.rs
```

**Structure Decision**: Keep the existing Rust workspace and governed mode
layout. Implement the new mode by extending shared registries and adding a
mode-specific service module, method file, skill files, docs, and focused tests
instead of introducing a new crate or persistence subsystem.

## Complexity Tracking

No constitution violations are currently expected for this feature.
