# Changelog

All notable changes to Canon are documented in this file.

This changelog is reconstructed from feature-spec delivery under `specs/` and
the workspace version bumps recorded in `Cargo.toml`. The
repository does not currently use release tags, so each release below maps a
published version to the spec directories first introduced between that version
bump and the previous one.

Canon follows Semantic Versioning. Before `1.0.0`, breaking changes may occur in
minor releases.

The repository history contains no release bumps for `0.10.0`, `0.13.0`,
`0.16.0`, or `0.17.0`, so adjacent feature slices are rolled into the next
recorded workspace version.

## [Unreleased]

Delivered specs:

- None recorded after `0.39.0`

Highlights:

- No additional released feature-spec delivery is recorded after `0.39.0`.

## [0.39.0] - 2026-05-02

Delivered specs:

- `039` - Authoring Experience And Packet Readiness

Highlights:

- Extend `inspect clarity` with an additive authoring-lifecycle summary that
  explains packet shape, authoritative brief inputs, supporting context, and
  remaining readiness delta for file-backed packets.
- Align the shared file-backed lifecycle across the mode guide,
  template-facing docs, carry-forward example, and `canon-inspect-clarity`
  skill so the path from authored brief to publishable packet stays explicit.
- Align the `0.39.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, README, roadmap, changelog, and focused
  validation evidence.

## [0.38.0] - 2026-05-02

Delivered specs:

- `038` - Guided Run Operations And Review Experience

Highlights:

- Align `run`, `status`, and shared next-step guidance around one result-first
  operator surface with ordered possible actions, explicit blocker handling,
  and review-before-approval ergonomics for gated and resumed runs.
- Align the `0.38.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, operator guidance, roadmap, changelog,
  and focused validation evidence.

## [0.37.0] - 2026-05-02

Delivered specs:

- `037` - Architecture Clarification Readiness And Mode Reroute

Highlights:

- Extend `inspect clarity --mode architecture` with structured clarification
  question metadata, explicit default-if-skipped behavior, and honest reroute
  guidance when a brief is not architecture-ready.
- Update `readiness-assessment.md` so architecture packets preserve working
  assumptions, unresolved questions, blockers, accepted risks, and the next
  recommended mode instead of generic readiness prose.
- Align the `0.37.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, architecture guidance, operator guidance,
  roadmap, changelog, and focused validation evidence.

## [0.36.0] - 2026-05-02

Delivered specs:

- `036` - Release Provenance And Channel Integrity

Highlights:

- Extend `canon-<VERSION>-distribution-metadata.json` with explicit
  `source_of_truth` and per-channel `channels` contracts so GitHub Releases
  stays the single source of truth for binaries, checksums, and release notes.
- Make the Homebrew, `winget`, and Scoop renderers plus the release verifier
  fail closed when provenance or channel-contract expectations drift from the
  canonical release bundle.
- Align the `0.36.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, install or maintainer docs, roadmap
  cleanup, changelog, and focused release-validation evidence.

## [0.35.0] - 2026-05-02

Delivered specs:

- `035` - Governance Adapter Surface

Highlights:

- Add the first-class machine-facing `canon governance start|refresh|capabilities --json`
  surface so external orchestrators can start or refresh governed work without
  scraping human CLI prose.
- Publish a flat `v1` adapter contract with strict `governed_ready` semantics,
  explicit approval posture, machine-readable reason codes, and canonical
  workspace-relative packet or document refs.
- Align the `0.35.0` release surface across workspace manifests, lockfile,
  shared runtime compatibility references, README, mode guidance, roadmap
  continuity, publication guides, changelog, and feature validation closeout.

## [0.34.0] - 2026-05-01

Delivered specs:

- `034` - Output Quality Gates

Highlights:

- Add an explicit output-quality posture to `inspect clarity` so Canon can say
  directly whether a packet is only `structurally-complete`, already
  `materially-useful`, fully `publishable`, or materially closed.
- Replace generic `ready` summary language across governed packet families with
  quality-aware posture wording and tighten the remaining backlog
  `planning-risks.md` fallback so it no longer invents risk bullets from weak
  evidence.
- Align the `0.34.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, shared skill guidance, README, mode
  guidance, publication guides, roadmap continuity, and final validation
  closeout.

## [0.33.0] - 2026-05-01

Delivered specs:

- `033` - Cross-Mode Reasoning Evidence And Clarity Expansion

Highlights:

- Extend `inspect clarity` and `reasoning_signals` across the remaining
  file-backed governed modes so maintainers can see missing context, weak
  evidence, or materially-closed decisions before a run starts.
- Tighten backlog fallback artifacts and review or verification result posture
  so Canon preserves explicit missing-body, evidence-bounded,
  unresolved-findings, and no-direct-contradiction honesty instead of generic
  reasoning theater.
- Align the `0.33.0` release surface across workspace manifests, runtime
  compatibility references, shared skill guidance, README, mode guidance,
  roadmap continuity, and final validation closeout.

## [0.32.0] - 2026-05-01

Delivered specs:

- `032` - Scoop Distribution Follow-On

Highlights:

- Add repository-owned Scoop manifest generation for the canonical Windows
  release archive while keeping GitHub Releases as the source of binaries,
  filenames, and checksums.
- Extend shared distribution metadata and release-surface verification so the
  canonical Windows asset advertises both `winget` and Scoop and mismatched
  URLs or hashes fail validation.
- Update install docs, maintainer publication guides, roadmap, runtime
  compatibility references, and release surfaces for the `0.32.0` delivery.

## [0.31.0] - 2026-05-01

Delivered specs:

- `031` - Remaining Industry-Standard Artifact Shapes

Highlights:

- Close the industry-standard artifact-shapes rollout for the remaining
  modeled modes by shaping `implementation`, `refactor`, and `verification`
  into task-mapped delivery, preserved-behavior, and claims-and-evidence
  packets.
- Keep canonical authored H2 contracts, explicit `## Missing Authored Body`
  honesty, and guidance-only persona boundaries intact while extending the
  reviewer-native packet framing across the final modeled-mode slice.
- Align the `0.31.0` release surface across workspace manifests, lockfile,
  shared runtime compatibility references, release-facing docs, focused docs
  regressions, and final validation evidence.

## [0.30.0] - 2026-05-01

Delivered specs:

- `030` - Artifact Shapes Follow-On

Highlights:

- Extend the industry-standard artifact-shapes rollout to `discovery`,
  `system-shaping`, and `review` with explicit exploratory, domain-map plus
  structural-options, and findings-first reviewer-native packet framing.
- Keep canonical authored H2 contracts, explicit `## Missing Authored Body`
  honesty, and bounded persona guidance intact while broadening the packet
  shape contract across the three follow-on modes.
- Align the `0.30.0` release surface across workspace manifests, lockfile,
  shared runtime compatibility references, release-facing docs, focused
  release regression tests, and final validation evidence.

## [0.29.0] - 2026-05-01

Delivered specs:

- `029` - Publish Destinations

Highlights:

- Replace run-id-only default publish destinations with readable
  date-prefixed descriptor folders under the existing `specs/` and `docs/`
  family roots.
- Emit `packet-metadata.json` with each published packet so run id, mode,
  risk, zone, publish timestamp, destination, and source artifact lineage stay
  recoverable outside `.canon/`.
- Align the `0.29.0` release surface across workspace manifests, lockfile,
  shared runtime compatibility references, release-facing docs, focused
  release regression tests, and final validation evidence.

## [0.28.0] - 2026-05-01

Delivered specs:

- `028` - Decision Alternatives

Highlights:

- Extend `system-shaping` and `change` with explicit rejected-alternative
  reasoning so structural and bounded-change packets preserve why competing
  options lost instead of only recording the winning path.
- Extend `implementation` and `migration` with candidate-framework,
  decision-evidence, and rejection-rationale sections so bounded stack and
  rollout choices remain inspectable and recommendation-only.
- Align the `0.28.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, templates, examples, guidance, and final
  validation for the shipped slice.

## [0.27.0] - 2026-04-30

Delivered specs:

- `027` - System Assessment Mode

Highlights:

- Add a first-class `system-assessment` mode for governed as-is architecture
  assessment with explicit coverage, dependencies, risks, and evidence gaps.
- Keep `system-assessment` recommendation-only, require
  `--system-context existing`, publish to
  `docs/architecture/assessments/<RUN_ID>/`, and distinguish direct evidence
  from inferred coverage and assessment gaps with explicit observed and
  inferred findings.
- Add the new method, skill, templates, examples, compatibility references,
  and focused contract, renderer, run, bootstrap, and release-surface
  validation for the `0.27.0` delivery.

## [0.26.0] - 2026-04-30

Delivered specs:

- `026` - Winget Distribution And Roadmap Refocus
- `025` - Distribution Channels Beyond GitHub Releases

Highlights:

- Add the first package-manager distribution slice for Canon through Homebrew
  while keeping GitHub Releases as the canonical source of binaries,
  checksums, and release metadata.
- Emit `canon-<version>-distribution-metadata.json` and
  `canon-<version>-homebrew-formula.rb` from the verified release bundle and
  validate their URL and checksum alignment before publication.
- Add repository-owned `winget` manifest generation for the Windows release
  archive while keeping GitHub Releases as the canonical binary and checksum
  surface.
- Add optional dedicated Homebrew tap synchronization with artifact-only
  fallback, plus install, roadmap, and release-surface updates for the `0.26.0`
  delivery.
- Extend the release workflow, verification helpers, and install docs so
  macOS and Linux users can prefer Homebrew, Windows users can prefer
  `winget`, and maintainers still retain the archive fallback plus optional
  tap synchronization.
- Remove speculative Protocol Interoperability / MCP roadmap work from the
  active next-feature list in favor of concrete packaging and authoring focus.

## [0.24.0] - 2026-04-29

Delivered specs:

- `024` - Supply Chain And Legacy Analysis Mode

Highlights:

- Add a first-class `supply-chain-analysis` mode for governed SBOM,
  vulnerability, license-compliance, and legacy-posture analysis.
- Keep `supply-chain-analysis` recommendation-only, bounded to existing-system
  repository surfaces, and explicit about missing scanner coverage or missing
  authored decisions.
- Add the new method, skill, templates, examples, compatibility references,
  and focused runtime plus release-surface validation for the `0.24.0`
  delivery.

## [0.23.0] - 2026-04-28

Delivered specs:

- `023` - Cybersecurity Risk Assessment Mode

Highlights:

- Add a first-class `security-assessment` mode with a seven-artifact security
  packet covering scope, threats, risks, mitigations, assumptions, compliance
  anchors, and evidence.
- Keep `security-assessment` recommendation-only, require
  `--system-context existing`, publish to `docs/security-assessments/<RUN_ID>/`,
  and gate systemic or red-zone packets through explicit `gate:risk` approval.
- Add the new skill, method, templates, examples, runtime compatibility
  references, and focused contract, renderer, run, bootstrap, and
  discoverability validation for the 0.22.0 release surface.

## [0.21.0] - 2026-04-27

Delivered specs:

- `021` - Industry-Standard Artifact Shapes With Personas

Highlights:

- Add the first slice of persona-aware industry-standard packet shaping for
  `requirements`, `architecture`, and `change`.
- Keep persona guidance bounded to presentation and audience fit while
  preserving canonical authored H2 contracts, explicit missing-body markers,
  and the existing approval plus evidence posture across the product lead,
  architect, and change owner authoring personas.
- Update roadmap, README, getting-started guidance, repo-local skills,
  templates, worked examples, and focused documentation validation so the
  first-slice shape-plus-persona mapping is visible outside chat history.

## [0.20.0] - 2026-04-26

Delivered specs:

- `020` - Authoring Specialization Completion

Highlights:

- Extend the canonical authored H2 contract to `review`, `verification`,
  `incident`, and `migration` across skills, templates, worked examples,
  renderers, contract tests, run fixtures, and release guidance.
- Preserve authored sections verbatim across the remaining targeted packet
  artifacts and emit explicit `## Missing Authored Body` markers when a
  required canonical heading is absent or near-missed.
- Keep `review` disposition gating explicit, keep `verification` blocked on
  unsupported or unresolved findings, and keep `incident` plus `migration`
  recommendation-only while still publishing readable blocked or approval-gated
  packets.

## [0.19.0] - 2026-04-26

Delivered specs:

- `019` - Mode Authoring Specialization Follow-On

Highlights:

- Extend the canonical authored H2 contract to `system-shaping`,
  `implementation`, and `refactor` across skills, templates, worked examples,
  runtime fixtures, and release guidance.
- Preserve authored sections verbatim across the remaining targeted packet
  artifacts and emit explicit `## Missing Authored Body` markers when a
  required canonical heading is absent or near-missed.
- Keep incomplete `system-shaping`, `implementation`, and `refactor` packets
  honestly gate-blocked while recommendation-only posture, direct-run fixtures,
  and focused contract/renderer/run/docs validation stay synchronized.

## [0.18.0] - 2026-04-26

Delivered specs:

- `016` - Mode Authoring Specialization
- `017` - Domain Modeling And Boundary Design
- `018` - Architecture ADR And Options

Highlights:

- Extend `requirements`, `discovery`, `change`, and `architecture` with
  explicit authored H2 section contracts, verbatim renderer preservation, and
  honest `## Missing Authored Body` markers when required content is absent.
- Deepen `system-shaping`, `architecture`, and `change` with bounded contexts,
  ubiquitous language, domain invariants, ownership boundaries, and explicit
  integration seams while preserving Canon's critique-first posture.
- Upgrade `architecture` with an ADR-like decision packet and explicit
  option-analysis artifacts that preserve `Decision Drivers`,
  `Options Considered`, `Pros`, `Cons`, `Recommendation`, and
  `Why Not The Others`.
- Keep the existing C4 architecture outputs intact while adding the
  `Risks`-to-`Consequences` compatibility path, synchronized docs, and focused
  contract, renderer, run, and docs validation for the strengthened packet.

## [0.15.0] - 2026-04-25

Delivered specs:

- `015` - Stronger Architecture Outputs (C4 Model)

Highlights:

- Extend `architecture` with C4-shaped textual artifacts for system context,
  container view, and component view.
- Preserve authored C4 sections verbatim and emit explicit
  `## Missing Authored Body` markers when the brief omits a required section.
- Add templates, examples, and skill guidance for the C4-authored architecture
  packet shape without removing the existing decision, invariant, boundary, or
  tradeoff artifacts.

## [0.14.0] - 2026-04-25

Delivered specs:

- `013` - PR Review Conventional Comments
- `014` - High-Risk Operational Programs

Highlights:

- Extend `pr-review` with Conventional Comments-shaped review artifacts while
  preserving the existing review summary and approval-aware disposition flow.
- Promote `incident` and `migration` from skeletons to first-class governed
  operational modes.
- Add durable operational packets for blast radius, containment,
  compatibility, sequencing, fallback planning, and publishable review outside
  the runtime.

## [0.12.0] - 2026-04-24

Delivered specs:

- `012` - Backlog Mode (Delivery Decomposition)

Highlights:

- Add `backlog` as a first-class Canon mode for governed delivery
  decomposition.
- Emit durable planning artifacts for epics, capability mapping,
  dependencies, delivery slices, sequencing, acceptance anchors, and planning
  risks.
- Block or downgrade decomposition when upstream architecture or shaping inputs
  are too weak for credible planning.

## [0.11.0] - 2026-04-23

Delivered specs:

- `010` - Controlled Execution Modes (`implementation` and `refactor`)
- `011` - Execution-Mode Approval and Action Chips

Highlights:

- Promote `implementation` and `refactor` to governed execution modes with task
  mapping, mutation bounds, validation hooks, rollback notes, and explicit
  preservation/safety-net expectations.
- Add an unconditional `gate:execution` approval step plus the
  approve-and-resume lifecycle for execution modes.
- Add host-renderable action chips and align execution summaries with the real
  run owner instead of placeholder template metadata.

## [0.9.0] - 2026-04-22

Delivered specs:

- `009` - Run Identity, Display Id, and Authored-Input Refactor

Highlights:

- Introduce human-friendly run ids, short ids, and slug-aware run directories
  while preserving UUID as Canon's canonical machine identity.
- Separate editable `canon-input/` authoring surfaces from immutable per-run
  snapshots under `.canon/runs/<...>/inputs/`.
- Improve CLI lookup, status, inspect, approve, and resume flows around the new
  run identity model.

## [0.8.0] - 2026-04-22

Delivered specs:

- `008` - Mode Context Split

Highlights:

- Replace `brownfield-change` with `change` as Canon's bounded
  existing-system modification mode.
- Introduce explicit `system_context = new | existing` as a first-class runtime
  and CLI concept.
- Rename canonical input and artifact namespaces around the new two-axis mode
  model and require explicit context wherever it is semantically necessary.

## [0.7.0] - 2026-04-19

Delivered specs:

- `007` - Review Mode Completion

Highlights:

- Promote `review` and `verification` to truthful end-to-end governed
  workflows with durable packets, real run state, and inspection compatibility.
- Add approval-aware behavior, missing-evidence handling, and adversarial
  verification artifacts for non-PR review surfaces.
- Align documentation and release guidance with the now-runnable review-heavy
  modes.

## [0.6.0] - 2026-04-18

Delivered specs:

- `006` - Analysis Mode Expansion

Highlights:

- Add full governed depth for `discovery`, `system-shaping`, and
  `architecture`.
- Reuse Canon's existing evidence, gating, persistence, and inspection model
  across the analysis-heavy front end of the engineering workflow.
- Close the early-lifecycle product gap between requirements framing and later
  execution-heavy modes.

## [0.5.0] - 2026-04-12

Delivered specs:

- `005` - Installable CLI Distribution and Release UX

Highlights:

- Shift Canon to an install-first, binary-first CLI posture.
- Define release artifact, install guidance, and cross-platform distribution
  expectations for the daily `canon` entrypoint.
- Keep build-from-source as a contributor workflow while moving product
  messaging and docs toward installed binary usage.

## [0.4.0] - 2026-03-30

Delivered specs:

- `004` - Runnable Skill Interaction and Ref-Safe Input Binding

Highlights:

- Make runnable skills collect typed inputs incrementally instead of forcing
  brittle full-command retries.
- Tighten command binding and ref-safe preflight behavior, especially for
  `canon-pr-review`.
- Keep repo-local skills as thin workflow frontends over Canon CLI rather than
  a second execution runtime.

## [0.3.0] - 2026-03-29

Delivered specs:

- `003` - Codex Skills Frontend for Canon

Highlights:

- Add a Codex-native skills frontend for Canon under `.agents/skills/`.
- Surface supported versus modeled workflows honestly through repo-local skill
  guidance.
- Add operational skills that keep users inside Canon's governed CLI and
  runtime state model.

## [0.2.0] - 2026-03-28

Delivered specs:

- `002` - Governed Execution Adapters

Highlights:

- Add governed invocation over external execution adapters.
- Persist policy decisions, invocation traces, and linked evidence for allowed
  and denied tool usage.
- Keep generation and validation independently challengeable instead of letting
  one execution path validate its own consequential output.

## [0.1.0] - 2026-03-28

Delivered specs:

- `001` - Canon v0.1 Product Specification

Highlights:

- Establish Canon's core governance contract for modes, risk, zones, artifact
  rules, gates, evidence, and decision persistence.
- Deliver the initial product boundary for requirements framing and bounded
  change planning.
- Define Canon as a governance layer over external tools rather than a
  replacement for them.