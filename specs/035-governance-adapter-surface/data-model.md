# Data Model: Governance Adapter Surface For External Orchestrators

## Entity: Governance Adapter Request

- **Purpose**: Represents the machine-facing request envelope an external
  orchestrator submits to Canon to start or refresh governed work.
- **Fields**:
  - `adapter_schema_version`: optional compatibility marker; omitted means `v1`
  - `request_kind`: `start` or `refresh`
  - `governance_attempt_id`: opaque consumer correlation identifier
  - `stage_key`: opaque consumer-owned stage or slice identifier
  - `goal`: user-facing goal summary for the governed attempt
  - `workspace_ref`: consumer-declared workspace root for bounded resolution
  - `autopilot`: consumer hint about orchestrator posture
  - `mode`: Canon governance mode requested for successful domain execution
  - `system_context`: `new` or `existing` system binding for successful domain
    execution
  - `risk`: governance risk posture required for successful domain execution
  - `zone`: usage-zone posture required for successful domain execution
  - `owner`: declared human owner required for successful domain execution
  - `run_ref`: governed run reference required for successful refresh execution
  - `packet_ref`: optional prior packet reference hint
  - `bounded_context`: optional narrower context describing read targets,
    stage-brief refs, and reused packets
  - `input_documents`: optional list of consumer-declared input documents
- **Validation Rules**:
  - A well-formed request may omit domain-required fields, but that omission
    must produce a blocked domain outcome rather than a protocol failure.
  - `workspace_ref` must resolve to an accessible workspace boundary.
  - Consumer-supplied paths must remain inside the declared workspace.
  - Unknown additive fields within a supported schema version are ignored.

## Entity: Governance Bounded Context

- **Purpose**: Carries consumer-supplied context narrowing hints without
  changing Canon's governance ownership.
- **Fields**:
  - `read_targets`: repository-relative paths the consumer believes are in
    scope
  - `stage_brief_ref`: optional canonical ref to a primary stage brief
  - `reused_packets`: previously emitted packet refs that the consumer wants to
    carry forward
- **Validation Rules**:
  - Bounded-context fields are advisory in `v1`; unsupported hints must not
    break compatibility.
  - Any referenced path must remain inside the workspace boundary.

## Entity: Governance Adapter Response

- **Purpose**: Represents the flat, versioned Canon-owned domain response
  returned to an external orchestrator.
- **Fields**:
  - `adapter_schema_version`: compatibility profile used for the response
  - `status`: lifecycle outcome vocabulary
  - `approval_state`: approval posture vocabulary
  - `message`: always-present machine-safe fallback summary
  - `run_ref`: optional Canon run reference
  - `packet_ref`: optional canonical packet reference
  - `expected_document_refs`: optional expected packet document refs
  - `document_refs`: optional materially present packet document refs
  - `packet_readiness`: optional packet readiness vocabulary
  - `missing_fields`: optional request-validation field names for blocked
    domain outcomes
  - `missing_sections`: optional unresolved packet gaps
  - `headline`: optional shorter summary consistent with `message`
  - `reason_code`: machine-usable domain explanation for blocked, failed, or
    approval-gated outcomes
- **Consistency Rules**:
  - `status: governed_ready` requires `packet_readiness: reusable`, a present
    `packet_ref`, and a non-empty `document_refs` set.
  - `status: awaiting_approval` requires `approval_state: requested`.
  - `blocked` and `failed` require a non-empty `reason_code`.
  - `missing_fields` is reserved for request-validation failures and must not
    be used to describe packet reuse gaps.
  - Refresh responses use `artifact_contract_unreadable` or
    `artifact_contract_missing` when persisted packet metadata cannot be
    trusted.
  - Packet and document refs must be canonical workspace-relative refs.

## Entity: Governed Packet Projection

- **Purpose**: Captures the portion of the response that lets a consumer reason
  about packet reuse and artifact lineage.
- **Fields**:
  - `packet_ref`: canonical workspace-relative ref to the packet root
  - `expected_document_refs`: document refs Canon expects for a complete packet
  - `document_refs`: document refs Canon currently considers present and usable
  - `packet_readiness`: `pending`, `incomplete`, `reusable`, or `rejected`
  - `missing_sections`: unresolved packet gaps blocking reuse
  - `headline`: optional reader-facing summary consistent with the domain state
- **Validation Rules**:
  - `reusable` requires at least one materially present document ref.
  - `incomplete` or `rejected` must never be paired with
    `status: governed_ready`.

## Entity: Governance Capabilities Descriptor

- **Purpose**: Describes Canon's published adapter compatibility surface before
  a consumer attempts live governance operations.
- **Fields**:
  - `canon_version`: producer version
  - `supported_schema_versions`: published compatibility profiles, including
    `v1`
  - `operations`: supported machine-facing operations
  - `supported_modes`: governance modes available through the adapter surface
  - `status_values`: exact lifecycle vocabulary
  - `approval_state_values`: exact approval vocabulary
  - `packet_readiness_values`: exact packet readiness vocabulary
  - `compatibility_notes`: additive-evolution notes for supported schema
    versions

## Entity: Reason Code

- **Purpose**: Provides a stable machine-readable explanation for domain
  outcomes that do not represent straight-line success.
- **Fields**:
  - `category`: domain reason family such as missing context, approval needed,
    invalid workspace, unsupported mode, run lookup failure, or packet
    inconsistency
  - `stability`: published within the compatibility profile so consumers can
    branch safely
  - `message_binding`: human-readable message paired with the code

## Relationships

- Each **Governance Adapter Request** may yield one **Governance Adapter
  Response**.
- Each **Governance Adapter Response** may contain zero or one **Governed
  Packet Projection**.
- Each **Governance Adapter Response** may include zero or one **Reason Code**.
- Each **Governance Capabilities Descriptor** publishes the vocabularies that
  constrain valid **Governance Adapter Response** values.

## State Semantics

- `start` may yield blocked, failed, running, awaiting-approval,
  governed-ready, or completed outcomes depending on Canon's domain state.
- `refresh` reprojects an existing governed run and must preserve idempotent
  semantics when the underlying run state has not changed.
- `governed_ready` is a strict terminal-quality projection for reusable packet
  output, not a generic synonym for "work progressed".