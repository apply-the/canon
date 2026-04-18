# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`greenfield`, `architecture`, `brownfield-change`, and `pr-review`. The next
roadmap should prioritize completing the remaining modeled modes before
widening Canon's surface area further.

## Feature: Review Mode Completion

### Outcome

Canon supports both pull-request review and non-PR review workflows with full
governed evidence, plus explicit verification-oriented challenge flows.

### Modes In Scope

- `review`
- `verification`

### Why These Modes Belong Together

- They are both review-heavy, evidence-heavy, and oriented around challenging
  claims instead of producing implementation.
- They should reuse the `pr-review` pipeline, findings model, review
  disposition handling, and verification records instead of branching into a
  separate subsystem.

### First Slice

- Promote `review` from contract-only to full depth for artifact-bundle or
  change-package review outside PR semantics.
- Promote `verification` from contract-only to full depth for adversarial review
  of claims, invariants, contracts, and evidence bundles.
- Keep their output compatible with the existing inspection and approval
  surfaces.

### Why This Feature Comes Next

- Canon has already completed the main analysis-heavy front end through
  `requirements`, `discovery`, `greenfield`, and `architecture`.
- Canon already has the beginnings of the review model through `pr-review`.
- Completing the broader review surface is a more natural next step than adding
  new protocols.
- It increases trust in the governed system before widening execution.

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
- Reuse brownfield preservation and release-readiness machinery where behavior
  preservation matters.

### Why This Feature Comes After Review Completion

- Execution-heavy modes raise the risk profile of Canon more than analysis or
  review modes.
- The system should finish more of its critique and verification depth before it
  expands deeper into mutation.

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

### Why This Feature Is Later

- These modes benefit from nearly every earlier roadmap improvement.
- They are important, but they should not be the proving ground for unfinished
  execution semantics.

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
