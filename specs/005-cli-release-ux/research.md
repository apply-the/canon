# Research: Installable CLI Distribution and Release UX

## Decision 1: Use a dedicated GitHub Actions release workflow and keep CI separate

**Decision**: add a new `.github/workflows/release.yml` for artifact
packaging, checksum generation, and GitHub Release publication, while keeping
`.github/workflows/ci.yml` focused on validation and compile confidence.

**Rationale**: release publication has different permissions, review gates, and
failure modes than pull-request validation. Splitting the workflows keeps
public distribution logic bounded and makes release-readiness easier to audit.

**Alternatives considered**:

- Extend `ci.yml` with release-only jobs. Rejected because it mixes validation
  and publication concerns under one workflow surface.
- Publish artifacts manually outside GitHub Actions. Rejected because it would
  weaken repeatability, evidence capture, and version-surface validation.

## Decision 2: Public artifacts use user-facing os/arch names, not raw target triples

**Decision**: name published archives using the format
`canon-<VERSION>-<os>-<arch>.<ext>` and keep Rust target triples as internal
workflow metadata.

**Rationale**: the spec requires version, operating system, and architecture in
the artifact name. User-facing names like `canon-0.5.0-macos-arm64.tar.gz` are
easier to choose correctly than raw triples while still mapping cleanly to
build targets inside the workflow.

**Alternatives considered**:

- Publish target-triple names such as
  `canon-0.5.0-aarch64-apple-darwin.tar.gz`. Rejected because they are more
  technical than the product surface needs.
- Hide architecture in the asset name. Rejected because it weakens first-time
  artifact selection and violates the release contract.

## Decision 3: Phase 1 ships five public artifacts and drops Linux musl from the public matrix

**Decision**: Phase 1 public releases ship exactly these platform artifacts:

- macOS arm64
- macOS x86_64
- Linux arm64
- Linux x86_64
- Windows x86_64

Linux musl is not part of the public artifact contract in this increment.

**Rationale**: the feature specification defines a practical desktop and server
baseline. Keeping the public matrix at five artifacts is large enough for
serious daily use while avoiding extra packaging scope in the first install-
first release.

**Alternatives considered**:

- Keep the existing Linux musl build as a public release asset. Rejected
  because it broadens the support claim without being required by the spec.
- Defer Linux arm64 to a later increment. Rejected because the spec explicitly
  includes it in the supported platform matrix.

## Decision 4: Use repo-local packaging scripts plus hosted runners and a GNU cross-compiler for Linux arm64

**Decision**: implement packaging with small repo-local Bash and PowerShell
helpers under `scripts/release/`, use hosted runners for macOS and Windows,
use native Linux x86_64 builds on `ubuntu-latest`, and build Linux arm64 on
`ubuntu-latest` with the Rust arm64 GNU target plus the system aarch64 GNU
cross-compiler.

**Rationale**: this stays close to the repository's existing tooling style,
keeps packaging logic reviewable outside YAML, and avoids introducing a broad
new release framework for a bounded Phase 1 rollout.

**Alternatives considered**:

- Adopt `cargo-dist` immediately. Rejected because it adds a larger release
  abstraction than this increment needs.
- Implement all packaging logic inline in workflow YAML. Rejected because it is
  harder to review and reuse locally.
- Require a separate arm64 Linux runner from day one. Rejected because the
  hosted cross-compile path is simpler to land inside the current repository.

## Decision 5: Phase 1 integrity is a versioned SHA256 manifest, not signing

**Decision**: publish a versioned checksum manifest named
`canon-<VERSION>-SHA256SUMS.txt` alongside every release, using standard
SHA256 two-space formatting, and defer artifact signing to a later increment.

**Rationale**: the specification requires integrity metadata, not mandatory
signing. Checksums are the smallest credible baseline for Phase 1 and can be
validated consistently on all supported platforms.

**Alternatives considered**:

- Require signing in the first release tranche. Rejected because it broadens
  operational scope and key-management work beyond the bounded Phase 1 target.
- Publish no integrity metadata until later. Rejected because it would violate
  the spec and weaken release trust.

## Decision 6: Rewrite documentation around install-first usage and demote source builds to contributor guidance

**Decision**: move binary installation and PATH verification near the top of
`README.md`, rewrite quickstart examples to use `canon` directly, and isolate
Cargo-based source builds in a clearly secondary contributor/development
section.

**Rationale**: the largest trust gap in the current repo is documentation
posture. Users should see a normal installed CLI first; contributors can still
see how to build from source without that flow becoming the product entrypoint.

**Alternatives considered**:

- Keep source-build install instructions near the top and add binary install
  lower down. Rejected because it preserves the source-first impression.
- Remove source-build guidance entirely. Rejected because contributors still
  need a documented development path.

## Decision 7: Keep the current skill preflight contract and update only the compatibility reference and recovery guidance

**Decision**: do not redesign Canon skill markdown or the shared preflight
scripts for Phase 1. Update `.agents/skills/canon-shared/references/
runtime-compatibility.toml` so missing or incompatible Canon installations point
to release-based install/update guidance instead of Cargo commands.

**Rationale**: the skills already assume a real `canon` binary on PATH and the
shared helper already validates that contract. The install-first gap is the
guidance source, not the enforcement model.

**Alternatives considered**:

- Rewrite every Canon skill for install-first messaging. Rejected because the
  shared compatibility reference already centralizes the relevant guidance.
- Leave the Cargo install command in place temporarily. Rejected because it
  would directly contradict the feature goal.

## Decision 8: Release publication is blocked on version parity and independent review

**Decision**: the release workflow must fail before publication if any version
surface differs across the Git tag, `Cargo.toml`, archive names,
`canon --version`, checksum manifest references, or release notes. Public
release publication also requires independent release-readiness review.

**Rationale**: release UX breaks down quickly when users download a file that
does not match the documented version or the binary output. Version parity and
independent review are the minimum trustworthy release gates for a CLI product.

**Alternatives considered**:

- Rely on manual maintainer checks. Rejected because drift is too easy to miss.
- Validate version only inside the binary. Rejected because users also see tags,
  file names, and release notes before they ever run the command.