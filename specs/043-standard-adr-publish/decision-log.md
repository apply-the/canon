# Decision Log: Standard ADR Publish Artifacts

- **D-001**: Keep ADR generation as a publish-time projection instead of a new runtime artifact family.
  - **Rationale**: `.canon/` remains the authoritative runtime record and the ADR register is a durable repository-facing projection.
  - **Consequence**: Implementation centers on publish code, publish tests, and source artifact mapping instead of mode-wide runtime contract growth.

- **D-002**: Make `architecture` default-on for ADR export and keep `change` and `migration` explicit opt-in.
  - **Rationale**: Architecture decisions are durable by default, while tactical change and migration packets should enter the ADR register only when the operator intends that permanence.
  - **Consequence**: CLI and publish logic must distinguish default, opt-in, and unsupported mode behavior.

- **D-003**: Use a fixed `docs/adr/` registry with sequential `ADR-XXXX-<slug>.md` filenames.
  - **Rationale**: This matches common ADR practice and preserves stable human references.
  - **Consequence**: Publish logic must scan existing ADR files and allocate the next non-conflicting identifier without rewriting history.

- **D-004**: Bound the initial ADR lifecycle to publish-generated `Accepted` records.
  - **Rationale**: The slice is about producing standard ADR artifacts, not implementing a full lifecycle editing system.
  - **Consequence**: Manual rejection, supersession, or status mutation workflows remain out of scope and must be documented as follow-on work.

- **D-005**: Preserve source-packet honesty markers in generated ADRs.
  - **Rationale**: Standard-looking ADR files must not hide weak or missing decision evidence.
  - **Consequence**: Tests must cover missing-context propagation and unsupported-mode rejection, not just happy-path ADR output.

- **D-006**: Treat the version bump as the first implementation task and the 95% touched-file coverage closeout as the last one.
  - **Rationale**: The user explicitly requested ordering and the feature pack should encode that ordering as durable planning intent.
  - **Consequence**: `tasks.md` must place version bump work before behavior edits and reserve final-phase tasks for coverage, fmt, clippy, and regression closeout.

- **D-007**: Reuse authored packet sections directly when synthesizing ADR context, decision, and consequence blocks, and keep packet traceability explicit in the published ADR.
  - **Rationale**: The ADR is a publish-time projection of governed artifacts, so it should stay grounded in authored packet content instead of inventing a second summary layer.
  - **Consequence**: `architecture` maps summary, decision, and tradeoff material into the standard ADR template; `change` and `migration` compose labeled sections from their packet artifacts and always retain a visible source-packet reference.