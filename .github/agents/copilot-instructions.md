# canon Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-06-02

## Governing Constitution

- Work MUST follow an explicit mode and artifact trail.
- Risk classification, scope boundaries, invariants, and decision traceability
  are required.
- Completion claims require layered validation evidence.

## Active Technologies

- Rust 1.96.0, edition 2024; Markdown plus machine-checkable contract artifacts in YAML/TOML/JSON + Existing workspace dependencies only (`serde`, `serde_json`, `serde_yaml`, `toml`, `strum`, `strum_macros`, `thiserror`, `time`, `tracing`, `uuid`, `assert_cmd`, `predicates`, `tempfile`) and Rust standard-library filesystem/path APIs; no new external runtime dependencies are planned for this slice (065-reasoning-posture-v2)

## Active Modes & Risk Profile

- Rust 1.96.0, edition 2024; Markdown plus machine-checkable contract artifacts in YAML/TOML/JSON + Existing workspace dependencies only (`serde`, `serde_json`, `serde_yaml`, `toml`, `strum`, `strum_macros`, `thiserror`, `time`, `tracing`, `uuid`, `assert_cmd`, `predicates`, `tempfile`) and Rust standard-library filesystem/path APIs; no new external runtime dependencies are planned for this slice (065-reasoning-posture-v2)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Verification Expectations

[VALIDATION COMMANDS, REVIEW GATES, AND EVIDENCE PATHS]

## Decision Log References

[LINKS TO ACTIVE DECISION LOGS]

## Recent Changes

- 065-reasoning-posture-v2: Added Rust 1.96.0, edition 2024; Markdown plus machine-checkable contract artifacts in YAML/TOML/JSON + Existing workspace dependencies only (`serde`, `serde_json`, `serde_yaml`, `toml`, `strum`, `strum_macros`, `thiserror`, `time`, `tracing`, `uuid`, `assert_cmd`, `predicates`, `tempfile`) and Rust standard-library filesystem/path APIs; no new external runtime dependencies are planned for this slice

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
