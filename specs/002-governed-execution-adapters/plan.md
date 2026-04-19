# Implementation Plan: Governed Execution Adapters

**Branch**: `002-governed-execution-adapters` | **Date**: 2026-03-28 | **Spec**: [specs/002-governed-execution-adapters/spec.md](./spec.md)
**Input**: Feature specification from `specs/002-governed-execution-adapters/spec.md`

## Summary

This increment adds a governed execution layer on top of Canon's existing
typed modes, gate model, artifact contracts, approvals, and local persistence.
The implementation centers on typed invocation requests, pre-execution policy
decisions, durable invocation traces, and evidence bundles that link tool
usage to run state, artifacts, decisions, approvals, and verification. The
delivery stays additive: Canon remains a local-first Rust CLI with three
serious runtime slices for `requirements`, `brownfield-change`, and
`pr-review`, while the core domain becomes capable of governing real external
tool invocation rather than only recording artifact outcomes.

## Governance Context

**Execution Mode**: `architecture` for additive runtime design over the current
Canon baseline  
**Risk Classification**: `SystemicImpact` because this increment changes how Canon
authorizes, constrains, records, and validates external execution across the
core runtime  
**Scope In**: invocation domain model, adapter capability typing,
pre-execution policy evaluation, constrained execution, invocation and evidence
persistence, generation versus validation path tracking, adapter integration
for `requirements`, `brownfield-change`, and `pr-review`, CLI inspection
surfaces, and validation strategy for the new governed execution layer  
**Scope Out**: IDE integrations, distributed execution, remote control plane,
generic plugin ecosystem, autonomous multi-agent orchestration, full prompt
transcript retention, and broad runtime redesign unrelated to invocation
governance

**Invariants**:

- Canon-specific mode semantics remain strongly typed in code and continue to
  drive policy, gates, and exit criteria.
- No external invocation is evaluated until mode, risk, zone, policy context,
  and any required ownership boundary are resolved.
- Denied, constrained, approval-gated, and executed invocations all leave
  durable evidence.
- Consequential generation and validation remain separately reviewable and may
  not collapse into the same reasoning path when policy requires independence.
- `.canon/` remains the local system of record for runs, traces, approvals,
  decisions, and execution evidence.

**Decision Log**: [specs/002-governed-execution-adapters/decision-log.md](./decision-log.md)
**Validation Ownership**: adapters and orchestration produce invocation
outcomes; policy evaluation, evidence assembly, gate evaluation, tests,
non-generative validation tools, adversarial critique, and human review
challenge those outcomes through separate recorded paths  
**Approval Gates**: named invocation approval targets for constrained or
systemic-impact capabilities, existing mode gates, and explicit human ownership for
systemic or red-zone actions where policy requires it

## 1. Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: existing `clap`, `serde`, `serde_json`, `serde_yaml`,
`toml`, `thiserror`, `time`, `uuid`, `tracing`, `tracing-subscriber`, and
test-only `assert_cmd`, `predicates`, `tempfile`, `insta`; add `blake3` for
content digests on retained invocation payloads and evidence references  
**Storage**: local filesystem only under `.canon/`; TOML for run and approval
manifests, JSONL for invocation trace streams, Markdown/JSON/YAML for derived
artifacts and evidence summaries  
**Testing**: `cargo test` locally and `cargo nextest run` in CI; unit tests for
policy evaluation, constraint enforcement, lineage rules, and evidence
assembly; integration tests for the three governed execution slices plus resume
behavior  
**Target Platform**: macOS, Linux, and Windows; adapter execution stays
blocking and platform-neutral, with OS-specific command differences isolated in
adapter modules and test fixtures  
**Project Type**: native CLI, single binary, local-first governance runtime  
**Existing System Touchpoints**: `canon-cli` commands, `canon-engine`
orchestration, `canon-adapters` request/dispatch code, `.canon/` runtime
persistence, git working tree inspection, shell process execution, optional
Copilot CLI presence, optional MCP stdio tooling  
**Performance Goals**: policy evaluation and invocation recording under 10ms
per request excluding external tool latency; trace append under 5ms per event;
`status` and `inspect` remain responsive by reading manifests rather than
replaying full tool output  
**Constraints**: no async runtime, no job queue, no plugin DSL, no remote
service, no silent policy bypass, no dependence on external AI connectivity for
core CLI behavior, and no storage of raw prompts by default  
**Scale/Scope**: one repository per run; dozens of invocation requests and
hundreds of trace events per run; payload retention capped by policy and linked
through digests and summaries rather than unlimited raw capture

## 2. Constitution Check

### Pre-Design Gate

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Highest-risk approval checkpoints are named
- [x] No constitution deviations are required before research

### Post-Design Re-Check

- [x] The plan remains additive to Canon's existing architecture
- [x] Mode semantics stay code-owned and do not collapse into a generic engine
- [x] Invocation governance is evaluated before execution rather than after it
- [x] Denials, constraints, approvals, and outcomes become durable evidence
- [x] Validation independence is modeled explicitly for consequential work
- [x] `.canon/` remains the local system of record
- [x] No unjustified weakening of governance or traceability was introduced

**Result**: PASS. No constitution violations or justified exceptions are
required by this design.

## 3. Current Baseline and Gap

Canon already has:

- typed `Mode`, `RiskClass`, `UsageZone`, `GateKind`, `RunState`, and
  `VerificationLayer`
- a local-first run model persisted under `.canon/`
- artifact contracts and gate outcomes for `requirements`,
  `brownfield-change`, and `pr-review`
- gate-scoped approval records, gate-oriented `resume` behavior, and JSONL
  trace streams for persisted filesystem and shell activity
- a minimal adapter model with `AdapterRequest`, `CapabilityKind`,
  `SideEffectClass`, concrete filesystem and shell adapters, coarse mutation
  blocking, and placeholder `CopilotCliAdapter` / `McpStdioAdapter` structs

The current gap is structural, not cosmetic:

- adapter capabilities are too coarse and do not distinguish orientation,
  constraint surface, or reasoning lineage
- policy evaluation is mostly a mutation yes/no check, not a pre-execution
  authorization model
- denied or approval-gated requests are not yet first-class invocation objects
- traces record low-level activity, but not a full invocation lifecycle with
  request, decision, outcome, and evidence links
- generation versus validation separation is modeled at the run level, not at
  the actual invocation-path level
- artifacts are still primarily emitted by mode renderers rather than derived
  from governed execution evidence

This increment closes that gap without replacing the existing runtime.

## 4. Architectural Decisions for Governed Execution

| Decision Area | Choice | Why This Choice |
| --- | --- | --- |
| Invocation domain shape | Add a dedicated execution domain inside `canon-engine` | Keeps Canon semantics in the core runtime instead of scattering them across adapters |
| Capability typing | Use explicit enums plus orthogonal metadata, not one-off capability variants or plugin traits | Prevents capability explosion while staying typed and inspectable |
| Constraint enforcement | Represent constraints as typed structs attached to a policy decision and carried into adapter execution | Makes “allow with constraints” both enforceable and reviewable |
| Policy ownership | Keep mode semantics, precedence rules, and independence logic in code; keep allowlists and adapter matrices in TOML | Preserves Canon’s semantics while allowing controlled policy tuning |
| Persistence | Keep JSONL trace streams for append-only event history, add per-invocation manifests and a run evidence bundle for indexed inspection | Avoids replay-heavy inspection and keeps evidence durable |
| Approval model | Approval attaches to invocation request ids or gates, not opaque adapter calls | Makes approval and resume semantics deterministic |
| Validation independence | Evaluate independence from typed lineage and trust-boundary metadata, not from adapter names alone | Same tool family must not masquerade as independent challenge |
| MCP staging | Model `McpStdio` cleanly in the domain and policy, but keep runtime delivery contract-only unless it can reuse the same invocation pipeline cleanly | Avoids over-generalizing around a not-yet-proven adapter |

### Proposed Project Structure

```text
specs/002-governed-execution-adapters/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── cli-contract.md
│   └── runtime-evidence-contract.md
└── tasks.md

crates/
├── canon-cli/
│   └── src/
│       ├── commands/
│       │   ├── approve.rs
│       │   ├── inspect.rs
│       │   ├── resume.rs
│       │   ├── run.rs
│       │   └── status.rs
│       └── output.rs
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── execution.rs         # new invocation and evidence types
│       │   ├── policy.rs            # extended policy set and approval semantics
│       │   ├── run.rs               # run links to evidence bundle and pending invocations
│       │   └── verification.rs      # extended validation path records
│       ├── orchestrator/
│       │   ├── invocation.rs        # pre-execution policy evaluation and dispatch
│       │   ├── evidence.rs          # build/update evidence bundles
│       │   ├── gatekeeper.rs
│       │   ├── resume.rs
│       │   └── service.rs
│       ├── persistence/
│       │   ├── invocations.rs       # per-invocation manifests and payload refs
│       │   ├── manifests.rs
│       │   ├── store.rs
│       │   └── traces.rs            # append/read structured invocation events
│       └── modes/
│           ├── requirements.rs
│           ├── brownfield_change.rs
│           └── pr_review.rs
└── canon-adapters/
    └── src/
        ├── capability.rs            # richer capability and trust-boundary typing
        ├── dispatcher.rs
        ├── filesystem.rs
        ├── shell.rs
        ├── copilot_cli.rs
        └── mcp_stdio.rs

defaults/
└── policies/
    ├── adapters.toml                # expanded adapter/capability matrices
    └── verification.toml            # lineage and independence policy
```

**Structure Decision**: keep the existing three-crate workspace and extend it
through a new execution domain and persistence slice inside `canon-engine`.
Adapters remain concrete modules in `canon-adapters`; no new crate or plugin
host is introduced.

## 5. Domain Model Changes

### New Core Enums

- `InvocationOrientation`: `Context`, `Generation`, `Validation`,
  `ArtifactDerivation`
- `MutabilityClass`: `ReadOnly`, `ArtifactWrite`, `BoundedWorkspaceWrite`,
  `BroadWorkspaceWrite`, `ExternalStateChange`
- `PolicyDecisionKind`: `Allow`, `AllowConstrained`, `NeedsApproval`, `Deny`
- `ToolOutcomeKind`: `Succeeded`, `Failed`, `PartiallySucceeded`, `Denied`,
  `AwaitingApproval`, `RecommendationOnly`
- `TrustBoundaryKind`: `LocalDeterministic`, `LocalProcess`, `AiReasoning`,
  `RemoteStructuredTool`
- `LineageClass`: `NonGenerative`, `AiVendorFamily`, `HumanReview`
- `EvidenceDisposition`: `Supporting`, `Blocking`, `NeedsDisposition`

### New or Strengthened Entities

| Entity | Key Fields | Integration |
| --- | --- | --- |
| `ExecutionAdapterDescriptor` | adapter kind, trust boundary, supported capabilities, availability | replaces implicit adapter assumptions with inspectable metadata |
| `AdapterCapability` | capability kind, orientation, mutability class, default trust boundary, allowed surfaces | extends current `CapabilityKind` without exploding into one-off variants |
| `InvocationRequest` | request id, run id, mode, risk, zone, adapter, capability, requested scope, orientation, lineage hint, requested output policy | evaluated before any external execution |
| `InvocationConstraintSet` | path scope, command profile id, max bytes, recommendation-only flag, apply-disabled flag, payload retention level | enforces “allow with constraints” |
| `InvocationPolicyDecision` | decision kind, constraints, approval requirement, rationale, effective policy refs | persisted before dispatch |
| `InvocationAttempt` | attempt number, request id, started/finished timestamps, executor metadata, outcome reference | supports retries and resume |
| `InvocationTrace` | request, decision, attempt summary, outcome summary, linked artifacts, linked decisions, linked approvals | durable per-invocation record |
| `ToolOutcome` | outcome kind, summary, exit code, payload refs, artifact candidates, evidence refs | normalized adapter result for gates |
| `DeniedInvocation` | request id, denial rationale, blocking policy refs, optional decision link | makes denials first-class evidence |
| `GenerationPath` | ordered request ids, lineage classes, derived artifacts | attached to consequential outputs |
| `ValidationPath` | ordered request ids, lineage classes, validation layers, independence assessment | attached to gate evaluation |
| `EvidenceBundle` | run id, generation paths, validation paths, trace refs, decision refs, approval refs, artifact refs | indexed run-level evidence object |

### Integration with Existing Canon Types

- `Run` gains references to pending invocation ids, evidence bundle location,
  and last lineage assessment.
- `RunState` keeps existing states; `AwaitingApproval` now covers invocation
  approvals as well as gate approvals.
- `ArtifactContract` remains, but artifacts may be marked as `derived_from`
  invocation traces rather than purely renderer-generated.
- `DecisionRecord` gains optional links to invocation request ids and evidence
  bundle ids.
- `ApprovalRequirement` becomes explicit for invocation decisions as well as
  gates.
- `VerificationRecord` gains validation-path links rather than only target
  artifact paths.

## 6. Runtime Flow for Invocation Governance

1. `run` or `resume` loads the existing run context, mode, risk, zone, policy
   set, artifact contract, and pending approvals.
2. A mode step requests an external action by constructing an `InvocationRequest`
   with adapter, capability, orientation, scope, and intended output type.
3. `InvocationOrchestrator` validates that mode, risk, zone, and required
   ownership are resolved before policy evaluation begins.
4. `PolicyEvaluator` loads the effective adapter rules and returns an
   `InvocationPolicyDecision`.
5. Canon persists the request and decision immediately under the run before any
   adapter execution.
6. If the decision is `Deny`, Canon writes a denied invocation trace, links it
   into the evidence bundle, and hands a blocking or recommendation-only result
   back to the mode.
7. If the decision is `NeedsApproval`, Canon writes a pending invocation
   record, moves the run to `AwaitingApproval`, and waits for `approve` or
   `resume`.
8. If the decision is `Allow` or `AllowConstrained`, Canon builds an execution
   envelope carrying constraints and dispatches the request to the concrete
   adapter.
9. The adapter returns raw output; `OutcomeNormalizer` turns it into a
   `ToolOutcome`.
10. `EvidenceAssembler` links the outcome to traces, artifacts, decisions,
    approvals, and generation or validation paths.
11. `Gatekeeper` evaluates mode gates against the updated evidence bundle
    rather than against artifacts alone.
12. `status` and `inspect` read run/evidence manifests directly and do not
    replay tool execution.

## 7. Policy and Constraint Representation

### Code-Defined

- mode semantics and `ModeProfile`
- enum definitions for capabilities, orientations, mutability, trust boundary,
  lineage class, and policy outcomes
- policy precedence rules
- independence evaluation rules
- resume invalidation rules for context drift

### Config-Defined in TOML

- which adapters and capabilities are enabled by mode
- per-risk and per-zone approval requirements
- allowed command profiles for shell execution
- maximum retained bytes and payload retention levels
- path scope profiles for bounded repository reads and writes
- which validation layers are mandatory per risk class

### Constraint Representation

`InvocationConstraintSet` is a typed struct rather than a free-form map. It
contains only enforceable fields:

- allowed path prefixes
- denied path prefixes
- command profile id
- max stdout/stderr bytes to retain
- max diff bytes to retain
- recommendation-only flag
- patch-application disabled flag
- artifact-only write flag
- payload retention level (`SummaryOnly`, `SummaryWithDigest`,
  `SummaryWithRetainedPayload`)

Adapters receive the effective constraint set and must reject execution if they
cannot enforce the requested constraints.

## 8. Trace and Evidence Persistence Strategy

### New or Expanded Runtime Layout

```text
.canon/
├── traces/
│   └── <run-id>.jsonl
└── runs/
    └── <run-id>/
        ├── run.toml
        ├── context.toml
        ├── artifact-contract.toml
        ├── state.toml
        ├── links.toml
        ├── evidence.toml
        ├── invocations/
        │   └── <request-id>/
        │       ├── request.toml
        │       ├── decision.toml
        │       ├── attempt-01.toml
        │       ├── attempt-02.toml
        │       └── payload/
        ├── approvals/
        ├── gates/
        └── verification/
```

### Persistence Rules

- Every invocation request gets a stable `request_id`.
- Every decision is written before dispatch.
- Every attempt appends a trace event to `.canon/traces/<run-id>.jsonl`.
- Heavy raw payloads are not stored inline in JSONL; they are stored as
  optional retained payload files referenced by digest and relative path.
- `evidence.toml` indexes generation paths, validation paths, denied
  invocations, and links to artifacts, decisions, and approvals.
- `links.toml` continues to point to trace streams, decisions, and artifacts,
  but will also link the run-level evidence manifest.

### Resume Interaction

- `resume` scans pending invocation records, checks input fingerprints and
  repository head, and invalidates approvals if scoped context changed.
- Retries create new attempt files under the same `request_id`.
- If a request's constraints or scoped context changed, Canon supersedes the
  pending request and requires a new policy decision and, if applicable, new
  approval.

### Raw I/O Retention

- default: summary plus digest only
- retained payloads only when policy allows, output size stays under cap, and
  the payload is needed for audit or review
- prompts and full chat transcripts are not preserved by default

## 9. Adapter Strategy and Initial Adapter Set

### Adapter Boundary

The initial adapter boundary stays simple:

- concrete adapter structs in `canon-adapters`
- a small `ExecutionAdapterDescriptor` catalog in `canon-engine`
- no public trait hierarchy for arbitrary plugins

### Capability Typing Strategy

Use enum-based capability typing with orthogonal metadata:

- `CapabilityKind` says what action is being requested
- `InvocationOrientation` says whether the action is context, generation,
  validation, or artifact derivation
- `MutabilityClass` says how dangerous the side effect is
- `TrustBoundaryKind` and `LineageClass` say how the result should be treated

This avoids capability explosion while keeping decisions explicit.

### Initial Adapter Set

- **Filesystem**: read repository files, write Canon-owned artifacts only, no
  arbitrary workspace mutation
- **Shell**: read-only commands, bounded validation commands, and later
  bounded-mutation commands when constraints can be enforced and reviewed
- **Copilot CLI**: generation and critique capabilities, with patch-apply
  disabled by default in this increment
- **Validation tools**: executed through the shell adapter but classified as
  validation-oriented requests
- **MCP-compatible tools**: model in the domain and policy, but runtime support
  is deferred unless it can use the same request/decision/outcome pipeline
  without special cases

### Trust and Mutability Representation

- read-only repository analysis: `ReadOnly`
- artifact derivation or Canon-owned write: `ArtifactWrite`
- bounded patch or generated file proposal: `BoundedWorkspaceWrite`
- broad or unconstrained mutation: `BroadWorkspaceWrite`
- external systems or side effects outside the repo: `ExternalStateChange`

Copilot CLI differs from shell execution because it is reasoning-bearing and
therefore contributes lineage concerns. Shell-based validation tools differ
from generic shell execution because their outputs are classified as
validation-oriented and usually have stronger independence value.

## 10. Generation vs Validation Architecture

### Request Tagging

Each `InvocationRequest` carries:

- `orientation`
- `lineage_class`
- `trust_boundary`
- `is_consequential_output`

### Independence Model

Canon computes a `ValidationIndependenceAssessment` for each consequential
generation path.

- same adapter family plus same lineage class: insufficient on its own
- different reasoning-bearing adapter family: partial independence
- non-generative validation tools: strong structural independence
- human review: strong social independence

### Gate Usage

- `ReleaseReadiness` and review-oriented gates inspect evidence bundles and
  reject runs where consequential generation lacks sufficient validation
  independence
- non-generative tools such as tests and linters attach to validation paths and
  may satisfy part of the required independence even when the generation path
  used Copilot CLI
- architecture-oriented recommendations require adversarial critique or human
  review, not just a second generation pass

## 11. Mode-by-Mode Delivery Plan for `requirements`, `brownfield-change`, and `pr-review`

### Slice 1: Requirements with Governed External Invocation

**Delivered first because** it is the smallest end-to-end surface that proves
pre-execution policy, trace persistence, and evidence-derived artifact
generation without workspace mutation complexity.

- governed filesystem and shell context capture
- governed Copilot CLI generation and critique requests
- derived requirements artifacts that cite source invocation ids
- `inspect invocations` and `inspect evidence` for the run

### Slice 2: Brownfield-Change with Repository Context

**Delivered second because** it showcases bounded autonomy, constrained
repository analysis, approval semantics, and validation attachment in a living
codebase.

- governed repository inspection using filesystem and shell read-only
  capabilities
- bounded AI generation of change-surface and implementation recommendations
- recommendation-only treatment for risky mutating proposals
- validation path attachment using tests, lint, or manual review evidence

### Slice 3: PR-Review with Real Diff Inspection

**Delivered third because** it is the most visibly valuable outward-facing
review mode and naturally exercises evidence preservation.

- governed diff capture through shell read-only commands
- critique invocation against changed surface
- validation evidence from diff analysis, optional tests/linters, and review
  disposition
- findings and review artifacts derived from invocation evidence

## 12. Testing Strategy

### Unit Tests

- invocation policy evaluation by mode, risk, zone, and ownership state
- capability classification and trust-boundary classification
- constraint propagation into adapter execution envelopes
- lineage and validation-independence assessment
- evidence bundle assembly and linkage logic

### Integration Tests

- `requirements` run with governed context capture, generation, critique, and
  evidence-linked artifacts
- denied invocation persistence and inspection
- approval-gated invocation followed by `approve` and `resume`
- trace persistence and `evidence.toml` linkage
- `brownfield-change` repository-context flow with bounded analysis and
  validation evidence
- `pr-review` diff inspection and review evidence preservation
- resumable run behavior when pending invocation context becomes stale

### Test Fixture Strategy

- keep default CI fully local
- use deterministic fixture executables or controlled shell commands instead of
  depending on real networked AI services
- gate any live Copilot CLI smoke tests behind explicit opt-in environment
  flags, outside the default CI contract

## 13. CI / Quality Implications

- existing `fmt`, `clippy`, `test`, and `nextest` remain unchanged
- add coverage for new integration suites around invocation governance and
  evidence inspection
- keep `cargo deny` and license policy unchanged
- ensure fixture-based adapter tests run on macOS, Linux, and Windows without
  assuming Unix-only shell semantics
- extend contract tests to cover new runtime files under `.canon/runs/<id>/`
  and new `inspect` output shapes

## 14. Risks and Complexity Tracking

| Risk | Why It Matters | Containment |
| --- | --- | --- |
| Capability taxonomy drifts into one-off variants | Would recreate the generic framework problem | Keep a small canonical `CapabilityKind` plus orthogonal metadata |
| Constraint model becomes unenforceable | “Allow with constraints” would become decorative | Only support typed constraints adapters can actually enforce |
| Trace capture becomes prompt bureaucracy | Auditability would degrade into noise | Store summaries and digests by default, retain raw payloads selectively |
| Validation lineage rules are too weak | Same-path validation could slip through | Encode lineage classes and require stronger evidence for consequential work |
| Approval flow becomes queue semantics by accident | Would introduce hidden runtime complexity | Keep no job queue; approval only unblocks persisted requests on `resume` |
| MCP support introduces special cases early | Would over-generalize around a still-hypothetical path | Keep MCP modeled in policy and domain, but stage runtime delivery later |

## 15. Open Technical Questions That Remain

- Should Copilot CLI retained payloads be stored as plain text files under the
  invocation directory or as digested blobs under a shared payload store?
- How strict should path-scope enforcement be for shell commands that can
  inspect large repository surfaces indirectly through git?
- Should `inspect evidence` be a new top-level inspection surface or should its
  data be folded into `status` plus `inspect invocations`?
- When a validation path combines non-generative tools and human review, what
  is the minimum evidence Canon should require before a readiness gate passes?
