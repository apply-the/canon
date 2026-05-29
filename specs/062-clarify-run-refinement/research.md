# Research: Mode Clarification And Run Refinement

**Branch**: `062-clarify-run-refinement`
**Date**: 2026-05-29
**Status**: Complete (all planning-stage unknowns resolved)

## R-001: Durable Draft Identity Reuses the Existing Run Identity

**Decision**: Model the durable draft work item as the existing Canon run
entity in `RunState::Draft`, keeping the same `RunIdentity` from draft
creation through governed run start.

**Rationale**: `crates/canon-engine/src/domain/run.rs` already has a durable
`RunIdentity` plus `RunState::Draft`. Reusing that surface satisfies the spec
requirement to promote or bind the same identity into governed lineage,
avoids inventing a second identifier family, and stays within the existing
`.canon/runs/<RUN_ID>/` persistence layout.

**Alternatives considered**:
- Separate `DraftWorkId` plus later governed `RunIdentity`: rejected because it
  adds a new persistence family and forces brittle promotion logic.
- In-memory draft only until execution starts: rejected because it violates the
  artifact-first requirement for durable clarification state.
- Path-derived identity from the working brief: rejected because it is not a
  stable machine identity and complicates lineage.

## R-002: Persist Typed Refinement State on `RunContext`

**Decision**: Add a typed `ClarificationRefinementContext` to `RunContext`,
following the existing `backlog_planning` extension pattern, and keep the
rendered working brief as an additive artifact under
`.canon/runs/<RUN_ID>/artifacts/<mode>/working-brief.md`.

**Rationale**: `RunContext` already carries mode-specific persisted context,
and `crates/canon-engine/src/persistence/store.rs` round-trips it via
`context.toml`. A typed refinement context can own authoritative and
supporting input refs, clarification records, readiness delta items, current
working brief artifact path, and continuation advice without creating a new
runtime storage family.

**Alternatives considered**:
- Store refinement state only in markdown artifacts: rejected because status,
  inspect, and tests need typed structured data.
- Add a new `.canon/drafts/` directory family: rejected because the existing
  `.canon/runs/<RUN_ID>/` layout already owns durable runtime state.
- Persist refinement state as ad hoc JSON blobs: rejected because stable shapes
  in this repository must use typed serde models.

## R-003: Manifest Lineage Carries Post-Start Mode Changes

**Decision**: Extend `RunManifest` with typed lineage metadata so successor
work created after run start records `carried_from`, `supersedes`, and the
mode-change rationale. Pre-start mode correction mutates the same draft run in
place; post-start correction creates a new successor run.

**Rationale**: `RunState::Superseded` already exists and the manifest is the
stable place for run identity, mode, and persisted lineage. Using manifest
lineage keeps the original started run inspectable while making the carry-
forward relationship explicit and queryable.

**Alternatives considered**:
- Always create a new run on any mode change: rejected because it contradicts
  the draft-in-place refinement rule.
- Mutate a started run in place: rejected because it would obscure evidence,
  approval, and audit history.
- Put lineage only in freeform markdown: rejected because downstream status and
  inspect surfaces need structured runtime data.

## R-004: Candidate Detection Is Advisory and Reuses Explicit Continuation Paths

**Decision**: Candidate detection remains advisory. Canon may surface one
likely draft or run candidate, but it mutates that candidate only after
explicit continuation intent. The existing `canon resume --run <RUN_ID>` path
remains the CLI's explicit continuation mechanism; assistant-first flows may
also accept explicit continuation language or explicit candidate selection.

**Rationale**: The spec and clarification answers require that fresh work does
not silently attach to an older run. Reusing `resume` preserves an existing
explicit command path, while assistant-host flows can still feel natural so
long as they capture an explicit continue, refine, resume, or same-run signal.

**Alternatives considered**:
- Auto-continue when exactly one candidate exists: rejected because a single
  candidate does not prove continuation intent.
- Always ask, even after explicit continue intent: rejected because it adds
  unnecessary friction.
- Require run ID only: rejected because it makes assistant-first refinement too
  rigid.
- Add a new top-level `canon refine` command in this slice: deferred because
  `resume` plus additive host affordances cover the required explicit-intent
  path without widening CLI scope immediately.

## R-005: Structured Clarification Records and Readiness Delta Drive Stable Output

**Decision**: Persist typed `ClarificationRecord` and `ReadinessDeltaItem`
collections in the refinement context, and derive the current flat
`readiness_delta: Vec<String>` summary for compatibility with existing inspect
and output rendering.

**Rationale**: `crates/canon-engine/src/orchestrator/service.rs` and
`input_handling.rs` show that Canon already exposes a flat readiness delta in
authoring-lifecycle output. The new feature needs section-level provenance,
defaults, resolution state, and blockers, which require structured persisted
records. Deriving the flat summary preserves current operator-facing output
patterns while enabling stable typed contracts underneath.

**Alternatives considered**:
- Keep only flat string entries: rejected because it cannot preserve affected
  sections, defaults, or resolution state.
- Replace operator-facing readiness delta with a nested object immediately:
  rejected because it would widen the CLI contract unnecessarily in the first
  slice.
- Store raw TOML or JSON maps with repeated string keys: rejected because the
  repository rules require typed stable models.

## R-006: Working-Brief Seeding Reuses Existing Authority Rules and Templates

**Decision**: Seed targeted-mode working briefs from the existing current-mode
authority rules (`brief.md` when folder-backed, or the single canonical input
when file-backed) plus the existing templates under
`docs/templates/canon-input/` and mode guidance under `defaults/`.

**Rationale**: The repository already distinguishes authoritative briefs from
supporting inputs in templates, docs, and skill-source guidance. Reusing that
surface keeps the feature additive and prevents clarification from rewriting
authored inputs or inventing a parallel authoring model.

**Alternatives considered**:
- Blindly re-template all source content into a new artifact shape: rejected
  because it risks losing authority provenance.
- Use support files as authoritative substitutes when `brief.md` is absent:
  rejected because the existing readiness logic already treats that as an
  authority gap.
- Create per-mode YAML or JSON schemas first: rejected because the existing
  markdown brief ecosystem is already the stable authoring contract.

## R-007: Separate Pre-Run Clarity Inspection from Run-Scoped Refinement Inspection

**Decision**: Keep `canon inspect clarity --mode --input ...` as the pre-run
authored-input analysis surface, extend `canon status --run <RUN_ID>` with a
compact refinement summary, and add a dedicated run-scoped refinement inspect
surface for the full working-brief lifecycle state.

**Rationale**: The current CLI already distinguishes input inspection from
run-scoped artifact, invocation, and evidence inspection. Reusing `inspect
clarity` for both authored-input analysis and persisted refinement state would
blur two different contracts. A dedicated run-scoped surface keeps status
compact while preserving full inspection depth when needed.

**Alternatives considered**:
- Overload `inspect clarity` with `--run`: rejected because it conflates pre-
  run analysis and runtime state.
- Put full refinement state only in `status`: rejected because it makes the
  summary too heavy.
- Rely on artifact browsing alone: rejected because operator workflows need a
  structured inspectable projection.