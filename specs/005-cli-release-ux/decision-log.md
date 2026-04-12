# Decision Log: Installable CLI Distribution and Release UX

## D-001: Separate release publication from validation CI

**Decision**: add a dedicated `.github/workflows/release.yml` and keep
`.github/workflows/ci.yml` focused on validation and compile coverage.

**Rationale**: publication requires different permissions, gates, and evidence
than pull-request validation.

**Consequences**:

- release automation stays auditable and easier to review
- CI and release matrices must remain intentionally aligned to avoid drift

## D-002: Use friendly os/arch artifact names as the public contract

**Decision**: public artifacts use `canon-<VERSION>-<os>-<arch>.<ext>`.

**Rationale**: users select artifacts by platform and architecture, not by Rust
target triple.

**Consequences**:

- workflow metadata must maintain a mapping from public names to build targets
- release notes and checksum manifests can mirror the same user-facing names

## D-003: Limit Phase 1 to the five-platform artifact matrix plus SHA256 checksums

**Decision**: Phase 1 public releases ship macOS arm64 and x86_64, Linux arm64
and x86_64, Windows x86_64, plus a versioned SHA256 checksum manifest.

**Rationale**: this is the smallest credible install-first baseline that meets
the spec without broadening into package managers or signing.

**Consequences**:

- Linux musl stays out of the public release contract for now
- signatures remain an explicit later-phase enhancement rather than a hidden
  requirement

## D-004: Make README and quickstart install-first and demote source builds

**Decision**: move binary installation and verification near the top of
`README.md`, rewrite quickstart examples to use `canon` directly, and isolate
Cargo-based build instructions under contributor/development guidance.

**Rationale**: Canon should feel like an installable CLI on first contact,
while still retaining a source build path for contributors.

**Consequences**:

- user-facing docs must not show Cargo in the normal repository flow
- contributor docs must remain explicit so development workflows do not regress

## D-005: Preserve the current skill preflight contract and update only the shared compatibility guidance

**Decision**: keep `.agents/skills/canon-shared/scripts/check-runtime.*` as the
runtime enforcement layer and update
`.agents/skills/canon-shared/references/runtime-compatibility.toml` to point to
release-based install/update guidance.

**Rationale**: the skills already assume `canon` on PATH; the install-first gap
sits in the shared recovery message, not in the execution model.

**Consequences**:

- skill behavior remains thin, deterministic, and Canon-backed
- Bash and PowerShell validators must confirm message parity after the update

## D-006: Block public release on version parity and independent readiness review

**Decision**: public release publication requires automated version-surface
parity checks and explicit independent review of artifacts, docs, and install
walkthroughs.

**Rationale**: install trust depends on the user seeing the same version across
release notes, file names, and command output.

**Consequences**:

- release automation must fail closed on any mismatch
- maintainers need a durable release-readiness checklist and reviewer evidence

## D-007: Use a versioned release-notes template rendered during the release workflow

**Decision**: keep release notes in `.github/release-notes-template.md` with
version placeholders rendered by the release workflow into a concrete
`release-notes.md` artifact before verification and publication.

**Rationale**: release notes are part of the public version surface. Keeping a
template in-repo makes the content reviewable while still allowing the release
workflow to bind the exact version and artifact names at publication time.

**Consequences**:

- version parity checks must validate the rendered release notes, not just the
  template
- the release workflow needs an explicit render step before checksum and
  release-asset verification

## D-008: Keep shared skill recovery guidance in the compatibility reference rather than rewriting preflight logic

**Decision**: preserve the current Bash and PowerShell preflight scripts and
change only `.agents/skills/canon-shared/references/runtime-compatibility.toml`
so recovery text points to release-based installation guidance.

**Rationale**: the helper already enforces Canon presence and compatibility.
The gap is the user guidance string, not the runtime decision path.

**Consequences**:

- install/update wording must fit the helper's existing `install_command`
  contract
- validator runs must confirm the changed message still reads coherently in
  both Bash and PowerShell flows

## D-009: Troubleshoot stale binaries through PATH ordering guidance instead of runtime changes

**Decision**: document PATH troubleshooting explicitly in README and
quickstart, with the expected recovery step being to move the intended Canon
install directory earlier on PATH and rerun `canon --version`.

**Rationale**: stale binaries earlier on PATH are an installation UX problem,
not a Canon runtime problem. The fix belongs in installation guidance rather
than in extra CLI or skill logic.

**Consequences**:

- install and validation docs must show both version verification and path
  resolution checks
- skill recovery guidance can stay thin and point users back to the install
  guide without inventing path-inspection behavior

## D-010: Record owner fallback as a follow-on runtime UX concern instead of expanding Feature 005 scope

**Decision**: owner fallback from explicit input, repository-local Git
identity, and weaker environment fallbacks is recorded as a follow-on runtime
UX decision in `owner-approval-addendum.md`, not as implementation scope for
Feature 005.

**Rationale**: Feature 005 governs installability, release trust, and shared
compatibility guidance. Owner inference changes run intake behavior and should
be implemented in a dedicated runtime feature rather than being smuggled into
the release tranche.

**Consequences**:

- Feature 005 can acknowledge first-run ownership friction without claiming to
  solve runtime identity resolution
- later runtime work has a durable decision trail to follow

## D-011: Preserve Canon approvals as local run evidence rather than treating provider PR approval as automatically equivalent

**Decision**: Canon approval-gated behavior continues to depend on Canon-native
approval records, not on provider-side PR approval state.

**Rationale**: Canon gates currently require local, durable, run-scoped
approval evidence. Provider review state does not guarantee the same scope or
traceability.

**Consequences**:

- teams may keep using provider review workflows, but those workflows do not
  silently satisfy Canon gates
- release UX documentation can describe the boundary without changing runtime
  approval semantics

## D-012: Future PR approval reuse must happen through explicit evidence import or adapter bridging

**Decision**: any future integration of PR approval state into Canon must be
modeled as explicit imported evidence or an adapter boundary rather than as
implicit live coupling.

**Rationale**: explicit bridging preserves auditability, offline reasoning,
and reproducible gate evaluation.

**Consequences**:

- Canon avoids hidden dependency on mutable provider state
- future approval integrations have a clear architectural constraint from the
  outset

## Release Approval Checkpoints

The following checkpoints govern implementation and publication for this
systemic-impact feature:

- documentation and compatibility guidance must be updated before release
  automation is considered ready
- the five public artifacts plus the checksum manifest must exist before any
  publication step
- version parity across `Cargo.toml`, release notes, archive names, checksum
  manifest, and `canon --version` must pass before publication
- a release-readiness reviewer distinct from the asset producer must review
  install guidance, artifact completeness, and validation evidence before a
  public release is considered complete

## Implementation Closeout Notes

- local structural validation passed for `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --locked`, `cargo nextest run --locked`, `git diff --check`, and both Canon skill validators
- a workflow-aligned local dry run under `.canon/tmp/release-dry-run` produced the five required archive names plus the checksum manifest, rendered release notes, and a native-host install smoke result for `canon --version` and `canon init --output json`
- shared Bash and PowerShell preflight helpers now emit release-guide actions that point to the README install section and GitHub Releases instead of a Cargo install command
- public publication remains blocked until a distinct human reviewer validates native GitHub-hosted artifacts and signs off on release readiness