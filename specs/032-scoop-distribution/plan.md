# Implementation Plan: Scoop Distribution Follow-On

**Branch**: `032-scoop-distribution` | **Date**: 2026-05-01 | **Spec**: `specs/032-scoop-distribution/spec.md`
**Input**: Feature specification from `/specs/032-scoop-distribution/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Deliver the next concrete distribution slice by turning the existing canonical
Windows release archive into a reviewable Scoop manifest artifact while keeping
GitHub Releases as the single source of truth. The implementation will extend
the current distribution metadata and release-surface verification helpers so
the Windows asset advertises both Windows package-manager channels, add a
deterministic Scoop manifest renderer from that metadata, wire the manifest into
the release workflow, update install and maintainer docs plus the roadmap and
changelog for `0.32.0`, and add a focused Rust regression test that exercises
the release bundle, generated manifest, and documentation surfaces together.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; the change modifies packaging,
workflow, docs, roadmap, and versioning surfaces in an existing system, but it
does not alter Canon runtime behavior, execution policy, or trust boundaries.  
**Scope In**: Scoop manifest generation and verification from canonical release
metadata; release workflow wiring for the Scoop artifact; install, release,
roadmap, and changelog documentation updates; `0.32.0` version alignment;
focused release-surface and documentation tests; validation evidence and
coverage for changed or new Rust files.  
**Scope Out**: Homebrew or `winget` redesign, Debian packaging, external bucket
automation, replacement of GitHub Releases as the artifact source, new runtime
modes, or changes to `.canon/`, policy, approval, or execution semantics.

**Invariants**:

- GitHub Releases remain the canonical source for Canon versioning, Windows
  binaries, filenames, and checksums.
- The existing Windows zip remains the installation payload for package-manager
  channels; Scoop must not introduce a second Windows build pipeline.
- Existing `winget`, Homebrew, and direct-download installation paths remain
  valid and documented.
- The feature stays bounded to repository-owned release, packaging, versioning,
  and documentation surfaces; it must not widen into runtime behavior.

**Decision Log**: `specs/032-scoop-distribution/decision-log.md`  
**Validation Ownership**: Generation work updates release scripts, workflow,
packaging templates, docs, version references, and feature artifacts; validation
is performed through a focused Rust release test, direct shell validation of the
release scripts, coverage collection for changed Rust files, formatting, clippy,
and an independent review of scope boundaries and install guidance.  
**Approval Gates**: No special human approval gate beyond normal review is
required for bounded-impact work; independent validation evidence remains
mandatory before completion.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Bash, PowerShell release helpers, GitHub Actions YAML, and JSON packaging metadata  
**Primary Dependencies**: existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), `jq`, `shasum`, `unzip`, GitHub Actions release automation, and Scoop manifest JSON conventions  
**Storage**: repository files plus ephemeral `dist/` release artifacts and generated `lcov.info` during validation  
**Testing**: `cargo test`, focused release and documentation tests, direct shell validation, `cargo llvm-cov`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`  
**Target Platform**: GitHub-hosted release automation and Windows x86_64 installation through `winget` and Scoop with GitHub Releases as the binary host  
**Project Type**: Rust CLI with governed release automation, packaging helpers, and documentation surfaces  
**Existing System Touchpoints**: `.github/workflows/release.yml`, `.github/release-notes-template.md`, `scripts/release/write-distribution-metadata.sh`, `scripts/release/verify-release-surface.sh`, `scripts/release/render-winget-manifests.sh`, new `scripts/release/render-scoop-manifest.sh`, `packaging/winget/`, new `packaging/scoop/`, `README.md`, `docs/guides/publishing-to-winget.md`, new `docs/guides/publishing-to-scoop.md`, `CHANGELOG.md`, `ROADMAP.md`, `Cargo.toml`, runtime-compatibility references, new `tests/release_032_scoop_distribution.rs`, and `tests/integration/skills_bootstrap.rs`  
**Performance Goals**: keep release-surface generation deterministic and bounded to the existing packaging flow, with no manual checksum or URL derivation for Scoop publication artifacts  
**Constraints**: GitHub Releases stay canonical, the Windows zip stays the source asset, Scoop submission remains manual to the bucket surface, archive fallback stays documented, and output filenames remain stable enough for release notes and verifier checks  
**Scale/Scope**: one new manifest type, one new renderer script, one new packaging template, one maintainer guide, one focused Rust regression test, one version bump, and bounded updates to existing release and documentation surfaces

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
specs/032-scoop-distribution/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── distribution-metadata.md
│   └── scoop-manifest.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
├── release-notes-template.md
└── workflows/
    └── release.yml

scripts/
└── release/
    ├── package-windows.ps1
    ├── render-homebrew-formula.sh
    ├── render-winget-manifests.sh
    ├── render-scoop-manifest.sh
    ├── verify-release-surface.sh
    └── write-distribution-metadata.sh

packaging/
├── homebrew/
├── winget/
└── scoop/
    └── canon.json.tpl

docs/
└── guides/
    ├── publishing-to-winget.md
    └── publishing-to-scoop.md

tests/
├── integration/
│   └── skills_bootstrap.rs
└── release_032_scoop_distribution.rs

README.md
CHANGELOG.md
ROADMAP.md
Cargo.toml
defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml
.agents/skills/canon-shared/references/runtime-compatibility.toml
```

**Structure Decision**: Keep the feature localized to the existing release
automation surface by adding one new Scoop renderer, one JSON template, one
maintainer guide, one focused release test, and bounded documentation and
versioning updates rather than introducing a new crate or generic packaging
subsystem.

## Complexity Tracking

No constitution deviations are currently identified.
