# Implementation Plan: Runnable Skill Interaction and Ref-Safe Input Binding

**Branch**: `004-ref-safe-binding` | **Date**: 2026-03-29 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/004-ref-safe-binding/spec.md`

## Summary

This patch repairs trust in existing runnable Canon skills without changing the
Canon runtime or the overall Codex skills architecture. The implementation
stays narrow:

- extend the shared preflight layer so it classifies typed inputs explicitly
  instead of overloading file-path checks
- update runnable skill instructions so missing fields are collected
  incrementally and valid fields are preserved within the current interaction
- make `canon-pr-review` ref-safe by accepting only deterministic local ref
  forms, normalizing them to the exact Canon CLI contract, and rejecting
  remote-ref ambiguity in this increment
- separate preflight failures from Canon-execution failures so retry guidance
  matches the real binding instead of sounding plausible but wrong

The proving case is `canon-pr-review`, while `canon-brownfield`,
`canon-requirements`, and run-id-driven operational skills opportunistically
reuse the same typed-input approach.

## Governance Context

**Execution Mode**: `brownfield` because this increment patches the current
Codex skills layer and shared helper scripts in place without redesigning
Canon  
**Risk Classification**: `bounded-impact` because runnable-skill misbinding can
start the wrong governed command or make delivered skills look dishonest, but
the blast radius stays bounded to the repo-local skills frontend while Canon
CLI remains the only execution engine

**Scope In**: typed input handling for executable skills, incremental
missing-input collection, ref-safe preflight and retry rendering for
`canon-pr-review`, deterministic failure semantics, and minimal shared helper
changes under `.agents/skills/canon-shared`  
**Scope Out**: Canon core runtime redesign, support-state taxonomy redesign,
plugin packaging, MCP runtime work, provider/runtime changes, generic
interactive form infrastructure, modeled-only skill redesign, and any hidden
second runtime inside Codex skills

**Invariants**:

- Canon CLI remains the only execution engine and system of record.
- Runnable skills remain thin deterministic frontends over Canon commands.
- No Canon command may start before typed preflight succeeds for all required
  inputs.
- Input preservation is limited to the current interaction only and is never
  persisted into `.canon/`, repo files, or cross-skill memory.
- `canon-pr-review` ref slots are always treated as refs first, never as file
  paths.

**Decision Log**: [decision-log.md](./decision-log.md)  
**Validation Ownership**: skill docs and shared helpers define the wrapper
contract; Canon CLI executes the real workflow; shell and PowerShell validators,
walkthroughs, and independent contract review validate that the frontend
matches the runtime rather than trusting skill prose alone  
**Approval Gates**: human review is required before changing runnable-skill
binding contracts, shared preflight status codes, or the accepted ref forms for
`canon-pr-review`

## 1. Technical Context

**Language/Version**: Markdown `SKILL.md` files, repo-local Bash and PowerShell
helper scripts, existing Canon Rust 1.95.0 CLI/runtime  
**Primary Dependencies**: installed `canon` binary, Git command-line ref
inspection (`git rev-parse`, `git show-ref`, `git for-each-ref`, `git remote`),
existing `.agents/skills/canon-shared/scripts/check-runtime.*`, and current
skill validation scripts  
**Storage**: repo-local skill files and shared helper scripts plus existing
`.canon/` runtime state; no new persistent state introduced by this patch  
**Testing**: `scripts/validate-canon-skills.sh`, `pwsh -File
scripts/validate-canon-skills.ps1`, targeted shell and PowerShell preflight
probes, walkthrough validation against a real initialized repo, and
`git diff --check` for artifact hygiene  
**Target Platform**: Codex on macOS, Linux, and Windows with shell and
PowerShell parity  
**Project Type**: repo-local skills frontend over a local-first governed CLI  
**Existing System Touchpoints**: `.agents/skills/canon-pr-review/SKILL.md`,
`.agents/skills/canon-brownfield/SKILL.md`,
`.agents/skills/canon-requirements/SKILL.md`,
`.agents/skills/canon-status/SKILL.md`,
`.agents/skills/canon-inspect-invocations/SKILL.md`,
`.agents/skills/canon-inspect-evidence/SKILL.md`,
`.agents/skills/canon-inspect-artifacts/SKILL.md`,
`.agents/skills/canon-approve/SKILL.md`,
`.agents/skills/canon-resume/SKILL.md`,
`.agents/skills/canon-shared/scripts/check-runtime.sh`,
`.agents/skills/canon-shared/scripts/check-runtime.ps1`,
`scripts/validate-canon-skills.sh`, and
`scripts/validate-canon-skills.ps1`  
**Performance Goals**: preflight and retry rendering stay under one second
excluding Canon execution; ref validation performs only local Git queries and
never fetches or mutates repository state  
**Constraints**: no generic form engine, no hidden conversational memory, no
Canon runtime changes, no remote-ref magic, and no retry guidance that diverges
from the exact accepted Canon binding  
**Scale/Scope**: direct patch of 3 run-starting skills, opportunistic reuse
across 6 run-id-oriented skills, and one shared preflight surface with shell
and PowerShell parity

## 2. Constitution Check

### Pre-Design Gate

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Required approval checkpoints are named
- [x] No constitution deviations are required before design

### Post-Design Re-Check

- [x] Canon runtime authority remains intact
- [x] Shared helpers stay bounded and do not become a second runtime
- [x] Typed input preservation remains intra-interaction only
- [x] Ref binding is deterministic and explicit rather than magical
- [x] Failure semantics stay actionable and aligned with real Canon commands
- [x] Validation covers both shell and PowerShell contracts plus runtime parity

**Result**: PASS. This plan tightens runnable-skill correctness without
violating any constitutional control.

## 3. Current Failure Analysis

The current frontend gap is implementation-detail drift, not product capability
drift.

- `.agents/skills/canon-pr-review/SKILL.md` currently sends both ref values to
  `check-runtime.*` through repeated `--input`, even though the shared helper
  already has a separate `--ref` channel.
- `.agents/skills/canon-shared/scripts/check-runtime.sh` and
  `.agents/skills/canon-shared/scripts/check-runtime.ps1` treat `--input`
  values as file-path-like unless the token already looks like `refs/*` or
  `HEAD`. Short semantic branch names like `main` or `master` therefore fail as
  missing file inputs instead of invalid or normalized refs.
- Missing-input handling is largely described in prose inside `SKILL.md` files
  rather than expressed as explicit typed slots. That makes retries brittle and
  encourages users to restate too much data after one failure.
- Retry wording today can tell the user to provide a “valid file path or ref”
  even when the runnable skill only accepts refs in that slot, which erodes
  trust even though the underlying Canon mode exists.
- Run-id-driven skills already have a simpler surface, but they still benefit
  from the same typed retry discipline so users do not re-enter valid values
  unnecessarily.

The narrow repair is therefore to make the current wrapper contract typed,
deterministic, and skill-aware, not to add a new orchestration layer.

## 4. Typed Input Model Changes

### 4.1 Ephemeral Interaction Model

The implementation introduces a logical `RunnableSkillInteraction` model for
the active skill invocation only. It is not persisted anywhere; it exists to
organize how skill instructions and preflight outputs talk about missing,
invalid, and normalized values.

Each active interaction tracks:

- `skill_name`
- `command_name`
- `slots`
- `preserved_valid_values`
- `phase` (`collecting`, `preflight-failed`, `ready`, `canon-executing`,
  `canon-returned`)

Each `TypedInputSlot` tracks:

- slot id such as `owner`, `risk`, `zone`, `run-id`, `input-path`, `base-ref`,
  `head-ref`
- input kind
- raw value
- normalized value, if normalization succeeds
- slot status (`missing`, `valid`, `invalid`)
- retry render value in semantic form and Canon CLI form

### 4.2 Input Classes

| Input Class | Validation | Normalization | Retry Rendering | Preservation Rule |
| --- | --- | --- | --- | --- |
| `OwnerField` | Non-empty after trim | Trim outer whitespace only | Semantic: `owner <VALUE>`; CLI: `--owner <VALUE>` | Preserve once non-empty |
| `RiskField` | Must match Canon risk tokens: `low-impact`, `bounded-impact`, `systemic-impact`, plus runtime-recognized aliases | Convert accepted aliases to canonical hyphenated token | Semantic first, CLI second using canonical token | Preserve canonical token |
| `ZoneField` | Must match Canon zone tokens: `green`, `yellow`, `red`, plus runtime-recognized aliases | Convert accepted aliases to canonical lowercase token | Semantic first, CLI second using canonical token | Preserve canonical token |
| `RunIdInput` | Non-empty, then must resolve under `.canon/runs/<RUN_ID>` | Trim only; never rewrite identity | Semantic: `run id <RUN_ID>`; CLI: `--run <RUN_ID>` | Preserve exact value if repo-local lookup passes |
| `FilePathInput` | Non-empty existing path, relative to repo root or explicit absolute path | Preserve repo-relative intent where possible; avoid absolute rewrite in docs | Semantic: `input path <PATH>`; CLI: `--input <PATH>` | Preserve normalized path until user changes it |
| `RefInput` | Resolve against Git refs, not filesystem existence | `HEAD` stays `HEAD`; short local branch names normalize to `refs/heads/<name>`; explicit local refs stay explicit | Semantic: `base ref master`; CLI: `--input refs/heads/master` | Preserve normalized canonical ref |
| `RefPairInput` | Both sides present, ordered, and individually valid refs | Normalize sides independently while keeping base/head identity | Semantic pair plus exact CLI pair | Preserve valid side when the other side is missing or invalid |

### 4.3 Typed Output Contract from Preflight

`check-runtime.*` remains the shared preflight entrypoint, but its output
contract becomes more explicit. Planned emitted fields:

- required on every response: `STATUS`, `CODE`, `PHASE`, `COMMAND`,
  `REPO_ROOT`, `MESSAGE`, `ACTION`
- present on typed failures: `FAILED_SLOT`, `FAILED_KIND`
- present on success or normalization: `NORMALIZED_RUN_ID`,
  `NORMALIZED_INPUT_1`, `NORMALIZED_REF_1`, `NORMALIZED_REF_2` as applicable

This keeps the shared layer small while giving affected skills enough signal to
render deterministic retry guidance instead of reinterpreting raw error text.

## 5. Incremental Interaction Strategy

The frontend remains skill-oriented and deterministic. It does not become a
generic multi-step form engine.

### 5.1 General Flow

1. The active runnable skill maps user-provided values into the typed slots it
   requires.
2. Environment gates run first: Canon installed, version compatible, Git repo
   context valid, and `.canon/` initialized when required.
3. Slot validation then runs by input kind.
4. If any slot is missing or invalid, the skill asks only for the unresolved
   slot or coordinated pair and preserves the valid slots already collected in
   the current interaction.
5. Only after all slots are valid does the skill build the Canon command and
   start execution.
6. If Canon itself fails after preflight passes, the skill reports that the
   failure occurred inside Canon execution rather than during preflight.

### 5.2 Interaction Rules

- Do not force the user to restate a full one-line payload after one failure.
- Do not drop valid values that already passed typed validation.
- Echo preserved valid values when asking for a correction so the user can see
  what the skill is carrying forward.
- Ask for one unresolved field at a time unless the skill genuinely needs a
  coordinated pair.
- Keep all preserved state intra-interaction only. A new skill invocation
  starts from empty slots unless the user explicitly restates values.

### 5.3 Skill-Family Prompt Shapes

- `canon-requirements` and `canon-brownfield`:
  - if `owner`, `risk`, and `zone` are all missing, ask for them as separate
    named fields in one short structured prompt
  - if only one is missing or invalid, ask only for that field
  - for file-path inputs, ask for one path at a time and validate existence
    before retry
- `canon-pr-review`:
  - treat `base-ref` and `head-ref` as a coordinated pair
  - if both are missing, ask for both as named fields
  - if one side is valid and the other is not, keep the valid side and ask only
    for the unresolved side
- `canon-status`, inspect skills, and `canon-resume`:
  - ask only for `RUN_ID`
- `canon-approve`:
  - reuse shared run-id handling
  - keep `TARGET`, `BY`, `DECISION`, and `RATIONALE` skill-local but preserve
    valid entries inside the same interaction

## 6. Ref Normalization and Binding Strategy

This section closes the critical design decision for `canon-pr-review`.

### 6.1 Accepted Ref Forms in This Patch

This increment accepts only local, deterministic ref forms:

- `HEAD`
- explicit local refs in `refs/heads/<name>` form
- short local branch names that resolve to an existing `refs/heads/<name>`

This increment does **not** accept remote refs such as `origin/main` or
`refs/remotes/origin/main` as runnable inputs. Those are rejected with explicit
guidance rather than silently converted.

### 6.2 Resolution Order

For each ref slot (`base-ref`, then `head-ref`):

1. If the raw value is exactly `HEAD`, accept it and preserve `HEAD`.
2. If the raw value starts with `refs/heads/`, validate it with
   `git show-ref --verify --quiet <raw-ref>`.
3. Otherwise attempt exact local branch resolution by checking
   `refs/heads/<raw-value>`.
4. If local resolution fails, check whether the token matches a known remote
   name or `refs/remotes/*` pattern.
5. If it is remote-like, classify it as `invalid-ref` with
   `FAILED_KIND=unsupported-remote-ref` and suggest using a local branch or an
   explicit `refs/heads/*` ref.
6. Otherwise classify it as `invalid-ref` and, when the repo has an obvious
   local candidate, suggest that candidate without auto-substituting it.

### 6.3 Canonical Form Passed to Canon CLI

- `HEAD` is passed through unchanged.
- Short local branches are normalized to `refs/heads/<name>`.
- Explicit local refs stay explicit.
- The Canon command always receives the normalized pair through:
  `canon run --mode pr-review ... --input <CANONICAL_BASE> --input <CANONICAL_HEAD> --output json`

### 6.4 Repo-Aware Guidance Rules

- Use local branch discovery via `git for-each-ref refs/heads --format='%(refname:short)'`
  to suggest obvious alternatives.
- Do not fetch remotes, create tracking branches, or switch branches during
  preflight.
- Do not rewrite `main` to `master` or vice versa automatically. If `main`
  does not resolve locally in a repo that has `master`, return `invalid-ref`
  and suggest `master` or `refs/heads/master`.
- Never treat a semantically ref-shaped value as a missing file path inside
  `canon-pr-review`.

### 6.5 Pair Validation

`canon-pr-review` preflight validates the pair as a pair after validating each
side independently:

- both sides must be present before Canon starts
- if one side is missing, preserve the other side and ask only for the missing
  side
- if both sides normalize to the same canonical ref, return
  `STATUS=malformed-ref-pair` with guidance to provide a meaningful diff range

## 7. Failure-Handling Strategy

### 7.1 Preflight Failure Classes

| Failure Class | Planned Status | Planned Meaning | Retry Shape |
| --- | --- | --- | --- |
| Canon CLI missing | `cli-missing` | `canon` unavailable on `PATH` | Install Canon before retry |
| Canon CLI incompatible | `version-incompatible` | CLI present but command contract or version mismatches repo expectation | Reinstall or update Canon |
| Wrong repo context | `wrong-repo-context` | Not inside intended Git repository | Switch into repo root |
| Repo not initialized | `repo-not-initialized` | `.canon/` absent where required | Run `canon-init` first |
| Missing required field | `missing-input` | Slot absent or empty | Ask only for missing slot |
| Invalid owner/risk/zone/run id | `invalid-input` | Value present but malformed or unsupported for that slot | Re-enter only failing slot |
| Invalid ref | `invalid-ref` | Ref slot supplied but not resolvable or unsupported in current rules | Ask only for failing ref slot |
| Missing file path | `missing-file` | File-path slot supplied but no file exists at resolved location | Ask only for path slot |
| Malformed ref pair | `malformed-ref-pair` | Base/head pair fails pair-level invariant | Ask only for pair correction |

### 7.2 Message Rules

Every executable-skill failure message must:

- state whether the failure happened in `preflight` or `canon-execution`
- name the exact failing slot when one exists
- give an action that matches the real command contract
- avoid cross-kind language such as telling a ref slot to provide a file path

### 7.3 Canon-Execution Failures

When Canon fails after preflight succeeds:

- skill messaging must say preflight passed and Canon execution failed
- runtime errors are reported as Canon-backed failures, not re-labeled as
  preflight issues
- retry guidance must not suggest re-entering already accepted typed inputs
  unless Canon itself says the command contract changed

## 8. Shared Helper Changes

The shared layer stays intentionally small.

### 8.1 `check-runtime.sh` and `check-runtime.ps1`

Planned changes:

- keep `check-runtime.*` as the single shared preflight entrypoint
- reserve file-path validation for `--input`
- move `canon-pr-review` to repeated `--ref` arguments
- validate `RiskField` and `ZoneField` against Canon runtime tokens instead of
  only checking presence
- emit typed failure metadata (`FAILED_SLOT`, `FAILED_KIND`, normalized values)
- add local-ref resolution helpers inside the existing scripts instead of
  introducing a generic framework
- align shell and PowerShell status names, codes, and normalization behavior

### 8.2 Validation Scripts

Planned changes:

- update `scripts/validate-canon-skills.sh` and
  `scripts/validate-canon-skills.ps1` to assert:
  - `canon-pr-review` uses `--ref` in preflight, not `--input`
  - runnable skills describe incremental correction and preserved valid inputs
  - file-path skills keep path-oriented guidance
  - run-id-oriented skills keep exact run-id retry guidance

### 8.3 Shared Components Explicitly Unchanged

- `render-support-state.*`
- support-state taxonomy
- modeled-only skill behavior
- `render-next-steps.*` except for incidental wording cleanup if validation
  finds mismatch

## 9. Skill-by-Skill Patch Plan

### 9.1 Direct Patch Scope

| Skill | Planned Changes |
| --- | --- |
| `canon-pr-review` | Switch preflight to `--ref` inputs; define explicit `base-ref` and `head-ref` slots; describe local-only ref acceptance; preserve valid side across retries; show exact normalized CLI pair in retry guidance; distinguish `invalid-ref` from `missing-file` |
| `canon-brownfield` | Clarify typed slots for `owner`, `risk`, `zone`, and bounded brief path; preserve valid ownership fields across retries; distinguish missing file path from missing ownership metadata |
| `canon-requirements` | Clarify typed slots for `owner`, `risk`, `zone`, and input path; preserve valid fields; make retry prompts incremental and path-specific; keep Canon authority for actual run start |

### 9.2 Opportunistic Reuse

| Skill | Reuse Scope |
| --- | --- |
| `canon-status` | Shared `RunIdInput` validation and exact retry rendering |
| `canon-inspect-invocations` | Shared `RunIdInput` validation and exact retry rendering |
| `canon-inspect-evidence` | Shared `RunIdInput` validation and exact retry rendering |
| `canon-inspect-artifacts` | Shared `RunIdInput` validation and exact retry rendering |
| `canon-resume` | Shared `RunIdInput` validation plus clear preflight-vs-runtime failure distinction |
| `canon-approve` | Shared `RunIdInput` validation; preserve `TARGET`, `BY`, `DECISION`, and `RATIONALE` within one interaction without adding a new shared field taxonomy in this patch |

### 9.3 Explicitly Unchanged Skills

- `canon-init`
- all modeled-only skills
- `canon-verification`
- overall support-state inventory and discoverability rules

## 10. Validation and Test Strategy

### 10.1 Structural Validation

- extend `scripts/validate-canon-skills.sh`
- extend `scripts/validate-canon-skills.ps1`
- confirm shell and PowerShell validators enforce the same skill contract
- verify no persisted docs introduce absolute local paths

### 10.2 Shared-Helper Validation Matrix

The shared preflight scripts must be exercised directly for both shell and
PowerShell with:

- missing `zone` after valid `owner` and `risk`
- missing run id
- invalid risk token
- invalid zone token
- missing file path for `canon-requirements` or `canon-brownfield`
- `canon-pr-review` with `--ref master --ref HEAD`
- `canon-pr-review` with `--ref main --ref HEAD` in a repo that only has local
  `master`
- remote-like ref input such as `origin/main`
- pair-level failure where base and head normalize to the same ref

### 10.3 Runnable Walkthrough Validation

Run walkthroughs must prove:

- valid inputs survive a retry after only one corrected field
- `canon-pr-review` never classifies `master`, `main`, or `HEAD` as file-path
  failures
- `canon-pr-review` only starts Canon after ref normalization succeeds
- `canon-requirements` and `canon-brownfield` only start Canon after typed
  ownership and file-path checks succeed
- run-id-oriented skills ask only for the missing run id when that is the only
  missing field

### 10.4 Independent Validation

Independent review must compare:

- preflight retry guidance against the real `canon run`, `canon status`,
  `canon inspect`, `canon approve`, and `canon resume` contracts
- ref normalization behavior against the current Canon runtime expectation that
  `pr-review` consumes two `--input` values and does not itself interpret file
  paths for that pair
- shell and PowerShell parity for status codes, normalized outputs, and failure
  classes

### 10.5 Evidence Artifacts

Validation evidence will be recorded in [validation-report.md](./validation-report.md)
and must cover:

- progressive missing-input collection
- preserving valid inputs across retry
- `canon-pr-review` with `base master, head HEAD`
- `canon-pr-review` with `base main, head HEAD` in a repo whose usable local
  branch is `master`
- invalid ref versus missing ref
- missing path versus missing ref
- retry after correcting only one field
- proof that semantically valid refs are never treated as file paths
- proof that Canon execution starts only after typed preflight succeeds

## 11. Risks and Complexity Tracking

### Delivery Risks

- **Shell/PowerShell drift**: the same typed contract must be implemented twice.
  Mitigation: define status names and normalization behavior in one planned
  contract and validate both shells against the same matrix.
- **Over-abstracting the shared layer**: a broad helper framework would violate
  scope. Mitigation: keep one public preflight entrypoint and add only
  bounded helper functions inside it.
- **Surprising ref normalization**: silent aliasing between `main` and
  `master` would feel magical. Mitigation: local-only acceptance, no silent
  substitution, explicit suggestions only.
- **Skill prose drifting from helper behavior**: Codex-facing instructions can
  become stale. Mitigation: extend validators to enforce the new preflight and
  retry language where the patch applies.

### Complexity Tracking

No constitution deviations are required. The plan deliberately rejects simpler
but unsafe alternatives:

| Simpler Alternative | Rejected Because |
| --- | --- |
| Keep using repeated `--input` for `canon-pr-review` refs | Continues the current ref/file ambiguity |
| Accept remote refs by heuristic | Introduces ambiguity and hidden command-intent changes |
| Build a generic conversational form engine | Broadens scope beyond a corrective patch |
| Persist preserved values across turns or skills | Makes the frontend appear smarter than its trust boundary allows |

## 12. Open Technical Questions That Remain

No open question blocks implementation. The critical design choices for remote
refs, resolution order, and interaction-memory scope are closed by this plan.

Non-blocking follow-up questions for later increments:

- Should a future patch type-check `canon-approve` fields such as `TARGET` and
  `DECISION` in the shared layer, or should that remain skill-local?
- Should future documentation tighten `canon-requirements` language from
  “input file or note” to “existing repo-visible input artifact” everywhere for
  consistency with typed file-path preflight?

## Project Structure

### Documentation (this feature)

```text
specs/004-ref-safe-binding/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── pr-review-ref-binding-contract.md
│   └── runnable-skill-input-contract.md
└── tasks.md
```

### Source Code and Skill Layout (repository root)

```text
.agents/
└── skills/
    ├── canon-pr-review/
    │   └── SKILL.md
    ├── canon-brownfield/
    │   └── SKILL.md
    ├── canon-requirements/
    │   └── SKILL.md
    ├── canon-status/
    │   └── SKILL.md
    ├── canon-inspect-invocations/
    │   └── SKILL.md
    ├── canon-inspect-evidence/
    │   └── SKILL.md
    ├── canon-inspect-artifacts/
    │   └── SKILL.md
    ├── canon-approve/
    │   └── SKILL.md
    ├── canon-resume/
    │   └── SKILL.md
    └── canon-shared/
        └── scripts/
            ├── check-runtime.sh
            └── check-runtime.ps1

scripts/
├── validate-canon-skills.sh
└── validate-canon-skills.ps1
```

**Structure Decision**: keep the patch entirely inside the existing skills
frontend layer and feature-doc directory. No new runtime package, storage
surface, or plugin boundary is introduced.
