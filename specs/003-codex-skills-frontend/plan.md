# Implementation Plan: Codex Skills Frontend for Canon

**Branch**: `003-codex-skills-frontend` | **Date**: 2026-03-28 | **Spec**: [specs/003-codex-skills-frontend/spec.md](./spec.md)  
**Input**: Feature specification from `specs/003-codex-skills-frontend/spec.md`

## Summary

This increment adds a repo-local Codex skills frontend on top of the existing
Canon CLI. The implementation is intentionally additive: Canon stays the
governed execution engine and system of record, while `.agents/skills`
provides the discoverable `$`-invoked workflow surface. The plan delivers:

- explicit mode and operational skills with deterministic Canon command
  contracts
- a shared support layer for preflight checks, support-state messaging, and
  next-step formatting
- an honest visibility policy that keeps the full Canon taxonomy discoverable
  while making support state explicit
- validation that proves supported skills drive real Canon runs and modeled-only
  skills never fabricate them

## Governance Context

**Execution Mode**: `architecture` for an additive UX and workflow layer on top
of the accepted Canon runtime  
**Risk Classification**: `SystemicImpact` because this increment changes the
primary user entry point into Canon across all mode semantics, even though it
does not redesign the runtime itself  
**Scope In**: repo-local skill layout under `.agents/skills`, skill contracts
for all Canon modes and core operations, support-state policy, deterministic
helper scripts and references, runtime preflight and failure handling,
discoverable skill rollout for the full Canon taxonomy, support-state wrappers
for modeled-only modes, and validation for supported versus modeled behavior  
**Scope Out**: Canon core runtime redesign, plugin packaging or marketplace
work, MCP runtime enablement, generic skill framework behavior, freeform
chat-only wrappers, and any attempt to make unsupported modes look runnable

**Invariants**:

- Canon CLI remains the execution engine and source of truth for runs,
  approvals, traces, evidence, artifacts, and decision memory.
- Skills may only drive Canon commands or honest support-state checks; they may
  not simulate Canon runs in chat.
- Supported skills must return users to Canon runtime state and `.canon/`
  evidence rather than hiding it.
- Modeled-only and intentionally limited skills must remain visibly honest
  about current support state.
- Shared helpers may reduce duplication, but they must not become a generic
  skill runtime or a second orchestration engine.

**Decision Log**: [specs/003-codex-skills-frontend/decision-log.md](./decision-log.md)  
**Validation Ownership**: skill contracts and helper scripts generate Codex UX
behavior; Canon CLI executes runtime behavior; structural validators,
walkthroughs, overlap reviews, and independent skill-boundary review validate
the frontend against the runtime rather than trusting skill text alone  
**Approval Gates**: no new Canon runtime approval type is introduced; human
review is required before changing the declared support state or message
contract of any Canon skill

## 1. Technical Context

**Language/Version**: Markdown `SKILL.md` files, repo-local shell helpers
(`.sh` and `.ps1`), existing Canon Rust 1.94.0 CLI/runtime  
**Primary Dependencies**: installed `canon` binary, existing repo-local
`AGENTS.md`, `.agents/skills`, `README.md`, Git working directory semantics,
and built-in Codex skill discovery via `$`  
**Storage**: repo-local files under `.agents/skills` plus existing `.canon/`
runtime persistence; support-state and compatibility references stored as
deterministic repo files, not remote config  
**Testing**: deterministic structural validation of skill tree and metadata,
walkthrough-based validation for supported skills, explicit failure-path
validation for missing CLI and wrong repo context, and existing Canon CLI test
suite as the backend confidence layer  
**Target Platform**: Codex on macOS, Linux, and Windows with shell-specific
helpers where needed; no requirement for plugin packaging in this tranche  
**Project Type**: repo-local UX layer on top of a local-first CLI runtime  
**Existing System Touchpoints**: `README.md`, `AGENTS.md`, `.agents/skills`,
existing Canon CLI commands (`init`, `run`, `status`, `inspect`, `approve`,
`resume`), `.canon/` runtime records, and current delivered modes
(`requirements`, `brownfield-change`, `pr-review`)  
**Performance Goals**: preflight and support-state skill responses should
complete in under one second excluding Canon CLI execution time; supported
skills should add minimal wrapper latency beyond the underlying Canon command  
**Constraints**: no plugin platform work, no generic skill runtime, no hidden
state outside repo files, no bypass of Canon approvals or evidence, no hard
dependency on new external tools, and no assumption that all modes are equally
runnable  
**Scale/Scope**: 19 named Canon skills plus one shared deterministic support
area; all 19 are discoverable in phase 1, with 10 executable wrappers and 9
support-state wrappers

## 2. Constitution Check

### Pre-Design Gate

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] High-risk approval checkpoints are named
- [x] No constitution deviations are required before design

### Post-Design Re-Check

- [x] Canon runtime authority remains intact
- [x] Skills are wrappers and support-state surfaces, not a second runtime
- [x] Visibility policy preserves honest support-state signaling
- [x] Mode semantics stay explicit and are not collapsed into a `canon`
  super-skill
- [x] Failure handling remains deterministic and actionable
- [x] Validation covers supported workflows, modeled-only honesty, and overlap
  boundaries

**Result**: PASS. No constitution violations or justified exceptions are
required by this design.

## 3. Current Baseline and UX Gap

Canon already has:

- a serious local-first CLI
- governed execution and durable evidence for `requirements`,
  `brownfield-change`, and `pr-review`
- operational commands for `init`, `status`, `inspect`, `approve`, and
  `resume`
- mode semantics, gates, approvals, and `.canon/` persistence that make the
  runtime trustworthy

The current gap is UX, not engine capability:

- users must remember CLI commands and runtime details
- there is no first-class `$`-discoverable workflow surface in Codex
- inspection and unblock flows are available but not naturally discoverable
- modeled-only modes have a defined taxonomy in Canon but no honest Codex
  frontend policy yet

This increment closes that UX gap without changing Canon's product center of
gravity.

## 4. Skill Visibility Policy

### Visibility Model

Codex repo-local skills are filesystem-discovered. In phase 1, the plan
therefore assumes that **every Canon skill is present and discoverable through
`$`**. Visibility policy no longer decides whether a Canon skill exists. It
decides how brutally explicit the skill must be about its current support
state.

| Support State | Discoverability | Required Messaging | Phase-1 Policy |
| --- | --- | --- | --- |
| `available-now` | Discoverable and prominent | Treat as a real workflow backed by Canon CLI | Highlight in README, quickstart, and skill next-step guidance |
| `modeled-only` | Discoverable | Lead with not-runnable warning, what Canon already knows about the mode, what is missing, and nearest runnable alternatives | Ship as a support-state wrapper and never start a run |
| `intentionally-limited` | Discoverable | Lead with explicit limitation warning, current boundary, and nearest usable alternative | Use for surfaces such as `canon-verification` that exist but are deliberately constrained |
| `experimental` | Discoverable | Lead with instability warning, current contract boundary, and non-guaranteed behavior | Reserve for future opt-in behavior, not core delivered Canon workflows |

### Visibility Decisions for This Increment

- **`available-now` and prominent**:
  `canon-init`, `canon-status`, `canon-inspect-invocations`,
  `canon-inspect-evidence`, `canon-inspect-artifacts`, `canon-approve`,
  `canon-resume`, `canon-requirements`, `canon-brownfield`,
  `canon-pr-review`
- **`modeled-only` and discoverable**:
  `canon-discovery`, `canon-greenfield`, `canon-architecture`,
  `canon-implementation`, `canon-refactor`, `canon-review`,
  `canon-incident`, `canon-migration`
- **`intentionally-limited` and discoverable**:
  `canon-verification`

No current Canon taxonomy skill uses the `experimental` state in phase 1, but
the contract preserves it for future opt-in surfaces.

### Practical Enforcement

Phase-1 visibility is enforced through:

- concise descriptions optimized for implicit discovery on `available-now`
  skills
- explicit support-state headers on every non-runnable skill
- a full skill inventory that keeps all Canon skills discoverable
- README and quickstart guidance that still emphasizes the runnable set first
  without hiding the rest of the taxonomy

## 5. Proposed `.agents/skills` Layout

### Documentation (this feature)

```text
specs/003-codex-skills-frontend/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── skill-contract.md
│   └── skill-state-and-failure-contract.md
└── tasks.md
```

### Source Code and Skill Layout (repository root)

```text
.agents/
└── skills/
    ├── canon-init/
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
    ├── canon-requirements/
    │   └── SKILL.md
    ├── canon-discovery/
    │   └── SKILL.md
    ├── canon-greenfield/
    │   └── SKILL.md
    ├── canon-brownfield/
    │   └── SKILL.md
    ├── canon-architecture/
    │   └── SKILL.md
    ├── canon-implementation/
    │   └── SKILL.md
    ├── canon-refactor/
    │   └── SKILL.md
    ├── canon-verification/
    │   └── SKILL.md
    ├── canon-review/
    │   └── SKILL.md
    ├── canon-pr-review/
    │   └── SKILL.md
    ├── canon-incident/
    │   └── SKILL.md
    ├── canon-migration/
    │   └── SKILL.md
    └── canon-shared/
        ├── references/
        │   ├── runtime-compatibility.toml
        │   ├── support-states.md
        │   ├── output-shapes.md
        │   └── skill-index.md
        └── scripts/
            ├── check-runtime.sh
            ├── check-runtime.ps1
            ├── render-support-state.sh
            ├── render-support-state.ps1
            ├── render-next-steps.sh
            └── render-next-steps.ps1
```

**Structure Decision**: keep the full Canon skill taxonomy explicit and
discoverable in the repo, use one folder per user-facing skill, and isolate
reusable deterministic glue under `canon-shared`. Do not introduce a generic
`run-skill` executor.

## 6. Skill Contract Template

Each `SKILL.md` must contain, at minimum:

- `name`
- `description`
- `support state`
- `default visibility`
- `purpose`
- `when to trigger`
- `when it must not trigger`
- `required inputs`
- `preflight profile`
- `Canon command contract`
- `expected output shape`
- `failure handling guidance`
- `next-step guidance`
- `related skills`

### Executable Wrapper Skill

Used for delivered workflows and operational commands.

- Drives a real Canon CLI command
- Runs shared preflight checks first
- Returns Canon-backed state and next steps
- Never substitutes chat output for a missing Canon result

Applies to:

- `canon-init`
- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`
- `canon-inspect-artifacts`
- `canon-approve`
- `canon-resume`
- `canon-requirements`
- `canon-brownfield`
- `canon-pr-review`

### Support-State Wrapper Skill

Used for modeled-only or intentionally limited skills.

- Performs preflight only when necessary to confirm runtime or repo context
- Never calls `canon run` for an unsupported mode
- Returns support-state messaging, nearest supported alternative, and no run id

Applies to:

- `canon-discovery`
- `canon-greenfield`
- `canon-architecture`
- `canon-implementation`
- `canon-refactor`
- `canon-review`
- `canon-incident`
- `canon-migration`
- `canon-verification`

## 7. Shared Support Script / Reference Strategy

### What Lives in Shared Scripts

- Canon CLI presence and version checks
- repo-context and `.canon/` initialization checks
- deterministic support-state response rendering
- deterministic next-step guidance for inspect, approve, and resume flows

### What Stays in `SKILL.md`

- workflow purpose
- trigger boundaries
- Canon mode or command binding
- required inputs
- support-state explanation
- skill-specific next-step guidance

### What Lives in Shared References

- compatibility contract for Canon version expectations
- support-state vocabulary and wording rules
- full skill index used by README, quickstart, and validation guidance
- canonical output examples for supported, gated, modeled-only, and failure
  responses

### Deliberate Exclusions

- no generic skill execution framework
- no YAML or plugin manifest layer in this increment
- no parser-heavy abstraction that duplicates Canon CLI output logic

The helper layer should be small, deterministic, and boring.

## 8. Runtime Dependency and Failure-Handling Strategy

### Preflight Contract

Every executable wrapper skill runs a deterministic preflight before invoking
Canon:

1. `canon` binary present
2. Canon version compatible with shared contract
3. current directory is the intended repository context
4. `.canon/` exists when the command depends on prior initialization
5. required inputs exist (`run id`, file path, refs, or owner/risk/zone fields)

### Shared Preflight Result Codes

| Code | Meaning | Action |
| --- | --- | --- |
| `0` | Ready | Proceed to Canon command |
| `10` | CLI missing | Print install guidance and stop |
| `11` | Version incompatible | Print detected vs expected version and stop |
| `12` | Wrong repo context | Tell the user to switch to the intended repo root and stop |
| `13` | Repo not initialized | Point to `canon-init` and stop |
| `14` | Missing required input | Identify the missing run id, file, or ref and stop |

### Failure Response Rules

| Failure Case | Detection | Deterministic Response |
| --- | --- | --- |
| `canon` CLI not installed | `command -v canon` / `Get-Command canon` fails | Show the supported install path from README and stop |
| Installed Canon version incompatible | Parse `canon --version` against `runtime-compatibility.toml` | Report detected version, expected range or pinned version, and the corrective step |
| `.canon/` not initialized | Missing `.canon/` for commands that require runtime state | State that the repo is not initialized and point to `canon-init` |
| Outside expected repo context | No intended repo root, wrong working directory, or missing project markers | State that the user is outside the target repo context and must switch directories before retrying |
| Required run id or input file missing | empty input or missing file/ref | Name the missing input and show the canonical retry form |

`canon-init` is the only executable wrapper skill that may run without existing
`.canon/`. It still performs CLI and repo-context checks.

## 9. Supported-State Strategy for Modeled-Only and Intentionally Limited Skills

### Support-State Wrapper Behavior

Modeled-only skills exist to make the taxonomy visible and future-compatible
without lying about runtime delivery.

Rules:

- do not call `canon run` for unsupported modes
- do not fabricate a run id
- do not imply that a mode is runnable because a skill exists
- do explain what Canon already knows about the mode
- do explain what is still missing before the mode becomes runnable
- do provide the nearest supported alternative when one is honest and useful

### Intentionally Limited Skill Behavior

`canon-verification` is special:

- it has a declared contract and folder
- it is marked `intentionally-limited`
- it remains discoverable through `$`
- if invoked, it returns that dedicated verification mode is not yet runnable
  end to end, explains the current limitation, and points to
  `canon-inspect-evidence` or `canon-pr-review` when that is the relevant
  current substitute

### Canon Command Usage for Support-State Skills

Support-state skills may use:

- shared references only
- `canon inspect modes` as an honest runtime-facing check when useful

They must not:

- start a fake run
- synthesize approval or evidence state
- route through a generic fallback assistant

## 10. Delivery Plan for High-Value Supported Skills

### Slice A: Shared Frontend Spine and Full Taxonomy Presence

Deliver first:

- `canon-shared/references/*`
- `canon-shared/scripts/check-runtime.*`
- `canon-shared/scripts/render-support-state.*`
- `canon-shared/scripts/render-next-steps.*`
- base `SKILL.md` template and conventions
- all Canon skill folders with initial support-state metadata
- `canon-init`

Why first: every other skill depends on deterministic preflight, output shape,
support-state wording, and immediate taxonomy discoverability.

### Slice B: First Runnable Workflow Set

Deliver next:

- `canon-requirements`
- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`

Why second: this proves the frontend can start a real Canon run and point the
user back to invocation and evidence surfaces without command memorization.

### Slice C: Unblock and Continue

Deliver next:

- `canon-approve`
- `canon-resume`
- `canon-inspect-artifacts`

Why third: supported workflows are incomplete in Codex if the user cannot
unblock gated runs or inspect the resulting artifact packet.

### Slice D: Deeper Delivered Modes

Deliver next:

- `canon-brownfield`
- `canon-pr-review`

Why fourth: both depend on the same preflight, status, evidence, approval, and
resume spine proven in earlier slices.

## 11. Delivery Plan for Modeled-Only / Support-State Skills

### Phase-1 Support-State Wrappers

Deliver initial versions together with Slice A so the full taxonomy is
discoverable, then refine wording and alternatives after the first runnable set
is stable:

- `canon-architecture`
- `canon-review`
- `canon-verification`
- `canon-discovery`
- `canon-greenfield`
- `canon-implementation`
- `canon-refactor`
- `canon-incident`
- `canon-migration`

### Ordering Policy

Refine first the support-state skills most likely to confuse users if they are
ambiguous:

1. `canon-architecture`
2. `canon-review`
3. `canon-verification`
4. remaining modeled-only skills

### Discoverability Rollout

- the full Canon skill taxonomy exists from the first delivery slice
- README and quickstart should lead with the runnable skills, but also point to
  the complete skill index
- `canon-verification` remains discoverable, but clearly labeled
  `intentionally-limited`

## 12. Validation and Test Strategy

### Structural Validation

- validate `.agents/skills` tree shape
- validate one folder per skill plus shared support area
- validate required metadata and sections in every `SKILL.md`
- validate that every defined Canon skill is discoverable and has the expected
  support-state label

### Metadata and Overlap Validation

- verify descriptions are narrow enough to avoid overlap between:
  - `review` vs `pr-review`
  - `brownfield` vs `refactor`
  - `requirements` vs `discovery`
- verify `modeled-only`, `intentionally-limited`, and `available-now` labels
  appear where required

### Runtime and Failure Validation

- deterministic checks for missing Canon CLI behavior
- deterministic checks for incompatible version behavior
- walkthrough checks for repo-not-initialized handling
- walkthrough checks for wrong-repo handling
- walkthrough checks for missing run id or input file handling

### Supported Workflow Validation

- end-to-end walkthrough for `canon-requirements`
- end-to-end walkthrough for `canon-brownfield`
- end-to-end walkthrough for `canon-pr-review`
- end-to-end walkthrough for `canon-inspect-evidence`
- end-to-end walkthrough for `canon-approve` plus `canon-resume`

### Modeled-Only Honesty Validation

- explicit validation for `canon-architecture`
- explicit validation for `canon-verification`
- explicit validation for `canon-review`
- confirm that none of the above emits a fake run id or fake Canon state
- confirm that each explains what Canon already knows, what is missing, and the
  nearest runnable alternative when useful

### Independent Validation

- separate review of the visibility policy against actual skill metadata
- separate review of skill output examples against Canon runtime state
- separate review that all Canon skills are discoverable and that non-runnable
  ones are labeled without pretending to execute

## 13. Risks and Complexity Tracking

| Risk / Complexity | Why It Matters | Mitigation |
| --- | --- | --- |
| All Canon skills are discoverable | Users may see more skills than are runnable today | Use brutally explicit support-state labels and clear runnable alternatives |
| Wrapper boilerplate could sprawl | Many skills share the same checks and guidance | Keep a small shared helper layer and a strict contract template |
| Version drift between skills and Canon CLI | Skills may promise behavior the installed binary does not support | Maintain `runtime-compatibility.toml` and preflight version checks |
| Support-state wrappers may feel noisy or useless | Users may perceive them as dead commands | Explain what Canon knows, what is missing, and where to go next |
| Overlap between adjacent skills | Codex may select the wrong skill implicitly | Validate descriptions and must-not-trigger boundaries as first-class artifacts |

No constitution violations require exception handling. Complexity is managed
through explicit contracts and staged rollout.

## 14. Open Technical Questions That Remain

- How aggressively should available-now skills be highlighted relative to the
  fully discoverable modeled-only set?
- Should support-state wrappers always call `canon inspect modes`, or is a
  repo-local reference sufficient for phase 1?
- Should the skill index live only in docs, or also in a shared reference file
  consumed by future tooling?
- Is a separate `canon-inspect-artifacts` promotion step necessary, or should
  it always ship with the first promoted workflow slice?
- When the repo-local skill set stabilizes, what is the cleanest path to plugin
  packaging without changing folder names or contract shape?
