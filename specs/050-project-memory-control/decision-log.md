# Decision Log: Project Memory And Delivery Control Contracts

- **D-001**: Keep the canonical contract under `docs/integration/`.
  **Status**: Accepted.
  **Rationale**: Consumers need a stable owner-side discovery path outside
  numbered spec directories.

- **D-002**: Use producer-neutral `project-memory:managed` markers.
  **Status**: Accepted.
  **Rationale**: `docs/evidence/` must support Canon and Boundline authored
  blocks without implying duplicate ownership.

- **D-003**: Keep V1 lineage required fields intentionally small.
  **Status**: Accepted.
  **Rationale**: The first implementable contract should minimize mandatory
  metadata while keeping integrity and traceability intact.

- **D-004**: Treat additive V1 changes as compatible and breaking changes as a
  new major contract line.
  **Status**: Accepted.
  **Rationale**: Consumers need explicit stop or proceed rules, not soft
  wording.

- **D-005**: Default repo-visible targets stay under `docs/project/` and
  `docs/evidence/` in V1.
  **Status**: Accepted.
  **Rationale**: A visible default repository shape is part of the value of the
  control layer.