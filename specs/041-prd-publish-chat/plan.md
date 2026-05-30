# Implementation Plan: Requirements PRD Publishing And Chat Publish Skill

**Branch**: `041-prd-publish-chat` | **Date**: 2026-05-07 | **Spec**: [/Users/rt/workspace/apply-the/canon/specs/041-prd-publish-chat/spec.md](/Users/rt/workspace/apply-the/canon/specs/041-prd-publish-chat/spec.md)
**Input**: Feature specification from `/specs/041-prd-publish-chat/spec.md`

## Summary

Add a consolidated published requirements PRD without breaking the existing sectional packet model, then extend the chat-first skill surface so Copilot and Codex users can explicitly invoke publish. Implement the change by adding an additive `prd.md` requirement to the requirements artifact contract, rendering it from the same authored evidence used by the sectional files, letting the generic publish pipeline copy it automatically, and aligning docs, release metadata, and validation coverage around the clearer publish UX.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact, because this feature changes artifact and documentation surfaces that users rely on while preserving existing governance and publish gating semantics.  
**Scope In**: requirements artifact contract and markdown rendering, publish-visible output, repo-local publish skill authoring, release/version surfaces, documentation, and focused tests or validators.  
**Scope Out**: publish engine rewrites, new governance adapter operations, non-requirements consolidated packets beyond documentation mentions, and visual diagram or image generation.

**Invariants**:

- Existing sectional requirements artifacts and `packet-metadata.json` remain published exactly as before; the consolidated PRD is additive.
- Publish continues to require a publishable run and does not bypass approval, critique, or destination boundary checks.
- Chat publish guidance remains a thin skill layer over the real CLI contract rather than a separate runtime path.

**Decision Log**: `specs/041-prd-publish-chat/decision-log.md`  
**Validation Ownership**: Generation happens through feature docs and code changes in the workspace; validation happens through focused Rust tests, lint or format checks, skill validation scripts, and a user-perspective review of the published packet and chat guidance.  
**Approval Gates**: No new human approval gate beyond normal bounded-impact review; existing run-state gates remain authoritative for publish itself.

## Technical Context

**Language/Version**: Rust 1.96.0 workspace plus Markdown documentation and repo-local skills  
**Primary Dependencies**: Existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`), `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`, and repo skill validation scripts  
**Storage**: Local filesystem under `.canon/` plus published repository files under `specs/`, `docs/`, and `.agents/skills/`  
**Testing**: `cargo test`, focused Rust integration or contract tests, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `scripts/validate-canon-skills.sh`  
**Target Platform**: Local-first Canon usage on macOS, Linux, and Windows with chat skills for Copilot or Codex  
**Project Type**: Rust CLI workspace with file-backed runtime artifacts, published markdown packets, and repo-local AI skills  
**Existing System Touchpoints**: `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/mode_requirements.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-cli/src/commands/publish.rs`, `.agents/skills/`, `defaults/embedded-skills/`, `README.md`, `docs/guides/modes.md`, `CHANGELOG.md`, `Cargo.toml`, and focused tests under `tests/` and `crates/canon-cli/src/commands/publish.rs`  
**Performance Goals**: No meaningful runtime performance change; publish output should remain linear in the number of artifacts and the consolidated PRD should be generated within the normal requirements packet flow  
**Constraints**: Preserve source artifact compatibility, keep metadata shape stable except for additive source paths, avoid fabricated publish semantics in chat skills, and keep the release slice tight enough to validate with targeted tests  
**Scale/Scope**: One additive requirements artifact, one new repo-local skill mirrored into embedded skills, a handful of doc and release files, and focused test updates rather than a broad publish-system rewrite

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [X] Execution mode is declared and matches the requested work
- [X] Risk classification is explicit and autonomy is appropriate for that risk
- [X] Scope boundaries and exclusions are recorded
- [X] Invariants are explicit before implementation
- [X] Required artifacts and owners are identified
- [X] Decision logging is planned and linked to a durable artifact
- [X] Validation plan separates generation from validation
- [X] Declared-risk approval checkpoints are named where required by the risk classification
- [X] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/041-prd-publish-chat/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── requirements-publish-surface.md
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml
CHANGELOG.md
README.md
docs/
└── guides/
  └── modes.md
.agents/
└── skills/
  └── canon-publish/
    └── SKILL.md
defaults/
└── embedded-skills/
  └── canon-publish/
    └── skill-source.md
crates/
├── canon-cli/
│   └── src/commands/publish.rs
└── canon-engine/
  └── src/
    ├── artifacts/
    │   ├── contract.rs
    │   └── markdown.rs
    └── orchestrator/
      ├── publish.rs
      └── service/mode_requirements.rs
tests/
├── contract/
│   └── requirements_contract.rs
└── requirements_authoring_renderer.rs
scripts/
└── validate-canon-skills.sh
```

**Structure Decision**: Keep the implementation in the existing Rust workspace and repo-local documentation or skills layout. The feature stays additive by touching only the requirements artifact layer, generic publish visibility through existing copy loops, skill packaging mirrors, release docs, and focused tests.

## Complexity Tracking

No constitution deviations are expected for this feature.
