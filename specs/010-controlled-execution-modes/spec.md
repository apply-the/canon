# Feature Specification: Controlled Execution Modes (`implementation` and `refactor`)

**Feature Branch**: `010-controlled-execution-modes`  
**Created**: 2026-04-23  
**Status**: Draft  
**Input**: User description: "Promote `implementation` and `refactor` from currently modeled but not fully delivered execution modes to fully specified, end-to-end governed modes in Canon, with safety-net requirements before mutation, distinct artifact contracts, and graceful degradation to recommendation-only when risk or evidence preclude safe mutation."

## Governance Context *(mandatory)*

**Mode**: `change`

**Risk Classification**: `bounded-impact`. This feature introduces governed execution behavior, mutation controls, safety-net requirements, and rollback visibility, but it remains bounded by Canon's existing CLI-first product entrypoint, manifest-based persistence, evidence-first model, run identity model, and policy architecture. It does not introduce a new persistence engine, a new identity scheme, or a new public surface outside the existing artifact and publish model.

**Scope In**:

- Promote `implementation` from skeleton to a fully governed execution mode
- Promote `refactor` from skeleton to a fully governed execution mode
- Define mode-distinct artifact contracts, gates, completion criteria, validation evidence, and failure modes for `implementation` and `refactor`
- Define a safety-net requirement that gates consequential mutation of existing code surfaces in both modes
- Define recommendation-only degradation behavior for high-risk, red-zone, or evidence-poor runs
- Compatibility with the canonical run identity, immutable input snapshots, run lookup, listing, and publishable outputs introduced and preserved by recent features
- User-facing and contributor-facing documentation updates required to keep mode guidance honest

**Scope Out**:

- Replacing or restructuring Canon's run identity, persistence layout, or publish model
- Introducing a new top-level execution surface outside the existing CLI contract
- Maturing red-zone or systemic-impact mutation beyond recommendation-only behavior
- Adding domain-modeling outputs to `system-shaping`, `architecture`, or `change`
- Adding `pr-review` or `architecture` output-quality work (Conventional Comments, C4 diagrams)
- Promoting `incident` or `migration` modes
- Distribution channels, packaging, or protocol interoperability work
- Strict TDD as a universal hard requirement on contributors using these modes

**Invariants**:

- Execution-heavy modes MUST NOT silently bypass the gatekeeper; mutation MUST require explicit target bounds
- `refactor` MUST preserve externally observable behavior unless an exception is explicitly authorized and recorded as an artifact decision
- `implementation` MUST map every executed change to approved plan intent, task mapping, or prior governed artifacts
- Authored inputs such as `canon-input/implementation.md`, `canon-input/implementation/`, `canon-input/refactor.md`, and `canon-input/refactor/` MUST be snapshotted immutably and never mutated by the runtime after run creation
- For folder-backed `implementation` and `refactor` packets, the current-mode brief remains authoritative; upstream references are provenance-only unless they are restated in the current brief
- Validation evidence MUST remain separate from generation behavior
- Both modes MUST remain compatible with Canon's current canonical run identity (display ids `R-YYYYMMDD-SHORTID`, UUID, short id, optional slug) and with the existing publish workflow
- Red-zone and systemic-impact runs in these modes MUST remain recommendation-only in this feature slice
- The Canon CLI MUST remain the product entrypoint for these modes; no hidden side channels

**Decision Traceability**: Decisions for this feature are recorded in `specs/010-controlled-execution-modes/decision-log.md` (seeded by `/speckit.plan`) and cross-linked from the Canon change run that implements this feature under `.canon/runs/<…>/decisions/`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Developer executes a bounded `implementation` plan against existing code (Priority: P1)

A developer has an approved bounded plan (typically the output of a prior `change`, `architecture`, or `requirements` run, or an authored brief that references one). They want Canon to execute that plan as governed work that may add new code, modify existing code, and connect new behavior to existing surfaces, but only within explicit mutation bounds and with completion evidence emitted at the end.

**Why this priority**: This is the core promise of `implementation`. Without it, Canon stays a planning and critique platform and never delivers the bounded change-execution loop that the rest of the roadmap depends on. It also unblocks honest mode messaging and removes a long-standing skeleton-vs-real gap.

**Independent Test**: Author an `implementation` brief that references a prior bounded plan and an explicit mutation surface, run the mode, and confirm Canon (a) refuses to start without an explicit mutation surface, (b) snapshots authored inputs immutably, (c) records each change against the plan intent, (d) emits completion evidence and validation hooks, (e) leaves the run discoverable, inspectable, and publishable through the existing CLI surfaces.

**Acceptance Scenarios**:

1. **Given** an approved plan with an explicit mutation surface and a green or yellow zone classification, **When** the developer starts an `implementation` run, **Then** the run progresses to `Completed` and emits the implementation artifact bundle with task mapping, mutation bounds, completion evidence, validation hooks, and rollback notes.
2. **Given** an `implementation` request without an explicit mutation surface or with an unbounded "modify whatever is relevant" intent, **When** the developer starts the run, **Then** the gatekeeper blocks execution and the run surfaces a clear "explicit mutation bounds required" reason.
3. **Given** a completed `implementation` run, **When** the developer runs the existing inspect, status, list, and publish commands, **Then** the run is resolvable by display id, UUID, short id, and `@last`, and its artifact bundle is publishable through the existing publish workflow without invoking a parallel publication path.

---

### User Story 2 - Developer performs a pure `refactor` with preservation gates (Priority: P1)

A developer wants to restructure existing code (extract, rename, deduplicate, decouple, simplify) without changing externally observable behavior. They want Canon to enforce that no new features are introduced, that preservation evidence exists, and that semantic drift, contract drift, or unauthorized feature addition becomes a blocking failure rather than an advisory note.

**Why this priority**: `refactor` is the mode where Canon's core value (governed evidence-first execution) is most exposed. If `refactor` collapses into "implementation with stricter language", Canon loses its strongest argument for being safer than ad-hoc tooling on behavior-sensitive code. Treating preservation as a real gate is what separates a refactor mode from a generic edit mode.

**Independent Test**: Author a `refactor` brief that names the affected behavior and the structural rationale, run the mode, and confirm Canon (a) requires preservation-oriented safety nets before consequential structural mutation, (b) emits preserved-behavior, refactor-scope, structural-rationale, regression-evidence, and contract-drift-check artifacts plus an explicit no-feature-addition proof, (c) treats undeclared semantic or contract drift as a blocking failure, (d) refuses to complete when an authorized exception is missing for any allowed deviation.

**Acceptance Scenarios**:

1. **Given** a `refactor` brief with named preserved behavior, declared structural rationale, and a sufficient existing safety net, **When** the developer starts the run, **Then** the run progresses to `Completed` and emits the refactor artifact bundle including preservation evidence and the no-feature-addition proof.
2. **Given** a `refactor` brief whose affected surface lacks a sufficient safety net, **When** the developer starts the run, **Then** the run blocks consequential structural mutation and surfaces a "preservation safety net missing" reason instead of completing.
3. **Given** a `refactor` run that detects unintended semantic drift, contract drift, or feature addition, **When** the developer reads the run summary, **Then** the run is `Blocked` (or recommendation-only as policy dictates) and the relevant artifact identifies the drift category, not a soft note.

---

### User Story 3 - High-risk request degrades gracefully to recommendation-only (Priority: P2)

A developer requests `implementation` or `refactor` work that is classified as red-zone, systemic-impact, or otherwise lacks the evidence required for safe mutation. They want Canon to execute the run as recommendation-only (dry-run / diff-style output without consequential mutation) and to make the degradation visible, rather than either silently mutating or silently refusing to do anything.

**Why this priority**: This is the safety story. It is what allows Canon to advertise these modes as governed without overreaching v0.1's approval and evidence maturity. It also keeps the door open for later features (high-risk operational programs, mutation-policy hardening) without forcing this slice to solve them.

**Independent Test**: Submit an `implementation` or `refactor` request whose classification is red-zone or systemic-impact, or whose mutation surface lacks the required safety net even after the focused-test step, and confirm Canon (a) does not perform consequential mutation, (b) emits the same artifact contract surfaces but populated as recommendation-only output (e.g. proposed diff, proposed task mapping, proposed regression checks), (c) marks the run state and reasons so existing inspect, status, and list commands can surface "recommendation-only" without ambiguity.

**Acceptance Scenarios**:

1. **Given** a red-zone `implementation` or `refactor` request, **When** the developer starts the run, **Then** Canon completes the run in a recommendation-only posture, does not mutate the workspace, and the run summary identifies the degradation reason.
2. **Given** a yellow-zone request whose safety net cannot be established within the scope of the run, **When** the developer starts the run, **Then** Canon either (a) recommends and executes the focused-test step first or (b) degrades the run to recommendation-only, depending on policy and authored guidance, and never silently mutates the existing surface without preservation evidence.
3. **Given** a recommendation-only run, **When** the developer inspects, lists, or publishes its outputs, **Then** the existing CLI surfaces clearly mark the artifacts as recommendation-only and the publish workflow respects that posture.

---

### Edge Cases

- A `refactor` brief that secretly introduces a new public function or a new branch in observable behavior MUST be detected and treated as a blocking feature-addition failure, not a soft note.
- An `implementation` brief whose authored mutation bounds drift mid-run (e.g. additional files appear in the executed change) MUST cause the run to block at completion rather than silently expanding the surface.
- An `implementation` or `refactor` run whose authored input is edited or deleted after run creation MUST continue to load and operate from the immutable run snapshot, with no mutation of the authored file.
- A run whose preservation safety net regresses (a previously green test goes red after a refactor step) MUST become `Blocked` until the regression is resolved or an explicitly authorized exception is recorded.
- A run that uses only repository-wide coverage as evidence and does not provide focused tests for the touched surface MUST NOT be treated as having sufficient validation evidence.
- A request that resolves to a legacy UUID-keyed run directory MUST still be addressable through these modes by display id, UUID, short id, or `@last`.
- An adapter or mutation request that exceeds the declared mutation bounds MUST be denied by the gatekeeper before execution, not after.
- A red-zone `refactor` request whose authored intent claims "no behavior change" MUST still be executed as recommendation-only in this feature slice.

## Requirements *(mandatory)*

### Functional Requirements

#### Mode lifecycle and gatekeeping

- **FR-001**: The system MUST treat `implementation` as a first-class governed execution mode that progresses through `Pending`, `In Progress`, `Blocked` (when applicable), and `Completed` (or recommendation-only completion) using the existing run lifecycle.
- **FR-002**: The system MUST treat `refactor` as a first-class governed execution mode that progresses through the same lifecycle states with mode-specific gates.
- **FR-003**: The system MUST refuse to start an `implementation` run that lacks an explicit mutation surface or an explicit reference to approved plan intent (prior `change`, `architecture`, or `requirements` artifacts, or an authored brief that names them).
- **FR-004**: The system MUST refuse to start a `refactor` run that does not name (a) the behavior to be preserved and (b) the structural rationale for the change.
- **FR-005**: Both modes MUST honor the existing gatekeeper's bounds-of-mutation enforcement and MUST NOT permit adapter actions that exceed the declared mutation surface.

#### Authored input contracts

- **FR-006**: The system MUST recognize at least the following authored inputs for `implementation`: `canon-input/implementation.md` and `canon-input/implementation/` directory contents, in addition to whatever shared briefing inputs Canon already supports.
- **FR-007**: The system MUST recognize at least the following authored inputs for `refactor`: `canon-input/refactor.md` and `canon-input/refactor/` directory contents, in addition to shared briefing inputs.
- **FR-008**: The system MUST snapshot all authored inputs for these modes immutably at run creation time, and MUST continue to operate from the snapshot for the duration of the run, regardless of subsequent edits or deletions of the authored files.
- **FR-009**: The system MUST surface a clear, machine-and-human-readable reason when an authored input is missing required fields (mutation surface, preserved behavior, structural rationale).

Folder-backed packets are expected to use `brief.md` as the authoritative current-mode brief, `source-map.md` for upstream provenance, and optional `selected-context.md` for narrowed excerpts. Any lineage derived from those files must remain provenance-only metadata and MUST NOT override the current-mode brief.

#### Safety-net (focused-test) requirement

- **FR-010**: For consequential mutation of an existing code surface in either mode, the system MUST require evidence that a focused safety net exists for the touched surface (focused tests, regression checks, or equivalent recorded coverage), recorded as a validation artifact.
- **FR-011**: The safety-net requirement MUST NOT be satisfied solely by the existence of repository-wide coverage; it MUST be tied to the touched surface or to recorded compensating evidence.
- **FR-012**: The system MUST distinguish "safety net present" from "safety net authored as part of this run" and MUST record both states explicitly in the run's evidence bundle.
- **FR-013**: The system MUST NOT mandate strict TDD (red-then-green) for contributors using these modes; the safety-net requirement is an evidence and gating requirement, not a workflow choreography requirement.

#### Mode-distinct artifact contracts

- **FR-014**: An `implementation` run MUST emit at minimum the following artifacts: task-mapping, mutation-bounds, implementation-notes, completion-evidence, validation-hooks, rollback-notes.
- **FR-015**: A `refactor` run MUST emit at minimum the following artifacts: preserved-behavior, refactor-scope, structural-rationale, regression-evidence, contract-drift-check, no-feature-addition.
- **FR-016**: The artifact contracts for `implementation` and `refactor` MUST be distinguishable: a run's persisted artifact set MUST allow a reader to tell which mode produced it without relying on metadata alone.
- **FR-017**: All emitted artifacts MUST be addressable from the run manifest and MUST be persisted under `.canon/runs/<…>/` using the existing layout, without introducing a parallel storage scheme.

#### Mode-distinct completion criteria

- **FR-018**: An `implementation` run MUST be permitted to complete only when (a) every executed change maps to declared plan intent or task mapping, (b) the mutation surface was respected, (c) completion evidence and validation hooks were recorded, (d) the safety-net requirement was satisfied for the touched surface.
- **FR-019**: A `refactor` run MUST be permitted to complete only when (a) preservation evidence shows the named behavior is unchanged, (b) the no-feature-addition proof is present, (c) the contract-drift-check and regression-evidence artifacts are present, (d) the safety-net requirement was satisfied for the touched surface, (e) any allowed deviation has an explicitly authorized exception recorded.

#### Mode-distinct failure semantics

- **FR-020**: An `implementation` run MUST treat the following as blocking: missing or expanded mutation bounds, missing safety net for a consequential mutation, executed changes unmapped to plan intent, missing completion evidence, missing rollback notes when mutation occurred.
- **FR-021**: A `refactor` run MUST treat the following as blocking: undeclared semantic drift, undeclared contract drift, feature addition, missing preservation evidence, missing safety net for the touched surface.
- **FR-022**: Failure reasons in both modes MUST be specific (named category) and MUST be discoverable through the existing inspect and status commands.

#### Recommendation-only degradation

- **FR-023**: The system MUST execute `implementation` and `refactor` runs as recommendation-only (no consequential mutation) when the run is classified as red-zone or systemic-impact.
- **FR-024**: The system MUST execute these runs as recommendation-only when the safety-net requirement cannot be satisfied within the scope of the run, unless an explicitly authorized exception is recorded.
- **FR-025**: Recommendation-only runs MUST emit the same artifact surfaces as fully executed runs, but populated with proposed (not enacted) changes, and MUST be marked unambiguously as recommendation-only in run state and summaries.
- **FR-026**: Recommendation-only runs MUST remain inspectable, listable, and publishable through existing CLI surfaces.

#### Compatibility with existing runtime model

- **FR-027**: Both modes MUST use the canonical run identity model (display id `R-YYYYMMDD-SHORTID`, UUID, short id, optional slug) and MUST be resolvable through existing run lookup, including `@last`.
- **FR-028**: Both modes MUST be compatible with the immutable input snapshot behavior introduced for prior modes; no mode-specific bypass is permitted.
- **FR-029**: Both modes MUST be compatible with the existing publish workflow and MUST NOT introduce a parallel publication path or a mode-specific identity scheme.
- **FR-030**: Both modes MUST be addressable via existing `canon list`, `canon status`, `canon inspect`, `canon resume`, and `canon publish` commands without requiring new top-level commands for basic lifecycle visibility.

#### Documentation, skills, defaults

- **FR-031**: Repository documentation that currently describes `implementation` and `refactor` as modeled-but-not-fully-runnable MUST be updated to reflect the promoted state, scope-in, scope-out, and recommendation-only behavior.
- **FR-032**: The Codex skills currently advertised as "support-state" for these modes (notably `canon-implementation` and `canon-refactor`) MUST be updated to invoke the real workflows and MUST stop fabricating runs, identities, evidence, or CLI output.
- **FR-033**: The defaults under `defaults/methods/` and `defaults/policies/` MUST reflect the promoted modes' gates, artifact contracts, and recommendation-only thresholds.

### Key Entities *(include if feature involves data)*

- **Implementation Run**: A governed execution run in `implementation` mode. Holds an immutable input snapshot, declared mutation bounds, plan-intent linkage, executed change mapping, completion evidence, validation hooks, and rollback notes.
- **Refactor Run**: A governed execution run in `refactor` mode. Holds an immutable input snapshot, named preserved behavior, structural rationale, refactor scope, regression evidence, contract-drift check, and a no-feature-addition proof.
- **Mutation Bounds**: An explicit, declared, machine-checkable surface of files, modules, or contracts that a run is permitted to change. Lives in the run manifest and gates adapter actions.
- **Safety-Net Evidence**: A recorded artifact tied to the touched surface that demonstrates a focused safety net exists (or was authored as part of the run). Used by completion criteria; not solely satisfied by repository-wide coverage.
- **Recommendation-Only Posture**: A run state indicating that artifacts are proposals, not enacted changes. Set by red-zone or systemic-impact classification, by missing safety net, or by explicit policy.
- **Run Identity**: The canonical identity (display id, UUID, short id, optional slug) that all modes use. Reused unchanged from the existing runtime; not redefined here.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A bounded `implementation` brief that names plan intent and an explicit mutation surface produces a `Completed` (or recommendation-only completed) run whose artifact bundle includes task-mapping, mutation-bounds, implementation-notes, completion-evidence, validation-hooks, and rollback-notes — verifiable by inspecting the persisted run on disk.
- **SC-002**: A `refactor` brief that names preserved behavior and structural rationale produces a `Completed` (or recommendation-only completed) run whose artifact bundle includes preserved-behavior, refactor-scope, structural-rationale, regression-evidence, contract-drift-check, and no-feature-addition — verifiable by inspecting the persisted run on disk.
- **SC-003**: An `implementation` request without an explicit mutation surface, or a `refactor` request without named preserved behavior, never reaches consequential mutation; the run is `Blocked` with a specific, named reason.
- **SC-004**: A red-zone or systemic-impact request in either mode never produces consequential mutation; the run completes as recommendation-only with the same artifact surfaces populated as proposals.
- **SC-005**: An `implementation` or `refactor` run whose touched surface lacks focused safety-net evidence does not complete as a mutating run; it either authors and records the safety net first or degrades to recommendation-only.
- **SC-006**: Every `implementation` and `refactor` run is addressable through existing run lookup (display id, UUID, short id, slug, `@last`) and listable, inspectable, and publishable through existing CLI commands without a new top-level command.
- **SC-007**: Every `implementation` and `refactor` run preserves an immutable input snapshot of its authored brief; deleting or editing the authored file mid-run does not change the run's behavior or persisted inputs.
- **SC-008**: The `canon-implementation` and `canon-refactor` Codex skills, after this feature ships, no longer fabricate runs, identities, evidence, or CLI output, and either invoke the real workflows or are explicitly retired in favor of skills that do.
- **SC-009**: Repository documentation that previously described these modes as modeled-but-not-fully-runnable no longer makes that claim once the feature is delivered, and matches the actual runtime behavior verifiable from the CLI.

## Validation Plan *(mandatory)*

- **Structural validation**: Format and lint the workspace (`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`); validate updated `defaults/methods/*.toml` and `defaults/policies/*.toml`; validate skill files via `scripts/validate-canon-skills.sh` (and the PowerShell equivalent where available).
- **Logical validation**: Add and run integration and contract tests under `tests/` covering: mode lifecycle for `implementation` and `refactor`, mutation-bounds enforcement, safety-net gating, recommendation-only degradation, mode-distinct artifact contracts, mode-distinct failure semantics, immutable input snapshot behavior under authored-input edits, run lookup and publish compatibility for both modes.
- **Independent validation**: A reviewer-mode pass (a Canon `review` run, or an equivalent independent reader) MUST confirm that `implementation` and `refactor` artifact contracts are distinguishable from each other and from prior modes, that recommendation-only posture is unambiguous in both run state and artifacts, and that no existing run from prior features regresses in lookup, inspection, or publication.
- **Evidence artifacts**: All validation findings (test fixtures, structural results, reviewer notes, decision rationales) are recorded under `.canon/runs/<…>/evidence/` for the change run that delivers this feature, and cross-linked from `specs/010-controlled-execution-modes/decision-log.md`.

## Decision Log *(mandatory)*

- **D-001**: Both `implementation` and `refactor` are promoted in this single feature slice. **Rationale**: Their artifact contracts and gate semantics differ enough that defining each in isolation would risk diverging gatekeeper behavior; promoting them together keeps the safety-net requirement, recommendation-only degradation, and mutation-bounds model consistent across both.
- **D-002**: The safety-net requirement is an evidence-and-gating requirement, not a mandatory TDD workflow. **Rationale**: Strict TDD as a hard contributor requirement is out of scope for v0.1; what Canon needs to defend is that consequential mutation does not happen against a touched surface without focused coverage evidence.
- **D-003**: Red-zone and systemic-impact runs in these modes remain recommendation-only in this feature. **Rationale**: Red-zone mutation requires approval and rollback maturity that is intentionally out of scope here; recommendation-only is the safe default that still delivers value.
- **D-004**: No new top-level CLI surface is introduced for these modes. **Rationale**: The existing run lookup, listing, status, inspect, resume, and publish commands already cover lifecycle visibility; introducing parallel commands would fracture the surface and conflict with the canonical run identity model.
- **D-005**: Authored input contracts (`canon-input/implementation*` and `canon-input/refactor*`) are required. **Rationale**: Without dedicated authored inputs there is no honest way to enforce mutation bounds, preserved behavior, or structural rationale at run start; reusing only generic briefing inputs would dilute the gates.
- **D-006**: Recommendation-only and fully executed completions share the same artifact surfaces. **Rationale**: Distinguishing posture via run state and explicit markers (rather than via different artifact shapes) keeps inspectors, publishers, and downstream tooling uniform and avoids a mode-within-a-mode model.

## Non-Goals

- Introducing strict TDD as a universal contributor workflow requirement
- Promoting `incident` or `migration` from skeleton to fully governed modes
- Maturing red-zone or systemic-impact mutation beyond recommendation-only
- Replacing or restructuring the canonical run identity, persistence layout, or publish workflow
- Adding `pr-review` Conventional Comments output, `architecture` C4 diagram output, or `system-shaping` domain-modeling outputs (those belong to other features in `NEXT_FEATURES.md`)
- Adding new top-level CLI commands for `implementation` or `refactor` lifecycle that duplicate existing run lookup, status, inspect, list, resume, or publish
- Defining a new artifact storage scheme outside `.canon/runs/<…>/`
- Allowing unbounded "modify whatever is needed" mutation in `implementation`
- Allowing `refactor` to introduce new features behind preservation language

## Assumptions

- The canonical run identity model and immutable input snapshot behavior delivered by recent features are stable and remain in scope as Canon's runtime baseline.
- Existing `change`, `requirements`, and `pr-review` modes continue to operate unchanged; promotion of `implementation` and `refactor` does not require restructuring those modes' contracts.
- The Canon CLI remains the product entrypoint; no IDE-only or chat-only surface is required to make these modes useful in v0.1.
- Adapters and the gatekeeper already support the bounds-of-mutation model that these modes will rely on; this feature primarily adds mode-specific artifact contracts, completion criteria, and recommendation-only posture, rather than rebuilding adapter governance.
- Repository documentation and skill files under `.agents/skills/` and `defaults/embedded-skills/` are within scope for updates required to keep mode messaging honest.
- Contributors using these modes are expected to author or already have a focused safety net for the touched surface; Canon's role is to gate on its presence, not to mandate a specific authoring workflow (e.g., red-then-green TDD).
