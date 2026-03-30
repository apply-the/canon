# Validation Report Plan: Runnable Skill Interaction and Ref-Safe Input Binding

## Status

MVP direct-scope validation complete. Optional Phase 6 reuse addendum complete.

Completed in this implementation pass:

- governance artifacts aligned to the helper-enforced typed-input contract
- Bash and PowerShell validator baselines updated
- direct-scope skill docs updated to mirror helper-enforced behavior
- shared Bash and PowerShell preflight helpers updated for typed output,
  canonical risk / zone normalization, deterministic failure classes, and
  local-only ref handling
- Bash and PowerShell structural validation and probe evidence captured

Optional follow-on work only:

- no additional follow-on work is required for the shipped optional reuse
  tranche in this feature

## Validation Ownership

- generation side:
  - updated skill contracts
  - updated shared preflight helpers
- validation side:
  - shell validator
  - PowerShell validator
  - runnable walkthrough review against real Canon CLI behavior
  - independent contract review against current Canon runtime docs and engine
    expectations

## Structural Validation

Planned checks:

- `scripts/validate-canon-skills.sh`
- `pwsh -File scripts/validate-canon-skills.ps1`
- `git diff --check`
- repo-relative path hygiene for new feature docs

Expected assertions:

- `canon-pr-review` preflight uses `--ref` for base/head
- path-oriented runnable skills continue to use `--input`
- affected runnable skills describe incremental correction and preserved valid
  inputs
- modeled-only skills remain unchanged in support-state honesty

Observed results on 2026-03-29:

- `/bin/bash scripts/validate-canon-skills.sh`: PASS
- `pwsh -File scripts/validate-canon-skills.ps1`: PASS
- `git diff --check`: PASS after each implementation burst

Helper output checkpoints:

- failure responses must include `STATUS`, `CODE`, `PHASE`, `COMMAND`,
  `REPO_ROOT`, `MESSAGE`, and `ACTION`
- typed failures must include `FAILED_SLOT` and, when applicable,
  `FAILED_KIND`
- ready or normalized responses must include the applicable normalized output
  keys: `NORMALIZED_RUN_ID`, `NORMALIZED_INPUT_1`, `NORMALIZED_REF_1`,
  `NORMALIZED_REF_2`
- helper diagnostics such as `VERSION_KIND` and `DETECTED_VERSION` may appear,
  but validation must not depend on them

## Shared Preflight Validation Matrix

| Case | Expected Result |
| --- | --- |
| missing `zone` after valid `owner` and `risk` | `STATUS=missing-input`, `FAILED_SLOT=zone`, preserved owner/risk |
| invalid risk token | `STATUS=invalid-input`, `FAILED_SLOT=risk` |
| invalid zone token | `STATUS=invalid-input`, `FAILED_SLOT=zone` |
| missing run id | `STATUS=missing-input`, `FAILED_SLOT=run-id` |
| unresolved run id | `STATUS=invalid-input`, `FAILED_SLOT=run-id` |
| missing input path | `STATUS=missing-file`, `FAILED_SLOT=input-path` |
| `canon-pr-review` with `master` and `HEAD` | `STATUS=ready`, normalized base/head returned |
| `canon-pr-review` with `main` and `HEAD` in master-only local repo | `STATUS=invalid-ref`, no file-path wording |
| `canon-pr-review` with `origin/main` | `STATUS=invalid-ref`, `FAILED_KIND=unsupported-remote-ref` |
| `canon-pr-review` with same normalized base/head | `STATUS=malformed-ref-pair` |

These cases must pass in both Bash and PowerShell.

Evidence for each case must capture the emitted `PHASE`, `ACTION`, and any
`FAILED_SLOT` / `FAILED_KIND` or normalized output keys needed to prove the
typed contract.

Observed Bash helper evidence on 2026-03-29:

| Case | Observed Result |
| --- | --- |
| missing `zone` after valid `owner` and `risk` | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=zone`, `FAILED_KIND=ZoneField`, `ACTION=Retry with --zone <ZONE>.` |
| missing input path | `STATUS=missing-file`, `CODE=15`, `PHASE=preflight`, `FAILED_SLOT=input-path`, `FAILED_KIND=FilePathInput`, `ACTION=Retry with an existing file path.` |
| `canon-pr-review` with `master` and `HEAD` | `STATUS=ready`, `CODE=0`, `PHASE=preflight`, `NORMALIZED_REF_1=refs/heads/master`, `NORMALIZED_REF_2=HEAD` |
| `canon-pr-review` with `main` and `HEAD` in a master-only local fixture | `STATUS=invalid-ref`, `CODE=16`, `PHASE=preflight`, `FAILED_SLOT=base-ref`, `FAILED_KIND=RefInput`, `ACTION=Retry with master or explicit refs/heads/master.` |
| `canon-pr-review` with `origin/main` | `STATUS=invalid-ref`, `CODE=16`, `PHASE=preflight`, `FAILED_SLOT=base-ref`, `FAILED_KIND=unsupported-remote-ref`, `ACTION=Retry with a local branch, explicit refs/heads/<name>, or HEAD.` |
| `canon-pr-review` with same normalized base/head | `STATUS=malformed-ref-pair`, `CODE=18`, `PHASE=preflight`, `FAILED_SLOT=ref-pair`, `FAILED_KIND=RefPairInput`, `NORMALIZED_REF_1=refs/heads/master`, `NORMALIZED_REF_2=refs/heads/master` |
| wrong repo context | `STATUS=wrong-repo-context`, `CODE=12`, `PHASE=preflight`, `ACTION=Switch into the intended repository root before invoking this skill.` |
| repo not initialized | `STATUS=repo-not-initialized`, `CODE=13`, `PHASE=preflight`, `ACTION=Run $canon-init or canon init ... first.` |
| ready requirements case | `STATUS=ready`, `CODE=0`, `PHASE=preflight`, `NORMALIZED_INPUT_1=specs/004-ref-safe-binding/spec.md` |

Observed PowerShell helper evidence on 2026-03-29:

| Case | Observed Result |
| --- | --- |
| missing `zone` after valid `owner` and `risk` | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=zone`, `FAILED_KIND=ZoneField`, `ACTION=Retry with --zone <ZONE>.` |
| missing input path | `STATUS=missing-file`, `CODE=15`, `PHASE=preflight`, `FAILED_SLOT=input-path`, `FAILED_KIND=FilePathInput`, `ACTION=Retry with an existing file path.` |
| `canon-pr-review` with `master` and `HEAD` | `STATUS=ready`, `CODE=0`, `PHASE=preflight`, `NORMALIZED_REF_1=refs/heads/master`, `NORMALIZED_REF_2=HEAD` |
| `canon-pr-review` with `main` and `HEAD` in master-only local fixture | `STATUS=invalid-ref`, `CODE=16`, `PHASE=preflight`, `FAILED_SLOT=base-ref`, `FAILED_KIND=RefInput`, `ACTION=Retry with master or explicit refs/heads/master.` |
| `canon-pr-review` with `origin/main` | `STATUS=invalid-ref`, `CODE=16`, `PHASE=preflight`, `FAILED_SLOT=base-ref`, `FAILED_KIND=unsupported-remote-ref`, `ACTION=Retry with a local branch, explicit refs/heads/<name>, or HEAD.` |
| `canon-pr-review` with same normalized base/head | `STATUS=malformed-ref-pair`, `CODE=18`, `PHASE=preflight`, `FAILED_SLOT=ref-pair`, `FAILED_KIND=RefPairInput`, `NORMALIZED_REF_1=refs/heads/master`, `NORMALIZED_REF_2=refs/heads/master` |
| wrong repo context | `STATUS=wrong-repo-context`, `CODE=12`, `PHASE=preflight`, `ACTION=Switch into the intended repository root before invoking this skill.` |
| repo not initialized | `STATUS=repo-not-initialized`, `CODE=13`, `PHASE=preflight`, `ACTION=Run $canon-init or canon init ... first.` |
| ready requirements case | `STATUS=ready`, `CODE=0`, `PHASE=preflight`, `NORMALIZED_INPUT_1=specs/004-ref-safe-binding/spec.md` |

## Runnable Walkthrough Validation

### `canon-pr-review`

Required walkthroughs:

- `base master, head HEAD`
- `base main, head HEAD` in a repo whose usable local branch is `master`
- one ref missing while the other remains valid
- remote-like ref rejected deterministically
- Canon execution begins only after typed preflight returns `ready`

Observed Bash walkthrough notes:

- `base master, head HEAD` in a temporary local fixture passed preflight with
  `NORMALIZED_REF_1=refs/heads/master` and `NORMALIZED_REF_2=HEAD`
- `base main, head HEAD` in the same master-only fixture failed as
  `invalid-ref`, not as `missing-file`
- `base origin/main, head HEAD` failed as `invalid-ref` with
  `FAILED_KIND=unsupported-remote-ref`
- `base master, head master` failed as `malformed-ref-pair`

### `canon-requirements`

Required walkthroughs:

- valid `owner` and `risk`, missing `zone`
- missing input path
- single corrected field retry without re-entering valid fields

Observed Bash walkthrough notes:

- missing `zone` with valid `owner` and `risk` failed as `missing-input` with
  `FAILED_SLOT=zone`
- the helper reported only the missing slot and preserved the valid run-start
  metadata for the retry path
- after a `ready` preflight, a real `canon run --mode requirements ...`
  completed successfully with run id `019d3b39-a899-74b1-b601-39f7aa8271cb`

Observed Canon execution proof:

- command: `canon run --mode requirements --risk bounded-impact --zone yellow --owner reviewer --input specs/004-ref-safe-binding/spec.md --output json`
- result: `state=Completed`, `artifact_count=6`, `invocations_total=4`,
  `invocations_denied=1`
- interpretation: Canon execution started only after the helper returned
  `STATUS=ready`, and the CLI contract accepted the same normalized input form

### `canon-brownfield`

Required walkthroughs:

- valid ownership metadata, missing brief path
- corrected path only, with ownership metadata preserved

Observed Bash walkthrough notes:

- missing brief path failed as `missing-file` with `FAILED_SLOT=input-path`
- the helper left ownership metadata untouched and asked only for an existing
  file path

### Run-id-Oriented Skills

Required walkthroughs:

- `canon-status` asks only for missing `RUN_ID`
- `canon-resume` asks only for missing `RUN_ID`
- `canon-approve` preserves valid non-run-id fields when only `RUN_ID` needs
  correction

## Independent Validation Focus

Independent review must confirm:

- semantically valid refs are never treated as file paths
- retry guidance matches the exact accepted Canon binding form
- preflight failures and Canon-execution failures are explicitly separated
- shell and PowerShell behavior remain materially identical

Current assessment:

- Bash-side evidence supports the direct-scope MVP claims
- PowerShell runtime evidence matches the Bash helper for all MVP cases that
  were planned in the validation matrix
- final independent review for the direct-scope MVP is complete

Independent review conclusion on 2026-03-29:

- retry guidance matches the current Canon CLI contract for `requirements`,
  `brownfield-change`, and `pr-review`
- semantically valid refs are never treated as file paths in the direct-scope
  patch
- preflight failures and Canon-execution boundaries are explicit in the skill
  docs and in helper output
- Canon CLI remains the only execution engine; the skill layer does not attempt
  to simulate runtime outcomes

Wording drift review:

- no further quickstart or decision-log wording changes were required after the
  final validator and probe pass

## MVP Verification Checkpoints

The MVP closeout must prove all of the following before optional reuse is
considered complete:

- valid `owner` and `risk` survive a retry where only `zone` was missing
- correcting one missing field does not require re-entering all valid fields
- `canon-pr-review` accepts `base master, head HEAD` and emits canonical retry
  forms
- `canon-pr-review` rejects `base main, head HEAD` in a master-only local repo
  as `invalid-ref`, not as a missing file
- semantically valid refs are never treated as file paths
- Canon execution starts only after typed preflight returns `ready`
- retry guidance matches the exact Canon CLI contract actually invoked

## Optional Reuse Addendum Checkpoints

If the Phase 6 reuse tranche ships, capture separate evidence that:

- `canon-status`, inspect skills, and `canon-resume` ask only for the missing
  `RUN_ID`
- `canon-approve` reuses run-id handling without broadening approval metadata
  into a new shared taxonomy
- the reuse addendum does not change MVP readiness criteria

Observed optional reuse addendum on 2026-03-29:

- `canon-status`, `canon-inspect-invocations`, `canon-inspect-evidence`,
  `canon-inspect-artifacts`, and `canon-resume` now ask only for the missing
  or invalid `RUN_ID` and show the exact Canon CLI retry form for that skill
- `canon-approve` reuses shared run-id handling while keeping `TARGET`, `BY`,
  `DECISION`, and `RATIONALE` skill-local
- addendum validation reran the Bash and PowerShell structure checks without
  changing MVP readiness criteria

Observed Bash addendum helper evidence on 2026-03-29:

| Case | Observed Result |
| --- | --- |
| `status` with missing run id | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Retry with --run-id <RUN_ID>.` |
| `status` with unknown run id | `STATUS=invalid-input`, `CODE=17`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Check the run id and retry with an existing run.` |
| `inspect-invocations` with missing run id | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Retry with --run-id <RUN_ID>.` |
| `inspect-evidence` with missing run id | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Retry with --run-id <RUN_ID>.` |
| `inspect-artifacts` with missing run id | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Retry with --run-id <RUN_ID>.` |
| `resume` with missing run id | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Retry with --run-id <RUN_ID>.` |
| `approve` with missing run id | `STATUS=missing-input`, `CODE=14`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Retry with --run-id <RUN_ID>.` |
| `approve` with unknown run id | `STATUS=invalid-input`, `CODE=17`, `PHASE=preflight`, `FAILED_SLOT=run-id`, `FAILED_KIND=RunIdInput`, `ACTION=Check the run id and retry with an existing run.` |

## Evidence To Record After Implementation

- validator command outputs
- targeted shell preflight outputs
- targeted PowerShell preflight outputs
- runnable walkthrough notes for affected skills
- any discovered deviations and corrective follow-up

Recorded so far:

- Bash structural validation pass from `scripts/validate-canon-skills.sh`
- PowerShell structural validation pass from `scripts/validate-canon-skills.ps1`
- Bash helper probe matrix covering missing input, missing file, ref success,
  invalid local ref, unsupported remote ref, malformed ref pair, wrong repo
  context, repo-not-initialized, and ready preflight cases
- PowerShell helper probe matrix covering the same MVP cases with materially
  identical status names, failure classes, actions, and normalized outputs
- one real completed `requirements` run proving post-preflight Canon execution
- Bash addendum validation pass after Phase 6 run-id reuse updates
- PowerShell addendum validation pass after Phase 6 run-id reuse updates
- run-id helper probes for `status`, inspect skills, `resume`, and `approve`
  confirming `missing-input` and `invalid-input` classification stays scoped to
  `FAILED_SLOT=run-id`

Closeout confirmation:

- direct-scope MVP tasks are implemented and validated
- optional reuse Phase 6 tasks are implemented and validated
- `git diff --check` passes on the final tree
- repo-relative artifact paths were preserved in the feature docs
- bounded-impact invariants remain intact: Canon CLI is still the only
  execution engine, shared helpers remain the enforcement point, and typed
  interaction memory remains intra-interaction only

## Optional Reuse Review Addendum

Observed addendum evidence on 2026-03-29:

- `status` with missing run id returned `STATUS=missing-input`,
  `FAILED_SLOT=run-id`, and the run-id-only retry action
- `status` with an unknown run id returned `STATUS=invalid-input`,
  `FAILED_SLOT=run-id`, and a retry action limited to an existing run id
- `resume` and `approve` used the same helper classification for missing and
  invalid run ids without widening retry guidance into other fields
- inspect skills now mirror the same run-id-only correction contract in prose

Addendum review conclusion:

- Phase 6 stayed within shared run-id reuse and did not introduce a new
  approval-field taxonomy
- Canon CLI remains the only execution engine and system of record
- the optional reuse tranche improves retry precision without changing the
  direct-scope MVP invariants
