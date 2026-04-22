# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `change`, `review`,
`verification`, and `pr-review`. The next roadmap should prioritize
completing the remaining modeled modes before widening Canon's surface area
further.

## Feature: Controlled Execution Modes

### Outcome

Canon can move from bounded planning into governed change execution while still
preserving evidence, approvals, and rollback visibility.

### Modes In Scope

- `implementation`
- `refactor`

### Why These Modes Belong Together

- They are the first modes where controlled mutation becomes central rather than
  incidental.
- They both depend on stronger execution controls, contract checks, validation
  hooks, and completion evidence.
- They should share the same mutation-policy and rollback-oriented runtime
  primitives.

### First Slice

- Promote `implementation` from skeleton to full depth.
- Promote `refactor` from contract-only to full depth.
- Keep red-zone and systemic-impact execution recommendation-only until the
  approval model and validation evidence are strong enough.
- Reuse change preservation and release-readiness machinery where behavior
  preservation matters.

### Why This Feature Comes Next

- Execution-heavy modes raise the risk profile of Canon more than analysis or
  review modes.
- The next roadmap step should finish the remaining modeled modes before it
  deepens already-shipped outputs or expands into new external surfaces.

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
