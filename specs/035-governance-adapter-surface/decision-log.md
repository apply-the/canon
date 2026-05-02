# Decision Log: Governance Adapter Surface For External Orchestrators

## D-001: Ship a first-class additive governance namespace

- **Decision**: Introduce `canon governance` as a dedicated machine-facing
  namespace rather than overloading the existing human-oriented commands.
- **Rationale**: External orchestrators need a stable producer-owned contract
  with explicit compatibility and domain-outcome semantics.

## D-002: Treat omitted schema version as `v1` compatibility input

- **Decision**: Accept well-formed requests without `adapter_schema_version`
  as `v1` compatibility input while still publishing supported schema versions.
- **Rationale**: The first consumer already assumes a `v1`-like flat contract,
  so strict version omission failure would create unnecessary migration cost.

## D-003: Model missing domain-required context as blocked outcomes

- **Decision**: Return blocked domain outcomes with stable `reason_code`
  values when a well-formed request omits the context Canon needs for
  successful domain execution.
- **Rationale**: Missing governance context is a domain precondition failure,
  not malformed protocol input.

## D-004: Normalize readiness and approval consistency before the response escapes

- **Decision**: Enforce strict adapter-surface invariants so
  `governed_ready` implies a reusable packet projection and
  `awaiting_approval` implies `approval_state: requested`.
- **Rationale**: Consumers should not have to downgrade or reinterpret
  contradictory Canon states themselves.

## D-005: Standardize canonical workspace-relative refs

- **Decision**: Return packet and document refs as canonical
  workspace-relative refs only.
- **Rationale**: Absolute paths are unstable across machines and leak local
  filesystem details into a public machine-facing contract.

## D-006: Treat coverage above 95% for touched Rust files as a release gate

- **Decision**: Require more than 95% line coverage for every modified or
  newly created Rust source file in this slice.
- **Rationale**: This feature publishes a new compatibility contract, so the
  touched code paths need a stronger regression bar than generic workspace
  coverage alone.

## User Story 1 Decisions

- **Decision**: Keep blocked request-validation details in `missing_fields`
  and reserve `missing_sections` for packet projection gaps only.
- **Rationale**: External orchestrators need to distinguish request
  precondition failures from packet reuse defects without overloading one
  response field.
- **Decision**: Implement the machine-facing adapter entirely in
  `canon-cli` by reusing public `EngineService` and `WorkspaceStore`
  surfaces rather than mutating engine orchestration code for this slice.
- **Rationale**: The public engine seams are sufficient for start projection,
  and keeping the feature in `canon-cli` preserves narrower mutation scope and
  higher touched-file coverage.

## User Story 2 Decisions

- **Decision**: Treat unreadable `artifact-contract.toml` state as a failed
  runtime outcome and treat missing artifact contracts as tolerable only when
  no packet artifacts are yet present.
- **Rationale**: A machine-facing refresh must not silently promote corrupted
  packet metadata into `governed_ready`, but still needs to degrade gracefully
  before packet materialization begins.
- **Decision**: Keep `governed_ready` strict even when packet files exist by
  requiring a readable artifact contract to define the expected packet shape.
- **Rationale**: Packet files alone are insufficient to prove reuse safety for
  an external consumer contract.

## User Story 3 Decisions

- **Decision**: Treat `missing_fields` as an additive optional `v1` response
  field rather than a breaking schema change.
- **Rationale**: The field tightens contract clarity for blocked requests while
  preserving flat-response compatibility and existing required fields.

## User Story 4 Decisions

- **Decision**: Close feature 035 only with recorded targeted governance
  suites, runtime compatibility validation, full workspace nextest, clean
  format or lint gates, touched-file coverage evidence, and independent
  Synod-driven review findings captured in-repo.
- **Rationale**: The adapter surface is a public boundary, so release evidence
  must live beside the implementation rather than only in ad hoc chat review.