# Implementation Plan: Supply Chain And Legacy Analysis Mode

**Branch**: `024-supply-chain-legacy` | **Date**: 2026-04-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/024-supply-chain-legacy/spec.md`

## Summary

Deliver a new first-class governed mode named `supply-chain-analysis` as part
of the `0.24.0` release by extending Canon's mode model, scanner-backed
orchestration flow, artifact contracts, markdown rendering, publishing,
clarification guidance, skill surfaces, examples, and tests. The mode will be
authored-input driven, recommendation-only, publishable to `docs/supply-chain/`,
and validated through focused direct-runtime coverage plus workspace-level
formatting, linting, and release-surface synchronization.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: systemic-impact because the feature adds a new
first-class governed mode to the runtime, introduces scanner-backed evidence
collection and user-decision prompting, touches the central mode registry,
classification rules, orchestration dispatch, publish flow, shared runtime
compatibility surfaces, and release validation across the workspace  
**Scope In**: feature `024` planning and release artifacts; the new
`supply-chain-analysis` mode across domain, orchestration, contract, markdown,
publish, summary, and compatibility layers; new embedded and mirrored skill
surfaces; new templates and examples; clarification and missing-scanner
decision handling; focused contract, renderer, direct-runtime, publish, and
docs tests; version and release-surface synchronization for `0.24.0`; and
validation evidence for high Rust-file coverage, formatting, linting, and
workspace quality gates  
**Scope Out**: reimplementing scanners; automatic installation or remediation;
compliance certification; unrelated protocol or packaging work; new `.canon/`
persistence schemas; and unrelated mode-family redesigns

**Invariants**:

- `supply-chain-analysis` remains recommendation-only and never installs tools,
  rewrites dependencies, or implies Canon can decide legal or security posture
  autonomously.
- Findings and posture claims stay grounded in authored input plus tool-backed
  evidence, or surface explicit coverage or decision gaps instead of fabricated
  certainty.
- Existing modes, publish destinations, run identities, approval semantics,
  and `.canon/` persistence layout remain unchanged except for the addition of
  the new mode and its dedicated outputs.

**Decision Log**: `specs/024-supply-chain-legacy/decision-log.md`  
**Validation Ownership**: Generation happens through spec, plan, tasks, code,
skills, docs, and test changes on the feature branch; validation happens
through focused automated tests, shared skill validation, coverage evidence,
workspace formatting and lint checks, and a separate read-only review of the
resulting packet and release surfaces before closeout  
**Approval Gates**: explicit human ownership is required by the systemic-impact
risk class; normal repository review remains required; runtime risk approval
for the mode follows the governed packet gates during execution rather than
adding a separate release-only approval path

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown skill sources, Bash runtime checks, and documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; existing shell and filesystem adapters; repo-local skill validation scripts  
**Storage**: Repository files plus existing `.canon/` runtime filesystem; scanner outputs and machine-readable SBOM references persist as normal run artifacts or payload references without a new persistent schema  
**Testing**: focused `cargo test --test ...` coverage for the new mode, direct `EngineService` tests for orchestration coverage, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `/bin/bash scripts/validate-canon-skills.sh`, and final workspace regression checks  
**Target Platform**: Cross-platform local CLI workflow on macOS, Linux, and Windows  
**Project Type**: Rust CLI workspace with governed mode orchestration, markdown artifact rendering, embedded skill sources, mirrored AI-facing skills, and integration-heavy regression tests  
**Existing System Touchpoints**: `Cargo.toml`; `AGENTS.md`; `CHANGELOG.md`; `README.md`; `docs/guides/modes.md`; `defaults/methods/`; `defaults/embedded-skills/canon-shared/`; `defaults/embedded-skills/canon-supply-chain-analysis/`; `.agents/skills/canon-shared/`; `.agents/skills/canon-supply-chain-analysis/`; `crates/canon-adapters/src/shell.rs`; `crates/canon-adapters/src/filesystem.rs`; `crates/canon-engine/src/domain/mode.rs`; `crates/canon-engine/src/artifacts/contract.rs`; `crates/canon-engine/src/artifacts/markdown.rs`; `crates/canon-engine/src/orchestrator/classifier.rs`; `crates/canon-engine/src/orchestrator/gatekeeper.rs`; `crates/canon-engine/src/orchestrator/publish.rs`; `crates/canon-engine/src/orchestrator/service.rs`; `crates/canon-engine/src/orchestrator/service/`; `crates/canon-engine/src/persistence/store.rs`; `docs/templates/canon-input/`; `docs/examples/canon-input/`; and `tests/`  
**Performance Goals**: Preserve current CLI responsiveness, keep analysis bounded to declared repository surfaces and detected ecosystems, and avoid introducing mandatory network dependency beyond user-invoked local scanners  
**Constraints**: Stay offline-capable at the Canon layer; do not auto-install tools; keep skill-source and mirrored-skill surfaces synchronized; keep recommendation-only posture explicit; make the first task the `0.24.0` version bump; and make the last task the high-coverage plus docs, examples, roadmap, formatting, and lint closeout  
**Scale/Scope**: One new runtime mode, one new skill family, one new publish destination, one new clarification and missing-scanner decision surface, several shared runtime edits, and a focused but repo-wide validation closeout

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
specs/024-supply-chain-legacy/
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
│   └── supply-chain-analysis.toml
└── embedded-skills/
    ├── canon-supply-chain-analysis/
    │   └── skill-source.md
    └── canon-shared/
        ├── references/
        └── scripts/

.agents/skills/
├── canon-supply-chain-analysis/
│   └── SKILL.md
└── canon-shared/
    ├── references/
    └── scripts/

crates/canon-adapters/src/
├── filesystem.rs
└── shell.rs

crates/canon-engine/src/
├── artifacts/
│   ├── contract.rs
│   └── markdown.rs
├── domain/
│   └── mode.rs
├── orchestrator/
│   ├── classifier.rs
│   ├── gatekeeper.rs
│   ├── publish.rs
│   └── service/
│       ├── mode_supply_chain_analysis.rs
│       └── summarizers.rs
└── persistence/
    └── store.rs

docs/
├── guides/modes.md
├── templates/canon-input/
└── examples/canon-input/

tests/
├── contract/
├── integration/
└── supply_chain_analysis_*.rs
```

**Structure Decision**: Keep the existing Rust workspace and governed mode
layout. Implement the new mode by extending shared registries and adding a
mode-specific service module, method file, skill files, docs, and focused
tests instead of introducing a new crate or a new adapter architecture.

## Complexity Tracking

No constitution violations are currently expected for this feature.
