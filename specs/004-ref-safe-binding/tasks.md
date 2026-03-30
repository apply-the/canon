---
description: "Implementation tasks for Runnable Skill Interaction and Ref-Safe Input Binding"
---

# Tasks: Runnable Skill Interaction and Ref-Safe Input Binding

**Input**: Design documents from `specs/004-ref-safe-binding/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`, `validation-report.md`

**Validation**: Layered validation is mandatory. This patch must prove typed input preservation, deterministic retry rendering, ref-safe `canon-pr-review` binding, failure-type separation, and Bash/PowerShell parity without broadening Canon runtime scope.

**Organization**: Tasks are grouped by governance anchors, shared typed-input foundation, direct user stories, optional reuse, and final verification so each increment can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel after listed dependencies are complete
- **[Story]**: User story label for story-specific phases only (`[US1]`, `[US2]`, `[US3]`)
- Every task line includes exact repo-relative file paths

## Constitution Alignment

- Execution mode is `brownfield` and risk classification is `bounded-impact`
- Canon CLI remains the only execution engine and system of record
- Shared helpers stay bounded to `.agents/skills/canon-shared/`
- Preserved valid inputs stay intra-interaction only
- Validation ownership stays split between generation artifacts and independent evidence capture in `specs/004-ref-safe-binding/validation-report.md`

## Phase 0: Governance & Artifact Anchors

**Purpose**: Lock the implementation contract, failure taxonomy, and evidence expectations before touching helper or skill files.

- [X] T001 Align typed input slots, failure classes, and normalized output fields in `specs/004-ref-safe-binding/contracts/runnable-skill-input-contract.md`, `specs/004-ref-safe-binding/data-model.md`, and `specs/004-ref-safe-binding/validation-report.md`
Acceptance: the contract, data model, and validation plan name the same slot kinds, `STATUS` values, `FAILED_SLOT` / `FAILED_KIND` fields, and normalized output keys.

- [X] T002 Lock the local-only ref resolution order and canonical CLI forms in `specs/004-ref-safe-binding/contracts/pr-review-ref-binding-contract.md` and `specs/004-ref-safe-binding/decision-log.md`
Acceptance: the ref-binding contract and decision log both preserve `HEAD` / `refs/heads/*` acceptance, reject remote refs, and forbid automatic `main` / `master` substitution.

- [X] T003 Expand evidence checkpoints and independent review expectations in `specs/004-ref-safe-binding/validation-report.md`
Acceptance: the validation report explicitly lists missing-zone, single-field retry, `master` / `HEAD`, `main` / `HEAD`, missing-file vs invalid-ref, and preflight-vs-execution proof points.

---

## Phase 1: Setup (Validation Baseline)

**Purpose**: Prepare the validator and walkthrough scaffolding that will guard the narrow patch.

- [X] T004 Update the Bash skill validator baseline in `scripts/validate-canon-skills.sh`
Acceptance: the validator can fail on stale direct-scope preflight contracts, missing incremental retry guidance, and missing overlap-boundary protections for runnable skills.

- [X] T005 [P] Update the PowerShell skill validator baseline in `scripts/validate-canon-skills.ps1`
Acceptance: the PowerShell validator checks the same baseline structure and overlap protections as the Bash validator.

- [X] T006 [P] Refresh the manual probe and walkthrough commands in `specs/004-ref-safe-binding/quickstart.md`
Acceptance: the quickstart covers the exact Bash and PowerShell probes needed later for missing `zone`, missing path, `master` / `HEAD`, `main` / `HEAD`, and run-id-only correction scenarios.

---

## Phase 2: Foundational Shared Typed-Input Support

**Purpose**: Implement the shared helper behavior that blocks or enables every direct-scope story.

**Checkpoint**: No user story work starts until the shared helper emits deterministic typed outcomes in both shells.

**Enforcement Rule**: `T007` through `T012` are the behavioral source of truth for typed preflight, normalization, failure classification, and ref-safe binding. Later `SKILL.md` tasks must only mirror helper-enforced behavior and must not introduce prose-only promises.

- [X] T007 Implement the typed preflight output contract in `.agents/skills/canon-shared/scripts/check-runtime.sh`
Acceptance: the Bash helper emits `STATUS`, `CODE`, `PHASE`, `COMMAND`, `REPO_ROOT`, `MESSAGE`, and `ACTION`, plus typed failure metadata and normalized values where applicable.

- [X] T008 [P] Implement the typed preflight output contract in `.agents/skills/canon-shared/scripts/check-runtime.ps1`
Acceptance: the PowerShell helper emits the same keys, status names, and failure metadata as the Bash helper.

- [X] T009 Normalize risk / zone values and classify missing vs invalid run-start inputs in `.agents/skills/canon-shared/scripts/check-runtime.sh` (depends on T007)
Acceptance: Bash preflight distinguishes missing `risk` / `zone` from invalid tokens, normalizes runtime-recognized aliases to Canon tokens, and does not start Canon on malformed run-start inputs.

- [X] T010 [P] Normalize risk / zone values and classify missing vs invalid run-start inputs in `.agents/skills/canon-shared/scripts/check-runtime.ps1` (depends on T008)
Acceptance: PowerShell preflight matches the Bash normalization rules and missing-vs-invalid classification for run-start inputs.

- [X] T011 Separate file-path validation, run-id validation, and local-only ref resolution in `.agents/skills/canon-shared/scripts/check-runtime.sh` (depends on T009)
Acceptance: Bash preflight returns `missing-file`, `invalid-input`, `invalid-ref`, or `malformed-ref-pair` deterministically, uses `--ref` for ref slots, and never treats semantically valid refs as filesystem paths.

- [X] T012 [P] Separate file-path validation, run-id validation, and local-only ref resolution in `.agents/skills/canon-shared/scripts/check-runtime.ps1` (depends on T010)
Acceptance: PowerShell preflight mirrors the Bash resolution order, rejection of remote refs, pair validation, and typed failure classes.

---

## Phase 3: User Story 1 - Complete a Runnable Skill Without Re-entering Everything (Priority: P1)

**Goal**: Let direct-scope runnable skills preserve valid inputs and ask only for the unresolved slot instead of resetting the whole request.

**Independent Test**: Invoke a runnable skill with valid `owner` and `risk` but missing `zone`, then correct one field and confirm the skill preserves the valid inputs and only asks for the missing or invalid slot before Canon starts.

### Validation for User Story 1

- [X] T013 [P] [US1] Add Bash validation cases for preserved valid fields and single-slot retry guidance in `scripts/validate-canon-skills.sh` (depends on T004, T009, T011)
Acceptance: the Bash validator fails if the affected runnable skills still require full re-entry after a single missing `zone` or path correction.

- [X] T014 [P] [US1] Add PowerShell validation cases for preserved valid fields and single-slot retry guidance in `scripts/validate-canon-skills.ps1` (depends on T005, T010, T012)
Acceptance: the PowerShell validator fails on the same single-slot retry regressions as the Bash validator.

### Implementation for User Story 1

- [X] T015 [US1] Update typed owner / risk / zone / input-path guidance in `.agents/skills/canon-requirements/SKILL.md` (depends on T009, T011)
Acceptance: `canon-requirements` documents the helper-enforced flow from `.agents/skills/canon-shared/scripts/check-runtime.*`, asks only for the missing slot, preserves valid ownership fields, distinguishes missing path from missing metadata, and shows the exact Canon CLI retry form.

- [X] T016 [US1] Update typed owner / risk / zone / brief-path guidance in `.agents/skills/canon-brownfield/SKILL.md` (depends on T009, T011)
Acceptance: `canon-brownfield` documents the helper-enforced flow from `.agents/skills/canon-shared/scripts/check-runtime.*`, preserves valid ownership fields, asks only for the missing brief path or missing ownership slot, and keeps Canon authority for actual run start.

- [X] T017 [US1] Record missing-`zone`, missing-path, and one-field-correction walkthrough evidence in `specs/004-ref-safe-binding/validation-report.md` (depends on T015, T016)
Acceptance: the validation report shows that valid fields survive retry and that Canon starts only after the corrected typed preflight succeeds.

**Checkpoint**: User Story 1 is independently testable for `canon-requirements` and `canon-brownfield` without any full-form retry reset.

---

## Phase 4: User Story 2 - Start `canon-pr-review` With Semantically Valid Refs (Priority: P1)

**Goal**: Make `canon-pr-review` treat base/head inputs as refs first, canonicalize them deterministically, and stop confusing refs with paths.

**Independent Test**: Invoke `canon-pr-review` with `base master, head HEAD` and with `base main, head HEAD` in a repo whose usable local branch is `master`, then confirm deterministic ref handling, canonical retry rendering, and no file-path wording.

### Validation for User Story 2

- [X] T018 [P] [US2] Add Bash validation cases for `--ref` preflight usage and canonical ref retry rendering in `scripts/validate-canon-skills.sh` (depends on T004, T011)
Acceptance: the Bash validator fails if `canon-pr-review` still documents `--input` preflight for refs or omits canonical `refs/heads/*` retry guidance.

- [X] T019 [P] [US2] Add PowerShell validation cases for `--ref` preflight usage and canonical ref retry rendering in `scripts/validate-canon-skills.ps1` (depends on T005, T012)
Acceptance: the PowerShell validator enforces the same `--ref` and canonical retry rules as the Bash validator.

### Implementation for User Story 2

- [X] T020 [US2] Update base-ref / head-ref collection, `--ref` preflight binding, and local-only retry guidance in `.agents/skills/canon-pr-review/SKILL.md` (depends on T011, T012)
Acceptance: `canon-pr-review` documents the helper-enforced flow from `.agents/skills/canon-shared/scripts/check-runtime.*`, preserves the valid side of the pair, uses `--ref` in preflight, accepts only local ref forms plus `HEAD`, rejects remote refs explicitly, and renders the exact Canon CLI form passed through `--input`.

- [X] T021 [US2] Record `master` / `HEAD`, `main` / `HEAD`, unsupported remote ref, and malformed-pair evidence in `specs/004-ref-safe-binding/validation-report.md` (depends on T020)
Acceptance: the validation report names the expected `STATUS`, `FAILED_SLOT`, `FAILED_KIND`, and canonical retry output for each PR review ref case.

**Checkpoint**: User Story 2 is independently testable and proves that semantically valid refs are no longer classified as paths.

---

## Phase 5: User Story 3 - Trust Failure Messaging Across Runnable Skills (Priority: P2)

**Goal**: Separate preflight failures from Canon-execution failures and make retry guidance deterministic by input kind across the direct-scope skills.

**Independent Test**: Trigger missing input, invalid ref, missing file, wrong repo context, repo-not-initialized, and Canon-execution failures, then confirm each message states the failure class, the interaction boundary, and the exact retry form accepted by the current binding.

### Validation for User Story 3

- [X] T022 [P] [US3] Add Bash validation cases for missing-file vs invalid-ref wording and preflight-vs-execution boundaries in `scripts/validate-canon-skills.sh` (depends on T004, T011, T015, T016, T020)
Acceptance: the Bash validator fails if affected skills blur file-path and ref guidance or fail to distinguish preflight failures from Canon-execution failures.

- [X] T023 [P] [US3] Add PowerShell validation cases for missing-file vs invalid-ref wording and preflight-vs-execution boundaries in `scripts/validate-canon-skills.ps1` (depends on T005, T012, T015, T016, T020)
Acceptance: the PowerShell validator enforces the same failure-class and phase-boundary rules as the Bash validator.

### Implementation for User Story 3

- [X] T024 [US3] Update failure-class and retry-boundary guidance in `.agents/skills/canon-requirements/SKILL.md` (depends on T015)
Acceptance: `canon-requirements` names missing input, invalid input, missing file, and Canon-execution outcomes without implying that Canon already started when preflight blocked the request, and does not promise any behavior not enforced by `.agents/skills/canon-shared/scripts/check-runtime.*`.

- [X] T025 [US3] Update failure-class and retry-boundary guidance in `.agents/skills/canon-brownfield/SKILL.md` (depends on T016)
Acceptance: `canon-brownfield` distinguishes missing brief paths from missing ownership metadata, reports Canon-execution failures separately from preflight failures, and does not promise any behavior not enforced by `.agents/skills/canon-shared/scripts/check-runtime.*`.

- [X] T026 [US3] Update failure-class, preserved-slot, and exact retry-form guidance in `.agents/skills/canon-pr-review/SKILL.md` (depends on T020)
Acceptance: `canon-pr-review` separates invalid ref, missing ref, malformed pair, and Canon-execution outcomes while keeping retry guidance aligned to the actual Canon CLI contract and does not promise any behavior not enforced by `.agents/skills/canon-shared/scripts/check-runtime.*`.

- [X] T027 [US3] Record missing-file vs missing-ref and preflight-vs-execution evidence in `specs/004-ref-safe-binding/validation-report.md` (depends on T024, T025, T026)
Acceptance: the validation report proves that direct-scope skills surface the correct failure type and boundary for each direct-scope scenario.

**Checkpoint**: User Story 3 closes the trust gap by making failure messages deterministic and boundary-aware across the direct patch scope.

---

## Phase 6: Opportunistic Reuse (Optional Refinement)

**Purpose**: Reuse the stabilized run-id / required-input guidance where it improves safety without broadening the patch into new architecture.

- [X] T028 Update run-id-only correction guidance in `.agents/skills/canon-status/SKILL.md` (depends on T009, T011)
Acceptance: `canon-status` asks only for the missing `RUN_ID` and shows the exact retry form without inventing other state.

- [X] T029 [P] Update run-id-only correction guidance in `.agents/skills/canon-inspect-invocations/SKILL.md` (depends on T009, T011)
Acceptance: `canon-inspect-invocations` keeps the retry surface limited to `RUN_ID` and aligns wording to the shared typed-input contract.

- [X] T030 [P] Update run-id-only correction guidance in `.agents/skills/canon-inspect-evidence/SKILL.md` (depends on T009, T011)
Acceptance: `canon-inspect-evidence` requests only the missing `RUN_ID` and does not broaden into generic conversational recovery.

- [X] T031 [P] Update run-id-only correction guidance in `.agents/skills/canon-inspect-artifacts/SKILL.md` (depends on T009, T011)
Acceptance: `canon-inspect-artifacts` uses the same run-id-only retry pattern and exact CLI form as the other inspect skills.

- [X] T032 Update run-id correction and phase-boundary guidance in `.agents/skills/canon-resume/SKILL.md` (depends on T009, T011)
Acceptance: `canon-resume` distinguishes missing `RUN_ID` from post-resume Canon state and keeps retry guidance scoped to the missing identifier.

- [X] T033 Update run-id correction guidance while keeping `TARGET`, `BY`, `DECISION`, and `RATIONALE` skill-local in `.agents/skills/canon-approve/SKILL.md` (depends on T009, T011)
Acceptance: `canon-approve` reuses shared run-id handling without introducing a new shared taxonomy for approval metadata.

- [X] T034 Record optional reuse evidence and any deliberate deferrals in `specs/004-ref-safe-binding/validation-report.md` (depends on T028, T029, T030, T031, T032, T033)
Acceptance: the validation report distinguishes shipped reuse from intentionally deferred approval-field typing or broader interaction changes.

---

## Final Phase: Verification & Compliance

**Purpose**: Prove the patch is correct, bounded, and ready to close without product expansion.

**MVP Verification Rule**: `T035` through `T042` validate the shippable direct-scope patch from `T001` through `T027` without requiring Phase 6. If optional reuse ships, validate it separately with `T043` through `T045`.

- [X] T035 [P] Run `/bin/bash scripts/validate-canon-skills.sh` for the MVP direct-scope patch and record structural results in `specs/004-ref-safe-binding/validation-report.md` (depends on T013, T015, T016, T018, T020, T022, T024, T025, T026)
Acceptance: the validation report captures a passing Bash structural validation run for the direct-scope MVP without requiring optional reuse tasks.

- [X] T036 [P] Run `pwsh -File scripts/validate-canon-skills.ps1` for the MVP direct-scope patch and record structural results in `specs/004-ref-safe-binding/validation-report.md` (depends on T014, T015, T016, T019, T020, T023, T024, T025, T026)
Acceptance: the validation report captures a passing PowerShell structural validation run for the direct-scope MVP without requiring optional reuse tasks.

- [X] T037 [P] Run targeted Bash preflight probes against `.agents/skills/canon-shared/scripts/check-runtime.sh` and record outputs in `specs/004-ref-safe-binding/validation-report.md` (depends on T011)
Acceptance: Bash evidence includes missing `zone`, invalid risk, missing path, `master` / `HEAD`, `main` / `HEAD`, unsupported remote ref, malformed ref pair, and run-id classification outputs.

- [X] T038 [P] Run targeted PowerShell preflight probes against `.agents/skills/canon-shared/scripts/check-runtime.ps1` and record outputs in `specs/004-ref-safe-binding/validation-report.md` (depends on T012)
Acceptance: PowerShell evidence covers the same probe matrix and produces materially identical status names, normalized values, and failure classes.

- [X] T039 Execute the runnable walkthroughs in `specs/004-ref-safe-binding/quickstart.md` and record outcomes in `specs/004-ref-safe-binding/validation-report.md` (depends on T017, T021, T027)
Acceptance: the walkthrough record proves preserved valid fields, single-field repair, ref-safe PR review binding, and Canon start only after typed preflight returns ready.

- [X] T040 Perform independent contract review against `canon run`, `canon status`, `canon inspect`, `canon approve`, and `canon resume` bindings in `specs/004-ref-safe-binding/validation-report.md` (depends on T035, T036, T037, T038, T039)
Acceptance: independent review confirms retry guidance matches the real Canon CLI contract and no direct-scope skill simulates Canon runtime logic for the chosen release scope.

- [X] T041 Update contract wording follow-through in `specs/004-ref-safe-binding/quickstart.md` and `specs/004-ref-safe-binding/decision-log.md` if validation finds wording drift (depends on T040)
Acceptance: any wording corrections remain within the approved patch direction and are traceable in the feature artifacts.

- [X] T042 Confirm bounded-impact invariants, repo-relative artifact paths, and `git diff --check` closeout in `specs/004-ref-safe-binding/validation-report.md` (depends on T041)
Acceptance: the closeout record confirms the patch stayed narrow, kept Canon CLI authoritative, preserved intra-interaction-only memory, and finished with clean diff hygiene.

- [X] T043 [P] Run `/bin/bash scripts/validate-canon-skills.sh` after optional reuse tasks `T028` through `T033` and record the addendum in `specs/004-ref-safe-binding/validation-report.md` (depends on T028, T029, T030, T031, T032, T033, T034)
Acceptance: the validation report captures a passing Bash addendum for the optional reuse tranche without changing MVP readiness.

- [X] T044 [P] Run `pwsh -File scripts/validate-canon-skills.ps1` after optional reuse tasks `T028` through `T033` and record the addendum in `specs/004-ref-safe-binding/validation-report.md` (depends on T028, T029, T030, T031, T032, T033, T034)
Acceptance: the validation report captures a passing PowerShell addendum for the optional reuse tranche without changing MVP readiness.

- [X] T045 Perform optional reuse review addendum for `canon-status`, `canon-inspect-*`, `canon-resume`, and `canon-approve` in `specs/004-ref-safe-binding/validation-report.md` (depends on T043, T044)
Acceptance: the addendum confirms Phase 6 stayed within shared run-id reuse and did not broaden into new approval-field taxonomy or generic interaction behavior.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifact Anchors**: no dependencies; must complete first.
- **Phase 1: Setup (Validation Baseline)**: depends on Phase 0.
- **Phase 2: Foundational Shared Typed-Input Support**: depends on Phase 1 and blocks all user-story work.
- **Phase 3: User Story 1**: depends on Phase 2.
- **Phase 4: User Story 2**: depends on Phase 2 and can run in parallel with User Story 1 after the shared helper foundation is stable.
- **Phase 5: User Story 3**: depends on User Story 1 and User Story 2 because it consolidates final failure semantics across the direct-scope skills.
- **Phase 6: Opportunistic Reuse**: optional; depends on Phase 2 and can ship after MVP closeout as a separate addendum.
- **Final Phase: Verification & Compliance**: MVP tasks `T035` through `T042` depend on Phases 0 through 5 only; optional addendum tasks `T043` through `T045` depend on Phase 6 if that tranche ships.

### User Story Dependencies

- **User Story 1 (US1)**: independent after the shared typed-input foundation; targets `canon-requirements` and `canon-brownfield`.
- **User Story 2 (US2)**: independent after the shared typed-input foundation; targets `canon-pr-review` as the proving case.
- **User Story 3 (US3)**: depends on US1 and US2 because it reconciles final failure wording and phase-boundary messaging across the patched direct-scope skills.

### Within Each User Story

- Validation tasks come before the skill-document updates they guard.
- Shared helper behavior lands before any direct-scope skill references it.
- Evidence capture must complete before the story is considered done.

### Parallel Opportunities

- `T005` and `T006` can run in parallel after `T004`.
- `T008`, `T010`, and `T012` can run in parallel with their Bash counterparts once the matching dependency is satisfied.
- US1 and US2 can be implemented in parallel after Phase 2.
- Optional reuse tasks `T029`, `T030`, and `T031` can run in parallel once the run-id foundation is stable.
- Final structural and probe tasks `T035` through `T038` can run in parallel after implementation is complete.
- Optional reuse verification tasks `T043` and `T044` can run in parallel after `T028` through `T034` if Phase 6 ships.

---

## Parallel Example: User Story 1

```bash
# Launch the story-specific validator work in parallel:
Task: "Add Bash validation cases for preserved valid fields and single-slot retry guidance in scripts/validate-canon-skills.sh"
Task: "Add PowerShell validation cases for preserved valid fields and single-slot retry guidance in scripts/validate-canon-skills.ps1"

# After the shared helper foundation is stable, patch the direct-scope skill docs in parallel:
Task: "Update typed owner / risk / zone / input-path guidance in .agents/skills/canon-requirements/SKILL.md"
Task: "Update typed owner / risk / zone / brief-path guidance in .agents/skills/canon-brownfield/SKILL.md"
```

## Parallel Example: User Story 2

```bash
# Launch the story-specific validator work in parallel:
Task: "Add Bash validation cases for --ref preflight usage and canonical ref retry rendering in scripts/validate-canon-skills.sh"
Task: "Add PowerShell validation cases for --ref preflight usage and canonical ref retry rendering in scripts/validate-canon-skills.ps1"
```

## Parallel Example: Optional Reuse

```bash
# Once the direct-scope stories are stable, apply the run-id-only reuse tasks in parallel:
Task: "Update run-id-only correction guidance in .agents/skills/canon-inspect-invocations/SKILL.md"
Task: "Update run-id-only correction guidance in .agents/skills/canon-inspect-evidence/SKILL.md"
Task: "Update run-id-only correction guidance in .agents/skills/canon-inspect-artifacts/SKILL.md"
```

---

## Implementation Strategy

### MVP First

The first shippable milestone for this patch is **Phases 0 through 5**, not the optional reuse phase. That scope gives:

1. shared typed-input handling for the affected runnable skills
2. incremental missing-input repair without re-entering valid fields
3. deterministic failure messages by input kind
4. ref-safe `canon-pr-review` binding that no longer confuses refs with paths
5. retry guidance that matches the real Canon CLI contract

Suggested MVP stop point:

1. Complete `T001` through `T027`
2. Run `T035` through `T040`
3. Close with `T041` and `T042`

### Incremental Delivery

1. Finish Phases 0 through 2 to stabilize the shared helper contract.
2. Deliver User Story 1 and validate it independently.
3. Deliver User Story 2 and validate it independently.
4. Deliver User Story 3 and validate the direct-scope trust repair end to end.
5. Close MVP with `T035` through `T042`.
6. Decide whether Phase 6 opportunistic reuse is worth shipping as a follow-on addendum.
7. If Phase 6 ships, finish with `T043` through `T045`.

### Parallel Team Strategy

1. One contributor owns the Bash helper / validator path (`T004`, `T007`, `T009`, `T011`, `T013`, `T018`, `T022`, `T035`, `T037`).
2. One contributor mirrors PowerShell parity (`T005`, `T008`, `T010`, `T012`, `T014`, `T019`, `T023`, `T036`, `T038`).
3. One contributor patches the direct-scope skill docs and evidence artifacts (`T015` through `T027`).
4. Optional reuse can be split across inspect/status/resume/approve tasks after the MVP scope is stable.