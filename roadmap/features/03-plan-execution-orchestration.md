# 03 - Plan Progress and Handoff Schemas

## Problem
While the `backlog` mode successfully decomposes bounded upstream decisions into delivery epics and slices, and `implementation` creates them, Canon natively lacks a structured metadata schema to represent the intermediate state of a long-running plan. Without a semantic packet definition, external orchestrators (like Boundline) have no standard way to report stateful handoffs, blocked tasks, or checkpoint progress back into Canon's governance layer.

## Proposal
Introduce a schema definition for `progress` and `handoff` packets within Canon. These metadata structures will:
1. Define the required fields for reporting intermediate task states.
2. Standardize the format for context-switching handoffs between execution phases.
3. Establish how blocked or deferred tasks must link to `verification-blockers.md` or `evidence_ref`.

*(Note: The actual runtime orchestration, sub-agent dispatch, state locking, and execution checkpointing are exclusively owned by the Boundline runtime engine. Canon's role is strictly limited to governing the metadata shape of the progress reporting.)*

## Risk Profile

**Governance Zone**: Green (schema definition only).
This feature introduces static metadata definitions for progress reporting. It does not mutate execution state or dictate runtime control flow.

## Why Existing Modes Are Not Enough
- `backlog` decomposes work, but it does not represent the real-time execution state of that work.
- `implementation` governs one bounded delivery slice, but it requires a structured way for the orchestrator to document intermediate handoffs when context limits are reached.

## Dependencies

- **02 - Completion Verification Gates**: Progress packets must reference the `evidence_ref` structures defined in the verification gates contract.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `backlog` | Upstream supplier: backlog decomposes; progress packets report against it. |
| `implementation` | Associated context: handoffs occur within implementation flows. |

## Entry Gates
- A plan must already be accepted and decomposed.
- The external orchestrator must hit a checkpoint, context limit, or explicit handoff boundary to emit the packet.

## Operational Mechanics
- **Inputs**: Task registries, execution logs, and verification evidence provided by the orchestrator.
- **Workflow Steps**:
  1. **Schema Validation**: Canon validates that the `handoff-packet` or `progress-packet` matches the strict structural requirements.
  2. **State Projection**: Canon updates the semantic representation of the project's memory based on the verified progress.
- **Required Artifacts**: Validated JSON or Markdown packet conforming to the schema.

## Exit Gates
- The handoff packet must explicitly mark each task as completed, blocked, skipped, or deferred, with linked `evidence_ref` for each state.
- Ambiguous states are rejected by the schema validator.

## Packet Shape
- `progress-packet`: live task registry mapping current owner, explicit state, and checkpoint references.
- `handoff-packet`: remaining work, resume instructions, and references to `execution-log.md` stored by the orchestrator.

## Success Criteria

- External orchestrators can pause and resume complex plans using Canon's standard metadata format without losing task state semantics.
- Handoff packets cleanly distinguish completed, blocked, skipped, and deferred tasks instead of collapsing them into one success state.