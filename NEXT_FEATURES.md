# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `change`, `review`,
`verification`, and `pr-review`. The next roadmap should prioritize
completing the remaining modeled modes before widening Canon's surface area
further.


## Feature: High-Risk Operational Programs

### Outcome

Canon can govern high-stakes operational work where sequencing, blast radius,
compatibility, and containment matter more than ordinary implementation flow.

### Modes In Scope

- `incident`
- `migration`

### Why These Modes Belong Together

- They are both high-risk, coordination-heavy, and gate-heavy workflows.
- They need stronger containment, compatibility, sequencing, and fallback
  semantics than the ordinary build/change path.
- They should build on a mature execution and verification core rather than
  force Canon to invent those primitives too early.

### First Slice

- Promote `incident` from skeleton to full depth.
- Promote `migration` from skeleton to full depth.
- Add explicit artifact contracts for blast radius, containment, compatibility,
  sequencing, and fallback planning.
- Keep approval and release-readiness expectations stricter than in ordinary
  implementation flows.

### Why This Feature Comes After Controlled Execution

- These are the remaining unfinished modeled modes after `implementation` and
  `refactor`.
- They benefit from nearly every earlier roadmap improvement, but they still
  belong ahead of output-polish, packaging, and protocol expansion.

## Feature: Stronger Review And Architecture Outputs

### Outcome

Canon makes already-delivered critique modes more directly reusable in real
engineering workflows by emitting `pr-review` feedback in a Conventional
Comments shape and extending `architecture` packets with C4 model documents.

### Modes In Scope

- `pr-review`
- `architecture`

### Why These Modes Belong Together

- They are already delivered end to end, so the next value step is output
  quality and interoperability with human workflows.
- Both produce artifacts that are read outside Canon and benefit from stronger
  standardization.
- They improve downstream handoff into pull request review and architecture
  communication without widening Canon into new runtime domains.

### First Slice

- Teach `pr-review` to structure review findings so they can be consumed as
  Conventional Comments for code review.
- Extend `architecture` output contracts to include C4 model documents,
  starting with system context, container, and component views when the
  authored input supports them.
- Keep the existing decision, invariants, boundary, and tradeoff artifacts so
  architecture remains critique-first instead of collapsing into diagram-only
  output.

### Why This Feature Comes After Mode Completion

- It deepens the usefulness of modes Canon already ships instead of increasing
  mode coverage.
- It should follow completion of the remaining modeled modes because it is
  output-quality work, not closure of a missing workflow.

## Feature: Domain Modeling And Boundary Design

### Outcome

Canon strengthens the already-delivered shaping and architecture modes by
making Domain-Driven Design outputs first-class: explicit ubiquitous language,
bounded contexts, context relationships, and domain invariants that can flow
into later architecture, change planning, and review.

### Modes In Scope

- `system-shaping`
- `architecture`
- `change`

### Why These Modes Belong Together

- They are the modes where domain boundaries, ownership, and invariants matter
  more than raw implementation sequencing.
- They benefit from a shared language for bounded contexts, context crossings,
  and preserved business rules.
- They reduce downstream drift by making the domain model explicit before
  execution-heavy modes begin.

### Continuity With Current Mode Model

- `system-shaping` becomes the primary entry point for domain discovery when a
  capability's structure is not yet fixed.
- `architecture` formalizes domain boundaries and relationships once the
  structure is being settled.
- `change` enforces domain invariants during bounded modification of an
  existing system.

This extends the current mode model instead of introducing a separate planning
vocabulary for domain work.

### First Slice

- Extend `system-shaping` to emit a candidate domain map, core-domain
  hypotheses, and a ubiquitous-language seed.
- Extend `architecture` to emit bounded-context and context-relationship
  artifacts, including integration seams and anti-corruption candidates where
  the decision surface warrants them.
- Extend `change` to tie the change surface to an explicit domain
  slice, preserved invariants, and ownership boundaries instead of treating the
  system as one undifferentiated code surface.
- Keep the existing critique-first posture so Canon challenges weak or blurry
  domain boundaries instead of rubber-stamping them.

### Why This Feature Comes After Mode Completion

- It improves the quality of modes Canon already ships before the roadmap
  expands into packaging or protocol work.
- It should follow completion of the remaining modeled modes because it
  strengthens existing workflows instead of closing a missing one.

## Feature: Backlog Mode — Delivery Decomposition

### Outcome

Canon transforms bounded architecture decisions and system shape into governed
epics, delivery slices, dependencies, and sequencing without descending into
false task details or ignoring architecture closure gaps.

### Modes In Scope

- `backlog`

### Problem This Solves

Currently, Canon has governance for understanding problems (`discovery`,
`requirements`), shaping systems (`system-shaping`), making decisions
(`architecture`), and executing changes (`implementation`, `refactor`). But
there is no governed mode that bridges from "we have decided what to build" to
"here is the decomposition into deliverable work".

Today that gap means either:

- Architecture artifacts get handed off informally to task splitting,
- Backlog planning happens outside Canon with no connection to gated decisions,
- Or implementation mode runs on unbounded problem spaces instead of bounded
  slices.

### Input Shape

A backlog brief with explicit source references and delivery intent:

```
canon-input/backlog/
  brief.md
  priorities.md
  context-links.md
```

or single file:

```
canon-input/backlog.md
```

### Good Input Should Include

- Source artifact references (which `architecture`, `system-shaping`,
  `requirements`, or `discovery` packets drive this backlog)
- Delivery horizon and priorities
- Known constraints (team, time, dependency)
- Desired granularity (epic only, epic+slice, epic+slice+story-candidate)
- Out-of-scope items and deferrable parts

### What Canon Emits

Backlog produces a delivery decomposition packet with these artifacts:

- `backlog-overview.md` — scope, horizon, sources, strategy
- `epic-tree.md` — initiative/epic/sub-epic hierarchy with clear boundaries
- `capability-to-epic-map.md` — traces from system-shaping/architecture
  capabilities to backlog structure
- `dependency-map.md` — explicit cross-epic and external dependencies
- `delivery-slices.md` — implementable vertical slices per epic, with
  foundation vs. feature distinction
- `sequencing-plan.md` — execution order, parallelism, and critical path
- `acceptance-anchors.md` — how to recognize each epic or slice as complete
  (not full acceptance criteria, but bounded enough to steer)
- `planning-risks.md` — gaps where architecture is not yet closed,
  underestimated dependencies, or epics too large to commit

### Key Constraints

#### Granularity Discipline

Backlog stops at **delivery slices and story candidates**. It does not emit
fine-grained task lists; that is implementation-mode work. If the backlog
reaches too far into task minutiae, it discovers false details and loses
credibility.

#### Architecture Closure Check

If the source architecture is too vague, `backlog` mode gates or downgrades
the result. It must be able to say:

> "Architecture is not sufficiently closed for credible decomposition."

This is a feature, not a bug.

#### No Blind Task Generation

Backlog does not decompose based on guesses about team capacity, story point
totals, or mechanical task splitting. Every slice and epic must be anchored in
either:

- the bounded scope from `architecture` or `system-shaping`,
- an explicit dependency identified in the decomposition,
- or a named gap that the backlog calls out as unsettled.

#### Reusable Outside Canon

The emitted artifacts must remain credible and useful when read as a standalone
planning document. Backlog is not scaffolding for a later mode; it is a
durable decomposition that implementation and review modes consume.

### Typical Flow

```
architecture (or system-shaping) → backlog → implementation
```

In simpler cases:

```
requirements → system-shaping → backlog
```

Or direct:

```
system-shaping → backlog → implementation
```

### Typical Handoff After This Mode

- publish the approved backlog packet with `canon publish <RUN_ID>` to
  `docs/planning/<RUN_ID>/`, or use `--to` for a different public destination
- pass individual slices or epics into `implementation` mode once they are
  approved and the blocking dependencies are resolved
- return to `architecture` only if decomposition reveals architecture is too
  vague or incomplete
- use the `capability-to-epic-map.md` and `dependency-map.md` to keep later
  work traceable back to earlier decisions

### Common Mistakes

- using Backlog before architecture or shape is actually bounded
- descending into task-level granularity instead of staying at slice/story
  candidate level
- treating Backlog as a Jira ticket generator instead of a bounded
  decomposition artifact
- ignoring architecture closure gaps and pretending decomposition is credible
  anyway
- generating false priorities without explicit source input
- omitting dependency tracking and sequencing

### Why This Feature

- It completes the pipeline from problem understanding through approved
  decisions to deliverable decomposition.
- It prevents architecture artifacts from being handed off informally or
  getting lost in untracked task splitting.
- It maintains traceability and governance over how decisions become work.
- It lets later modes (`implementation`, `review`) stay focused on execution or
  verification instead of also trying to decompose.

### Why After Remaining Modes Completion

- `backlog` depends on closure of architecture or system-shaping decisions, so
  it makes sense as a follow-on.
- Prioritizing remaining modeled modes (`incident`, `migration`) is higher
  priority than adding new planning modes, since backlog works well with
  existing infrastructure and only requires a new skill and mode implementation.
- This feature enables better execution flow for the later implemented modes
  without blocking their completion.

### Possible Names

- `backlog` (team-familiar, tool-obvious)
- `delivery-planning` (more methodological)
- `decomposition` (more technical)
- `roadmapping` (less actionable but less Jira-flavored)

Recommendation: stick with `backlog` for clarity.

## Feature: Distribution Channels Beyond GitHub Releases

### Outcome

Users can install Canon through familiar package-manager channels without
depending on manual archive download and placement.

### First Slice

- Support Homebrew as the first package-manager channel for macOS and Linux.
- Support `winget` as the primary Windows package-manager channel.
- Support Scoop as a secondary Windows channel.

### Deferred

- `apt` or Debian packaging, until Canon has a stable release and repository
  publishing process worth maintaining.

### Why This Feature

- GitHub Releases are already the canonical source of downloadable binaries.
- The next distribution step is reach and upgrade convenience, not basic release
  production.
- Homebrew gives the strongest first install story on macOS and a viable one on
  Linux.
- `winget` is the most credible first-class Windows distribution target.
- Debian packaging adds maintenance cost too early.

## Feature: Protocol Interoperability

### Outcome

Canon remains protocol-agnostic at the core while gaining a practical path to
govern structured external-tool invocation and future interoperable exposure.

### Direction

- Canon core must remain protocol-agnostic.
- First protocol adapter: MCP consumer support.
- Future external surface: a minimal MCP server.
- A2A remains a later architectural option, not a near-term implementation
  priority.

### First Slice

- Treat MCP as the first protocol Canon actively consumes for governed external
  tool invocation.
- Route MCP requests through the same request, decision, approval, and evidence
  pipeline used by other execution adapters.
- Keep policy, capability typing, lineage, and verification semantics shared
  across protocols.
- Only ship MCP runtime behavior if it fits the common invocation pipeline
  cleanly, without introducing protocol-specific special cases into the core.

### Future Minimal MCP Server Surface

- Start a governed run.
- Inspect run state and evidence.
- Request a structured review.
- Avoid exposing the full internal engine surface.

### Explicitly Deferred

- A2A support, unless Canon needs to operate as a remote interoperable agent.
- Agent discovery, remote delegation, and network-visible multi-agent topology.
- Broad protocol exposure beyond narrowly governed capabilities.

### Why This Order

- MCP solves tool and resource interoperability, which matches Canon's current
  need to govern calls to external tools.
- A2A solves agent-to-agent interoperability, which is a different layer of the
  system.
- Canon is currently closer to a governed local execution runtime than to a
  remote agent platform.
- Supporting MCP first strengthens the execution layer without forcing premature
  multi-agent architecture.

### Planning Principle

MCP first, A2A later if Canon becomes a network-visible agent.
