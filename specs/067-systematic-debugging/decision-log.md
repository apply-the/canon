# Decision Log: Systematic Debugging Mode

- **Decision 1**: Introduce a dedicated `debugging` mode rather than extending `change` or `incident`.
  - **Rationale**: The `change` mode assumes boundaries and invariants are already understood. The `incident` mode optimizes for crisis containment and follow-up rather than disciplined source-level fault isolation. The `debugging` mode enforces systematic methodology (reproduction -> TDD -> fix -> verify).
