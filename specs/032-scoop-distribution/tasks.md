# Tasks: Scoop Distribution Follow-On

**Input**: Design documents from `/specs/032-scoop-distribution/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review, coverage evidence, and closeout artifacts are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with explicit version, scope, invariant, and
  validation artifacts.
- No implementation task may appear before the artifacts and checks that
  authorize it.
- Every user story MUST include validation tasks before implementation tasks.
- Independent review, coverage evidence, and repository quality gates are
  mandatory before completion.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the bounded release surface and durable records that
permit implementation to start.

- [x] T001 Update the workspace release version to `0.32.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-phase decisions in `specs/032-scoop-distribution/decision-log.md`
- [x] T003 Update planned structural, logical, independent, and coverage validation checkpoints in `specs/032-scoop-distribution/validation-report.md`
- [x] T004 [P] Confirm the shared release-surface contracts in `specs/032-scoop-distribution/contracts/distribution-metadata.md` and `specs/032-scoop-distribution/contracts/scoop-manifest.md`
- [x] T005 [P] Refresh `AGENTS.md` from `specs/032-scoop-distribution/plan.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the Scoop-specific scaffolding required by every story.

- [x] T006 [P] Create the Scoop packaging template in `packaging/scoop/canon.json.tpl`
- [x] T007 [P] Create the Scoop renderer scaffold in `scripts/release/render-scoop-manifest.sh`
- [x] T008 [P] Create the focused release regression test scaffold in `tests/release_032_scoop_distribution.rs`
- [x] T009 [P] Create the maintainer publication guide scaffold in `docs/guides/publishing-to-scoop.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared release metadata, verification, and test
foundation that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T010 [P] Add failing shared Windows distribution metadata and Scoop manifest assertions in `tests/release_032_scoop_distribution.rs`
- [x] T011 [P] Add failing version, release-notes, and documentation alignment assertions in `tests/release_032_scoop_distribution.rs` and `tests/integration/skills_bootstrap.rs`
- [x] T012 Extend shared Windows channel metadata and release-surface verification in `scripts/release/write-distribution-metadata.sh` and `scripts/release/verify-release-surface.sh`
- [x] T013 Implement deterministic Scoop manifest rendering in `scripts/release/render-scoop-manifest.sh` and `packaging/scoop/canon.json.tpl`
- [x] T014 Capture foundational packaging evidence in `specs/032-scoop-distribution/validation-report.md`

**Checkpoint**: The canonical release bundle now produces and verifies the
shared metadata and Scoop artifact needed by every story.

---

## Phase 3: User Story 1 - Publish A Scoop-Ready Release Artifact (Priority: P1) 🎯 MVP

**Goal**: Deliver the release workflow and publication artifact for the Scoop
channel.

**Independent Test**: A maintainer can assemble a release bundle and obtain a
valid Scoop manifest artifact whose URL, filename, and checksum match the
verified GitHub Release assets.

### Validation for User Story 1 (MANDATORY)

- [x] T015 [P] [US1] Extend workflow and generated-artifact assertions in `tests/release_032_scoop_distribution.rs`
- [x] T016 [US1] Record story-specific release decisions under `## User Story 1 Decisions` in `specs/032-scoop-distribution/decision-log.md`

### Implementation for User Story 1

- [x] T017 [US1] Wire Scoop manifest generation, verification, and artifact publication into `.github/workflows/release.yml`
- [x] T018 [P] [US1] Update release artifact guidance in `.github/release-notes-template.md` and `docs/guides/publishing-to-scoop.md`
- [x] T019 [US1] Capture release workflow validation evidence in `specs/032-scoop-distribution/validation-report.md`

**Checkpoint**: The release workflow emits a reviewable Scoop manifest artifact
alongside the canonical Windows zip.

---

## Phase 4: User Story 2 - Install Canon Through Scoop On Windows (Priority: P2)

**Goal**: Deliver explicit Scoop install and upgrade guidance while keeping the
archive fallback honest.

**Independent Test**: A Windows user can read the install docs and find the
Scoop install command, Scoop upgrade path, and direct-download fallback without
external context.

### Validation for User Story 2 (MANDATORY)

- [x] T020 [P] [US2] Add failing Scoop install-guidance assertions in `tests/release_032_scoop_distribution.rs`
- [x] T021 [US2] Record story-specific install-guidance decisions under `## User Story 2 Decisions` in `specs/032-scoop-distribution/decision-log.md`

### Implementation for User Story 2

- [x] T022 [US2] Update Scoop install and upgrade guidance plus Windows fallback wording in `README.md`, `docs/guides/publishing-to-scoop.md`, and `docs/guides/publishing-to-winget.md`
- [x] T023 [US2] Update impacted docs and changelog for the `0.32.0` release in `CHANGELOG.md` and `.github/release-notes-template.md`
- [x] T024 [US2] Capture install-guidance validation evidence in `specs/032-scoop-distribution/validation-report.md`

**Checkpoint**: Documentation presents Scoop clearly without hiding the direct
Windows archive fallback.

---

## Phase 5: User Story 3 - Keep Distribution And Roadmap Surfaces Focused (Priority: P3)

**Goal**: Keep delivered and remaining distribution work aligned across the
roadmap, release notes, and version-sensitive compatibility surfaces.

**Independent Test**: A reviewer can inspect the roadmap, version-sensitive
references, and release-facing docs and find no stale completed-feature blocks
or outdated workspace version expectations.

### Validation for User Story 3 (MANDATORY)

- [x] T025 [P] [US3] Add failing roadmap and version-reference assertions in `tests/release_032_scoop_distribution.rs` and `tests/integration/skills_bootstrap.rs`
- [x] T026 [US3] Record story-specific roadmap and compatibility decisions under `## User Story 3 Decisions` in `specs/032-scoop-distribution/decision-log.md`

### Implementation for User Story 3

- [x] T027 [US3] Update `ROADMAP.md`, `README.md`, and version-sensitive compatibility references for the delivered Scoop follow-on
- [x] T028 [US3] Capture roadmap and version-alignment validation evidence in `specs/032-scoop-distribution/validation-report.md`

**Checkpoint**: The roadmap, release docs, and compatibility references all
describe the same delivered distribution surface.

---

## Final Phase: Verification & Compliance

**Purpose**: Execute focused validation, capture coverage, and close the feature
safely.

- [x] T029 [P] Run `cargo test --test release_032_scoop_distribution --test skills_bootstrap` and record results in `specs/032-scoop-distribution/validation-report.md`
- [x] T030 [P] Run `/bin/bash scripts/validate-canon-skills.sh` and record results in `specs/032-scoop-distribution/validation-report.md`
- [x] T031 [P] Run direct shell validation for `scripts/release/write-distribution-metadata.sh`, `scripts/release/render-scoop-manifest.sh`, and `scripts/release/verify-release-surface.sh`, then record results in `specs/032-scoop-distribution/validation-report.md`
- [x] T032 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and record changed Rust file coverage or direct-test coverage notes in `specs/032-scoop-distribution/validation-report.md`
- [x] T033 [P] Run `cargo fmt` and `cargo fmt --check`, then record results in `specs/032-scoop-distribution/validation-report.md`
- [x] T034 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and record results in `specs/032-scoop-distribution/validation-report.md`
- [x] T035 [P] Run `cargo nextest run` and record results in `specs/032-scoop-distribution/validation-report.md`
- [x] T036 Perform independent review of release-surface integrity, install guidance, version alignment, and roadmap cleanup in `specs/032-scoop-distribution/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the MVP.
- **User Story 2 (P2)**: Can start after Foundational. Reuses the generated manifest but remains independently testable through docs.
- **User Story 3 (P3)**: Can start after Foundational. Reuses the docs and version surfaces but remains independently testable through roadmap and compatibility assertions.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected workflow or docs land.
- Shared metadata and verification changes happen before workflow or docs wiring.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T004` and `T005` can run in parallel during governance.
- `T006` through `T009` can run in parallel during setup.
- `T010` and `T011` can run in parallel before `T012` and `T013`.
- Within User Story 1, release-note guidance and workflow wiring can run in parallel once the renderer and verifier are stable.
- Final validation tasks `T029` through `T035` can run in parallel once implementation is stable, but `T036` MUST be last.

---

## Parallel Example: User Story 1

```bash
# After foundational metadata and renderer work is complete:
Task: "Wire Scoop manifest generation, verification, and artifact publication into .github/workflows/release.yml"
Task: "Update release artifact guidance in .github/release-notes-template.md and docs/guides/publishing-to-scoop.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the generated Scoop manifest matches the canonical release asset before expanding into docs and version cleanup.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with focused tests, shell validation, coverage, formatting, clippy, nextest, and independent review.

### Parallel Team Strategy

With multiple developers:

1. One developer owns metadata and verifier changes.
2. One developer owns the Scoop renderer, packaging template, and release workflow wiring.
3. One developer owns docs, roadmap, versioning, and compatibility-reference closeout after the artifact contract stabilizes.

---

## Notes

- Keep the Windows asset canonical and shared across `winget` and Scoop.
- Do not replace the direct-download install path.
- Do not hand-edit generated Scoop URLs or hashes after verification; regenerate from the release bundle instead.
- Keep decision-log and validation-report entries current as tasks close.