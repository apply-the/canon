# Feature Specification: Codex Skills Frontend for Canon

**Feature Branch**: `003-codex-skills-frontend`  
**Created**: 2026-03-28  
**Status**: Draft  
**Input**: User description: "Create the next product specification for Canon: Codex Skills Frontend for Canon."

## Governance Context *(mandatory)*

**Mode**: architecture  
**Risk Classification**: High (maps to `SystemicImpact` in Canon runtime
terms) because this increment changes how users discover and invoke Canon
workflows across the whole product surface, even though the governing runtime
itself remains intact.  
**Scope In**: a Codex-native skills frontend for all Canon workflows, repo-local
skill layout under `.agents/skills`, support-state signaling for delivered
versus modeled modes, operational skills for inspection and approvals, and a
clear contract between Codex skills and Canon CLI.  
**Scope Out**: redesign of the Canon engine, plugin marketplace work, remote or
distributed execution, MCP runtime enablement, generic prompt-pack behavior,
and any attempt to replace Canon CLI or `.canon/` persistence with skills.

**Invariants**:

- Canon CLI remains the execution engine and system of record for runs,
  approvals, traces, evidence bundles, artifacts, and decisions.
- No skill may bypass Canon's mode, risk, zone, gate, approval, or evidence
  requirements.
- Skills for modeled-only modes must not pretend those workflows are already
  runnable end to end.
- Skill output must lead users back to Canon runtime state, not away from it.
- Standard user-facing skill output must not expose internal run-state TOML
  files under `.canon/runs/`; readable file pointers belong under
  `.canon/artifacts/`.

**Decision Traceability**: decisions for this increment must be recorded in the
feature decision log and linked to the resulting `.agents/skills` layout,
operator guidance, and any sample governed runs produced during validation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Start a Supported Canon Workflow from Codex (Priority: P1)

A developer working in Codex wants to start a real Canon workflow with `$`
discovery instead of remembering raw CLI commands.

**Why this priority**: Canon is already operationally real, but adoption is
held back by a UX layer that still assumes command memorization.

**Independent Test**: invoke a supported mode skill such as
`$canon-requirements`, confirm the skill gathers or confirms the required
inputs, drives Canon CLI, and returns the run id plus next inspection steps.

**Acceptance Scenarios**:

1. **Given** a repo with Canon installed and initialized, **When** a user
   invokes `$canon-requirements`, **Then** the skill starts a real Canon run
   through the CLI and returns the run id, current state, and evidence pointers.
2. **Given** a supported workflow that enters an approval-gated state,
   **When** the skill reports the result, **Then** it shows the approval target,
   next recommended skill or command, and the resume path.

---

### User Story 2 - Inspect and Unblock a Governed Run from Codex (Priority: P2)

A technical lead wants to inspect invocation evidence, approve a blocked step,
and resume a run without dropping into raw CLI discovery.

**Why this priority**: supported Canon workflows are not actually usable inside
Codex unless inspection, approval, and resume are equally discoverable.

**Independent Test**: invoke `$canon-inspect-evidence`, `$canon-approve`, and
`$canon-resume` against a gated run and confirm the skill output reflects real
Canon runtime state instead of simulated summaries.

**Acceptance Scenarios**:

1. **Given** a run with persisted invocation records, **When** a user invokes
   `$canon-inspect-invocations` or `$canon-inspect-evidence`, **Then** the skill
   surfaces concise results backed by Canon runtime files.
2. **Given** a run blocked on an invocation or gate, **When** a user invokes
   `$canon-approve`, **Then** the skill records the approval through Canon CLI
   and points the user to `$canon-resume`.

---

### User Story 3 - Avoid False UX Promises for Unsupported Modes (Priority: P3)

A developer browsing Canon skills in Codex wants to understand which workflows
are fully delivered now and which are only modeled, without being misled by a
friendly but inaccurate skill.

**Why this priority**: the frontend must improve discoverability without
creating a second layer of product drift.

**Independent Test**: invoke a modeled-only skill such as
`$canon-architecture` and confirm it explicitly reports the current support
state and nearest supported alternative instead of inventing a workflow.

**Acceptance Scenarios**:

1. **Given** a mode that is modeled but not runnable end to end, **When** its
   skill is invoked, **Then** the skill explicitly states that the workflow is
   modeled-only and does not fabricate a Canon run.
2. **Given** overlapping skills such as `$canon-review` and
   `$canon-pr-review`, **When** Codex selects or a user invokes a skill,
   **Then** the skill description and trigger boundaries make the intended use
   clear enough to prevent ambiguous routing.

### Edge Cases

- What happens when a user invokes a mode skill before `canon init` has
  materialized `.canon/` in the current repo?
- How should a skill respond when the underlying Canon command exists, but the
  mode is modeled-only and not yet runnable?
- Which invariant is most likely to be stressed when a skill tries to be
  helpful by summarizing a run from memory instead of reading Canon runtime
  state?

## 1. Product Delta

This increment adds a Codex-native skills frontend for Canon.

Canon already has the hard part: a governed execution engine, durable evidence,
mode semantics, approvals, and inspectable local persistence under `.canon/`.
What it lacks is a first-class Codex workflow surface that makes those
capabilities discoverable and low-friction for users who expect `$`-invoked
skills.

The delta is deliberate and narrow:

- Canon CLI remains the engine.
- Codex skills become the frontend and workflow layer.
- Skills wrap or orchestrate Canon behavior instead of reimplementing it.
- The result should feel native inside Codex without turning Canon into a
  generic customization framework.

This is a UX and workflow increment on top of the accepted runtime, not a
rewrite of Canon itself.

## 2. Problem Statement

Canon is operationally real but still too dependent on users remembering raw
CLI commands and runtime details.

That creates four operational problems:

- discoverability is weak for Codex-first users because Canon's workflows are
  still mostly hidden behind command memorization
- the product does not yet benefit from Codex's explicit skills UX, even though
  that is the natural entry point for repo-local workflows
- supported workflows are harder to adopt than they need to be, because users
  must remember mode names, approval commands, and inspection commands
- Canon risks feeling like a niche runtime for insiders instead of a usable
  developer workflow layer

The missing layer is not more engine behavior. The missing layer is a
mode-aware, bounded, discoverable Codex frontend that exposes Canon workflows
as skills without loosening Canon's governance contract.

## 3. Goals

- Make the main Canon workflows discoverable through `$` inside Codex.
- Provide one skill per meaningful Canon workflow instead of a single generic
  wrapper.
- Define explicit mode skills for all Canon modes, even when some remain
  modeled-only for now.
- Define operational skills for setup, status, inspection, approval, and
  resume.
- Keep distribution repo-local first under `.agents/skills`.
- Keep the skill set structurally ready for future plugin packaging without
  requiring plugin work now.
- Define an explicit visibility policy so discoverability improves without
  presenting unsupported skills as runnable workflows.
- Preserve Canon's governance semantics, evidence model, and runtime authority.
- Reduce user cognitive load without hiding state transitions or approvals.

## 4. Non-Goals

- Plugin marketplace implementation
- Broad plugin packaging and publishing flow
- Replacement of the Canon CLI
- Replacing `AGENTS.md` with skills
- Generic agent orchestration
- Broad provider abstraction
- Runtime MCP work
- Collapsing all user flows into a single chat-like assistant interface
- Freeform skill behavior that fabricates unsupported Canon workflows

## 5. Codex Model Assumptions

- Codex supports explicit skill invocation with `$`.
- Codex supports repo-scoped skills under `.agents/skills`.
- Skills are directories containing `SKILL.md` and optional deterministic
  support assets such as `scripts/`, `references/`, or `assets/`.
- Plugins are the later distribution unit, but repo-local skills are the right
  iteration surface for one repo.
- `AGENTS.md` remains the persistent repo-level guidance layer and complements,
  rather than replaces, skill descriptions.
- Skill descriptions materially affect skill selection quality, so naming and
  description discipline are product requirements, not documentation polish.

## 6. Skill Taxonomy

Canon must ship a complete, named skill surface. The taxonomy is intentionally
split into mode skills and operational skills so that Codex users can discover
specific workflows instead of one overloaded wrapper.

### Mode Skills

#### `canon-requirements`

- **Purpose**: start a governed `requirements` run from Codex.
- **Trigger**: use when the user needs bounded framing before design, planning,
  or code changes.
- **Must not trigger**: for repository-preserving change work, review, incident
  response, or implementation execution.
- **Required inputs**: problem statement or file, owner, risk, zone, optional
  excludes.
- **Canon commands it drives**: `canon init` if needed, then
  `canon run --mode requirements ...`.
- **Evidence surfaces**: run id, `inspect invocations`, `inspect evidence`,
  `inspect artifacts`.
- **Approval expectations**: must surface invocation-scoped approval when the
  run enters `AwaitingApproval`.
- **Support state**: fully supported now.

#### `canon-discovery`

- **Purpose**: represent discovery-oriented Canon work in Codex.
- **Trigger**: use when the user wants exploratory framing beyond strict
  requirements capture.
- **Must not trigger**: when the request can be satisfied by delivered
  `requirements`, `brownfield-change`, or `pr-review` flows.
- **Required inputs**: discovery objective, repo or domain boundary.
- **Canon commands it drives**: must use Canon status or mode inspection to
  confirm support state; must not fabricate a `discovery` run.
- **Evidence surfaces**: current support state, mode metadata, nearest
  supported alternative.
- **Approval expectations**: none until the underlying runtime exists.
- **Support state**: modeled-only.

#### `canon-system-shaping`

- **Purpose**: represent system-shaping delivery framing in Codex.
- **Trigger**: use when the user wants to start net-new system work.
- **Must not trigger**: for existing-repo changes or review flows.
- **Required inputs**: initiative description, expected scope boundary.
- **Canon commands it drives**: support-state inspection only until the mode is
  runnable.
- **Evidence surfaces**: mode support state and nearest supported alternative.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

#### `canon-brownfield`

- **Purpose**: start a governed `brownfield-change` run from Codex.
- **Trigger**: use when the user wants to modify a live codebase while
  preserving invariants.
- **Must not trigger**: for system-shaping ideation, PR review, or broad
  implementation execution.
- **Required inputs**: change brief, owner, risk, zone, optional repo scope.
- **Canon commands it drives**: `canon init` if needed, then
  `canon run --mode brownfield-change ...`.
- **Evidence surfaces**: run id, `inspect invocations`, `inspect evidence`,
  `inspect artifacts`, pending approval targets if any.
- **Approval expectations**: must surface invocation-scoped approvals and
  resume flow for systemic or red-zone requests.
- **Support state**: fully supported now.

#### `canon-architecture`

- **Purpose**: represent architecture-specific Canon work in Codex.
- **Trigger**: use when the user asks for structure, boundaries, tradeoffs, or
  architecture review outside a PR context.
- **Must not trigger**: for delivered requirements or brownfield workflows
  unless the user explicitly wants an unsupported architecture mode.
- **Required inputs**: architectural question, boundaries, context.
- **Canon commands it drives**: support-state inspection only.
- **Evidence surfaces**: explicit modeled-only status and nearest supported
  workflow.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

#### `canon-implementation`

- **Purpose**: represent implementation-mode Canon work in Codex.
- **Trigger**: use when the user wants a future direct implementation mode.
- **Must not trigger**: for current supported change-control or review flows.
- **Required inputs**: implementation objective, repo boundary, owner.
- **Canon commands it drives**: support-state inspection only.
- **Evidence surfaces**: explicit modeled-only status and recommended
  alternatives.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

#### `canon-refactor`

- **Purpose**: represent refactor-specific Canon work in Codex.
- **Trigger**: use when the user asks for structural cleanup without a
  user-visible change.
- **Must not trigger**: for brownfield changes where behavior and invariants are
  the primary concern and `canon-brownfield` is the better fit.
- **Required inputs**: refactor goal, invariants, impacted surface.
- **Canon commands it drives**: support-state inspection only.
- **Evidence surfaces**: modeled-only status and why `canon-brownfield` may be
  the practical substitute today.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

#### `canon-verification`

- **Purpose**: represent dedicated verification-mode Canon work in Codex.
- **Trigger**: use when the user wants a future standalone verification mode.
- **Must not trigger**: for current approval, evidence inspection, or
  `pr-review` flows.
- **Required inputs**: verification target and scope.
- **Canon commands it drives**: support-state inspection only; must not imply
  that `canon verify` is implemented.
- **Evidence surfaces**: modeled-only status and current inspection
  alternatives.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only and blocked by `verify` backlog.

#### `canon-review`

- **Purpose**: represent general review-mode Canon work in Codex.
- **Trigger**: use when the user wants future non-PR review flows.
- **Must not trigger**: for explicit pull request or diff review requests,
  which belong to `canon-pr-review`.
- **Required inputs**: review subject and scope.
- **Canon commands it drives**: support-state inspection only.
- **Evidence surfaces**: modeled-only status and routing to `canon-pr-review`
  when the actual need is diff review.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

#### `canon-pr-review`

- **Purpose**: start a governed `pr-review` run from Codex.
- **Trigger**: use when the user wants review against a real diff, branch
  range, or PR-like comparison.
- **Must not trigger**: for architecture review, requirements framing, or
  non-diff review discussion.
- **Required inputs**: base ref, head ref, owner, risk, zone.
- **Canon commands it drives**: `canon init` if needed, then
  `canon run --mode pr-review ...`.
- **Evidence surfaces**: run id, `inspect invocations`, `inspect evidence`,
  `inspect artifacts`, review-disposition approval guidance.
- **Approval expectations**: must surface gate-target approval when
  `review-disposition` is required.
- **Support state**: fully supported now.

#### `canon-incident`

- **Purpose**: represent incident-mode Canon work in Codex.
- **Trigger**: use when the user asks for incident or outage workflows.
- **Must not trigger**: for change planning, PR review, or requirements work.
- **Required inputs**: incident context and scope.
- **Canon commands it drives**: support-state inspection only.
- **Evidence surfaces**: modeled-only status and current nearest workflows.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

#### `canon-migration`

- **Purpose**: represent migration-mode Canon work in Codex.
- **Trigger**: use when the user asks for a future dedicated migration flow.
- **Must not trigger**: for current brownfield or refactor work that can use
  existing slices.
- **Required inputs**: migration objective, source and target boundaries.
- **Canon commands it drives**: support-state inspection only.
- **Evidence surfaces**: modeled-only status and nearest supported workflow.
- **Approval expectations**: none until runtime support exists.
- **Support state**: modeled-only.

### Operational Skills

#### `canon-init`

- **Purpose**: initialize Canon in the current repo.
- **Trigger**: use when `.canon/` is missing or the user is trying Canon for
  the first time in a repo.
- **Must not trigger**: when the user wants to start a run and the repo is
  already initialized, unless initialization is genuinely missing.
- **Required inputs**: current repo root.
- **Canon commands it drives**: `canon init`.
- **Evidence surfaces**: runtime layout summary and next recommended skill.
- **Approval expectations**: none.
- **Support state**: fully supported now.

#### `canon-status`

- **Purpose**: inspect the current state of a run.
- **Trigger**: use when the user asks what happened, what is blocked, or what
  remains to do for a run.
- **Must not trigger**: as a substitute for deeper evidence or invocation
  inspection when the user explicitly asks for underlying records.
- **Required inputs**: run id.
- **Canon commands it drives**: `canon status --run <RUN_ID>`.
- **Evidence surfaces**: run state, pending approvals, evidence summary, next
  step.
- **Approval expectations**: surfaces pending approval needs but does not
  approve.
- **Support state**: fully supported now.

#### `canon-inspect-invocations`

- **Purpose**: inspect request-level authorization and execution history.
- **Trigger**: use when the user wants to know what Canon allowed, denied, or
  gated.
- **Must not trigger**: when the user only wants a high-level run state.
- **Required inputs**: run id.
- **Canon commands it drives**: `canon inspect invocations --run <RUN_ID>`.
- **Evidence surfaces**: request ids, decisions, outcomes, approval state,
  linked evidence.
- **Approval expectations**: may point to `canon-approve` when a request is
  gated.
- **Support state**: fully supported now.

#### `canon-inspect-evidence`

- **Purpose**: inspect run-level evidence bundles and lineage.
- **Trigger**: use when the user wants generation versus validation paths,
  artifact provenance, or the rationale behind a gated run.
- **Must not trigger**: when the user only wants emitted artifact names.
- **Required inputs**: run id.
- **Canon commands it drives**: `canon inspect evidence --run <RUN_ID>`.
- **Evidence surfaces**: evidence summary, generation paths, validation paths,
  denied invocations, and readable artifact provenance.
- **Public output boundary**: standard user-facing output must not surface
  internal `.canon/runs/.../*.toml` paths, approval refs, or decision refs as
  readable file pointers.
- **Approval expectations**: may point to `canon-approve` or `canon-resume`
  when evidence indicates a gated run.
- **Support state**: fully supported now.

#### `canon-inspect-artifacts`

- **Purpose**: inspect emitted artifacts for a run.
- **Trigger**: use when the user wants the artifact bundle or file pointers.
- **Must not trigger**: when the user wants execution lineage or invocation
  history.
- **Required inputs**: run id.
- **Canon commands it drives**: `canon inspect artifacts --run <RUN_ID>`.
- **Evidence surfaces**: artifact names and readable artifact paths when
  available.
- **Public output boundary**: artifact inspection must not point users to
  internal `.canon/runs/...` TOML files as a follow-on readable surface.
- **Approval expectations**: none directly, though artifacts may imply a gated
  status.
- **Support state**: fully supported now.

#### `canon-approve`

- **Purpose**: record an explicit approval for a gate or invocation.
- **Trigger**: use when a run is blocked on `AwaitingApproval`.
- **Must not trigger**: when the user only wants to inspect a gated run or when
  no approval target exists.
- **Required inputs**: run id, target, approver, decision, rationale.
- **Canon commands it drives**: `canon approve --run ... --target ...`.
- **Evidence surfaces**: approval target, recorded decision, next resume step.
- **Approval expectations**: this is the approval surface; the skill must not
  invent approvals without explicit user intent.
- **Support state**: fully supported now.

#### `canon-resume`

- **Purpose**: continue a blocked or approval-gated run.
- **Trigger**: use after an approval is recorded or after blocked conditions
  have been corrected.
- **Must not trigger**: before a pending approval or correction has occurred.
- **Required inputs**: run id.
- **Canon commands it drives**: `canon resume --run <RUN_ID>`.
- **Evidence surfaces**: resumed run state, next inspection step, any new
  gated conditions.
- **Approval expectations**: must surface if approval is still missing.
- **Support state**: fully supported now.

### Optional Support Skills

This increment does not require optional support skills such as
`canon-skill-doctor`, `canon-run-summary`, or `canon-mode-router`.

Those may be considered later, but they must not replace the explicit mode and
operational skill taxonomy defined above. Phase 1 should prove the direct skill
surface first.

## 7. UX Principles

Canon skills in Codex should feel:

- direct rather than ceremonial
- discoverable through names that match real Canon workflows
- explicit about what is fully supported versus modeled-only
- concise in output, but never vague about state or next steps
- grounded in Canon evidence surfaces, not manifesto-like wrappers
- mode-specific rather than generic
- helpful without inventing unsupported behavior

The frontend should reduce cognitive load, not hide governance.
Discoverable does not mean universally visible.

## 8. Runtime Relationship

The contract is simple:

- skills are wrappers or workflow drivers around Canon CLI
- Canon remains the source of truth for runs, approvals, traces, evidence, and
  decisions
- skills invoke Canon commands instead of duplicating Canon logic
- skills may use deterministic helper scripts only when command assembly or
  output shaping is repetitive and mechanical
- any summary shown in Codex must come from Canon runtime state, inspectable
  artifacts, or deterministic transforms of Canon output

Skills may improve ergonomics. They must not become a second runtime.

Before invoking Canon CLI, a skill must check four runtime dependency
conditions:

- the `canon` binary is installed and callable
- the installed Canon version is compatible with the skill contract
- the current repository is inside the expected Canon repo context
- `.canon/` is initialized if the requested action requires runtime state

When any of these checks fail, the skill must stop cleanly and return
actionable guidance. It must not invent a run id, a fake summary, or a
best-effort chat substitute for Canon runtime behavior.

## 9. Repository Layout

Phase 1 should live entirely in the repo:

```text
.agents/
  skills/
    canon-init/
      SKILL.md
    canon-status/
      SKILL.md
    canon-inspect-invocations/
      SKILL.md
    canon-inspect-evidence/
      SKILL.md
    canon-inspect-artifacts/
      SKILL.md
    canon-approve/
      SKILL.md
    canon-resume/
      SKILL.md
    canon-requirements/
      SKILL.md
    canon-discovery/
      SKILL.md
    canon-system-shaping/
      SKILL.md
    canon-brownfield/
      SKILL.md
    canon-architecture/
      SKILL.md
    canon-implementation/
      SKILL.md
    canon-refactor/
      SKILL.md
    canon-verification/
      SKILL.md
    canon-review/
      SKILL.md
    canon-pr-review/
      SKILL.md
    canon-incident/
      SKILL.md
    canon-migration/
      SKILL.md
    canon-shared/
      references/
      scripts/
```

Repository rules:

- one folder per skill
- Canon skill names use the `canon-` prefix
- `AGENTS.md` provides repo-wide governance and runtime guidance
- each `SKILL.md` provides narrow workflow intent, trigger boundaries, and
  deterministic Canon command guidance
- any future plugin packaging should be able to reuse this structure with
  minimal churn

## 10. Skill Metadata and Trigger Quality

Skill naming and description quality are product requirements because Codex may
select skills implicitly.

Standards:

- names must be specific and mode-shaped, not generic
- descriptions must say what the skill does and what it does not do
- descriptions must identify the underlying Canon mode or operation
- descriptions must call out support-state boundaries when the mode is
  modeled-only

Boundary rules for ambiguous pairs:

- **review vs pr-review**: `canon-pr-review` is only for diff or branch review;
  `canon-review` is for future non-diff review work and must not hijack PR
  review requests.
- **brownfield vs refactor**: `canon-brownfield` is the delivered workflow for
  live-codebase change control; `canon-refactor` stays modeled-only until it
  has distinct runtime semantics.
- **requirements vs discovery**: `canon-requirements` handles bounded framing
  today; `canon-discovery` must not pretend exploratory discovery is already a
  runnable Canon mode.
- **architecture vs implementation**: architecture concerns structure and
  tradeoffs; implementation concerns executing a change. Both remain distinct
  even when modeled-only.
- **inspect skills vs mode skills**: inspect skills report runtime state;
  mode skills initiate or route workflow intent.

## 11. Supported State Matrix

### Skill Visibility Policy

Skill exposure in Codex must follow support state and UX policy, not just the
existence of a skill directory.

- **Available now**: shown prominently and safe to present as the default
  Canon skill surface.
- **Modeled but not runnable end to end**: hidden by default; may be exposed
  intentionally only when clearly marked as modeled-only and non-runnable.
- **Experimental**: hidden by default unless explicitly enabled for testing or
  operator iteration; must carry an instability warning.
- **Hidden**: not presented in normal discovery flows and not implied to end
  users.

The default policy for this increment is:

- show available-now skills prominently
- do not present modeled-only skills as if they are runnable
- allow modeled or planned skills only when intentionally exposed with explicit
  support-state labeling
- keep experimental or backlog-blocked skills hidden by default unless there is
  a specific UX reason to surface them

| Skill | Canon Surface | Support State | Default Visibility | Codex Behavior |
| --- | --- | --- | --- | --- |
| `canon-init` | `canon init` | Available now | Prominent | Initializes `.canon/` and points to the next workflow skill. |
| `canon-status` | `canon status` | Available now | Prominent | Reports run state, pending approvals, and next step. |
| `canon-inspect-invocations` | `canon inspect invocations` | Available now | Prominent | Shows request-level decisions and outcomes. |
| `canon-inspect-evidence` | `canon inspect evidence` | Available now | Prominent | Shows lineage plus readable artifact provenance without exposing internal run-state files as user-facing paths. |
| `canon-inspect-artifacts` | `canon inspect artifacts` | Available now | Prominent | Shows emitted readable artifacts without pointing users at internal run-state TOML files. |
| `canon-approve` | `canon approve` | Available now | Prominent | Records gate or invocation approval, then points to resume. |
| `canon-resume` | `canon resume` | Available now | Prominent | Continues a blocked or approved run. |
| `canon-requirements` | `run --mode requirements` | Available now | Prominent | Starts a real governed run and returns run id plus evidence pointers. |
| `canon-brownfield` | `run --mode brownfield-change` | Available now | Prominent | Starts a real governed brownfield run with approval guidance. |
| `canon-pr-review` | `run --mode pr-review` | Available now | Prominent | Starts a real governed review run against a diff or ref pair. |
| `canon-discovery` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and routes to nearest supported workflow. |
| `canon-system-shaping` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and routes to nearest supported workflow. |
| `canon-architecture` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and does not fabricate a run. |
| `canon-implementation` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and nearest supported workflow. |
| `canon-refactor` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and possible `canon-brownfield` substitute. |
| `canon-verification` | modeled mode only; `verify` backlog | Experimental | Hidden by default | States that dedicated verification mode is not runnable yet. |
| `canon-review` | modeled mode only | Modeled, not runnable end to end | Hidden by default | States non-PR review is not delivered and routes to `canon-pr-review` when appropriate. |
| `canon-incident` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and current limitations. |
| `canon-migration` | modeled mode only | Modeled, not runnable end to end | Hidden by default | Reports modeled-only state and current limitations. |

No skill in this matrix may present modeled-only work as delivered runtime
behavior.

## 12. Output and Inspection Model

Each skill should return a compact, operationally useful result.

For a supported run-starting skill:

- concise run summary
- run id
- current state (`Completed`, `Blocked`, `AwaitingApproval`, or equivalent)
- optional action chips that mirror the same Canon-backed next step when the
  host supports them
- next recommended skill or Canon command
- pointers to `inspect invocations`, `inspect evidence`, and
  `inspect artifacts`

For a gated run:

- explicit approval target
- clear statement that the run is gated
- optional `Inspect evidence` chip when inspection is the honest next move
- optional `Approve generation...` chip only when Canon returned a real
  approval target for the active run
- next step using `canon-approve`
- follow-up step using `canon-resume`

For inspection skills:

- concise summary of what was found
- run id
- the specific evidence surface inspected
- optional action chips that preserve the same run context, never inventing
  approval or resume eligibility
- pointer to related surfaces when helpful

For modeled-only skills:

- explicit statement that the underlying Canon mode is not fully implemented
- current support state
- nearest supported skill, if one exists
- no fake run id and no fabricated summary of work Canon did not actually do

### 12.1 Action-Chip Progressive Enhancement

Action chips are an optional frontend affordance layered on top of the same
Canon-backed text contract. They do not create a second execution model.

- Every chip must mirror an already-valid entry under `Possible Actions:` or
  `Recommended Next Step:`.
- Text fallback remains mandatory in every host, including hosts with no chip
  UI.
- The initial supported chip labels are:
  - `Approve generation...`
  - `Resume run`
  - `Inspect evidence`
- `Approve generation...` must never appear unless Canon already emitted a
  real `RUN_ID` and approval `TARGET` for the active run.
- Approval chips may prefill Canon-backed arguments such as `RUN_ID` and
  `TARGET`, but they must still collect any missing human decision fields such
  as `BY`, `DECISION`, and `RATIONALE`.
- `Resume run` must never appear unless Canon still allows continuation on the
  same run id.
- When a run is gated and no readable artifact packet exists yet, `Inspect
  evidence` is the preferred chip over approval-oriented actions.
- The frontend must not use `Proceed with generation` as a label because it
  hides the explicit approval decision Canon requires.

For runtime dependency failures:

- **CLI not installed**: tell the user Canon is missing and show the supported
  install path
- **CLI version incompatible**: report the detected version, the expected
  compatible range or pinned version, and the corrective upgrade or downgrade
  step
- **repo not initialized**: explain that `.canon/` is missing for the requested
  workflow and point to `canon-init`
- **outside expected repo context**: explain that the current directory is not
  the intended Canon repo context and tell the user to switch to the repo root
  or target repository before retrying

## 13. Distribution Strategy

The staged distribution strategy is:

- **Phase 1**: repo-local skills under `.agents/skills`
- **Phase 2**: optional plugin packaging once the skill set is stable and worth
  sharing beyond one repo
- **Phase 3**: optional curated marketplace later, only if real reuse justifies
  it

This increment is strictly **Phase 1**.

It should optimize for real workflow quality inside this repo, not for early
distribution complexity.

## 14. Acceptance Criteria

- Codex users can discover Canon workflows through `$`.
- The main Canon skills map cleanly to Canon CLI workflows.
- Skills preserve Canon governance semantics instead of bypassing them.
- Supported workflows are clearly distinguished from modeled-only workflows.
- Skill visibility follows explicit support-state policy instead of exposing
  every skill by default.
- Skills surface run ids, evidence pointers, and approval or resume next steps
  for supported runs.
- Optional action chips, when rendered by a host, preserve the same Canon
  governance semantics and do not replace the text contract.
- The repo can ship a coherent `.agents/skills` layout without requiring plugin
  packaging.
- `AGENTS.md` and skills complement each other without duplicated
  responsibility.
- Modeled-only skills never fabricate Canon runs or unsupported outcomes.
- Inspection and approval skills return information backed by Canon runtime
  state.
- Runtime dependency failures produce clear, actionable guidance instead of
  vague skill-level failures.
- Repo-local skill structure can be lifted into plugin packaging later without
  redesigning the taxonomy.

## 15. Success Criteria

### Measurable Outcomes

- **SC-001**: A developer can start any currently delivered Canon workflow from
  Codex with a named skill and without memorizing the underlying raw CLI
  command.
- **SC-002**: 100% of supported Canon skills return the run id and at least one
  concrete next inspection step.
- **SC-003**: 100% of approval-gated skill flows show an explicit approval
  target and a resume path.
- **SC-004**: 100% of modeled-only skills explicitly state that the workflow is
  not fully delivered and do not create false expectations of a runnable mode.
- **SC-005**: The three delivered Canon workflows become substantially easier
  to invoke and inspect inside Codex than via command memorization alone.

## 16. Open Questions

- Should modeled-only modes get visible skills immediately, or should some stay
  hidden until they are closer to runnable?
- Should any future helper skill route users to the right mode, or is that too
  close to a generic `canon` super-skill?
- How much summary formatting belongs in skills versus Canon CLI output itself?
- Should approval and inspection stay split into fine-grained skills, or should
  some be consolidated after usage data is available?
- When is the repo-local skill set stable enough to justify plugin packaging?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repo MUST define a repo-local Canon skill surface under
  `.agents/skills`, with one folder per skill.
- **FR-002**: Canon MUST define a complete named skill taxonomy and execution
  contract for every Canon mode and every core operational action required for
  setup, inspection, approval, and resume. Visible exposure in Codex MAY vary
  by support state and explicit UX visibility policy.
- **FR-003**: Skills for delivered Canon workflows MUST drive real Canon CLI
  commands rather than simulate a run in chat.
- **FR-004**: Skills for modeled-only modes MUST explicitly report that the
  workflow is not fully supported and MUST NOT fabricate a Canon run.
- **FR-005**: Skills MUST preserve Canon's explicit mode semantics and MUST NOT
  bypass risk, zone, gate, approval, evidence, or decision-memory requirements.
- **FR-006**: Supported mode skills MUST collect or confirm the minimum Canon
  inputs needed to run the workflow, including owner, risk, zone, and workflow
  inputs.
- **FR-007**: Supported mode skills MUST return a concise run summary, the run
  id, and at least one concrete next inspection step.
- **FR-008**: Inspection skills MUST surface information backed by Canon
  runtime state or inspectable artifacts, not freeform recollection.
- **FR-009**: Approval skills MUST require explicit user intent and MUST record
  approvals through Canon CLI targets rather than inferred or implicit actions.
- **FR-010**: Resume skills MUST continue an existing Canon run rather than
  starting a new one.
- **FR-011**: Skill metadata MUST make support state and trigger boundaries
  explicit enough to reduce ambiguous overlap between similar skills.
- **FR-012**: The skill set MUST clearly distinguish available-now, modeled but
  not runnable end-to-end, experimental, and hidden skills, and MUST apply an
  explicit visibility policy to those states.
- **FR-013**: Canon skill output MUST point users toward Canon inspection
  surfaces and `.canon/` evidence rather than treating emitted markdown as the
  complete product outcome.
- **FR-014**: Skills MUST detect and handle Canon runtime dependency failures
  for missing CLI installation, incompatible CLI version, missing `.canon/`
  initialization, and incorrect repository context with clear, actionable
  guidance.
- **FR-015**: The phase-1 skill layout MUST remain compatible with later plugin
  packaging without requiring a taxonomy redesign.
- **FR-016**: This increment MUST NOT introduce runtime MCP execution or broad
  plugin-packaging requirements.

### Key Entities *(include if feature involves data)*

- **Skill Definition**: a repo-local Codex skill with a name, description,
  trigger boundary, support state, and Canon command contract.
- **Skill Support State**: the declared delivery state of a skill, such as
  available now, modeled but not runnable end to end, experimental, or hidden.
- **Skill Execution Contract**: the mapping from skill invocation to Canon
  command behavior, expected inputs, and required output surfaces.
- **Skill Output Summary**: the concise Codex-visible result emitted after a
  Canon command or support-state check, including run id, state, and next
  actions.
- **Runtime Dependency Check**: the preflight evaluation that determines
  whether Canon CLI is installed, compatible, initialized, and being used in
  the correct repository context.

## Validation Plan *(mandatory)*

- **Structural validation**: validate the `.agents/skills` layout, naming
  conventions, skill metadata completeness, and command mapping against the
  accepted skill taxonomy.
- **Logical validation**: walk through supported flows for `canon-requirements`,
  `canon-brownfield`, `canon-pr-review`, and operational flows for inspection,
  approval, and resume; also test modeled-only skill behavior for at least one
  unsupported mode.
- **Independent validation**: perform a separate review focused on trigger
  overlap, support-state honesty, and whether skills ever bypass Canon runtime
  records.
- **Evidence artifacts**: record evidence in the feature package, plus any
  resulting Canon runs, `.canon/` records, and final skill inventory created
  during implementation.

## Decision Log *(mandatory)*

- **D-001**: Skills are the Codex frontend and Canon CLI remains the governed
  execution engine, **Rationale**: this preserves Canon's system of record and
  prevents the UX layer from becoming a shadow runtime.
- **D-002**: Modeled-only skills should exist as explicit support-state
  frontends instead of being omitted entirely, **Rationale**: visibility is
  useful as long as the frontend is honest and does not fake delivery.

## Non-Goals

- Replacing Canon CLI with a chat-native runtime
- Treating skills as a prompt pack detached from Canon runs
- Using a single generic `canon` super-skill instead of explicit workflows
- Expanding this increment into plugin packaging, marketplace work, or runtime
  MCP enablement

## Assumptions

- Codex users discover and invoke repo-local skills through `$` frequently
  enough that skill naming and descriptions materially affect adoption.
- Repo-local skills are the fastest safe way to iterate on Canon UX before any
  broader distribution work.
- Canon CLI output and `.canon/` runtime records are stable enough to serve as
  the backend contract for a Codex skills frontend.
- The first value of this increment comes from making the three delivered
  workflows easier to invoke and inspect, not from making every modeled mode
  runnable immediately.
