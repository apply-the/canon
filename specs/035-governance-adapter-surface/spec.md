# Feature Specification: Governance Adapter Surface For External Orchestrators

**Feature Branch**: `035-governance-adapter-surface`  
**Created**: 2026-05-02  
**Status**: Draft  
**Input**: User description: "Expose a first-class, versioned, machine-facing Canon governance adapter surface aligned with Synod: `canon governance start|refresh|capabilities --json`, flat responses, domain-level blocked validation, `governed_ready` only with reusable packets, stable workspace-relative refs, and explicit machine-readable reason codes."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: systemic-impact because this feature establishes Canon's first public machine-facing governance contract for external orchestrators, changes how Canon projects lifecycle and packet readiness to downstream consumers, and creates a new compatibility promise that must remain stable across releases.

**Scope In**:

- Introduce a first-class `canon governance` namespace with `start`, `refresh`, and `capabilities` as machine-facing operations.
- Define a versioned compatibility profile for adapter requests and responses that external orchestrators can rely on without scraping human CLI output.
- Project Canon's existing governance semantics into stable machine-readable outcomes covering lifecycle, approval posture, packet readiness, packet references, and machine-usable reason codes.
- Make domain-missing context such as `mode`, `system_context`, `risk`, `zone`, `owner`, and `run_ref` on refresh produce deterministic domain outcomes instead of protocol breakage.
- Require canonical workspace-relative references for packet and document paths returned to consumers.
- Align docs, changelog, version surfaces, and validation evidence for the `0.35.0` release.

**Scope Out**:

- Session ownership, planning loops, retry or replan behavior, cluster orchestration, assistant routing, and other orchestration-brain concerns.
- New persistence schema, new publish destinations, or changes to `.canon/` layout, run identity, or approval targets.
- An HTTP service, daemon, or remote orchestration transport in this slice.
- Consumer-specific workflow taxonomies, stage-state models, or packet formats that bypass Canon's existing governance model.

**Invariants**:

- Canon MUST remain the downstream governed runtime; it MUST NOT become the owner of orchestration, session control, or delivery-loop decisions.
- `status: governed_ready` MUST only be emitted when the packet is actually reusable, inspectable, and backed by a non-empty packet projection.
- `status: awaiting_approval` MUST only be emitted with `approval_state: requested`.
- `packet_ref`, `expected_document_refs`, and `document_refs` MUST be canonical workspace-relative references, never machine-dependent absolute paths.
- Domain outcomes such as blocked, failed, approval-gated, and ready states MUST stay machine-readable and deterministic, separate from transport or protocol failures.
- Existing human-oriented CLI flows such as `run`, `resume`, `status`, `approve`, `verify`, `inspect`, and `publish` MUST remain behaviorally intact.

**Decision Traceability**: Decisions for this feature will be recorded in `specs/035-governance-adapter-surface/decision-log.md`, with validation evidence, compatibility checks, and closeout notes recorded in `specs/035-governance-adapter-surface/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Start A Governed Attempt From An External Orchestrator (Priority: P1)

As an external orchestrator, I want to start a governed Canon attempt through a stable machine-facing command so I can delegate governance work to Canon without translating through human-oriented CLI flows.

**Why this priority**: Without a supported start surface, the Canon integration remains an implementation assumption rather than a product capability.

**Independent Test**: A consumer can invoke the start operation with a well-formed request and receive one valid domain response that allows it to continue, block, or escalate without reading `.canon/` internals.

**Acceptance Scenarios**:

1. **Given** a well-formed start request with the domain fields Canon needs to execute successfully, **When** the consumer invokes the machine-facing start operation, **Then** Canon returns one valid domain response including lifecycle state, approval posture, and a `run_ref` when a governed run is materialized.
2. **Given** a well-formed start request that omits one required domain field such as `mode` or `owner`, **When** the consumer invokes the operation, **Then** Canon returns a blocked domain outcome with a machine-usable reason code instead of a protocol failure.
3. **Given** a well-formed request that targets an unsupported governance context, **When** the consumer invokes the operation, **Then** Canon returns a blocked domain outcome that explains why the request could not proceed.

---

### User Story 2 - Refresh A Governed Attempt And Trust Packet Readiness (Priority: P2)

As an external orchestrator, I want to refresh an existing Canon-governed attempt so I can decide whether the packet is reusable, incomplete, rejected, approval-gated, or still in progress without reverse-engineering Canon state.

**Why this priority**: Long-running orchestration depends on deterministic re-entry; if refresh is ambiguous, the downstream integration remains brittle.

**Independent Test**: A consumer can invoke the refresh operation against an existing run and determine next action solely from Canon's JSON outcome and packet projection.

**Acceptance Scenarios**:

1. **Given** an existing governed run with a reusable packet, **When** the consumer invokes refresh, **Then** Canon returns `status: governed_ready` together with a reusable packet projection.
2. **Given** an existing governed run whose packet is incomplete, rejected, or scaffold-only, **When** the consumer invokes refresh, **Then** Canon does not emit `governed_ready` and instead returns a blocked or approval-gated outcome with the missing packet signals exposed.
3. **Given** the same run state and packet state across repeated refresh requests, **When** the consumer invokes refresh multiple times, **Then** Canon returns materially identical readiness semantics and does not create an unrelated new run.

---

### User Story 3 - Discover Compatibility Before Binding To Canon (Priority: P3)

As an orchestrator maintainer, I want to inspect Canon's supported adapter schema versions and governance vocabularies before binding a consumer release so I can fail early on compatibility rather than learning at runtime.

**Why this priority**: A machine-facing contract without version or capability inspection is fragile and expensive to evolve.

**Independent Test**: A consumer can inspect Canon's capability response and decide whether the current Canon binary can support the expected governance contract without attempting a live run.

**Acceptance Scenarios**:

1. **Given** a Canon binary that ships the adapter surface, **When** the consumer requests capabilities, **Then** Canon returns supported schema versions, supported operations, supported vocabularies, and supported governance modes in valid machine-readable form.
2. **Given** a consumer that requires an unsupported schema version or unsupported operation, **When** it inspects capabilities, **Then** it can stop before attempting start or refresh.
3. **Given** a future additive Canon release within the same compatibility profile, **When** a consumer inspects capabilities, **Then** it can still discover the stable versioned contract without reading release notes first.

---

### User Story 4 - Ship A Trustworthy Consumer Contract In 0.35.0 (Priority: P4)

As a Canon maintainer, I want the adapter contract, release surfaces, and validation evidence to align so downstream consumers can rely on a supported product boundary rather than a stubbed local convention.

**Why this priority**: A public machine-facing surface is incomplete if it ships in code but remains undocumented, unversioned in release surfaces, or validated only through stubs.

**Independent Test**: A maintainer can inspect the completed slice and confirm version alignment, contract documentation, producer-side validation, and at least one live consumer-driven smoke against a real Canon binary.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects docs, version anchors, compatibility references, and `CHANGELOG.md`, **Then** they consistently describe the `0.35.0` adapter surface.
2. **Given** the completed implementation, **When** a maintainer inspects the validation record, **Then** it contains producer-side start, refresh, and capabilities contract checks plus an independent live consumer smoke.
3. **Given** existing human-oriented Canon users, **When** this feature ships, **Then** their current CLI workflows remain intact and are not replaced by adapter-only flows.

### Edge Cases

- A request is well-formed JSON but omits a domain field required for successful execution; Canon must return a blocked domain outcome with a reason code instead of treating the request as malformed protocol input.
- A `v1` consumer omits `adapter_schema_version`; Canon must treat the request as `v1` compatibility input instead of rejecting it for missing version metadata.
- A response attempts to combine `status: governed_ready` with an incomplete, rejected, or empty packet projection; Canon must prevent that state from escaping the adapter surface.
- A response attempts to combine `status: awaiting_approval` with an approval posture other than `requested`; Canon must normalize or reject that inconsistent state before the response escapes the adapter surface.
- A packet exists but still contains scaffold-only or missing-authored-body content; Canon must not project it as reusable.
- A consumer sends additive unknown request fields within a supported schema version; Canon should ignore them rather than fail compatibility.
- A consumer reuses a stale `run_ref`; Canon must produce a deterministic failed or blocked domain outcome with a machine-usable reason code.
- An adapter response would otherwise expose absolute paths from the local machine; Canon must normalize them to canonical workspace-relative refs before returning them.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST expose a first-class machine-facing CLI namespace for external governance consumers.
- **FR-002**: The machine-facing namespace MUST provide three operations in this slice: `start`, `refresh`, and `capabilities`.
- **FR-003**: The start and refresh operations MUST support JSON request and response exchange suitable for non-interactive consumers.
- **FR-004**: The capabilities operation MUST return machine-readable compatibility metadata without requiring a governed run to exist.
- **FR-005**: The adapter surface MUST be versioned under the public `adapter_schema_version` compatibility marker and MUST ship with `v1` as the initial supported schema.
- **FR-006**: Canon MUST publish the list of supported schema versions and MUST treat well-formed requests that omit `adapter_schema_version` as `v1` compatibility input.
- **FR-007**: For successful domain execution, start requests MUST provide the governance context Canon needs to govern work meaningfully, including request kind, correlation identifiers, user goal, workspace reference, mode, system context, risk, zone, and owner.
- **FR-008**: For successful domain execution, refresh requests MUST additionally provide the governed run reference being refreshed.
- **FR-009**: A well-formed request that omits one or more domain-required fields MUST return a blocked domain outcome with a machine-usable reason code rather than a protocol or parsing failure.
- **FR-010**: Canon MUST treat consumer correlation fields such as governance attempt identifiers and stage keys as opaque consumer-owned values rather than requiring Canon-native taxonomy.
- **FR-011**: The workspace reference in a domain request MUST bind to an accessible workspace; otherwise Canon MUST return a deterministic domain outcome or clearly classified protocol failure.
- **FR-012**: Consumer-supplied document paths MUST resolve inside the declared workspace boundary.
- **FR-013**: Additive unknown request fields within a supported schema version MUST NOT break compatibility.
- **FR-014**: Start MUST materialize or advance a governed Canon run using Canon's existing governance semantics and MUST return a run reference when that work succeeds.
- **FR-015**: Refresh MUST project the state of the referenced run and MUST NOT create an unrelated new run as a side effect.
- **FR-016**: Refresh MUST be idempotent when the underlying run and packet state have not changed.
- **FR-017**: The adapter response MUST remain flat in `v1` and MUST include schema version, lifecycle status, approval state, and message.
- **FR-018**: The `v1` response MAY additionally include run reference, packet reference, expected document refs, document refs, packet readiness, `missing_fields`, missing sections, headline, and reason code.
- **FR-019**: The response vocabulary for lifecycle status MUST be exactly `pending_selection`, `running`, `governed_ready`, `awaiting_approval`, `blocked`, `completed`, and `failed`.
- **FR-020**: The response vocabulary for approval state MUST be exactly `not_needed`, `requested`, `granted`, `rejected`, and `expired`.
- **FR-021**: The response vocabulary for packet readiness MUST be exactly `pending`, `incomplete`, `reusable`, and `rejected`.
- **FR-022**: `status: governed_ready` MUST imply `packet_readiness: reusable`, a present packet reference, and a non-empty document reference set.
- **FR-023**: If packet readiness is incomplete or rejected, Canon MUST NOT emit `status: governed_ready`.
- **FR-024**: If `status` is `awaiting_approval`, Canon MUST set `approval_state` to `requested`.
- **FR-025**: Canon MUST provide a non-empty `reason_code` for blocked and failed domain outcomes.
- **FR-026**: Canon SHOULD also provide a non-empty `reason_code` for `awaiting_approval` outcomes so consumers never need to infer control flow from message text alone.
- **FR-027**: `message` MUST always be present as the safe fallback summary for logs and diagnostics.
- **FR-028**: `headline` MAY be omitted, but when present it MUST remain semantically consistent with the domain outcome.
- **FR-029**: `packet_ref`, `expected_document_refs`, and `document_refs` MUST be canonical workspace-relative references.
- **FR-030**: Canon MUST NOT emit absolute or machine-specific path references in the adapter response.
- **FR-031**: The capabilities response MUST include Canon version, supported schema versions, supported operations, supported governance modes, and the exact published vocabularies for lifecycle status, approval state, and packet readiness.
- **FR-032**: The capabilities response MUST describe additive-compatibility expectations for supported schema versions.
- **FR-033**: Well-formed requests that reach Canon's domain logic MUST return exit code `0` for domain outcomes such as blocked, failed, approval-gated, running, completed, or governed-ready states.
- **FR-034**: Malformed JSON, unsupported schema versions, and CLI misuse MAY return non-zero exit codes and MUST be treated as protocol failures rather than domain outcomes.
- **FR-035**: The adapter surface MUST allow an external consumer to determine next action from JSON alone without reading `.canon/` internals or scraping human CLI prose.
- **FR-036**: The feature MUST preserve the behavior of Canon's existing human-oriented CLI commands unless a later scoped feature explicitly changes them.
- **FR-037**: The feature MUST reuse Canon's existing run, artifact, approval, and readiness semantics rather than introducing a parallel persistence or lifecycle system.
- **FR-038**: The feature MUST NOT introduce orchestration concerns such as session control, replanning, retry loops, cluster coordination, or assistant routing into Canon.
- **FR-039**: Producer-side contract validation MUST cover positive-path and blocked-path behavior for start, refresh, and capabilities.
- **FR-040**: Producer-side validation MUST cover the invariant that governed-ready responses always carry reusable packets and canonical workspace-relative refs.
- **FR-041**: Independent validation MUST include at least one live consumer-driven smoke against a real Canon binary rather than relying only on consumer-side shell stubs.
- **FR-042**: Cargo manifests, compatibility references, impacted docs, and `CHANGELOG.md` MUST align to `0.35.0` for this feature.
- **FR-043**: The task plan for this feature MUST include explicit tasks for version bump, impacted docs plus changelog, focused Rust coverage, `cargo clippy`, and `cargo fmt`.
- **FR-044**: Modified or newly created Rust files in this slice MUST receive focused automated validation coverage before the feature is complete.
- **FR-045**: Final validation for this slice MUST include clean formatting and lint closeout.
- **FR-046**: This feature MUST NOT change `.canon/` persistence layout, publish destinations, approval targets, or recommendation-only execution posture.

## Wire Contract v1 *(mandatory)*

### Command Surface

- `canon governance start --json`
- `canon governance refresh --json`
- `canon governance capabilities --json`

### Request Compatibility Rules

- `adapter_schema_version` is optional in `v1` requests; when omitted Canon MUST interpret the request as `v1`.
- `request_kind` values are `start` and `refresh` and MUST match the operation being invoked.
- Start and refresh requests are each carried by exactly one JSON object on standard input.
- For successful domain execution, Canon MUST accept the consumer-supplied fields Synod currently depends on: `request_kind`, `governance_attempt_id`, `stage_key`, `goal`, `workspace_ref`, `mode`, `system_context`, `risk`, `zone`, and `owner`; refresh also requires `run_ref`.
- `autopilot`, `packet_ref`, `bounded_context`, and `input_documents` remain optional request fields in `v1` and MUST NOT break compatibility when present.
- Additive unknown request fields within a supported schema version MUST be ignored for compatibility rather than treated as protocol failure.
- `governance_attempt_id` and `stage_key` are opaque consumer correlation fields, not Canon-owned taxonomy keys.

### Response Compatibility Rules

- The adapter response is exactly one flat JSON object on standard output.
- Required `v1` response fields are `adapter_schema_version`, `status`, `approval_state`, and `message`.
- Optional `v1` response fields are `run_ref`, `packet_ref`, `expected_document_refs`, `document_refs`, `packet_readiness`, `missing_fields`, `missing_sections`, `headline`, and `reason_code`.
- `status` values are exactly `pending_selection`, `running`, `governed_ready`, `awaiting_approval`, `blocked`, `completed`, and `failed`.
- `approval_state` values are exactly `not_needed`, `requested`, `granted`, `rejected`, and `expired`.
- `packet_readiness` values are exactly `pending`, `incomplete`, `reusable`, and `rejected`.
- `status: governed_ready` implies `packet_readiness: reusable`, a present `packet_ref`, and a non-empty `document_refs` set.
- `status: awaiting_approval` implies `approval_state: requested`.
- `reason_code` is required for `blocked` and `failed` outcomes and should also be present for `awaiting_approval` outcomes.
- `missing_fields`, when present, reports request-validation fields that blocked domain execution.
- `missing_sections`, when present, is reserved for packet projection gaps only.
- Refresh failures caused by unreadable or missing persisted packet contracts use `artifact_contract_unreadable` or `artifact_contract_missing`.
- `packet_ref`, `expected_document_refs`, and `document_refs` are canonical workspace-relative refs only.

### Capabilities Response Minimum

- The capabilities response MUST include `canon_version`, `supported_schema_versions`, `operations`, `supported_modes`, `status_values`, `approval_state_values`, and `packet_readiness_values`.
- The capabilities response SHOULD also include additive compatibility notes so consumers can distinguish stable `v1` guarantees from future optional extensions.

### Key Entities *(include if feature involves data)*

- **Governance Adapter Request**: The machine-facing request envelope that an external orchestrator submits to Canon to start or refresh governed work.
- **Governance Adapter Response**: The flat, versioned Canon-owned domain response that communicates lifecycle state, approval posture, packet state, and machine-usable reasoning for the outcome.
- **Governed Packet Projection**: The response subset that tells a consumer which packet Canon considers relevant, which documents are expected or materially present, and whether the packet is reusable.
- **Compatibility Profile**: The versioned contract that defines supported schema versions, additive-evolution rules, and the stable vocabularies consumers can rely on.
- **Reason Code**: A stable machine-usable explanation attached to blocked, failed, or approval-gated outcomes so consumers can branch safely without parsing prose.
- **Canonical Workspace-Relative Reference**: A stable path-like reference rooted at the declared workspace and safe to exchange across different machines without leaking absolute local paths.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In independent validation, an external orchestrator can start and refresh governed work against a real Canon binary and determine next action from one adapter response without reading `.canon/` files.
- **SC-002**: In producer-side validation, 100% of governed-ready responses also carry reusable packet readiness, a packet reference, and at least one document reference.
- **SC-003**: In validation evidence, 100% of blocked and failed domain outcomes include a non-empty machine-usable reason code, and approval-gated outcomes do the same wherever Canon surfaces them.
- **SC-004**: In validation evidence, 100% of packet and document references returned by the adapter surface are canonical workspace-relative refs rather than machine-dependent absolute paths.
- **SC-005**: A maintainer can inspect one capabilities response and determine supported schema versions, supported operations, and supported outcome vocabularies in under one minute without consulting release notes.
- **SC-006**: Release-facing docs, compatibility references, and `CHANGELOG.md` consistently describe the `0.35.0` adapter surface and no targeted regression is observed in existing human CLI flows.
- **SC-007**: Every modified or newly created Rust file in the slice is exercised by focused automated validation and the final closeout is clean on `cargo fmt` and `cargo clippy` expectations.

## Validation Plan *(mandatory)*

- **Structural validation**: spec quality checklist closeout, version-surface consistency checks, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and any impacted contract-surface documentation checks.
- **Logical validation**: focused producer-side contract tests for start, refresh, capabilities, domain-blocked validation, governed-ready versus reusable-packet invariants, canonical workspace-relative references, and backward-compatible capability reporting.
- **Independent validation**: a live consumer-driven smoke using Synod's adapter expectations against a real Canon binary, plus an adversarial review pass on blocked, failed, and approval-gated outcomes.
- **Evidence artifacts**: `specs/035-governance-adapter-surface/validation-report.md`, `specs/035-governance-adapter-surface/decision-log.md`, focused contract-test output, and the recorded consumer-driven smoke evidence.

## Decision Log *(mandatory)*

- **D-001**: Ship a first-class machine-facing surface as a new feature instead of treating it as fallout from artifact-shape work, **Rationale**: the missing product boundary is external governance compatibility, not packet prose.
- **D-002**: Define `governed_ready` as a strict reusable-packet outcome, **Rationale**: downstream consumers cannot safely continue if readiness is stronger than the packet state.
- **D-003**: Model missing domain context as blocked outcomes rather than parser failures, **Rationale**: external orchestrators need deterministic machine control flow for well-formed requests.
- **D-004**: Standardize canonical workspace-relative refs, **Rationale**: machine-facing contracts must remain stable across machines and must not leak local absolute paths.
- **D-005**: Publish supported schema versions in capabilities from the first release, **Rationale**: compatibility discovery is required for a public consumer contract to evolve safely.

## Non-Goals

- Turning Canon into a session runtime, workflow engine, planner, retry coordinator, or multi-agent orchestrator.
- Replacing or redesigning Canon's existing human-oriented CLI around adapter-only workflows.
- Adding an HTTP API, daemon, or remote orchestration protocol in this slice.
- Changing `.canon/` layout, run identifiers, publish destinations, approval targets, or recommendation-only operational semantics.
- Introducing consumer-specific stage taxonomies or packet models that bypass Canon's existing governance model.

## Assumptions

- Synod is the first consumer that matters for this surface, but the Canon-owned contract should remain general enough for other external orchestrators later.
- Canon's current run, artifact, approval, and readiness semantics are sufficient foundations for a machine-facing projection without requiring new persistence schema.
- A flat `v1` response is the lowest-risk compatibility profile for initial consumer adoption.
- Additive request metadata may grow over time, so supported schema versions must tolerate unknown optional fields within a compatibility profile.
- `0.35.0` is the intended release anchor for this slice, so versioned docs, compatibility references, and `CHANGELOG.md` updates are in scope.
