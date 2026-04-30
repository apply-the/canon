# Implementation Plan: Distribution Channels Beyond GitHub Releases

**Branch**: `025-distribution-channels` | **Date**: 2026-04-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/025-distribution-channels/spec.md`

## Summary

Deliver Canon's first package-manager distribution slice for the `0.25.0`
release by extending the existing release workflow to emit a verified
machine-readable distribution metadata artifact and a generated Homebrew
formula artifact from the canonical GitHub Release bundle. The implementation
will reuse the current archive packaging and release-surface verification flow,
optionally synchronize the formula to a dedicated Homebrew tap repository,
preserve the manual direct-download fallback, and shape the metadata contract
so later `winget` and Scoop work can consume the same verified asset
inventory.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact because the feature extends release and
installation surfaces, adds channel-facing metadata and optional external tap
automation, and can affect how users acquire Canon across macOS and Linux, but
it leaves Canon's core runtime, governed modes, and execution semantics
unchanged  
**Scope In**: feature `025` planning and release artifacts; distribution
metadata generation from the verified release bundle; Homebrew formula
rendering; optional dedicated tap synchronization or artifact-only fallback;
release-surface validation for metadata and formula correctness; install
documentation updates; and focused regression coverage for the new channel
surface  
**Scope Out**: `winget`; Scoop; `apt` or Debian packaging; replacing GitHub
Releases as the canonical source of truth; changing Canon's runtime behavior;
and introducing a second build or packaging pipeline outside the existing
release flow

**Invariants**:

- GitHub Releases remain the single source of truth for Canon distribution
  artifacts, checksums, and canonical filenames.
- Homebrew must consume the already packaged release archives instead of
  rebuilding Canon from a separate path.
- Direct archive installation remains documented and supported as a fallback.
- Canon runtime behavior, mode semantics, `.canon/` persistence, and governed
  packet workflows remain unchanged.

**Decision Log**: `specs/025-distribution-channels/decision-log.md`  
**Validation Ownership**: Generation happens through spec, plan, tasks,
workflow, script, docs, and test changes on the feature branch; validation
happens through release-surface checks, focused automated tests, optional local
Homebrew smoke validation, and a separate artifact review that compares the
generated metadata and formula against the verified release bundle  
**Approval Gates**: bounded-impact work proceeds under normal repository review;
release-maintainer review is required for workflow and tap-publication changes,
but no additional pre-implementation human gate is required

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus GitHub Actions YAML, Bash,
PowerShell, Markdown, and generated Ruby formula output  
**Primary Dependencies**: existing workspace crates `canon-cli`,
`canon-engine`, and `canon-adapters`; current release workflow in
`.github/workflows/release.yml`; existing packaging scripts in
`scripts/release/`; GitHub Release assets and checksum manifest; optional local
Homebrew CLI for smoke validation when available  
**Storage**: repository files; generated `dist/` release bundle artifacts;
GitHub Release assets; optional external Homebrew tap repository state outside
this workspace  
**Testing**: focused `cargo test --test ...` release-surface coverage, script
and workflow validation, `cargo fmt --check`,
`cargo clippy --workspace --all-targets --all-features -- -D warnings`, and
optional local Homebrew install smoke validation  
**Target Platform**: GitHub Actions release pipeline for macOS, Linux, and
Windows artifacts; Homebrew consumers on macOS and Linux; tap automation via
GitHub-hosted repository workflows  
**Project Type**: Rust CLI workspace with repo-local release automation,
artifact packaging scripts, documentation, and release-facing regression tests  
**Existing System Touchpoints**: `Cargo.toml`; `.github/workflows/release.yml`;
`.github/release-notes-template.md`; `scripts/release/package-unix.sh`;
`scripts/release/package-windows.ps1`;
`scripts/release/verify-release-surface.sh`; `README.md`; `ROADMAP.md`;
and `tests/release_021_docs.rs`, `tests/release_022_docs.rs`,
`tests/release_024_docs.rs` plus new release-surface tests for `025`  
**Performance Goals**: keep the added distribution steps linear in the number
of packaged assets, preserve current release job behavior for existing archive
outputs, and avoid any new rebuild stage for package-manager publishing  
**Constraints**: reuse the current archive names and verified release bundle;
keep tap publication optional and fail closed when metadata is inconsistent;
preserve visible fallback install guidance; and keep the metadata contract broad
enough for future Windows-oriented channels without shipping them now  
**Scale/Scope**: one release workflow extension, one new metadata contract, one
generated Homebrew formula surface, one optional tap synchronization path, one
docs update set, and one focused validation slice for release metadata and
formula correctness

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
specs/025-distribution-channels/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)

```text
.github/
├── workflows/
│   └── release.yml
└── release-notes-template.md

scripts/
└── release/
    ├── package-unix.sh
    ├── package-windows.ps1
    ├── verify-release-surface.sh
    ├── write-distribution-metadata.sh
    ├── render-homebrew-formula.sh
    └── sync-homebrew-tap.sh

packaging/
└── homebrew/
    └── canon.rb.tpl

docs/
└── examples/

README.md
ROADMAP.md

tests/
├── release_021_docs.rs
├── release_022_docs.rs
├── release_024_docs.rs
└── release_025_distribution.rs
```

**Structure Decision**: Keep the existing Rust workspace and release automation
layout. Implement the feature by extending the current release workflow,
release scripts, and release-facing tests, while adding one generated formula
template surface and one machine-readable metadata surface instead of creating a
new crate or an external packaging service.

## Complexity Tracking

No constitution violations are currently expected for this feature.
