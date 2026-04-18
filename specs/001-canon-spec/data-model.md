# Data Model: Canon v0.1

## 1. Core Enumerations

### Mode

The domain must define all of these as first-class variants:

- `requirements`
- `discovery`
- `system-shaping` (internal enum variant: `Greenfield`)
- `brownfield-change`
- `architecture`
- `implementation`
- `refactor`
- `verification`
- `review`
- `pr-review`
- `incident`
- `migration`

### Supporting Enums

- `RiskClass`: `LowImpact`, `BoundedImpact`, `SystemicImpact`
- `UsageZone`: `Green`, `Yellow`, `Red`
- `ModeEmphasis`: `AnalysisHeavy`, `ExecutionHeavy`, `ReviewHeavy`
- `ImplementationDepth`: `Full`, `ContractOnly`, `Skeleton`
- `GateKind`: `Exploration`, `BrownfieldPreservation`, `Architecture`, `Risk`,
  `ReviewDisposition`, `ReleaseReadiness`, `ImplementationReadiness`,
  `IncidentContainment`, `MigrationSafety`
- `GateStatus`: `Pending`, `Passed`, `Blocked`, `NeedsApproval`, `Overridden`
- `VerificationLayer`: `SelfCritique`, `AdversarialCritique`, `PeerReview`,
  `ArchitecturalReview`
- `RunState`: `Draft`, `ContextCaptured`, `Classified`, `Contracted`, `Gated`,
  `Executing`, `AwaitingApproval`, `Verifying`, `Completed`, `Blocked`,
  `Failed`, `Aborted`, `Superseded`
- `AdapterKind`: `Filesystem`, `Shell`, `CopilotCli`, `McpStdio`
- `CapabilityKind`: `ReadRepository`, `WriteArtifact`, `ExecReadOnlyCommand`,
  `ExecMutatingCommand`, `InvokeAiGeneration`, `InvokeAiCritique`,
  `InvokeStructuredTool`
- `SideEffectClass`: `ReadOnly`, `ArtifactWrite`, `WorkspaceMutation`,
  `ExternalStateChange`

## 2. Primary Entities

| Entity | Key Fields | Why It Exists |
| --- | --- | --- |
| `ModeProfile` | mode, purpose, emphasis, artifact families, gate profile, verification baseline, allowed adapters, implementation depth | Captures the semantics of a mode without turning it into a generic workflow |
| `MethodDefinition` | mode, ordered steps, stop conditions, exit criteria | Defines the bounded sequence of work for a mode |
| `StepDefinition` | step id, label, required inputs, produced outputs, stop conditions | Makes runs resumable and inspectable at step boundaries |
| `PolicySet` | version, risk rules, zone rules, gate rules, verification rules, adapter rules | Centralizes governance without embedding it in prompts |
| `RunContext` | repo root, git head, selected inputs, excluded paths, owner, adapter availability, parent run, input digests, input snapshot refs | Captures the bounded context of a run |
| `Run` | run id, state, mode, risk, zone, policy version, method version, pointers to contract and evidence | The durable execution object for the engine |
| `ArtifactRequirement` | artifact key, file name, format, required sections, mandatory conditions, gate dependencies | Expresses what must exist before progress is allowed |
| `ArtifactContract` | mode, risk, zone, artifact requirements, required verification, required approvals | Freezes the artifact burden for the run |
| `ArtifactRecord` | artifact key, actual path, format, checksum, producing step, validation status | Tracks each emitted artifact and links it to the contract |
| `GateEvaluation` | gate, status, inputs, blockers, approval requirement, evaluator, timestamp | Persists whether a gate passed and why |
| `DecisionRecord` | decision id, run id, context, alternatives, rationale, consequences, unresolved questions, owner | Makes consequential choices durable and reviewable |
| `VerificationRecord` | layer, actor, target artifacts, findings, disposition, evidence paths | Preserves independent challenge evidence |
| `ApprovalRecord` | gate, approver, decision, rationale, timestamp | Captures explicit approval evidence and overrides |
| `AdapterInvocation` | invocation id, adapter kind, capability, purpose, status, trace path | Makes external tool usage auditable |
| `StopCondition` | trigger, severity, action, owner requirement | Forces the engine to halt when assumptions become unsafe |
| `ExitCriteria` | required gates, required artifacts, required verification, required approvals | Defines when a run may complete |

### User-Facing Summary Projections

- `RunSummary` and `StatusSummary` may expose the persisted `owner` from the run manifest so user-facing summaries show who owns the run without re-deriving identity from host state.
- `ApprovalSummary` may expose `approved_by` and `recorded_at` from the persisted `ApprovalRecord` so Canon surfaces who actually recorded the approval and when.
- Summary projections must not invent `started_by` or `initiated_by` until Canon persists a distinct actor for that concept.

## 3. Relationships

- One `ModeProfile` belongs to exactly one `Mode`.
- One `MethodDefinition` belongs to exactly one `ModeProfile`.
- One `Run` references exactly one `RunContext`, one effective `PolicySet`, and
  one `ArtifactContract`.
- One `ArtifactContract` contains many `ArtifactRequirement` entries.
- One `Run` produces many `ArtifactRecord`, `GateEvaluation`,
  `VerificationRecord`, and `AdapterInvocation` records.
- One `Run` may link to zero or many `DecisionRecord` entries.
- One `Run` may have zero or one `parent_run_id`.
- One `ApprovalRecord` is tied to exactly one gate or explicit override event.
- `VerificationRecord` targets one or many `ArtifactRecord` entries.

## 4. Mode Artifact Family Catalog

| Mode | Artifact Families | Implementation Depth |
| --- | --- | --- |
| `requirements` | problem framing, constraints, options, tradeoffs, scope cuts, decision checklist | Full |
| `discovery` | discovery brief, assumptions register, evidence log, unknowns register, discovery summary | ContractOnly |
| `system-shaping` | system intent, domain map, architecture options, boundary decisions, delivery plan | ContractOnly |
| `brownfield-change` | system map or slice, legacy invariants, change surface, implementation plan, validation strategy, decision record | Full |
| `architecture` | invariants, boundary map, architecture options, tradeoffs, decision record, risk memo | ContractOnly |
| `implementation` | execution brief, task bundle, contract checklist, change log, verification hooks, completion record | Skeleton |
| `refactor` | equivalence criteria, preserved surface, untangling plan, rollback notes, validation strategy | ContractOnly |
| `verification` | invariants checklist, contract matrix, adversarial review, verification report, unresolved findings | ContractOnly |
| `review` | review brief, boundary assessment, missing evidence, decision impact, review disposition | ContractOnly |
| `pr-review` | PR analysis, boundary check, duplication check, contract drift, missing tests, decision impact, review summary | Full |
| `incident` | incident frame, hypothesis log, blast-radius map, containment plan, incident decision record, follow-up verification | Skeleton |
| `migration` | source-target map, compatibility matrix, sequencing plan, fallback plan, migration verification report, decision record | Skeleton |

## 5. Run State Transitions

| From | To | Trigger |
| --- | --- | --- |
| `Draft` | `ContextCaptured` | context captured and persisted |
| `ContextCaptured` | `Classified` | risk and zone classified |
| `Classified` | `Contracted` | artifact contract written |
| `Contracted` | `Gated` | initial required gates passed |
| `Gated` | `Executing` | execution or artifact generation step begins |
| `Executing` | `AwaitingApproval` | gate or policy requires human approval |
| `Executing` | `Verifying` | execution phase complete and verification begins |
| `AwaitingApproval` | `Executing` | approval granted |
| `AwaitingApproval` | `Blocked` | approval rejected or timed out |
| `Verifying` | `Completed` | exit criteria satisfied |
| Any active state | `Blocked` | missing artifact, failed gate, stale context, or unavailable mandatory adapter |
| Any active state | `Failed` | unrecoverable persistence or execution failure |
| Any active state | `Aborted` | explicit user stop |
| Any prior run | `Superseded` | new forked run replaces it as the active lineage |

## 6. Persistence Schema Notes

- Run metadata lives in TOML for readability and low-friction editing.
- Trace streams live in JSONL because they are append-friendly and easy to
  inspect or post-process.
- Markdown artifacts remain plain Markdown with required headings enforced by
  the contract. Metadata such as schema version and checksum stays in the run
  manifest.
- JSON and YAML artifacts are permitted for machine-oriented reports, but they
  still appear in the artifact contract and gain `ArtifactRecord` entries.

## 7. Resumability Model

Resumability depends on four persisted anchors:

1. `RunState`
2. step completion records
3. gate evaluations
4. input fingerprints, content digests, snapshot refs, and repository head

If any referenced input changes, the engine does not guess. It blocks the run
and requires one of three explicit actions:

- reuse previous artifacts as-is
- refresh artifacts from the affected step onward
- fork a new run with a `parent_run_id`

## 8. Approval and Decision Memory

`DecisionRecord` is mandatory when:

- risk is `SystemicImpact`
- a gate override occurs
- a boundary or invariant changes
- a review identifies an unrecorded structural consequence
- a human accepts known risk

`ApprovalRecord` is mandatory when:

- a gate returns `NeedsApproval`
- a run is systemic or red-zone
- an override is used
- release readiness depends on named ownership

