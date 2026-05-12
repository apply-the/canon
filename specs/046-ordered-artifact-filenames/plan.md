# Implementation Plan: Ordered Artifact Filenames

**Branch**: `046-ordered-artifact-filenames` | **Date**: 2026-05-12 | **Spec**: `specs/046-ordered-artifact-filenames/spec.md`
**Input**: Feature specification from `specs/046-ordered-artifact-filenames/spec.md`

## Summary

Add a two-digit numeric prefix (`01-`, `02-`, ...) to every artifact filename Canon emits so that packet directories sort in reading order rather than alphabetically. The ordering derives from the `artifact_families` array in each mode's `ModeProfile`. Publish, manifest, and summary paths update to use the prefixed filenames. No artifact content, mode semantics, or governance behavior changes.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact; touches artifact contract and rendering across all 16 modes but preserves content and governance semantics.
**Scope In**: artifact filename emission, artifact contract, markdown renderer, publish path handling, manifest references, documentation, tests, changelog, and roadmap.
**Scope Out**: artifact content, mode semantics, gate logic, approval flow, governance adapter JSON shape, CLI argument surface, skill file behavior.

**Invariants**:

- Artifact content stays identical; only the filename prefix changes.
- Packet directory layout under `.canon/artifacts/` and publish destinations stays compatible.
- Reading order matches `artifact_families` in the mode profile.

**Decision Log**: `specs/046-ordered-artifact-filenames/decision-log.md`
**Validation Ownership**: automated tests generate coverage evidence; human reviewer validates reading-order correctness.
**Approval Gates**: none beyond standard PR review for bounded-impact work.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024
**Primary Dependencies**: existing workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; `serde`, `serde_json`, `toml`, `thiserror`, `tracing`.
**Storage**: local filesystem under `.canon/`
**Testing**: `cargo nextest run`, `cargo llvm-cov`
**Target Platform**: macOS, Linux, Windows
**Project Type**: CLI
**Existing System Touchpoints**: `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`, `crates/canon-engine/src/domain/mode.rs`, publish logic, all integration and contract tests referencing artifact filenames.
**Performance Goals**: no measurable change to CLI responsiveness.
**Constraints**: two-digit prefix (`01`-`99`); current max is 15 artifacts per mode.
**Scale/Scope**: 16 modes, up to 15 artifacts per mode.

## Constitution Check

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
specs/046-ordered-artifact-filenames/
├── spec.md
├── plan.md
├── research.md
├── decision-log.md
├── quickstart.md
├── validation-report.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
  canon-engine/
    src/
      artifacts/
        contract.rs     # artifact contract with filename definitions
        markdown.rs     # markdown rendering with filename emission
      domain/
        mode.rs         # mode profiles with artifact_families ordering
      orchestrator/
        service/
          mode_shaping.rs  # architecture-specific artifact emission
tests/
  integration/          # integration tests referencing artifact filenames
  contract/             # contract tests for artifact emission
  snapshots/            # snapshot files with expected filenames
```

## Implementation Phases

### Phase 1: Core filename prefixing

1. Add a helper function in `canon-engine` that maps an artifact slug and its position in the mode's `artifact_families` to a prefixed filename (`format!("{:02}-{}", index + 1, slug)`).
2. Update `contract.rs` to emit prefixed filenames.
3. Update `markdown.rs` to use prefixed filenames in rendering and file writes.
4. Update `mode_shaping.rs` for architecture-specific artifact emission (Mermaid sidecars share prefix with their companion).

### Phase 2: Manifest and publish updates

1. Update manifest and metadata JSON emission to reference prefixed filenames.
2. Update publish logic to copy prefixed filenames to the destination.
3. Update run and status summary rendering to reference the prefixed primary artifact.

### Phase 3: Tests and documentation

1. Update all integration and contract tests to expect prefixed filenames.
2. Update snapshot files.
3. Update docs: `modes.md`, `README.md`, changelog, and roadmap.
4. Run full validation suite.
