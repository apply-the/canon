# Feature Specification: Run Identity, Display Id, and Authored-Input Refactor

**Feature Branch**: `009-run-id-display`  
**Created**: 2026-04-22  
**Status**: Draft  
**Input**: User description: "Refactor Canon run identity, run storage, authored input handling, and CLI lookup so Canon has a robust immutable machine identity, a human-readable display id, an optional human-readable slug/title, and a better day-to-day operator experience, while preserving governance, auditability, and backward compatibility where reasonable."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact — touches run identity, persistence layout, CLI lookup, and authored-input semantics across the engine and CLI, but does not redesign the governance model, the orchestrator, the skill system, or introduce new external dependencies. Existing repositories must keep working with read compatibility for legacy UUID-keyed run directories.

**Scope In**:

- Run manifest schema additions for `uuid`, `run_id`, `short_id`, `slug`, `title`, `created_at`.
- Deterministic display-id generation (`R-YYYYMMDD-SHORTID`, UTC).
- Slug derivation, sanitization rules, and optional fallback behavior.
- New on-disk run directory layout `.canon/runs/YYYY/MM/R-YYYYMMDD-SHORTID[--slug]/`.
- Strict separation of `canon-input/` (editable authoring surface) from `.canon/runs/<…>/inputs/` (immutable per-run snapshot).
- CLI run resolution by `run_id`, by unique `short_id`, by legacy `uuid`, and (where it fits cleanly) `@last`.
- Listing surface that surfaces human-readable identity columns.
- Updates to docs and help text to speak in `run_id` / `uuid` / `slug` terms and to be install-first / binary-first for daily use.
- Read compatibility for legacy UUID-keyed run directories.

**Scope Out**:

- Replacing UUID as canonical machine identity.
- Introducing a progressive numeric counter as canonical identity.
- Introducing a database or any new persistence engine.
- Redesigning the governance model, gates, zones, risk taxonomy, or skill system.
- Moving `.canon/` to a different top-level location or restructuring the rest of the runtime architecture.
- Bulk silent migration of existing run directories on disk.
- Product-level UX changes outside CLI lookup, listing, status, inspect, approve, and resume.

**Invariants**:

- UUID remains the immutable, canonical machine identity for every run; nothing derived from authored content or slugs may replace it.
- Authored files under `canon-input/` MUST NOT be mutated by runtime persistence; per-run snapshots under `.canon/runs/<…>/inputs/` MUST remain immutable evidence with preserved digests and provenance.
- Existing governance capabilities (`status`, `inspect evidence`, `inspect artifacts`, `approve`, `resume`, gate / verification / invocation persistence) MUST continue to function for both new and legacy runs.
- Slug and title are descriptive metadata only and MUST NOT be used for identity, uniqueness, or lookup correctness.
- Display-id parsing relies on the **first** `--` separator only; later `--` sequences belong to the slug payload.

**Decision Traceability**: `specs/009-run-id-display/decision-log.md` and the run manifest decision references; cross-linked into `.canon/` artifacts of the change run that implements this feature.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Operator reads and references runs by human-friendly id (Priority: P1)

An operator working day-to-day with Canon needs to identify, paste, and look up runs without juggling raw UUIDs. They see runs by `run_id` (e.g. `R-20260413-6f2b8d4e`), optionally enriched with a slug, and can resolve them on the CLI by full id, by unique short id, or by legacy UUID.

**Why this priority**: This is the primary daily friction the refactor is designed to remove. Without it, none of the downstream usability or documentation value lands. It is also a prerequisite for updating docs and skills to speak in `run_id` terms.

**Independent Test**: Create a new run, observe that the CLI prints a `R-YYYYMMDD-SHORTID` display id, copy that id into `canon status`, `canon inspect …`, `canon approve`, and `canon resume`, and confirm each command resolves the run. Repeat with the bare short id and with the underlying UUID.

**Acceptance Scenarios**:

1. **Given** a freshly initialized repo, **When** the operator starts a run, **Then** the CLI prints both the `run_id` and `uuid` and the run is created at `.canon/runs/YYYY/MM/R-YYYYMMDD-SHORTID[--slug]/`.
2. **Given** a run exists with `run_id = R-20260413-6f2b8d4e`, **When** the operator runs `canon status R-20260413-6f2b8d4e`, **Then** the run is resolved and its current state is shown.
3. **Given** the same run, **When** the operator runs `canon status 6f2b8d4e` and that short id is unique, **Then** the run is resolved.
4. **Given** two runs share the same short id, **When** the operator runs `canon status 6f2b8d4e`, **Then** the command fails clearly, lists the matching runs by `run_id`, and does not pick one silently.
5. **Given** a legacy run directory keyed by raw UUID, **When** the operator runs `canon status <uuid>`, **Then** the run is resolved using read compatibility.

---

### User Story 2 - Authored inputs and immutable run snapshots stay separate (Priority: P1)

A user authors mode briefs in `canon-input/<mode>.md` (or `canon-input/<mode>/`). When they start a run, Canon snapshots those files into the run's `inputs/` directory and never edits the authoring surface afterward. Evidence and provenance for the snapshot are preserved.

**Why this priority**: Confusing authored inputs with run snapshots is the most likely way governance and auditability silently break during this refactor. Locking the contract is required for safe execution of every other story.

**Independent Test**: Author a brief in `canon-input/requirements.md`, start a requirements run, modify the authored file after the run starts, and confirm the run's `inputs/requirements.md` still matches the original digest and the authored file remains exactly as the user left it.

**Acceptance Scenarios**:

1. **Given** an authored file at `canon-input/requirements.md`, **When** a run is created from it, **Then** an immutable snapshot is written into `.canon/runs/<…>/inputs/requirements.md` with preserved digest and provenance.
2. **Given** a run has started, **When** the authored file is edited or deleted on disk, **Then** the snapshot under the run's `inputs/` directory is unchanged and the run continues to load successfully.
3. **Given** a mode that supports a directory form (`canon-input/<mode>/`), **When** a run is created, **Then** the directory tree is snapshotted with the same immutability and provenance rules.
4. **Given** runtime persistence runs (gate updates, invocation records), **When** persistence completes, **Then** no file under `canon-input/` has been modified.

---

### User Story 3 - Operator browses runs chronologically and lists them (Priority: P2)

An operator wants to see what runs exist, in roughly chronological order, with enough metadata (`run_id`, mode, slug/title, `created_at`, state) to choose one without opening individual manifests.

**Why this priority**: Listing is high-leverage but not strictly required to keep existing flows working. It builds on stories 1 and 2.

**Independent Test**: Create several runs across at least two months, run the listing command, and confirm output shows `run_id`, mode, slug/title, `created_at`, and current state, with chronological ordering aligned to the `YYYY/MM` filesystem layout.

**Acceptance Scenarios**:

1. **Given** several runs exist, **When** the operator invokes the listing command, **Then** rows are printed with at least `run_id`, `mode`, `slug`-or-`title`, `created_at`, and `state`.
2. **Given** the user opens the filesystem, **When** they navigate `.canon/runs/`, **Then** runs are grouped under `YYYY/MM/` and named with their display id and optional slug.

---

### User Story 4 - `@last` shortcut for the most recent run (Priority: P3)

When the existing CLI resolution layer can host it cleanly, the operator can refer to the most recent run as `@last` for inspect / status / approve / resume.

**Why this priority**: Pure ergonomic sugar, intentionally optional and gated on a clean fit with the existing resolution layer.

**Independent Test**: After creating a run, invoke `canon status @last` and confirm it resolves to the run with the latest `created_at`.

**Acceptance Scenarios**:

1. **Given** at least one run exists, **When** the operator runs `canon status @last`, **Then** the most recently created run (by stored `created_at`) is resolved.
2. **Given** no runs exist, **When** the operator runs `canon status @last`, **Then** the command fails clearly.

### Edge Cases

- Slug derivation produces an empty string after sanitization → directory name uses `run_id` only, no trailing `--`.
- Slug payload contains `--` itself → directory parser still treats only the first `--` as the run-id / slug boundary.
- Two runs created within the same UTC second produce the same `YYYYMMDD` and conflicting short ids → the full UUID still disambiguates; short-id collisions are surfaced as ambiguous lookups.
- Authored input file is deleted after run creation → run's snapshot continues to load; status, inspect, approve, resume still work.
- Legacy UUID-keyed run directory exists alongside new layout → both are discoverable; lookup by UUID still succeeds.
- Filesystem is case-insensitive (macOS default) → slug is lowercased; ASCII-only rule prevents accidental case collisions.
- A non-UTC machine clock drifts → display-id date portion is still derived from UTC `created_at`, not local time.
- An operator passes a short id that matches a substring of multiple full UUIDs → ambiguity is reported, no guess is made.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST generate a fresh UUIDv7 (already used by the engine) for every new run and store it as the canonical machine identity in the run manifest.
- **FR-002**: System MUST derive a stable `short_id` from the UUID (first 8 hex characters of the canonical UUID string) and store it on the manifest.
- **FR-003**: System MUST compute `created_at` in UTC and persist it in canonical RFC 3339 form.
- **FR-004**: System MUST build the display `run_id` as `R-YYYYMMDD-SHORTID` using the UTC date portion of `created_at`.
- **FR-005**: System MUST derive an optional `slug` from authored input, mode summary, or first meaningful heading when one is reasonably available.
- **FR-006**: Slug derivation MUST sanitize to lowercase ASCII, replace whitespace with `-`, strip filesystem-hostile punctuation, collapse repeated separators, trim leading/trailing separators, and bound length (≤ 60 characters).
- **FR-007**: Slug MUST be optional and MUST NOT be required for uniqueness; absence of a slug MUST NOT block run creation.
- **FR-008**: System MUST persist `slug` and an optional `title` as descriptive metadata only, never as identity.
- **FR-009**: System MUST place each run on disk at `.canon/runs/YYYY/MM/R-YYYYMMDD-SHORTID[--slug]/`, where `YYYY/MM` are the UTC year/month from `created_at`.
- **FR-010**: When parsing a run directory name, the system MUST treat only the **first** `--` as the boundary between display id and slug payload.
- **FR-011**: System MUST snapshot authored input into the run's `inputs/` directory at run creation, preserving content digests and provenance, and MUST NOT mutate `canon-input/` thereafter.
- **FR-012**: System MUST continue to support both `canon-input/<mode>.md` and `canon-input/<mode>/` authoring forms where they already exist.
- **FR-013**: System MUST resolve a run when given any of: full `run_id`, full `uuid`, or `short_id` if it uniquely identifies one run on disk.
- **FR-014**: When a `short_id` lookup is ambiguous, system MUST fail with a clear error and MUST list the matching runs by `run_id`; it MUST NOT pick one silently.
- **FR-015**: System MUST preserve read compatibility for legacy run directories keyed by raw UUID; lookups by UUID MUST continue to succeed.
- **FR-016**: System MUST keep `status`, `inspect evidence`, `inspect artifacts`, `approve`, and `resume` working for both new and legacy run layouts.
- **FR-017**: System MUST surface a listing capability (extension of the existing CLI surface) that shows at least `run_id`, mode, `slug`-or-`title`, `created_at`, and run state.
- **FR-018**: System SHOULD support `@last` resolution to the most recently created run when this fits cleanly into the existing CLI resolution layer; absence of this convenience MUST NOT block the rest of the change.
- **FR-019**: Documentation and CLI help text MUST refer to `run_id` for users, `uuid` as internal identity, and `slug`/`title` as descriptive metadata; daily-use examples MUST use the installed `canon` binary.
- **FR-020**: System MUST NOT introduce silent destructive moves of existing runs; any explicit migration step MUST be narrow, opt-in, and documented.

### Key Entities *(include if feature involves data)*

- **Run Manifest**: Persisted TOML record per run. Fields include `uuid` (canonical machine identity), `run_id` (display), `short_id` (derived), `slug` (optional metadata), `title` (optional metadata), `created_at` (UTC), `mode`, `owner`, `risk`, `zone`, plus existing governance fields. Source of truth for identity.
- **Run Directory**: On-disk container at `.canon/runs/YYYY/MM/R-YYYYMMDD-SHORTID[--slug]/` holding the manifest, `inputs/` snapshot, artifacts, evidence, decisions, approvals, and traces.
- **Authored Input Surface**: User-owned files under `canon-input/` (file or directory form per mode); editable, never mutated by runtime persistence.
- **Run Input Snapshot**: Immutable copy under `.canon/runs/<…>/inputs/`; carries digests and provenance for governance.
- **Run Lookup Index**: Resolution layer that maps any of `run_id`, `short_id`, or legacy `uuid` (and optionally `@last`) to a unique run directory; reports ambiguity instead of guessing.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of new runs created after the change land at `.canon/runs/YYYY/MM/R-YYYYMMDD-SHORTID[--slug]/` and include `uuid`, `run_id`, `short_id`, `created_at`, `mode`, `owner`, `risk`, and `zone` in their persisted manifest.
- **SC-002**: An operator can resolve any new run with `canon status`, `canon inspect …`, `canon approve`, and `canon resume` using the printed `run_id` without ever needing to copy the UUID, in 100% of cases observed during validation.
- **SC-003**: All authored files under `canon-input/` remain byte-identical (same digest) before and after a run is created, executed to its current stopping point, persisted, and re-loaded.
- **SC-004**: Snapshots under `.canon/runs/<…>/inputs/` produce stable digests across reload and across status / inspect / approve / resume invocations, validated by automated tests.
- **SC-005**: Lookup by `short_id` succeeds when unique and fails with a clear, listing-style error when ambiguous, in 100% of test cases.
- **SC-006**: Existing repositories with legacy UUID-keyed run directories continue to load via `canon status` and `canon inspect …` without manual intervention.
- **SC-007**: Daily-use documentation and CLI help text consistently refer to `run_id` for human use and `uuid` for internal identity, verified by a documentation pass and validator update.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, manifest-shape contract tests for the new fields, path-parser tests for the `--` boundary rule, and updated skill-validator scripts (`scripts/validate-canon-skills.sh`, `scripts/validate-canon-skills.ps1`).
- **Logical validation**: `cargo nextest run` covering run creation, persistence, reload, lookup by `run_id` / `short_id` / `uuid`, ambiguity handling, snapshot immutability, authored-input non-mutation, legacy UUID directory read compatibility, and (if implemented) `@last` resolution.
- **Independent validation**: A separate human reviewer (not the implementer) MUST confirm that authored-input non-mutation, legacy read compatibility, and CLI lookup ergonomics behave as specified, using the quickstart in this feature directory.
- **Evidence artifacts**: `specs/009-run-id-display/validation-report.md` referencing the test run output, coverage data, the manifest contract fixtures, and any quickstart transcript; cross-linked from the change run's evidence under `.canon/`.

## Decision Log *(mandatory)*

- **D-001**: Keep UUIDv7 (the engine already uses `Uuid::now_v7`) as canonical machine identity and derive `short_id` as the first 8 hex characters of the canonical lowercase UUID string. **Rationale**: Preserves current generation, leverages v7's embedded timestamp ordering, gives a stable short form, avoids introducing a new identity scheme.
- **D-002**: Use UTC for the date component of `run_id` and for `created_at` storage. **Rationale**: Eliminates timezone ambiguity in filesystem layout and identity strings; matches existing time-handling conventions.
- **D-003**: Treat slug and title as metadata only and place slug after a single `--` separator in the directory name. **Rationale**: Keeps display-id parsing trivial (`split_once("--")`) and prevents slug content (which may contain `--`) from corrupting identity parsing.
- **D-004**: Preserve read compatibility for legacy UUID-keyed run directories without forcing migration. **Rationale**: Avoids destructive moves on existing repositories and keeps the change low-risk for current users.
- **D-005**: Make `@last` optional and gated on a clean fit with the existing CLI resolution layer. **Rationale**: It is ergonomic sugar, not a correctness requirement; deferring or skipping it must not block the rest of the refactor.

## Non-Goals

- Replacing UUID, introducing a numeric counter, or otherwise altering canonical machine identity.
- Introducing a database, search index, or other new persistence engine.
- Redesigning the governance model, gates, zones, risk taxonomy, skill system, or orchestrator.
- Moving `.canon/` to a different top level or restructuring the broader runtime architecture.
- Bulk silent migration of existing on-disk runs.
- Product-level UX work outside run identity, lookup, listing, and the authored-input vs snapshot contract.

## Assumptions

- The existing run manifest is TOML and can accept additive fields without destabilizing existing readers.
- UUIDv7 generation is already used by the engine (`Uuid::now_v7`) and is acceptable as canonical machine identity.
- The current CLI command surface has a single resolution layer where `run_id` / `short_id` / `uuid` (and optionally `@last`) can be added without restructuring command parsing.
- A short id of the first 8 hex characters of UUIDv7 provides sufficient uniqueness within a single repository's run history for typical use; collisions are tolerated as long as they are reported clearly.
- Operators run Canon on systems where UTC is reliably available (standard for server, CI, and developer machines).
- Daily-use docs target operators using an installed `canon` binary; contributor docs may continue to mention `cargo run` for local engine development but should not lead daily examples.
- Existing repositories may contain legacy UUID-keyed run directories that must remain readable without explicit migration.
# Feature Specification: [FEATURE NAME]

**Feature Branch**: `[###-feature-name]`  
**Created**: [DATE]  
**Status**: Draft  
**Input**: User description: "$ARGUMENTS"

## Governance Context *(mandatory)*

**Mode**: [system-shaping, change, review, architecture, debugging, operations
or NEEDS CLARIFICATION]
**Risk Classification**: [low-impact | bounded-impact | systemic-impact with rationale]
**Scope In**: [What this feature is allowed to change]
**Scope Out**: [Explicitly excluded work and non-goals]

**Invariants**:

- [Constraint that MUST remain true]
- [Boundary that implementation MUST preserve]

**Decision Traceability**: [Where decisions for this feature will be recorded]

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - [Brief Title] (Priority: P1)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why this has the highest priority]

**Independent Test**: [Describe how this can be tested independently and what
value it delivers]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]
2. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

### User Story 2 - [Brief Title] (Priority: P2)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why this has this priority]

**Independent Test**: [Describe how this can be tested independently]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

### User Story 3 - [Brief Title] (Priority: P3)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why this has this priority]

**Independent Test**: [Describe how this can be tested independently]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
-->

- What happens when [boundary condition]?
- How does the system handle [error scenario]?
- Which invariant is most likely to be stressed by this case?

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST [specific capability, e.g., "allow users to create accounts"]
- **FR-002**: System MUST [specific capability, e.g., "validate email addresses"]
- **FR-003**: Users MUST be able to [key interaction, e.g., "reset their password"]
- **FR-004**: System MUST [data requirement, e.g., "persist user preferences"]
- **FR-005**: System MUST [behavior, e.g., "log all security events"]

*Example of marking unclear requirements:*

- **FR-006**: System MUST authenticate users via [NEEDS CLARIFICATION: auth method not specified - email/password, SSO, OAuth?]
- **FR-007**: System MUST retain user data for [NEEDS CLARIFICATION: retention period not specified]

### Key Entities *(include if feature involves data)*

- **[Entity 1]**: [What it represents, key attributes without implementation]
- **[Entity 2]**: [What it represents, relationships to other entities]

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: [Measurable metric, e.g., "Users can complete account creation in under 2 minutes"]
- **SC-002**: [Measurable metric, e.g., "System handles 1000 concurrent users without degradation"]
- **SC-003**: [User satisfaction metric, e.g., "90% of users successfully complete primary task on first attempt"]
- **SC-004**: [Business metric, e.g., "Reduce support tickets related to [X] by 50%"]

## Validation Plan *(mandatory)*

- **Structural validation**: [linting, schema checks, static analysis, contract
  validation, or N/A]
- **Logical validation**: [tests, user journey walkthroughs, simulations, or
  manual scenarios]
- **Independent validation**: [reviewer, separate model, or adversarial pass]
- **Evidence artifacts**: [where validation results and findings will be
  recorded]

## Decision Log *(mandatory)*

- **D-001**: [Initial decision or open question], **Rationale**: [Why this
  default is acceptable for now]

## Non-Goals

- [Explicitly excluded behavior]
- [Deferred concern that MUST NOT silently enter scope]

## Assumptions

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right assumptions based on reasonable defaults
  chosen when the feature description did not specify certain details.
-->

- [Assumption about target users, e.g., "Users have stable internet connectivity"]
- [Assumption about scope boundaries, e.g., "Mobile support is out of scope for v1"]
- [Assumption about data/environment, e.g., "Existing authentication system will be reused"]
- [Dependency on existing system/service, e.g., "Requires access to the existing user profile API"]
