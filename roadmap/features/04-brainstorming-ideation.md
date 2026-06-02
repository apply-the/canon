# 04 - Brainstorming and Ideation

## Problem
When users want to evaluate high-level ideas, explore lateral thinking, or brainstorm possibilities, they are forced into `discovery` or `system-shaping` modes. However, the specs for those modes are rigidly designed to enforce fast convergence toward system limits, domains, and structured artifact outputs, stifling divergent thinking.

## Proposal
Introduce a `brainstorming` mode (or `ideation`). 
- Its primary output artifact is an "Option Map" rather than a final design decision.
- It explicitly accommodates open questions, trade-offs, and experimental proposals.
- It lifts the formal pressure of discovery, enabling a safer, more creative space prior to formal shaping.

## Risk Profile

**Governance Zone**: Green (read-only exploration, no mutations).
Brainstorming produces only advisory artifacts. No code, schema, or runtime
state is modified. The primary risk is downstream: an accepted option could be
cited as justification for convergent work without proper follow-through.

## Why Existing Modes Are Not Enough
- `discovery` and `system-shaping` are intentionally convergent and packetized;
  they are designed to reduce ambiguity, not to widen the option space first.
- Early ideation needs room for structured divergence without pretending that a
  final architecture or bounded change already exists.

## Dependencies

- **None**: brainstorming is self-contained and can start in parallel with any
  other roadmap item.
- **Downstream consumers**: `discovery`, `requirements`, and `architecture`
  benefit from having a bounded option map as input rather than raw user intent.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `discovery` | Downstream: brainstorming produces an option map that scopes a focused discovery run. |
| `system-shaping` | Downstream: a selected option feeds shaping constraints. |
| `architecture` | Downstream: spikes and selected options inform ADR alternatives. |
| `requirements` | Adjacent: brainstorming clarifies intent before requirements formalize it. |

## Entry Gates
- The user intent may still be loose, but the runtime must know the problem area
  and the decision horizon: explore ideas, not implement them.
- The mode must start in read-only, no-implementation posture.
- If the requested scope already implies an accepted design, Canon should route
  to `requirements`, `architecture`, or `change` instead.

## Operational Mechanics
- **Inputs**: A rough user prompt, a loosely defined problem statement, or a raw `idea.md`.
- **Workflow Steps**:
  1. **Divergence Guard**: The agent is explicitly instructed *not* to write implementation code and *not* to prematurely converge. It must generate at least 3 distinct conceptual approaches to the problem.
  2. **Trade-off Analysis**: For each approach, the agent populates a uniform matrix of Pros, Cons, and "Unknowns" (risks that demand further technical validation).
  3. **Spike Proposals**: If an unknown is critical, the agent drafts a minimal "spike" (a disposable technical experiment scope) to validate the hypothesis.
- **Required Artifacts**: An `option-map.md` detailing the approaches, and a `spike-proposals.md` listing potential throwaway experiments. These serve as perfect, bounded inputs for a subsequent, formal `discovery` or `architecture` run.

## Exit Gates
- At least three materially different approaches must be recorded unless the
  packet proves why the search space is smaller.
- The packet must end with explicit unknowns and a recommended next mode, not
  with disguised implementation advice.
- No production code, schema, or migration plan should be emitted from this
  mode.

## Packet Shape
- `01-context.md`: problem framing, constraints, and decision horizon.
- `02-options.md`: candidate approaches described symmetrically.
- `03-tradeoffs.md`: pros, cons, reversibility, and unknowns.
- `04-open-questions.md`: questions that block convergence.
- `05-spikes.md`: bounded experiments worth running before formal shaping.

## Success Criteria

- Divergent exploration runs produce at least three materially distinct
  approaches rather than converging prematurely on the first idea.
- Downstream `discovery` or `architecture` runs cite an explicit option-map
  reference rather than starting from raw user intent.
- Zero production code or schema is emitted from any brainstorming packet.