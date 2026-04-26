# Canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-26

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

- Rust 1.95.0, Edition 2024
- `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`
- `thiserror`, `tracing`, `tracing-subscriber`, `uuid`, `time`

## Active Modes and Risk Profile

- Full v0.1 depth: `requirements`, `discovery`, `system-shaping`,
  `architecture`, `backlog`, `change`, `implementation`, `refactor`,
  `verification`, `review`, `pr-review`
- First-class modeled modes: `discovery`, `requirements`, `system-shaping`,
  `architecture`, `backlog`, `change`, `implementation`, `refactor`,
  `verification`, `review`, `pr-review`, `incident`, `migration`
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

## Verification Expectations

- No run may progress without mode, risk, zone, and artifact contract.
- `Systemic Impact` or `Red` zone work requires explicit human ownership.
- Mutating red-zone execution is recommendation-only in v0.1.
- Verification evidence must be linked from the run manifest and artifact set.

## Decision Log References

- `specs/001-canon-spec/decision-log.md`

## Recent Changes
- 019-authoring-specialization-remaining: Added Rust 1.95.0, Edition 2024. + existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.
- 018-architecture-adr-options: Added Rust 1.95.0, Edition 2024. + existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.
- 017-domain-boundary-design: Added Rust 1.95.0, Edition 2024. + existing workspace crates (`canon-engine`, `canon-cli`, `canon-adapters`) with `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`.
  model, `.canon/` persistence model, and CLI plus runtime filesystem contracts

<!-- MANUAL ADDITIONS START -->
## Codex Skills Frontend

- Repo-local Canon skills are authored for Codex and also usable in compatible
  Copilot environments that load `.agents/skills/`.
- Available-now Codex skills: `canon-init`, `canon-discovery`,
  `canon-requirements`, `canon-system-shaping`, `canon-architecture`,
  `canon-backlog`,
  `canon-status`, `canon-inspect-invocations`, `canon-inspect-evidence`,
  `canon-inspect-artifacts`, `canon-inspect-clarity`, `canon-approve`, `canon-resume`,
  `canon-change`, `canon-pr-review`.
- Discoverable support-state skills MUST NOT fabricate Canon runs, run ids,
  approvals, evidence, or CLI output.
- Skill validation commands:
  - `/bin/bash scripts/validate-canon-skills.sh`
  - `pwsh -File scripts/validate-canon-skills.ps1` when PowerShell is available
<!-- MANUAL ADDITIONS END -->
