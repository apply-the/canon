# Canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-05-29

## Governing Constitution

- Work MUST declare mode, risk, scope boundaries, invariants, and durable
  artifacts before execution.
- Generation and validation remain separate phases with separate evidence.
- Decisions, approvals, and traces are part of the system of record under
  `.canon/`.

## Active Technologies

- Rust 1.96.0, Edition 2024
- `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`
- `thiserror`, `tracing`, `tracing-subscriber`, `uuid`, `time`
- Local filesystem under `.canon/` for runtime artifacts and evidence
- Repository files under `docs/`, `specs/`, `.agents/skills/`, etc.

## Active Modes and Risk Profile

- Full v0.1 depth: `requirements`, `discovery`, `system-shaping`,
  `architecture`, `backlog`, `change`, `implementation`, `refactor`,
  `verification`, `review`, `pr-review`, `incident`,
  `security-assessment`, `migration`
- First-class modeled modes: `discovery`, `requirements`, `system-shaping`,
  `architecture`, `backlog`, `change`, `implementation`, `refactor`,
  `verification`, `review`, `pr-review`, `incident`,
  `security-assessment`, `migration`
- Current planning feature risk: `Systemic Impact`

## Project Structure

```text
defaults/
  methods/
  policies/
crates/
  canon-cli/
  canon-engine/
  canon-adapters/
tests/
  integration/
  contract/
  fixtures/
  snapshots/
```

## Specs rules
Crate versioning follows Semantic Versioning.
Before 1.0.0, breaking changes MAY occur in minor versions.

## Commands

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo nextest run`
- `cargo deny check licenses advisories bans sources`
- Patch-coverage helpers live under `scripts/common/coverage/`; prefer `intersect_patch_coverage.py` when the question is about uncovered diff lines rather than full-file coverage.

## Verification Expectations

- No run may progress without mode, risk, zone, and artifact contract.
- `Systemic Impact` or `Red` zone work requires explicit human ownership.
- Mutating red-zone execution is recommendation-only in v0.1.
- Verification evidence must be linked from the run manifest and artifact set.

## Decision Log References

- `specs/001-canon-spec/decision-log.md`

## Recent Changes
- 062-clarify-run-refinement: Added Rust 1.96.0, Edition 2024; Markdown documentation and templates; existing Spec Kit shell helpers + workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local methods, templates, and skill-source documents
- 061-skill-runtime-contracts: Added Bash 5.x (macOS/Linux), PowerShell 7.x (cross-platform) + `jq` (JSON validation), `canon` CLI (version
- 060-pr-review-anchors: Added Rust 1.96.0, Edition 2024, plus Markdown contract and planning artifacts + existing workspace crates `canon-engine`, `canon-cli`, and `canon-adapters`; `serde`, `strum_macros`, `thiserror`, `toml`, `tracing`, `uuid`, and `time` already used by the workspace

<!-- MANUAL ADDITIONS START -->
## Codex Skills Frontend

- Repo-local Canon skills are authored for Codex and also usable in compatible
  Copilot environments that load `.agents/skills/`.
- Available-now Codex skills: `canon-init`, `canon-discovery`,
  `canon-requirements`, `canon-system-shaping`, `canon-architecture`,
  `canon-backlog`,
  `canon-status`, `canon-inspect-invocations`, `canon-inspect-evidence`,
  `canon-inspect-artifacts`, `canon-inspect-clarity`, `canon-approve`, `canon-resume`,
  `canon-change`, `canon-review`, `canon-verification`,
  `canon-implementation`, `canon-refactor`, `canon-incident`,
  `canon-security-assessment`, `canon-system-assessment`, `canon-migration`, `canon-pr-review`.
- Discoverable support-state skills MUST NOT fabricate Canon runs, run ids,
  approvals, evidence, or CLI output.
- Skill validation commands:
  - `/bin/bash scripts/validate-canon-skills.sh`
  - `pwsh -File scripts/validate-canon-skills.ps1` when PowerShell is available

## Rust Language Rules

- AI-visible Rust language rules live in
  `.agents/skills/canon-shared/references/rust-language-rules.md`.
- Rust code outside `main.rs`, `#[cfg(test)]`, and files under `tests/` MUST
  NOT introduce panic-prone control flow such as `unwrap`, `expect`,
  `panic!`, `todo!`, `unimplemented!`, `unreachable!`, or assert-family
  runtime guards; use explicit error propagation instead.
- Stable serialized or deserialized shapes in Rust code outside `main.rs`,
  `#[cfg(test)]`, and files under `tests/` MUST use typed `struct` or
  `enum` models with `serde` derives instead of ad hoc `serde_json::Map`
  assembly, repeated raw field-name strings, or stable `json!` object
  construction.

## Clean Code & Modularity (Strict Enforcement)

- **NO GIGANTIC FILES**: Do not dump all logic into a single massive file. If a module grows complex, extract helpers, algorithms, and state transitions into private submodules (`pub(crate)`).
- **APPLY DESIGN PATTERNS**: Do not use monolithic match statements or procedural god-functions. Extract responsibilities using appropriate design patterns (e.g. Builder, Strategy, Dependency Injection). Keep business logic strictly isolated from I/O and HTTP/CLI transport boundaries.
- **ZERO MAGIC STRINGS/NUMBERS**: You MUST NOT use magic strings or magic numbers in domain logic, protocol handling, persistence, configuration, CLI contracts, timeouts, retry limits, or serialization paths. Extract them into named `const` items or typed `enum`s/newtypes owned by the relevant module or type.
- **EXTRACT HELPERS PROACTIVELY**: Aim for <50 lines per function. If you need a comment to explain the middle of a function, extract that block into a well-named helper function.
- **NO DEAD CODE**: Remove all commented-out code, unused variables, and unreachable branches immediately. `git` remembers.
- **WHY NOT WHAT**: Documentation and comments must explain the *why*, business constraints, and invariants, not narrate the *what*.
- **COMPREHENSIVE DOCUMENTATION**: Every folder/module MUST have a module-level doc comment (e.g. `//!` in `mod.rs` or `<module_name>.rs`) explaining its purpose, and these docs must be kept up to date. Furthermore, all structs, public functions, enums, and constants MUST have clear and up-to-date doc comments (`///`).
- **LOGGING & OUTPUT BOUNDARIES**: Log at major state-transition decision points using structured `tracing` spans/events. Always include reproducible context (IDs) but NEVER log secrets, tokens, or PII. Maintain strict separation between presentation and core logic: use `println!` or `eprintln!` ONLY in presentation layers (e.g., `cli.rs`, `init.rs`). For orchestrator, core logic, and adapters, NEVER use `println!`. User-facing messages must be propagated up to the CLI layer via return values (e.g., `Result<T, Error>`).
- **CONCURRENCY**: Avoid `Arc<Mutex<T>>` lock-contention. Prefer message-passing (channels) or immutable data snapshots to share state across async boundaries.
## Pre-Commit & Code Quality

- **CLIPPY**: After any code modification or run, you MUST check and fix any `clippy` issues by running `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **CARGO FMT**: After any code modification or run, you MUST run `cargo fmt` to ensure the codebase remains correctly formatted.

## Repo Safety Rules

- NEVER save fully qualified paths in any file, but use relative paths and only related to this git repo. Other repos must be referenced by url (or if working locally by git project name).
- NEVER run `canon` CLI commands against this repository root as a working
	workspace. Doing so writes workspace-local `.canon/` session state,
	pollutes tracked repo history, and can dirty the developer worktree. Use a
	temporary fixture workspace, isolated temp repo, or explicit test harness
	instead.
<!-- MANUAL ADDITIONS END -->
