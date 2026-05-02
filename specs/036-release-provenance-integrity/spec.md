# Feature Specification: Release Provenance And Channel Integrity

**Feature Branch**: `036-release-provenance-integrity`  
**Created**: 2026-05-02  
**Status**: Draft  
**Input**: User description: "Add a canonical release provenance and channel-integrity slice for 0.36.0 so Canon can publish and verify one durable release manifest across GitHub Releases, Homebrew, Winget, and Scoop, detect metadata drift before publication, update impacted docs, roadmap, and changelog, and close the feature with greater than 95 percent coverage for touched Rust files plus clean cargo fmt and clippy."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice changes Canon's release metadata, package-channel rendering, verification workflow, and release-facing documentation, but it does not alter governed runtime semantics, `.canon/` persistence, or approval behavior.  
**Scope In**: extend the canonical distribution metadata contract with explicit provenance and per-channel integrity fields; tighten release-surface verification so Homebrew, Winget, and Scoop are validated against the same canonical release inventory; keep GitHub Releases as the source of truth; update impacted release-facing docs, roadmap, and changelog for `0.36.0`; add focused Rust validation coverage for modified or new Rust files plus clean `cargo fmt` and `cargo clippy` closeout.  
**Scope Out**: introducing a new package-manager channel; redesigning archive naming or packaging payloads; adding runtime governance commands or mode behavior; introducing cryptographic signing or external attestation infrastructure; replacing GitHub Releases as the canonical release host; adding a parallel packaging pipeline.

**Invariants**:

- GitHub Releases MUST remain the canonical source of downloadable binaries, archive filenames, checksums, and release notes.
- Homebrew, Winget, and Scoop MUST continue to derive from the same canonical release bundle rather than rebuilding Canon through a separate path.
- Canon runtime behavior, modeled modes, `.canon/` storage, publish semantics, and approval posture MUST remain unchanged by this feature.
- Release-surface validation MUST fail closed when metadata, generated channel artifacts, or documented install paths drift from the canonical release bundle.

**Decision Traceability**: Decisions and closeout evidence for this feature MUST be recorded in `specs/036-release-provenance-integrity/decision-log.md` and `specs/036-release-provenance-integrity/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Emit A Canonical Provenance-Rich Release Manifest (Priority: P1)

As a Canon maintainer, I want one canonical release manifest that records the source-of-truth bundle and channel expectations so I can detect package-channel drift before any publication step succeeds.

**Why this priority**: This is the control point for the whole slice. Without a stronger canonical manifest, every channel renderer and verifier keeps relying on duplicated assumptions.

**Independent Test**: Given a valid `dist/` release bundle, a maintainer can generate distribution metadata and then validate that manifest against the canonical bundle without rendering or publishing any package-manager artifact.

**Acceptance Scenarios**:

1. **Given** a valid release bundle with archives, checksum manifest, and release notes, **When** the distribution metadata step runs, **Then** Canon emits one machine-readable manifest that records canonical provenance fields and the release asset inventory.
2. **Given** a release bundle missing a required archive, checksum entry, or provenance field, **When** validation runs, **Then** the release surface fails closed before package-channel rendering is treated as ready.
3. **Given** a manifest whose channel expectations disagree with the canonical asset inventory, **When** verification runs, **Then** the mismatch is reported explicitly instead of being silently normalized.

---

### User Story 2 - Render Package Channels From Explicit Channel Contracts (Priority: P2)

As a Canon maintainer, I want Homebrew, Winget, and Scoop artifacts to derive from explicit channel contracts in the canonical metadata so package-manager outputs stop depending on hidden assumptions inside separate scripts.

**Why this priority**: Once provenance is explicit, the next highest value is removing drift-prone channel knowledge from the renderers and verifiers.

**Independent Test**: From a canonical metadata file, a maintainer can render Homebrew, Winget, and Scoop artifacts and observe that each renderer succeeds only when its declared channel contract is present and consistent.

**Acceptance Scenarios**:

1. **Given** metadata that declares a valid Homebrew channel contract, **When** the Homebrew renderer runs, **Then** it emits a formula whose URLs and checksums match the canonical assets named by that contract.
2. **Given** metadata that declares valid Winget and Scoop channel contracts, **When** the Windows renderers run, **Then** they emit manifests that reference the canonical Windows asset and checksum with no hand-maintained package values.
3. **Given** metadata that omits or contradicts a required channel contract, **When** a renderer runs for that channel, **Then** it fails closed and reports the contract mismatch explicitly.

---

### User Story 3 - Keep Release Docs And Roadmap Aligned With The Shipped Contract (Priority: P3)

As a release reviewer, I want the release-facing docs and roadmap to describe the same `0.36.0` provenance contract the repository actually ships so future planning and publication stay auditable.

**Why this priority**: This slice is about trust in the release surface. If the roadmap or docs drift from the metadata contract, the feature is incomplete.

**Independent Test**: A reviewer can inspect the updated README, package publication guides, roadmap, changelog, and validation evidence and confirm they all describe one coherent `0.36.0` release provenance story.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a reviewer checks README, package publication guides, and changelog, **Then** they all describe GitHub Releases as the source of truth and package channels as derived surfaces.
2. **Given** the updated roadmap, **When** a maintainer checks the next-feature section, **Then** already delivered distribution work is not left behind as an active remaining candidate.
3. **Given** the completed validation report, **When** a reviewer inspects it, **Then** it records the real tests, lint, coverage, smoke or walkthrough evidence, and independent review outcomes for this slice.

### Edge Cases

- A canonical asset exists, but the channel contract names the wrong asset id or generated artifact path.
- A renderer template remains valid syntactically while pointing at URLs or checksums that no longer match the canonical metadata.
- Distribution metadata includes a channel contract for a package manager that the release bundle cannot actually support.
- README or publication guides continue to describe install or publication behavior that is no longer derivable from the canonical release manifest.
- A release notes or checksum manifest filename changes without the provenance contract being updated in lockstep.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon's release flow MUST emit one canonical distribution metadata artifact for each released version.
- **FR-002**: The canonical distribution metadata MUST record top-level provenance fields for version, tag, release URL, release notes artifact, checksum manifest artifact, source of truth, and generated timestamp.
- **FR-003**: The canonical distribution metadata MUST continue to record the complete canonical asset inventory, including platform, architecture, archive format, binary name, checksum, download URL, and supported distribution channels.
- **FR-004**: The canonical distribution metadata MUST additionally record explicit per-channel contracts for Homebrew, Winget, and Scoop.
- **FR-005**: Each channel contract MUST identify the channel name, the asset ids it is allowed to consume, and the generated artifact or manifest shapes the channel expects.
- **FR-006**: The metadata writer MUST derive provenance and channel-contract fields from the canonical release bundle rather than from hand-maintained package-manager values.
- **FR-007**: Release-surface verification MUST validate the top-level provenance fields and the per-channel contracts against the canonical release bundle.
- **FR-008**: Release-surface verification MUST fail closed when a channel contract references an asset id, download URL, checksum, or generated artifact expectation that does not match the canonical release inventory.
- **FR-009**: Homebrew, Winget, and Scoop renderers MUST consume the explicit channel contracts rather than silently assuming channel eligibility from hardcoded script knowledge alone.
- **FR-010**: A renderer MUST fail closed when its required channel contract is absent or inconsistent instead of inferring a replacement contract.
- **FR-011**: The feature MUST preserve GitHub Releases as the single source of truth for published binaries, filenames, checksums, and release notes.
- **FR-012**: The feature MUST NOT introduce a parallel build or packaging pipeline for any distribution channel.
- **FR-013**: The feature MUST preserve the existing package-manager set of Homebrew, Winget, and Scoop without adding a new channel in this slice.
- **FR-014**: The feature MUST preserve Canon runtime behavior, modeled mode semantics, `.canon/` persistence, publish destinations, and approval posture unchanged.
- **FR-015**: Release-facing docs MUST describe the provenance model and keep direct-download fallback guidance aligned with the derived package-manager channels.
- **FR-016**: The roadmap MUST be cleaned so the active next-feature section reflects the post-`0.36.0` state rather than stale already-delivered distribution work.
- **FR-017**: Cargo manifests, runtime compatibility references, impacted docs, and `CHANGELOG.md` MUST align to `0.36.0` for this delivery.
- **FR-018**: The generated task plan for this feature MUST include an explicit version-bump task and an explicit impacted-docs-plus-changelog task.
- **FR-019**: The generated task plan for this feature MUST include explicit coverage, `cargo clippy`, and `cargo fmt` closeout tasks.
- **FR-020**: Modified or newly created Rust files in this slice MUST receive focused automated validation coverage before the feature is complete.
- **FR-021**: Validation evidence for this slice MUST record the real release-surface checks, channel rendering checks, docs or roadmap review, and independent review outcome.

### Key Entities *(include if feature involves data)*

- **Release Provenance Record**: The top-level machine-readable record that binds a Canon version to its canonical GitHub Release tag, release notes, checksum manifest, asset inventory, and source-of-truth declaration.
- **Distribution Asset Record**: One canonical release artifact entry containing asset id, filename, OS, architecture, archive format, binary name, checksum, download URL, and supported channels.
- **Distribution Channel Contract**: The explicit per-channel declaration that names which canonical asset ids a channel may consume and which generated artifacts should result.
- **Generated Channel Artifact Expectation**: The durable expectation for a rendered Homebrew formula, Winget manifest set, or Scoop manifest derived from a channel contract.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can generate and verify the canonical release provenance manifest for a valid release bundle in one bounded workflow without manually comparing package-manager metadata.
- **SC-002**: Release-surface verification blocks 100% of tested cases where channel contracts drift from canonical asset URLs, checksums, or allowed asset ids.
- **SC-003**: Homebrew, Winget, and Scoop renderers all consume the same canonical metadata contract and emit artifacts that reference the same release version and checksums.
- **SC-004**: A reviewer can inspect README, publication guides, roadmap, changelog, and the validation report and find one coherent `0.36.0` provenance story in under 5 minutes.

## Validation Plan *(mandatory)*

- **Structural validation**: distribution metadata contract checks, release-surface verification checks, JSON or manifest validation, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **Logical validation**: focused tests for provenance metadata generation, channel-renderer fail-closed behavior, release-surface verification, and documentation or roadmap consistency scenarios.
- **Independent validation**: a separate review pass over the provenance contract, release docs, roadmap cleanup, and recorded validation evidence after implementation lands.
- **Evidence artifacts**: `specs/036-release-provenance-integrity/validation-report.md`, focused release-surface tests, generated release metadata examples, updated docs, and `lcov.info` for touched Rust files.

## Decision Log *(mandatory)*

- **D-001**: Strengthen the existing canonical release metadata and verification flow instead of creating a new packaging pipeline, **Rationale**: Canon already has one release bundle and three derived package-manager channels, so the highest-value next step is to make their shared provenance explicit and fail closed on drift.

## Non-Goals

- Introduce a new package-manager channel or a new release host.
- Add binary signing, external attestation infrastructure, or trust-policy distribution beyond the existing checksum-based release surface.
- Change Canon runtime behavior, modeled modes, or machine-facing governance surfaces.
- Redesign archive naming or the canonical release asset set unless required to preserve current package-channel behavior.

## Assumptions

- Canon will continue to package and publish the same canonical archive set under GitHub Releases for macOS, Linux, and Windows.
- Homebrew, Winget, and Scoop remain the only repository-owned package-manager channels in scope for this release.
- The current release scripts and packaging templates are the right seams to extend rather than replacing them with a new release subsystem.
- Focused Rust validation for this feature can be delivered through repository-owned tests that exercise the release metadata contract and channel integrity behavior end to end.
