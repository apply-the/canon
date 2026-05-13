# Tasks: Project Memory Promotion Policy

**Input**: Design documents from `specs/048-project-memory-promotion-policy/`
**Prerequisites**: plan.md (required), spec.md (required), contracts/ (required)

**Validation**: Layered validation is mandatory. Independent review and
evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 0: Version Bump

**Purpose**: Establish the release identity for this slice

- [x] T001 Bump workspace version to `0.48.0` in `Cargo.toml` (workspace.package.version), `README.md`, `assistant/plugin-metadata.json`, `.codex-plugin/plugin.json`, `.claude-plugin/manifest.json`, `.cursor-plugin/manifest.json`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`

---

## Phase 1: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start

- [x] T002 Verify execution mode, risk, scope, and invariants are recorded in `specs/048-project-memory-promotion-policy/spec.md` and `plan.md`
- [x] T003 Verify decision log exists at `specs/048-project-memory-promotion-policy/decision-log.md`

---

## Phase 2: Domain Types (Blocking Prerequisites)

**Purpose**: Core domain types that MUST be complete before publish logic

- [x] T004 Create `crates/canon-engine/src/domain/publish_profile.rs` with `PublishProfile` enum (`ProjectMemory`), `PromotionState` enum (`Auto`, `AutoIfApproved`, `PendingIndex`, `IndexOnly`, `EvidenceOnly`, `Manual`), `UpdateStrategy` enum (`ManagedBlocks`, `ProposalFiles`, `AppendOnlyIndex`), and `LineageMetadata` struct with required fields (`contract_version`, `source_run`, `mode`, `profile`, `promotion_state`, `approval_state`, `readiness`, `published_at`, `update_strategy`, `source_artifacts`)
- [x] T005 Register module in `crates/canon-engine/src/domain/mod.rs` and re-export from `crates/canon-engine/src/lib.rs`
- [x] T006 Create `defaults/policies/publish-profiles.toml` with default promotion-state and update-strategy per mode

**Checkpoint**: Domain types compile and serde round-trips pass

---

## Phase 3: User Story 1 - Promote With Policy Boundaries (Priority: P1)

**Goal**: Canon promotes governed output to project memory according to policy
states, distinguishing stable, pending, evidence-only, and manual outcomes.

**Independent Test**: Run a completed packet through each promotion state and
verify the publish path, metadata sidecar, and destination surface match policy.

### Validation for User Story 1 (MANDATORY)

- [x] T007 [US1] Write unit tests for `PromotionState` serde round-trip and `PublishProfile` defaults in `crates/canon-engine/src/domain/publish_profile.rs` (inline `#[cfg(test)]`)
- [x] T008 [US1] Write unit tests for promotion policy evaluation: each state maps to the correct publish-target category (stable / pending / evidence / manual)

### Implementation for User Story 1

- [x] T009 [US1] Add promotion policy evaluation function in `crates/canon-engine/src/orchestrator/publish.rs`: `evaluate_promotion_policy(mode, manifest, state, profile_config) -> PromotionState`
- [x] T010 [US1] Extend `PublishMetadata` sidecar struct with `profile`, `promotion_state`, and `update_strategy` fields (use serde defaults where tolerant parsing is still required within the current `0.1.x` line)
- [x] T011 [US1] Add profile-aware publish path in `publish_run`: when profile is `Some(ProjectMemory)`, evaluate promotion policy and route to appropriate destination (`docs/project/`, `docs/evidence/`, or proposal file)
- [x] T012 [US1] Extend `EngineService::publish()` in `crates/canon-engine/src/orchestrator/service.rs` to accept `Option<PublishProfile>`
- [x] T013 [US1] Add `--profile project-memory` argument to publish subcommand in `crates/canon-cli/src/main.rs`
- [x] T014 [US1] Capture validation evidence in `specs/048-project-memory-promotion-policy/decision-log.md`

---

## Phase 4: User Story 2 - Preserve Lineage And Curated Documents (Priority: P1)

**Goal**: Every promoted document preserves source lineage and uses a
non-destructive update strategy.

**Independent Test**: Publish packets into managed-block, proposal-file, and
append-only-index targets and verify lineage fields and human-authored content
preservation.

### Validation for User Story 2 (MANDATORY)

- [x] T015 [US2] Write unit tests for `LineageMetadata` serde round-trip in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T016 [US2] Write unit tests for non-destructive update helpers: managed-block insertion preserves surrounding content, proposal-file emits without overwriting, append-only-index appends without rewriting

### Implementation for User Story 2

- [x] T017 [P] [US2] Implement `write_managed_block(target, block_id, content)` helper in `crates/canon-engine/src/orchestrator/publish.rs`: inserts or replaces only the Canon-owned range, preserving curated text
- [x] T018 [P] [US2] Implement `write_proposal_file(target, content, lineage)` helper: emits a `.proposal.md` next to the stable target
- [x] T019 [P] [US2] Implement `append_index_entry(target, entry, lineage)` helper: appends to an index surface without rewriting existing entries
- [x] T020 [US2] Emit `LineageMetadata` into the `packet-metadata.json` sidecar when profile is `ProjectMemory`
- [x] T021 [US2] Capture validation evidence

---

## Phase 5: User Story 3 - Publish A Stable Consumer Contract (Priority: P2)

**Goal**: The feature-local contract brief is promoted to a stable Canon
documentation path.

**Independent Test**: Verify the stable contract document at
`docs/integration/project-memory-promotion-contract.md` matches the accepted
feature-local contract.

### Implementation for User Story 3

- [x] T022 [US3] Copy accepted contract from `specs/048-project-memory-promotion-policy/contracts/boundline-project-memory-promotion-contract.md` to `docs/integration/project-memory-promotion-contract.md`
- [x] T023 [US3] Add a header note to the stable contract pointing back to the authoritative feature-local source

---

## Phase 6: Verification & Compliance

**Purpose**: Ensure everything compiles, passes, and is well-formatted

- [x] T024 Run `cargo fmt` and verify clean with `cargo fmt --check`
- [x] T025 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fix any issues
- [x] T026 Run `cargo nextest run` and verify all tests pass
- [x] T027 Increase coverage of modified files to ≥95% using `cargo llvm-cov`
- [x] T028 Update `docs/guides/modes.md` with project-memory publish profile documentation
- [x] T029 Update `CHANGELOG.md` with 0.48.0 entry
- [x] T030 Update `ROADMAP.md` if applicable

---

## Dependencies

```text
T001 → T004, T005, T006 (version bump before domain types)
T004 + T005 → T007, T008, T009 (domain types before publish logic)
T009 + T010 → T011, T012 (policy evaluation before profile-aware publish)
T012 → T013 (service before CLI)
T011 → T017, T018, T019, T020 (publish path before update helpers)
T017 + T018 + T019 can run in parallel (different files/functions)
T026 → T027 (tests pass before coverage)
T024, T025, T026, T027, T028, T029, T030 form the final verification phase
```

## Summary

- **Total tasks**: 30
- **Phase 0 (Version Bump)**: 1 task
- **Phase 1 (Governance)**: 2 tasks
- **Phase 2 (Domain Types)**: 3 tasks
- **Phase 3 (US1 - Policy Promotion)**: 8 tasks (2 validation + 6 implementation)
- **Phase 4 (US2 - Lineage & Updates)**: 7 tasks (2 validation + 5 implementation)
- **Phase 5 (US3 - Stable Contract)**: 2 tasks
- **Phase 6 (Verification)**: 7 tasks
- **Parallel opportunities**: T017/T018/T019 (update helpers), T007/T008 (domain tests)
- **MVP scope**: Phase 0-3 (version bump + governance + domain types + policy promotion)
