# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `migration`, and `supply-chain-analysis`. The next
roadmap should prioritize output quality, packaging, distribution,
existing-system assessment, publish structure, and authoring improvements
instead of reopening already delivered mode surfaces.

## Current Delivered Reference

The roadmap below is intentionally forward-looking. Delivered work is kept here
only as a compact baseline so future candidates have the right constraints and
continuity.

- requirements, discovery, change now ship the first slice of
  author-specialized mode guidance and remain in the delivered scope.
- `architecture` keeps the established persona baseline for
  `architect for C4/ADR work`; `requirements` keeps the baseline for
  `product lead for PRD work`; `change` keeps the baseline for
  `change owner for bounded change decisions`. Keep personas guidance-only.
- `review`, `verification`, `incident`, and `migration` now share canonical authored H2 contracts, and Mode Authoring Specialization is now complete for the currently modeled governed modes.
- `security-assessment` remains delivered and publishes readable packets under
  `docs/security-assessments/<RUN_ID>/`.
- `supply-chain-analysis` remains delivered and publishes readable packets
  under `docs/supply-chain/<RUN_ID>/`.

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
- The same skills also underspecify the authored persona and intended audience,
  so packets often read like generic AI summaries instead of work produced by
  a credible product, architecture, review, or operations counterpart.
- The delivered `pr-review` Conventional Comments slice proved the pattern, and
  the remaining architecture C4 work plus other mode-specific shapes should now
  bind that direction into the broader authoring contract.

### Mode To Shape Mapping (Roadmap Vision)

The mapping below describes the broader roadmap direction. The delivered first
slice remains explicitly limited to `requirements`, `architecture`, and
`change` as scoped in the subsection that follows.

- `requirements` → PRD shape (Problem, Outcomes, Users, Use Cases, Constraints,
  Success Metrics, Open Questions, Out of Scope) and a Lean Canvas seed
  artifact when the input is product-shaped rather than engineering-shaped,
  authored with an explicit product-facing persona.
- `discovery` → Opportunity Solution Tree seed (Outcome, Opportunities,
  Solutions, Assumption Tests) and a Jobs-To-Be-Done framing when the input
  surface supports it, authored with an explicit exploratory research persona.
- `system-shaping` → Domain map seed (bounded contexts, ubiquitous language,
  core vs supporting subdomains) aligned with the upcoming
  `Domain Modeling And Boundary Design` feature, plus candidate structural
  patterns (modular monolith vs services, sync orchestration vs events, etc.)
  with pros/cons when the source leaves room for a real choice, authored with
  an explicit system-design persona.
- `architecture` now delivers C4 model artifacts (System Context, Container,
  Component) plus an ADR-like decision packet that preserves `Decision`,
  `Constraints`, `Decision Drivers`, `Recommendation`, `Consequences`, and
  explicit option-analysis sections in the existing artifact family, authored
  with an explicit architecture-decision persona.
- `change` → ADR-shaped decision record (Context, Decision, Status,
  Consequences) attached to the change surface, with a design-pattern-choice
  appendix when the change materially hinges on choosing Strategy vs State,
  pipeline vs direct composition, adapter vs direct integration, and similar
  bounded alternatives, authored with an explicit change-planning persona.
- `implementation` → Task mapping plus a contract test plan shape and an
  Implementation Notes shape that links each task to the bounded slice it
  implements, plus a framework/library evaluation dossier when execution
  depends on choosing a concrete stack, authored with an explicit delivery
  lead persona.
- `refactor` → Preserved Behavior matrix in invariant-vs-mechanism form, plus
  a structural-rationale ADR, authored with an explicit preservation-focused
  maintainer persona.
- `review` → Findings shape compatible with reviewer workflows (Severity,
  Location, Rationale, Recommended Change), authored with an explicit reviewer
  persona.
- `pr-review` → Conventional Comments shape is now the delivered reference
  implementation for reviewer-facing standardization, authored with an explicit
  PR reviewer persona.
- `verification` → Claims/Evidence/Independence matrix authored with an
  explicit adversarial verifier persona.

### First Slice

- Pick three high-leverage modes for the next pass: `architecture` (C4 + ADR),
  `requirements` (PRD), and `change` (ADR + pattern-choice appendix).
- For each, extend the skill with the required H2 sections in the chosen
  industry shape, extend the renderer to recognize and preserve those
  sections, and add per-shape unit tests.
- Add an explicit persona layer to the same skills so the assistant authors as
  the right bounded counterpart for the packet: product lead for PRD work,
  architect for C4/ADR work, and change owner for bounded change decisions.
- Keep personas guidance-only: it may shape voice, emphasis, critique posture,
  and audience fit, but it must never override artifact contracts, invent
  authority, or weaken evidence requirements.
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

## Next Feature: 022 Decision Alternatives, Pattern Choices, And Framework Evaluations

### Outcome

Canon becomes a real engineering companion for decision-heavy work: instead of
jumping straight to one recommendation, it presents 2 to 4 viable
alternatives, compares them explicitly, and recommends one with clear
rationale grounded in the user's constraints, the real source surface, and
observable ecosystem signals.

This next slice also closes the most obvious persona gap left by the first
artifact-shapes rollout: the in-scope decision-heavy modes MUST declare a
credible authored counterpart so option packets read like bounded engineering
work rather than anonymous summaries.

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

### Modes And Persona Coverage In Scope

- `architecture` keeps the architecture-decision persona and strengthens it
  with explicit alternative analysis and reopening triggers.
- `system-shaping` uses a system-design persona that can compare structural
  patterns without pretending the structure is already fixed.
- `change` uses a change-planning persona that can compare bounded pattern and
  implementation alternatives while staying subordinate to invariants.
- `implementation` uses a delivery-lead persona that can evaluate concrete
  framework and library options against execution constraints.
- `migration` uses a migration-lead persona that can compare coexistence,
  modernization, and replacement paths without weakening compatibility gates.

Persona guidance remains guidance-only: it shapes audience fit, critique
posture, and decision framing, but it MUST NOT fabricate evidence, imply extra
authority, or override Canon's approval, risk, or missing-gap semantics.

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

- `architecture` now ships the option-analysis shape: `Decision Drivers`,
  `Options Considered`, `Pros`, `Cons`, `Recommendation`, and
  `Why Not The Others`, alongside ADR-like `Consequences` in the decision
  artifact.
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

### Follow-On Persona Completion

- `review` should read like a lead or staff software engineer performing a
  bounded code and design review.
- `pr-review` should read like a PR reviewer with explicit severity and change
  recommendation discipline.
- `verification` should read like an adversarial verifier challenging claims
  instead of restating them.
- `incident` should read like an incident commander or operational lead focused
  on containment credibility.
- `migration` and `implementation` should preserve their execution-facing
  personas consistently across future artifact-shape upgrades.

The first artifact-shapes release intentionally stopped at `requirements`,
`architecture`, and `change`. Future slices should treat persona coverage as a
normal part of mode shaping rather than a one-off exception.

## Remaining Roadmap Candidates

The roadmap beyond `022` should continue with the features below, ordered by
their fit with the already-delivered authoring and decision-support layers.

## Feature: Distribution Channels Beyond GitHub Releases

### Outcome

Users can install Canon through familiar package-manager channels without
depending on manual archive download and placement.

### Current Baseline

- `winget` is now the primary Windows package-manager channel.

### Follow-On Slices

- Support Scoop as a secondary Windows channel after `winget` is stable.
- Revisit Homebrew only when there is a concrete public tap and release
  automation plan worth maintaining.

### Deferred

- `apt` or Debian packaging, until Canon has a stable release and repository
  publishing process worth maintaining.

### Why This Feature

- GitHub Releases are already the canonical source of downloadable binaries.
- The next distribution step is reach and upgrade convenience, not basic release
  production.
- `winget` is the most credible first-class Windows distribution target.
- Debian packaging adds maintenance cost too early, and speculative protocol
  work should not displace concrete packaging or authoring improvements until a
  named interoperability target creates immediate value.

## Feature: System Assessment Mode

### Outcome

Canon gains a dedicated existing-system assessment mode that explains what a
system is today, with explicit evidence, confidence, coverage, and gaps,
without pretending the user is already making a new architecture decision.
The mode should use ISO 42010 as the reference frame for architectural views,
stakeholder concerns, and coverage reporting, while staying bounded to the
subset of views the first slice can support credibly.

### Distinction From `architecture`

- `architecture` remains the mode for authored decisions, tradeoffs,
  recommendations, and bounded C4 plus ADR packets.
- `system-assessment` becomes the read-only mode for understanding an existing
  codebase as-is before a user writes an architecture, change, migration, or
  security packet.

### First Slice

- Require `system_context=existing`.
- Keep the analysis read-only against the source tree and repository runtime
  surfaces.
- Emit an assessment packet with an executive summary, coverage map, evidence
  discipline, explicit `FACT` / `INFERENCE` / `GAP` findings, confidence per
  assessed surface, an asset inventory, and a risk register.
- Use ISO 42010 view language to name what was assessed, what was skipped, and
  what remains only partially covered.
- Start with the highest-value as-is views for engineering decisions:
  functional, component, deployment, technology, and integration, while making
  missing data, operations, security, or history coverage explicit instead of
  inventing it.
- Treat deeper full-spectrum assessment as a follow-on, not as a reason to
  over-widen the first slice.

### Why This Feature

- Existing-system users often need a trustworthy picture of the current system
  before they can author a meaningful `architecture` or `change` packet.
- The current `architecture` mode is intentionally decision-shaped and should
  not be overloaded into repo archaeology.
- Evidence-first assessment strengthens downstream `change`, `migration`,
  `security-assessment`, and architecture work by turning an unknown repo into
  a bounded, reviewable understanding surface.

## Feature: Structured External Publish Destinations

### Outcome

Published Canon packets land outside `.canon/` in human-browsable directory
structures with meaningful names and date-prefixed folders, while preserving
run-id traceability in packet metadata instead of making the run id the only
visible path anchor.

### First Slice

- Keep `.canon/` as runtime and evidence storage only.
- Standardize `publish` on external roots such as `docs/` rather than treating
  `.canon/` as the final reading surface.
- Replace run-id-only publish destinations with a canonical structure like
  `<publish-root>/<family>/<YYYY-MM-DD>-<descriptor>/`.
- Derive `<descriptor>` from the authored packet title or an explicit publish
  slug; fall back to the mode name only when no better label exists.
- Write publish metadata into the published packet so run id, mode, risk,
  zone, publish timestamp, and source artifact lineage stay recoverable.
- Preserve blocked or approval-gated publishing where the mode already allows
  it, but publish those packets under the same structured external contract.

### Why This Feature

- `.canon/` is the governed runtime surface; it should not be the only place
  humans browse final documents.
- Run-id-only paths are machine-traceable but poor for repository navigation,
  Git review, and long-term documentation browsing.
- Teams need to understand what a published packet was from the path alone,
  and a date-prefixed descriptor gives that without sacrificing traceability.
