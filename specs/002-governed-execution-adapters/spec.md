# Feature Specification: Governed Execution Adapters

**Feature Branch**: `002-governed-execution-adapters`  
**Created**: 2026-03-28  
**Status**: Draft  
**Input**: User description: "Create a focused product specification for the next Canon increment: Governed Execution Adapters."

## Governance Context *(mandatory)*

**Mode**: architecture  
**Risk Classification**: Critical - this increment changes how Canon authorizes,
constrains, records, and challenges real AI-assisted execution across external
tools, which is central to the product's trust model.  
**Scope In**: governed invocation over external tools, adapter capability
modeling, invocation policy, invocation trace persistence, generation versus
validation separation for tool usage, mode-aware adapter permissions, and
evidence bundles derived from real execution.  
**Scope Out**: autonomous multi-agent orchestration, arbitrary plugin
marketplaces, distributed execution, IDE integrations, replacement of external
AI tools, and preservation of full prompt or chat transcripts as the system of
record.

**Invariants**:

- Canon must resolve mode, risk, zone, policy, and any required ownership
  boundary before any external invocation is attempted.
- Consequential execution must leave durable evidence of what was attempted,
  allowed, denied, escalated, challenged, and accepted.
- Validation must remain independently challengeable and must not collapse into
  the same reasoning path that generated a consequential output.
- Adapters remain subordinate execution surfaces; they do not define the
  product or bypass governance.

**Decision Traceability**: implementation decisions for this increment must be
recorded in the feature decision log and linked to runtime evidence under
`.canon/runs/`, `.canon/traces/`, `.canon/decisions/`, and derived artifacts
for affected runs.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Govern a Real External Invocation (Priority: P1)

An engineer running Canon in `brownfield-change` or `requirements` mode needs
Canon to govern real tool usage instead of only producing documents after the
fact.

**Why this priority**: if Canon cannot authorize and constrain live tool
execution, it cannot fully enforce its operating model.

**Independent Test**: start a run with an adapter-backed action, confirm Canon
classifies the run first, evaluates policy before invocation, and persists an
invocation trace whether the request is allowed or denied.

**Acceptance Scenarios**:

1. **Given** a run with unresolved mode, risk, or zone, **When** an external
   adapter invocation is requested, **Then** Canon rejects the request and
   records that no governed invocation was authorized.
2. **Given** a run with resolved context and an allowed adapter capability,
   **When** the invocation executes, **Then** Canon persists the policy
   decision, execution outcome, and linked evidence inside the run record.

---

### User Story 2 - Keep Generation and Validation Separate (Priority: P2)

A technical lead needs Canon to prevent a single tool or reasoning path from
both generating a consequential change and validating that same change without
independent challenge.

**Why this priority**: circular validation is one of Canon's primary failure
modes; adapter governance is incomplete if it only governs generation.

**Independent Test**: execute a run where one adapter produces a proposal or
change and a separate validation path is required before the run can pass its
gate.

**Acceptance Scenarios**:

1. **Given** a high-consequence generation invocation, **When** Canon evaluates
   readiness, **Then** it requires a validation path that is recorded
   separately from the generation path.
2. **Given** a validation attempt that reuses the same reasoning path without
   independent challenge, **When** the mode or policy requires independence,
   **Then** Canon blocks readiness and records the validation gap.

---

### User Story 3 - Preserve Evidence of Work in Motion (Priority: P3)

A reviewer or approver needs to see not only final artifacts but also which
tools were used, what they were allowed to do, what they actually did, and
which invocations were denied or escalated.

**Why this priority**: auditability depends on evidence of execution, not just
evidence of final text.

**Independent Test**: inspect a completed run and confirm it contains
invocation traces, linked decisions, linked verification steps, and any denied
or approval-gated requests.

**Acceptance Scenarios**:

1. **Given** a run with multiple external invocations, **When** a reviewer
   inspects the run, **Then** Canon exposes a durable trace for each governed
   invocation and links them to the resulting evidence bundle.
2. **Given** an invocation that was denied or required approval, **When** a
   reviewer inspects the run, **Then** Canon shows the attempted capability,
   the policy outcome, and the approval or denial evidence.

### Edge Cases

- What happens when a mode permits repository analysis but forbids mutation,
  and a tool requests a mutating capability anyway?
- How does Canon handle an invocation that partially succeeds, produces output,
  but violates a postcondition required by the gate?
- Which invariant is most likely to be stressed when generation and validation
  both route through tools that share similar reasoning patterns?

## 1. Product Delta

Canon v0.1 already models modes, risk classes, usage zones, gates, artifact
contracts, decision memory, and filesystem persistence. That baseline proved
the governance skeleton, but it still behaves too much like an artifact-driven
runtime: it excels at producing durable records after a run, yet it does not
fully govern work while it is happening.

This increment adds the missing capability layer: governed execution over
external tools. Canon must authorize, constrain, record, and review actual
invocations of Copilot CLI, shell commands, MCP-compatible tools, repository
inspection surfaces, and validation tools. Documents remain important, but they
become evidence produced by governed execution rather than the product's main
output.

The correction is deliberate. Canon must move from "artifact-first runtime
behavior" toward "governed execution runtime behavior" without becoming a
generic agent framework, plugin system, or prompt runner.

## 2. Problem Statement

Artifact generation without governed execution is insufficient. A method engine
that only governs what gets persisted after the fact cannot fully enforce the
operating model described by Canon's constitution. Governance has to apply to
invocation, not only to resulting files.

The operational gap is clear:

- Canon can require artifacts, but an external tool may still be used outside a
  governed authorization path.
- Canon can persist decisions, but not yet always persist what was attempted,
  denied, constrained, retried, or escalated during execution.
- Canon can classify risk and zone for a run, but those classifications must
  also directly shape which tool capabilities are allowed in motion.
- Canon can define verification layers, but must bind them to real generation
  and validation paths rather than only to generated documents.

A trustworthy run must therefore capture not only what was written, but also
what was attempted, allowed, denied, reviewed, challenged, and accepted.

## 3. Goals

- Govern invocation of external tools as first-class runtime behavior.
- Model adapter capabilities explicitly so Canon reasons about what a tool may
  do before it is called.
- Enforce policy before invocation, not only after outputs exist.
- Persist an invocation trace for every governed request, including denials and
  approval-gated outcomes.
- Separate generation adapters and validation adapters where consequence
  requires independent challenge.
- Make adapter permissions mode-aware, risk-aware, and zone-aware.
- Produce evidence bundles from real execution flows so artifacts become
  inspectable proof of governed work.
- Preserve Canon's product identity as a governed method engine rather than a
  generic workflow or agent platform.

## 4. Non-Goals

- Autonomous multi-agent orchestration
- Arbitrary plugin ecosystem or marketplace behavior
- Full distributed execution across remote workers
- Replacing Copilot CLI, MCP tools, test runners, or source-control tools
- Building a generic workflow engine detached from Canon's typed modes
- Preserving every prompt, token stream, or full chat transcript by default
- Treating artifacts as the primary product output instead of as evidence

## 5. Core Concepts Introduced Or Strengthened

### Execution Adapter

- **What it is**: a governed integration surface that Canon can invoke, such as
  Copilot CLI, shell execution, filesystem inspection, or an MCP-compatible
  tool.
- **Why it exists**: Canon governs real tool usage through adapters rather than
  embedding every capability directly in the core runtime.
- **Behavioral effect**: all external execution flows pass through a typed
  adapter boundary that can be classified, authorized, denied, traced, and
  reviewed.

### Adapter Capability

- **What it is**: a declared action an adapter may perform, such as repository
  analysis, code-change proposal, critique generation, command execution, or
  validation.
- **Why it exists**: governance decisions must be made against precise
  capabilities, not vague tool names.
- **Behavioral effect**: policy evaluates requested capabilities rather than
  trusting an adapter wholesale.

### Invocation Request

- **What it is**: the typed request Canon prepares before any external tool is
  called, including the adapter, capability, run context, and requested scope.
- **Why it exists**: Canon needs a stable unit of authorization and trace.
- **Behavioral effect**: no tool runs outside an invocation request evaluated
  against mode, risk, zone, policy, and ownership requirements.

### Invocation Policy

- **What it is**: the policy decision Canon applies to an invocation request:
  allowed, allowed with constraints, approval required, or denied.
- **Why it exists**: authorization must happen before execution and must be
  inspectable after execution.
- **Behavioral effect**: invocation policy determines whether a request may
  execute, must be narrowed, requires approval, or is blocked and recorded as a
  denial.

### Invocation Trace

- **What it is**: the durable record of a governed invocation attempt and its
  outcome.
- **Why it exists**: Canon needs evidence of work in motion, not just final
  artifacts.
- **Behavioral effect**: every consequential invocation leaves a stable trace
  that can be linked to artifacts, decisions, approvals, and verification.

### Tool Outcome

- **What it is**: the normalized result of an invocation, including success,
  failure, partial completion, denied execution, or recommendation-only output.
- **Why it exists**: different adapters emit different raw outputs, but Canon
  needs a common outcome model for gates and evidence.
- **Behavioral effect**: gates and review layers evaluate normalized outcomes
  instead of adapter-specific output quirks.

### Denied Invocation

- **What it is**: a persisted invocation request that Canon refused to execute.
- **Why it exists**: denial is evidence, not absence.
- **Behavioral effect**: blocked actions are reviewable and contribute to
  decision traceability, approval workflows, and audit history.

### Generation Path

- **What it is**: the chain of invocations that produced a proposal, artifact,
  critique candidate, or bounded change.
- **Why it exists**: Canon must know which reasoning path created a
  consequential output.
- **Behavioral effect**: consequential outputs are tagged with their generation
  path and cannot silently inherit trust.

### Validation Path

- **What it is**: the chain of invocations and review steps used to challenge,
  test, critique, or verify a generated output.
- **Why it exists**: independent challenge requires its own evidence path.
- **Behavioral effect**: readiness gates inspect validation paths separately
  from generation paths and can reject insufficient independence.

### Adapter Trust Boundary

- **What it is**: the declared limit of what Canon can assume about an adapter,
  its side effects, and its reasoning independence.
- **Why it exists**: different adapters expose different levels of determinism,
  mutability, and explainability.
- **Behavioral effect**: trust boundary influences whether an adapter can be
  used for generation, validation, or only bounded analysis.

### Evidence Bundle

- **What it is**: the collection of traces, artifacts, decisions, approvals,
  and verification links that together explain a governed execution outcome.
- **Why it exists**: Canon's durable memory must show how a run reached its
  state, not only which files it emitted.
- **Behavioral effect**: run inspection and gates operate over evidence bundles
  rather than isolated documents.

## 6. Adapter Model

Canon models external tools as governed execution surfaces, not as the product's
identity.

- **Copilot CLI**: used for AI-assisted generation or critique within bounded
  invocation requests. It must declare whether a request is generation or
  validation oriented.
- **Shell commands**: used for repository inspection, diff analysis, test
  execution, and bounded command workflows. Shell execution must distinguish
  read-only from mutating capabilities.
- **MCP-compatible tools**: used for structured tool invocation where Canon can
  classify the requested operation and attach it to a typed capability and
  trust boundary.
- **Repository inspection and filesystem access**: used to read source, inspect
  diffs, and derive context. Mutation to working tree or generated files is a
  separate capability from inspection.
- **Test runners and linters**: treated as validation-capable tools whose
  outputs contribute to the validation path and evidence bundle.

Canon may integrate additional adapters later, but adapters remain governed
execution surfaces. They do not form a public plugin marketplace and do not
erase the semantics of Canon's modes.

## 7. Capability Model

Each adapter must declare the capabilities it exposes to Canon. At minimum,
Canon must reason about capabilities such as:

- generate artifact content
- propose code changes
- analyze repository state
- run command
- read files
- produce critique
- validate output
- inspect diff or pull request
- execute bounded transformation

Canon must authorize capabilities, not tool names alone. A tool that can both
read and mutate the repository must expose those as separate capabilities. A
tool that can generate and critique must expose separate generation and
validation capabilities. Capabilities must also carry side-effect boundaries so
Canon can distinguish read-only, bounded-write, and broad-write behavior before
invocation.

## 8. Invocation Governance

No external invocation may occur until Canon has resolved:

- mode
- risk class
- usage zone
- applicable policy set
- required human ownership where policy demands it

For each invocation request, Canon evaluates policy before execution. Policy
may:

- allow invocation
- allow invocation with constraints
- require approval before invocation
- deny invocation

Constraints may include narrowed file scope, read-only enforcement, limitation
to critique-only behavior, recommendation-only output, or explicit prohibition
of mutation. Policy enforcement must happen before tool execution, not as a
post-hoc note in a generated artifact.

Every policy decision must be persisted, including denials, required approvals,
and constrained allowances.

## 9. Generation vs Validation Separation

High-consequence work must not be generated and validated through the same
reasoning path. Canon must therefore distinguish:

- **generation adapters**: adapters or capabilities used to create proposals,
  changes, summaries, or artifacts that may shape engineering decisions
- **validation adapters**: adapters or capabilities used to challenge, test,
  critique, or independently assess generated outputs

For low-consequence work, the same adapter class may be acceptable for both
paths if the policy explicitly allows it. For bounded or systemic impact work,
the same adapter class is not sufficient for validation when it shares the same
reasoning basis as the generation path. In those cases, Canon must require one
or more of:

- adversarial critique through a separate path
- independent human review
- validation through non-generative tools such as tests, linters, or diff
  analysis
- architectural review for structural recommendations

Layered verification must therefore attach to actual execution paths, not only
to documents produced after execution.

## 10. Trace And Evidence Model

For every governed invocation, Canon must persist:

- adapter used
- capability requested
- run id
- mode
- risk
- zone
- policy decision
- whether invocation was allowed, constrained, blocked, or escalated
- execution outcome
- trace reference
- linked artifacts
- linked decisions
- linked verification steps

Canon should not preserve every prompt transcript by default. Instead, it must
preserve durable evidence of consequential execution: what capability was
requested, under what policy, with what outcome, and with what attached review
or challenge.

Evidence bundles must be inspectable per run and should support later review,
approval, replay analysis, and decision traceability without requiring access
to ephemeral chat logs.

## 11. Mode Interaction

### Requirements

- **Likely adapters**: repository inspection, filesystem read, bounded AI
  generation, critique
- **Allowed actions**: analyze context, synthesize bounded options, derive
  constraints and trade-offs
- **Stronger gating**: any broadening from analysis into executable change
  recommendation requires explicit approval or handoff
- **Persisted evidence**: source context references, generation traces,
  critique traces, resulting requirements artifacts

### Brownfield-Change

- **Likely adapters**: repository inspection, shell diff/context commands,
  bounded AI generation, tests and validation tools
- **Allowed actions**: map legacy invariants, propose change surfaces, produce
  bounded implementation recommendations
- **Stronger gating**: mutation, code-change proposals that broaden surface
  area, or systemic-impact recommendations require stronger policy and approval
- **Persisted evidence**: repository context traces, denied or constrained
  mutation requests, validation outputs, linked decision records

### PR-Review

- **Likely adapters**: diff inspection, critique, shell read-only commands,
  validation tools
- **Allowed actions**: inspect changed surface, produce findings, run bounded
  validation commands
- **Stronger gating**: auto-remediation or mutating fix generation in review
  mode requires explicit policy support and should default to denial
- **Persisted evidence**: diff traces, critique traces, findings, review
  summaries, approval or disposition records

### Architecture

- **Likely adapters**: repository analysis, bounded AI synthesis, adversarial
  critique, structural validation sources
- **Allowed actions**: form structural options, compare alternatives, challenge
  recommendations
- **Stronger gating**: architectural recommendations cannot pass without
  adversarial critique or equivalent independent challenge
- **Persisted evidence**: option-generation traces, critique traces, decision
  records, architecture review evidence

### Verification

- **Likely adapters**: test runners, linters, critique tools, repository and
  diff inspection
- **Allowed actions**: execute validation, produce critiques, challenge prior
  generation
- **Stronger gating**: validation paths that are not independent enough from
  the generation path must fail readiness
- **Persisted evidence**: validation outcomes, critique traces, failed
  assertions, final verification bundle

### Review

- **Likely adapters**: critique tools, repository inspection, shell read-only
  commands, policy lookups
- **Allowed actions**: challenge assumptions, inspect impact, summarize risk
- **Stronger gating**: recommendation to accept consequential risk must require
  named human ownership
- **Persisted evidence**: review traces, finding records, approval and
  disposition links

## 12. Policy Model

Canon policies must constrain adapter usage based on:

- mode
- risk class
- usage zone
- surface touched
- human ownership
- approval status

The policy model must support rules such as:

- AI may analyze but not modify in red zones.
- Systemic-impact brownfield changes require explicit approval before
  generation broadens from analysis into change recommendation or mutation.
- PR review may invoke critique and diff analysis adapters but must preserve a
  review evidence bundle for every consequential finding.
- Architecture mode must require adversarial critique before a structural
  recommendation can pass.
- Validation-capable tools may run automatically only when the policy allows
  their capability for the current mode, risk, and zone.

Policies may remain partly config-defined, but the semantics of Canon modes,
risk classes, and approval requirements must remain strongly typed and
code-owned.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST require mode, risk class, usage zone, and any
  policy-required human ownership context to be resolved before any external
  invocation is evaluated.
- **FR-002**: Canon MUST represent each external tool invocation as a typed
  invocation request referencing an adapter and a declared capability.
- **FR-003**: Canon MUST evaluate invocation policy before execution and MUST
  persist the resulting authorization decision.
- **FR-004**: Canon MUST support policy outcomes of allow, allow with
  constraints, approval required, and deny.
- **FR-005**: Canon MUST persist an invocation trace for every governed
  invocation attempt, including denied and approval-gated requests.
- **FR-006**: Canon MUST distinguish generation paths from validation paths for
  consequential work and MUST attach both to the run evidence bundle.
- **FR-007**: Canon MUST require independent challenge for bounded-impact or
  systemic-impact work when policy marks the generated output as consequential.
- **FR-008**: Canon MUST allow adapter permissions to vary by mode, risk class,
  usage zone, touched surface, approval status, and ownership status.
- **FR-009**: Canon MUST allow artifacts to be derived from governed execution
  outputs rather than only from static template rendering.
- **FR-010**: Canon MUST persist links between invocation traces, resulting
  artifacts, related decisions, approvals, and verification steps.
- **FR-011**: Canon MUST treat denied invocations and recommendation-only
  outcomes as first-class evidence records.
- **FR-012**: Canon MUST preserve Canon's typed mode semantics and MUST NOT
  reduce adapter governance into a generic workflow runtime detached from those
  modes.
- **FR-013**: Canon MUST support governed repository-context consumption in
  `brownfield-change` and `pr-review` using real repository state and real tool
  outputs.
- **FR-014**: Canon MUST allow validation-capable adapters such as test runners
  and linters to contribute evidence without being mistaken for the generation
  path.

### Key Entities *(include if feature involves data)*

- **Execution Adapter**: A governed integration surface with declared
  capabilities, trust boundary, and side-effect profile.
- **Invocation Request**: The unit Canon evaluates before tool execution,
  carrying adapter, capability, scope, and run context.
- **Invocation Trace**: The durable record of an attempted invocation and its
  policy and execution outcomes.
- **Invocation Policy Decision**: The persisted authorization result for an
  invocation request.
- **Tool Outcome**: The normalized result of a governed invocation.
- **Evidence Bundle**: The linked set of traces, artifacts, decisions,
  approvals, and verification records for a run.

## 13. Acceptance Criteria

- Canon cannot invoke an adapter before mode, risk, and zone are resolved.
- Canon can deny an invocation through policy and persist the denial as
  evidence.
- Canon can allow an invocation with constraints and persist what those
  constraints were.
- Canon persists a trace record for every governed invocation attempt.
- Canon distinguishes generation and validation paths for high-consequence
  work.
- Canon can attach execution evidence to both the run record and its decision
  memory.
- `brownfield-change` can consume real repository context and real tool output
  under governed adapter invocation.
- `pr-review` can consume real diff or repository context and preserve review
  evidence from actual inspection and critique paths.
- Artifacts can be derived from governed execution outputs and traces, not only
  from static template generation.
- Denied or approval-gated invocations remain inspectable after the run.

## 14. Open Questions

- How should adapter capabilities be typed so they remain explicit without
  becoming an explosion of one-off capability variants?
- How should "allow with constraints" be represented so that constrained
  invocations stay inspectable and enforceable at runtime?
- Which parts of invocation policy should remain code-defined and which should
  be safe to express in config?
- How can Canon keep traces useful for audit and review without turning them
  into prompt bureaucracy?
- How should Canon model validation independence when multiple tools share
  similar reasoning patterns or vendor lineage?
- How should approvals interact with queued, retried, or resumed invocations?

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In pilot runs for `requirements`, `brownfield-change`, and
  `pr-review`, 100% of external tool invocations are either authorized or
  denied through Canon before execution.
- **SC-002**: Reviewers can inspect a completed run and identify the adapter,
  capability, policy outcome, and execution outcome for every consequential
  invocation without consulting chat history.
- **SC-003**: For bounded-impact and systemic-impact runs, 100% of
  consequential generation outputs have at least one separately recorded
  validation path before readiness can pass.
- **SC-004**: Canon can show, for every denied or approval-gated invocation,
  why it was blocked or escalated and which policy condition caused that
  outcome.

## Validation Plan *(mandatory)*

- **Structural validation**: specification review against the existing Canon
  product spec, plan, adapter policy baseline, and mode model to confirm this
  increment remains additive and mode-aware.
- **Logical validation**: walkthroughs for `requirements`, `brownfield-change`,
  `pr-review`, `architecture`, `verification`, and `review` showing how
  invocation authorization, trace capture, and evidence linking behave in each
  mode.
- **Independent validation**: adversarial design review focused on whether the
  increment still risks reducing Canon to an artifact runtime or thin wrapper
  around external tools.
- **Evidence artifacts**: feature spec, checklist, future plan and decision
  log, runtime invocation traces, linked run manifests, and decision records.

## Decision Log *(mandatory)*

- **D-001**: Canon will treat governed execution as the primary runtime concern
  and artifacts as durable evidence of that execution, **Rationale**: this
  realigns the implementation with the product thesis and avoids reducing Canon
  to a markdown-producing wrapper.
- **D-002**: Adapter governance will stay subordinate to typed modes rather
  than introducing a generic workflow abstraction, **Rationale**: the product's
  value comes from Canon-specific governance semantics, not from generic
  orchestration flexibility.

## Non-Goals

- Turning Canon into a general-purpose agent framework
- Turning Canon into a plugin marketplace for arbitrary tools
- Capturing complete prompt or chat transcripts as default system memory
- Allowing adapters to bypass policy because they are "trusted" tools
- Reframing Canon as a thin shell around Copilot CLI

## Assumptions

- The next implementation increment will build on the existing local-first
  `.canon/` persistence model rather than replacing it.
- Initial governed execution depth will focus on the already-runnable modes
  before broadening execution depth across the remaining typed modes.
- Canon will continue to support local repository execution as the primary
  operating environment for v0.x.
- Existing artifact contracts remain useful, but future artifacts will be
  increasingly derived from execution evidence rather than only from template
  rendering.
