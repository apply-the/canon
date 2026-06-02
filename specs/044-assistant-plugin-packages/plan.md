# Implementation Plan: Assistant Plugin Packages

**Branch**: `PLACEHOLDER` | **Date**: PLACEHOLDER | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/044-assistant-plugin-packages/spec.md`

## Summary

Add host-specific assistant plugin package folders that let Claude Code, Codex, and Cursor discover Canon as a governed packet runtime while keeping Canon CLI and the governance adapter authoritative. The implementation will add shared assistant plugin metadata, host manifests or glue that reference existing Canon skills and methods, a documented Copilot command/prompt pack boundary, validation coverage for metadata/path/version drift, and installation documentation.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact, because the slice changes repository packaging metadata, public documentation, and validation for assistant host install surfaces without changing runtime execution authority, packet persistence, approvals, evidence, or provenance semantics.  
**Scope In**: version bump to `0.44.0`, shared plugin metadata, Claude/Codex/Cursor package folders, documented Copilot command/prompt pack, package assets, plugin install docs, README summary, validation script/test coverage, and final formatting/lint/test/coverage closeout.  
**Scope Out**: new Canon runtime behavior, new governance adapter semantics, assistant-host-owned governance decisions, Boundline requirements, duplicated skill corpora in package folders, private internal exposure, or invented Copilot manifest support.

**Invariants**:

- Canon CLI and the machine-facing governance adapter remain authoritative; assistant host packages provide install, discovery, metadata, command binding, prompt, and glue surfaces only.
- Shared skill and method source material stays in `.agents/skills/`, `.canon/methods/`, and `defaults/embedded-skills/`; host package folders reference those paths instead of copying the corpus.
- Plugin positioning must describe Canon as a governed packet runtime with structured packets, evidence, approvals, and provenance, and must not imply agent orchestration, code execution, or host-owned workspace mutation.
- Validation must fail when manifests drift from Canon version, omit required metadata, reference missing paths, omit required governed method surfaces, contain invalid JSON, or use prohibited positioning phrases.

**Decision Log**: `specs/044-assistant-plugin-packages/decision-log.md`  
**Validation Ownership**: Generation happens through repository metadata, package folders, docs, scripts, and tests; validation happens through the dedicated plugin package test/script, JSON parsing, path/version/prohibited-language checks, cargo test, clippy, fmt, and coverage evidence captured in `validation-report.md`.  
**Approval Gates**: No additional human approval gate is required for bounded-impact packaging work; final closeout includes explicit coherence review and validation evidence.

## Technical Context

**Language/Version**: Rust 1.96.0 workspace plus JSON, Markdown, Bash, and SVG repository assets.  
**Primary Dependencies**: existing workspace crates and dev dependencies including `serde_json`; no new external crates are planned.  
**Storage**: repository files only: hidden host package folders, shared assistant metadata under `assistant/`, docs under `tech-docs/`, validation scripts under `scripts/`, and Spec Kit artifacts under `specs/044-assistant-plugin-packages/`.  
**Testing**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test`, `cargo test --test assistant_plugin_packages`, `bash scripts/validate-assistant-plugins.sh`, and touched-file coverage inspection with `cargo llvm-cov` if available.  
**Target Platform**: local-first repository workflows on macOS, Linux, and Windows-friendly metadata; shell validation targets Unix-like local/CI environments.  
**Project Type**: Rust CLI workspace with file-backed governed runtime artifacts and repository-local assistant skills.  
**Existing System Touchpoints**: `Cargo.toml`, `Cargo.lock`, `README.md`, `.agents/skills/`, `.canon/methods/`, `defaults/embedded-skills/`, `tech-docs/guides/`, `scripts/`, `tests/`, and `AGENTS.md` via Speckit context update.  
**Performance Goals**: package validation should complete in under one second excluding cargo compile time and must not run Canon runtime commands.  
**Constraints**: version bump is the first implementation task; package folders must mostly contain manifests/metadata/glue; JSON files must remain valid; referenced paths must be repository-relative and existing; no host package may become the source of truth for Canon behavior; final validation must include 95% coverage for new or modified Rust source files.  
**Scale/Scope**: one bounded feature slice with three host package folders, one documented Copilot command/prompt pack, one shared metadata source, one guide, one README section, one validation script, and focused Rust integration tests.

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
specs/044-assistant-plugin-packages/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── assistant-plugin-package-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml
Cargo.lock
README.md
.claude-plugin/
├── manifest.json
└── commands.json
.codex-plugin/
└── plugin.json
.cursor-plugin/
├── manifest.json
└── commands.json
assistant/
├── plugin-metadata.json
├── assets/
│   ├── canon-plugin-icon.svg
│   └── canon-plugin-logo.svg
├── commands/
│   └── governed-methods.json
└── prompts/
    ├── starter-prompts.md
    └── copilot-command-pack.md
tech-docs/
└── guides/
    └── assistant-plugin-packages.md
scripts/
└── validate-assistant-plugins.sh
tests/
└── assistant_plugin_packages.rs
```

**Structure Decision**: Keep host-specific package folders at the repository root because the requested package shape names `.claude-plugin/`, `.codex-plugin/`, and `.cursor-plugin/` directly. Keep common metadata, assets, commands, and prompts under `assistant/` so host manifests can reference Canon-owned shared material without duplicating skill content.

## Complexity Tracking

No constitution deviations are required for this feature.
