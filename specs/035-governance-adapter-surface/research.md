# Research: Governance Adapter Surface For External Orchestrators

## Decision 1: Expose a dedicated additive governance namespace

- **Decision**: Add a first-class `canon governance` command namespace for the
  machine-facing surface instead of overloading `run`, `status`, or `inspect`.
- **Rationale**: External orchestrators need a stable product boundary with its
  own compatibility and error semantics. Reusing human-oriented commands would
  blur the contract and increase parser fragility.
- **Alternatives considered**:
  - Hide the surface behind internal helpers only.
  - Reuse `canon run` with extra flags for machine mode.
  - Introduce a separate standalone binary.

## Decision 2: Keep the v1 response flat and treat omitted schema version as v1

- **Decision**: Use a flat `v1` JSON response and interpret well-formed
  requests that omit `adapter_schema_version` as `v1` compatibility input.
- **Rationale**: The current external consumer already expects a flat response.
  Defaulting omitted version markers to `v1` reduces migration friction while
  still letting Canon publish explicit supported schema versions.
- **Alternatives considered**:
  - Require a nested response object for packet details in `v1`.
  - Reject all requests that omit the version marker.
  - Delay version publication until a later adapter release.

## Decision 3: Model missing domain context as blocked outcomes, not parser failures

- **Decision**: Treat well-formed requests that omit domain-required context as
  blocked domain outcomes with stable `reason_code` values rather than protocol
  failures.
- **Rationale**: Downstream orchestrators need deterministic control flow from
  well-formed requests. Missing governance context is a domain precondition
  failure, not malformed transport.
- **Alternatives considered**:
  - Reject missing domain fields as hard schema errors.
  - Infer missing mode, risk, zone, or owner values from local defaults.

## Decision 4: Normalize readiness and approval invariants at the adapter boundary

- **Decision**: Enforce adapter-surface invariants so `governed_ready` only
  escapes with reusable packet state, and `awaiting_approval` only escapes with
  `approval_state: requested`.
- **Rationale**: External consumers should not need to reconcile contradictory
  lifecycle and packet states themselves. The producer owns semantic
  normalization.
- **Alternatives considered**:
  - Forward internal state combinations directly and let consumers downgrade.
  - Expose readiness and approval semantics only in prose messages.

## Decision 5: Standardize canonical workspace-relative refs in responses

- **Decision**: Return packet and document references as canonical
  workspace-relative refs only.
- **Rationale**: Absolute paths are machine-specific, leak local environment
  details, and make downstream comparison brittle. Workspace-relative refs are
  stable across machines and still resolvable by the consumer.
- **Alternatives considered**:
  - Return absolute filesystem paths.
  - Mix relative and absolute refs depending on source provenance.

## Decision 6: Publish capabilities and exact vocabularies in the first release

- **Decision**: Ship a `capabilities` operation in the initial adapter surface
  that publishes supported schema versions, operations, modes, and exact status
  vocabularies.
- **Rationale**: Version discovery and vocabulary inspection are required for a
  public consumer contract to evolve safely. Without them, future changes would
  force trial-and-error integration.
- **Alternatives considered**:
  - Document compatibility only in README and release notes.
  - Defer capability discovery to a later feature.

## Decision 7: Prove the contract with producer-side tests plus a live consumer smoke

- **Decision**: Validate the feature with focused producer-side contract tests,
  a live consumer-driven smoke against a real Canon binary, and line coverage
  above 95% for every modified or newly created Rust source file.
- **Rationale**: Consumer-side stubs alone are insufficient for a public
  compatibility surface. The producer must prove both semantic correctness and
  regression resistance.
- **Alternatives considered**:
  - Validate only with local shell stubs.
  - Rely on whole-workspace coverage without a touched-file threshold.