# Tasks: Release Provenance And Channel Integrity

**Input**: Design documents from `/specs/036-release-provenance-integrity/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Validation**: Layered validation is mandatory. Add executable test tasks whenever behavior, interfaces, or regressions must be checked. Independent review, coverage evidence, and closeout artifacts are always required.

**Organization**: Tasks are grouped by user story to enable independent implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with explicit version, scope, invariant, and validation artifacts.
- No implementation task may appear before the artifacts and checks that authorize it.
- Every user story MUST include validation tasks before implementation tasks.
- Independent review, coverage evidence, and repository quality gates are mandatory before completion.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the bounded provenance scope, durable contracts, and version baseline that permit implementation to start.

- [x] T001 Set the workspace release version to `0.36.0` in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-phase decisions for provenance and fail-closed channel integrity in `specs/036-release-provenance-integrity/decision-log.md`
- [x] T003 Update planned structural, logical, independent, and coverage validation checkpoints in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T004 [P] Confirm the canonical metadata and channel-integrity contracts in `specs/036-release-provenance-integrity/contracts/distribution-metadata.md` and `specs/036-release-provenance-integrity/contracts/channel-integrity.md`
- [x] T005 [P] Refresh `AGENTS.md` from `specs/036-release-provenance-integrity/plan.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the focused validation and fixture scaffolding required by every story.

- [x] T006 [P] Create the focused release regression test scaffold in `tests/release_036_release_provenance_integrity.rs`
- [x] T007 [P] Extend version-alignment test scaffolding in `tests/integration/skills_bootstrap.rs`
- [x] T008 [P] Prepare provenance walkthrough expectations in `specs/036-release-provenance-integrity/quickstart.md` and `specs/036-release-provenance-integrity/validation-report.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared provenance and release-verification foundation that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T009 [P] Add failing canonical provenance-manifest assertions in `tests/release_036_release_provenance_integrity.rs`
- [x] T010 [P] Add failing version, compatibility, and release-doc alignment assertions in `tests/release_036_release_provenance_integrity.rs` and `tests/integration/skills_bootstrap.rs`
- [x] T011 Extend canonical provenance generation in `scripts/release/write-distribution-metadata.sh`
- [x] T012 Extend release-surface verification for provenance and channel drift in `scripts/release/verify-release-surface.sh`
- [x] T013 Capture foundational provenance evidence in `specs/036-release-provenance-integrity/validation-report.md`

**Checkpoint**: Canonical metadata now carries explicit provenance and the verifier can reject inconsistent release bundles before channel rendering proceeds.

---

## Phase 3: User Story 1 - Emit A Canonical Provenance-Rich Release Manifest (Priority: P1) 🎯 MVP

**Goal**: Deliver one canonical release manifest that records the source-of-truth bundle and fails closed when provenance or asset inventory drift.

**Independent Test**: A maintainer can generate distribution metadata from a valid synthetic release bundle and observe that provenance validation rejects missing or contradictory bundle data before any channel renderer is treated as ready.

### Validation for User Story 1 (MANDATORY)

- [x] T014 [P] [US1] Extend failing provenance and asset-inventory assertions in `tests/release_036_release_provenance_integrity.rs`
- [x] T015 [US1] Record story-specific provenance decisions under `## User Story 1 Decisions` in `specs/036-release-provenance-integrity/decision-log.md`

### Implementation for User Story 1

- [x] T016 [US1] Implement top-level provenance fields and source-of-truth declarations in `scripts/release/write-distribution-metadata.sh`
- [x] T017 [P] [US1] Implement explicit canonical-bundle validation for provenance records in `scripts/release/verify-release-surface.sh`
- [x] T018 [US1] Capture provenance-manifest validation evidence in `specs/036-release-provenance-integrity/validation-report.md`

**Checkpoint**: The repository can emit and validate a provenance-rich canonical manifest without relying on implicit package-manager knowledge.

---

## Phase 4: User Story 2 - Render Package Channels From Explicit Channel Contracts (Priority: P2)

**Goal**: Make Homebrew, Winget, and Scoop render strictly from explicit channel contracts declared by the canonical metadata.

**Independent Test**: A maintainer can render Homebrew, Winget, and Scoop artifacts from the same metadata file and observe that each renderer succeeds only when its declared channel contract is present and coherent.

### Validation for User Story 2 (MANDATORY)

- [x] T019 [P] [US2] Add failing channel-contract and renderer fail-closed assertions in `tests/release_036_release_provenance_integrity.rs`
- [x] T020 [US2] Record story-specific channel-contract decisions under `## User Story 2 Decisions` in `specs/036-release-provenance-integrity/decision-log.md`

### Implementation for User Story 2

- [x] T021 [US2] Implement explicit top-level channel contracts in `scripts/release/write-distribution-metadata.sh`
- [x] T022 [P] [US2] Make Homebrew rendering validate channel contracts in `scripts/release/render-homebrew-formula.sh`
- [x] T023 [P] [US2] Make Winget rendering validate channel contracts in `scripts/release/render-winget-manifests.sh`
- [x] T024 [P] [US2] Make Scoop rendering validate channel contracts in `scripts/release/render-scoop-manifest.sh`
- [x] T025 [US2] Capture channel-rendering validation evidence in `specs/036-release-provenance-integrity/validation-report.md`

**Checkpoint**: Every package-manager artifact is derived from an explicit channel contract and renderer drift fails closed.

---

## Phase 5: User Story 3 - Keep Release Docs And Roadmap Aligned With The Shipped Contract (Priority: P3)

**Goal**: Align release-facing docs, roadmap, changelog, and validation evidence with the delivered `0.36.0` provenance contract.

**Independent Test**: A reviewer can inspect README, package-publication guides, roadmap, changelog, compatibility references, and validation evidence and find one coherent `0.36.0` provenance story in under five minutes.

### Validation for User Story 3 (MANDATORY)

- [x] T026 [P] [US3] Add failing README, guide, roadmap, and version-reference assertions in `tests/release_036_release_provenance_integrity.rs` and `tests/integration/skills_bootstrap.rs`
- [x] T027 [US3] Record story-specific release-alignment decisions under `## User Story 3 Decisions` in `specs/036-release-provenance-integrity/decision-log.md`

### Implementation for User Story 3

- [x] T028 [US3] Update impacted docs and changelog for the `0.36.0` provenance release in `README.md`, `docs/guides/publishing-to-winget.md`, `docs/guides/publishing-to-scoop.md`, and `CHANGELOG.md`
- [x] T029 [US3] Clean roadmap continuity after the delivered provenance slice in `ROADMAP.md`
- [x] T030 [US3] Align release-facing version and compatibility references in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and `tests/integration/skills_bootstrap.rs`
- [x] T031 [US3] Capture docs, roadmap, and release-alignment evidence in `specs/036-release-provenance-integrity/validation-report.md`

**Checkpoint**: Documentation, roadmap, compatibility surfaces, and validation evidence all describe the shipped provenance contract consistently.

---

## Final Phase: Verification & Compliance

**Purpose**: Execute focused validation, capture coverage, and close the feature safely.

- [x] T032 [P] Run `cargo test --test release_036_release_provenance_integrity --test skills_bootstrap` and record results in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T033 [P] Run direct shell validation for `scripts/release/write-distribution-metadata.sh`, `scripts/release/render-homebrew-formula.sh`, `scripts/release/render-winget-manifests.sh`, `scripts/release/render-scoop-manifest.sh`, and `scripts/release/verify-release-surface.sh`, then record results in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T034 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and record changed Rust file coverage or direct-test coverage notes in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T035 [P] Run `cargo fmt` and `cargo fmt --check`, then record results in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T036 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and record results in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T037 [P] Run `cargo nextest run` and record results in `specs/036-release-provenance-integrity/validation-report.md`
- [x] T038 Perform independent review of provenance integrity, channel drift handling, release-doc alignment, roadmap cleanup, and final evidence in `specs/036-release-provenance-integrity/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the canonical provenance manifest MVP.
- **User Story 2 (P2)**: Depends on the provenance fields from US1 and reuses the verifier hardening from Foundational.
- **User Story 3 (P3)**: Depends on the shipped provenance contract from US1 and US2 so docs and roadmap describe real delivered behavior.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected script or documentation changes land.
- Shared metadata and verification changes happen before renderer or doc alignment tasks.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T004` and `T005` can run in parallel during governance.
- `T006` through `T008` can run in parallel during setup.
- `T009` and `T010` can run in parallel before `T011` and `T012`.
- Within User Story 2, Homebrew, Winget, and Scoop renderer tightening can run in parallel once the channel-contract schema is stable.
- Final validation tasks `T032` through `T037` can run in parallel once implementation is stable, but `T038` MUST be last.

---

## Parallel Example: User Story 2

```bash
# After explicit channel contracts are present in the metadata:
Task: "Make Homebrew rendering validate channel contracts in scripts/release/render-homebrew-formula.sh and packaging/homebrew/canon.rb.tpl"
Task: "Make Winget rendering validate channel contracts in scripts/release/render-winget-manifests.sh and packaging/winget/version.yaml.tpl"
Task: "Make Scoop rendering validate channel contracts in scripts/release/render-scoop-manifest.sh and packaging/scoop/canon.json.tpl"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the canonical provenance manifest can be generated and verified before renderer tightening begins.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with coverage, formatting, clippy, nextest, and independent review.

### Parallel Team Strategy

With multiple developers:

1. One developer owns metadata and verifier hardening.
2. One developer owns channel renderer tightening.
3. One developer owns docs, roadmap, changelog, and compatibility closeout after the metadata contract stabilizes.

---

## Notes

- `T001` is intentionally the explicit version-bump task requested for this feature.
- `T028` is intentionally the explicit impacted-docs-plus-changelog task requested for this feature.
- `T029` is intentionally the roadmap cleanup task requested for this feature.
- `T034`, `T035`, and `T036` are intentionally the explicit coverage, `cargo fmt`, and `cargo clippy` closeout tasks requested for this feature.
- Keep the decision log and validation report current as tasks close.