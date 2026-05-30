# Implementation Plan: Release Provenance And Channel Integrity

**Branch**: `PLACEHOLDER` | **Date**: PLACEHOLDER | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/036-release-provenance-integrity/spec.md`

## Summary

Deliver `0.36.0` as a release-surface hardening slice by extending Canon's
distribution metadata into an explicit provenance and per-channel contract,
tightening release-surface verification to fail closed on channel drift,
making Homebrew, Winget, and Scoop renderers consume that explicit contract,
and aligning README, publication guides, roadmap, changelog, and validation
artifacts around one canonical GitHub Release source-of-truth story.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; this work changes release metadata,
distribution renderers, verification scripts, version surfaces, docs, and
roadmap text, but it does not modify Canon runtime semantics, policy
boundaries, or persisted governed state.  
**Scope In**: canonical distribution metadata contract expansion; provenance and
channel-integrity verification; bounded updates to Homebrew, Winget, and Scoop
renderer scripts; focused Rust release-surface tests; `0.36.0` version
alignment; impacted docs plus changelog updates; roadmap cleanup; coverage,
`cargo fmt`, and `cargo clippy` closeout.  
**Scope Out**: new package-manager channels; runtime CLI or governance changes;
archive naming redesign; binary signing or external attestations; replacing
GitHub Releases as the canonical artifact host; a new packaging pipeline.

**Invariants**:

- GitHub Releases remain the single source of truth for binaries, archive
  filenames, checksums, and release notes.
- Package-manager channels continue to derive from the same canonical release
  bundle rather than rebuilding Canon through separate packaging paths.
- Core Canon runtime behavior, `.canon/` storage, publish semantics, and
  approval posture remain unchanged.
- Release-surface verification fails closed when provenance fields, channel
  contracts, generated artifacts, or documentation drift from the canonical
  bundle.

**Decision Log**: `specs/036-release-provenance-integrity/decision-log.md`  
**Validation Ownership**: Generation updates release scripts, packaging
templates, version surfaces, docs, roadmap, and feature artifacts; validation
is performed through focused Rust release tests, direct script execution,
coverage review for touched Rust files, `cargo fmt --check`, `cargo clippy`,
and an independent review of provenance or doc coherence.  
**Approval Gates**: No additional human approval gate beyond normal review is
required for bounded-impact work; independent validation evidence remains
mandatory before completion.

## Technical Context

**Language/Version**: Rust 1.96.0 workspace plus Bash and PowerShell release helpers, JSON metadata, and Markdown documentation artifacts  
**Primary Dependencies**: existing workspace crates (`canon-cli`, `canon-engine`, `canon-adapters`), `jq`, `shasum`, `unzip`, existing packaging templates, and GitHub Actions release automation  
**Storage**: repository files plus ephemeral `dist/` release artifacts and generated `lcov.info` during validation  
**Testing**: `cargo test`, focused Rust release-surface tests, direct shell validation of release scripts, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`  
**Target Platform**: repository-owned release automation for macOS, Linux, and Windows package-channel outputs derived from GitHub Releases  
**Project Type**: Rust CLI workspace with repository-owned release packaging, validation scripts, and documentation surfaces  
**Existing System Touchpoints**: `scripts/release/write-distribution-metadata.sh`, `scripts/release/verify-release-surface.sh`, `scripts/release/render-homebrew-formula.sh`, `scripts/release/render-winget-manifests.sh`, `scripts/release/render-scoop-manifest.sh`, `packaging/homebrew/canon.rb.tpl`, `packaging/winget/*.tpl`, `packaging/scoop/canon.json.tpl`, `README.md`, `docs/guides/modes.md`, `docs/guides/publishing-to-winget.md`, `docs/guides/publishing-to-scoop.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, runtime compatibility references, and focused tests under `tests/`  
**Performance Goals**: keep metadata generation and verification deterministic, fail fast on mismatches, and avoid any second build path or manual checksum reconciliation  
**Constraints**: preserve existing release archive set and filenames; keep GitHub Releases canonical; keep Homebrew, Winget, and Scoop as derived channels only; deliver >95% automated coverage for touched Rust files; finish with clean `cargo fmt` and `cargo clippy`  
**Scale/Scope**: one metadata contract expansion, bounded updates to existing release scripts and docs, one focused release test slice, one version bump, and one roadmap cleanup

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
specs/036-release-provenance-integrity/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── distribution-metadata.md
│   └── channel-integrity.md
└── tasks.md
```

### Source Code (repository root)

```text
scripts/
└── release/
    ├── render-homebrew-formula.sh
    ├── render-scoop-manifest.sh
    ├── render-winget-manifests.sh
    ├── verify-release-surface.sh
    └── write-distribution-metadata.sh

packaging/
├── homebrew/
│   └── canon.rb.tpl
├── scoop/
│   └── canon.json.tpl
└── winget/
    ├── defaultLocale.yaml.tpl
    ├── installer.yaml.tpl
    └── version.yaml.tpl

tests/
├── integration/
│   └── skills_bootstrap.rs
└── release_036_release_provenance_integrity.rs

README.md
ROADMAP.md
CHANGELOG.md
Cargo.toml
Cargo.lock
docs/guides/modes.md
docs/guides/publishing-to-winget.md
docs/guides/publishing-to-scoop.md
defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml
.agents/skills/canon-shared/references/runtime-compatibility.toml
```

**Structure Decision**: Keep the slice localized to the existing release
automation surface by extending the canonical metadata contract and current
renderer or verifier scripts, plus one focused Rust release-surface test and
bounded doc or version updates, instead of introducing a new crate or generic
packaging subsystem.

## Complexity Tracking

No constitution deviations are currently identified.
