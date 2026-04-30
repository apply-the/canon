# Tasks: Winget Distribution And Roadmap Refocus

**Input**: Design documents from `/specs/026-winget-distribution/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Independent review and closeout evidence are mandatory before completion.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the release boundary and durable records that permit
implementation to start

- [x] T001 Set Canon version `0.25.0` in `Cargo.toml`, `CHANGELOG.md`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions in `specs/026-winget-distribution/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/026-winget-distribution/validation-report.md`
- [x] T004 Confirm the Windows distribution contracts in `specs/026-winget-distribution/contracts/distribution-metadata.md` and `specs/026-winget-distribution/contracts/winget-manifest-bundle.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and packaging-surface setup

- [x] T005 Update agent context from `specs/026-winget-distribution/plan.md` into `AGENTS.md`
- [x] T006 [P] Create winget packaging templates in `packaging/winget/version.yaml.tpl`, `packaging/winget/defaultLocale.yaml.tpl`, and `packaging/winget/installer.yaml.tpl`
- [x] T007 [P] Create the winget renderer scaffold in `scripts/release/render-winget-manifests.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared release metadata and validation work that all user stories
depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T008 [P] Add failing Windows distribution coverage in `tests/release_026_winget_distribution.rs`
- [x] T009 [P] Add failing documentation regression coverage in `tests/release_026_winget_distribution.rs` and `tests/release_024_docs.rs`
- [x] T010 Extend Windows distribution metadata and release verification in `scripts/release/write-distribution-metadata.sh` and `scripts/release/verify-release-surface.sh`
- [x] T011 Implement deterministic winget manifest rendering in `scripts/release/render-winget-manifests.sh` and `packaging/winget/version.yaml.tpl`, `packaging/winget/defaultLocale.yaml.tpl`, `packaging/winget/installer.yaml.tpl`
- [x] T012 Capture foundational packaging evidence in `specs/026-winget-distribution/validation-report.md`

**Checkpoint**: Release metadata, manifest rendering, and verification surfaces recognize the Windows package-manager channel

---

## Phase 3: User Story 1 - Publish A Winget-Ready Release (Priority: P1) 🎯 MVP

**Goal**: Deliver the release workflow and publication bundle for the Windows package-manager channel

**Independent Test**: A maintainer can assemble a release bundle and obtain a valid Windows package-manager artifact set without manually deriving URLs or checksums.

### Validation for User Story 1 (MANDATORY)

- [x] T013 [P] [US1] Extend workflow and artifact assertions in `tests/release_026_winget_distribution.rs`
- [x] T014 [US1] Record story-specific release decisions under `## User Story 1 Decisions` in `specs/026-winget-distribution/decision-log.md`

### Implementation for User Story 1

- [x] T015 [US1] Wire winget manifest generation and artifact publication in `.github/workflows/release.yml`
- [x] T016 [P] [US1] Update release-note publication guidance in `.github/release-notes-template.md` and `CHANGELOG.md`
- [x] T017 [US1] Capture release workflow validation evidence in `specs/026-winget-distribution/validation-report.md`

**Checkpoint**: The release workflow emits a reviewable winget publication bundle alongside the canonical Windows release asset

---

## Phase 4: User Story 2 - Install Canon On Windows Through A Familiar Channel (Priority: P2)

**Goal**: Deliver explicit Windows install and upgrade guidance centered on `winget`

**Independent Test**: A Windows user can read the install docs and find the primary `winget` path plus the direct-download fallback without external context.

### Validation for User Story 2 (MANDATORY)

- [x] T018 [P] [US2] Add failing Windows install guidance assertions in `tests/release_026_winget_distribution.rs` and `tests/release_024_docs.rs`
- [x] T019 [US2] Record story-specific install-guidance decisions under `## User Story 2 Decisions` in `specs/026-winget-distribution/decision-log.md`

### Implementation for User Story 2

- [x] T020 [US2] Update Windows installation and upgrade guidance in `README.md`
- [x] T021 [US2] Capture install-guidance validation evidence in `specs/026-winget-distribution/validation-report.md`

**Checkpoint**: Windows documentation presents `winget` first and keeps archive fallback visible

---

## Phase 5: User Story 3 - Keep The Roadmap Focused On Concrete Value (Priority: P3)

**Goal**: Remove speculative protocol work from the active roadmap and re-center next work on concrete distribution and authoring/evidence improvements

**Independent Test**: A roadmap reader no longer sees Protocol Interoperability or MCP as active next work and can identify the concrete Windows distribution follow-on instead.

### Validation for User Story 3 (MANDATORY)

- [x] T022 [P] [US3] Add failing roadmap-priority assertions in `tests/release_026_winget_distribution.rs` and `tests/release_024_docs.rs`
- [x] T023 [US3] Record roadmap-refocus decisions under `## User Story 3 Decisions` in `specs/026-winget-distribution/decision-log.md`

### Implementation for User Story 3

- [x] T024 [US3] Remove Protocol Interoperability and refocus the next-feature narrative in `ROADMAP.md`
- [x] T025 [US3] Capture roadmap validation evidence in `specs/026-winget-distribution/validation-report.md`

**Checkpoint**: The roadmap is concrete, current, and free of MCP / Protocol Interoperability as active next work

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and release-quality closeout

- [x] T026 [P] Run the focused release and documentation test suite in `tests/release_026_winget_distribution.rs` and `tests/release_024_docs.rs`, then record results in `specs/026-winget-distribution/validation-report.md`
- [x] T027 [P] Run shell validation for `scripts/release/write-distribution-metadata.sh`, `scripts/release/verify-release-surface.sh`, and `scripts/release/render-winget-manifests.sh`, then record results in `specs/026-winget-distribution/validation-report.md`
- [x] T028 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record clean results in `specs/026-winget-distribution/validation-report.md`
- [x] T029 Perform independent review of release-surface integrity, Windows guidance, and roadmap cleanup in `specs/026-winget-distribution/validation-report.md`

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
- **User Story 2 (P2)**: Can start after Foundational. Reuses the release artifacts but remains independently testable through documentation.
- **User Story 3 (P3)**: Can start after Foundational. Reuses the docs and release context but remains independently testable through roadmap assertions.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected workflow or docs land.
- Release metadata and manifest rendering before workflow wiring.
- Workflow wiring before user-facing release guidance.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- `T006` and `T007` can run in parallel after Phase 0.
- `T008` and `T009` can run in parallel before `T010` and `T011`.
- Within User Story 1, release-note guidance and workflow wiring can run in parallel once the renderer and verification surfaces are stable.
- Final validation tasks `T026` through `T028` can run in parallel once implementation is stable, but `T029` must be last.

---

## Parallel Example: User Story 1

```bash
# Launch failing validation in parallel:
Task: "Extend workflow and artifact assertions in tests/release_026_winget_distribution.rs"
Task: "Update story-specific release decisions in specs/026-winget-distribution/decision-log.md"

# Launch compatible implementation slices in parallel after foundational work:
Task: "Wire winget manifest generation and artifact publication in .github/workflows/release.yml"
Task: "Update release-note publication guidance in .github/release-notes-template.md and CHANGELOG.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the release workflow emits a correct winget artifact bundle before expanding into docs and roadmap cleanup.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with focused tests, shell validation, formatting, clippy, and independent review.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
   - Developer A: User Story 1 release workflow and artifact work.
   - Developer B: User Story 2 Windows install guidance.
   - Developer C: User Story 3 roadmap cleanup and evidence updates.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- `T001` intentionally bumps the next release to `0.25.0`, which is the next semantic version after `0.24.0` even though the feature branch number is `026`
- Keep the decision log and validation report current as work progresses