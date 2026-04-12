# Canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-30

## Governing Constitution

- Work MUST declare mode, risk, scope boundaries, invariants, and durable
  artifacts before execution.
- Generation and validation remain separate phases with separate evidence.
- Decisions, approvals, and traces are part of the system of record under
  `.canon/`.

## Active Technologies
- Rust 1.94.1, Edition 2024 + existing `clap`, `serde`, `serde_json`, `serde_yaml`, (002-governed-execution-adapters)
- local filesystem only under `.canon/`; TOML for run and approval (002-governed-execution-adapters)
- Markdown `SKILL.md` files, repo-local shell helpers + installed `canon` binary, existing repo-local (003-codex-skills-frontend)
- repo-local files under `.agents/skills` plus existing `.canon/` (003-codex-skills-frontend)
- Markdown `SKILL.md` files, repo-local Bash and PowerShell helpers, installed `canon` binary, Git local ref inspection commands (004-ref-safe-binding)
- repo-local skill files under `.agents/skills`; no new persistent storage beyond existing `.canon/` (004-ref-safe-binding)
- Rust 1.94.1 workspace, Markdown documentation, + existing `clap`, `serde`, `serde_json`, (005-cli-release-ux)
- repository files for workflow and documentation changes, GitHub (005-cli-release-ux)

- Rust 1.94.1, Edition 2024
- `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`
- `thiserror`, `tracing`, `tracing-subscriber`, `uuid`, `time`

## Active Modes and Risk Profile

- Full v0.1 depth: `requirements`, `brownfield-change`, `pr-review`
- First-class modeled modes: `discovery`, `greenfield`, `architecture`,
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
- `cargo clippy --all-targets --all-features -- -D warnings`
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
- 005-cli-release-ux: Added Rust 1.94.1 workspace, Markdown documentation, + existing `clap`, `serde`, `serde_json`,
- 004-ref-safe-binding: Added Markdown `SKILL.md` files, repo-local Bash and PowerShell + installed `canon` binary, Git command-line ref
- 003-codex-skills-frontend: Added Markdown `SKILL.md` files, repo-local shell helpers + installed `canon` binary, existing repo-local
  model, `.canon/` persistence model, and CLI plus runtime filesystem contracts

<!-- MANUAL ADDITIONS START -->
## Codex Skills Frontend

- Repo-local Canon skills are authored for Codex and also usable in compatible
  Copilot environments that load `.agents/skills/`.
- Available-now Codex skills: `canon-init`, `canon-requirements`,
  `canon-status`, `canon-inspect-invocations`, `canon-inspect-evidence`,
  `canon-inspect-artifacts`, `canon-approve`, `canon-resume`,
  `canon-brownfield`, `canon-pr-review`.
- Discoverable support-state skills MUST NOT fabricate Canon runs, run ids,
  approvals, evidence, or CLI output.
- Skill validation commands:
  - `/bin/bash scripts/validate-canon-skills.sh`
  - `pwsh -File scripts/validate-canon-skills.ps1` when PowerShell is available
<!-- MANUAL ADDITIONS END -->
