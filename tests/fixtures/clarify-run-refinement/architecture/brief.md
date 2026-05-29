# Architecture Brief

## Decision
Persist typed refinement state on the existing run context and render a run-local working brief artifact.

## Options
- reuse RunState::Draft and RunContext extensions
- create a separate draft identity family

## Constraints
- preserve approval and lineage semantics
- keep inspect clarity separate from run-scoped refinement inspection
