# Implementation Plan: Standard ADR Publish Artifacts

**Branch**: `043-standard-adr-publish` | **Date**: 2026-05-10 | **Spec**: [specs/043-standard-adr-publish/spec.md](specs/043-standard-adr-publish/spec.md)
**Input**: Feature specification from `/specs/043-standard-adr-publish/spec.md`

## Summary

Extend Canon's publish surface so `architecture` emits a standard Nygard ADR into a durable `docs/adr/` registry by default, while `change` and `migration` can opt into the same ADR export through the publish command when the operator wants a durable repository decision record. The implementation remains additive: existing runtime packets and publish destinations stay authoritative, ADR files are synthesized from the already-governed packet artifacts at publish time, and unsupported modes remain outside the ADR registry.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact, because the slice changes public publish behavior, release-facing documentation, and repository decision outputs without expanding destructive execution authority or altering approval semantics.  
**Scope In**: publish-time ADR synthesis, `publish` CLI surface for opt-in ADR export, ADR numbering and path rules, supported-mode gating, version bump to `0.43.0`, documentation or skill updates, and focused validation including touched-file coverage closeout.  
**Scope Out**: new execution modes, full ADR lifecycle editing workflows, unsupported-mode ADR export, runtime evidence-model changes, and any weakening of current publish or approval semantics.

**Invariants**:

- Existing `.canon/` artifacts and the current mode publish outputs remain authoritative; ADR files are additive projections created during publish.
- `architecture` is default-on for ADR publication, `change` and `migration` are explicit opt-in, and unsupported modes never create ADR entries.
- Exported ADR content must preserve missing-context honesty from the source packet rather than fabricating standard-looking decision text.

**Decision Log**: `specs/043-standard-adr-publish/decision-log.md`  
**Validation Ownership**: Generation happens through spec, code, CLI, and documentation changes in the workspace; validation happens through focused publish/contract/integration tests, touched-file coverage inspection, formatting or lint checks, and a manual ADR-to-packet readback captured in `validation-report.md`.  
**Approval Gates**: No new human approval gate beyond bounded-impact feature review; runtime publish approval rules for governed packets remain unchanged.

## Technical Context

**Language/Version**: Rust 1.96.0 workspace plus Markdown documentation and Spec Kit feature artifacts.  
**Primary Dependencies**: existing workspace crates `canon-cli`, `canon-engine`, and `canon-adapters` with `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`.  
**Storage**: local filesystem under `.canon/` for runtime artifacts plus repository-published outputs under `docs/`, `specs/`, and the new `docs/adr/` registry.  
**Testing**: focused Rust CLI, publish, contract, and integration tests; `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo nextest run`; and touched-file coverage inspection with `cargo llvm-cov`.  
**Target Platform**: local-first CLI workflows on macOS, Linux, and Windows with repository-local publish destinations.  
**Project Type**: Rust CLI workspace with file-backed governed runtime artifacts and structured publish surfaces.  
**Existing System Touchpoints**: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, `README.md`, `docs/guides/modes.md`, `.agents/skills/canon-architecture/SKILL.md`, `.agents/skills/canon-change/SKILL.md`, `.agents/skills/canon-migration/SKILL.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, `defaults/embedded-skills/canon-change/skill-source.md`, `defaults/embedded-skills/canon-migration/skill-source.md`, `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands/publish.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, supported-mode artifact contracts, and publish regression tests under `tests/` and `crates/canon-cli/src/commands/publish.rs`.  
**Performance Goals**: publish remains effectively linear in the number of artifacts plus one extra ADR write, with no meaningful regression in normal packet publish latency.  
**Constraints**: version bump happens before behavior edits, `architecture` ADR publication defaults to accepted entries, `change` and `migration` require an explicit publish flag, unsupported modes must reject or ignore ADR export deterministically, and every new or modified Rust source file touched by the slice must finish at or above 95% line coverage.  
**Scale/Scope**: one bounded feature slice touching the publish command, publish orchestrator, decision-mode documentation, release surfaces, and a focused set of tests.

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
specs/043-standard-adr-publish/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── adr-publish-surface.md
└── tasks.md
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
Cargo.toml
Cargo.lock
CHANGELOG.md
README.md
docs/
├── adr/
└── guides/
  └── modes.md
.agents/
└── skills/
  ├── canon-architecture/
  │   └── SKILL.md
  ├── canon-change/
  │   └── SKILL.md
  └── canon-migration/
      └── SKILL.md
defaults/
└── embedded-skills/
  ├── canon-architecture/
  │   └── skill-source.md
  ├── canon-change/
  │   └── skill-source.md
  └── canon-migration/
      └── skill-source.md
crates/
├── canon-cli/
│   └── src/
│       ├── app.rs
│       └── commands/
│           └── publish.rs
└── canon-engine/
  └── src/
    ├── artifacts/
    │   └── contract.rs
    └── orchestrator/
      └── publish.rs
tests/
├── contract/
├── integration/
└── *.rs publish or runtime regression files
```

**Structure Decision**: Keep the feature inside the existing Rust workspace and publish pipeline. The CLI surface lives in `canon-cli`, the publish synthesis and registry logic lives in `canon-engine`, durable design artifacts stay under `specs/043-standard-adr-publish/`, and the generated ADR registry lives under `docs/adr/` alongside the existing documentation tree.

## Complexity Tracking

No constitution deviations are required for this feature.
