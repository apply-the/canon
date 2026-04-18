# Feature Specification: Analysis Mode Expansion

**Feature Branch**: `006-analysis-expansion`  
**Created**: 2026-04-13  
**Status**: Reviewed  
**Input**: User description: "Title: Analysis Mode Expansion. This is a focused additive specification. Do not rewrite Canon. Do not redesign the runtime..."

## Product Delta

This increment adds full governed depth for `discovery`, `system-shaping`, and `architecture` modes.

It closes the most visible front-of-funnel product gap after the currently strongest delivered modes (`requirements`, `brownfield-change`, and `pr-review`), completing the analysis-heavy front end of the engineering workflow before Canon expands into mutation-heavy execution.

## Problem Statement

Canon currently supports requirements framing, bounded legacy planning, and PR review, but the analysis-heavy front end of the workflow is still incomplete. Users cannot yet use Canon end-to-end for:
- Problem exploration
- New-system shaping
- Architecture decision flow

This gap makes Canon feel uneven across the early engineering lifecycle, forcing users to rely on external documentation or generic chat sessions before they have enough context to initiate a `requirements` or `brownfield-change` run.

## Governance Context *(mandatory)*

**Mode Set**: discovery, system-shaping, architecture
**Risk Classification**: bounded-impact
*Rationale*: We are expanding existing analysis modes safely without mutating execution or adopting new external protocols.
**Scope In**: 
- Full governed depth for `discovery`, `system-shaping`, and `architecture` modes
- Reuse of the existing evidence, gate, persistence, and inspection infrastructure
- Mode-specific artifact contracts, gate evaluations, and inspection expectations
**Scope Out**: 
- Code mutation workflows, refactor, or implementation execution
- Review or verification modes
- Incident or migration modes
- Redesigning the Codex skills taxonomy
- MCP runtime or protocol work
- Inventing a second orchestration subsystem for analysis modes

**Invariants**:

- These modes MUST remain analysis-heavy and non-mutation-first.
- They MUST reuse Canon’s existing governance model rather than invent a new orchestration path.
- They MUST produce outputs grounded in bounded context, not derived from deterministic placeholder generation.
- Decisions and validation evidence for these modes MUST be recorded and persist under `.canon/artifacts/` and `.canon/runs/`.

**Decision Traceability**: Feature decisions live in this specification and its follow-on planning artifacts under `specs/006-analysis-expansion/`; runtime decisions for delivered modes must persist under `.canon/artifacts/<RUN_ID>/{discovery|system-shaping|architecture}/`.

## Why These Modes Belong Together

`discovery`, `system-shaping`, and `architecture` form a coherent increment because:
- All are analysis-heavy and non-mutation-first.
- All share the same broad execution shape.
- All can reuse the existing exploration, architecture, risk, and release-readiness gating with fewer runtime differences than execution-heavy modes.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Problem Exploration (discovery) (Priority: P1)

Users encounter a vague or unstructured problem and need to explore unknowns without immediately committing to requirements or solutions. 

**Why this priority**: Discovery is the operational prerequisite for framing valid boundaries in new domains.

**Independent Test**: Execute `canon run --mode discovery` with partial context. The system must gather context, explore boundaries, critique its own assumptions, and emit mode-specific discovery artifacts without hallucinating fixed requirements prematurely.

**Acceptance Scenarios**:
1. **Given** a rough idea input, **When** the user starts a `discovery` run, **Then** Canon emits a `problem-map.md` and an `unknowns-and-assumptions.md` derived from actual governed analysis, not from generic templates.
2. **Given** a discovery run with unbounded intent and no clear problem domain, **When** the Risk gate evaluates, **Then** the run is blocked with an explicit blocker referencing the missing problem boundary.

---

### User Story 2 - New-System Shaping (`system-shaping`) (Priority: P2)

Users want to translate an approved bounded intent into a concrete system shape and initial delivery paths for a new capability.

**Why this priority**: Completes the arc from discovery/requirements into actionable shaping before implementation begins.

**Independent Test**: Execute a `system-shaping` run starting from bounded requirements. Canon creates system shape artifacts and architecture boundaries, passing governed critique before emitting artifacts.

**Acceptance Scenarios**:
1. **Given** a set of constraints and scope, **When** the user runs `system-shaping`, **Then** the output contains a `system-shape.md` with concrete boundaries and a `risk-hotspots.md` referencing the supplied context.
2. **Given** insufficient context for the Architecture gate, **When** the gate evaluates, **Then** the run blocks rather than emitting an unbounded system shape.

---

### User Story 3 - Architecture Decision Flow (architecture) (Priority: P2)

Users need to formally evaluate architectural boundaries, invariants, and structural trade-offs as a standalone controlled run.

**Why this priority**: Allows deep, deliberate tracking of consequential structural decisions independent of immediate coding efforts.

**Independent Test**: Provide multiple options to an `architecture` run. Canon performs governed challenge/critique against the options, resulting in an architecture decision set and tradeoff matrix.

**Acceptance Scenarios**:
1. **Given** competing system designs, **When** an `architecture` run executes, **Then** Canon challenges claims, records critique evidence, and emits both `architecture-decisions.md` and `tradeoff-matrix.md` traceable to the challenge evidence.
2. **Given** a systemic-impact architecture run, **When** the Risk gate evaluates, **Then** the run requires explicit approval before any artifacts are considered final.

### Edge Cases
- **Context Boundaries Unclear**: If user supplies insufficient requirements context to `system-shaping`, the system should fail the Architecture gate rather than hallucinating random boundaries.
- **Discovery Rabbit Holes**: If a `discovery` run exceeds a bounded depth ratio without resolving, the runtime warns rather than indefinitely looping.
- **Conflicting Invariants**: If an `architecture` run is fed explicitly contradictory baseline invariants, the critique engine must flag the contradiction instead of choosing a random winner.

## Common Execution Pattern

These modes must strictly adhere to Canon's common runtime shape without branching into a second orchestration model:

1. **Bounded context capture**
2. **Governed analysis or generation**
3. **Critique or challenge path** (for consequential architectural/exploratory reasoning)
4. **Gate evaluation**
5. **Evidence persistence**
6. **Artifact emission**
7. **Inspection expectations**

## Mode-by-Mode Specification

### Discovery
- **Purpose**: Explore unknowns without turning exploration into solution drift.
- **Expected Inputs**: Raw problem statements, initial research, or user quotes.
- **Required Artifacts**: `problem-map.md`, `unknowns-and-assumptions.md`, `context-boundary.md`, `exploration-options.md`, `decision-pressure-points.md`.
- **Likely Evidence Sources**: Summarized internal documentation reads, iterative assumption challenges.
- **Required Gates**: Exploration, Risk, ReleaseReadiness.
- **Critique/Challenge Expectations**: Challenge premature commitments to solutions; hunt for hidden unknowns.
- **Inspection Expectations**: Evidence of exploration paths tracked under `.canon/runs/...`.
- **Exploration Gate Specificity**: The discovery Exploration gate must verify that the problem domain is bounded, not that solutions are bounded; unbounded problem domains fail the gate.
- **Support-State Transition**: From modeled-only to full-depth runnable.

### System-Shaping
- **Purpose**: Define a new capability from bounded intent through early delivery structure.
- **Expected Inputs**: Discovery briefs, requirements constraints, bounded problem framing.
- **Required Artifacts**: `system-shape.md`, `architecture-outline.md`, `capability-map.md`, `delivery-options.md`, `risk-hotspots.md`.
- **Likely Evidence Sources**: Domain boundaries reasoning, delivery phasing rationale.
- **Required Gates**: Exploration, Architecture, Risk, ReleaseReadiness.
- **Critique/Challenge Expectations**: Challenge overly tight coupling, overly ambitious delivery slices, and unmitigated risk hotspots.
- **Inspection Expectations**: Readily visible progression from inputs to architecture outlines in the evidence log.
- **Support-State Transition**: From modeled-only to full-depth runnable.

### Architecture
- **Purpose**: Evaluate boundaries, invariants, and structural decisions heavily.
- **Expected Inputs**: Current system baselines, system-shaping options, or specific technical dilemmas.
- **Required Artifacts**: `architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`, `boundary-map.md`, `readiness-assessment.md`.
- **Likely Evidence Sources**: Governed evaluation of alternatives, adversarial review runs, and explicit tradeoff scoring.
- **Required Gates**: Exploration, Architecture, Risk, ReleaseReadiness.
- **Critique/Challenge Expectations**: Severe governed challenge for consequential structural claims and boundary crossing.
- **Inspection Expectations**: Clear traceability from the tradeoff matrix back to the critique evidence that shaped the final decision.
- **Approval Expectations**: Formal sign-off required for systemic-impact or red-zone architectural runs.
- **Support-State Transition**: From modeled-only to full-depth runnable.

## Artifact Contracts

Each mode must define a concrete artifact contract following the same structure used by `requirements`, `brownfield-change`, and `pr-review` in `contract.rs`: file name, required sections, and gate bindings.

### Discovery

| Artifact | Required Sections | Gate Bindings |
|----------|-------------------|---------------|
| `problem-map.md` | Summary, Problem Domain, Boundaries, Unknowns | Exploration, Risk |
| `unknowns-and-assumptions.md` | Summary, Unknowns, Assumptions, Confidence Levels | Exploration, Risk |
| `context-boundary.md` | Summary, In-Scope Context, Out-of-Scope Context | Exploration, ReleaseReadiness |
| `exploration-options.md` | Summary, Options, Constraints, Recommended Direction | Exploration, Risk |
| `decision-pressure-points.md` | Summary, Pressure Points, Open Questions | Risk, ReleaseReadiness |

### System-Shaping

| Artifact | Required Sections | Gate Bindings |
|----------|-------------------|---------------|
| `system-shape.md` | Summary, System Shape, Boundary Decisions, Domain Responsibilities | Exploration, Architecture |
| `architecture-outline.md` | Summary, Structural Options, Selected Boundaries, Rationale | Architecture, Risk |
| `capability-map.md` | Summary, Capabilities, Dependencies, Gaps | Exploration, Architecture |
| `delivery-options.md` | Summary, Delivery Phases, Sequencing Rationale, Risk per Phase | Architecture, ReleaseReadiness |
| `risk-hotspots.md` | Summary, Hotspots, Mitigation Status, Unresolved Risks | Risk, ReleaseReadiness |

### Architecture

| Artifact | Required Sections | Gate Bindings |
|----------|-------------------|---------------|
| `architecture-decisions.md` | Summary, Decisions, Tradeoffs, Consequences, Unresolved Questions | Architecture, Risk |
| `invariants.md` | Summary, Invariants, Rationale, Verification Hooks | Architecture, ReleaseReadiness |
| `tradeoff-matrix.md` | Summary, Options, Evaluation Criteria, Scores, Selected Option | Architecture, Risk |
| `boundary-map.md` | Summary, Boundaries, Ownership, Crossing Rules | Exploration, Architecture |
| `readiness-assessment.md` | Summary, Readiness Status, Blockers, Accepted Risks | Risk, ReleaseReadiness |

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST support executing `discovery`, `system-shaping`, and `architecture` modes end-to-end by reusing the common evidence, gate, persistence, and inspection infrastructure while defining mode-specific execution logic and artifact contracts.
- **FR-002**: System MUST generate bounded, mode-specific artifacts (as defined in the Mode-by-Mode section) populated by real governed execution, rather than via deterministic placeholder generation.
- **FR-003**: System MUST execute governed critique/challenge paths for meaningful analysis claims (especially in architecture and system-shaping).
- **FR-004**: System MUST reuse the `Exploration`, `Architecture`, `Risk`, and `ReleaseReadiness` gates where appropriate, applying mode-specific evaluation logic without fragmenting the gate models.
- **FR-005**: System MUST record explicit evidence of the analysis and challenge phases under `.canon/artifacts/` and `.canon/runs/`, keeping inspection surfaces compatible with existing patterns.
- **FR-006**: The Codex skills frontend MUST expose `canon-discovery`, `canon-system-shaping`, and `canon-architecture` as runnable wrappers once the runtime supports them.

### Relationship to Existing Modes

Canon is an integrated lifecycle, not a pile of isolated modes. These three modes sit upstream or adjacent to existing execution flows:

- **discovery → requirements**: Discovery outputs (problem map, unknowns register, pressure points) provide the bounded framing that a `requirements` run consumes as input. A user who runs discovery first should be able to feed its artifacts directly into `canon run --mode requirements --input ...`.
- **system-shaping → architecture / brownfield-change**: `system-shape.md` and `architecture-outline.md` become the baseline artifacts that an `architecture` run evaluates or that a `brownfield-change` run constrains against. The artifact names and evidence structure must stay compatible across these handoff points.
- **architecture → implementation / refactor / review**: `architecture-decisions.md`, `invariants.md`, and `boundary-map.md` constrain the scope of later execution-heavy or review-heavy modes. Downstream gate evaluation should be able to reference those artifacts as evidence of bounded intent.

This feature does not implement cross-run artifact linking; it only requires artifact names and structures that downstream modes can consume without reinterpretation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In acceptance validation, each of the three modes emits its required artifact family with at least one concrete boundary, option, invariant, or tradeoff derived from supplied context rather than generic framing.
- **SC-002**: 100% of successful runs in these modes persist evidence and gate outcomes that reviewers can trace back from emitted artifacts to governed generation and challenge activity.
- **SC-003**: No run is considered successful if emitted artifacts remain unresolved placeholders, generic boilerplate, or analysis that cannot be tied to supplied context.
- **SC-004**: All three primary user stories complete through the existing run, inspect, approval, and resume trust model without mode-specific workarounds outside the defined scope of this feature.

## Validation Plan *(mandatory)*

- **Structural validation**: Contract validation for the new mode artifact schemas against the declared artifact contracts.
- **Logical validation**: End-to-end functional runs of `discovery`, `architecture`, and `system-shaping` verifying the execution patterns limit payload outputs to context and evidence accurately.
- **Independent validation**: User acceptance testing against standard problem prompts to verify output utility over generic LLM generation.
- **Evidence artifacts**: Feature-level validation notes recorded under `specs/006-analysis-expansion/`, with runtime evidence for delivered behavior recorded under `.canon/artifacts/<RUN_ID>/{discovery|system-shaping|architecture}/`.

**Acceptance Criteria for the Feature**:
- Canon can start real governed runs for `discovery`, `system-shaping`, and `architecture`.
- These modes produce mode-specific artifact sets rather than generic brownfield-style placeholders.
- These modes reuse the common evidence and inspection model.
- Gate evaluation consumes real evidence specific to these modes.
- Governed critique/challenge is demonstrably present where architectural claims are consequential.
- Outputs are strongly bounded by the actual problem context rather than generic templates.
- Codex skill support truthfully exposes these modes as runnable.

## Decision Log *(mandatory)*

- **D-001**: How much ambiguity should `discovery` tolerate before gating blocks progress? **Decision**: Discovery expects high solution ambiguity but MUST require a bounded problem domain; the Risk gate fails if the intent boundary is completely unbounded.
- **D-002**: How should `architecture` challenge differ mechanically or procedurally from `requirements` critique? **Decision**: Architecture challenge focuses heavily on invariants and structural boundary preservation, weighting contract verification over generalized completeness.
- **D-003**: Should `system-shaping` optimize for phased delivery design or conceptual full-system shape first? **Decision**: Conceptual full-system shape comes first to anchor constraints securely before breaking down into phases.

## Open Questions

- **OQ-001**: Which evidence sources are mandatory versus optional in each mode? **Planning implication**: The plan must define the minimum evidence contract for `discovery`, `system-shaping`, and `architecture` without forcing parity where the modes differ materially.
- **OQ-002**: Should explicit approvals apply only to `systemic-impact` analysis runs, or do some `bounded-impact` architectural cases also require approval? **Planning implication**: The plan must make approval thresholds explicit so analysis-heavy modes do not inherit ambiguous policy from adjacent modes.

## Non-Goals

- Executing code mutation workflows.
- Enabling `refactor` or `implementation` execution paths.
- Integrating new protocols (e.g., MCP).
- Handling distribution or packaging work.
- Full expansion of `review` / `verification` modes.
- Performing generic framework updates.
- Expanding the scope to include modes outside of `discovery`, `system-shaping`, and `architecture`.
- Replacing or refactoring the current `requirements` or `brownfield-change` orchestration pipelines.
- Modifying the Codex skills taxonomy structure (only transitioning existing skills to runnable).

## Assumptions

- The core engine orchestration (gatekeeper, service dispatcher, traces) is stable enough to absorb these modes without massive refactoring to `canon-engine`.
- Output formatting and CLI workflows for `run`, `resume`, `approve`, and `status` are generically capable of managing these new modes.
- Users executing these modes have adequate context to define the problem space input accurately.
