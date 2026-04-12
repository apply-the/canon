# Feature Specification: Installable CLI Distribution and Release UX

**Feature Branch**: `005-cli-release-ux`  
**Created**: 2026-03-30  
**Status**: Draft  
**Input**: User description: "Create the next product specification for Canon: Installable CLI Distribution and Release UX"

## Governance Context *(mandatory)*

**Mode**: `brownfield` because this increment adds installability, packaging,
release artifacts, and documentation UX to an existing Canon product without
redesigning the runtime or the Codex skills frontend.  
**Risk Classification**: `systemic-impact` because distribution and release UX
become the first product touchpoint for every new user, and mistakes here
would weaken trust across supported platforms and the product's public install
surface even though runtime semantics remain intact.  
**Scope In**: install-first CLI distribution, cross-platform release artifacts,
artifact naming and integrity expectations, installation guidance, runtime
dependency expectations for installed binaries, README and quickstart updates,
and minimum CI/release workflow expectations for shipping Canon as a serious
developer tool.  
**Scope Out**: Canon runtime redesign, Codex skills redesign, plugin
marketplace work, mode semantics changes, enterprise installer tooling,
mandatory self-update, and broad package ecosystem expansion beyond the minimum
required for a trustworthy installable CLI.

**Invariants**:

- Canon CLI remains the execution engine and the stable daily-use entrypoint.
- Codex skills remain a frontend layer over `canon`, not an installation or
  packaging mechanism.
- Daily end-user usage must not require Cargo inside the user’s working
  repository.
- Build-from-source remains available for contributors but must stay secondary
  to binary-first product messaging and documentation.

**Decision Traceability**: Decisions and validation evidence for this feature
will be recorded under `specs/005-cli-release-ux/`, with design decisions in
`decision-log.md`, release/install validation evidence in
`validation-report.md`, and runtime-boundary follow-up decisions in
`owner-approval-addendum.md`.

## Product Delta

This increment adds installation, packaging, and release UX for Canon as a
real CLI product.

Canon already works as a local-first governed CLI and already has a Codex
skills frontend. What changes here is the product posture: daily users should
encounter Canon first as an installable binary with a stable `canon` entrypoint
on PATH, not as a Rust project they are expected to compile manually.

The feature therefore defines a serious distribution baseline:

- downloadable release artifacts for supported platforms
- a canonical PATH-based install model
- release asset and version visibility contracts
- documentation that leads with install and binary usage
- contributor source builds retained as a secondary workflow

## Problem Statement

Canon is usable today, but the current posture still risks feeling
source-first instead of install-first.

Operationally, that creates four product problems:

- end users should not depend on Cargo in daily repository use
- the current installation and distribution posture weakens trust and product
  feel
- README and quickstart guidance do not yet define an install-first reality as
  the primary story
- Canon needs to behave like a real CLI with a stable binary entrypoint that a
  user installs once and reuses across repositories

If this gap remains, demanding engineers will continue to perceive Canon as a
source project rather than a serious installable developer tool.

## Goals

- Make Canon installable as a standalone CLI.
- Define a release artifact contract for macOS, Linux, and Windows.
- Establish clear end-user install paths and verification steps.
- Align README and quickstart guidance to binary-first usage.
- Preserve contributor build-from-source documentation while making it clearly
  secondary.
- Ensure Codex skills can assume a real `canon` binary on PATH.

## First-Run Governance Boundary

Successful installation is necessary but not sufficient for a smooth first-run
experience. Once Canon is installed, users still encounter runtime ownership,
approval, and preflight behavior that shapes perceived product quality.

Feature 005 does not implement new runtime semantics for ownership or
approvals. It does record the intended follow-on boundary so install-first UX
does not drift away from later runtime UX work:

- future runtime UX should prefer trustworthy local identity defaults over
  repeated manual prompts where governance permits
- provider-side PR or review approval does not automatically satisfy Canon
  approval-gated runtime behavior
- detailed runtime-boundary decisions for owner fallback and approval evidence
  are recorded in `owner-approval-addendum.md`

## Supported Platforms and Artifact Matrix

| Platform | Supported Architectures | Artifact Format | Expected Install Path Shape | Minimum Viable User Story |
| --- | --- | --- | --- | --- |
| macOS | arm64, x86_64 | `.tar.gz` archive containing `canon` | user PATH directory such as `~/.local/bin/` or system PATH directory such as `/usr/local/bin/` | A user downloads the archive, extracts `canon`, places it on PATH, runs `canon --version`, and uses `canon` in any repo |
| Linux | x86_64, arm64 | `.tar.gz` archive containing `canon` | user PATH directory such as `~/.local/bin/` or system PATH directory such as `/usr/local/bin/` | A user downloads the archive, extracts `canon`, places it on PATH, runs `canon --version`, and uses `canon` in any repo |
| Windows | x86_64 | `.zip` archive containing `canon.exe` | user PATH directory such as `%USERPROFILE%\\bin` or another documented PATH location | A user downloads the archive, extracts `canon.exe`, places it on PATH, runs `canon --version`, and uses `canon` in any repo |

The initial platform matrix is intentionally practical rather than exhaustive.
It must be large enough to support serious daily usage without broadening into
full ecosystem packaging in the same increment.

## Distribution Strategy

Distribution must be staged and opinionated.

### Phase 1: Install-First Baseline

- Downloadable prebuilt binaries are the canonical distribution path.
- PATH-based installation is the canonical daily-use model.
- Release guidance must show direct binary installation before any
  build-from-source instructions.
- Windows must have a documented lightweight install path that feels natural to
  Windows users without requiring a heavyweight installer stack.
- Build-from-source remains available only as contributor fallback and recovery
  path.

### Phase 2: Convenience Channels

- Homebrew support may be added as a convenience channel if the feature plan
  proves it is worth shipping in the next tranche.
- Additional install helpers may be added for Windows if zip-only guidance is
  judged too brittle.
- Extra polish such as stronger integrity or signing UX may be layered on top
  of the baseline release artifact contract.

This increment does not require every package manager ecosystem. It requires a
credible baseline that makes Canon feel installable, trustworthy, and normal
to use.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install Canon Once and Use It Anywhere (Priority: P1)

A daily Canon user wants to install Canon once, keep `canon` on PATH, and use
it directly in any repository without Cargo.

**Why this priority**: This is the primary product entrypoint and the core
trust repair. If daily usage still feels source-first, the feature fails.

**Independent Test**: On each supported platform, a fresh user follows the
published install guidance, verifies `canon --version`, then runs `canon init`
successfully in a test repository without using Cargo.

**Acceptance Scenarios**:

1. **Given** a supported macOS or Linux machine without Canon installed,
   **When** the user follows the published install guidance, **Then** `canon`
   becomes available on PATH and version verification succeeds.
2. **Given** a supported Windows machine without Canon installed, **When** the
   user follows the published install guidance, **Then** `canon.exe` becomes
   available through PATH and version verification succeeds.

---

### User Story 2 - Trust a Release as a Real CLI Delivery (Priority: P1)

A release maintainer wants each Canon release to publish a complete, verifiable
set of artifacts so users can install with confidence instead of guessing how
to build the tool.

**Why this priority**: A serious install-first experience depends on the
release contract being trustworthy before documentation or adoption can scale.

**Independent Test**: A release candidate can be reviewed and shown to contain
the full agreed platform matrix, integrity metadata, version-correct binaries,
and release notes with install guidance.

**Acceptance Scenarios**:

1. **Given** a new Canon release is prepared, **When** release assets are
   published, **Then** each agreed platform and architecture has a clearly
   named downloadable artifact and integrity metadata.
2. **Given** a published release, **When** a reviewer compares the release
   notes and binary version output, **Then** version visibility and install
   guidance are aligned.

---

### User Story 3 - Use Codex Skills on Top of an Installed Canon Binary (Priority: P2)

A user working in a repository with Canon skills wants those skills to behave
like a frontend over a real installed CLI, not like a hidden build step.

**Why this priority**: Skills are part of the product surface, but they must
remain clearly downstream of the CLI installation model.

**Independent Test**: In a repository with Canon skills available, the user can
invoke a skill successfully when `canon` is on PATH, and receives clear
recovery guidance when `canon` is missing or incompatible.

**Acceptance Scenarios**:

1. **Given** `canon` is installed and on PATH, **When** the user invokes a
   Canon skill in a supported workspace, **Then** the skill assumes the real
   binary entrypoint and proceeds with its normal preflight behavior.
2. **Given** `canon` is missing or incompatible, **When** the user invokes a
   Canon skill, **Then** the skill reports the missing or incompatible runtime
   explicitly and points to the supported install or update flow.

### Edge Cases

- A user installs the wrong platform or architecture artifact for their
  machine.
- A user has an older `canon` binary earlier on PATH than the newly installed
  one.
- A repository has Canon skills available but no usable `canon` binary on PATH.
- A user installs Canon correctly but has not initialized `.canon/` in the
  target repository yet.
- A release contains partial artifacts or notes that do not match the shipped
  version.
- A user lands on contributor build instructions before end-user install
  guidance and misclassifies the product entrypoint.
- A user completes installation successfully but still encounters avoidable
  first-run friction because ownership defaults are missing or weaker than the
  local environment could support.
- A team assumes provider-side PR approval should unblock Canon gates even
  though Canon has not recorded equivalent run-scoped approval evidence.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon releases MUST provide downloadable prebuilt binaries for
  the agreed macOS, Linux, and Windows platform matrix.
- **FR-002**: Each published release MUST expose Canon as a stable binary
  entrypoint named `canon` or `canon.exe` within its platform archive.
- **FR-003**: Each release artifact MUST use a naming convention that includes
  Canon version, operating system, and architecture.
- **FR-004**: Each release MUST include integrity metadata sufficient for a
  user or reviewer to verify artifact completeness before installation.
- **FR-005**: Canon MUST publish a canonical PATH-based installation story for
  daily end users on each supported platform.
- **FR-006**: Canon documentation MUST present end-user installation guidance
  near the top of the README and quickstart experience.
- **FR-007**: Daily-use examples in README and quickstart MUST use `canon`
  directly and MUST NOT require Cargo as part of the normal repository usage
  flow.
- **FR-008**: Contributor build-from-source guidance MUST remain documented,
  but it MUST be explicitly separated from the primary end-user install flow.
- **FR-009**: The installation UX MUST define how a user verifies a successful
  install, including version visibility and confirmation that `canon` resolves
  from PATH.
- **FR-010**: The product contract MUST state what an installed Canon binary
  expects at runtime, including repository context and `.canon/`
  initialization expectations.
- **FR-011**: Canon skills guidance and compatibility behavior MUST assume a
  real `canon` binary on PATH and MUST treat missing or incompatible CLI state
  as a recoverable runtime condition.
- **FR-012**: The distribution strategy MUST distinguish Phase 1 baseline
  distribution from later convenience channels so release scope remains tight.
- **FR-013**: Release notes MUST include enough install and compatibility
  guidance for a first-time user to select the correct artifact and complete
  verification.
- **FR-014**: The release workflow MUST verify that published binaries report
  the same version that is documented in release notes and end-user install
  guidance.
- **FR-015**: The release workflow MUST produce or attach the full agreed
  artifact set before a release is considered ready for public use.
- **FR-016**: Canon MUST retain a documented fallback path for contributors who
  need to build from source without allowing that fallback to become the
  default end-user story.

### Key Entities

- **Release Artifact**: A versioned downloadable package for one operating
  system and architecture containing the Canon executable and any required
  release metadata.
- **Platform Support Entry**: A documented support statement that defines one
  operating system, one or more architectures, its artifact format, and the
  expected install path shape.
- **Installation Flow**: The documented end-user sequence for downloading,
  placing Canon on PATH, verifying the binary, and starting work in a
  repository.
- **Release Integrity Metadata**: The published information that lets a user or
  reviewer confirm artifact identity and completeness before use.
- **Release Note**: The user-facing release summary that communicates what was
  shipped, how to install it, and how the release version should be verified.

## Release Artifact Contract

Each release must produce a complete artifact set for the agreed platform
matrix.

- Binary naming must remain `canon` on Unix-like platforms and `canon.exe` on
  Windows.
- Archive names must include Canon version, platform, and architecture.
- Integrity metadata must be published alongside release artifacts.
- Release notes must state the version, supported platform matrix, install
  guidance, and any compatibility caveats.
- `canon --version` must expose the same release version that appears in the
  release notes and published artifact names.

## Installation UX

The desired user-facing install experience is:

1. choose the correct artifact for platform and architecture
2. download and extract the archive
3. place the binary in a PATH directory
4. verify with `canon --version`
5. confirm the command resolves normally from the shell
6. move into a repository and start with `canon init` or another appropriate
   Canon command

“Installed correctly” means:

- `canon` resolves from PATH without invoking Cargo
- version verification returns the expected release version
- the user can start normal Canon repository workflows immediately after
  installation

Contributor setup differs in one explicit way: contributors may still build
from source for development, but that path is documented as a development
workflow rather than the primary product entrypoint.

## Runtime Dependency Expectations

An installed Canon binary is expected to run in a repository context where the
user intends Canon to operate.

- repository workflows may still require `.canon/` initialization where
  applicable
- Canon must behave as a PATH-resolved executable, not as a per-repo build step
- Codex skills, where present, rely on the shared `canon` binary and must
  surface missing or incompatible CLI state clearly
- runtime guidance must explain what the user should do when Canon is present
  but the repository has not yet been initialized

## README and Quickstart Contract

README and quickstart documentation must reflect install-first usage.

- an install section must appear near the top of the README
- top-level quickstart steps must assume a prebuilt `canon` binary is already
  installed or explain how to install it first
- binary-first examples must use `canon` directly
- build-from-source guidance must move into development or contributor sections
- documentation must clearly separate end-user install and usage from
  contributor build and development workflows

## CI / Release Workflow Expectations

The minimum release workflow for this feature must cover:

- building release binaries for the agreed platform matrix
- verifying artifact completeness before release publication
- generating or publishing integrity metadata
- publishing or attaching artifacts to a release surface that users can reach
- validating that release notes, binary version output, and artifact naming are
  aligned
- ensuring documentation updates land in sync with the release story so users
  are not sent into source-first flows by mistake

## Open Questions

- Should Homebrew be included in Phase 1, or deferred to Phase 2 once the
  baseline artifact contract is stable?
- On Windows, is direct zip distribution sufficient for the first tranche, or
  does the product need a lightweight install helper to feel credible?
- Are checksums alone sufficient in the first release tranche, or must signing
  be mandatory from the first install-first release?
- Should the first platform matrix stop at mainstream desktop and server
  targets, or is broader architecture coverage required immediately?

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On each supported platform, a first-time user can complete
  installation and version verification in under 10 minutes using only the
  published release guidance.
- **SC-002**: 100% of public Canon releases publish the full agreed artifact
  matrix, integrity metadata, and release notes before being announced.
- **SC-003**: 100% of top-level daily-use examples in the README and quickstart
  use `canon` directly, and 0 require Cargo as part of normal day-to-day
  repository usage.
- **SC-004**: In validation walkthroughs, at least 95% of fresh-environment
  installs on supported platforms reach successful CLI version verification on
  the first documented attempt.
- **SC-005**: In repositories with Canon skills available, 100% of validation
  walkthroughs either find a usable `canon` binary on PATH or surface a clear
  recovery path for missing or incompatible CLI state.

## Validation Plan *(mandatory)*

- **Structural validation**: verify artifact matrix completeness, artifact
  naming consistency, version visibility, release note presence, install
  section placement in README, and binary-first quickstart coverage.
- **Logical validation**: run end-user install walkthroughs on each supported
  platform, verify PATH-based CLI usage in a test repository, and confirm skill
  recovery behavior when Canon is missing or incompatible.
- **Independent validation**: require a release-readiness review by someone
  other than the artifact producer, using fresh-environment install checks and
  documentation review.
- **Evidence artifacts**: record findings in
  `specs/005-cli-release-ux/validation-report.md`, with supporting release
  manifests, install transcripts, release-note review notes, and
  cross-platform verification summaries.

## Decision Log *(mandatory)*

- **D-001**: Prebuilt downloadable binaries are the Phase 1 canonical
  distribution path, **Rationale**: this is the smallest credible step that
  makes Canon feel install-first without redesigning the product.
- **D-002**: Build-from-source remains documented but explicitly secondary,
  **Rationale**: contributor flexibility is preserved while the product entry
  point becomes the installed binary.

## Non-Goals

- Redesigning Canon runtime behavior, execution model, or mode semantics.
- Redesigning the Codex skills frontend or turning skills into the
  installation mechanism.
- Building a plugin marketplace or broader ecosystem packaging strategy.
- Supporting every package manager or enterprise installer format in the first
  tranche.
- Requiring mandatory self-update or a full installer platform in this
  increment.

## Assumptions

- Users on supported platforms can place binaries in a PATH directory and have
  permission to update their local shell or system PATH configuration.
- Canon can publish downloadable release assets and release notes on a release
  surface users can access directly.
- The first install-first release can focus on a bounded, mainstream platform
  matrix without blocking future convenience distribution channels.
- Existing Canon workflows and skill behaviors remain product-valid once a real
  `canon` binary is available on PATH.
