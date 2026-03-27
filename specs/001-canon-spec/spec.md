# Canon v0.1 Product Specification

## Governance Context *(mandatory)*

**Mode**: requirements  
**Risk Classification**: High - this specification defines the governing contract
for future execution modes, artifact rules, and validation behavior, but does
not authorize production code changes by itself.  
**Scope In**: product thesis, domain model, initial modes, risk model, usage
zones, artifact contracts, gates, verification model, decision memory,
high-level architecture, persistence model, execution flow, adapter philosophy,
MVP boundaries, and acceptance criteria for v0.1.  
**Scope Out**: final runtime implementation details, full autonomous delivery,
IDE-first workflows, generic agent marketplace behavior, and organization-wide
analytics or control-plane services beyond local persistence.

**Invariants**:

- No run may begin without an explicit mode, risk classification, usage-zone
  classification, artifact contract, and human ownership boundary.
- Durable artifacts and decision records outrank chat transcripts as the system
  of record.
- Generation and validation must remain separable for non-trivial work.
- Canon governs external tools; it does not replace them.

**Decision Traceability**: consequential decisions, approvals, and verification
evidence must be persisted under `.canon/decisions/` and linked from run records
under `.canon/runs/`.

## User Scenarios & Testing *(mandatory)*

### Operational Scenario 1 - Requirements Mode (Priority: P1)

A product or engineering lead needs to turn a vague initiative into a bounded,
reviewable product definition without allowing AI to expand the scope into a
platform fantasy.

**Why this priority**: v0.1 is only valuable if it can bound intent before
generation accelerates ambiguity into structure.

**Independent Test**: given a raw idea and explicit constraints, the system
emits the required requirements artifacts, records exclusions and trade-offs,
and blocks progress until decision checkpoints are satisfied.

**Acceptance Scenarios**:

1. **Given** a raw product idea with incomplete boundaries, **When** a run
   starts in `requirements` mode, **Then** the system classifies risk and zone
   before generating bounded options or artifacts.
2. **Given** AI-proposed options that expand beyond the MVP boundary, **When**
   the Exploration Gate runs, **Then** the system requires explicit scope cuts
   and recorded trade-offs before the run can complete.

---

### Operational Scenario 2 - Brownfield Change Mode (Priority: P2)

An engineer needs to plan a change in an existing multimodule codebase without
letting AI rewrite legacy behavior or system boundaries before invariants are
mapped.

**Why this priority**: brownfield work is where AI can destroy history while
still appearing locally correct, so containment must exist before
implementation.

**Independent Test**: given a repository slice and a change goal, the system
produces a system slice, legacy invariants, change surface, implementation
plan, and validation strategy before any execution recommendation is treated as
valid.

**Acceptance Scenarios**:

1. **Given** a brownfield change request, **When** the run enters
   `brownfield-change` mode, **Then** the system requires `legacy-invariants.md`
   and `change-surface.md` before `implementation-plan.md` can pass its gate.
2. **Given** a proposed change that touches shared invariants or architectural
   boundaries, **When** risk is classified as systemic, **Then** the run
   requires named human ownership and elevated verification before any approval.

---

### Operational Scenario 3 - PR Review Mode (Priority: P3)

A reviewer needs a structured review artifact set that checks boundary
violations, duplication, contract drift, missing tests, and decision impact
without collapsing review into generic AI commentary.

**Why this priority**: review is where governance must challenge locally
plausible change before it becomes repository precedent.

**Independent Test**: given a pull request diff or branch comparison, the
system emits the review artifact set, flags severity by risk and zone, and
requires an explicit human disposition for unresolved high-impact findings.

**Acceptance Scenarios**:

1. **Given** a pull request affecting core logic, **When** the run enters
   `pr-review` mode, **Then** the system emits the defined review artifact set
   and links findings to the changed surface.
2. **Given** a review that detects contract drift or missing verification in a
   systemic-impact area, **When** the Review Disposition Gate runs, **Then**
   the run cannot complete as "ready" without explicit human acceptance or
   rejection.

## 1. Product Thesis

Canon is a governed method engine for AI-assisted software
engineering. It sits above tools such as Copilot CLI, MCP-compatible tools,
local shell commands, GitHub integrations, linters, test runners, and
documentation generators and imposes explicit method, risk, artifact, gate,
and verification discipline on top of them.

It is not a generic AI assistant. It is not a prompt library. It is not a
Copilot replacement. It is not a generic multi-agent framework. It does not
win by generating more. It wins by making acceleration governable.

The product exists to turn AI-generated acceleration into structured,
auditable, reviewable delivery. Its value is not "better generation." Its
value is that engineering work becomes bounded by explicit methods, preserved
as durable artifacts, challenged by independent review, and traceable through
decision memory.

## 2. Problem Statement

AI changes software engineering by collapsing the cost of generation. That
creates leverage, but it also creates epistemic risk. Code, architecture,
tests, summaries, and reviews can now be produced faster than humans can
independently understand or challenge them.

The operational failure modes are already clear:

- Circular validation: the same reasoning path can generate code, tests,
  explanations, and review narratives that all agree while still being wrong.
- Decision provenance loss: architectural and workflow decisions increasingly
  happen in chat-like flows that disappear before they become durable memory.
- Architectural drift: unconstrained AI-assisted changes can be locally
  plausible while globally inconsistent with system boundaries and canonical
  ownership.
- Review collapse: code and artifact volume can outrun human review depth,
  turning approval into a thin processing step rather than genuine challenge.
- Artifact decay: conversations appear useful in the moment but do not become
  reliable inputs for the next step unless persisted as inspectable artifacts.

Ephemeral conversations are therefore inadequate as system memory. Once
generation outpaces comprehension, an engineering system becomes unsafe even
before production failures appear because the organization no longer knows what
it knows, why it believes it, or how independently that belief was tested.

## 3. Goals

The goals of v0.1 are operational, not aspirational:

- Establish mode-driven workflows so that requirements work, brownfield change,
  and PR review use different methods, artifacts, and gates.
- Require risk classification before generation or execution broadens the
  change surface.
- Require durable artifact production at each meaningful step so that outputs
  become reusable inputs rather than disposable chat residue.
- Make consequential decisions traceable through durable decision records with
  context, alternatives, rationale, consequences, and ownership.
- Enforce layered verification so that generated coherence is not mistaken for
  independent validation.
- Bound autonomy according to what the organization can still validate.
- Provide reusable governed workflows that sit above external tools without
  becoming dependent on one tool vendor or model brand.
- Preserve local auditability through filesystem persistence of runs,
  artifacts, traces, policies, and decisions.
- Favor depth in three initial modes over breadth across the entire engineering
  lifecycle.

## 4. Non-Goals

v0.1 is intentionally narrow. It does not attempt to:

- deliver full autonomous software development from idea to production
- become an IDE-first product experience
- become a generic multi-agent framework
- host a broad plugin or marketplace ecosystem
- replace existing coding assistants or source-control tools
- solve every engineering phase at once
- centralize organizational governance in a remote SaaS control plane
- optimize for unconstrained flexibility over governability

## 5. Core Domain Model

### Mode

- **What it is**: a named operating context such as `requirements`,
  `brownfield-change`, or `pr-review`.
- **Why it exists**: different kinds of engineering work require different
  methods, allowed tools, artifacts, and gates.
- **Behavioral effect**: mode selection determines which method definition is
  loaded, which artifacts are mandatory, which shortcuts are prohibited, and
  how completion is judged.

### Method

- **What it is**: the ordered workflow contract for a mode, including steps,
  inputs, artifacts, gates, stop conditions, and exit criteria.
- **Why it exists**: it prevents ad-hoc prompting from masquerading as
  engineering workflow.
- **Behavioral effect**: the system executes a method rather than a free-form
  conversation, and every run is evaluated against that method contract.

### Step

- **What it is**: a bounded unit of method execution that consumes inputs and
  produces artifacts, classifications, or gate decisions.
- **Why it exists**: it keeps runs inspectable and decomposes work into
  challengeable segments.
- **Behavioral effect**: the system can stop, persist, review, or reject work
  at step boundaries rather than only at the end.

### Risk Class

- **What it is**: the declared consequence class of being wrong before
  generation or execution begins.
- **Why it exists**: autonomy and verification must scale with consequence, not
  with enthusiasm.
- **Behavioral effect**: risk class changes the allowed autonomy level,
  artifact burden, human involvement, gate strictness, and verification depth.

### Usage Zone

- **What it is**: the sensitivity zone of the work surface: Green, Yellow, or
  Red.
- **Why it exists**: some code and decision surfaces are inherently less safe
  for autonomous action than others.
- **Behavioral effect**: the zone constrains what AI may do even when the task
  itself looks locally simple.

### Artifact

- **What it is**: a durable, inspectable output produced by the method.
- **Why it exists**: artifacts are the real system memory and the inputs to
  later steps.
- **Behavioral effect**: progress is invalid unless required artifacts are
  produced and persisted.

### Artifact Requirement

- **What it is**: the rule that ties specific artifacts to a mode, risk class,
  and gate sequence.
- **Why it exists**: it prevents artifact production from becoming optional
  documentation theater.
- **Behavioral effect**: runs cannot pass gates when mandatory artifacts or
  minimum contents are missing.

### Gate

- **What it is**: a mandatory checkpoint that answers whether the run may move
  forward.
- **Why it exists**: AI must not smuggle structural impact into the system as
  ordinary output.
- **Behavioral effect**: gate outcomes block or authorize later steps and are
  persisted as audit evidence.

### Decision Record

- **What it is**: the durable record of a consequential decision, including
  alternatives and ownership.
- **Why it exists**: decisions otherwise disappear into transient prompting and
  become archaeology later.
- **Behavioral effect**: structural and systemic-impact decisions remain
  reconstructable across sessions and team changes.

### Verification Layer

- **What it is**: one independent form of challenge applied to generated
  output, such as self-critique, adversarial critique, peer review, or
  architectural review.
- **Why it exists**: no single reasoning path is sufficient to justify trust.
- **Behavioral effect**: higher-risk work must survive multiple kinds of
  scrutiny before completion claims are accepted.

### Run Context

- **What it is**: the persisted description of the current run, including repo
  state, inputs, mode, classifications, artifacts, tool capabilities,
  approvals, and traces.
- **Why it exists**: a run must be reproducible and auditable without
  reconstructing chat history from memory.
- **Behavioral effect**: every run becomes a discrete object that later steps,
  reviewers, and reruns can inspect.

### Execution Adapter

- **What it is**: the boundary that exposes an external tool capability to the
  method engine.
- **Why it exists**: the product governs tools rather than calling them
  informally.
- **Behavioral effect**: tools are invoked through declared capabilities with
  traceable side effects and capability boundaries.

### Human Ownership / Approval

- **What it is**: the named human sponsor or approver for consequential work.
- **Why it exists**: AI can assist and propose, but accountability cannot be
  delegated to a model.
- **Behavioral effect**: systemic-impact and red-zone work requires explicit
  human authority before the system can treat the run as acceptable.

### Stop Condition

- **What it is**: the rule that forces a run to halt instead of continuing
  under invalid assumptions.
- **Why it exists**: unsafe momentum is one of the main failure modes of
  AI-assisted work.
- **Behavioral effect**: missing artifacts, failed gates, absent ownership, or
  invalid classifications stop the run rather than producing more output.

### Exit Criteria

- **What it is**: the conditions under which a run may end in a valid state.
- **Why it exists**: completion must mean more than "AI produced something."
- **Behavioral effect**: the system only completes when artifact contracts,
  gate decisions, decision records, and verification obligations are satisfied.

## 6. Modes in v0.1

### requirements

- **Purpose**: convert raw demand into a bounded product definition before AI
  accelerates ambiguity into false clarity.
- **Expected inputs**: idea, business goal, ticket, product brief, user
  outcome, known constraints, and explicit exclusions if already known.
- **Required artifacts**: `problem-statement.md`, `constraints.md`,
  `options.md` or `mvp-options.md`, `tradeoffs.md`, `scope-cuts.md`,
  `decision-checklist.md`.
- **Allowed tools**: read-only document ingestion, repository context lookup,
  shell commands that inspect context, AI exploration through governed prompts,
  and filesystem persistence.
- **Prohibited shortcuts**: generating full solution catalogs before constraints
  exist, expanding into platform scope, skipping exclusions, or treating a
  polished PRD as proof of convergence.
- **Gate sequence**: Exploration Gate -> Risk Gate -> Architecture Gate (for
  structural consequences) -> Release / Readiness Gate.
- **Verification expectations**: bounded options must survive subtraction,
  trade-off review, and human approval of the product core.
- **Output contract**: a bounded v0.1 definition with explicit trade-offs,
  scope cuts, and unresolved questions captured as durable artifacts.

### brownfield-change

- **Purpose**: constrain change in an existing system before AI rewrites legacy
  behavior or boundaries it does not understand.
- **Expected inputs**: repository slice, change goal, affected modules,
  existing interfaces, available documentation, and known operational or domain
  constraints.
- **Required artifacts**: `system-map.md` or `system-slice.md`,
  `legacy-invariants.md`, `change-surface.md`, `implementation-plan.md`,
  `validation-strategy.md`, `decision-record.md`.
- **Allowed tools**: repository inspection, diff and history inspection, test
  and documentation discovery, read-only shell commands, governed AI analysis,
  and filesystem persistence.
- **Prohibited shortcuts**: touching the domain core first, inferring behavior
  solely from structure, redefining boundaries without authorization, or
  treating green tests as sufficient proof of preservation.
- **Gate sequence**: Exploration Gate -> Brownfield Preservation Gate ->
  Architecture Gate -> Risk Gate -> Release / Readiness Gate.
- **Verification expectations**: legacy invariants, constrained change surface,
  independent validation strategy, and named ownership for systemic decisions.
- **Output contract**: a bounded change package that defines what may change,
  what must remain true, how the change will be validated, and who owns the
  consequence.

### pr-review

- **Purpose**: produce structured, challenge-oriented review artifacts for a
  pull request or branch diff.
- **Expected inputs**: diff, changed files, claimed intent, related issue or
  spec links, available tests, and repository context.
- **Required artifacts**: `pr-analysis.md`, `boundary-check.md`,
  `duplication-check.md`, `contract-drift.md`, `missing-tests.md`,
  `decision-impact.md`, `review-summary.md`.
- **Allowed tools**: diff inspection, repository search, test and documentation
  inspection, review-only AI analysis, and filesystem persistence.
- **Prohibited shortcuts**: review by summary alone, approval based only on CI
  green status, mutating the primary implementation during review mode, or
  collapsing architectural concerns into style commentary.
- **Gate sequence**: Risk Gate -> Architecture Gate (if structural impact is
  present) -> Review Disposition Gate -> Release / Readiness Gate.
- **Verification expectations**: findings must be cross-checked against changed
  surfaces, missing verification must be explicit, and systemic issues require
  named human disposition.
- **Output contract**: a structured review packet that states severity,
  rationale, and disposition rather than generic approval language.

## 7. Risk Model

### Low Impact

- **Definition**: local, cheap-to-reverse work whose failure is unlikely to
  alter meaningful system behavior beyond its immediate surface.
- **Examples**: bounded documentation updates, small scaffolding, formatting or
  boilerplate in non-sensitive areas.
- **Autonomy level**: AI may generate and recommend aggressively inside Green
  or Yellow zones.
- **Artifact requirements**: minimal run record, explicit classification, and
  any mode-specific lightweight artifacts.
- **Required gates**: Exploration Gate and Release / Readiness Gate; Risk Gate
  remains mandatory but lightweight.
- **Required human involvement**: proportional review or spot check.
- **Verification intensity**: self-critique and proportional human review.

### Bounded Impact

- **Definition**: change that affects real logic, interfaces, or multiple files
  or modules but does not obviously threaten shared invariants or system-wide
  stability.
- **Examples**: contained feature work, local refactors, bounded interface
  changes, non-core service behavior changes.
- **Autonomy level**: AI may assist heavily but must not outrun human review.
- **Artifact requirements**: full mode-specific artifact set, explicit decision
  notes where boundaries or trade-offs matter, persisted gate outcomes.
- **Required gates**: all core gates relevant to the mode.
- **Required human involvement**: named reviewer and explicit acceptance of the
  bounded change surface.
- **Verification intensity**: self-critique, adversarial critique, and peer
  review are mandatory.

### Systemic Impact

- **Definition**: change that can alter architecture, shared invariants,
  security, compliance, money flow, operational behavior, or organizational
  trust if wrong.
- **Examples**: architectural boundary shifts, core domain behavior changes,
  security-sensitive logic, financial flows, governance engine policy changes.
- **Autonomy level**: AI may analyze, compare options, and draft artifacts, but
  human ownership is mandatory before structural consequences are accepted.
- **Artifact requirements**: full mode-specific artifact set, decision record,
  explicit approval, persisted verification evidence, and traceable gate
  results.
- **Required gates**: all core gates plus any mode-specific preservation or
  disposition gates.
- **Required human involvement**: named sponsor or approver with explicit risk
  acceptance.
- **Verification intensity**: self-critique, adversarial critique, peer review,
  and architectural review are all mandatory.

## 8. Usage Zones

### Green Zone

- **Work that belongs here**: low-consequence scaffolding, bounded mechanical
  transformation, non-sensitive documentation, or repeatable local work.
- **What AI may do**: generate, transform, summarize, and prepare outputs with
  limited oversight.
- **What AI may not do**: silently broaden scope, invent structural changes, or
  skip required artifact persistence.

### Yellow Zone

- **Work that belongs here**: business logic, bounded feature work, refactors,
  and interfaces whose mistakes are real but locally containable.
- **What AI may do**: assist with analysis, drafting, and implementation under
  explicit constraints.
- **What AI may not do**: merge or finalize consequential change without human
  review, or treat generated tests as sufficient proof by themselves.

### Red Zone

- **Work that belongs here**: core invariants, architecture shifts, security,
  money flows, regulatory concerns, shared contracts, and governance logic.
- **What AI may do**: analyze, compare options, summarize evidence, and draft
  candidate artifacts under explicit containment.
- **What AI may not do**: autonomously authorize structural impact, finalize
  decisions, or pass a run without named human ownership and elevated
  verification.

### Interaction Between Zones and Risk

Zone classification describes surface sensitivity. Risk classification describes
the consequence of being wrong. The system MUST apply the stricter control
regime of the two. A systemic-impact change in a Green surface is still treated
as systemic. A low-impact change in a Red surface remains Red for autonomy and
approval purposes.

## 9. Artifact Model

Artifacts are not documentation theater. They are durable, inspectable,
reusable outputs that become inputs to later steps and later gates. An artifact
is only valid if it can be read independently of chat history and if later
steps can rely on it as the system of record.

### Requirements Mode Artifacts

| Artifact | Purpose | Minimum Required Content | Mandatory When | Later Gates That Depend On It |
| --- | --- | --- | --- | --- |
| `problem-statement.md` | Define the user outcome and bounded problem. | User outcome, triggering pain, explicit problem boundary, success signal. | Every `requirements` run. | Exploration, Risk |
| `constraints.md` | Freeze hard limits before expansion. | Capacity limits, complexity ceiling, domain or business constraints, non-negotiables. | Every `requirements` run. | Exploration, Risk, Architecture |
| `options.md` or `mvp-options.md` | Present bounded choices instead of a feature universe. | At most 2-3 options, excluded complexity, primary risk per option. | Every `requirements` run. | Exploration, Architecture |
| `tradeoffs.md` | Record why one path survives. | Chosen direction, rejected alternatives, consequences accepted. | Before output contract completion. | Architecture, Risk, Release / Readiness |
| `scope-cuts.md` | Force explicit subtraction. | Deferred features, excluded flows, why each cut preserves focus. | Every `requirements` run. | Exploration, Release / Readiness |
| `decision-checklist.md` | Surface unresolved human judgments. | Required approvals, open decisions, acceptance questions, unresolved risks. | Before run completion. | Risk, Release / Readiness |

### Brownfield Change Mode Artifacts

| Artifact | Purpose | Minimum Required Content | Mandatory When | Later Gates That Depend On It |
| --- | --- | --- | --- | --- |
| `system-map.md` or `system-slice.md` | Bound the affected surface. | Affected modules, interfaces, dependencies, excluded areas, ownership hints. | Every `brownfield-change` run. | Exploration, Architecture |
| `legacy-invariants.md` | Freeze preserved behavior before change. | Behavioral invariants, weird-but-required behavior, legacy contracts, forbidden normalization. | Every `brownfield-change` run. | Brownfield Preservation, Risk, Release / Readiness |
| `change-surface.md` | State what may change and what may not. | Files, modules, interfaces, data paths, excluded surfaces, blast radius. | Before planning. | Risk, Architecture |
| `implementation-plan.md` | Constrain execution. | Ordered steps, bounded surfaces, rollback awareness, ownership, dependencies. | Before any execution recommendation is accepted. | Architecture, Release / Readiness |
| `validation-strategy.md` | Define independent challenge. | Contract checks, invariant checks, shadow validation or equivalent, evidence plan. | Every `brownfield-change` run. | Risk, Release / Readiness |
| `decision-record.md` | Preserve rationale for structural or risky choices. | Context, alternatives, rationale, consequences, sponsor, unresolved questions. | Every `brownfield-change` run. | Architecture, Release / Readiness |

### PR Review Mode Artifacts

| Artifact | Purpose | Minimum Required Content | Mandatory When | Later Gates That Depend On It |
| --- | --- | --- | --- | --- |
| `pr-analysis.md` | Summarize claimed intent versus actual changed surface. | Scope summary, changed modules, inferred intent, surprising surface area. | Every `pr-review` run. | Risk, Review Disposition |
| `boundary-check.md` | Detect boundary and ownership violations. | Boundary findings, ownership breaks, unauthorized structural impact. | Every `pr-review` run. | Architecture, Review Disposition |
| `duplication-check.md` | Detect parallel logic and shadow abstractions. | Duplicate behavior, near-duplicates, canonical-owner conflicts. | Every `pr-review` run. | Review Disposition |
| `contract-drift.md` | Detect external or internal contract divergence. | Interface drift, schema drift, error-shape drift, compatibility concerns. | Every `pr-review` run. | Architecture, Release / Readiness |
| `missing-tests.md` | State verification gaps. | Missing invariant checks, missing contract checks, weak or mirrored tests. | Every `pr-review` run. | Review Disposition, Release / Readiness |
| `decision-impact.md` | Surface hidden structural consequences. | Decisions implied by the diff, absent decision records, reversibility concerns. | Every `pr-review` run. | Risk, Review Disposition |
| `review-summary.md` | Produce the final review disposition. | Severity, rationale, must-fix items, accepted risks, final disposition. | Every `pr-review` run. | Review Disposition, Release / Readiness |

## 10. Gates

### Exploration Gate

- **Question answered**: is the problem space bounded enough to continue?
- **Required inputs**: mode, run context, initial constraints, early artifacts.
- **Artifacts checked**: requirements or brownfield discovery artifacts relevant
  to the mode.
- **What blocks progress**: missing constraints, runaway option expansion,
  undefined scope boundary, absent system slice.
- **Approval required**: human confirmation when the run would otherwise expand
  scope or invent new surfaces.
- **Applicability**: all modes.

### Brownfield Preservation Gate

- **Question answered**: has preserved behavior been made explicit before
  planning or execution?
- **Required inputs**: legacy context, system slice, invariants, change
  surface.
- **Artifacts checked**: `legacy-invariants.md`, `system-map.md` or
  `system-slice.md`, `change-surface.md`.
- **What blocks progress**: missing behavioral invariants, unconstrained change
  surface, premature core rewrite.
- **Approval required**: human approval when the preserved surface is unclear.
- **Applicability**: `brownfield-change` only.

### Architecture Gate

- **Question answered**: does the proposed change preserve or intentionally
  alter boundaries and invariants?
- **Required inputs**: affected boundaries, decision records, artifacts
  describing trade-offs and ownership.
- **Artifacts checked**: `tradeoffs.md`, `decision-record.md`,
  `boundary-check.md`, `contract-drift.md`, or equivalent mode artifacts.
- **What blocks progress**: unauthorized boundary changes, ambiguous ownership,
  unclear data or contract authority, missing rationale.
- **Approval required**: named human sponsor for structural or systemic impact.
- **Applicability**: all modes when structure or invariants are touched.

### Risk Gate

- **Question answered**: is the declared risk proportional to the likely
  consequence of being wrong?
- **Required inputs**: risk class, usage zone, blast radius, ownership, and
  verification plan.
- **Artifacts checked**: classification record, `constraints.md`,
  `validation-strategy.md`, `decision-impact.md`, or equivalent.
- **What blocks progress**: missing classification, under-classified impact,
  absent owner, weak verification for the declared risk.
- **Approval required**: explicit human ownership for systemic impact or Red
  zone work.
- **Applicability**: all modes.

### Review Disposition Gate

- **Question answered**: is the reviewed change acceptable, rejectable, or
  blocked pending fixes?
- **Required inputs**: review artifacts, severity findings, decision impact,
  missing verification notes.
- **Artifacts checked**: the full PR review packet.
- **What blocks progress**: unresolved must-fix findings, unsupported contract
  drift, systemic issues without owner acceptance.
- **Approval required**: explicit human acceptance or rejection for unresolved
  high-impact findings.
- **Applicability**: `pr-review` only.

### Release / Readiness Gate

- **Question answered**: is the run complete enough to be treated as valid
  output for the next phase?
- **Required inputs**: full artifact set, gate results, decision records,
  verification evidence, final disposition.
- **Artifacts checked**: all mandatory mode artifacts plus run manifest and
  verification evidence.
- **What blocks progress**: missing artifacts, missing approvals, incomplete
  verification, unresolved open questions that invalidate the output contract.
- **Approval required**: proportional human approval; mandatory named owner for
  systemic impact.
- **Applicability**: all modes.

## 11. Verification Model

Verification in Canon is layered. It is not reducible to "tests
passed." A single reasoning path is insufficient because generated artifacts
can agree with each other while sharing the same hidden mistake.

The minimum layers in v0.1 are:

- **Self-critique**: the cheapest first pass. It asks the generator to surface
  obvious gaps before human attention is spent.
- **Adversarial critique**: a separate critical pass that assumes the proposed
  output is wrong and looks for weak assumptions, missing edge cases, and
  hidden failure modes.
- **Peer review**: human scrutiny of logic, contracts, and completeness.
- **Architectural review**: human scrutiny of boundaries, invariants, and
  system-level coherence.

Why a single reasoning path is insufficient:

- generated code and generated tests can mirror the same invented contract
- generated explanations can justify the same mistaken assumptions that shaped
  the output
- generated review summaries can collapse into stylistic reassurance unless a
  separate challenge path is introduced

Verification scales with risk and zone:

- **Green + Low impact**: self-critique plus proportional human review
- **Yellow or Bounded impact**: self-critique, adversarial critique, and peer
  review are mandatory
- **Red or Systemic impact**: all four layers are mandatory, and named human
  ownership is required before readiness can pass

## 12. Decision Memory

Decision memory is the mechanism that turns transient reasoning into durable
organizational knowledge. The system must preserve:

- context
- options considered
- rejected paths
- rationale
- consequences
- unresolved questions
- owner or approver

Decision Records are mandatory when:

- risk is classified as systemic
- a boundary, invariant, or contract changes or is intentionally deferred
- a mode output creates an architectural or governance consequence
- a review identifies decision impact that is not already recorded
- a human overrides a gate or accepts a known risk

The role of decision memory is not archival completeness. It is reconstructable
provenance. Future engineers must be able to understand why a path was chosen
without reopening private chat history or reverse-engineering intent from code.

## 13. System Architecture for v0.1

### Method Layer

Responsibilities:

- define modes, methods, steps, artifact contracts, stop conditions, and exit
  criteria
- bind run execution to an explicit method definition

Boundaries:

- may not call external tools directly
- may not bypass policy decisions or persistence

### Policy Layer

Responsibilities:

- classify risk and usage zone
- evaluate gates
- determine required approvals and verification layers

Boundaries:

- may not generate business artifacts as a substitute for policy outcomes
- may not allow execution to outrun classification

### Execution Layer

Responsibilities:

- invoke adapters for external tools and repositories
- gather bounded context from local files, diffs, and declared capabilities
- operate across the multimodule Rust workspace without making Rust itself the
  product identity

Boundaries:

- may not decide policy
- may not write untracked outputs outside the artifact contract

### Artifact Layer

Responsibilities:

- persist artifacts, run manifests, decisions, traces, and evidence
- expose durable outputs as inputs to later steps

Boundaries:

- may not authorize progression by itself
- may not treat chat transcripts as canonical memory

### Review Layer

Responsibilities:

- run self-critique, adversarial critique, peer review, and architectural
  review hooks
- attach review outputs to the run and artifact set

Boundaries:

- may not silently mutate the primary output without producing review artifacts
- may not collapse review into generic summary language

The architecture governs external tools. It does not replace them. Copilot CLI,
shell, GitHub, MCP tools, linters, and test runners remain execution surfaces.
Canon is the control plane that determines when, why, and under what
constraints they may be used.

## 14. Filesystem and Persistence Model

v0.1 persists locally in the repository under a dedicated `.canon/` directory.

```text
.canon/
  sessions/
  artifacts/
  decisions/
  traces/
  methods/
  policies/
  runs/
```

Directory responsibilities:

- **`sessions/`**: lightweight session context, referenced inputs, and local
  session metadata.
- **`artifacts/`**: mode-specific artifact bundles produced by runs.
- **`decisions/`**: decision records, approvals, reversibility notes, and
  decision history.
- **`traces/`**: adapter invocation logs, lightweight reasoning traces,
  capability use, and evidence references.
- **`methods/`**: method definitions and output contracts available to the
  engine.
- **`policies/`**: risk, zone, gate, and approval policies.
- **`runs/`**: run manifests, lifecycle status, links to artifacts, gate
  results, and final disposition.

Each run persists:

- run identifier and timestamp
- repo and context references
- selected mode
- risk and zone classifications
- artifact contract
- gate outcomes
- adapter invocations and side-effect classification
- approvals and owners
- verification results
- final output disposition

This model supports auditability because a run can be reconstructed from local
artifacts and manifests rather than from memory alone. It supports reuse
because future runs can reference prior artifacts and decisions directly.

## 15. Execution Flow

The standard flow of a run in v0.1 is:

1. **Mode selection**
2. **Context capture**
3. **Risk classification**
4. **Usage-zone classification**
5. **Artifact contract creation**
6. **Gated analysis and planning**
7. **Constrained execution or recommendation generation**
8. **Layered verification**
9. **Decision recording**
10. **Final artifact emission**

Mode-specific adjustments:

- **Requirements mode** ends in bounded product-definition artifacts rather
  than implementation recommendations by default.
- **Brownfield-change mode** inserts legacy preservation and system-slice work
  before planning and treats execution planning as the main output contract.
- **PR review mode** starts from diff ingestion, produces review artifacts, and
  ends with a disposition rather than with execution.

### Operational Edge Conditions

- If risk classification and usage-zone classification conflict, the stricter
  control regime wins.
- If an adapter required by the current step is unavailable, the run either
  stops or degrades to recommendation-only mode; it does not silently continue
  as if execution succeeded.
- If a rerun reuses prior artifacts, the run manifest must link to the prior
  run instead of overwriting provenance.
- If required artifacts are partially present from an interrupted run, the next
  run must explicitly confirm reuse, refresh, or rejection before readiness can
  pass.

## 16. External Tooling and Adapters

v0.1 may integrate with:

- Copilot CLI
- shell commands
- MCP-compatible tools
- GitHub APIs
- test runners
- linters
- file system operations

Adapter philosophy:

- adapters are execution surfaces, not the product's identity
- each adapter must declare capability, side-effect profile, and trace output
- adapters may be read-only or state-changing; the method and policy layers
  decide when that capability is allowed
- adapter results must be attached to the run as evidence or trace data

The engine therefore remains tool-agnostic at the governance layer while still
being explicit about what capabilities are active in a given run.

## 17. Product Differentiation

Canon differs from adjacent categories in one decisive way:
governance is the product, not a side effect.

- **Versus generic agent frameworks**: it optimizes for bounded method, risk,
  artifacts, gates, and auditability, not for agent orchestration volume.
- **Versus prompt libraries**: it enforces workflow contracts, persistence, and
  approvals instead of distributing reusable text snippets as if they were
  enough.
- **Versus pure spec-driven development tools**: it adds risk classification,
  usage zones, bounded autonomy, decision memory, and layered verification
  across the run, not only structured planning artifacts.
- **Versus skill and workflow packs**: it governs the lifecycle above tool
  usage and preserves decisions and evidence as first-class outputs.

The differentiators are:

- risk classification
- decision gates
- artifact contracts
- bounded autonomy
- layered verification
- decision memory

## 18. v0.1 Scope and MVP Boundaries

v0.1 should be implemented with depth, not breadth. The MVP must center on:

- the three initial modes: `requirements`, `brownfield-change`, and
  `pr-review`
- artifact generation contracts for those modes
- pre-generation risk classification
- usage-zone classification
- gate evaluation
- decision records
- layered verification hooks
- local filesystem persistence under `.canon/`
- adapter support for local filesystem, shell, repository context, and at least
  one AI execution surface

The MVP must explicitly defer:

- additional modes beyond the initial three
- centralized hosted control planes
- generic plugin marketplace behavior
- full autonomous code execution across Red zone work
- IDE-first user experience
- broad cross-organization reporting dashboards

## Requirements *(mandatory)*

- **FR-001**: The system MUST require explicit mode selection before a run can
  start.
- **FR-002**: The system MUST classify risk before generation or execution.
- **FR-003**: The system MUST classify usage zone and apply the stricter of
  zone or risk constraints.
- **FR-004**: The system MUST create and persist an artifact contract before
  the first gate can pass.
- **FR-005**: The system MUST enforce mode-specific required artifact sets.
- **FR-006**: The system MUST persist run manifests, gate results, approvals,
  traces, and final dispositions locally.
- **FR-007**: The system MUST require durable decision records for structural
  or systemic-impact decisions.
- **FR-008**: The system MUST separate generation and validation for
  non-trivial outputs.
- **FR-009**: The system MUST provide layered verification hooks.
- **FR-010**: The system MUST govern external tool usage through adapters
  rather than direct untracked execution.
- **FR-011**: `requirements` mode MUST require bounded options, explicit
  trade-offs, and scope cuts before completion.
- **FR-012**: `brownfield-change` mode MUST require legacy invariants before
  execution planning.
- **FR-013**: `pr-review` mode MUST emit the full review artifact set and a
  final review summary.
- **FR-014**: Systemic-impact or Red zone work MUST require named human
  ownership.

## 19. Acceptance Criteria

- **AC-001**: The system cannot start a run without a mode.
- **AC-002**: The system cannot proceed beyond initial context capture without
  explicit risk classification.
- **AC-003**: The system cannot proceed beyond initial context capture without
  explicit usage-zone classification.
- **AC-004**: Required artifacts must be persisted before a gate can pass.
- **AC-005**: `requirements` mode must emit the six defined requirement
  artifacts before the run is marked complete.
- **AC-006**: `brownfield-change` mode must require
  `legacy-invariants.md` before `implementation-plan.md` can pass readiness.
- **AC-007**: `pr-review` mode must emit the structured review artifact set and
  a disposition summary.
- **AC-008**: Systemic-impact work must require explicit human ownership.
- **AC-009**: Red zone work cannot auto-complete without elevated verification
  and human approval.
- **AC-010**: Decision records must be present for structural decisions and
  linked from the run manifest.
- **AC-011**: Verification evidence must show at least one challenge layer that
  is distinct from the original generation path.
- **AC-012**: Adapter invocations must be traceable from the run manifest.
- **AC-013**: Brownfield mode must persist a bounded change surface before
  recommending execution.
- **AC-014**: Review findings that affect boundaries, contracts, or missing
  verification must remain explicit in `review-summary.md` until disposition.

## 20. Open Questions

- Should methods be code-defined, config-defined, or hybrid?
- How strict should gates be at runtime when artifacts are incomplete but a
  human explicitly wants to proceed?
- How should adapters expose capabilities safely without turning capability
  metadata into a shadow policy language?
- How should human approvals be represented in a way that is lightweight but
  still durable and auditable?
- How should artifact schemas be versioned so that old runs remain readable
  after method evolution?
- How should reruns relate to prior decision memory: reuse, fork, or inherit?
- How much trace detail is enough to preserve consequential reasoning without
  creating transcript bureaucracy?
- Should Red zone execution be permanently recommendation-only in v0.1, or only
  recommendation-only for specific adapter classes?

## Success Criteria *(mandatory)*

- **SC-001**: In pilot usage, 100% of runs require mode, risk, and zone
  selection before the first governed step can proceed.
- **SC-002**: In pilot usage, 100% of systemic-impact runs require both a
  decision record and named human ownership.
- **SC-003**: For the three v0.1 modes, the engine emits the complete required
  artifact bundle in 100% of successful runs.
- **SC-004**: In pilot reviews, engineers can reconstruct the governing
  rationale of a completed run from persisted artifacts alone without relying on
  chat history in at least 90% of cases.

## Validation Plan *(mandatory)*

- **Structural validation**: confirm required sections, artifact contracts,
  gate definitions, risk classes, usage zones, and persistence model are all
  present and internally consistent.
- **Logical validation**: walk the three operational scenarios and verify that
  mode behavior, gates, artifacts, and acceptance criteria align.
- **Independent validation**: require peer review of the specification plus a
  separate adversarial critique before implementation planning.
- **Evidence artifacts**: this specification, the requirements checklist,
  review notes, and later planning artifacts under the generated feature
  directory.

## Decision Log *(mandatory)*

- **D-001**: v0.1 defines Canon as a governed method engine layered
  above external tools rather than as a general-purpose AI assistant.
  **Rationale**: the product's differentiator is governability, not raw
  generation.
- **D-002**: v0.1 starts with `requirements`, `brownfield-change`, and
  `pr-review` only. **Rationale**: these three modes establish the core product
  loop of definition, containment, and verification without diffusing the MVP.
- **D-003**: Local filesystem persistence is part of the MVP. **Rationale**:
  auditability and decision memory must exist before broader platform ambitions
  are considered.

## Assumptions

- The first implementation will run locally against a multimodule Rust
  repository and related engineering artifacts, but the product identity remains
  governance rather than language tooling.
- Teams using v0.1 can identify a named human owner for systemic-impact work.
- External tools and AI execution surfaces may vary by team, but the adapter
  contract remains stable enough to capture capability, traces, and side
  effects.
- v0.1 prioritizes durable behavior and auditability over polished UX or broad
  configurability.
