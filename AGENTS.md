# Canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-19

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

- Rust 1.95.0, Edition 2024
- `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`
- `thiserror`, `tracing`, `tracing-subscriber`, `uuid`, `time`

## Active Modes and Risk Profile

- Full v0.1 depth: `requirements`, `brownfield-change`, `pr-review`
- First-class modeled modes: `discovery`, `system-shaping`, `architecture`,
  `implementation`, `refactor`, `verification`, `review`, `incident`,
  `migration`
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
- 007-review-mode-completion: Added Rust 1.95.0 + existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, `time`
- 006-analysis-expansion: Added Rust 1.95.0, Edition 2024 + clap, serde, serde_json, serde_yaml, toml, thiserror,
- 005-cli-release-ux: Added Rust 1.95.0 workspace, Markdown documentation, + existing `clap`, `serde`, `serde_json`,
  model, `.canon/` persistence model, and CLI plus runtime filesystem contracts

<!-- MANUAL ADDITIONS START -->
## Codex Skills Frontend

- Repo-local Canon skills are authored for Codex and also usable in compatible
  Copilot environments that load `.agents/skills/`.
- Available-now Codex skills: `canon-init`, `canon-discovery`,
  `canon-requirements`, `canon-system-shaping`, `canon-architecture`,
  `canon-status`, `canon-inspect-invocations`, `canon-inspect-evidence`,
  `canon-inspect-artifacts`, `canon-inspect-clarity`, `canon-approve`, `canon-resume`,
  `canon-brownfield`, `canon-pr-review`.
- Discoverable support-state skills MUST NOT fabricate Canon runs, run ids,
  approvals, evidence, or CLI output.
- Skill validation commands:
  - `/bin/bash scripts/validate-canon-skills.sh`
  - `pwsh -File scripts/validate-canon-skills.ps1` when PowerShell is available
<!-- MANUAL ADDITIONS END -->
