# Decision Log: Governed Expertise Inputs

## 2026-05-14

- **D-001**: Initial expertise kinds are limited to `domain-language` and `domain-model`.
  **Rationale**: these existing modes provide reusable governed knowledge without leaking runtime orchestration semantics.

- **D-002**: The expertise-input contract extends existing project-memory publication semantics instead of creating a new packet channel.
  **Rationale**: Canon should not publish the same governed knowledge through competing contract surfaces.

- **D-003**: Canon remains the semantic producer and Boundline remains the runtime selector.
  **Rationale**: cross-repo clarity depends on keeping publication semantics separate from delivery control flow.

- **D-004**: Unknown expertise kinds and unsupported contract lines fail closed for consumers.
  **Rationale**: consumers must not invent runtime behavior from partially understood Canon output.
