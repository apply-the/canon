# Execution Context: Brainstorming Ideation Mode

- **Execution Mode**: `brainstorming`
- **Risk Classification**: Green (read-only exploration)
- **Scope Boundaries**: IN-SCOPE: Defining problem framing, options, trade-offs, unknowns, spikes. OUT-OF-SCOPE: Emitting production code, schema, runtime state mutation.
- **Invariants**:
  1. The agent MUST NOT write implementation code.
  2. The agent MUST generate at least 3 distinct conceptual approaches to the problem.
- **Validation Ownership**: Structural/logical by automation/engine; Independent by senior engineer reviewing the output packet.
