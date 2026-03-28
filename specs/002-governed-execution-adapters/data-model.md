# Data Model: Governed Execution Adapters

## 1. Enumerations

### Existing Enums Extended

- `Mode`: unchanged; all current Canon modes remain first-class
- `RiskClass`: `LowImpact`, `BoundedImpact`, `SystemicImpact`
- `UsageZone`: `Green`, `Yellow`, `Red`
- `RunState`: unchanged, but `AwaitingApproval` now also covers invocation
  approvals

### New Enums

- `InvocationOrientation`: `Context`, `Generation`, `Validation`,
  `ArtifactDerivation`
- `CapabilityKind`: `ReadRepository`, `InspectDiff`, `ReadArtifact`,
  `EmitArtifact`, `RunCommand`, `GenerateContent`, `ProposeWorkspaceEdit`,
  `CritiqueContent`, `ValidateWithTool`, `InvokeStructuredTool`,
  `ExecuteBoundedTransformation`
- `MutabilityClass`: `ReadOnly`, `ArtifactWrite`, `BoundedWorkspaceWrite`,
  `BroadWorkspaceWrite`, `ExternalStateChange`
- `PolicyDecisionKind`: `Allow`, `AllowConstrained`, `NeedsApproval`, `Deny`
- `ToolOutcomeKind`: `Succeeded`, `PartiallySucceeded`, `Failed`, `Denied`,
  `AwaitingApproval`, `RecommendationOnly`
- `TrustBoundaryKind`: `LocalDeterministic`, `LocalProcess`, `AiReasoning`,
  `RemoteStructuredTool`
- `LineageClass`: `NonGenerative`, `AiVendorFamily`, `HumanReview`
- `PayloadRetentionLevel`: `SummaryOnly`, `SummaryWithDigest`,
  `SummaryWithRetainedPayload`
- `EvidenceDisposition`: `Supporting`, `Blocking`, `NeedsDisposition`

## 2. Primary Entities

| Entity | Key Fields | Why It Exists |
| --- | --- | --- |
| `ExecutionAdapterDescriptor` | adapter kind, availability, trust boundary, supported capabilities | makes adapter identity explicit and reviewable |
| `AdapterCapability` | capability kind, orientation, mutability, default lineage, allowed surfaces | keeps capability semantics typed without exploding variants |
| `InvocationRequest` | request id, run id, mode, risk, zone, adapter, capability, orientation, scope, owner state | unit of governance before execution |
| `InvocationConstraintSet` | path restrictions, command profile, retention limits, recommendation-only flag, patch disable flag | captures enforceable policy constraints |
| `InvocationPolicyDecision` | decision kind, constraints, approval requirement, rationale, policy refs | persisted authorization result |
| `InvocationAttempt` | request id, attempt number, timestamps, executor metadata, outcome ref | supports retries and resume |
| `InvocationTrace` | request summary, decision summary, attempt summary, linked refs | append-friendly work-in-motion evidence |
| `ToolOutcome` | kind, exit code, summary, payload refs, candidate artifact refs | normalized result across adapters |
| `DeniedInvocation` | request id, denial rationale, blocking refs, recorded at | denial as evidence |
| `GenerationPath` | path id, request ids, lineage classes, derived refs | records how consequential outputs were produced |
| `ValidationPath` | path id, request ids, lineage classes, verification refs, independence assessment | records challenge to generation outputs |
| `ValidationIndependenceAssessment` | target path id, sufficiency, rationale, supporting refs | makes independence explicit rather than implied |
| `EvidenceBundle` | run id, generation paths, validation paths, trace refs, artifact refs, decision refs, approval refs | indexed run-level evidence summary |

## 3. Relationships

- One `Run` has zero or many `InvocationRequest` records.
- One `InvocationRequest` has exactly one latest `InvocationPolicyDecision`.
- One `InvocationRequest` has zero or many `InvocationAttempt` records.
- One `InvocationAttempt` may produce zero or one `ToolOutcome`.
- One `ToolOutcome` may produce zero or many derived artifacts.
- One `EvidenceBundle` belongs to exactly one `Run`.
- One `EvidenceBundle` references many invocation traces, artifacts, decisions,
  approvals, and verification records.
- One `GenerationPath` and one or more `ValidationPath` entries may target the
  same consequential artifact or recommendation.
- One `ApprovalRecord` may target a gate or a specific invocation request.

## 4. State Transitions

### Invocation Request Lifecycle

| From | To | Trigger |
| --- | --- | --- |
| `Prepared` | `Denied` | policy decision is `Deny` |
| `Prepared` | `AwaitingApproval` | policy decision is `NeedsApproval` |
| `Prepared` | `Dispatchable` | policy decision is `Allow` or `AllowConstrained` |
| `Dispatchable` | `Running` | adapter dispatch begins |
| `Running` | `Succeeded` | normalized outcome succeeds |
| `Running` | `PartiallySucceeded` | adapter output exists but postconditions fail |
| `Running` | `Failed` | adapter execution fails |
| `AwaitingApproval` | `Dispatchable` | approval granted and context remains valid |
| `AwaitingApproval` | `Superseded` | context or constraints change before resume |

### Run Impact

- denied invocation may leave run `Executing`, `Blocked`, or `Completed`
  depending on whether the invocation was mandatory or optional
- approval-gated invocation moves run to `AwaitingApproval`
- insufficient validation independence blocks readiness and moves run to
  `Blocked` or keeps it in `Verifying`

## 5. Persistence Notes

- `.canon/traces/<run-id>.jsonl` remains append-only and contains one event per
  decision or attempt transition
- `.canon/runs/<run-id>/invocations/<request-id>/` stores request, decision,
  attempt manifests, and optional payload refs
- `.canon/runs/<run-id>/evidence.toml` stores path summaries and links
- raw payload retention is optional and policy-limited

## 6. Compatibility Notes

- Existing run manifests remain readable
- `links.toml` gains evidence references but does not drop current fields
- Artifact manifests gain provenance links back to invocation request ids
