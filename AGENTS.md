# Canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-27

## Governing Constitution

- Work MUST declare mode, risk, scope boundaries, invariants, and durable
  artifacts before execution.
- Generation and validation remain separate phases with separate evidence.
- Decisions, approvals, and traces are part of the system of record under
  `.canon/`.

## Active Technologies

- Rust 1.94.0, Edition 2024
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

- `/Users/rt/workspace/apply-the/canon/specs/001-canon-spec/decision-log.md`

## Recent Changes

- `001-canon-spec`: established the Rust CLI workspace, typed mode
  model, `.canon/` persistence model, and CLI plus runtime filesystem contracts

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
