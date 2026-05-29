# Canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-05-29

## Governing Constitution

- Work MUST declare mode, risk, scope boundaries, invariants, and durable
  artifacts before execution.
- Generation and validation remain separate phases with separate evidence.
- Decisions, approvals, and traces are part of the system of record under
  `.canon/`.

## Active Technologies
- Rust 1.95.0, Edition 2024 + existing `clap`, `serde`, `serde_json`, `serde_yaml`, (002-governed-execution-adapters)
- local filesystem only under `.canon/`; TOML for run and approval (002-governed-execution-adapters)
- Markdown `SKILL.md` files, repo-local shell helpers + installed `canon` binary, existing repo-local (003-codex-skills-frontend)
- repo-local files under `.agents/skills` plus existing `.canon/` (003-codex-skills-frontend)
- Markdown `SKILL.md` files, repo-local Bash and PowerShell helpers, installed `canon` binary, Git local ref inspection commands (004-ref-safe-binding)
- repo-local skill files under `.agents/skills`; no new persistent storage beyond existing `.canon/` (004-ref-safe-binding)
- Rust 1.95.0 workspace, Markdown documentation, + existing `clap`, `serde`, `serde_json`, (005-cli-release-ux)
- repository files for workflow and documentation changes, GitHub (005-cli-release-ux)
- Rust 1.95.0, Edition 2024 + clap, serde, serde_json, serde_yaml, toml, thiserror, (006-analysis-expansion)
- Local filesystem under `.canon/` (TOML for manifests, Markdown for (006-analysis-expansion)
- Rust 1.95.0 + existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time` (007-review-mode-completion)
- local filesystem under `.canon/`, Markdown artifacts, TOML manifests and policies (007-review-mode-completion)
- local filesystem under `.canon/`, TOML manifests and `context.toml`, Markdown artifacts, repo-local skill source documents under `defaults/` and `.agents/skills/` (008-mode-context-split)
- Rust 1.95.0, Edition 2024 + `clap`, `serde`, `serde_json`, `serde_yaml`, (009-run-id-display)
- Local filesystem under `.canon/` only; TOML manifests, Markdown (009-run-id-display)
- Rust 1.95.0, Edition 2024 + `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time` (010-controlled-execution-modes)
- Local filesystem under `.canon/`, TOML manifests and `context.toml`, Markdown artifacts, repo-local skill source documents under `defaults/embedded-skills/` and `.agents/skills/` (010-controlled-execution-modes)
- Local filesystem under `.canon/` for runtime artifacts and (013-pr-review-comments)
- Local filesystem under `.canon/` for runtime artifacts and evidence, plus published markdown under `docs/incidents/` and `docs/migrations/` (014-high-risk-ops)
- Rust 1.95.0, Edition 2024. + existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`); `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`. (015-architecture-c4)
- local filesystem under `.canon/`; no schema or layout changes; published architecture artifacts continue to land in their existing publish destination. (015-architecture-c4)
- Rust 1.95.0, Edition 2024. + existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`. (016-mode-authoring-specialization)
- repository files plus existing `.canon/` runtime persistence; no schema or layout changes. (016-mode-authoring-specialization)
- Rust 1.95.0, Edition 2024. + existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`. (018-architecture-adr-options)
- Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts + workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local skill validation scripts (021-artifact-shapes-personas)
- Repository files plus existing `.canon/` runtime filesystem; no new persistent schema (021-artifact-shapes-personas)
- Rust 1.95.0 workspace plus Markdown skill sources, Bash runtime checks, and documentation artifacts + workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; existing shell and filesystem adapters; repo-local skill validation scripts (024-supply-chain-legacy)
- Repository files plus existing `.canon/` runtime filesystem; scanner outputs and machine-readable SBOM references persist as normal run artifacts or payload references without a new persistent schema (024-supply-chain-legacy)
- Rust 1.95.0 workspace plus Bash and PowerShell release helpers and GitHub Actions YAML + existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), shell tooling (`jq`, `shasum`, `unzip`), and Windows Package Manager manifest schema v1.12.0 (026-winget-distribution)
- repository files plus ephemeral release artifacts in `dist/` during packaging and validation (026-winget-distribution)
- Rust 1.95.0 workspace plus GitHub Actions YAML, Bash, + existing workspace crates `canon-cli`, (025-distribution-channels)
- repository files; generated `dist/` release bundle artifacts; (025-distribution-channels)
- Rust 1.95.0 workspace plus Markdown documentation and Spec Kit artifacts + workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; Spec Kit scripts under `.specify/scripts/bash/` (029-publish-destinations)
- Rust 1.95.0 workspace plus Bash, PowerShell release helpers, GitHub Actions YAML, and JSON packaging metadata + existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), `jq`, `shasum`, `unzip`, GitHub Actions release automation, and Scoop manifest JSON conventions (032-scoop-distribution)
- repository files plus ephemeral `dist/` release artifacts and generated `lcov.info` during validation (032-scoop-distribution)
- Rust 1.95.0 workspace plus Markdown skill sources and documentation artifacts + workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time` (033-reasoning-evidence-clarity)
- Rust 1.95.0 workspace plus Markdown and JSON-facing contract artifacts + workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time` (035-governance-adapter-surface)
- Rust 1.95.0 workspace plus Bash and PowerShell release helpers, JSON metadata, and Markdown documentation artifacts + existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), `jq`, `shasum`, `unzip`, existing packaging templates, and GitHub Actions release automation (036-release-provenance-integrity)
- Rust 1.95.0 workspace plus Markdown skill sources, templates, and documentation artifacts + existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), existing `serde`/`serde_json` surfaces, shared architecture skill documents, and existing Spec Kit scripts (037-architecture-clarification-readiness)
- repository files plus the existing `.canon/` runtime artifact layout only; no new persistence family (037-architecture-clarification-readiness)
- Rust 1.95.0 workspace plus Bash, PowerShell, and Markdown documentation artifacts + workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `serde`/`serde_json` output contracts; shared skill helper scripts under `.agents/skills` and `defaults/embedded-skills` (038-guided-run-operations)
- Rust 1.95.0 workspace plus Markdown documentation and embedded skill artifacts + workspace crates `canon-engine`, `canon-cli`, and `canon-adapters`; existing `serde` or `serde_json` output contracts; repo-local Speckit and embedded skill mirrors (039-authoring-packet-readiness)
- repository files plus the existing `.canon/` runtime layout only; no new persistence family (039-authoring-packet-readiness)
- Rust 1.95.0 workspace plus Markdown documentation + Existing workspace crates, `assert_cmd`, `predicates`, `serde_json`, `tempfile`, `toml`, and the existing Spec Kit shell scripts (040-governance-runtime-framing)
- Repository files and `.canon/` runtime semantics are documented but not structurally changed (040-governance-runtime-framing)
- Rust 1.95.0 workspace plus Markdown documentation and repo-local skills + Existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`), `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`, and repo skill validation scripts (041-prd-publish-chat)
- Local filesystem under `.canon/` plus published repository files under `specs/`, `docs/`, and `.agents/skills/` (041-prd-publish-chat)
- Rust 1.95.0 workspace plus Markdown documentation and Spec Kit feature artifacts. + existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`, plus `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`, and current release helper shell scripts. (042-visual-artifact-generation)
- local filesystem under `.canon/`, published repository docs under `docs/` and `specs/`, and repo-local feature artifacts under `specs/042-visual-artifact-generation/`. (042-visual-artifact-generation)
- Rust 1.95.0 workspace plus Markdown documentation and Spec Kit feature artifacts. + existing workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`. (043-standard-adr-publish)
- local filesystem under `.canon/` for runtime artifacts plus repository-published outputs under `docs/`, `specs/`, and the new `docs/adr/` registry. (043-standard-adr-publish)
- Rust 1.95.0 workspace plus JSON, Markdown, Bash, and SVG repository assets. + existing workspace crates and dev dependencies including `serde_json`; no new external crates are planned. (044-assistant-plugin-packages)
- repository files only: hidden host package folders, shared assistant metadata under `assistant/`, docs under `docs/`, validation scripts under `scripts/`, and Spec Kit artifacts under `specs/044-assistant-plugin-packages/`. (044-assistant-plugin-packages)
- Rust 1.95.0, Edition 2024, plus Markdown/JSON/YAML repository docs and metadata + Existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time` surfaces (045-mode-publish-alignment)
- Local filesystem under `.canon/` for runtime artifacts plus repository files under `docs/`, `assistant/`, `.agents/`, `defaults/`, `tests/`, and release metadata surfaces (045-mode-publish-alignment)
- Rust 1.95.0, Edition 2024 + `clap`, `serde`, `serde_json`, `toml`, `thiserror`, `tracing`, `uuid`, `time` (048-project-memory-promotion-policy)
- Local filesystem under `.canon/` (TOML manifests, Markdown artifacts) (048-project-memory-promotion-policy)
- Rust 1.95.0, Edition 2024 + existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; `serde`, `serde_json`, `toml`, `thiserror`, `tracing`, `uuid`, `time` (049-logical-packet-ordering)
- local filesystem under `.canon/` plus repo-visible published packet directories and docs (049-logical-packet-ordering)
- Rust 1.95.0, Edition 2024 + `clap`, `serde`, `serde_json`, `thiserror`, (050-project-memory-control)
- local filesystem under `.canon/` plus repo-visible docs under (050-project-memory-control)
- Rust 1.95.0, edition 2024 + Existing workspace dependencies `serde`, `serde_json`, `serde_yaml`, `strum`, `strum_macros`, `thiserror`, `time`, `toml`, `tracing`, `uuid`, and Rust standard-library filesystem and path APIs; no new runtime dependencies planned for this slice (054-authority-zone-contract)
- Canon packet metadata and governed artifacts under `.canon/`, integration docs under `docs/integration/`, repo-facing guides under `docs/`, and feature-local artifacts under `specs/054-authority-zone-contract/` (054-authority-zone-contract)
- Canon packet metadata and governed artifacts under `.canon/`, integration docs under `docs/integration/`, repo-facing guides under `docs/`, and feature-local planning artifacts under `specs/055-adaptive-governance/` (055-adaptive-governance)
- Rust 1.95.0, Edition 2024, plus Markdown and JSON + existing workspace crates `canon-engine`, (056-semantic-artifact-contract)
- repository-published Markdown contracts under `docs/integration/` (056-semantic-artifact-contract)
- Rust 1.95.0, Edition 2024 (existing types only, no new Rust code); + Existing workspace crates `canon-engine`, `canon-cli`, (057-s7-delight-provider)
- Repository files only; no new persistent schema. (057-s7-delight-provider)
- Rust 1.95.0, Edition 2024, plus Markdown contract and planning artifacts + existing workspace crates `canon-engine`, `canon-cli`, and `canon-adapters`; `serde`, `strum_macros`, `thiserror`, `toml`, `tracing`, `uuid`, and `time` already used by the workspace (060-pr-review-anchors)
- repository planning artifacts under `specs/060-pr-review-anchors/` and existing `.canon/` runtime evidence/artifact files for `pr-review` runs (060-pr-review-anchors)
- Bash 5.x (macOS/Linux), PowerShell 7.x (cross-platform) + `jq` (JSON validation), `canon` CLI (version (061-skill-runtime-contracts)
- `.canon/hooks.toml` (TOML), preflight JSON to stdout (ephemeral), (061-skill-runtime-contracts)
- Rust 1.95.0, Edition 2024; Markdown documentation and templates; existing Spec Kit shell helpers + workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local methods, templates, and skill-source documents (062-clarify-run-refinement)
- existing `.canon/runs/<RUN_ID>/manifest.toml`, `context.toml`, `artifacts/`, and `inputs/`; published docs and templates under `docs/`, `defaults/`, and `.agents/skills/` (062-clarify-run-refinement)

- Rust 1.95.0, Edition 2024
- `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`
- `thiserror`, `tracing`, `tracing-subscriber`, `uuid`, `time`

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
- 062-clarify-run-refinement: Added Rust 1.95.0, Edition 2024; Markdown documentation and templates; existing Spec Kit shell helpers + workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local methods, templates, and skill-source documents
- 061-skill-runtime-contracts: Added Bash 5.x (macOS/Linux), PowerShell 7.x (cross-platform) + `jq` (JSON validation), `canon` CLI (version
- 060-pr-review-anchors: Added Rust 1.95.0, Edition 2024, plus Markdown contract and planning artifacts + existing workspace crates `canon-engine`, `canon-cli`, and `canon-adapters`; `serde`, `strum_macros`, `thiserror`, `toml`, `tracing`, `uuid`, and `time` already used by the workspace

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
- Rust code outside `main.rs`, `#[cfg(test)]`, and files under `tests/` MUST
  NOT introduce magic strings or magic numbers in repository logic, mode
  dispatch, protocol handling, persistence, configuration, CLI contracts, or
  serialization paths; use named constants or typed enums/newtypes owned by
  the relevant module or type.
- Stable serialized or deserialized shapes in Rust code outside `main.rs`,
  `#[cfg(test)]`, and files under `tests/` MUST use typed `struct` or
  `enum` models with `serde` derives instead of ad hoc `serde_json::Map`
  assembly, repeated raw field-name strings, or stable `json!` object
  construction.

## Clean Code & Modularity (Strict Enforcement)
- **NO GIGANTIC FILES**: Do not dump all logic into a single massive file. If a module grows complex, extract helpers, algorithms, and state transitions into private submodules (`pub(crate)`).
- **APPLY DESIGN PATTERNS**: Do not use monolithic match statements or procedural god-functions. Extract responsibilities using appropriate design patterns (e.g. Builder, Strategy, Dependency Injection). Keep business logic strictly isolated from I/O and HTTP/CLI transport boundaries.
- **ZERO MAGIC STRINGS/NUMBERS**: You MUST NOT use magic numbers, timeouts, retry limits, or repeated raw strings inline. Extract them into named `const` items or typed `enum`s.
- **EXTRACT HELPERS PROACTIVELY**: Aim for <50 lines per function. If you need a comment to explain the middle of a function, extract that block into a well-named helper function.
- **NO DEAD CODE**: Remove all commented-out code, unused variables, and unreachable branches immediately. `git` remembers.
- **WHY NOT WHAT**: Documentation and comments must explain the *why*, business constraints, and invariants, not narrate the *what*.
- **COMPREHENSIVE DOCUMENTATION**: Every folder/module MUST have a module-level doc comment (e.g. `//!` in `mod.rs` or `<module_name>.rs`) explaining its purpose, and these docs must be kept up to date. Furthermore, all structs, public functions, enums, and constants MUST have clear and up-to-date doc comments (`///`).
- **LOGGING & SECRETS**: Log at major state-transition decision points using structured `tracing` spans/events. Always include reproducible context (IDs) but NEVER log secrets, tokens, or PII.
- **CONCURRENCY**: Avoid `Arc<Mutex<T>>` lock-contention. Prefer message-passing (channels) or immutable data snapshots to share state across async boundaries.

<!-- MANUAL ADDITIONS END -->
