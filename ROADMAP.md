# Next Features

This file captures candidate product features that are intentionally beyond the
current implementation slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `system-assessment`, `migration`, and
`supply-chain-analysis`. The next roadmap should prioritize output quality,
packaging, distribution, and authoring improvements instead
of reopening already delivered mode surfaces.

## Current Delivered Reference

The roadmap below is intentionally forward-looking. Delivered work is kept here
only as a compact baseline so future candidates have the right constraints and
continuity.

- `requirements`, `architecture`, and `change` ship the first
  industry-standard artifact-shapes slice with PRD, C4 plus ADR, and ADR-style
  bounded change packet guidance.
- `discovery`, `system-shaping`, and `review` now ship the 030 follow-on slice
  with Opportunity Solution Tree plus Jobs-To-Be-Done exploratory framing,
  domain map plus structural-options shaping, and findings-first review packet
  guidance. Keep personas guidance-only.
- `review`, `verification`, `incident`, and `migration` now share canonical authored H2 contracts, and Mode Authoring Specialization is now complete for the currently modeled governed modes.
- `implementation`, `refactor`, and `verification` now ship the 031 closeout
  slice with a task-mapped delivery packet, a preserved-behavior plus
  structural-rationale packet, and a claims-and-evidence challenge packet.
  Persona guidance remains presentation only.
- `security-assessment` remains delivered and publishes readable packets under
  `docs/security-assessments/<YYYY-MM-DD>-<descriptor>/`.
- `system-assessment` remains delivered as the as-is architecture packet,
  publishes readable packets under
  `docs/architecture/assessments/<YYYY-MM-DD>-<descriptor>/`,
  and uses ISO 42010-style coverage with explicit observed findings, inferred
  findings, and assessment gaps.
- `system-shaping`, `change`, `implementation`, and `migration` now preserve
  explicit alternatives, decision evidence, and rejected-option rationale in
  their decision-heavy packet surfaces.
- `supply-chain-analysis` remains delivered and publishes readable packets
  under `docs/supply-chain/<YYYY-MM-DD>-<descriptor>/`.
- Published packets now use structured default leaf directories such as
  `<YYYY-MM-DD>-<descriptor>/` and ship a `packet-metadata.json` sidecar so
  canonical run identity and source artifact lineage remain recoverable
  outside `.canon/`.
- `Distribution Channels Beyond GitHub Releases` now ships Homebrew
  installation for macOS and Linux, generated distribution metadata,
  Homebrew formula artifacts with optional tap synchronization, `winget`
  publication artifacts for Windows, and a versioned Scoop manifest artifact
  derived from the same canonical release bundle while GitHub Releases remain
  the source of truth.

There are no active remaining candidate feature blocks recorded immediately
after the delivered 033 slice. Future roadmap work should continue to focus on
output quality and authoring durability without reopening the now-delivered
reasoning-evidence contract.

## Delivered Feature: 033 Cross-Mode Reasoning Evidence And Clarity Expansion

### Outcome

Canon now exposes cross-mode reasoning posture directly instead of letting
structural completeness masquerade as strong reasoning.

### Problem We Solved

- `inspect clarity` previously exposed `reasoning_signals` for only a narrow
  subset of modes, leaving many file-backed surfaces without a shared pre-run
  honesty check.
- Backlog fallback artifacts could still read like approved decomposition even
  when no authored epic tree, slices, sequencing, or anchors existed.
- Review-family result posture still risked collapsing evidence-bounded or
  no-direct-contradiction states into generic success language.

### Delivered Surface

- Extend `inspect clarity` and `reasoning_signals` across the remaining
  file-backed governed modes while keeping `pr-review` explicitly diff-backed.
- Surface materially-closed and weak-support reasoning signals so Canon can say
  directly when a packet is still shallow or when the decision is already
  bounded.
- Tighten backlog fallback artifacts so Canon preserves explicit missing-body
  language instead of synthesizing plausible epics, slices, sequencing, or
  acceptance anchors.
- Surface `evidence-bounded` review posture and explicit
  `no-direct-contradiction` verification posture in the runtime summaries.
- Synchronize version surfaces, shared skill guidance, README, mode guidance,
  roadmap continuity, and changelog references for the `0.33.0` delivery.

### Invariants Preserved

- Canon still preserves explicit `## Missing Authored Body`, blocked,
  unsupported, and unresolved-findings honesty markers instead of softening
  them into generic filler.
- `pr-review` remains diff-backed and is not forced into the file-backed
  clarity path.
- `.canon/` persistence, approval targets, publish destinations, and
  recommendation-only operational posture remain unchanged.

## Delivered Feature: 032 Scoop Distribution Follow-On

### Outcome

Canon now extends the Windows distribution surface with a repository-generated
Scoop manifest derived from the canonical release bundle so users can install
through Scoop once the main bucket picks up the generated manifest.

### Problem We Solved

- Homebrew and `winget` already reused canonical release metadata, but Scoop
  remained the next concrete Windows distribution follow-on.
- Maintainers still needed a durable Scoop artifact instead of hand-copying the
  Windows asset URL, filename, and checksum into a bucket submission.

### Delivered Surface

- Shared Windows distribution metadata now advertises both `winget` and Scoop
  for the canonical `windows-x86_64.zip` asset.
- The release workflow now renders and verifies
  `canon-<VERSION>-scoop-manifest.json` alongside the existing distribution
  metadata, Homebrew formula, and `winget` bundle.
- README and maintainer docs now cover Scoop install, upgrade, manual
  submission, and archive fallback posture.

### Invariants Preserved

- GitHub Releases remain the canonical source of binaries, filenames, and
  checksums.
- The existing Windows zip remains the single Windows installation payload.
- Direct-download fallback remains documented alongside package-manager paths.

## Delivered Feature: 031 Remaining Industry-Standard Artifact Shapes

### Outcome

Canon now extends industry-standard artifact shapes to `implementation`,
`refactor`, and `verification` so delivery, preservation, and challenge work
read like reviewer-native packets rather than generic AI summaries.

### Problem We Solved

- The earlier artifact-shapes slices proved the pattern for planning,
  architecture, exploratory, and review work, but execution, preservation, and
  challenge packets still needed clearer native framing.
- These modes already had canonical authored-body contracts, but the remaining
  rollout needed explicit packet-shape and persona coverage in skills, docs,
  release surfaces, and regressions.

### Delivered Shape Mapping

- `implementation` now reads like a task-mapped delivery packet with option
  comparison, decision evidence, and rollback posture authored by a bounded
  implementation lead.
- `refactor` now reads like a preserved-behavior matrix plus
  structural-rationale packet authored by a preservation-focused maintainer.
- `verification` now reads like a claims, evidence, and independence
  challenge packet authored by an adversarial verifier.

### Invariants Preserved

- Canon still preserves the exact authored H2 contracts for the targeted
  modes.
- Missing or weak authored sections still emit explicit `## Missing Authored
  Body` markers instead of confident filler.
- Persona guidance remains presentation only; it does not widen authority,
  weaken evidence, or normalize missing context.

## Delivered Feature: 030 Industry-Standard Artifact Shapes Follow-On

### Outcome

Canon now extends industry-standard artifact shapes to `discovery`,
`system-shaping`, and `review` so exploratory, system-design, and bounded
packet-review work read like reviewer-native artifacts rather than generic AI
summaries.

### Problem We Solved

- The earlier artifact-shapes slice proved the pattern for `requirements`,
  `architecture`, and `change`, but discovery, shaping, and review still read
  too much like Canon-internal summaries.
- Those modes already had canonical authored-body contracts, but they needed
  clearer packet framing and authored personas to match how engineers actually
  consume exploratory, domain-boundary, and review work.

### Delivered Shape Mapping

- `discovery` now reads like an Opportunity Solution Tree plus
  Jobs-To-Be-Done flavored exploratory brief authored by an exploratory
  research lead.
- `system-shaping` now reads like a domain map plus structural-options packet
  authored by a bounded system designer.
- `review` now reads like a findings-first review bundle carrying severity,
  location, rationale, and recommended change detail inside the canonical
  review sections.

### Invariants Preserved

- Canon still preserves the exact authored H2 contracts for the targeted modes.
- Missing or weak authored sections still emit explicit `## Missing Authored
  Body` markers instead of reviewer-sounding filler.
- Approval posture, evidence semantics, `.canon/` runtime storage, and publish
  behavior remain unchanged by this slice.

## Delivered Feature: 028 Decision Alternatives, Pattern Choices, And Framework Evaluations

### Outcome

Canon now acts as a real engineering companion for decision-heavy work: instead of
jumping straight to one recommendation, it presents 2 to 4 viable
alternatives, compares them explicitly, and recommends one with clear
rationale grounded in the user's constraints, the real source surface, and
observable ecosystem signals.

This delivered slice also closes the most obvious persona gap left by the first
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

- `architecture` keeps the explicit architecture-decision persona and strengthens it
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

### Delivered Slice

- `architecture` now ships the option-analysis shape: `Decision Drivers`,
  `Options Considered`, `Pros`, `Cons`, `Recommendation`, and
  `Why Not The Others`, alongside ADR-like `Consequences` in the decision
  artifact.
- `system-shaping` and `change` now preserve pattern-selection and bounded
  change alternatives directly in their authored artifacts, including explicit
  rejected-option rationale.
- `implementation` and `migration` now preserve framework or rollout
  candidate comparison plus decision evidence directly in their execution and
  migration packets.
- Final selections remain recommendation-only in v0.x.

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

## Delivered History

## Delivered Feature: 029 Structured External Publish Destinations

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
