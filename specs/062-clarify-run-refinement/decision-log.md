# Decision Log: Mode Clarification And Run Refinement

**Branch**: `062-clarify-run-refinement`

## Design-Stage Decisions

### DL-001: Durable Draft Uses the Existing Run Identity

**Decision**: The durable draft work item reuses the existing Canon run
identity and `RunState::Draft` rather than introducing a second identity type.

**Context**: The feature must preserve same-work continuity from clarification
start through governed execution without creating a disconnected pre-run
object.

**Alternatives**:
- Separate draft identifier plus later promotion: rejected because it widens
  persistence and complicates lineage.
- In-memory clarification state only: rejected because it violates
  artifact-first durability.

**Consequences**: Draft clarification, pre-start mode correction, and run start
all operate on the same `run_id` and `uuid`.

---

### DL-002: Refinement State Lives on `RunContext` and Existing Run Artifacts

**Decision**: Persist typed clarification refinement state on `RunContext` and
materialize the working brief under `.canon/runs/<RUN_ID>/artifacts/`.

**Context**: The repository already persists mode-specific run context and
artifacts under `.canon/runs/<RUN_ID>/`.

**Alternatives**:
- New `.canon/drafts/` family: rejected because it duplicates existing runtime
  ownership.
- Markdown-only refinement state: rejected because status, inspect, and tests
  need typed data.

**Consequences**: No new persistence family is introduced; refinement remains
additive to existing run storage.

---

### DL-003: Candidate Detection Is Advisory, Not Authoritative

**Decision**: Candidate detection may suggest a likely existing draft or run,
but Canon mutates that work only after explicit continuation intent.

**Context**: The feature must avoid silently attaching new work to an older
run while still reducing user friction.

**Alternatives**:
- Auto-continue a single likely candidate: rejected because a single match does
  not prove intent.
- Always ask, even after explicit continue intent: rejected because it adds
  avoidable friction.
- Require run ID only: rejected because it makes assistant-first refinement too
  rigid.

**Consequences**: `resume --run <RUN_ID>` remains the CLI's explicit path; host
surfaces may accept explicit continuation language or selection.

---

### DL-004: Pre-Start Mode Correction Is In-Place; Post-Start Correction Uses Successor Lineage

**Decision**: Canon updates the same draft run in place when clarification
finds the wrong mode before run start, but creates a successor run with
`carried_from` and `supersedes` after the original run has started.

**Context**: The feature must preserve auditability, approval semantics, and
evidence for started governed work.

**Alternatives**:
- Always create a new run on any mode change: rejected because it creates draft
  churn.
- Mutate started runs in place: rejected because it obscures lineage and
  evidence boundaries.

**Consequences**: The original started run stays inspectable, and successor
relationships are explicit in manifest lineage.

---

### DL-005: Structured Runtime Records Derive Flat Operator Summaries

**Decision**: Persist structured clarification records and readiness-delta
items, then derive the existing flat readiness summary and compact status
counts from them.

**Context**: Operator-facing output already uses flat readiness strings, but
the feature needs durable section-level provenance and resolution state.

**Alternatives**:
- Persist only flat strings: rejected because it cannot encode affected
  sections, defaults, and resolution state.
- Replace all operator output with nested structures immediately: rejected
  because it widens the CLI contract too aggressively for the first slice.

**Consequences**: Persistence remains typed and auditable while status and
inspect stay operator-friendly.

---

### DL-006: Run-Scoped Refinement Inspection Gets a Dedicated Surface

**Decision**: Keep `inspect clarity` for authored-input preflight analysis,
extend `status` with a compact refinement summary, and add a dedicated run-
scoped refinement inspect target for detailed lifecycle state.

**Context**: Pre-run clarity inspection and persisted refinement state are
different contracts with different audiences.

**Alternatives**:
- Overload `inspect clarity` with run-scoped refinement output: rejected
  because it conflates pre-run and runtime surfaces.
- Put all refinement detail into `status`: rejected because status should stay
  compact.

**Consequences**: Operator workflows gain a precise inspect surface without
breaking existing clarity semantics.

---

## Implementation-Stage Decisions

### DL-007: Refinement Persistence Stays Additive to Existing Run Storage

**Decision**: Implementation will extend `RunContext` and `RunManifest` with
typed additive refinement and lineage fields instead of introducing a new
runtime storage family or mutating existing authored-input surfaces.

**Context**: The feature must preserve `.canon/runs/<RUN_ID>/` as the only
runtime persistence family while keeping `canon-input/` immutable evidence.

**Alternatives**:
- New `.canon/drafts/` storage: rejected because it duplicates run ownership
  and weakens continuity.
- Freeform map-based persistence: rejected because stable shapes in this
  repository must remain typed and serde-backed.

**Consequences**: Runtime changes stay localized to existing run manifests,
context persistence, and additive working-brief artifacts.

---

### DL-008: Explicit Continuation Intent Is Enforced at the Identity Boundary

**Decision**: Implementation will enforce advisory candidate detection and
explicit continuation intent in the identity and lifecycle services before any
existing draft or run is mutated.

**Context**: Continuation semantics are governance-sensitive because accidental
reuse would silently attach new work to old evidence and approval history.

**Alternatives**:
- Mutate when exactly one likely candidate exists: rejected because candidate
  uniqueness does not establish operator intent.
- Gate only in CLI parsing: rejected because assistant-host and service-layer
  flows must share the same mutation guardrail.

**Consequences**: Continuation safety becomes a runtime invariant rather than a
best-effort UI hint.

---

### DL-009: Successor Lineage Is the Only Post-Start Mode-Change Path

**Decision**: Implementation will treat successor creation with
`carried_from`, `supersedes`, and a recorded mode-change rationale as the only
valid post-start redirection path.

**Context**: Started governed runs must preserve auditability, approval state,
and evidence visibility even when clarification redirects the work.

**Alternatives**:
- Mutate started runs in place: rejected because it obscures evidence and mode
  history.
- Leave successor linkage only in markdown artifacts: rejected because status,
  inspect, and persistence review need structured lineage.

**Consequences**: Post-start redirects remain inspectable, reviewable, and
queryable through persisted runtime metadata.

---

### DL-010: Working-Brief Rendering Preserves Mode Body and Appends Standard Refinement Sections

**Decision**: Implementation will render targeted working briefs by preserving
the mode-specific canonical brief body and appending the standard refinement
appendix only in the run-local artifact.

**Context**: The feature must improve the authoritative brief without
rewriting `canon-input/` or creating an incompatible artifact shape.

**Alternatives**:
- Re-template the full document on every clarification step: rejected because
  it risks losing authoritative section intent and provenance.
- Inject refinement details into authored inputs: rejected because source-input
  immutability is an explicit invariant.

**Consequences**: Renderer work stays mode-appropriate while exposing a stable,
auditable refinement appendix across all targeted modes.

---

### DL-011: Targeted Planning Modes Start as Persisted Draft Refinement Runs

**Decision**: `requirements`, `discovery`, `system-shaping`, `architecture`,
and `change` now materialize a persisted `Draft` run with typed refinement
state before governed execution starts.

**Context**: User Story 1 requires same-work identity continuity through
clarification, plus enough persisted state for advisory continuation guidance
and later explicit `resume` without inventing a second draft identity.

**Alternatives**:
- Keep executing the legacy mode pipeline directly from `run`: rejected
  because it skips the durable draft refinement boundary.
- Stage refinement only in CLI memory: rejected because status, resume, and
  successor lineage need persisted runtime state.

**Consequences**: A targeted `run` now creates the durable draft identity,
stores refinement metadata on `RunContext`, and defers governed execution to
the explicit continuation path.

---

### DL-012: Fresh Same-Work Requests Stay New Runs While Status Surfaces Advisory Continuation

**Decision**: When Canon finds one likely targeted draft candidate with the
same authored-input fingerprint, it still creates a new draft for a fresh
request and surfaces the older draft only as advisory continuation guidance.

**Context**: Candidate detection must help operators recover same-work context
without silently mutating an older governed work item.

**Alternatives**:
- Reuse the candidate automatically: rejected because uniqueness does not prove
  intent.
- Hide the candidate unless the operator already resumed: rejected because it
  removes the operator hint that makes explicit continuation practical.

**Consequences**: Status JSON now promotes a top-level
`suggested_continuation` object with `mutation_allowed = false`, while
markdown status output explicitly states that candidate detection is advisory
and continuation still requires intent.

---

### DL-013: Explicit Resume Captures Continuation and Mode Correction Respects Draft Boundaries

**Decision**: `resume --run <RUN_ID>` clears the persisted
`explicit_continuation_required` flag before downstream execution logic runs,
while mode correction stays in-place only for `Draft` runs and creates a
lineage-linked successor once the original run has started.

**Context**: The explicit CLI resume path is the current authoritative
continuation signal, and mode redirection must preserve auditability after
governed execution begins.

**Alternatives**:
- Keep `explicit_continuation_required` set after resume: rejected because the
  operator already supplied explicit continuation intent.
- Reuse the same run for post-start mode correction: rejected because it would
  blur evidence and lineage.

**Consequences**: Resume now persists the captured continuation intent on the
same run identity, pre-start correction updates the draft in place, and
post-start correction creates a successor draft with structured lineage back to
the original run.

---

### DL-014: Working Brief Rendering Preserves The Authored Body And Appends A Standard Refinement Appendix

**Decision**: The targeted working brief renderer preserves the authored or
template-seeded mode body verbatim and appends the standard refinement appendix
for clarification provenance, source snapshots, unresolved questions,
readiness delta, and continuation state.

**Context**: User Story 2 requires a run-local authoritative brief that can be
updated during clarification without mutating `canon-input/` or dropping the
operator-visible provenance of answers, defaults, and unresolved items.

**Alternatives**:
- Overwrite the body with a new synthesized template on every clarification
  step: rejected because it risks erasing authoritative authored structure.
- Keep refinement metadata only in runtime context: rejected because operators
  need a readable working brief artifact that carries the same durable state.

**Consequences**: `working-brief.md` remains the canonical human-readable
artifact for targeted refinement, while the structured runtime context remains
the source of truth for records and readiness items.

---

### DL-015: Inspect Refinement Uses A Dedicated Run-Scoped Payload Instead Of Reusing Status Summary

**Decision**: `inspect refinement` now emits a run-scoped payload with
`run_id`, `mode`, `state`, `working_brief_path`, full clarification records,
structured readiness items, advisory continuation state, and optional lineage,
instead of reusing the compact refinement summary exposed by `status`.

**Context**: The compact status summary is intentionally additive and brief,
but User Story 2 needs a detailed inspect surface that can expose record-level
clarification and successor lineage without overloading `status`.

**Alternatives**:
- Reuse the status refinement summary for inspect: rejected because it hides
  clarification records and structured readiness items.
- Add the full inspect payload under `status`: rejected because it widens the
  operational summary contract too aggressively.

**Consequences**: The inspect surface can evolve around detailed refinement
state without weakening the compact status contract, and markdown rendering can
mirror the run-scoped payload headings directly.

---

### DL-016: Targeted Clarification Questions Must Name Specific Decision Surfaces

**Decision**: Targeted requirements and discovery clarification questions now
carry specific `affects` metadata that names mode fit, scope boundaries,
operator choices, handoff readiness, or other concrete decision surfaces,
rather than the generic `packet readiness` label.

**Context**: User Story 2 requires clarification questions to stay bounded to
decision-changing gaps so Canon does not ask trivia or restate already
authoritative content.

**Alternatives**:
- Keep generic `packet readiness` metadata everywhere: rejected because it does
  not tell operators which downstream decision or artifact a skipped answer
  would actually change.
- Push mode-specific metadata only into the renderer: rejected because the
  metadata needs to stay durable and reusable across inspect and runtime flows.

**Consequences**: Clarification questions now communicate why a missing answer
matters to mode fit, scope, handoff, or user-visible output, which aligns the
inspect surface with the feature’s decision-changing question boundary.

---

### DL-017: Cross-Mode Guidance Mirrors Targeted Refinement And Advisory Continuity Split

**Decision**: Canon guidance now treats `requirements`, `discovery`,
`system-shaping`, `architecture`, and `change` as the only modes with the full
working-brief refinement lifecycle, while `review`, `verification`,
`implementation`, `refactor`, `incident`, and `migration` preserve same-work
identity continuity without claiming that targeted working-brief behavior.

**Context**: User Story 3 extends runtime continuity across modes, but the
feature scope still limits the first-class working-brief artifact and targeted
clarification loop to the five planning modes. Methods, templates, embedded
skill guidance, and repo-local skill fronts must all describe that split the
same way or operators will infer behavior Canon does not actually provide.

**Alternatives**:
- Describe all modes as if they already share the full working-brief lifecycle:
  rejected because non-targeted modes currently persist advisory continuity
  state, not the targeted planning artifact flow.
- Avoid mentioning the split in method and skill guidance: rejected because
  that leaves the runtime behavior discoverable only through status and inspect
  outputs.

**Consequences**: Targeted guidance now tells operators that draft refinement
 keeps one run id, seeds a run-local working brief, and only mutates existing
 work after explicit continuation intent. Non-targeted guidance now states that
 candidate detection is advisory, continuation still requires explicit intent,
 and fresh requests remain new runs unless the operator explicitly continues
 existing work.