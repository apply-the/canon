# Tasks: Run Identity, Display Id, and Authored-Input Refactor

**Feature**: 009-run-id-display  
**Branch**: `009-run-id-display`  
**Inputs**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md),
[data-model.md](data-model.md), [contracts/run-identity-contract.md](contracts/run-identity-contract.md),
[quickstart.md](quickstart.md), [decision-log.md](decision-log.md),
[validation-report.md](validation-report.md)

**Execution mode**: `change`. **Risk**: `bounded-impact`. **Validation
ownership**: implementer generates code; `cargo nextest`, contract tests,
skill validators, and one independent human reviewer validate.

---

## Phase 0: Governance & Artifacts

- [ ] T001 Confirm governance posture in [plan.md](plan.md): mode=`change`, risk=`bounded-impact`, scope-in/out, invariants, validation ownership, approval gate are all current; record any drift in [decision-log.md](decision-log.md).
- [ ] T002 Verify [decision-log.md](decision-log.md) contains D-001..D-008 and reflects this feature's open questions; add new entries inline as decisions arise during implementation.
- [ ] T003 Verify [validation-report.md](validation-report.md) lists every required check, owner, and evidence path before implementation begins; update column "Evidence path" as each artifact lands.
- [ ] T004 Cross-link this feature into the Canon decision trail by adding a reference to `specs/009-run-id-display/` from the change run's `.canon/decisions/` record at run start (recorded by the implementer when the run is created).

## Phase 1: Setup

- [ ] T005 Confirm `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo nextest run` are green on `main` before branch work; capture output in `target/tmp/009-baseline.log`.
- [ ] T006 Create empty fixture directory `tests/fixtures/legacy_uuid_run/` (will be populated by T013).
- [ ] T007 [P] Add `tests/contract/run_identity_contract.rs` test file (initially `#[ignore]`-only stubs for the eight contracts in [contracts/run-identity-contract.md](contracts/run-identity-contract.md)) so contract test names appear in `cargo nextest --list` from day one.
- [ ] T008 [P] Add `tests/integration/run_lookup.rs` and `tests/integration/inputs_snapshot_immutability.rs` skeleton files with `#[ignore]` placeholder tests for the matrix in [validation-report.md](validation-report.md).

## Phase 2: Foundational (blocking prerequisites)

These land before any user-story work because all stories depend on the
identity types, slug pipeline, and dated layout helpers.

- [ ] T009 Implement `RunIdentity` value type (fields: `uuid`, `run_id`, `short_id`, `created_at`) with constructors `from_now_v7()` and `from_uuid_with_created_at(uuid, created_at)` in `crates/canon-engine/src/domain/run.rs`.
- [ ] T010 [P] Implement `slugify(source: &str) -> Option<String>` per the pipeline in [contracts/run-identity-contract.md](contracts/run-identity-contract.md) §C-3, in new file `crates/canon-engine/src/persistence/slug.rs`; add unit tests covering: empty, all-punctuation, non-ASCII fold/drop, length cap with trailing-dash trim, collapse repeats, leading/trailing trim.
- [ ] T011 [P] Extend `crates/canon-engine/src/persistence/layout.rs` with `run_dir_for(identity: &RunIdentity, slug: Option<&str>)` returning `.canon/runs/YYYY/MM/<dir-name>/`, plus `parse_run_dir_name(name: &str) -> Option<(DisplayId, Option<&str>)>` using `split_once("--")` only; preserve existing `run_dir(run_id)` for backward compatibility.
- [ ] T012 Add `RunManifest` schema fields `uuid`, `run_id`, `short_id`, `created_at`, `slug?`, `title?` in `crates/canon-engine/src/persistence/manifests.rs`; implement read shim that derives `run_id` / `short_id` / `created_at` from `uuid` (and directory mtime when needed) when older manifests omit them.
- [ ] T013 Populate `tests/fixtures/legacy_uuid_run/` with a UUID-shaped directory containing a minimal pre-feature manifest (only `uuid` + existing required fields); used by lookup and read-shim tests.
- [ ] T014 Replace each `Uuid::now_v7().to_string()` site in `crates/canon-engine/src/orchestrator/service.rs` with `RunIdentity::from_now_v7()`, threading `RunIdentity` through `RunRequest` / `RunContext` while keeping `run_id: String` accessor stable for downstream callers.

## Phase 3: User Story 1 — Operator references runs by human-friendly id (P1)

**Story goal**: An operator can resolve any run via printed `run_id`,
unique `short_id`, or full `uuid`, including legacy UUID-keyed runs.

**Independent test**: Quickstart steps 1, 3, 4, 7 ([quickstart.md](quickstart.md)).

- [ ] T015 [US1] Replace ignored stubs in `tests/contract/run_identity_contract.rs` with active tests for §C-1 (manifest schema), §C-2 (display-id regex), §C-4 (path layout + first-`--` parser), and §C-8 (legacy compat read).
- [ ] T016 [US1] Add active tests in `tests/integration/run_lookup.rs` for: (a) lookup by full `run_id`, (b) lookup by full `uuid`, (c) lookup by unique `short_id`, (d) ambiguity error listing matches in deterministic order, (e) lookup against the legacy UUID fixture.
- [ ] T017 [US1] Implement `RunHandle`, `LookupQuery`, `LookupError` in new file `crates/canon-engine/src/persistence/lookup.rs`; resolver walks `.canon/runs/YYYY/MM/*/` and the legacy `.canon/runs/<UUID>/` directories, parses each `manifest.toml`, and matches per [data-model.md](data-model.md) "RunHandle" / "LookupQuery" tables.
- [ ] T018 [US1] Wire `crates/canon-engine/src/persistence/store.rs` to write new runs at the dated path returned by `layout.run_dir_for(...)` and to keep evidence/snapshot path strings (`runs/<…>/inputs/...`, `runs/<…>/evidence.toml`, `runs/<…>/invocations/...`) consistent with the new directory.
- [ ] T019 [US1] Update `crates/canon-cli/src/app.rs` so the `run-id` positional arg accepts `R-YYYYMMDD-XXXXXXXX`, full UUID, prefix `short_id`, or `@last`.
- [ ] T020 [US1] Update `crates/canon-cli/src/commands/run.rs` to print both `uuid` and `run_id` on run creation, using the value formatted by `output.rs`.
- [ ] T021 [P] [US1] Update `crates/canon-cli/src/commands/status.rs` to resolve via `persistence::lookup`.
- [ ] T022 [P] [US1] Update `crates/canon-cli/src/commands/inspect.rs` to resolve via `persistence::lookup` for both evidence and artifacts subcommands.
- [ ] T023 [P] [US1] Update `crates/canon-cli/src/commands/approve.rs` (or equivalent) to resolve via `persistence::lookup`.
- [ ] T024 [P] [US1] Update `crates/canon-cli/src/commands/resume.rs` to resolve via `persistence::lookup`.
- [ ] T025 [US1] Update `crates/canon-cli/src/output.rs` so run summaries print `run_id` (primary) and `uuid` (secondary) consistently.
- [ ] T026 [US1] Run `cargo nextest run -E 'test(run_identity_contract) | test(run_lookup)'` and append output path to [validation-report.md](validation-report.md) §"Evidence index".

**Checkpoint A — Independent reviewer (US1)**: reviewer executes [quickstart.md](quickstart.md) steps 1, 3, 4, 7 and signs off in `validation-report.md`. Implementation MUST NOT proceed to Phase 4 if reviewer flags any failure.

## Phase 4: User Story 2 — Authored inputs vs immutable snapshot (P1)

**Story goal**: `canon-input/` is never mutated by the runtime;
`.canon/runs/<…>/inputs/` is a stable, digest-preserving snapshot.

**Independent test**: Quickstart step 2 ([quickstart.md](quickstart.md)).

- [ ] T027 [US2] Replace ignored stubs in `tests/integration/inputs_snapshot_immutability.rs` with active tests for: (a) sha256 of every file under `canon-input/` is byte-stable across run create / status / inspect / approve / resume; (b) sha256 of every file under `.canon/runs/<…>/inputs/` is byte-stable across the same commands; (c) editing or deleting an authored file after run creation does not affect snapshot or run loading.
- [ ] T028 [US2] Add an active contract test in `tests/contract/run_identity_contract.rs` for §C-5 (snapshot immutability and authored-surface non-mutation): assert no `std::fs::write`, `OpenOptions::write(true)`, or rename targets a path under `canon-input/` from any persistence call site.
- [ ] T029 [US2] Audit `crates/canon-engine/src/persistence/store.rs`, `crates/canon-engine/src/persistence/manifests.rs`, `crates/canon-engine/src/persistence/invocations.rs`, and `crates/canon-engine/src/orchestrator/{service,gatekeeper,verification_runner}.rs` for any write path that could touch `canon-input/`; refactor each to write only beneath `.canon/`.
- [ ] T030 [US2] Confirm snapshot writer in `store.rs` (currently around `fingerprint.snapshot_ref = Some(format!("runs/{run_id}/inputs/{snapshot_name}"))`) writes once at run creation and is never re-invoked from gate / approve / resume code paths.
- [ ] T031 [US2] Run `cargo nextest run -E 'test(inputs_snapshot_immutability)'` and append output to [validation-report.md](validation-report.md).

**Checkpoint B — Independent reviewer (US2)**: reviewer greps for write
calls under `canon-input/` and confirms quickstart step 2 passes; signs
off in `validation-report.md`.

## Phase 5: User Story 3 — Listing runs (P2)

**Story goal**: Operator can list runs with `run_id`, mode, slug/title,
`created_at`, and state, sorted by `created_at` descending.

**Independent test**: Quickstart step 5 ([quickstart.md](quickstart.md)).

- [ ] T032 [US3] Add an active contract test in `tests/contract/run_identity_contract.rs` for §C-7 (listing columns and sort order), using a CLI snapshot file under `tests/snapshots/list_runs.snap`.
- [ ] T033 [US3] Implement `crates/canon-cli/src/commands/list.rs` (or extend an existing `list` command) that calls `persistence::lookup::scan_all()` and prints rows `run_id | mode | slug-or-title | created_at | state`; wire into `crates/canon-cli/src/app.rs`.
- [ ] T034 [US3] Ensure the listing also includes legacy UUID-keyed runs from the fixture, exercised by the snapshot test in T032.
- [ ] T035 [US3] Run `cargo nextest run -E 'test(list_runs)'` and append output to [validation-report.md](validation-report.md).

**Checkpoint C — Independent reviewer (US3)**: reviewer executes
quickstart step 5 and confirms required columns / sort; signs off.

## Phase 6: User Story 4 — `@last` shortcut (P3)

**Story goal**: `@last` resolves to the most recent run by `created_at`.

**Independent test**: Quickstart step 6 ([quickstart.md](quickstart.md)).

- [ ] T036 [US4] Add active tests in `tests/integration/run_lookup.rs` for: (a) `@last` against a multi-run repo returns the latest by `created_at`, (b) `@last` against an empty repo returns `LookupError::EmptyHistory`.
- [ ] T037 [US4] Implement `LookupQuery::Last` branch in `crates/canon-engine/src/persistence/lookup.rs`, ordering by `run_id` lexicographically with `created_at` tiebreaker (per [research.md](research.md) R-008).
- [ ] T038 [US4] Confirm CLI parsing in `crates/canon-cli/src/app.rs` accepts the literal `@last` token without quoting friction on common shells.

## Final Phase: Verification & Compliance

- [ ] T039 Update [MODE_GUIDE.md](../../MODE_GUIDE.md), [README.md](../../README.md), and CLI `--help` text to speak in `run_id` (user-facing) / `uuid` (internal) / `slug`+`title` (metadata) terms; daily-use examples MUST use the installed `canon` binary, not `cargo run`.
- [ ] T040 Update embedded skill help text under `defaults/embedded-skills/canon-status/`, `…/canon-inspect-evidence/`, `…/canon-inspect-artifacts/`, `…/canon-approve/`, `…/canon-resume/` to use `run_id` examples; mirror updates in `.agents/skills/` counterparts.
- [x] T041 Update `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml` and `.agents/skills/canon-shared/references/runtime-compatibility.toml` to bump compatible runtime version (e.g. 0.8.x → 0.9.0) reflecting the new manifest schema and resolver surface.
- [ ] T042 Run `/bin/bash scripts/validate-canon-skills.sh` and `pwsh -File scripts/validate-canon-skills.ps1`; both MUST pass.
- [ ] T043 Run the full validation gate: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run`; capture output and append paths to [validation-report.md](validation-report.md) §"Evidence index".
- [ ] T044 Generate `cargo llvm-cov --workspace --all-features --lcov --output-path target/tmp/009-lcov.info`; confirm touched-patch coverage on `persistence/lookup.rs`, `persistence/slug.rs`, `persistence/layout.rs`, and the manifest read shim is ≥ 85 %; record in [validation-report.md](validation-report.md).
- [ ] T045 Independent human reviewer (not the implementer) executes the full [quickstart.md](quickstart.md) walkthrough, performs the four review activities listed in [validation-report.md](validation-report.md) §3, and signs off the bounded-impact approval gate by checking the four boxes under [validation-report.md](validation-report.md) §"Approval gate".
- [ ] T046 Tag a brief AGENTS.md entry under "Active Technologies" for this feature if anything new was introduced; otherwise confirm the auto-update from `.specify/scripts/bash/update-agent-context.sh codex` is sufficient.

---

## Dependencies

```
Phase 0 (T001-T004)
   │
Phase 1 (T005-T008)
   │
Phase 2 (T009-T014)        ← foundational; blocks all user stories
   │
   ├──► Phase 3 US1 (T015-T026)
   │        │
   │        ▼
   │     Checkpoint A
   │        │
   ├──► Phase 4 US2 (T027-T031)        — runs in parallel with US1 wiring once T012/T014 land
   │        │
   │        ▼
   │     Checkpoint B
   │
   ├──► Phase 5 US3 (T032-T035)        — depends on US1 resolver landing (T017)
   │        │
   │        ▼
   │     Checkpoint C
   │
   └──► Phase 6 US4 (T036-T038)        — depends on US1 resolver landing (T017)

Final Phase (T039-T046)                — runs after all user-story phases
```

**Story dependency notes**:

- US1 (resolver + new layout) is the linchpin; US3 and US4 depend on the
  resolver and lister built in US1.
- US2 (input non-mutation) is logically independent of US1's CLI work
  but shares persistence files; sequence US2 implementation tasks after
  T014 to avoid merge conflicts in `store.rs` / `manifests.rs`.

## Parallel execution opportunities

**Phase 1 (Setup)**:

```
T007 ─┐
T008 ─┴── parallel  (different test files)
```

**Phase 2 (Foundational)**:

```
T010 ─┐
T011 ─┴── parallel  (slug.rs and layout.rs are disjoint)
```

**Phase 3 (US1) CLI wiring (after T017–T019 land)**:

```
T021 ─┐
T022 ─┤
T023 ─┼── parallel  (one CLI command file each)
T024 ─┘
```

## Independent test criteria per story

| Story | Independent test reference                                  | Pass criterion                                                    |
|-------|-------------------------------------------------------------|-------------------------------------------------------------------|
| US1   | [quickstart.md](quickstart.md) steps 1, 3, 4, 7             | run_id printed; resolution by run_id / short_id / uuid / legacy UUID; ambiguity reported |
| US2   | [quickstart.md](quickstart.md) step 2                       | authored files byte-stable; snapshot byte-stable across commands  |
| US3   | [quickstart.md](quickstart.md) step 5                       | listing shows required columns; new + legacy runs both appear     |
| US4   | [quickstart.md](quickstart.md) step 6                       | `@last` resolves correctly; empty repo errors clearly             |

## Validation evidence and review checkpoints

- Structural: T005, T042, T043 → CI logs and `target/tmp/009-baseline.log`.
- Logical: T015, T016, T026, T027, T031, T032, T035, T036 → nextest output paths recorded in [validation-report.md](validation-report.md) §1–§2.
- Coverage: T044 → `target/tmp/009-lcov.info`.
- Independent review checkpoints: A (after Phase 3), B (after Phase 4),
  C (after Phase 5), final sign-off T045.

## Implementation strategy

**Suggested MVP**: Phase 0 → Phase 1 → Phase 2 → Phase 3 (US1) →
Checkpoint A. At that point Canon already prints `run_id`, runs land at
the new dated path, every existing CLI command resolves runs by
`run_id` / `short_id` / `uuid` / legacy UUID, and the foundational
contract tests are green. Stories US2, US3, US4 then layer on top
without rework.

**Order of execution**:

1. T001–T008 (governance, baseline, scaffolds).
2. T009–T014 (foundational types, schema, layout, identity threading).
3. T015–T026 (US1 — MVP).
4. **Checkpoint A** (independent review of US1).
5. T027–T031 (US2) and T032–T035 (US3) and T036–T038 (US4) — can be
   sequenced or parallelized per available reviewer cycles.
6. **Checkpoints B, C** as their phases complete.
7. T039–T046 (docs, skill text, validators, coverage, final independent
   review, AGENTS.md).
