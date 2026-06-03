# Tasks: Systematic Debugging Mode

**Input**: Design documents from `/specs/067-systematic-debugging/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 0: Governance & Artifacts

**Purpose**: Mode, risk, scope, and artifact decisions tracking.

- [x] T001 Bump version in Cargo.toml workspace

## Phase 1: Setup

**Purpose**: Project initialization and basic structure

- [x] T002 Initialize new mode struct and module in crates/canon-engine/src/modes/debugging.rs

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [x] T003 Register `debugging` mode in crates/canon-engine/src/modes/mod.rs

## Phase 3: User Story 1 - Systematic Resolution of a Defect (Priority: P1)

**Goal**: Developers facing a reproducible bug need a systematic way to reproduce the defect, propose hypotheses, verify the failure, and apply a fix, so that the fix is verified to address the root cause without regressions.

**Independent Test**: Can be independently tested by feeding a known defect (e.g., a stack trace) into the mode and verifying the packet constraints are enforced.

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T004 [P] [US1] Integration test for debugging mode in tests/integration/debugging_mode_test.rs ensuring gates are enforced

### Implementation for User Story 1

- [x] T005 [US1] Implement Reproduction Gate and Root Cause linking in crates/canon-engine/src/modes/debugging.rs
- [x] T006 [US1] Enforce required artifacts (01-context, 02-reproduction, 03-root-cause, 04-fix-decision, 05-verification) in crates/canon-engine/src/modes/debugging.rs
- [x] T007 [US1] Integrate with evidence generation and verify red-to-green state transition in crates/canon-engine/src/modes/debugging.rs
- [x] T008 [US1] Generate debugging-trace.md artifact correctly in crates/canon-engine/src/modes/debugging.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T009 Ensure 95% of coverage on touched rust files (crates/canon-engine/src/modes/debugging.rs)
- [x] T010 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and ensure no issues
- [x] T011 Remove completed feature from roadmap/features/01-systematic-debugging.md
- [x] T012 Update general docs in docs/ or tech-docs/
- [x] T013 Update CHANGELOG.md with the new debugging mode
- [x] T014 Update README.md to list debugging mode among available modes

## Dependencies & Execution Order

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete
