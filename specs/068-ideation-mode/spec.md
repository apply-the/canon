# Feature Specification: Brainstorming Ideation Mode

**Feature Branch**: `068-ideation-mode`

**Created**: 2026-06-03

**Status**: Draft

**Input**: ./brainstorming-ideation.md — A proposal to introduce a green-zone `brainstorming` (or `ideation`) mode in Canon CLI, designed to support divergent thinking and lateral exploration of high-level ideas. This mode prevents premature implementation or convergence by enforcing a read-only, no-code/no-mutation posture, producing an Option Map with at least 3 distinct conceptual approaches, and drafting Spike Proposals for critical validation experiments before routing to downstream convergent modes (discovery, requirements, architecture).

## Governance Context

- **Execution Mode**: `brainstorming`
- **Risk Classification**: Green (read-only exploration, no mutations, systemic impact is downstream).
- **Scope Boundaries**: IN-SCOPE: Defining problem framing, options, trade-offs, unknowns, and spikes. OUT-OF-SCOPE: Emitting production code, schema, or runtime state modification.
- **Invariants**:
  1. The agent MUST NOT write implementation code.
  2. The agent MUST generate at least 3 distinct conceptual approaches to the problem.

## User Scenarios & Testing

### User Story 1 - Explore Multiple Ideas (Priority: P1)

A user provides a rough idea or problem statement to the Canon CLI with the `brainstorming` mode. They want to explore possible conceptual approaches without being forced to converge on a single design immediately.

**Why this priority**: Core value proposition of the brainstorming mode.

**Independent Test**: Can be tested by running the brainstorming mode with a loose prompt and ensuring the system responds with an option map and multiple distinct approaches, not implementation code.

**Acceptance Scenarios**:

1. **Given** a rough problem statement, **When** the user runs the brainstorming mode, **Then** the agent produces at least 3 distinct conceptual approaches.
2. **Given** a brainstorming session, **When** the agent generates options, **Then** it populates a trade-off matrix with pros, cons, and unknowns.

### User Story 2 - Propose Validation Spikes (Priority: P2)

When critical unknowns exist in the trade-off matrix, the user wants the agent to suggest bounded experiments (spikes) to test hypotheses before proceeding to formal architecture or discovery.

**Why this priority**: Bridges the gap between divergent thinking and the concrete experiments needed to move to convergent phases.

**Independent Test**: Can be tested by verifying that `spike-proposals.md` is correctly generated when an approach has significant "Unknowns".

**Acceptance Scenarios**:

1. **Given** an option with critical unknowns, **When** the agent analyzes trade-offs, **Then** it drafts a minimal spike proposal.

### Edge Cases

- What happens when the user intent is already a highly constrained, converged design? (The runtime should suggest routing to `requirements`, `architecture`, or `change` instead).
- How does the system handle an inability to find 3 materially distinct approaches? (The packet must explain why the search space is smaller).

## Requirements

### Functional Requirements

- **FR-001**: System MUST support a new execution mode called `brainstorming`.
- **FR-002**: System MUST enforce a read-only, no-implementation posture during this mode.
- **FR-003**: System MUST accept a rough user prompt, a loosely defined problem statement, or a raw `idea.md` as input.
- **FR-004**: System MUST generate an `option-map.md` (or equivalent) detailing at least 3 distinct conceptual approaches unless justified.
- **FR-005**: System MUST perform trade-off analysis for each approach (Pros, Cons, Unknowns).
- **FR-006**: System MUST generate `spike-proposals.md` (or equivalent) for critical unknowns.
- **FR-007**: System MUST output a defined packet shape: context, options, trade-offs, open questions, and spikes.
- **FR-008**: System MUST recommend a next mode (e.g., `discovery`, `architecture`) upon exit.

### Key Entities

- **Option Map**: Document detailing multiple conceptual approaches.
- **Spike Proposal**: Minimal experiment scope to validate hypotheses.
- **Trade-off Matrix**: Structured evaluation of pros, cons, and unknowns for each option.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Brainstorming runs produce at least three materially distinct approaches 95% of the time.
- **SC-002**: Zero production code or schema mutations are emitted from any brainstorming packet.
- **SC-003**: Downstream `discovery` or `architecture` runs correctly cite an explicit option-map reference from a previous brainstorming run.

## Validation Plan

- **Structural Validation**: Verify the mode creates the required packet files (`01-context.md`, `02-options.md`, `03-tradeoffs.md`, `04-open-questions.md`, `05-spikes.md`).
- **Logical Validation**: Verify that the generated options are distinct and contain the required trade-off analysis structure.
- **Independent Validation**: Review by a senior engineer to ensure no implementation code or schema mutations were suggested, and that the option map properly sets up a discovery or architecture run.

## Decision Log

- **Decision 001**: The brainstorming mode will strictly avoid implementation or schema generation, serving entirely as an advisory phase to output option maps and spikes.
  *Rationale*: Prevents premature convergence and anchors the mode firmly in the divergence phase, distinguishing it from `discovery` or `system-shaping`.

## Non-Goals

- Converging on a single final architecture decision.
- Writing production code.
- Modifying database schemas.
- Creating detailed, implementation-ready specifications.

## Assumptions

- Users understand the difference between exploring options (brainstorming) and defining formal systems (system-shaping).
- The underlying LLM or agent has enough contextual reasoning to generate multiple distinct approaches to a given problem.
