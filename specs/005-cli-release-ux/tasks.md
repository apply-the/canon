# Tasks: Installable CLI Distribution and Release UX

**Input**: Design documents from `/specs/005-cli-release-ux/`  
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md),
[research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/)

**Validation**: Layered validation is mandatory. Add executable test and
evidence tasks before implementation closeout, and keep independent review
separate from generation.

**Organization**: Tasks are grouped by user story to keep each increment
independently testable and auditable while preserving the workstream ordering
requested in the implementation plan.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`)
- Every task includes exact file paths

## Constitution Alignment

- Start with governance, approval, and evidence artifacts before implementation.
- Do not begin release or documentation work before invariants and validation
  ownership remain current.
- Keep runtime-compatibility changes limited to shared install/update guidance;
  do not redesign Canon engine or Codex skill core logic.
- Systemic-impact release publication requires independent review tasks before
  completion.

## Phase 0: Governance & Artifacts

**Purpose**: Keep the systemic-impact execution controls, decision
traceability, and validation ownership current before implementation starts.

- [x] T001 Refresh release approval checkpoints, reviewer ownership, and scope guards in `specs/005-cli-release-ux/decision-log.md` and `specs/005-cli-release-ux/validation-report.md`
- [x] T002 Add implementation-stage evidence tables for artifact completeness, version parity, install walkthroughs, and skill recovery in `specs/005-cli-release-ux/validation-report.md`
- [x] T003 Align implementation-ready notes in `specs/005-cli-release-ux/contracts/release-artifact-contract.md`, `specs/005-cli-release-ux/contracts/installation-runtime-compatibility-contract.md`, and `specs/005-cli-release-ux/contracts/version-visibility-contract.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the new repo-local release artifacts that later phases will
fill in.

- [x] T004 Create the release notes scaffold in `.github/release-notes-template.md`
- [x] T005 [P] Create the Unix packaging script entrypoint in `scripts/release/package-unix.sh`
- [x] T006 [P] Create the Windows packaging script entrypoint in `scripts/release/package-windows.ps1`
- [x] T007 [P] Create the release verification script entrypoint in `scripts/release/verify-release-surface.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the shared naming, matrix, and validation scaffolding
that all user stories depend on.

**⚠️ CRITICAL**: No user story should be closed until this phase is complete.

- [x] T008 Define the shared public asset names, target mapping, and checksum manifest naming in `.github/release-notes-template.md` and `specs/005-cli-release-ux/contracts/release-artifact-contract.md`
- [x] T009 [P] Update the cross-platform compile matrix to the five public release targets in `.github/workflows/ci.yml`
- [x] T010 [P] Add shared artifact-matrix and version-parity evidence placeholders in `specs/005-cli-release-ux/validation-report.md`
- [x] T011 Encode the final version-surface expectations and publication rationale in `specs/005-cli-release-ux/contracts/version-visibility-contract.md` and `specs/005-cli-release-ux/decision-log.md`

**Checkpoint**: Shared asset naming, target coverage, and evidence scaffolding
are stable.

---

## Phase 3: User Story 1 - Install Canon Once and Use It Anywhere (Priority: P1) 🎯 MVP

**Goal**: Deliver install-first documentation and validation flows so a user can
install Canon, verify `canon --version`, and run `canon init` without Cargo.

**Independent Test**: A fresh user follows the published install guidance,
verifies `canon --version`, confirms PATH resolution, and runs `canon init`
successfully in a new Git repository without using Cargo.

### Validation for User Story 1 (MANDATORY)

- [x] T012 [P] [US1] Add install walkthrough checkpoints and transcript placeholders in `specs/005-cli-release-ux/validation-report.md`
- [x] T013 [US1] Record install-path, PATH-shadowing, and contributor-doc separation decisions in `specs/005-cli-release-ux/decision-log.md`

### Implementation for User Story 1

- [x] T014 [US1] Rewrite install-first overview, platform install paths, verification steps, and contributor/source-build sections in `README.md`
- [x] T015 [P] [US1] Replace Cargo-based end-user flows with installed-binary usage in `specs/001-canon-spec/quickstart.md`
- [x] T016 [US1] Align platform install validation, checksum steps, and `canon init` smoke tests in `specs/005-cli-release-ux/quickstart.md`
- [x] T017 [US1] Capture expected install transcripts, PATH-resolution evidence, and `canon init` evidence notes in `specs/005-cli-release-ux/validation-report.md`

**Checkpoint**: User Story 1 is fully documented, independently testable, and
free of Cargo in the normal user journey.

---

## Phase 4: User Story 2 - Trust a Release as a Real CLI Delivery (Priority: P1)

**Goal**: Deliver a release pipeline that produces the agreed artifact matrix,
checksum manifest, release notes, and enforced version parity.

**Independent Test**: A release candidate run produces the five platform
archives plus the checksum manifest, and a reviewer can verify artifact names,
release notes, and binary versions all match.

### Validation for User Story 2 (MANDATORY)

- [x] T018 [P] [US2] Add release-candidate review checkpoints and parity evidence sections in `specs/005-cli-release-ux/validation-report.md`
- [x] T019 [US2] Record release workflow approval gates and publication decisions in `specs/005-cli-release-ux/decision-log.md`

### Implementation for User Story 2

- [x] T020 [P] [US2] Implement Unix archive packaging, flat archive layout, and user-facing asset naming in `scripts/release/package-unix.sh`
- [x] T021 [P] [US2] Implement Windows archive packaging, flat archive layout, and user-facing asset naming in `scripts/release/package-windows.ps1`
- [x] T022 [US2] Implement checksum generation, archive inspection, and version-surface verification in `scripts/release/verify-release-surface.sh`
- [x] T023 [US2] Create the GitHub release workflow with metadata, build, package, checksum, verify, and publish jobs in `.github/workflows/release.yml`
- [x] T024 [US2] Wire release notes rendering and version-parity enforcement into `.github/workflows/release.yml` and `.github/release-notes-template.md`
- [x] T025 [US2] Capture artifact completeness, checksum, and independent release-readiness evidence in `specs/005-cli-release-ux/validation-report.md`

**Checkpoint**: User Story 2 yields a reviewable release candidate with the
full artifact surface and hard version-parity gates.

---

## Phase 5: User Story 3 - Use Codex Skills on Top of an Installed Canon Binary (Priority: P2)

**Goal**: Keep Canon skills as thin frontends over the installed binary while
making missing or incompatible CLI state recoverable through release-based
guidance.

**Independent Test**: In a repository with Canon skills, the shared preflight
reports `STATUS=ready` when a valid Canon binary is on PATH and reports
release-based recovery guidance when Canon is missing or incompatible.

### Validation for User Story 3 (MANDATORY)

- [x] T026 [P] [US3] Add missing, incompatible, and PATH-shadowed Canon evidence cases in `specs/005-cli-release-ux/validation-report.md`
- [x] T027 [US3] Record shared compatibility guidance decisions in `specs/005-cli-release-ux/decision-log.md`

### Implementation for User Story 3

- [x] T028 [US3] Update release-based install and update guidance in `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T029 [US3] Align skill-facing recovery expectations with install/runtime rules in `specs/005-cli-release-ux/contracts/installation-runtime-compatibility-contract.md`
- [x] T030 [US3] Capture Bash and PowerShell ready/missing/incompatible validation expectations in `specs/005-cli-release-ux/validation-report.md`

**Checkpoint**: User Story 3 preserves the existing preflight contract while
switching recovery guidance from Cargo to release-based installation.

---

## Final Phase: Verification & Compliance

**Purpose**: Run cross-cutting validation, collect release evidence, and close
the systemic-impact compliance loop.

- [x] T031 [P] Run skill structure validation from `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1`, then record results in `specs/005-cli-release-ux/validation-report.md`
- [x] T032 [P] Run repository structural validation with `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --locked`, `cargo nextest run --locked`, and `git diff --check`, then record results in `specs/005-cli-release-ux/validation-report.md`
- [x] T033 Run a release dry run from `.github/workflows/release.yml` and record install walkthrough evidence in `specs/005-cli-release-ux/validation-report.md`
- [x] T034 Perform independent release-readiness review and record findings in `specs/005-cli-release-ux/validation-report.md`
- [x] T035 Confirm final invariants, approval gates, and closeout notes in `specs/005-cli-release-ux/decision-log.md` and `specs/005-cli-release-ux/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. Must complete first.
- **Phase 1: Setup**: Depends on Phase 0. Creates the new files used later.
- **Phase 2: Foundational**: Depends on Phase 1. Blocks user story completion.
- **Phase 3: User Story 1**: Depends on Phase 2.
- **Phase 4: User Story 2**: Depends on Phase 2. It can overlap with User
  Story 1 once shared naming and evidence scaffolding are stable.
- **Phase 5: User Story 3**: Depends on Phase 2 and should finish after User
  Story 1 install guidance is stable.
- **Final Phase**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Phase 2 and provides the install-first
  user journey.
- **User Story 2 (P1)**: Starts after Phase 2 and turns the artifact contract
  into a publishable release pipeline.
- **User Story 3 (P2)**: Starts after Phase 2 but should use the final install
  guidance and release surfaces from User Stories 1 and 2.

### Within Each User Story

- Validation and evidence tasks come before implementation closeout.
- Decision-log updates precede or accompany behavior changes that affect user
  guidance or release policy.
- Packaging scripts precede the release workflow that invokes them.
- Documentation and compatibility guidance must be aligned before final
  validation runs.

### Dependency Graph

- `Phase 0 -> Phase 1 -> Phase 2 -> US1 -> Final Phase`
- `Phase 0 -> Phase 1 -> Phase 2 -> US2 -> Final Phase`
- `Phase 0 -> Phase 1 -> Phase 2 -> US3 -> Final Phase`
- `US1 + US2 + US3 -> T033 -> T034 -> T035`

---

## Parallel Opportunities

- `T005`, `T006`, and `T007` can run in parallel because they create different
  release helper files.
- `T009` and `T010` can run in parallel once Phase 1 completes.
- `T015` can run in parallel with `T014` because README and quickstart are
  separate files.
- `T020` and `T021` can run in parallel because Unix and Windows packaging are
  isolated.
- `T031` and `T032` can run in parallel during closeout.

## Parallel Example: User Story 1

```bash
Task: "Rewrite install-first overview, platform install paths, verification steps, and contributor/source-build sections in README.md"
Task: "Replace Cargo-based end-user flows with installed-binary usage in specs/001-canon-spec/quickstart.md"
```

## Parallel Example: User Story 2

```bash
Task: "Implement Unix archive packaging, flat archive layout, and user-facing asset naming in scripts/release/package-unix.sh"
Task: "Implement Windows archive packaging, flat archive layout, and user-facing asset naming in scripts/release/package-windows.ps1"
```

## Parallel Example: User Story 3

```bash
Task: "Update release-based install and update guidance in .agents/skills/canon-shared/references/runtime-compatibility.toml"
Task: "Capture Bash and PowerShell ready/missing/incompatible validation expectations in specs/005-cli-release-ux/validation-report.md"
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver User Story 1 so the install-first narrative, quickstart, and
   walkthroughs are correct.
3. Validate User Story 1 independently before widening to release automation.

### Incremental Delivery

1. Add User Story 1 for end-user install guidance.
2. Add User Story 2 for release automation and artifact parity.
3. Add User Story 3 for skill recovery guidance.
4. Finish with the Final Phase validation and independent review.

### Parallel Team Strategy

1. One contributor handles shared release helper scripts after Setup.
2. One contributor handles install-first documentation.
3. One contributor handles release workflow and parity gating after the helper
   scripts stabilize.
4. One reviewer closes the independent release-readiness evidence at the end.

---

## Notes

- `[P]` tasks touch separate files or non-overlapping surfaces.
- User story labels preserve traceability back to the spec.
- No task proposes changes to Canon Rust engine behavior or Codex skill core
  logic.
- Keep `specs/005-cli-release-ux/decision-log.md` and
  `specs/005-cli-release-ux/validation-report.md` current throughout
  implementation.