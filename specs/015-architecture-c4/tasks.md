# Tasks: Stronger Architecture Outputs (C4 Model)

**Input**: Design documents from `/specs/015-architecture-c4/`
**Prerequisites**: spec.md, plan.md, research.md, data-model.md, contracts/, quickstart.md, decision-log.md, validation-report.md

**Validation**: Layered validation is mandatory. Add executable test tasks whenever behavior, interfaces, or regressions must be checked. Independent review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- All paths are repo-root-relative.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start

- [x] T001 Confirm execution mode `change`, risk classification `bounded-impact`, scope boundaries, and invariants in `specs/015-architecture-c4/spec.md` and `specs/015-architecture-c4/plan.md`
- [x] T002 Confirm decision log seed in `specs/015-architecture-c4/decision-log.md`
- [x] T003 Confirm validation report scaffold in `specs/015-architecture-c4/validation-report.md`
- [x] T004 Note that bounded-impact does not require explicit human approval gates beyond the standard merge review

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T005 Verify `cargo +1.95.0 build --workspace` builds clean before starting
- [x] T006 Verify `cargo test --workspace` passes before starting (baseline)
- [x] T007 [P] Confirm directories `docs/templates/canon-input/architecture/` and `docs/examples/canon-input/architecture/` exist (create if missing)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before user-story work

- [x] T008 Add helper to `crates/canon-engine/src/artifacts/markdown.rs` (or extend `extract_marker`) so the architecture renderer can extract authored sections by canonical H2 heading using exact match, returning `Option<String>` with verbatim body. Verify nearby `extract_marker` helpers cover the case before adding a new one.
- [x] T009 Define a single shared constant for the missing-body marker text used by all three new C4 artifacts so test and renderer share one source of truth (define in `crates/canon-engine/src/artifacts/markdown.rs`).

**Checkpoint**: shared extraction + marker plumbing in place

---

## Phase 3: User Story 1 - Architect gets a C4-shaped architecture packet (Priority: P1) 🎯 MVP

**Goal**: emit `system-context.md`, `container-view.md`, and `component-view.md` alongside the existing five legacy artifacts when the brief authors the C4 sections.

**Independent Test**: run `architecture` with `docs/examples/canon-input/architecture/brief.md` (after T020) as `--input` and confirm 8 artifacts are produced, with the three C4 artifact bodies matching the brief sections verbatim.

### Validation for User Story 1 (MANDATORY)

- [x] T010 [P] [US1] Add failing contract test `tests/contract/architecture_c4_contract.rs` that asserts the architecture artifact contract list contains all 8 artifacts and that the gate associations match `specs/015-architecture-c4/contracts/architecture-c4.md`.
- [x] T011 [P] [US1] Add failing renderer test `tests/architecture_c4_renderer.rs` that calls `render_architecture_artifact` for each new C4 file with an authored brief and asserts the body contains the verbatim authored content.

### Implementation for User Story 1

- [x] T012 [US1] Extend the architecture entry in `crates/canon-engine/src/artifacts/contract.rs` to include `system-context.md`, `container-view.md`, and `component-view.md` with the gate sets defined in `specs/015-architecture-c4/contracts/architecture-c4.md`.
- [x] T013 [US1] Extend `render_architecture_artifact` in `crates/canon-engine/src/artifacts/markdown.rs` with three new match arms for the C4 artifacts. Each arm extracts the canonical H2 section from `context_summary` (which the orchestrator already populates with the authored brief) and renders the authored body verbatim under the artifact title heading.
- [x] T014 [US1] Add a focused integration-style test `tests/architecture_c4_run.rs` that runs an end-to-end `architecture` run with an inline authored brief containing all three C4 sections and asserts the 8 artifacts exist on disk under `.canon/artifacts/<RUN_ID>/architecture/` with the C4 bodies matching the authored content.
- [x] T015 [US1] Capture evidence (test names + run id pattern) in `specs/015-architecture-c4/validation-report.md`

**Checkpoint**: User Story 1 is fully functional and independently validated

---

## Phase 4: User Story 2 - Honest blocker when C4 content is missing (Priority: P2)

**Goal**: when an authored C4 H2 section is absent in the brief, the corresponding C4 artifact is still emitted but contains an explicit `## Missing Authored Body` marker.

**Independent Test**: run `architecture` with a brief that omits one or more C4 sections and confirm the corresponding artifacts contain the literal marker, and that the run state reflects the gap honestly without fabricated content.

### Validation for User Story 2 (MANDATORY)

- [x] T016 [P] [US2] Extend `tests/architecture_c4_renderer.rs` with cases that omit each C4 section individually, one that omits all three, and one that uses a near-miss heading variant (for example `## C4 - System Context`); assert the rendered output contains `## Missing Authored Body` for each omitted or non-canonical artifact and verbatim authored content only for canonical-heading present ones.
- [x] T017 [P] [US2] Extend `tests/architecture_c4_run.rs` with a run case where the brief omits all C4 sections and asserts the three C4 artifacts on disk contain the missing-body marker.

### Implementation for User Story 2

- [x] T018 [US2] In `render_architecture_artifact`, ensure each C4 match arm returns the missing-body markdown when the canonical heading is absent or the extracted body is empty after trimming. Use the shared marker constant added in T009.
- [x] T019 [US2] Capture evidence (test names and the chosen marker text) in `specs/015-architecture-c4/validation-report.md`

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: User Story 3 - Templates and examples bootstrap the new shape (Priority: P3)

**Goal**: ship a starter template and a realistic example for the architecture brief, including the new C4 sections, so users can author a credible packet without already knowing the contract.

**Independent Test**: read `docs/templates/canon-input/architecture/brief.md` and confirm every required H2 section is present; run `architecture` with `docs/examples/canon-input/architecture/brief.md` and confirm a fully authored 8-artifact packet with no missing-body markers.

### Validation for User Story 3 (MANDATORY)

- [x] T020 [P] [US3] Add `tests/architecture_c4_docs.rs` that opens `docs/templates/canon-input/architecture/brief.md` and asserts it contains every required H2 section (`## Decisions`, `## Invariants`, `## Tradeoffs`, `## Boundaries`, `## Readiness`, `## System Context`, `## Containers`, `## Components`).
- [x] T021 [P] [US3] Extend the same test or add a sibling test that reads `docs/examples/canon-input/architecture/brief.md` and asserts it contains the same required H2 sections plus authored bodies (non-empty after trimming).

### Implementation for User Story 3

- [x] T022 [US3] Create `docs/templates/canon-input/architecture/brief.md` with all required H2 sections and explanatory placeholders for each.
- [x] T023 [US3] Create `docs/examples/canon-input/architecture/brief.md` with a realistic, non-trivial authored brief that exercises all eight required H2 sections.
- [x] T024 [US3] Update `defaults/embedded-skills/canon-architecture/skill-source.md` to add an `Author Architecture Body Before Invoking Canon` section requiring authored C4 H2 sections, mirroring the structure used in `canon-backlog`.
- [x] T025 [US3] Mirror the embedded skill update into `.agents/skills/canon-architecture/SKILL.md` so the materialized skill stays in sync.
- [x] T026 [US3] Update `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1` only if a new validator-required phrase is introduced. If no new validator phrase is needed, leave validators untouched (and capture that decision in the validation report).
- [x] T027 [US3] Capture evidence (template/example paths, skill source diff summary) in `specs/015-architecture-c4/validation-report.md`

**Checkpoint**: All three user stories are independently functional

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation and closeout

- [x] T028 [P] Run structural validation: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `/bin/bash scripts/validate-canon-skills.sh`. Record results in `specs/015-architecture-c4/validation-report.md`.
- [x] T029 [P] Run logical validation: `cargo test --test architecture_c4_contract --test architecture_c4_renderer --test architecture_c4_run --test architecture_c4_docs` plus the existing architecture-related test suites for non-regression. Record results in `specs/015-architecture-c4/validation-report.md`.
- [x] T030 Perform independent review by generating an architecture packet in a temp repo using the example brief, publishing it, and reading only the published markdown to confirm the artifact set, verbatim authored bodies, and unchanged legacy critique artifacts. Record findings in `specs/015-architecture-c4/validation-report.md`.
- [x] T031 Update `ROADMAP.md` to mark `Stronger Architecture Outputs` as delivered and remove it from the candidate list. Update `docs/guides/modes.md` only if architecture-mode-facing guidance changes.
- [x] T032 Confirm invariants still hold and close the validation report (`Closeout: Passed`).

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 0 must complete first (governance preconditions).
- Phase 1 setup runs after Phase 0.
- Phase 2 foundational plumbing runs after Phase 1.
- Phase 3 (US1) runs after Phase 2.
- Phase 4 (US2) runs after Phase 3 (shares renderer code path).
- Phase 5 (US3) can begin in parallel with Phase 3 for docs (T022, T023) but the skill update (T024–T025) should land after the renderer is real (Phase 3 done) so the skill description matches actual behavior.
- Final Phase runs after all user stories complete.

### User Story Dependencies

- US1 depends on Phase 2 plumbing.
- US2 depends on US1 (same renderer code path).
- US3 depends on US1 conceptually for accuracy, but its docs files (T022, T023) may be authored in parallel with US1.

## Parallel Execution Examples

- T010 + T011 can run in parallel (different test files, both depend only on Phase 2 plumbing).
- T020 + T021 can run in parallel.
- T022, T023, T024, T025 touch different files and can be authored in parallel as long as the renderer is real.
- T028 + T029 in the final phase can run in parallel (independent commands).

## MVP Scope

User Story 1 alone produces a real C4 architecture packet. US2 and US3 add truthfulness and discoverability, respectively.
