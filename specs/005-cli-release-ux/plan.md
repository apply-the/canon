# Implementation Plan: Installable CLI Distribution and Release UX

**Branch**: `005-cli-release-ux` | **Date**: 2026-03-30 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/005-cli-release-ux/spec.md`

## Summary

This feature moves Canon from a source-first repository to an install-first CLI
product without changing the governed runtime model. The implementation stays
bounded to Phase 1 distribution:

- add a dedicated GitHub Actions release workflow that builds, packages, and
  publishes prebuilt binaries for the agreed platform matrix
- align the current CI build matrix with the public artifact matrix so compile
  coverage and shipped assets do not drift
- define explicit release contracts for artifact naming, checksum publication,
  and version visibility
- rewrite README and quickstart surfaces so binary install and PATH-based usage
  become the default story, while source builds move into contributor guidance
- update the shared Canon skill compatibility reference so missing or
  incompatible CLI messaging points users to release-based installation instead
  of Cargo

The design deliberately avoids Homebrew, Chocolatey, mandatory self-update,
enterprise installers, or any Canon runtime redesign.

## Governance Context

**Execution Mode**: `brownfield` because this increment adds release,
packaging, documentation, and compatibility surfaces to an existing Canon
product without redesigning the execution engine or the skills frontend  
**Risk Classification**: `systemic-impact` because broken artifacts,
inconsistent version surfaces, or misleading install guidance would damage the
first-touch product experience across all supported platforms even though
runtime semantics stay stable  
**Scope In**: release workflow design, artifact packaging and checksum
contracts, README and quickstart install-first restructuring, runtime
compatibility guidance for installed binaries, and release-readiness validation
artifacts  
**Scope Out**: Canon runtime redesign, Codex skill redesign, Homebrew,
Chocolatey, winget, mandatory self-update, signing as a Phase 1 requirement,
enterprise installers, and any change to mode semantics or `.canon/`
persistence

**Invariants**:

- Canon CLI remains the only execution engine and the stable daily-use
  entrypoint.
- Daily end-user usage must not require Cargo inside the working repository.
- Codex skills remain thin frontends over `canon` on PATH; they do not become
  installers or a second runtime.
- Build-from-source remains available for contributors, but it is explicitly
  secondary to the installed-binary user journey.
- Release publication must not proceed without artifact completeness,
  version-surface alignment, and independent release-readiness review.

**Decision Log**: [decision-log.md](./decision-log.md)  
**Validation Ownership**: this plan and the release workflow define generation
surfaces; structural CI checks, fresh-environment install walkthroughs, and a
human release-readiness reviewer validate those outputs independently before a
public release is accepted  
**Approval Gates**: human release-owner approval is required before publishing
public release assets, and an independent reviewer must sign off on artifact
matrix completeness, version parity, and install-guide accuracy for every
install-first release  
**Recorded Runtime Boundary**: owner fallback and Canon approval-source
semantics are captured in [owner-approval-addendum.md](./owner-approval-addendum.md)
as follow-on runtime UX decisions, not as implementation scope for this plan

## 1. Technical Context

**Language/Version**: Rust 1.94.1 workspace, Markdown documentation,
GitHub Actions YAML, repo-local Bash and PowerShell helper scripts  
**Primary Dependencies**: existing `clap`, `serde`, `serde_json`,
`serde_yaml`, `toml`, GitHub Releases, `.github/workflows/ci.yml`, planned
`.github/workflows/release.yml`, repo-local `scripts/release/*`, and current
`.agents/skills/canon-shared` compatibility references  
**Storage**: repository files for workflow and documentation changes, GitHub
release assets for published archives and checksum manifests, and existing
`.canon/` runtime state unchanged by this feature  
**Testing**: `cargo fmt --check`, `cargo clippy --all-targets --all-features
-- -D warnings`, `cargo test --locked`, `cargo nextest run --locked`, release
workflow artifact verification, checksum verification, fresh-environment
install walkthroughs, and `scripts/validate-canon-skills.sh` plus
`scripts/validate-canon-skills.ps1` after compatibility-message updates  
**Target Platform**: macOS arm64 and x86_64, Linux x86_64 and arm64, Windows
x86_64, plus GitHub-hosted CI/release automation  
**Project Type**: installable local-first CLI product with a repo-local skills
frontend  
**Existing System Touchpoints**: `Cargo.toml`, `crates/canon-cli/Cargo.toml`,
`.github/workflows/ci.yml`, `README.md`,
`.agents/skills/canon-shared/references/runtime-compatibility.toml`,
`.agents/skills/canon-shared/scripts/check-runtime.sh`,
`.agents/skills/canon-shared/scripts/check-runtime.ps1`,
`specs/001-canon-spec/quickstart.md`, and this feature's docs under
`specs/005-cli-release-ux/`  
**Performance Goals**: release packaging remains deterministic and bounded to
the five public artifacts in the platform matrix; CLI preflight messaging stays
sub-second aside from Canon command execution; published install steps support
successful version verification in under 10 minutes per the spec  
**Constraints**: Phase 1 only, no package-manager rollout, no runtime
behavior change, no hidden Cargo dependency in day-to-day usage, checksum-based
integrity in Phase 1, and exact version parity across tag, artifact names,
release notes, compatibility references, and `canon --version`  
**Scale/Scope**: one dedicated release workflow, one aligned CI build matrix,
one release-script surface, five public platform artifacts plus one checksum
manifest per release, README and quickstart restructuring, and one shared skill
compatibility reference update

## 2. Constitution Check

### Pre-Design Gate

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Systemic-impact approval checkpoints are named
- [x] No constitution deviations are required before design

### Post-Design Re-Check

- [x] Canon runtime authority remains unchanged; only distribution and UX
  surfaces move
- [x] Binary-first installation is explicit without turning skills into an
  installation mechanism
- [x] Source builds remain available but clearly secondary to the product
  entrypoint
- [x] Release publication is gated by artifact completeness, version parity,
  and independent review rather than by generation alone
- [x] Documentation, workflow, and runtime-compatibility changes stay inside
  the declared scope
- [x] Validation layers cover structural workflow checks, install walkthroughs,
  and independent release-readiness review

**Result**: PASS. The design improves installability and trust posture without
weakening Canon governance or broadening into runtime redesign.

## 3. Existing System Analysis

### 3.1 CI and Build Surfaces

- `.github/workflows/ci.yml` already runs format, lint, tests, deny checks, and
  a cross-platform debug build matrix.
- The current matrix compiles `linux-musl`, macOS arm64 and x86_64, and Windows
  x86_64, but it does not cover the Linux arm64 artifact required by the spec.
- The current cross-platform build step uses debug builds only and does not
  package archives, generate checksums, or publish release assets.

### 3.2 Documentation Surfaces

- `README.md` currently leads with source-based installation via `cargo
  install`, which directly violates the install-first goal for daily users.
- `specs/001-canon-spec/quickstart.md` still uses `cargo build` and `cargo run`
  for end-user workflows; this must move into contributor/development posture.
- There is no explicit top-level release-note or install verification contract
  yet.

### 3.3 Skill Compatibility Surfaces

- `.agents/skills/canon-shared/scripts/check-runtime.*` already validates that
  `canon` exists on PATH, probes the version or command contract, and reports
  actionable failures.
- `.agents/skills/canon-shared/references/runtime-compatibility.toml` still
  points missing or incompatible users to a Cargo install command.
- The Canon skills themselves already assume a shared installed binary; the
  primary Phase 1 design change is therefore the compatibility reference and
  the install guidance it points to, not a broad skill rewrite.

## 4. Design Strategy

### 4.1 Release Workflow Topology

Add a dedicated `.github/workflows/release.yml` rather than overloading the
existing CI workflow.

Planned workflow topology:

1. `prepare-release-metadata`
   - derive the release version from the Git tag or manual input
   - verify it matches `Cargo.toml`
   - emit normalized variables for artifact naming and release-note rendering
2. `build-and-package`
   - matrix over the five public artifacts:
     - `macos-arm64`
     - `macos-x86_64`
     - `linux-arm64`
     - `linux-x86_64`
     - `windows-x86_64`
   - use release builds of `canon`
   - native macOS and Windows jobs stay on hosted runners
   - Linux arm64 is built on `ubuntu-latest` using the Rust target plus the
     system aarch64 GNU cross-compiler to avoid broad packaging-tool adoption
   - package archives through repo-local scripts under `scripts/release/`
3. `generate-checksums`
   - create a versioned checksum manifest covering every published artifact
4. `verify-release-surface`
   - assert artifact count, archive names, binary names inside the archive,
     checksum manifest entries, and version parity with `canon --version`
5. `publish-release`
   - upload assets and release notes to GitHub Releases only after verification
     passes and the release owner approves the publication step

The existing `.github/workflows/ci.yml` remains the validation workflow, but
its cross-platform build matrix should be updated to mirror the public Phase 1
targets so compile coverage and release coverage stay aligned.

### 4.2 Artifact Packaging and Naming

Adopt user-facing asset names keyed by version, operating system, and
architecture rather than raw Rust target triples.

Planned public artifact names:

- `canon-<VERSION>-macos-arm64.tar.gz`
- `canon-<VERSION>-macos-x86_64.tar.gz`
- `canon-<VERSION>-linux-arm64.tar.gz`
- `canon-<VERSION>-linux-x86_64.tar.gz`
- `canon-<VERSION>-windows-x86_64.zip`
- `canon-<VERSION>-SHA256SUMS.txt`

Packaging rules:

- Unix archives contain exactly one executable named `canon`
- the Windows archive contains exactly one executable named `canon.exe`
- archive roots stay flat to avoid PATH confusion during extraction
- checksum output uses standard SHA256 two-space formatting so users can verify
  archives with platform-appropriate tools
- Linux musl artifacts remain out of the public Phase 1 release surface even if
  compile-only CI continues to exercise them separately in the future

### 4.3 Install-First Documentation Overhaul

Documentation work is intentionally opinionated.

- `README.md` gains a top-level `Install Canon` section near the top, with one
  primary install path per supported platform and a separate `Verify
  Installation` section showing `canon --version`
- the README quickstart is rewritten to assume `canon` is already installed and
  on PATH
- source build instructions move into a clearly secondary `Contributor /
  Development` section
- `specs/001-canon-spec/quickstart.md` is updated or superseded so user-facing
  quickstart examples no longer require `cargo run`
- release notes link back to the README install section and restate the exact
  artifact selection and verification flow
- PATH troubleshooting guidance is explicit for the shadowed-binary case, where
  an older `canon` may appear earlier on PATH than the newly installed one

### 4.4 Skills Compatibility and Preflight

The design keeps the existing shared preflight contract and changes only the
install guidance it emits.

- `check-runtime.sh` and `check-runtime.ps1` remain the enforcement point for
  `canon` presence, version compatibility, and repo context
- `runtime-compatibility.toml` becomes the canonical repo-local source for
  install/update guidance, moving from a Cargo command to release-based
  directions that point users to the README install section and the current
  release surface

### 4.5 Deferred Runtime UX Follow-Up

Feature 005 intentionally stops at installability, compatibility messaging,
and release trust. It does not change Canon runtime intake semantics.

The adjacent runtime UX concerns are still worth recording so release and
first-run experience do not drift apart:

- future runtime work should infer `owner` from explicit input or trustworthy
  Git identity before falling back to manual prompting
- Canon approvals remain repo-local and run-scoped even when teams use
  provider-hosted pull request review workflows
- any future reuse of PR approval state must be an explicit bridge or imported
  evidence path rather than an implicit dependency inside the core runtime

The durable record for those decisions is
[owner-approval-addendum.md](./owner-approval-addendum.md).
- runnable skill docs keep their existing thin-wrapper structure and continue
  to tell users to install or update Canon when the shared helper reports a
  missing or incompatible CLI
- the plan must preserve a recoverable path for missing, incompatible, or
  shadowed binaries without inventing runtime state or changing skill support
  semantics

## 5. Implementation Workstreams

### Workstream A: Release Automation

- add `.github/workflows/release.yml`
- align `.github/workflows/ci.yml` cross-platform compile coverage to the five
  shipped targets
- add `scripts/release/package-unix.sh`,
  `scripts/release/package-windows.ps1`, and
  `scripts/release/verify-release-surface.sh` so packaging logic is repo-local,
  reviewable, and reusable outside YAML
- produce archives and a versioned SHA256 manifest for every release

### Workstream B: Version and Contract Surfaces

- add explicit release contracts under `specs/005-cli-release-ux/contracts/`
- fail the release workflow if any version surface differs across tag,
  workspace version, artifact names, release notes, checksum manifest, or
  `canon --version`
- codify release notes as a required part of the public artifact set

### Workstream C: Documentation and Install UX

- restructure `README.md` around install-first guidance
- update or replace quickstart surfaces so the default flow is: install, verify,
  `canon init`, then run Canon
- add contributor-only source build guidance as a secondary path
- document PATH and architecture-selection troubleshooting

### Workstream D: Skills Compatibility and Validation

- update `.agents/skills/canon-shared/references/runtime-compatibility.toml`
  to release-based install guidance
- rerun Bash and PowerShell skill validators to ensure preflight prose remains
  aligned with the shared helper behavior
- capture cross-platform install evidence and skill recovery evidence in
  `validation-report.md`

## 6. Project Structure

### Documentation (this feature)

```text
specs/005-cli-release-ux/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── installation-runtime-compatibility-contract.md
│   ├── release-artifact-contract.md
│   └── version-visibility-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
.github/
└── workflows/
    ├── ci.yml
    └── release.yml

.agents/
└── skills/
    └── canon-shared/
        ├── references/
        │   └── runtime-compatibility.toml
        └── scripts/
            ├── check-runtime.ps1
            └── check-runtime.sh

scripts/
└── release/
    ├── package-unix.sh
    ├── package-windows.ps1
    └── verify-release-surface.sh

crates/
└── canon-cli/

README.md
specs/001-canon-spec/quickstart.md
Cargo.toml
```

**Structure Decision**: extend the existing Rust workspace with one dedicated
release workflow, small repo-local packaging scripts, documentation changes,
and shared compatibility-reference updates. No new crate or runtime storage is
required for this feature.

## Complexity Tracking

No constitution violations or scope exceptions are required for this plan.
