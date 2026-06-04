# Decision Log: Brainstorming Ideation Mode

## D-001: Strict Avoidance of Implementation
- **Decision**: The brainstorming mode will strictly avoid implementation or schema generation.
- **Rationale**: This is an advisory phase meant to explore divergent options. Allowing it to emit implementation code would blur the lines between `brainstorming` and `system-shaping`, leading to premature convergence.
- **Alternatives Considered**: Allow generating skeletal code. Rejected because it violates the divergence goal and creates downstream bias.

## D-002: Output Structure
- **Decision**: The mode will output a defined packet shape consisting of `01-context.md`, `02-options.md`, `03-tradeoffs.md`, `04-open-questions.md`, and `05-spikes.md`.
- **Rationale**: Structuring the output cleanly allows downstream modes (like `discovery` or `architecture`) to consume these artifacts predictably.
- **Alternatives Considered**: A single `brainstorm.md` document. Rejected because structured packets are easier for the canon-engine to parse and track.
