# Feature Specification: Canon Skill Runtime Contracts

**Feature Branch**: `061-skill-runtime-contracts`  
**Created**: 2026-05-28  
**Status**: Implemented  
**Input**: User description: "Canon Skill Runtime Contracts: structured preflight JSON, declarative preflight YAML, and lifecycle hooks with detect/propose semantics"

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact; the feature modifies shared skill
infrastructure and shell scripts but does not touch the Rust CLI, governance
semantics, or packet contracts. Regression risk is limited to skill invocation
reliability across AI hosts.  
**Scope In**: Shared preflight script output (JSON), declarative preflight YAML
in SKILL.md frontmatter, `.canon/hooks.toml` schema and detect/propose
consumption pattern, and updates to `canon-implementation`, `canon-change`, and
`canon-publish` skills.  
**Scope Out**: Rust CLI changes, CLI-enforced hook execution, plugin
marketplace, install/uninstall lifecycle, remote hooks, automatic mutation
hooks, new governance modes, new approval semantics.

**Invariants**:

- Existing skills with prose-only preflight MUST continue working unchanged
  until explicitly migrated.
- Canon governance semantics, approval state, and packet contracts MUST NOT be
  affected by this feature.
- Hook execution MUST always leave an inspectable trace; no opaque side-effects.
- The Rust codebase under `crates/` MUST NOT be modified by this feature.

**Decision Traceability**: `specs/061-skill-runtime-contracts/` and subsequent
Canon-governed run artifacts under `.canon/`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Preflight JSON Eliminates Repeated Prose Checks (Priority: P1)

A skill author or AI assistant invokes `canon-preflight.sh --mode implementation`
at the start of a governed skill flow and receives a single structured JSON
object with all environment state needed for the preflight phase, instead of
manually checking each prerequisite through separate commands.

**Why this priority**: This is the highest-value, lowest-risk change. It
directly reduces ~40% of repeated prose in every skill while making the AI's
first step deterministic and portable across hosts.

**Independent Test**: Run `canon-preflight.sh --mode implementation` in a repo
with `.canon/` initialized and a non-empty `canon-input/implementation.md`.
Verify JSON output contains all expected fields with correct values.

**Acceptance Scenarios**:

1. **Given** a repo with `canon` on PATH and `.canon/` initialized, **When**
   `canon-preflight.sh --mode implementation` is run, **Then** it outputs valid
   JSON with `canon.available: true`, `canon.initialized: true`,
   `input.canonical_input_exists: true`, and `workspace.git_branch` matching
   the current branch.
2. **Given** a repo without `canon` on PATH, **When** `canon-preflight.sh
   --mode change` is run, **Then** it outputs valid JSON with
   `canon.available: false` and all other fields populated with safe defaults.
3. **Given** a repo where `canon-input/change.md` does not exist, **When**
   `canon-preflight.sh --mode change` is run, **Then** it outputs
   `input.canonical_input_exists: false`.

---

### User Story 2 - Declarative Preflight Contract in Skill Frontmatter (Priority: P2)

A skill author declares the skill's environment requirements as a structured
`preflight:` block in the SKILL.md YAML frontmatter. The AI assistant parses
this declaration, compares it against the preflight JSON output, and surfaces
only the missing or invalid requirements to the user, instead of interpreting
30 lines of English-prose instructions.

**Why this priority**: Reduces skill authoring friction and makes skill
requirements machine-parseable, which improves portability across AI hosts
(Copilot, Codex, Claude).

**Independent Test**: Add a `preflight:` block to `canon-implementation`
SKILL.md. Invoke the skill in a repo missing `.canon/`. Verify the AI surfaces
exactly the missing requirement (`requires_initialized: true` fails) without
processing the full prose preflight section.

**Acceptance Scenarios**:

1. **Given** a SKILL.md with `preflight: { requires_canon: true,
   requires_initialized: true, canonical_input: implementation }`, **When** the
   AI parses it against preflight JSON showing `canon.initialized: false`,
   **Then** the AI reports only "Canon is not initialized" and suggests
   `$canon-init`.
2. **Given** a SKILL.md with `preflight: { risk_required: true,
   zone_required: true }`, **When** the user has not provided risk or zone,
   **Then** the AI asks for risk and zone with guided choices before proceeding.

---

### User Story 3 - Lifecycle Hooks with Detect/Propose (Priority: P3)

A repository maintainer authors `.canon/hooks.toml` to declare lifecycle
actions (e.g., `git add docs/` after publish). When a Canon skill reaches the
matching lifecycle point, it detects the hook, explains it to the user, and
proposes execution. The user or AI decides whether to run it. Every execution
is recorded.

**Why this priority**: Enables repo-local customization without modifying
skills or the CLI. Lower priority because it requires Phase A and B to be
stable first.

**Independent Test**: Create `.canon/hooks.toml` with an `after_publish` hook.
Run `canon-publish` skill flow. Verify the skill surfaces the hook as a
proposal with command, description, and required/optional status. Verify that
if executed, the trace is recorded.

**Acceptance Scenarios**:

1. **Given** `.canon/hooks.toml` with `[[hooks.after_publish.actions]]` having
   `optional = true`, **When** the publish skill reaches the after-publish
   lifecycle point, **Then** the skill outputs a proposal block showing the
   hook command, description, and "Required: false".
2. **Given** `.canon/hooks.toml` with `[[hooks.before_run.actions]]` having
   `optional = false`, **When** a Canon run skill begins, **Then** the skill
   outputs the hook as mandatory with a clear explanation of why.
3. **Given** no `.canon/hooks.toml` exists, **When** any Canon skill runs,
   **Then** no hook detection output appears and the skill proceeds normally.

---

### Edge Cases

- What happens when `canon-preflight.sh` is called with an invalid `--mode`?
  Returns JSON with `input.resolved_path: null`, `input.ambiguous: false`, and
  `input.error: "unknown mode: <value>"`.
- What happens when `.canon/hooks.toml` has invalid TOML syntax? The skill
  skips hook detection silently and logs a diagnostic; it does not block the
  governed flow.
- What happens when a mandatory hook command fails? The skill reports the
  failure, records the trace, and asks the user how to proceed (retry, skip
  with acknowledgement, or abort).
- What happens when the preflight JSON schema changes between Canon versions?
  The script emits a `schema_version` field; skills check version compatibility
  before relying on new fields.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a `canon-preflight.sh` script under
  `.agents/skills/canon-shared/scripts/` that accepts `--mode <MODE>` and
  outputs valid JSON to stdout. On partial failure (e.g., `canon` binary
  unavailable), the script MUST still emit a JSON envelope containing
  `schema_version` and populated fields for sections that succeeded, plus an
  `"error"` field in sections that failed.
- **FR-002**: The preflight JSON MUST include `canon` (available, version,
  initialized), `workspace` (path, git_branch, git_user), `input`
  (file_exists, file_path, file_empty, folder_exists, folder_path,
  folder_empty, resolved_path, ambiguous), and `runs` (active,
  pending_approvals) sections. The `input.resolved_path` field uses
  file-first precedence; `input.ambiguous` is `true` when both file and
  folder exist for the same mode.
- **FR-003**: System MUST provide a `canon-preflight.ps1` PowerShell equivalent
  with identical JSON output shape.
- **FR-004**: SKILL.md frontmatter MUST support an optional `preflight:` YAML
  block declaring requirements (requires_canon, requires_initialized,
  canonical_input, system_context, risk_required, zone_required,
  owner_optional).
- **FR-005**: System MUST support a `.canon/hooks.toml` file declaring
  lifecycle hooks with `before_` and `after_` prefixes for `run`, `publish`,
  `approve`, and `resume` events.
- **FR-006**: Each hook entry MUST include `id`, `command`, `description`, and
  `optional` fields. Optional fields: `prompt`, `condition`, `mode_filter`,
  `trusted`. The `trusted` field defaults to `false` when absent; untrusted
  hooks require an additional confirmation step beyond the standard proposal.
- **FR-007**: Hook detection MUST follow detect, explain, propose semantics;
  hooks MUST NOT auto-execute under any circumstance in V1. The AI MUST show
  full command, working directory, description, and expected effect before
  proposing execution. This is the primary V1 security boundary.
- **FR-008**: Every hook execution (whether proposed and accepted, or
  mandatory) MUST leave a trace in the `ai-provenance.md` sidecar alongside
  the run, recording hook id, command, proposal outcome, and execution
  result.
- **FR-009**: The preflight script MUST emit a `schema_version` field for
  forward compatibility.
- **FR-010**: The preflight script MUST complete in under 2 seconds on a
  typical repository with fewer than 50 active runs.
- **FR-011**: Skills updated with the declarative preflight contract MUST pass
  `scripts/validate-canon-skills.sh` without regression.

### Key Entities

- **Preflight Report**: JSON document emitted by `canon-preflight.sh`
  representing the full environment state for a skill invocation.
- **Preflight Contract**: YAML block in SKILL.md frontmatter declaring what
  a skill requires from the environment.
- **Lifecycle Hook**: TOML entry in `.canon/hooks.toml` declaring a command
  to be proposed or surfaced at a specific Canon lifecycle point.
- **Hook Trace**: Record of hook detection, proposal, and execution outcome.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Skills migrated to the preflight contract reduce their Preflight
  Profile prose section by at least 60% (measured in lines of text).
- **SC-002**: The preflight script produces valid JSON output in under 2
  seconds on the Canon repository itself.
- **SC-003**: All 3 target skills (`canon-implementation`, `canon-change`,
  `canon-publish`) pass `scripts/validate-canon-skills.sh` after migration.
- **SC-004**: AI assistants (Copilot, Codex) can parse the preflight JSON and
  declarative contract on first invocation without fallback to prose
  interpretation.
- **SC-005**: Hook detection adds no more than 200ms to skill startup when
  `.canon/hooks.toml` exists with up to 10 declared hooks.

## Validation Plan *(mandatory)*

- **Structural validation**: `scripts/validate-canon-skills.sh` confirms
  migrated skills retain valid frontmatter and required sections. JSON schema
  validation of preflight output with `jq`.
- **Logical validation**: Manual invocation of `canon-preflight.sh` under
  various conditions (missing canon, missing .canon/, missing input, active
  runs). Skill invocation tests in Copilot verifying correct error surfacing.
- **Independent validation**: Run migrated skills in both Copilot and Codex
  hosts to verify portable behavior. Adversarial test with malformed
  `.canon/hooks.toml` to confirm graceful degradation.
- **Evidence artifacts**: Test fixture outputs under
  `tests/fixtures/preflight/`, hook parsing test outputs under
  `tests/fixtures/hooks/`, and validation script pass/fail logs.

## Decision Log *(mandatory)*

- **D-001**: Use TOML for `.canon/hooks.toml` rather than YAML. **Rationale**:
  Canon's existing config surface (context.toml, run manifests) uses TOML;
  consistency reduces cognitive load and avoids a second parser dependency.
- **D-002**: Hooks are AI-executed (detect/propose) in V1, not CLI-enforced.
  **Rationale**: Avoids Rust changes, keeps the feature scoped to skill
  reliability, and allows incremental CLI enforcement in a future slice.
- **D-003**: JSON for preflight output, not key=value. **Rationale**: JSON is
  more reliably parsed by AI assistants across hosts; key=value requires
  ad-hoc parsing.
- **D-004**: Start with 3 skills only (`canon-implementation`, `canon-change`,
  `canon-publish`). **Rationale**: High-value, high-usage skills; validates the
  pattern before broad rollout.
- **D-005**: Hook traces land in `ai-provenance.md` sidecar. **Rationale**:
  Keeps hook evidence co-located with AI-generated lineage and avoids
  introducing new persistence surfaces.
- **D-006**: Preflight script uses partial-JSON on failure rather than
  hard-crashing. **Rationale**: Maximizes usable context in degraded
  environments.
- **D-007**: `input` section reports both file and folder, resolves file-first.
  **Rationale**: Skills get one normalized consumption point; ambiguity is
  flagged rather than silently masked.
- **D-008**: `preflight:` YAML block is authoritative over legacy prose.
  **Rationale**: Removes AI interpretation ambiguity; migration deprecation
  markers enable gradual cleanup.
- **D-009**: Hook `trusted` field with detect/propose as the V1 security
  boundary. **Rationale**: No auto-execution removes entire attack surface;
  `trusted` field prepares for future UX differentiation without weakening
  V1 guarantees.

## Non-Goals

- Modifying the Rust CLI codebase or adding new Canon subcommands.
- Building a plugin marketplace or extension install/uninstall lifecycle.
- Supporting remote hooks, webhook callbacks, or network-dependent actions.
- Automatic mutation hooks that modify the workspace without user visibility.
- Replacing Canon's existing approval gates or governance semantics.
- Migrating all 20+ skills in this feature; only 3 target skills are in scope.

## Assumptions

- Canon CLI binary is already stable enough that version checking in the
  preflight script can rely on `canon --version` output format.
- AI assistants (Copilot, Codex, Claude) can reliably parse YAML frontmatter
  blocks in SKILL.md files.
- The `.canon/` directory structure and `context.toml` format remain stable for
  the duration of this feature.

## Clarifications

Resolved during spec review on 2026-05-28.

### C-001: Hook Trace Storage Location

**Question**: Where does the hook execution trace land?

**Answer**: Hook traces are recorded in the `ai-provenance.md` sidecar
alongside the run. This keeps hook evidence co-located with the AI-generated
lineage and avoids introducing new persistence surfaces under `.canon/`.

**Impact**: FR-008 is satisfied by the existing `ai-provenance.md` sidecar
pattern; no new file schemas are required for Phase C.

---

### C-002: Preflight Script Failure Mode

**Question**: What happens when the preflight script itself fails internally?

**Answer**: The script outputs partial JSON with populated fields for sections
that succeeded, plus an `"error"` field for sections that failed. This gives
the AI maximum usable context even on degraded environments (e.g., `canon`
unavailable but git info still accessible). The script MUST NOT crash to
stderr only; it always emits at least a partial JSON envelope with
`schema_version` and section-level error diagnostics.

**Impact**: FR-001 and FR-009 are strengthened; the JSON contract is reliable
even under degraded conditions. Adds a per-section error reporting
requirement to the output shape.

---

### C-003: Folder-Backed Input Detection

**Question**: Does the preflight script check both file and folder input
conventions?

**Answer**: The `input` section reports both `file_exists` and `folder_exists`
with their respective paths and emptiness state. It also provides a resolved
`resolved_path` field using file-first precedence, so skills can consume one
normalized value without losing diagnostics. If both file and folder exist,
the script sets `input.ambiguous: true` while still resolving to the
file-backed input by convention.

**Impact**: FR-002 is refined; the `input` section shape now includes
`file_exists`, `file_path`, `file_empty`, `folder_exists`, `folder_path`,
`folder_empty`, `resolved_path`, and `ambiguous` fields instead of a single
`canonical_input_exists` boolean.

---

### C-004: Preflight Contract vs Prose Coexistence

**Question**: When a skill has both a `preflight:` YAML block and legacy prose
preflight instructions, which takes precedence?

**Answer**: The `preflight:` YAML block is authoritative whenever present. The
AI must use the declarative YAML contract for preflight decisions and must not
independently execute or reinterpret legacy prose preflight instructions.
During migration, legacy prose may remain in the skill body only as
human-readable context, but it must be marked with a deprecation comment:

```
<!-- DEPRECATED: preflight behavior is governed by the `preflight:` block.
     Do not use this prose as an execution contract. -->
```

If YAML and prose conflict, YAML wins. The validation script should eventually
warn when a skill contains both a `preflight:` YAML block and unmarked legacy
preflight prose.

**Impact**: FR-004 and FR-011 are strengthened. Migration strategy is
explicit: YAML-first, prose-as-context, validation warns on unmarked
coexistence.

---

### C-005: Hook Security Model

**Question**: How are hooks prevented from executing dangerous commands?

**Answer**: Detect/propose semantics ARE the V1 security boundary. Hooks must
never auto-execute. The assistant must always show the full command, working
directory, description, and expected effect before proposing execution.

Additionally, each hook entry supports a `trusted` field (`true`/`false`,
defaulting to `false`). Untrusted hooks require an explicit extra confirmation
step beyond the standard proposal. Trusted hooks still require full command
visibility and proposal; `trusted: true` must not mean silent execution in V1.
If a hook is missing the `trusted` field, treat it as `trusted: false`.

**Impact**: FR-006 is extended to include `trusted` as an optional field.
FR-007 is strengthened to make detect/propose the explicit security contract.
A future slice may add allowlisting or sandboxing, but V1 relies on
transparency and user consent.
- Shell script execution (bash/PowerShell) is available in all target AI host
  environments where Canon skills are consumed.
- The existing `check-runtime.sh` script can be extended or replaced without
  breaking current skill behavior during migration.
