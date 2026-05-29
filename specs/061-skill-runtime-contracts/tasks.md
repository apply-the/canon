# Tasks: Canon Skill Runtime Contracts

**Input**: Design documents from `specs/061-skill-runtime-contracts/`
**Prerequisites**: plan.md (required), spec.md (required for user stories),
research.md, data-model.md, contracts/

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
- Bounded-impact work requires peer review; no separate approval gate task.

## Path Conventions

- **Scripts**: `.agents/skills/canon-shared/scripts/`
- **Skills**: `.agents/skills/canon-{implementation,change,publish}/SKILL.md`
- **Hooks config**: `.canon/hooks.toml`
- **Fixtures**: `tests/fixtures/preflight/`, `tests/fixtures/hooks/`
- **Docs**: `docs/guides/skill-runtime-contracts.md`
- **Specs**: `specs/061-skill-runtime-contracts/`

---

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start

- [x] T001 Bump version in `Cargo.toml` workspace metadata and update `CHANGELOG.md` with `061-skill-runtime-contracts` entry header
- [x] T002 Verify execution mode, risk, scope, invariants already recorded in `specs/061-skill-runtime-contracts/spec.md` and `specs/061-skill-runtime-contracts/plan.md`
- [x] T003 Verify decision log exists at `specs/061-skill-runtime-contracts/decision-log.md` with design-stage decisions DL-001 through DL-006
- [x] T004 Verify validation report scaffold exists at `specs/061-skill-runtime-contracts/validation-report.md` with 3-layer structure

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create shared fixtures and test infrastructure before implementation

- [x] T005 [P] Create test fixture `tests/fixtures/preflight/full-environment.json` with all fields populated per `contracts/preflight-json-schema.json`
- [x] T006 [P] Create test fixture `tests/fixtures/preflight/missing-canon.json` with `canon.available: false` and section-level error
- [x] T007 [P] Create test fixture `tests/fixtures/preflight/missing-input.json` with `input.resolved_path: null`
- [x] T008 [P] Create test fixture `tests/fixtures/preflight/partial-failure.json` with mixed section errors
- [x] T009 [P] Create test fixture `tests/fixtures/preflight/ambiguous-input.json` with `input.ambiguous: true`
- [x] T010 [P] Create test fixture `tests/fixtures/hooks/valid-hooks.toml` with before_run and after_publish hooks
- [x] T011 [P] Create test fixture `tests/fixtures/hooks/malformed-hooks.toml` with invalid TOML syntax
- [x] T012 [P] Create test fixture `tests/fixtures/hooks/trusted-untrusted-mix.toml` with mixed trusted/untrusted hooks and mode_filter

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared script utilities that all user stories depend on

- [x] T013 Extract reusable utility functions (`trim()`, `is_placeholder()`, `json_escape()`) into `.agents/skills/canon-shared/scripts/preflight-utils.sh`
- [x] T014 Create corresponding PowerShell utility module `.agents/skills/canon-shared/scripts/preflight-utils.ps1`
- [x] T014a [P] Mirror `preflight-utils.sh` and `preflight-utils.ps1` to `defaults/embedded-skills/canon-shared/scripts/`

---

## Phase 3: User Story 1 — Preflight JSON (P1)

**Goal**: `canon-preflight.sh` outputs structured JSON with all environment
state needed for preflight.

**Independent Test**: Run `canon-preflight.sh --mode implementation` in a repo
with `.canon/` initialized and a non-empty `canon-input/implementation.md`.
Verify JSON output contains all expected fields with correct values.

- [x] T015 [US1] Create `canon-preflight.sh` at `.agents/skills/canon-shared/scripts/canon-preflight.sh` with `--mode` argument parsing, sourcing `preflight-utils.sh`
- [x] T016 [US1] Implement `canon` section: detect binary, extract version, check `.canon/` directory
- [x] T017 [US1] Implement `workspace` section: repo root path, git branch, git user.email
- [x] T018 [US1] Implement `input` section: file/folder detection with file-first resolution and ambiguity flag per C-003
- [x] T019 [US1] Implement `runs` section: count active runs and pending approvals from `.canon/runs/`
- [x] T020 [US1] Implement JSON assembly with `schema_version`, `timestamp`, `mode` envelope and partial-failure error fields per C-002
- [x] T021 [US1] Create `canon-preflight.ps1` at `.agents/skills/canon-shared/scripts/canon-preflight.ps1` with identical JSON output shape using `ConvertTo-Json`
- [x] T022 [US1] Mirror to `defaults/embedded-skills/canon-shared/scripts/canon-preflight.sh` and `defaults/embedded-skills/canon-shared/scripts/canon-preflight.ps1`
- [x] T023 [US1] Validate: run `canon-preflight.sh --mode implementation` and pipe through `jq --exit-status '.'`; confirm schema matches `contracts/preflight-json-schema.json`
- [x] T024 [US1] Validate: run script without `canon` on PATH; confirm `canon.available: false` and partial JSON envelope present
- [x] T025 [US1] Validate: run with both `canon-input/implementation.md` and `canon-input/implementation/` existing; confirm `input.ambiguous: true`
- [x] T026 [US1] Validate: time script execution; confirm < 2 seconds (SC-002)
- [x] T027 [US1] Compare bash and PowerShell outputs for identical JSON shape; validate both against `contracts/preflight-json-schema.json` field-by-field

---

## Phase 4: User Story 2 — Declarative Preflight Contract (P2)

**Goal**: SKILL.md frontmatter `preflight:` block is parsed by AI to surface
only unmet requirements.

**Independent Test**: Add a `preflight:` block to `canon-implementation`
SKILL.md. Invoke the skill in a repo missing `.canon/`. Verify the AI surfaces
exactly the missing requirement.

- [x] T028 [US2] Add `preflight:` YAML block to `.agents/skills/canon-implementation/SKILL.md` frontmatter with `requires_canon: true`, `requires_initialized: true`, `canonical_input: implementation`, `system_context: existing`, `risk_required: true`, `zone_required: true`, `owner_optional: true`
- [x] T029 [P] [US2] Add `preflight:` YAML block to `.agents/skills/canon-change/SKILL.md` frontmatter with matching contract fields
- [x] T030 [P] [US2] Add `preflight:` YAML block to `.agents/skills/canon-publish/SKILL.md` frontmatter with `requires_canon: true`, `requires_initialized: true`, no canonical_input requirement
- [x] T031 [US2] Mark existing prose preflight instructions in all 3 skills with deprecation comment per C-004: `<!-- DEPRECATED: preflight behavior is governed by the 'preflight:' block. Do not use this prose as an execution contract. -->`
- [x] T032 [US2] Mirror updated SKILL.md files to `defaults/embedded-skills/canon-implementation/SKILL.md`, `defaults/embedded-skills/canon-change/SKILL.md`, `defaults/embedded-skills/canon-publish/SKILL.md`
- [x] T033 [US2] Validate: run `scripts/validate-canon-skills.sh` and confirm all 3 migrated skills pass (SC-003)
- [x] T034 [US2] Validate: confirm non-migrated skills are unchanged and still pass validator

---

## Phase 5: User Story 3 — Lifecycle Hooks (P3)

**Goal**: `.canon/hooks.toml` lifecycle hooks are detected, explained, and
proposed at lifecycle points with full trace recording.

**Independent Test**: Create `.canon/hooks.toml` with an `after_publish` hook.
Run `canon-publish` skill flow. Verify the skill surfaces the hook as a
proposal with command, description, and required/optional status. Verify that
if executed, the trace is recorded.

- [x] T035 [US3] Document hooks.toml schema in skill shared reference at `.agents/skills/canon-shared/references/hooks-schema.md` linking to `contracts/hooks-toml-schema.md`
- [x] T036 [US3] Add hook detection logic to `canon-publish` SKILL.md: parse `.canon/hooks.toml`, filter by event (`after_publish`) and mode, emit proposal block per `contracts/hooks-toml-schema.md` confirmation rules
- [x] T037 [US3] Add hook detection logic to `canon-implementation` SKILL.md: detect `before_run` hooks, emit proposal with trusted/untrusted differentiation per C-005
- [x] T038 [US3] Add hook detection logic to `canon-change` SKILL.md: detect `before_run` hooks matching mode filter
- [x] T039 [US3] Add hook trace recording pattern to all 3 skills: append `## Hook Traces` section to `ai-provenance.md` per data-model.md Hook Trace entity
- [x] T040 [US3] Mirror updated SKILL.md files to `defaults/embedded-skills/`
- [x] T041 [US3] Validate: create temporary `.canon/hooks.toml` with `after_publish` hook; confirm skill emits proposal block with command, description, required/optional, trusted status
- [x] T042 [US3] Validate: test with malformed `.canon/hooks.toml`; confirm skill skips hook detection silently without blocking
- [x] T043 [US3] Validate: test with hook having `mode_filter` not matching current mode; confirm hook not proposed
- [x] T044 [US3] Validate: confirm hook detection adds < 200ms to skill startup (SC-005)

---

## Phase 6: Documentation & Changelog

**Purpose**: Update user-facing documentation and changelog

- [x] T045 [P] Create user guide at `docs/guides/skill-runtime-contracts.md` covering preflight JSON usage, YAML contract authoring, and hooks.toml setup per `quickstart.md` content
- [x] T046 [P] Update `CHANGELOG.md` with full feature description under the version bumped in T001
- [x] T047 Update `docs/guides/modes.md` implementation section to reference the new preflight contract and hook detection behavior

---

## Phase 7: Verification & Compliance

**Purpose**: Final validation pass ensuring all invariants hold and coverage target met

- [x] T048 Run `scripts/validate-canon-skills.sh` and confirm zero failures across all skills (not just migrated ones)
- [x] T049 Run `cargo fmt --check` to confirm no formatting regressions
- [x] T050 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fix any issues
- [x] T051 Run `cargo test` and fix any test failures
- [x] T052 Measure coverage with `cargo llvm-cov` and confirm 95% line coverage on modified/new files; add tests if below threshold
- [x] T053 Run `scripts/validate-canon-skills.ps1` (if PowerShell available) for cross-platform parity check
- [x] T053a Validate mirror parity: diff `.agents/skills/` against `defaults/embedded-skills/` for all modified scripts and SKILL.md files; confirm zero divergence
- [x] T054 Capture validation evidence: record pass/fail logs in `specs/061-skill-runtime-contracts/validation-report.md`

---

## Dependencies

```text
Phase 0 (T001-T004) ──► Phase 1 (T005-T012)
                    ──► Phase 2 (T013-T014)

Phase 2 (T013-T014) ──► Phase 3/US1 (T015-T027)

Phase 3/US1 (T015-T027) ──► Phase 4/US2 (T028-T034)
                              (US2 needs working preflight script)

Phase 3/US1 (T015-T027) ──► Phase 5/US3 (T035-T044)
                              (US3 is independent of US2)

Phase 4 + Phase 5 ──► Phase 6 (T045-T047)
Phase 6 ──► Phase 7 (T048-T054)
```

## Parallel Execution Opportunities

**Within Phase 1**: All fixture tasks (T005-T012) are fully parallel.

**Within Phase 3/US1**: T016-T019 (section implementations) are parallel after
T015 creates the script skeleton.

**Within Phase 4/US2**: T028-T030 (YAML additions to different skills) are
parallel.

**Between Phase 4 and Phase 5**: US2 and US3 can proceed in parallel after US1
is complete (both depend on the preflight script but not on each other).

**Within Phase 6**: T045-T046 are parallel (different files).

## Implementation Strategy

**MVP (User Story 1 only)**: Phases 0-3 deliver a working `canon-preflight.sh`
that replaces the key=value pattern with structured JSON. This is
independently testable and immediately valuable without YAML contracts or hooks.

**Incremental delivery**:
1. Phase 0-3: Preflight JSON (highest value, lowest risk)
2. Phase 4: Declarative YAML (builds on Phase 3, reduces skill prose)
3. Phase 5: Lifecycle hooks (independent of Phase 4, enables repo customization)
4. Phase 6-7: Documentation, compliance, and coverage closure
