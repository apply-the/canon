# Feature Specification: Runnable Skill Interaction and Ref-Safe Input Binding

**Feature Branch**: `004-ref-safe-binding`  
**Created**: 2026-03-29  
**Status**: Draft  
**Input**: User description: "Create a focused corrective patch specification for the current Canon skills layer so runnable skills collect inputs reliably, bind to Canon commands correctly, and restore trust, with `canon-pr-review` as the proving case."

## Governance Context *(mandatory)*

**Mode**: brownfield  
**Risk Classification**: High because this patch changes how delivered
executable skills collect and normalize user inputs before invoking Canon, and a
mistake here can start the wrong governed workflow or make a runnable skill
look dishonest even when Canon itself is correct.  
**Scope In**: runnable skill interaction quality, typed input handling,
incremental collection of missing inputs, ref-safe preflight and retry guidance
for executable skills, and correction of command-binding behavior for the
current Codex skills frontend.  
**Scope Out**: Canon core runtime redesign, support-state taxonomy redesign,
full skill inventory changes, plugin packaging, MCP runtime work, generic
conversational form infrastructure, and any change that lets skills guess
inputs and accidentally start the wrong Canon command.

**Invariants**:

- Canon CLI remains the only execution engine and system of record for runs,
  approvals, evidence, artifacts, and status.
- Skills remain thin workflow frontends and must not become a second runtime or
  a chat-like freeform assistant layer.
- Modeled-only skills remain honest and must not fabricate runs, run ids, or
  runtime-backed summaries.
- Canon execution must not start until typed preflight has accepted all
  required inputs for the chosen runnable skill.

**Decision Traceability**: decisions for this patch must be recorded in the
feature decision log, while validation evidence for shell and PowerShell
interaction behavior must be recorded in the feature validation report.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Complete a Runnable Skill Without Re-entering Everything (Priority: P1)

A Codex user starts a runnable Canon skill, provides some valid inputs, misses
one required field, and wants the skill to ask only for what is still missing
instead of forcing a brittle one-line retry blob.

**Why this priority**: trust breaks fastest when a runnable skill feels harder
to use than the Canon command it fronts.

**Independent Test**: invoke a runnable skill with one missing field, confirm
the skill preserves valid prior inputs, asks only for the missing field, and
starts Canon only after the corrected typed preflight passes.

**Acceptance Scenarios**:

1. **Given** a user provides `owner` and `risk` but omits `zone`, **When**
   the runnable skill evaluates preflight, **Then** it asks only for `zone`
   and preserves the already valid `owner` and `risk`.
2. **Given** a user corrects one missing field after a failed preflight,
   **When** the skill retries, **Then** it does not ask for all fields again
   and does not discard valid previously collected inputs.

---

### User Story 2 - Start `canon-pr-review` With Semantically Valid Refs (Priority: P1)

A reviewer wants to run `canon-pr-review` using branch-style inputs such as
`main`, `master`, `HEAD`, or `refs/heads/main` without the frontend confusing
those refs for file paths.

**Why this priority**: `canon-pr-review` is the clearest proving case for trust
repair because its inputs are semantically valid even when they are not file
paths, and current friction makes a delivered mode feel modeled-only.

**Independent Test**: invoke `canon-pr-review` with `base master, head HEAD`
and with `base main, head HEAD` in a repository whose default branch is
actually `master`, then confirm the frontend distinguishes refs from paths,
normalizes them correctly, and binds to the Canon CLI contract without false
retry guidance.

**Acceptance Scenarios**:

1. **Given** a repo where the intended base is `master`, **When** the user
   provides `main` as a semantic base ref, **Then** the skill either
   normalizes it to the canonical accepted ref or reports a deterministic
   invalid-ref message rather than treating it as a missing file path.
2. **Given** the user provides `HEAD` as the head input, **When** preflight
   runs, **Then** the skill treats it as a ref candidate, not a file path, and
   renders retry guidance in the same form the Canon command binding accepts.

---

### User Story 3 - Trust Failure Messaging Across Runnable Skills (Priority: P2)

A user needs a clear explanation of whether a failure happened before Canon
execution or inside Canon execution, and what exact retry form is valid.

**Why this priority**: deterministic failures are only trustworthy if they are
specific, actionable, and aligned with the true runtime binding.

**Independent Test**: trigger failures for missing input, invalid ref, missing
file, wrong repo context, repo not initialized, and incompatible CLI; confirm
each message states the failure class, the interaction boundary, and the exact
retry guidance that the current binding will accept.

**Acceptance Scenarios**:

1. **Given** a user supplies a missing file path, **When** preflight fails,
   **Then** the skill reports a missing-file style failure and does not reuse
   ref-specific retry guidance.
2. **Given** Canon rejects a command after preflight has passed, **When** the
   skill reports the result, **Then** it states that the failure occurred
   inside Canon execution rather than before Canon execution.

### Edge Cases

- What happens when only one field in an otherwise valid runnable-skill request
  is missing?
- How should `canon-pr-review` behave when a semantic branch name is plausible
  but does not resolve cleanly in the current repo context?
- Which invariant is most stressed when retry guidance suggests a form that the
  current Canon command binding still rejects?

## 1. Product Delta

This patch improves runnable skill interaction quality and command-binding
correctness for the existing Codex skills frontend.

It does not add new Canon modes. It does not change Canon's governance model.
It does not reduce Canon CLI authority. The delta is a trust-repair correction:
runnable skills should feel reliable, Codex-native, and operationally aligned
with the real Canon command contract.

## 2. Problem Statement

The current product failure is operational rather than architectural:

- executable skills may ask for multiple inputs in a brittle one-line freeform
  format
- users can provide semantically correct values that still fail because the
  frontend normalizes them poorly
- refs and file-path inputs are not always separated cleanly
- retry guidance can suggest a form that the runtime binding still rejects
- a runnable skill that fails this way looks dishonest even when the
  underlying Canon mode exists and works

The most visible example is `canon-pr-review`, where semantically valid ref
inputs can be mishandled in a way that makes a delivered mode look less real
than it is.

## 3. Goals

- provide guided collection of missing inputs for executable skills
- handle runnable-skill inputs by typed input kind rather than by a flat
  freeform blob
- normalize ref inputs canonically for `canon-pr-review`
- make retry guidance deterministic and aligned with the real Canon command
  contract
- separate preflight validation for refs versus file paths
- strengthen trust in runnable skills without changing Canon CLI authority

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Executable skills MUST identify exactly which required inputs are
  missing before Canon execution starts.
- **FR-002**: Executable skills MUST be allowed to collect missing inputs
  incrementally instead of requiring a single one-line parameter blob.
- **FR-003**: Executable skills MUST preserve already valid inputs across a
  retry and MUST NOT ask for all fields again when only one field is missing or
  invalid.
- **FR-004**: Executable skills MUST validate and normalize inputs according to
  typed input classes before building Canon retry guidance or command previews.
- **FR-005**: `canon-pr-review` MUST treat base and head inputs as ref-typed
  inputs first and MUST NOT misclassify semantically valid refs as file paths.
- **FR-006**: `canon-pr-review` MUST accept `HEAD`, short local branch names
  that resolve in repo context, and `refs/heads/*`, and MUST map those forms
  to the Canon CLI contract in a deterministic, repo-aware, non-magical way.
- **FR-006a**: Support for remote refs is optional at specification level, but
  the implementation plan MUST explicitly decide whether remote refs are
  accepted and, if so, define the exact resolution order before implementation
  begins.
- **FR-007**: Runnable-skill retry guidance MUST render the exact input form
  accepted by the current Canon binding, not a merely plausible semantic form.
- **FR-008**: Preflight MUST distinguish missing input, invalid ref, missing
  file, wrong repo context, repo not initialized, and Canon CLI missing or
  incompatible as separate failure classes.
- **FR-009**: Failure messages for executable skills MUST state whether failure
  happened before Canon execution or inside Canon execution.
- **FR-010**: Skills MUST remain thin frontends and MUST NOT guess unprovided
  critical inputs in a way that could start the wrong Canon command.
- **FR-011**: Shared interaction rules MAY be reused by approval, resume, and
  inspect skills where they improve input preservation and retry safety, but
  the patch MUST NOT rewrite modeled-only skill behavior beyond shared rules.
- **FR-012**: Repo-aware guidance MAY suggest obvious canonical values when the
  repo context makes them clear, but the frontend MUST still require explicit
  user confirmation before using a value that changes command intent.

### Key Entities *(include if feature involves data)*

- **RunnableSkillRequest**: a frontend request for a delivered executable skill
  that may contain complete, partial, or invalid typed inputs before Canon
  execution begins.
- **TypedInputSlot**: a named input requirement attached to a runnable skill,
  including its input class, current value status, and retry rendering form.
- **PreflightOutcome**: a deterministic result stating whether the request is
  ready, missing input, invalid, blocked by environment, or rejected before
  Canon execution.
- **RetryGuidance**: the precise frontend response that tells the user what to
  correct next and what canonical input form the current binding accepts.
- **RefBindingContext**: the repo-aware context used to interpret semantic
  base/head ref values without treating them as path inputs.

## 4. Input Interaction Model

Runnable skills collect missing inputs incrementally.

- A runnable skill MUST identify exactly which inputs are still missing or
  invalid.
- It MUST preserve already valid inputs across the same interaction.
- It MAY suggest canonical values when repo context makes them obvious, but it
  MUST not silently assume them.
- If only one field is missing, it MUST ask only for that field.
- If one field is corrected, it MUST retry with preserved valid inputs rather
  than resetting the whole request.

### Interaction Memory Boundary

- Preservation of valid inputs is limited to the current interaction flow for
  the active runnable skill.
- The frontend MUST NOT persist typed input memory into Canon runtime state,
  repo state, or a generic cross-skill conversational memory layer.
- The frontend MUST NOT make later runnable-skill decisions based on stale
  inputs captured outside the active interaction without asking the user to
  confirm them again.

### Interaction Model by Input Family

- **Owner / Risk / Zone**:
  collect each field separately when missing; preserve provided valid values;
  render retry guidance in semantic form first and in Canon CLI form second.
- **Run id**:
  ask only for the missing run id when required by status, inspect, approve, or
  resume flows; preserve any valid ancillary fields.
- **File path**:
  ask for one missing file path at a time; confirm existence relative to repo
  context before retry.
- **Base ref / Head ref**:
  treat the pair as a coordinated typed input; preserve the valid side if only
  one ref is missing or invalid.

## 5. Typed Input Classes

### OwnerField

- **What it is**: the explicit human ownership value required for
  run-starting executable skills.
- **Validation**: non-empty, non-whitespace, interaction-safe string.
- **Normalization**: trim outer whitespace; preserve user-intended content.
- **Retry rendering**: render as `owner <VALUE>` in semantic guidance and
  `--owner <VALUE>` in Canon CLI form.

### RiskField

- **What it is**: the Canon risk value required for run-starting executable
  skills.
- **Validation**: must match the Canon runtime contract for accepted risk
  values.
- **Normalization**: trim and canonicalize to the runtime-recognized token.
- **Retry rendering**: render with the canonical runtime token, not a synonym.

### ZoneField

- **What it is**: the Canon usage-zone value required for run-starting
  executable skills.
- **Validation**: must match the Canon runtime contract for accepted zone
  values.
- **Normalization**: trim and canonicalize to the runtime-recognized token.
- **Retry rendering**: render with the canonical runtime token, not an
  approximate label.

### RunIdInput

- **What it is**: the Canon run identifier used by status, inspect, approve,
  and resume flows.
- **Validation**: non-empty and structurally consistent with Canon run id
  expectations, then validated against repo runtime state.
- **Normalization**: trim only; never rewrite a run id into another value.
- **Retry rendering**: render as the exact run id field required by the target
  skill.

### FilePathInput

- **What it is**: a repo-scoped or explicit filesystem input expected by a
  runnable skill.
- **Validation**: path existence check relative to repo root or explicit path
  location.
- **Normalization**: preserve repo-relative intent; canonicalize only enough to
  remove avoidable ambiguity.
- **Retry rendering**: render as a path input and never as a ref.

### RefInput

- **What it is**: a Git ref input such as `HEAD`, `main`, `master`,
  `refs/heads/*`, or an allowed remote ref form.
- **Validation**: resolve against repo ref context, not filesystem existence.
- **Normalization**: convert accepted semantic forms to the canonical form that
  the current binding accepts.
- **Retry rendering**: render as the exact accepted ref token, not as a path.

### RefPairInput

- **What it is**: the ordered base/head ref pair required by `canon-pr-review`.
- **Validation**: both sides present, distinct where required, and individually
  valid as refs in the current repo context.
- **Normalization**: preserve side identity while mapping each side to the
  canonical accepted ref form.
- **Retry rendering**: show the exact base/head pair the Canon command binding
  accepts.

## 6. Ref-Safe Binding for PR Review

`canon-pr-review` is the proving case for this patch.

### Accepted User Forms

- short local branch names such as `main` or `master`, when they resolve in
  the current repo context
- `HEAD`
- fully qualified local refs such as `refs/heads/main`
- remote refs only if the implementation plan explicitly opts in and defines a
  deterministic resolution order

### Binding Rules

- semantically valid refs MUST never be treated as file paths by mistake
- base/head inputs MUST be classified as `RefPairInput` before any file-path
  validation runs
- branch names MAY be normalized from a semantic short form to the canonical
  form accepted by the current Canon binding
- local ref handling MUST be the default baseline; remote ref handling MUST be
  an explicit plan decision rather than an implicit side effect of branch
  discovery
- retry guidance MUST use the exact form the runtime binding accepts

### Repo-Aware Guidance

- repo-aware branch discovery MAY improve guidance by identifying obvious local
  or tracked remote refs
- this discovery MUST remain advisory, not magical
- if the repo suggests a likely canonical branch, the skill may suggest it, but
  it MUST not silently substitute it without explicit user confirmation

## 7. Preflight Typing and Failure Semantics

Preflight must distinguish these classes cleanly:

- **Missing input**: a required typed input is absent
- **Invalid ref**: a ref-shaped input is present but does not validate as a
  ref in the current repo context
- **Missing file**: a file-path input is expected but does not exist
- **Wrong repo context**: the current location is not the intended git repo
- **Repo not initialized**: `.canon/` is missing where the workflow requires it
- **Canon CLI missing or incompatible**: Canon cannot satisfy the current
  frontend binding contract

Failure messages for executable skills must be:

- deterministic
- specific
- actionable
- aligned with the real command binding

They must also state whether failure happened before Canon execution or inside
Canon execution.

## 8. Runnable Skill UX Rules

- Do not force unnatural one-line parameter blobs when progressive prompting is
  more reliable.
- Do not discard already provided valid inputs.
- Do not suggest a retry command that will fail for the same normalization
  reason.
- Always report whether the failure happened before Canon execution or inside
  Canon execution.
- Preserve Canon CLI authority while still giving a good frontend experience.
- Prefer semantic prompts for humans, but ensure retry guidance resolves to the
  exact Canon CLI contract the runtime binding accepts.

## 9. Affected Skills

Directly affected:

- `canon-pr-review`
- `canon-brownfield`
- `canon-requirements`

Conditionally affected through shared typed-input improvements:

- `canon-approve`
- `canon-resume`
- `canon-inspect-invocations`
- `canon-inspect-evidence`
- `canon-inspect-artifacts`
- `canon-status`

Modeled-only skills are not redefined by this patch except where shared
interaction rules improve consistency without changing their support-state
policy.

## 10. Acceptance Criteria

- Executable skills can request missing inputs incrementally.
- Provided valid inputs are preserved across retries.
- `canon-pr-review` accepts and normalizes base/head refs correctly.
- Semantically valid refs are never misclassified as file paths.
- Failure guidance for refs is distinct from failure guidance for file paths.
- Retry guidance matches the actual Canon CLI contract.
- Runnable skills no longer look modeled-only because of avoidable interaction
  failures.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In validation walkthroughs, 100% of runnable-skill retries after
  a single missing field correction require the user to re-enter only the field
  that was missing or invalid.
- **SC-002**: In validation scenarios for `canon-pr-review`, 100% of
  semantically valid ref inputs are classified as refs rather than file paths.
- **SC-003**: In deterministic failure validation, 100% of retry guidance for
  executable skills matches the actual Canon binding form accepted by the
  current runtime contract.
- **SC-004**: Runnable-skill validation shows no case where a delivered Canon
  workflow appears unsupported solely because the frontend mishandled typed
  inputs.

## 11. Validation Cases

- `canon-pr-review` with `base master, head HEAD`
- `canon-pr-review` with `base main, head HEAD` in a repo whose default branch
  is actually `master`
- invalid ref versus missing ref
- missing input path versus missing ref
- progressive collection of `owner`, `risk`, and `zone`
- retry after a single corrected field without re-entering all fields
- confirmation that Canon execution starts only after typed preflight passes
- confirmation that failure messages distinguish preflight failures from
  Canon-execution failures

## Validation Plan *(mandatory)*

- **Structural validation**: shell and PowerShell validator coverage for typed
  interaction contracts, retry guidance, and runnable-skill metadata.
- **Logical validation**: runnable walkthroughs for `canon-requirements`,
  `canon-brownfield`, and especially `canon-pr-review`, including missing-field,
  ref-normalization, and retry-preservation scenarios.
- **Independent validation**: targeted review of typed-input classification,
  ref-safe binding, and retry guidance parity against the real Canon command
  contract.
- **Evidence artifacts**: feature validation report, walkthrough records, and
  command-contract evidence linked from the patch increment.

## Decision Log *(mandatory)*

- **D-001**: Treat this patch as trust repair for delivered executable skills,
  not as a runtime redesign. **Rationale**: the core failure is interaction
  quality and command binding correctness in the existing skills layer.

## Non-Goals

- Generic interactive form framework
- Changes to modeled-only skill behavior except where shared interaction rules
  apply
- Changes to Canon runtime mode semantics
- Broad UX rewriting for all support-state skills
- Provider or AI execution changes
- Plugin packaging
- MCP runtime work

## 12. Open Questions

- Beyond the mandatory local baseline (`HEAD`, short local branches,
  `refs/heads/*`), should branch discovery and acceptance extend to tracked
  remote refs too?
- If remote refs are accepted, what exact resolution order should the
  implementation use when both local and remote candidates appear plausible?
- Should canonical retry guidance show semantic form, full Canon CLI form, or
  both in a consistent order?
- How much typed input memory should a skill keep across one interaction before
  it should ask the user to confirm preserved values?
- Should some executable skills define preferred defaults for risk and zone, or
  should all such values remain explicitly user-confirmed?

## Assumptions

- The current Canon runtime contract for runnable skills remains unchanged by
  this patch.
- The current skills inventory remains intact and this patch only improves the
  interaction quality of the existing frontend.
- Repo context is available for executable skills that need ref or path
  validation.
- Users benefit more from incremental, typed correction than from being asked
  to restate an entire runnable-skill payload after every preflight failure.
