# Tasks: Authoring Experience And Packet Readiness

**Input**: Design documents from `/specs/039-authoring-packet-readiness/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Validation**: Layered validation is mandatory. Every runtime or docs change
must receive focused executable or reviewer-visible checks before full-suite
closeout.

**Organization**: Tasks are grouped by user story for traceability, but feature
`039` remains one macrofeature to be delivered whole rather than split into
separate slices.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- Every task includes exact file paths

## Phase 0: Governance & Artifacts

**Purpose**: Keep the execution authority artifacts current before mutation

- [x] T001 Refresh mode, risk, scope, invariants, and `0.39.0` obligations in `specs/039-authoring-packet-readiness/spec.md` and `specs/039-authoring-packet-readiness/plan.md` if implementation scope moves
- [x] T002 Keep accepted decisions and tradeoffs current in `specs/039-authoring-packet-readiness/decision-log.md`
- [x] T003 Keep evidence buckets current in `specs/039-authoring-packet-readiness/validation-report.md`
- [x] T004 Reconfirm the authoring-lifecycle and clarity-packet-shape contracts in `specs/039-authoring-packet-readiness/contracts/authoring-lifecycle.md` and `specs/039-authoring-packet-readiness/contracts/clarity-packet-shape.md` before code lands

---

## Phase 1: Foundational Authoring-Lifecycle Contract

**Purpose**: Establish one shared packet-shape and authority model that runtime and docs will reuse

**⚠️ CRITICAL**: No story work should invent a second authoring workflow outside this shared contract

- [x] T005 Define the additive authoring-lifecycle data surface in `crates/canon-engine/src/orchestrator/service.rs` so clarity output can carry packet shape, authority status, authoritative inputs, supporting inputs, readiness delta, and next authoring step
- [x] T006 Implement shared packet-role derivation in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, and `crates/canon-engine/src/orchestrator/service/clarity.rs` using explicit file-backed inputs without hidden inference
- [x] T007 [P] Add or tighten foundational engine coverage in `crates/canon-engine/src/orchestrator/service/tests.rs` and `tests/contract/inspect_clarity.rs` for single-file authority, `brief.md` preference, and ambiguous folder packets

**Checkpoint**: One shared authoring-lifecycle contract exists and has focused engine-level tests

---

## Phase 2: User Story 1 - Understand Packet Readiness Before Run (Priority: P1) 🎯 MVP

**Goal**: Make `inspect clarity` tell the operator which inputs control readiness, which only support the packet, and what still needs to be authored

**Independent Test**: A reviewer can inspect clarity output for single-file, carry-forward directory, and ambiguous folder packets and determine authority plus next authoring step without extra inference

### Validation for User Story 1 (MANDATORY)

- [x] T008 [P] [US1] Add failing clarity-contract assertions in `tests/contract/inspect_clarity.rs` for packet shape, authoritative inputs, supporting inputs, and readiness delta
- [x] T009 [P] [US1] Add failing renderer expectations in `crates/canon-cli/src/output.rs` for a dedicated authoring-lifecycle section in clarity markdown
- [x] T010 [US1] Record packet-authority and readiness-delta decisions in `specs/039-authoring-packet-readiness/decision-log.md`

### Implementation for User Story 1

- [x] T011 [US1] Implement the authoring-lifecycle summary and readiness-delta assembly in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, and `crates/canon-engine/src/orchestrator/service/clarity.rs`
- [x] T012 [US1] Update `crates/canon-cli/src/output.rs` to render the new packet-shape, authority, support-input, and next-step guidance coherently for clarity output
- [x] T013 [US1] Export any new clarity summary types through `crates/canon-engine/src/lib.rs` if the public engine contract requires them
- [x] T014 [US1] Capture US1 validation notes and output examples in `specs/039-authoring-packet-readiness/validation-report.md`

**Checkpoint**: Clarity output is independently readable and exposes authority plus readiness honestly

---

## Phase 3: User Story 2 - Follow One Canonical Authoring Lifecycle (Priority: P2)

**Goal**: Align shared docs, example packets, and inspect-clarity skill guidance around one lifecycle from authored brief to publishable packet

**Independent Test**: A reviewer can inspect the shared authoring docs, carry-forward example, and inspect-clarity skill and find the same lifecycle and authority story that the runtime exposes

### Validation for User Story 2 (MANDATORY)

- [x] T015 [P] [US2] Add or tighten docs or skill assertions in a new `tests/inspect_clarity_authoring_docs.rs` file and any affected existing authoring-doc tests for the shared lifecycle language and source or mirror sync
- [x] T016 [US2] Record docs and skill guidance decisions in `specs/039-authoring-packet-readiness/decision-log.md`

### Implementation for User Story 2

- [x] T017 [US2] Update `docs/guides/modes.md` to explain the shared file-backed lifecycle and the runtime authority rules for packet readiness
- [x] T018 [US2] Add or update shared template-facing guidance in `docs/templates/canon-input/README.md` so authors see the same lifecycle before choosing a mode-specific template
- [x] T019 [US2] Update `docs/examples/canon-input/carry-forward-packets.md` so `brief.md`, `source-map.md`, optional narrowed context, critique, and publish guidance match the delivered runtime contract
- [x] T020 [US2] Update `defaults/embedded-skills/canon-inspect-clarity/skill-source.md` and `.agents/skills/canon-inspect-clarity/SKILL.md` to match the lifecycle and authority guidance shipped by the runtime
- [x] T021 [US2] Capture US2 validation evidence in `specs/039-authoring-packet-readiness/validation-report.md`

**Checkpoint**: Shared authoring docs and inspect-clarity skill tell the same story as the runtime

---

## Phase 4: User Story 3 - Ship 039 As The Coherent Release Line (Priority: P3)

**Goal**: Ship the authoring-lifecycle slice with matching docs, roadmap, changelog, and explicit `0.39.0` alignment

**Independent Test**: Release-alignment checks and human review show one coherent `0.39.0` story for authoring experience and packet readiness

### Validation for User Story 3 (MANDATORY)

- [x] T022 [P] [US3] Add or update release-surface assertions in `tests/release_036_release_provenance_integrity.rs` and `tests/integration/skills_bootstrap.rs` for the delivered `0.39.0` version line and skill install expectations
- [x] T023 [US3] Record release-surface decisions in `specs/039-authoring-packet-readiness/decision-log.md`

### Implementation for User Story 3

- [x] T024 [US3] Bump the workspace release line to `0.39.0` in `Cargo.toml`, `Cargo.lock`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- [x] T025 [US3] Update `README.md`, `docs/guides/publishing-to-winget.md`, and `docs/guides/publishing-to-scoop.md` for the delivered `0.39.0` authoring-lifecycle release line where version surfaces are shown
- [x] T026 [US3] Update `CHANGELOG.md` with the delivered authoring experience and packet readiness behavior
- [x] T027 [US3] Clean `ROADMAP.md` so the delivered `039` slice is removed from future-work listings after completion
- [x] T028 [US3] Capture US3 validation evidence in `specs/039-authoring-packet-readiness/validation-report.md`

**Checkpoint**: Runtime behavior and repository release guidance are aligned for the delivered slice

---

## Final Phase: Verification & Compliance

**Purpose**: Close the slice with focused evidence, workspace hygiene, and final review

- [x] T029 [P] Run `cargo test --test inspect_clarity`, `cargo test -p canon-cli clarity_markdown_surfaces_questions_and_signals`, `cargo test --test release_036_release_provenance_integrity`, and `cargo test --test skills_bootstrap skills_install_for_codex_carries_current_runtime_compatibility_reference`, then record results in `specs/039-authoring-packet-readiness/validation-report.md`
- [x] T030 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and record coverage implications for touched Rust files in `specs/039-authoring-packet-readiness/validation-report.md`
- [x] T031 [P] Run `cargo fmt --check` and fix any formatting fallout in touched files
- [x] T032 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and resolve any issues introduced by this slice
- [x] T033 Run `cargo nextest run` and record full-suite closeout evidence in `specs/039-authoring-packet-readiness/validation-report.md`
- [x] T034 Perform an independent review of authoring-lifecycle honesty, explicitly challenge hidden-input-inference risk, confirm invariants still hold, and close `specs/039-authoring-packet-readiness/validation-report.md`
- [x] T035 Prepare the final commit message for the delivered `039` slice and record it with the closeout notes

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0**: Must remain current before mutation and closeout.
- **Phase 1**: Must land before renderer, docs, and release surfaces diverge.
- **Phase 2**: Depends on Phase 1.
- **Phase 3**: Depends on Phase 1 and the delivered clarity contract.
- **Phase 4**: Depends on the delivered runtime and docs surfaces.
- **Final Phase**: Depends on all story work being complete.

### User Story Dependencies

- **US1 (P1)**: Starts after foundational contract work and establishes the runtime MVP.
- **US2 (P2)**: Depends on the US1 contract so docs and skills describe the real runtime behavior.
- **US3 (P3)**: Depends on the delivered runtime and docs story so release surfaces describe the actual `0.39.0` slice.

### Parallel Opportunities

- T007 can run in parallel across engine and contract test files once the shared packet-role design is chosen.
- T008 and T009 can run in parallel as failing checks for US1.
- T015 and T022 can run in parallel once the target lifecycle wording and version line are settled.
- T029 through T032 can run as separate closeout commands once code changes are stable.

## Implementation Strategy

### Whole-Feature Delivery

1. Lock the shared packet-shape and authority derivation in engine code.
2. Make CLI clarity render the same lifecycle and readiness story.
3. Align shared docs, template-facing guidance, and inspect-clarity skill text to that contract.
4. Finish with `0.39.0` release alignment, roadmap cleanup, coverage, lint, format, nextest, and final commit-message preparation.

## Notes

- Keep the first executable validation focused on the touched clarity slice before widening to workspace-wide checks.
- Do not add a new mode, new storage family, or hidden authored-input source while delivering this feature.
- Treat `0.39.0` alignment, docs or changelog updates, roadmap cleanup, coverage, `cargo fmt`, and `cargo clippy` as part of the feature definition, not optional cleanup.