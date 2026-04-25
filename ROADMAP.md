# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`, and
`migration`. The next roadmap should prioritize output quality, packaging,
distribution, and authoring improvements instead of reopening already
delivered mode surfaces.

## Feature: Domain Modeling And Boundary Design

### Outcome

Canon strengthens the already-delivered shaping and architecture modes by
making Domain-Driven Design outputs first-class: explicit ubiquitous language,
bounded contexts, context relationships, and domain invariants that can flow
into later architecture, change planning, and review.

### Modes In Scope

- `system-shaping`
- `architecture`
- `change`

### Why These Modes Belong Together

- They are the modes where domain boundaries, ownership, and invariants matter
  more than raw implementation sequencing.
- They benefit from a shared language for bounded contexts, context crossings,
  and preserved business rules.
- They reduce downstream drift by making the domain model explicit before
  execution-heavy modes begin.

### Continuity With Current Mode Model

- `system-shaping` becomes the primary entry point for domain discovery when a
  capability's structure is not yet fixed.
- `architecture` formalizes domain boundaries and relationships once the
  structure is being settled.
- `change` enforces domain invariants during bounded modification of an
  existing system.

This extends the current mode model instead of introducing a separate planning
vocabulary for domain work.

### First Slice

- Extend `system-shaping` to emit a candidate domain map, core-domain
  hypotheses, and a ubiquitous-language seed.
- Extend `architecture` to emit bounded-context and context-relationship
  artifacts, including integration seams and anti-corruption candidates where
  the decision surface warrants them.
- Extend `change` to tie the change surface to an explicit domain
  slice, preserved invariants, and ownership boundaries instead of treating the
  system as one undifferentiated code surface.
- Keep the existing critique-first posture so Canon challenges weak or blurry
  domain boundaries instead of rubber-stamping them.

### Why This Feature Comes After Mode Completion

- It improves the quality of modes Canon already ships before the roadmap
  expands into packaging or protocol work.
- It should follow the now-complete mode-coverage slice because it strengthens
  existing workflows instead of closing a missing one.

## Delivered Recently

- `architecture` now extends its packet with three C4 artifacts —
  `system-context.md` (Architecture + Exploration gates), `container-view.md`
  (Architecture gate), and `component-view.md` (Architecture +
  ReleaseReadiness gates). The renderer preserves authored `## System Context`,
  `## Containers`, and `## Components` sections verbatim and emits an explicit
  `## Missing Authored Body` block referencing the canonical heading whenever
  the brief omits a section. Existing decision, invariant, tradeoff, boundary,
  and readiness artifacts are unchanged.

- `incident` now ships as a governed operational mode that emits a six-artifact
  containment packet, remains recommendation-only, can stop for explicit risk
  approval, and publishes readable packets under `docs/incidents/<RUN_ID>/`
  even when the packet is approval-gated or blocked.
- `migration` now ships as a governed operational mode that emits a six-artifact
  compatibility packet, remains recommendation-only, blocks explicitly on
  missing fallback credibility, and publishes readable packets under
  `docs/migrations/<RUN_ID>/` even when the packet is blocked or approval-gated.
- `backlog` is no longer a roadmap candidate. It now ships as a governed mode
  that publishes to `docs/planning/<RUN_ID>/` and preserves downstream handoff
  context through epics, slices, dependencies, sequencing, acceptance anchors,
  and planning risks.
- `canon-backlog` skill now requires the assistant to author the real backlog
  body (Epic Tree, Capability To Epic Map, Dependency Map, Delivery Slices,
  Sequencing Plan, Acceptance Anchors, Planning Risks) before invoking Canon,
  and the renderer preserves those authored sections verbatim instead of
  emitting templated placeholders.
- `pr-review` now emits `conventional-comments.md` as a reviewer-facing
  companion to `review-summary.md`, publishes it under
  `docs/reviews/prs/<RUN_ID>/`, and preserves the existing review-disposition
  gate and primary artifact semantics.

## Feature: Mode Authoring Specialization (Skills As Real AI Authors)

### Outcome

Every governed Canon mode that produces a multi-artifact packet treats the AI
assistant as the actual author of the per-artifact body, the same way
`canon-backlog` and SpecKit's `/speckit.tasks` do today. Canon stays the
governor, validator, and persister; the model stays responsible for producing
real, source-grounded content for each artifact, not a templated echo of the
brief.

### Problem We Are Solving

- Today most generative modes ship a *Clarification Loop* and a *Provenance
  Sidecar*, but they do not enforce a per-artifact authored body shape.
- When the AI submits a thin brief, the renderer fills artifacts with generic
  scaffolding ("Establish a bounded foundation", "Deliver visible slices"),
  which the user correctly perceives as Canon "producing nothing".
- The fix already shipped for `backlog` is a repeatable pattern: required H2
  sections per artifact, renderer preserves authored sections verbatim, and an
  explicit `## Missing Authored Body` marker when the model failed to author
  the section.

### Modes In Scope

- `requirements`
- `discovery`
- `system-shaping`
- `architecture`
- `change`
- `implementation`
- `refactor`
- `review`
- `verification`
- `pr-review`
- `incident`
- `migration`

### First Slice

- Define a shared `Author <Mode> Body Before Invoking Canon` skill section that
  enumerates the required H2 sections for each emitted artifact in that mode,
  mirroring the new `canon-backlog` skill section.
- For decision-heavy modes, require authored `## Options Considered`,
  `## Recommended Option`, and `## Rejected Alternatives` sections so Canon
  preserves real tradeoffs instead of only the final answer.
- Update each per-mode renderer in `crates/canon-engine/src/artifacts/markdown.rs`
  to extract the authored H2 sections via `extract_marker` and render them
  verbatim, falling back to a placeholder + explicit `## Missing Authored Body`
  marker only when the section is absent.
- Pass the raw authored `context_summary` through to the evidence-backed
  summary in each `mode_*` orchestrator file so the renderer actually sees the
  authored body.
- Keep the existing critique-first posture and Provenance Sidecar.

### Why This Feature

- It generalizes the only fix that turned `backlog` from a placeholder
  generator into a real backlog producer.
- It keeps Canon honest: when the AI did not do the work, the artifact says so
  out loud instead of pretending.
- It is strictly additive to the current orchestrator and skill structure; no
  domain model changes required.

## Feature: Industry-Standard Artifact Shapes

### Outcome

When a Canon mode has a well-known industry artifact shape, the skill guides
the AI to author the body in that shape, and the renderer recognizes and
preserves it. Users can trust that delivered `pr-review` already produces real
Conventional Comments, and future slices should make `architecture` produce a
real C4-style packet, `requirements` produce a real PRD, `change` produce a
real ADR, and so on. Where the mode ends in a real engineering decision, the
artifact shape should also make the alternatives visible instead of only
preserving the winning choice.

### Problem We Are Solving

- The AI authoring skills today are domain-neutral and underspecify artifact
  shape, so output drifts toward Canon-internal headings instead of the shapes
  reviewers and engineers already read elsewhere.
- The delivered `pr-review` Conventional Comments slice proved the pattern, and
  the remaining architecture C4 work plus other mode-specific shapes should now
  bind that direction into the broader authoring contract.

### Mode To Shape Mapping (First Slice)

- `requirements` → PRD shape (Problem, Outcomes, Users, Use Cases, Constraints,
  Success Metrics, Open Questions, Out of Scope) and a Lean Canvas seed
  artifact when the input is product-shaped rather than engineering-shaped.
- `discovery` → Opportunity Solution Tree seed (Outcome, Opportunities,
  Solutions, Assumption Tests) and a Jobs-To-Be-Done framing when the input
  surface supports it.
- `system-shaping` → Domain map seed (bounded contexts, ubiquitous language,
  core vs supporting subdomains) aligned with the upcoming
  `Domain Modeling And Boundary Design` feature, plus candidate structural
  patterns (modular monolith vs services, sync orchestration vs events, etc.)
  with pros/cons when the source leaves room for a real choice.
- `architecture` → C4 model (System Context, Container, Component) plus an ADR
  per architecturally significant decision, alongside the existing critique
  artifacts and explicit `Options Considered`, `Decision Drivers`, `Pros`,
  `Cons`, and `Operational Tradeoffs` sections.
- `change` → ADR-shaped decision record (Context, Decision, Status,
  Consequences) attached to the change surface, with a design-pattern-choice
  appendix when the change materially hinges on choosing Strategy vs State,
  pipeline vs direct composition, adapter vs direct integration, and similar
  bounded alternatives.
- `implementation` → Task mapping plus a contract test plan shape and an
  Implementation Notes shape that links each task to the bounded slice it
  implements, plus a framework/library evaluation dossier when execution
  depends on choosing a concrete stack.
- `refactor` → Preserved Behavior matrix in invariant-vs-mechanism form, plus
  a structural-rationale ADR.
- `review` → Findings shape compatible with reviewer workflows (Severity,
  Location, Rationale, Recommended Change).
- `pr-review` → Conventional Comments shape is now the delivered reference
  implementation for reviewer-facing standardization.
- `verification` → Claims/Evidence/Independence matrix.

### First Slice

- Pick three high-leverage modes for the next pass: `architecture` (C4 + ADR),
  `requirements` (PRD), and `change` (ADR + pattern-choice appendix).
- For each, extend the skill with the required H2 sections in the chosen
  industry shape, extend the renderer to recognize and preserve those
  sections, and add per-shape unit tests.
- Defer the remaining modes to a second slice once the first three prove the
  authoring + renderer contract.

### Why This Feature

- It makes Canon outputs directly readable in the workflows users already have
  (Backstage docs, ADR repos, PR review tooling).
- It reduces the chance that the AI invents idiosyncratic structure per run.
- It strengthens the existing modes instead of widening Canon into new runtime
  domains.

### Relationship To Existing Features

- Builds on the now-delivered `architecture` C4 packet by generalizing
  community-standard artifact shapes (ADRs, RFCs, runbooks) into the rest of
  Canon's mode coverage, and generalizes the delivered `pr-review`
  Conventional Comments pattern into other modes.
- Composes with `Domain Modeling And Boundary Design` for `system-shaping`.
- Provides the natural artifact homes for the option-analysis feature below,
  especially `Options Considered`, `Why Not`, and evaluation dossiers.

## Feature: Decision Alternatives, Pattern Choices, And Framework Evaluations

### Outcome

Canon becomes a real engineering companion for decision-heavy work: instead of
jumping straight to one recommendation, it presents 2 to 4 viable
alternatives, compares them explicitly, and recommends one with clear
rationale grounded in the user's constraints, the real source surface, and
observable ecosystem signals.

### Problem We Are Solving

- Today Canon is strongest when the user already knows the shape of the answer,
  but many high-value engineering conversations are about choosing between
  multiple valid approaches before implementation starts.
- An architecture packet, ADR, or implementation plan that records only the
  selected option loses most of the reasoning value. Teams later ask: what did
  we reject, why, and what would make us revisit that decision?
- Framework and library recommendations are especially weak if they do not
  compare ecosystem health, feature fit, maturity, and operational cost.

### Decision Surfaces In Scope

- Architecture choice: modular monolith vs service split, synchronous API vs
  event-driven, relational vs document vs stream-first persistence, hosted vs
  self-managed infrastructure.
- Design-pattern choice: Strategy vs State, Observer vs domain events,
  Decorator vs middleware pipeline, adapter layer vs direct integration,
  orchestrator vs choreography.
- Framework / library / platform choice: web framework, UI stack, ORM, queue,
  auth provider, workflow engine, search engine, test runner, scanner suite,
  and similar bounded stack decisions.
- Build-vs-buy and OSS-vs-commercial choices where Canon stays
  recommendation-only and keeps explicit approval boundaries.

### Required Comparison Packet Shape

- `decision-summary.md` — the decision to be made, current constraints, the
  recommended option, and the trigger that would reopen the decision.
- `options-matrix.md` — 2 to 4 viable options with explicit pros, cons, risks,
  and fit against the named constraints.
- `tradeoff-analysis.md` — decision drivers, non-goals, irreversible costs,
  lock-in risk, migration pressure, operational burden, and team-skill impact.
- `ecosystem-health.md` — maintenance cadence, release freshness, contributor
  spread, issue responsiveness, stars/forks as weak signals only,
  documentation quality, plugin ecosystem, and LTS/commercial support when
  relevant.
- `adoption-and-migration.md` — migration complexity, coexistence strategy,
  exit cost, training burden, backward-compatibility impact, and rollout risk.
- `decision-evidence.md` — links to source artifacts, package registries,
  project metadata, release notes, GitHub activity, benchmark references, and
  user-provided constraints.

### Evaluation Rules

- Canon must never pretend a comparison exists when the choice is already
  constrained to one viable option; it should say that the decision is already
  materially closed.
- Stars are advisory only and must never be the decisive signal on their own.
- Framework recommendations should account for maintenance status, release
  cadence, maturity, feature fit, documentation quality, ecosystem depth,
  licensing, security posture, operational burden, and migration cost.
- When live ecosystem data is missing, stale, or unavailable, Canon marks
  `## Missing Evidence` instead of inventing confidence.
- Closed-source or paid products may be compared when the user allows them,
  but OSS and commercial options must be labeled clearly and evaluated against
  the same declared constraints.

### First Slice

- Add an option-analysis shape to `architecture`: `Decision Drivers`, `Options
  Considered`, `Pros`, `Cons`, `Recommendation`, and `Why Not The Others`.
- Add a pattern-selection shape to `system-shaping` and `change` for
  pattern-heavy problems where structure is the real decision.
- Add a framework/library evaluation shape to `implementation` and
  `migration` when the bounded task includes selecting a concrete stack.
- Add read-only evidence collectors for registry, GitHub, release, and
  project-health signals so comparisons are backed by real evidence rather
  than vibes.
- Keep final selections recommendation-only in v0.x.

### Why This Feature

- It turns Canon from a packet generator into a real decision companion.
- It creates durable reasoning artifacts teams can revisit months later when a
  framework, pattern, or platform choice stops fitting.
- It reduces hindsight debates because rejected options remain visible instead
  of disappearing from the record.

### Relationship To Existing Features

- Depends on *Mode Authoring Specialization* so option packets are authored
  explicitly rather than inferred from a final choice.
- Composes with *Industry-Standard Artifact Shapes* because ADRs, PRDs, and
  architecture packets are the natural homes for `Options Considered` and
  tradeoff analysis.
- Feeds *Supply Chain And Legacy Analysis Mode* when ecosystem health,
  maintenance posture, and migration cost matter to the recommendation.

## Feature: Cybersecurity Risk Assessment Mode

### Outcome

Canon governs an explicit security-risk-assessment workflow that produces a
real, reviewable threat model and risk register grounded in the source the AI
actually read, instead of leaving security as an implicit concern of other
modes.

### Mode

- New mode: `security-assessment` (working name; final name to be decided
  during the planning run).

### Why A Dedicated Mode

- Security risk assessment is a recognized engineering workflow with its own
  artifact shapes (threat model, risk register, mitigation plan) and its own
  reviewers.
- Folding it into `architecture` or `change` hides it and produces weaker
  output than a dedicated mode with a critique-first posture aimed
  specifically at threats and mitigations.
- It composes with the rest of Canon: `architecture` defines the surface,
  `security-assessment` interrogates that surface for threats, `change` and
  `implementation` carry the mitigations into bounded execution.

### First Slice Artifact Contract

- `assessment-overview.md` — bounded scope, in-scope assets, trust boundaries,
  data classifications, and out-of-scope assets.
- `threat-model.md` — STRIDE-shaped threats per trust boundary or asset, with
  attacker-goal framing.
- `risk-register.md` — likelihood × impact rated risks with owner, status, and
  source trace links.
- `mitigations.md` — proposed controls, mapped to risk-register entries, with
  preserved-vs-changed-behavior framing.
- `assumptions-and-gaps.md` — explicit unverified assumptions, missing telemetry,
  and unobservable surfaces.
- `compliance-anchors.md` — references to applicable standards (OWASP ASVS,
  CIS, ISO 27001 control families, GDPR articles when in scope) without
  pretending Canon performs a compliance audit.
- `assessment-evidence.md` — independent verification notes, separate from the
  generation lineage.

### Required Authored H2 Sections

The assistant must author the body before invoking Canon, mirroring the
backlog pattern: scope, asset inventory, trust boundaries, STRIDE-per-boundary,
risk register, mitigation plan, assumptions, and compliance anchors. Generic
boilerplate fails the run.

### Required Inputs

- `RISK`
- `ZONE`
- at least one of: an architecture run id, an authored architecture packet, or
  a real source surface (code paths, infra-as-code, threat-relevant configs).

### Why This Feature

- Closes a real gap: today `architecture` and `change` mention security as a
  constraint but do not produce a reviewable security artifact.
- Gives downstream `pr-review`, `verification`, and `incident` a concrete
  upstream to reference.
- Stays within Canon's recommendation-only posture for v0.x; mitigations are
  proposed, not auto-applied.

### Why This Order

- It depends on the *Mode Authoring Specialization* and *Industry-Standard
  Artifact Shapes* features so it can be authored as a real STRIDE/risk
  packet from day one rather than as a placeholder mode.

## Feature: Supply Chain And Legacy Analysis Mode

### Outcome

Canon governs a bounded analysis of an existing codebase's supply chain and
legacy posture, producing a reviewable packet with the SBOM, known-vulnerability
triage, license-compliance posture, and a legacy-modernization snapshot.
Canon does not reimplement scanners; it orchestrates established tools as
governed adapters and turns their raw output into a critique-first artifact
set.

### Mode

- New mode: `supply-chain-analysis` (working name; final name to be decided
  during the planning run).

### Why A Dedicated Mode

- SBOM, CVE triage, license compatibility, and legacy-modernization framing
  are all reads against an existing system, not changes to it. They fit
  Canon's analysis posture, not its execution posture.
- Today no Canon mode produces a defensible answer to "is it safe and legal to
  ship this dependency surface, and what does the legacy debt look like?"
- Folding this into `architecture`, `change`, or `security-assessment` hides
  the supply-chain dimension and dilutes each mode's purpose.

### Tool Composition (Recommendation, Not Reimplementation)

- SBOM generation: Syft (or equivalent) per ecosystem.
- Vulnerability triage: OSV-Scanner, Grype, GitHub Advisory Database lookups,
  and ecosystem-native sources such as `cargo audit`, `npm audit`, `pip-audit`,
  `govulncheck`.
- License analysis: ScanCode Toolkit, `cargo deny`, `license-checker`, or
  ecosystem-native equivalents, with explicit policy for commercial vs OSS
  compatibility.
- Legacy posture: ecosystem-native version, EOL, and abandonment signals
  (e.g. `cargo outdated`, `npm outdated`, `pip list --outdated`,
  endoflife.date references).

Each tool runs as a governed adapter invocation through the existing
request/decision/evidence pipeline. Canon never invents scanner output and
never claims a vulnerability without a tool-backed source.

### Pre-Run Clarification Loop

Before invoking any scanner, the assistant MUST run a short clarification
loop with the user when required information is not present in the authored
brief. Defaults are never silently assumed for choices that change the
license-compatibility verdict or the toolchain that gets executed.

Required clarifications, asked in this order, only when the brief does not
already answer them:

1. **Project licensing posture.** Ask whether the project is `commercial`
   (proprietary, distributed under a non-OSI license), `oss-permissive`
   (MIT/Apache/BSD class), `oss-copyleft` (GPL/AGPL/MPL class), or `mixed`.
   This drives `license-compliance.md`. There is no default.
2. **Distribution model.** Ask whether dependencies will be distributed
   (binary shipped, container image published, library released) or only
   used internally. Copyleft obligations and SBOM expectations differ.
3. **Ecosystem confirmation.** Show the ecosystems Canon detected from
   manifests and ask the user to confirm or remove any that are
   intentionally out of scope.
4. **Out-of-scope components.** Ask for vendored, third-party, or generated
   directories that should be excluded from the scan surface.
5. **Tool licensing preference.** Ask whether the user authorizes Canon to
   propose installation of scanners that are not OSI-approved open source
   (for example, source-available, free-tier-only, or commercial scanners).
   The default is `oss-only`. Canon never proposes a non-OSS scanner unless
   the user opts in here, and even then only as an alternative alongside an
   OSS option.

The clarification loop must batch these questions (3 to 5 per round, never
more than 7), accept inline answers, and write the user's responses into
the run's provenance sidecar so the policy decisions per finding can be
traced back to a real authored choice rather than to an inferred default.

If the user refuses to answer a question that materially changes the verdict
(licensing posture, tool licensing preference), the run stops with an
explicit `## Missing Authored Decision` marker. Canon does not guess.

### Toolchain Detection And Install Prompting

Canon must detect which scanners are needed for the ecosystems present in the
repository, check whether each one is on `PATH`, and prompt the user for an
explicit installation decision when a required scanner is missing. The
assistant never installs anything silently and never bypasses the missing
tool by inventing the scan result.

- Ecosystem detection is driven by the manifests present in the source
  surface (`Cargo.toml`, `package.json` / `pnpm-lock.yaml` / `yarn.lock`,
  `pyproject.toml` / `requirements.txt` / `poetry.lock`, `go.mod`, `pom.xml`
  / `build.gradle`, `Gemfile`, `composer.json`, `*.csproj`, `pubspec.yaml`,
  etc.). Each detected ecosystem maps to a required scanner set.
- For each missing scanner, Canon emits a structured prompt that includes:
  the scanner name, the ecosystem it serves, why it is needed, the supported
  install commands per platform (Homebrew, `winget`, `apt`, language-native
  installers), the upstream project URL, and the SPDX license identifier.
- Canon proposes installation **only for OSI-approved open source tools** by
  default. Closed-source, source-available, or commercially-licensed scanners
  are never auto-suggested by Canon. The user can opt in to non-OSS scanner
  proposals through the *Pre-Run Clarification Loop*; even then, Canon must
  also surface an OSS alternative whenever one exists, and must record the
  user's authorization in the provenance sidecar.
- The user's install decision is recorded as a Canon decision artifact with
  three possible outcomes: `installed` (with the install command actually
  run), `skipped` (with rationale, and the affected scanner output marked as
  `unavailable` in the packet), or `replaced` (the user pointed Canon at an
  alternative OSS scanner that covers the same capability).
- When a scanner is skipped, the corresponding sections of
  `vulnerability-triage.md`, `license-compliance.md`, or `legacy-posture.md`
  must carry an explicit `## Coverage Gap` marker naming the missing scanner
  and the ecosystem left uncovered, so downstream readers see the gap
  instead of assuming the surface was scanned.
- Installation itself stays a user action. Canon shows the command,
  Canon does not execute the install. This preserves the recommendation-only
  posture and avoids privileged side effects in a governed run.

### First Slice Artifact Contract

- `analysis-overview.md` — bounded scope (which ecosystems, which manifests,
  which paths), commercial vs OSS posture declaration, and out-of-scope
  components.
- `sbom-bundle.md` — human-readable index pointing at machine SBOMs
  (CycloneDX or SPDX) emitted alongside the packet.
- `vulnerability-triage.md` — known vulnerabilities grouped by severity, with
  exploitability framing, affected component, fixed version, and triage
  decision (`accept`, `mitigate`, `defer with rationale`).
- `license-compliance.md` — license inventory grouped by compatibility class,
  flagged incompatibilities for the declared posture (commercial vs OSS),
  and license obligations the project must honor.
- `legacy-posture.md` — outdated, EOL, and abandoned dependencies with
  modernization recommendations bounded to slice level (no task breakdown).
- `policy-decisions.md` — explicit accept/reject decisions per finding, tied
  to the source manifest line and to a decision rationale.
- `analysis-evidence.md` — independent verification notes, tool versions,
  scan timestamps, and source manifests, separate from the generation
  lineage.

### Required Authored H2 Sections

The assistant must author the body before invoking Canon, mirroring the
backlog and security patterns: declared scope, commercial-vs-OSS posture,
ecosystems in scope, scanner selection rationale, triage decisions per
finding, modernization slices. Generic boilerplate fails the run.

### Required Inputs

- `RISK`
- `ZONE`
- a real source surface (manifest paths such as `Cargo.toml`, `package.json`,
  `pyproject.toml`, `go.mod`, `pom.xml`, or a repository root containing
  them).
- explicit declaration of commercial vs OSS posture so license compliance is
  evaluated against the right policy class.

### Why This Feature

- Closes a real gap: today Canon has no defensible answer for SBOM,
  vulnerability, or license posture.
- Makes Canon useful for commercial teams that must justify dependency
  choices and modernization roadmaps to stakeholders or auditors.
- Stays within Canon's recommendation-only posture: scans are evidence,
  triage decisions are authored, no automatic dependency rewrites.

### Relationship To Other Features

- Composes with `Cybersecurity Risk Assessment Mode`: supply-chain findings
  feed the threat model and risk register; the security mode does not
  duplicate scanner orchestration.
- Composes with `Refactor` and the delivered `Migration` mode: legacy posture
  produces the bounded slices those modes can pick up.
- Depends on the *Governed Execution Adapters* primitives already shipped, so
  scanner adapters slot into the existing capability and decision pipeline.

### Why This Order

- It depends on *Mode Authoring Specialization* so the packet has authored
  triage decisions instead of raw scanner dumps.
- It depends on *Industry-Standard Artifact Shapes* so SBOM, vulnerability,
  and license outputs use CycloneDX/SPDX, CVSS, and SPDX license identifiers
  rather than Canon-internal vocabulary.

## Feature: Distribution Channels Beyond GitHub Releases

### Outcome

Users can install Canon through familiar package-manager channels without
depending on manual archive download and placement.

### First Slice

- Support Homebrew as the first package-manager channel for macOS and Linux.
- Support `winget` as the primary Windows package-manager channel.
- Support Scoop as a secondary Windows channel.

### Deferred

- `apt` or Debian packaging, until Canon has a stable release and repository
  publishing process worth maintaining.

### Why This Feature

- GitHub Releases are already the canonical source of downloadable binaries.
- The next distribution step is reach and upgrade convenience, not basic release
  production.
- Homebrew gives the strongest first install story on macOS and a viable one on
  Linux.
- `winget` is the most credible first-class Windows distribution target.
- Debian packaging adds maintenance cost too early.

## Feature: Protocol Interoperability

### Outcome

Canon remains protocol-agnostic at the core while gaining a practical path to
govern structured external-tool invocation and future interoperable exposure.

### Direction

- Canon core must remain protocol-agnostic.
- First protocol adapter: MCP consumer support.
- Future external surface: a minimal MCP server.
- A2A remains a later architectural option, not a near-term implementation
  priority.

### First Slice

- Treat MCP as the first protocol Canon actively consumes for governed external
  tool invocation.
- Route MCP requests through the same request, decision, approval, and evidence
  pipeline used by other execution adapters.
- Keep policy, capability typing, lineage, and verification semantics shared
  across protocols.
- Only ship MCP runtime behavior if it fits the common invocation pipeline
  cleanly, without introducing protocol-specific special cases into the core.

### Future Minimal MCP Server Surface

- Start a governed run.
- Inspect run state and evidence.
- Request a structured review.
- Avoid exposing the full internal engine surface.

### Explicitly Deferred

- A2A support, unless Canon needs to operate as a remote interoperable agent.
- Agent discovery, remote delegation, and network-visible multi-agent topology.
- Broad protocol exposure beyond narrowly governed capabilities.

### Why This Order

- MCP solves tool and resource interoperability, which matches Canon's current
  need to govern calls to external tools.
- A2A solves agent-to-agent interoperability, which is a different layer of the
  system.
- Canon is currently closer to a governed local execution runtime than to a
  remote agent platform.
- Supporting MCP first strengthens the execution layer without forcing premature
  multi-agent architecture.

### Planning Principle

MCP first, A2A later if Canon becomes a network-visible agent.
