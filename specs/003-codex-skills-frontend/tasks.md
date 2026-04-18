# Tasks: Codex Skills Frontend for Canon

**Input**: Design documents from `specs/003-codex-skills-frontend/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. Structural checks, failure-path checks, walkthroughs, overlap checks, and independent review are required before closeout.

**Organization**: Tasks are grouped by delivery slice and mapped to user stories where applicable so the frontend can land incrementally without weakening Canon runtime authority.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on incomplete tasks)
- **[Story]**: User story label where applicable
- Every task includes an exact file path and a brief acceptance note

## Phase 0: Governance & Contracts

**Purpose**: Lock the vocabulary, contract, and validation rules that authorize implementation.

- [X] T001 Update `specs/003-codex-skills-frontend/contracts/skill-contract.md`; acceptance: required `SKILL.md` fields, two skill classes, and Canon CLI authority are frozen for implementation.
- [X] T002 Update `specs/003-codex-skills-frontend/contracts/skill-state-and-failure-contract.md`; acceptance: `available-now`, `modeled-only`, `intentionally-limited`, and `experimental` are defined consistently with deterministic failure responses.
- [X] T003 Update `specs/003-codex-skills-frontend/decision-log.md`; acceptance: implementation-start decisions record discoverability, helper-layer boundaries, and MVP cut line.
- [X] T004 Update `specs/003-codex-skills-frontend/validation-report.md`; acceptance: structural, overlap, failure, walkthrough, and independent-review checkpoints are fully enumerated.
- [X] T005 Update `specs/003-codex-skills-frontend/quickstart.md`; acceptance: quickstart leads with runnable skills and points to the full discoverable skill taxonomy.

---

## Phase 1: Shared Frontend Spine and Taxonomy Presence

**Purpose**: Materialize the shared deterministic support layer and make the full Canon skill taxonomy discoverable in the repo.

- [X] T006 Create new file to be created intentionally `.agents/skills/canon-shared/references/runtime-compatibility.toml`; acceptance: file pins the Canon CLI compatibility contract consumed by skill preflight checks.
- [X] T007 Create new file to be created intentionally `.agents/skills/canon-shared/references/support-states.md`; acceptance: file defines the four support states and the required wording rules for each.
- [X] T008 Create new file to be created intentionally `.agents/skills/canon-shared/references/output-shapes.md`; acceptance: file defines canonical output shapes for runnable, gated, failure, and non-runnable skill responses.
- [X] T009 Create new file to be created intentionally `.agents/skills/canon-shared/references/skill-index.md`; acceptance: file lists every Canon skill, its support state, and its nearest related skills.
- [X] T010 Create new file to be created intentionally `.agents/skills/canon-shared/references/skill-template.md`; acceptance: file captures the base `SKILL.md` contract, fixed field checklist, and section order used to stamp all Canon skills semi-mechanically.
- [X] T011 Create new file to be created intentionally `.agents/skills/canon-shared/scripts/check-runtime.sh`; acceptance: script returns deterministic status for CLI presence, version, repo context, `.canon/` initialization, and missing inputs.
- [X] T012 Create new file to be created intentionally `.agents/skills/canon-shared/scripts/check-runtime.ps1`; acceptance: PowerShell preflight behavior matches the shell contract from T011.
- [X] T013 Create new file to be created intentionally `.agents/skills/canon-shared/scripts/render-support-state.sh`; acceptance: script renders deterministic modeled-only and intentionally-limited responses without fabricating Canon runtime state.
- [X] T014 Create new file to be created intentionally `.agents/skills/canon-shared/scripts/render-support-state.ps1`; acceptance: PowerShell output matches the shell support-state renderer contract from T013.
- [X] T015 Create new file to be created intentionally `.agents/skills/canon-shared/scripts/render-next-steps.sh`; acceptance: script renders deterministic inspect/approve/resume guidance for Canon-backed skill results.
- [X] T016 Create new file to be created intentionally `.agents/skills/canon-shared/scripts/render-next-steps.ps1`; acceptance: PowerShell output matches the shell next-step renderer contract from T015.
- [X] T017 Create new file to be created intentionally `scripts/validate-canon-skills.sh`; acceptance: validator entrypoint exists for lightweight structural, label, overlap, and fake-run checks only.
- [X] T018 Create new file to be created intentionally `scripts/validate-canon-skills.ps1`; acceptance: PowerShell validator entrypoint mirrors the shell validator contract from T017 without expanding scope beyond the lightweight checks.
- [X] T019 Implement validator logic in `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1`; acceptance: validators fail on missing skill files, missing required sections, invalid support-state labels, overlap rule regressions, or support-state skills that attempt fake runs, and they do not attempt semantic repo parsing or generic repo analysis.
- [X] T020 [P] Create new file to be created intentionally `.agents/skills/canon-init/SKILL.md`; acceptance: initial skill file exists with required sections, `available-now` state, and `canon init` binding placeholder.
- [X] T021 [P] Create new file to be created intentionally `.agents/skills/canon-status/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T022 [P] Create new file to be created intentionally `.agents/skills/canon-inspect-invocations/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T023 [P] Create new file to be created intentionally `.agents/skills/canon-inspect-evidence/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T024 [P] Create new file to be created intentionally `.agents/skills/canon-inspect-artifacts/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T025 [P] Create new file to be created intentionally `.agents/skills/canon-approve/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T026 [P] Create new file to be created intentionally `.agents/skills/canon-resume/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T027 [P] Create new file to be created intentionally `.agents/skills/canon-requirements/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T028 [P] Create new file to be created intentionally `.agents/skills/canon-brownfield/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T029 [P] Create new file to be created intentionally `.agents/skills/canon-pr-review/SKILL.md`; acceptance: initial skill file exists with required sections and `available-now` state.
- [X] T030 [P] Create new file to be created intentionally `.agents/skills/canon-discovery/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T031 [P] Create new file to be created intentionally `.agents/skills/canon-system-shaping/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T032 [P] Create new file to be created intentionally `.agents/skills/canon-architecture/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T033 [P] Create new file to be created intentionally `.agents/skills/canon-implementation/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T034 [P] Create new file to be created intentionally `.agents/skills/canon-refactor/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T035 [P] Create new file to be created intentionally `.agents/skills/canon-review/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T036 [P] Create new file to be created intentionally `.agents/skills/canon-incident/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T037 [P] Create new file to be created intentionally `.agents/skills/canon-migration/SKILL.md`; acceptance: initial skill file exists with required sections, `modeled-only` state, and no runnable Canon binding.
- [X] T038 [P] Create new file to be created intentionally `.agents/skills/canon-verification/SKILL.md`; acceptance: initial skill file exists with required sections, `intentionally-limited` state, and no fake `canon verify` promise.
- [X] T039 Run `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1`, then record taxonomy-presence evidence in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: all Canon skills are discoverable and structurally valid before runnable refinement starts.

**Execution Note**: T020-T038 are intentionally repetitive taxonomy-stamping tasks. Execute them from the shared template and fixed checklist in T010 rather than treating each skill as a bespoke mini-project.

**Checkpoint**: Full `.agents/skills` taxonomy exists, shared helpers exist, `canon-init` is seeded, and non-runnable modes are already discoverable with honest initial support-state metadata.

---

## Phase 2: First Runnable Workflow Set (User Story 1 - P1) 🎯 MVP

**Goal**: Make the first Canon workflows runnable and inspectable from Codex without raw CLI memorization.

**Independent Test**: Invoke `$canon-requirements`, then use `$canon-status`, `$canon-inspect-invocations`, and `$canon-inspect-evidence` to confirm the run id and evidence surfaces come from real Canon runtime output.

### Validation for User Story 1 (MANDATORY)

- [X] T040 [P] [US1] Add runnable-skill walkthrough cases for `canon-requirements`, `canon-status`, `canon-inspect-invocations`, and `canon-inspect-evidence` in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: expected inputs, run-id behavior, and evidence pointers are explicit.
- [X] T041 [US1] Record User Story 1 execution decisions in `specs/003-codex-skills-frontend/decision-log.md`; acceptance: any skill-boundary or output-shape choices are captured before runnable skill polish lands.

### Implementation for User Story 1

- [X] T042 [US1] Refine `.agents/skills/canon-requirements/SKILL.md` to call shared preflight helpers and drive `canon run --mode requirements`; acceptance: skill returns real run-id guidance and never simulates Canon output.
- [X] T043 [US1] Refine `.agents/skills/canon-status/SKILL.md` to drive `canon status --run <RUN_ID>`; acceptance: skill returns real state, pending approvals, and next-step guidance.
- [X] T044 [US1] Refine `.agents/skills/canon-inspect-invocations/SKILL.md` to drive `canon inspect invocations --run <RUN_ID>`; acceptance: skill returns request-level decisions and links to related next steps.
- [X] T045 [US1] Refine `.agents/skills/canon-inspect-evidence/SKILL.md` to drive `canon inspect evidence --run <RUN_ID>`; acceptance: skill returns evidence lineage and no artifact-only summary.
- [X] T046 [US1] Update `.agents/skills/canon-shared/references/output-shapes.md` and `.agents/skills/canon-shared/references/skill-index.md`; acceptance: runnable examples and next-step contracts for the phase-2 skills are canonicalized.
- [X] T047 [US1] Record runnable-skill validation evidence in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: evidence confirms skills map to real Canon CLI behavior.

**Checkpoint**: `$canon-requirements`, `$canon-status`, `$canon-inspect-invocations`, and `$canon-inspect-evidence` are usable against the real Canon runtime.
**Release Gate**: Do not start Phase 3, Phase 4, or Phase 5 until this checkpoint has been exercised through real `$` invocation in Codex and the evidence is recorded in `specs/003-codex-skills-frontend/validation-report.md`.

---

## Phase 3: Unblock and Continue (User Story 2 - P2)

**Goal**: Make approval-gated and blocked Canon runs inspectable and resumable from Codex.

**Independent Test**: Use `$canon-approve`, `$canon-resume`, and `$canon-inspect-artifacts` against a gated run and confirm the outputs reflect real Canon approval and runtime state.

### Validation for User Story 2 (MANDATORY)

- [X] T048 [P] [US2] Add unblock-flow walkthrough cases for `canon-approve`, `canon-resume`, and `canon-inspect-artifacts` in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: approval targets, resume behavior, and artifact inspection expectations are explicit.
- [X] T049 [US2] Record User Story 2 decisions in `specs/003-codex-skills-frontend/decision-log.md`; acceptance: approval/resume wording and artifact-inspection boundaries are captured.

### Implementation for User Story 2

- [X] T050 [US2] Refine `.agents/skills/canon-approve/SKILL.md` to drive `canon approve --run ... --target ...`; acceptance: skill requires explicit approval intent and never invents approval results.
- [X] T051 [US2] Refine `.agents/skills/canon-resume/SKILL.md` to drive `canon resume --run <RUN_ID>`; acceptance: skill continues an existing Canon run and reports the resumed state from Canon output.
- [X] T052 [US2] Refine `.agents/skills/canon-inspect-artifacts/SKILL.md` to drive `canon inspect artifacts --run <RUN_ID>`; acceptance: skill reports artifact paths and related evidence pointers without fabricating runtime state.
- [X] T053 [US2] Update `.agents/skills/canon-shared/references/output-shapes.md`, `.agents/skills/canon-shared/scripts/render-next-steps.sh`, and `.agents/skills/canon-shared/scripts/render-next-steps.ps1`; acceptance: gated, approval, and resume guidance is deterministic across shells.
- [X] T054 [US2] Record unblock-flow validation evidence in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: evidence proves approval and resume skills stay subordinate to Canon runtime state.

**Checkpoint**: Approval, resume, and artifact-inspection flows are usable from Codex and remain Canon-backed.

---

## Phase 4: Deeper Delivered Modes (User Story 1 Continuation)

**Goal**: Extend the runnable Codex frontend to the two other delivered Canon workflows.

**Independent Test**: Invoke `$canon-brownfield` and `$canon-pr-review` and confirm they launch real Canon runs, surface real run ids, and point to the correct inspection and approval follow-ups.

### Validation for User Story 1 Continuation (MANDATORY)

- [X] T055 [P] [US1] Add walkthrough cases for `canon-brownfield` and `canon-pr-review` in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: mode-specific inputs, review disposition, and nearest follow-up skills are explicit.
- [X] T056 [US1] Record deeper runnable-mode decisions in `specs/003-codex-skills-frontend/decision-log.md`; acceptance: any brownfield/pr-review UX distinctions are durable before implementation closes.

### Implementation for User Story 1 Continuation

- [X] T057 [US1] Refine `.agents/skills/canon-brownfield/SKILL.md` to drive `canon run --mode brownfield-change`; acceptance: skill uses shared preflight, returns real run state, and points to evidence or approval when gated.
- [X] T058 [US1] Refine `.agents/skills/canon-pr-review/SKILL.md` to drive `canon run --mode pr-review`; acceptance: skill requires base/head inputs and returns real review run guidance without simulating a review packet.
- [X] T059 [US1] Update `.agents/skills/canon-shared/references/output-shapes.md` and `.agents/skills/canon-shared/references/skill-index.md`; acceptance: brownfield and pr-review examples reflect real Canon CLI expectations.
- [X] T060 [US1] Record deeper delivered-mode validation evidence in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: evidence proves the two additional mode skills still map to Canon CLI, not chat simulation.

**Checkpoint**: All currently delivered Canon workflows are runnable from Codex through named skills.

---

## Phase 5: Modeled-Only and Intentionally-Limited Support-State Wrappers (User Story 3 - P3)

**Goal**: Keep the full Canon taxonomy discoverable while making non-runnable and intentionally-limited workflows brutally honest.

**Independent Test**: Invoke modeled-only skills such as `$canon-architecture`, `$canon-review`, and `$canon-discovery`, plus `$canon-verification`, and confirm they never create fake runs, explain what Canon knows, explain what is missing, and point to the nearest runnable skills when useful.

### Validation for User Story 3 (MANDATORY)

- [X] T061 [P] [US3] Add modeled-only, intentionally-limited, and overlap validation cases to `specs/003-codex-skills-frontend/validation-report.md`; acceptance: the report explicitly covers fake-run prohibition, support-state labeling, and overlap checks for `review` vs `pr-review`, `brownfield` vs `refactor`, and `requirements` vs `discovery`.
- [X] T062 [US3] Record User Story 3 support-state decisions in `specs/003-codex-skills-frontend/decision-log.md`; acceptance: wording rules and nearest-runnable alternatives are durable before wrapper refinement lands.

### Implementation for User Story 3

- [X] T063 [US3] Refine `.agents/skills/canon-architecture/SKILL.md`; acceptance: skill is discoverable, labeled `modeled-only`, explains what Canon knows and what is missing, and never starts a run.
- [X] T064 [US3] Refine `.agents/skills/canon-review/SKILL.md`; acceptance: skill is discoverable, labeled `modeled-only`, distinguishes itself from `canon-pr-review`, and never starts a run.
- [X] T065 [US3] Refine `.agents/skills/canon-verification/SKILL.md`; acceptance: skill is discoverable, labeled `intentionally-limited`, explains the `verify` backlog, and never fabricates `canon verify` behavior.
- [X] T066 [US3] Refine `.agents/skills/canon-discovery/SKILL.md` and `.agents/skills/canon-system-shaping/SKILL.md`; acceptance: both skills stay discoverable, labeled `modeled-only`, and route honestly to nearest runnable workflows where appropriate.
- [X] T067 [US3] Refine `.agents/skills/canon-implementation/SKILL.md` and `.agents/skills/canon-refactor/SKILL.md`; acceptance: both skills explain why the workflow is not runnable and preserve boundaries between implementation and refactor versus delivered modes.
- [X] T068 [US3] Refine `.agents/skills/canon-incident/SKILL.md` and `.agents/skills/canon-migration/SKILL.md`; acceptance: both skills are discoverable, labeled `modeled-only`, and provide honest nearest alternatives when useful.
- [X] T069 [US3] Update `.agents/skills/canon-shared/references/support-states.md`, `.agents/skills/canon-shared/references/output-shapes.md`, and `.agents/skills/canon-shared/references/skill-index.md`; acceptance: every non-runnable skill has a canonical support-state response and nearest-runnable guidance.
- [X] T070 [US3] Run `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1` with overlap and fake-run cases enabled; acceptance: validators prove modeled-only and intentionally-limited skills are discoverable, clearly labeled, and unable to start fake runs.
- [X] T071 [US3] Record modeled-only and intentionally-limited validation evidence in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: evidence captures honesty, overlap, and nearest-runnable guidance across the full non-runnable set.

**Checkpoint**: The full Canon taxonomy is discoverable in Codex, and non-runnable skills preserve trust instead of pretending capability.

---

## Final Phase: Verification & Compliance

**Purpose**: Run cross-cutting validation, update operator docs, and close the increment cleanly.

- [X] T072 [P] Run structural validation with `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1`, then record results in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: tree shape, required metadata, discoverability, and support-state labels all pass.
- [X] T073 [P] Run deterministic failure-path checks for missing CLI, incompatible version, wrong repo, missing `.canon/`, and missing inputs using `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1`; acceptance: each failure produces actionable guidance and no fake Canon result.
- [X] T074 Run walkthrough validation for `canon-requirements`, `canon-brownfield`, `canon-pr-review`, `canon-inspect-evidence`, and `canon-approve` plus `canon-resume`, then record results in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: runnable skills all map cleanly to Canon CLI behavior.
- [X] T075 [P] Update `README.md` and `specs/003-codex-skills-frontend/quickstart.md`; acceptance: docs lead with runnable skills, explain the full discoverable taxonomy, and preserve brutal support-state honesty.
- [X] T076 [P] Update `AGENTS.md` and `.agents/skills/canon-shared/references/skill-index.md`; acceptance: repo guidance and shipped skill inventory agree on runnable versus non-runnable behavior.
- [X] T077 Perform independent review of skill discoverability, support-state honesty, overlap boundaries, and Canon CLI mapping in `specs/003-codex-skills-frontend/validation-report.md`; acceptance: separate review findings are recorded and any residual risks are named explicitly.
- [X] T078 Close `specs/003-codex-skills-frontend/validation-report.md` and `specs/003-codex-skills-frontend/decision-log.md`; acceptance: invariants are re-confirmed, residual backlog is listed, and the increment can be accepted without hidden assumptions.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Contracts**: no dependencies; MUST complete first.
- **Phase 1: Shared Frontend Spine and Taxonomy Presence**: depends on Phase 0; blocks all runnable-skill refinement.
- **Phase 2: First Runnable Workflow Set**: depends on Phase 1, especially T006-T019 and the initial skill files T020-T039.
- **Phase 3: Unblock and Continue**: depends on Phase 2 and MUST NOT begin until the Phase-2 `$` UX walkthrough is recorded in the validation report because approval/resume/artifact flows assume a proven runnable skill baseline.
- **Phase 4: Deeper Delivered Modes**: depends on Phase 2 and MUST NOT begin until the Phase-2 `$` UX walkthrough is recorded in the validation report; it then benefits from Phase 3 helper/output refinements.
- **Phase 5: Modeled-Only and Intentionally-Limited Support-State Wrappers**: depends on Phase 1 for taxonomy presence and helpers, but MUST NOT be treated as default next work; start it only after the Phase-2 MVP has been validated in real Codex usage and later slices are intentionally chosen.
- **Final Phase: Verification & Compliance**: depends on all targeted slices being complete.

### User Story Dependencies

- **User Story 1 (P1)**: starts after Phase 1 and expands across Phase 2 and Phase 4.
- **User Story 2 (P2)**: starts after Phase 2 establishes runnable run ids and evidence surfaces.
- **User Story 3 (P3)**: initial discoverability starts in Phase 1; support-state refinement and honesty validation land in Phase 5.

### Within Each Slice

- Contract and validation tasks happen before the affected skill behavior is refined.
- Shared helpers and references land before any skill depends on them.
- Executable wrapper skills must pass preflight before they may claim runnable behavior.
- Support-state wrappers must be labeled and fake-run-safe before they are considered done.
- Validation evidence must be recorded before a slice is declared complete.

### Parallel Opportunities

- Reference files T006-T010 can run in parallel.
- Helper scripts T011-T018 can run in parallel after the reference vocabulary is fixed.
- Initial skill-file creation T020-T038 can run in parallel once T010 exists and should be executed as a fixed checklist/template sweep, not as 19 bespoke design efforts.
- Runnable skill refinement within Phase 2 can parallelize after T040-T041.
- Support-state wrapper refinement within Phase 5 can parallelize after T061-T062.

---

## Parallel Example: User Story 1

```bash
# After shared references and preflight helpers land:
Task: "Refine .agents/skills/canon-status/SKILL.md"
Task: "Refine .agents/skills/canon-inspect-invocations/SKILL.md"
Task: "Refine .agents/skills/canon-inspect-evidence/SKILL.md"
```

## Parallel Example: User Story 2

```bash
# After unblock-flow validation cases are recorded:
Task: "Refine .agents/skills/canon-approve/SKILL.md"
Task: "Refine .agents/skills/canon-resume/SKILL.md"
Task: "Refine .agents/skills/canon-inspect-artifacts/SKILL.md"
```

## Parallel Example: User Story 3

```bash
# After support-state wording and overlap cases are fixed:
Task: "Refine .agents/skills/canon-architecture/SKILL.md"
Task: "Refine .agents/skills/canon-review/SKILL.md"
Task: "Refine .agents/skills/canon-discovery/SKILL.md and .agents/skills/canon-system-shaping/SKILL.md"
```

## Implementation Strategy

### MVP First

1. Complete Phase 0.
2. Complete Phase 1.
3. Complete Phase 2.
4. **STOP and VALIDATE**: confirm the full taxonomy exists, the shared helper layer works, `canon-init` is usable, the first runnable set works, and the non-runnable skills are already discoverable with honest initial support-state metadata.
5. Record a real Codex `$` walkthrough in `specs/003-codex-skills-frontend/validation-report.md` before authorizing any work from Phase 3, Phase 4, or Phase 5.

### Incremental Delivery

1. Ship taxonomy presence plus shared deterministic spine.
2. Ship the first runnable workflow set.
3. Add unblock-and-continue flows.
4. Add the deeper delivered mode skills.
5. Refine modeled-only and intentionally-limited support-state wrappers.
6. Finish with verification, docs, and independent review.

### Suggested MVP Scope

- Phase 0
- Phase 1
- Phase 2

This MVP is shippable because it already provides:

- full `.agents/skills` taxonomy presence
- shared deterministic helper layer
- `canon-init`
- `canon-requirements`
- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`
- discoverable, honest initial support-state wrappers for every non-runnable Canon mode

### Secondary Refinement After MVP

- Only start this section after the MVP walkthrough has been recorded and reviewed as a real Codex UX checkpoint.
- Phase 3 for unblock and continue flows
- Phase 4 for deeper delivered modes
- Phase 5 for stronger modeled-only and intentionally-limited support-state refinement
- Final Phase for full compliance closeout
