# Feature Specification: Backlog Mode (Delivery Decomposition)

**Feature Branch**: `012-backlog-mode`  
**Created**: 2026-04-23  
**Status**: Draft  
**Input**: User description: "Add a first-class `backlog` mode to Canon that transforms bounded upstream artifacts into governed delivery decomposition artifacts without descending into false implementation detail."

## Governance Context *(mandatory)*

**Mode**: change

**Risk Classification**: bounded-impact. This feature adds a new first-class planning mode, authored input contracts, artifact contracts, and closure-gating behavior, but it remains bounded by Canon's existing CLI-first runtime, canonical run identity model, immutable input snapshots, publish flow, and evidence-first persistence under `.canon/`. It does not introduce a new persistence engine, a new identity scheme, or a new execution surface outside the current product boundary.

**Scope In**:

- Add `backlog` as a first-class Canon mode for governed delivery decomposition
- Define authored input contracts for `canon-input/backlog.md` and folder-backed backlog packets
- Define backlog artifact contracts for overview, epic structure, traceability, dependencies, slices, sequencing, acceptance anchors, and planning risks
- Define architecture-closure assessment and block-or-downgrade behavior when source inputs are not credible enough for decomposition
- Define traceability rules from upstream `architecture`, `system-shaping`, `requirements`, or `discovery` sources into backlog outputs
- Ensure backlog packets remain publishable, inspectable, resumable, and reusable as durable planning artifacts
- Update documentation, skills, defaults, and policy surfaces required to support backlog mode honestly

**Scope Out**:

- Fine-grained implementation task generation
- Story-point estimation, sprint planning, team-capacity heuristics, or staffing forecasts
- Tool-specific ticket generation such as Jira- or board-shaped output
- Replacing Canon's run identity, persistence layout, or publish model
- Introducing a new top-level execution surface outside the existing CLI contract
- Promoting unrelated modeled modes such as `incident` or `migration`
- Reworking upstream mode artifact contracts beyond what backlog needs for source traceability

**Invariants**:

- Backlog MUST stop at epics, sub-epics, delivery slices, and story candidates; it MUST NOT generate fine-grained implementation task lists
- Every epic, slice, dependency, and sequencing decision MUST trace to a bounded source artifact, an explicit authored priority, or a named planning gap
- If source architecture or system shape is not sufficiently closed for credible decomposition, Canon MUST block or downgrade the result explicitly rather than pretend confidence
- The current backlog brief MUST remain authoritative; upstream packets are provenance and evidence, not silent overrides
- Backlog authored inputs MUST be snapshotted immutably at run creation and MUST NOT be mutated by the runtime
- Validation evidence MUST remain separate from generation behavior, and emitted planning artifacts MUST remain credible outside Canon

**Decision Traceability**: Decisions for this feature are recorded in `specs/012-backlog-mode/decision-log.md` and cross-linked from the Canon change run that implements backlog mode under `.canon/runs/<…>/decisions/`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Delivery lead produces a governed backlog packet (Priority: P1)

A delivery lead has bounded upstream decisions from architecture or system shaping and wants Canon to turn those decisions into governed epics, slices, dependencies, and sequencing that can guide execution without inventing false implementation detail.

**Why this priority**: This is the core value of backlog mode. Without it, Canon still leaves a governance gap between approved design intent and executable delivery planning.

**Independent Test**: Start a backlog run from bounded upstream artifacts with explicit priorities and scope boundaries, then confirm Canon emits a durable backlog packet that covers epics, slices, dependencies, sequencing, and acceptance anchors without descending into task-level breakdown.

**Acceptance Scenarios**:

1. **Given** a backlog brief that references bounded upstream artifacts and explicit priorities, **When** the user starts a backlog run, **Then** Canon emits a backlog packet containing overview, epic hierarchy, capability mapping, dependency mapping, delivery slices, sequencing, acceptance anchors, and planning risks.
2. **Given** a backlog brief that requests epic-plus-slice decomposition, **When** the run completes, **Then** the output stops at slice or story-candidate level and does not fabricate detailed implementation tasks.
3. **Given** a completed backlog packet, **When** the user publishes or shares it, **Then** the packet remains understandable as a standalone planning artifact with visible source traceability.

---

### User Story 2 - Planner is blocked when source architecture is too vague (Priority: P1)

A planner attempts backlog decomposition from upstream artifacts that are incomplete, contradictory, or insufficiently bounded. They want Canon to say so explicitly instead of producing a confident-looking but untrustworthy decomposition.

**Why this priority**: Honest closure gating is essential to trust. If backlog mode decomposes vague architecture into precise-looking work, it weakens both backlog mode and the upstream governance chain.

**Independent Test**: Start a backlog run from source artifacts that lack bounded capabilities, unresolved dependency boundaries, or coherent exclusions, then confirm Canon blocks or downgrades decomposition and emits explicit closure findings instead of a misleading full packet.

**Acceptance Scenarios**:

1. **Given** source architecture that does not clearly bound capabilities or dependencies, **When** the user starts a backlog run, **Then** the run is blocked or downgraded with the explicit finding that architecture is not sufficiently closed for credible decomposition.
2. **Given** contradictory or incomplete upstream source inputs, **When** Canon assesses the backlog request, **Then** the run surfaces the contradiction or gap as a planning risk or block reason rather than silently choosing one interpretation.
3. **Given** a blocked or downgraded backlog run, **When** the user inspects the outputs, **Then** Canon exposes closure findings and planning risks instead of a false full decomposition.

---

### User Story 3 - Implementation lead reuses backlog slices as bounded handoff (Priority: P2)

An implementation lead wants to take an approved backlog slice or epic and carry it into later execution work without losing the rationale, source lineage, or dependency context that shaped it.

**Why this priority**: Backlog mode only closes the planning gap if its outputs survive handoff into later work instead of becoming disposable notes.

**Independent Test**: Complete a backlog run, publish the resulting packet, and confirm a downstream reader can select a slice for implementation while still seeing its source capabilities, blocking dependencies, sequencing context, and bounded acceptance anchors.

**Acceptance Scenarios**:

1. **Given** a completed backlog packet, **When** a downstream user selects a slice for later execution work, **Then** the slice retains visible lineage to source artifacts, dependencies, and acceptance anchors.
2. **Given** a published backlog packet, **When** a reader consumes it outside Canon, **Then** the packet remains credible as a planning artifact and does not depend on hidden runtime state to make sense.
3. **Given** a backlog packet with explicit cross-epic dependencies, **When** later work is scoped from it, **Then** those dependencies remain visible and do not need to be rediscovered manually.

### Edge Cases

- A backlog brief references multiple upstream artifacts whose priorities or exclusions conflict.
- A user requests sprint-sized task lists, story points, or tool-specific ticket output that exceeds backlog granularity boundaries.
- Source architecture identifies capabilities but leaves ownership or dependency boundaries unresolved.
- A folder-backed backlog packet omits `brief.md` or provides priorities that contradict named dependency constraints.
- Authored backlog inputs are edited or deleted after run creation.
- External dependencies dominate sequencing and make apparently independent epics non-parallel in practice.

## Requirements *(mandatory)*

### Functional Requirements

#### Mode lifecycle and gatekeeping

- **FR-001**: The system MUST treat `backlog` as a first-class governed mode using Canon's existing run lifecycle, identity, and lookup surfaces.
- **FR-002**: The system MUST allow users to start backlog runs from authored backlog input that references upstream bounded artifacts or provides explicit bounded planning intent.
- **FR-003**: The system MUST require a stated delivery intent and desired decomposition granularity for each backlog run.
- **FR-004**: The system MUST assess whether the upstream architecture or system shape is sufficiently closed for credible decomposition before emitting a full backlog packet.
- **FR-005**: The system MUST block or downgrade backlog runs whose source inputs are too vague, contradictory, or insufficiently bounded for credible decomposition.
- **FR-006**: The system MUST surface specific closure findings when a backlog run is blocked or downgraded.
- **FR-007**: The system MUST enforce backlog granularity boundaries so output stops at epics, sub-epics, delivery slices, and story candidates.
- **FR-008**: The system MUST NOT emit fine-grained implementation tasks, sprint allocations, story points, or team-capacity-based sequencing guesses.

#### Authored input contracts

- **FR-009**: The system MUST recognize `canon-input/backlog.md` as a valid single-file authored input for backlog mode.
- **FR-010**: The system MUST recognize a folder-backed backlog packet rooted at `canon-input/backlog/`.
- **FR-011**: For folder-backed packets, the system MUST treat `brief.md` as the authoritative current-mode brief.
- **FR-012**: For folder-backed packets, the system MUST accept `priorities.md` and `context-links.md` as supporting authored inputs when present.
- **FR-013**: The system MUST snapshot all backlog authored inputs immutably at run creation and continue operating from the snapshot for the duration of the run.
- **FR-014**: The system MUST surface clear reasons when required authored fields are missing, including source references, scope boundaries, delivery intent, or desired granularity.

#### Traceability and decomposition discipline

- **FR-015**: Every emitted epic MUST trace to at least one bounded source artifact, explicit authored priority, or named planning gap.
- **FR-016**: Every emitted delivery slice MUST trace to at least one bounded source artifact, explicit authored priority, or named planning gap.
- **FR-017**: Dependency relationships and sequencing rationale MUST trace to source constraints, explicit authored priorities, or named planning risks.
- **FR-018**: The system MUST distinguish source-backed decomposition from planner assumptions and label assumptions explicitly.
- **FR-019**: The system MUST preserve source traceability in both persisted run artifacts and published backlog packets.
- **FR-020**: The system MUST keep out-of-scope and deferrable work visible in the backlog packet rather than silently dropping it.

#### Backlog artifact contract

- **FR-021**: A successful backlog run MUST emit `backlog-overview.md` covering scope, planning horizon, source inputs, and decomposition strategy.
- **FR-022**: A successful backlog run MUST emit `epic-tree.md` covering initiative, epic, and sub-epic structure with clear boundaries.
- **FR-023**: A successful backlog run MUST emit `capability-to-epic-map.md` linking upstream capabilities or decisions to backlog structure.
- **FR-024**: A successful backlog run MUST emit `dependency-map.md` covering cross-epic, intra-epic, and external dependencies relevant to planning.
- **FR-025**: A successful backlog run MUST emit `delivery-slices.md` covering bounded vertical slices and their intended delivery shape.
- **FR-026**: A successful backlog run MUST emit `sequencing-plan.md` covering proposed order, critical path, and safe parallelism.
- **FR-027**: A successful backlog run MUST emit `acceptance-anchors.md` covering bounded signals that an epic or slice is complete at planning level.
- **FR-028**: A successful backlog run MUST emit `planning-risks.md` covering unresolved architecture gaps, oversized work items, hidden dependencies, and sequencing uncertainty.
- **FR-029**: The emitted artifact set MUST be recognizable as backlog output without depending on metadata alone.

#### Closure and downgrade semantics

- **FR-030**: A backlog run that is blocked or downgraded for insufficient closure MUST NOT emit a misleading full decomposition packet.
- **FR-031**: When a backlog run is downgraded rather than fully blocked, the outputs MUST make the closure weakness explicit and center planning risks over false precision.
- **FR-032**: Contradictory upstream source inputs MUST be surfaced as explicit planning risks or blocking findings.
- **FR-033**: Missing dependency boundaries, unresolved exclusions, or unclear capability ownership MUST be surfaced explicitly in closure findings or planning risks.

#### Downstream reuse and runtime compatibility

- **FR-034**: Backlog outputs MUST remain usable as bounded source material for later `implementation` work without losing dependency or source-decision context.
- **FR-035**: Backlog packets MUST remain publishable, inspectable, listable, and resumable through Canon's existing CLI surfaces.
- **FR-036**: Backlog runs MUST use Canon's existing canonical run identity model and immutable input behavior without introducing a parallel scheme.
- **FR-037**: Backlog mode MUST remain compatible with Canon's existing publish workflow and MUST NOT introduce a separate publication path.

#### Documentation, defaults, and skills

- **FR-038**: Repository documentation MUST describe backlog as a governed delivery-decomposition mode with explicit scope boundaries and non-goals.
- **FR-039**: Backlog-related skills and defaults MUST describe real backlog behavior and MUST NOT fabricate runs, identities, evidence, or output artifacts.
- **FR-040**: Defaults and policy surfaces MUST reflect backlog-specific closure checks, artifact expectations, and granularity rules.

### Key Entities *(include if feature involves data)*

- **Backlog Run**: A governed Canon run in backlog mode that holds immutable authored inputs, source references, closure findings, decomposition artifacts, and planning-risk evidence.
- **Backlog Brief**: The authoritative authored planning input that states source references, delivery intent, desired granularity, priorities, constraints, and explicit exclusions.
- **Epic Tree Node**: A bounded planning unit such as an initiative, epic, or sub-epic with explicit scope boundaries and source traceability.
- **Delivery Slice**: A bounded vertical slice derived from one or more source capabilities or decisions that is small enough for downstream execution planning without becoming a task checklist.
- **Acceptance Anchor**: A bounded planning-level signal that an epic or slice should be considered complete, without turning into full acceptance criteria or a test plan.
- **Planning Risk**: A named gap, unresolved dependency, contradiction, or oversize concern that weakens delivery credibility and must remain visible in the packet.
- **Closure Finding**: A specific reason the source architecture or system shape is not sufficiently closed for credible backlog decomposition.
- **Source Trace Link**: A durable reference connecting a backlog output element to the upstream artifact, authored priority, or named planning gap that justifies it.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In sample backlog runs started from bounded inputs, 100% of completed runs emit all eight required planning artifacts and zero fine-grained implementation task lists.
- **SC-002**: In reviewed backlog packets, 100% of emitted epics and delivery slices show at least one visible trace to a source artifact, an authored priority, or a named planning gap.
- **SC-003**: In sample runs started from insufficiently closed architecture, 100% are blocked or downgraded with explicit closure findings instead of producing a misleading full decomposition.
- **SC-004**: A reviewer can identify scope, dependencies, sequencing, and acceptance anchors from a published backlog packet without requiring access to hidden runtime context.
- **SC-005**: A downstream implementation lead can select a backlog slice for later execution work while retaining visible lineage to source decisions and blocking dependencies.
- **SC-006**: Editing or deleting the authored backlog input after run creation does not change the persisted input snapshot used for the run.
- **SC-007**: No backlog output includes story points, sprint plans, capacity-based sequencing guesses, or tool-specific ticket formatting.

## Validation Plan *(mandatory)*

- **Structural validation**: Validate mode definitions, defaults, policy surfaces, skill descriptions, and artifact expectations for backlog mode; run repository formatting and linting checks to ensure the added mode surfaces remain consistent with existing Canon standards.
- **Logical validation**: Add and execute contract and integration checks covering authored input discovery, immutable input snapshots, closure gating, traceability preservation, artifact emission, granularity discipline, and compatibility with inspect, status, list, resume, and publish flows.
- **Independent validation**: Perform a separate review pass on emitted backlog packets to confirm they are credible as standalone planning artifacts, that closure blocks are explicit when needed, and that no output fabricates task-level detail.
- **Evidence artifacts**: Record validation findings, example backlog packets, closure-failure examples, and review notes under the implementing Canon run's evidence bundle and cross-link them from `specs/012-backlog-mode/decision-log.md`.

## Decision Log *(mandatory)*

- **D-001**: `backlog` is introduced as a first-class mode rather than an appendix to `architecture` or `implementation`, **Rationale**: delivery decomposition has its own artifact contract, gates, and downstream handoff semantics.
- **D-002**: Backlog stops at slices and story candidates instead of implementation tasks, **Rationale**: task-level detail creates false precision too early and belongs to execution-focused work.
- **D-003**: Insufficiently closed architecture blocks or downgrades decomposition rather than permitting a confident-looking packet, **Rationale**: trust depends on explicit honesty about closure gaps.
- **D-004**: The current backlog brief remains authoritative over upstream sources, **Rationale**: provenance must inform decomposition without silently overriding the user's current planning intent.
- **D-005**: Backlog outputs remain durable standalone artifacts rather than transient scaffolding, **Rationale**: planning packets must remain useful when published or consumed outside Canon.
- **D-006**: Story points, sprint plans, capacity heuristics, and tool-specific tickets remain out of scope for this feature, **Rationale**: they are organization-specific and would dilute the mode's core goal of bounded, traceable decomposition.

## Non-Goals

- Generating implementation-ready task checklists
- Producing Jira-, board-, or sprint-specific ticket output
- Estimating story points, staffing, velocity, or team capacity
- Replacing architecture or system-shaping as the source of bounded technical decisions
- Replacing implementation mode as the place where execution work is scoped in detail
- Introducing a new persistence, identity, or publication model for backlog artifacts
- Promoting unrelated modes such as `incident` or `migration` in this feature slice

## Assumptions

- Upstream `architecture`, `system-shaping`, `requirements`, or `discovery` artifacts already exist for most backlog requests, or the authored backlog brief explains a bounded basis for decomposition.
- Users of backlog mode want durable planning artifacts that can be consumed both inside Canon and outside it after publication.
- Existing Canon lifecycle surfaces for run lookup, inspection, status, resume, and publish are sufficient for backlog mode and do not require a parallel interface.
- Explicit authored priorities, constraints, exclusions, and deferrable work are available when decomposition quality depends on them; when they are not available, Canon should surface the gap rather than invent a default.
