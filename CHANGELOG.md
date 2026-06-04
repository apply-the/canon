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

## [0.65.0] - 2026-06-03

Delivered specs:

- `specs/068-ideation-mode/`
- `specs/067-systematic-debugging/`

Highlights:

- Added a new `brainstorming` (or `ideation`) mode to evaluate high-level ideas, explore lateral thinking, and generate option maps before formal shaping.
- Included structured outputs for `brainstorming`: option maps with multiple conceptual approaches, trade-off matrices, open questions, and spike proposals.
- Integrated `brainstorming` mode into the governance surface and CLI contracts as a read-only, green-zone exploration workflow.
- Added a first-class `debugging` mode for systematic defect resolution, emphasizing root-cause isolation and red-to-green test transitions.
- Enforced required artifacts for the `debugging` mode: `01-context`, `02-reproduction`, `03-root-cause`, `04-fix-decision`, and `05-verification`.
- Integrated `debugging` mode into the governance surface and CLI contracts as a recommendation-only, green-zone workflow.
- Updated docs, README, and CLI references to include `brainstorming` and `debugging`.
- Bumped workspace version to `0.65.0`.

## [0.64.0] - 2026-06-02

Delivered specs:

- `specs/065-reasoning-posture-v2/`

Highlights:

- Published `governed_reasoning_posture_v2` as the current Canon-owned
  reasoning-posture contract while freezing `governed_reasoning_posture_v1` as
  legacy-only migration context.
- Replaced the flat `v1` posture shape with typed selector,
  `minimum_independence`, `confidence_handoff`, `provenance`, and
  `compatibility_window` blocks, all backed by executable fixture validation.
- Added fail-closed fixtures for malformed selectors, weakened independence,
  incomplete confidence handoff, incompatible or stale provenance, unsupported
  vocabulary, invalid compatibility windows, and release-metadata drift.
- Published explicit dual-line coexistence and migration rejection rules so one
  active line and one legacy line remain the only supported mixed-line state.
- Aligned the workspace version, runtime compatibility metadata, assistant
  package metadata, README, roadmap, CLI reference, and stable integration
  contract to the Canon `0.64.0` release line.

## [0.63.1] - 2026-06-02

Delivered changes:

- Patch-only bugfix release; no new `specs/` directory.

Highlights:

- Hardened `canon-pr-review` intake so missing or semantically unclear base/head
  refs now start from a guided comparison choice instead of an open-ended ref
  prompt.
- Kept the repo-local and embedded `canon-pr-review` skill surfaces aligned,
  including host-rich-input guidance with plain guided-text fallback.
- Tightened the Bash and PowerShell skill validators so the guided comparison
  choice contract stays enforced alongside the existing ref-binding rules.
- Updated assistant package metadata, prompt-pack guidance, roadmap, and docs
  release-line references to align on `0.63.1`.

## [0.63.0] - 2026-06-01

Delivered specs:

- `specs/063-interactive-init-ui/`

Highlights:

- Added a guided full-screen `canon init` experience in `canon-cli` using
  `ratatui` and `crossterm`, with assistant preselection and a confirmation
  step before runtime initialization.
- Preserved script-safe init behavior behind `--non-interactive`, including
  assistant flags, machine-readable output, and default fallback when an
  interactive terminal is unavailable.
- Expanded assistant coverage so guided and non-interactive init now support
  Cursor and Antigravity alongside the existing assistant targets.
- Added explicit rejection for structured output without
  `--non-interactive`, plus preflight layout-fit checks that fail before the
  TUI opens when the current terminal is too small.
- Hardened terminal cleanup so success, `Ctrl+C`, and guided-path failures all
  restore the terminal before control returns to the shell.
- Aligned the published docs, roadmap, and source-guide links with the current
  `0.63.0` release line.

## [0.61.0] - 2026-05-28

Delivered specs:

- `specs/061-skill-runtime-contracts/`

Highlights:

- Added structured preflight JSON output via `canon-preflight.sh` and
  `canon-preflight.ps1`, replacing key=value `check-runtime.sh` output for
  migrated skills.
- Introduced declarative `preflight:` YAML frontmatter in SKILL.md for
  `canon-implementation`, `canon-change`, and `canon-publish` skills.
- Added `.canon/hooks.toml` lifecycle hooks with detect/propose semantics and
  trace recording in `ai-provenance.md`.
- Existing skills with prose-only preflight continue working unchanged.

## [0.60.0] - 2026-05-19

Delivered specs:

- `specs/060-pr-review-anchors/`

Highlights:

- Added typed optional `ReviewAnchor` coordinates to `pr-review` findings when
  persisted diff evidence resolves to one changed surface and one contiguous
  changed interval.
- Updated `conventional-comments.md` to keep explicit scope mandatory while
  rendering host-agnostic `surface:start` and `surface:start-end` anchor text
  whenever durable precision exists.
- Added focused unit, contract, and integration coverage plus aligned
  reviewer guidance and wiki/docs examples for anchored versus scope-only
  review output.
- Bumped the Canon workspace, assistant package manifests, and shared runtime
  compatibility metadata to `0.60.0`.

## [0.59.0] - 2026-05-19

Delivered specs:

- `specs/059-reasoning-profile-closure-alignment/`

Highlights:

- Bumped the Canon workspace and published assistant package manifests to
  `0.59.0`.
- Advanced the governed reasoning posture compatibility window to Canon
  `0.59.x` while keeping the Boundline companion line at `0.62.x`.
- Kept the embedded and repo-local skill runtime compatibility metadata aligned
  so installed skills validate against the current workspace release.

## [0.58.0] - 2026-05-18

Delivered specs:

- `specs/059-reasoning-profile-closure-alignment/`

Highlights:

- Updated the stable `tech-docs/integration/governed-reasoning-posture-contract.md`
  publication to keep Canon as the owner of `governed_reasoning_posture_v1`
  while advertising the supported Boundline `0.62.x` / Canon `0.58.x`
  compatibility window.
- Kept the provider contract line and required posture vocabulary unchanged;
  this release is publication-only alignment for the paired Boundline 062
  reasoning-profile closure.
- Refreshed the Canon-side contract tests, feature-local contract brief, and
  release-facing assistant metadata so the published consumer pairing stays
  internally consistent.
- Bumped workspace version to `0.58.0` across the Canon crates and active
  assistant package manifests.

## [0.57.0] - 2026-05-17

Delivered specs:

- `specs/057-s7-delight-provider/`

Highlights:

- Added the stable `tech-docs/integration/delight-provider-contract.md` contract so
  Boundline S7 can consume Canon-owned packets, approval states, readiness
  signals, security findings, audit findings, and promotion references without
  depending on ambient Canon semantics.
- Defined the five-state `degradation_state` contract (`available`, `stale`,
  `incompatible`, `absent`, `contradicted`) and the contract-line versioning
  rules that downstream S7 consumers must respect.
- Added executable contract coverage in `tests/delight_provider_contract.rs`
  and `tests/contract/delight_provider_contract.rs` to lock the stable doc and
  the feature-local brief together and fail closed on drift.
- Aligned the documented downstream consumer pairing to Boundline `0.61.0`
  while keeping Canon the semantic owner of the delight-provider contract.
- Bumped workspace version to `0.57.0` across the Canon crates, assistant
  package manifests, and runtime compatibility metadata.

## [0.55.0] - 2026-05-16

Delivered specs:

- `specs/051-artifact-indexing-contract/`

Highlights:

- Extended Canon publish metadata and runtime packet sidecars with typed
  `artifact_indexing` projection data so downstream consumers can recover the
  published artifact class, metadata carrier, and discovery rule without
  reconstructing them from prose.
- Added publish-path guards that reject unsupported target-class and update-
  strategy mappings instead of silently inventing artifact-indexing semantics.
- Validated managed-surface, proposal, and evidence publish paths so the
  emitted sidecars carry the expected artifact indexing contract fields.
- Bumped workspace version to `0.55.0` across the Canon crates and active
  compatibility metadata.

## [0.54.0] - 2026-05-15

Delivered specs:

- `specs/054-authority-zone-contract/`

Highlights:

- Added the typed `authority-governance-v1` contract vocabulary for authority
  zones, change classes, intended personas, advisory role hints, and stable
  required-versus-optional metadata rules.
- Extended runtime packet metadata sidecars so governed Canon packets can carry
  the `authority_governance` envelope without introducing a second publication
  channel.
- Kept Canon on the semantic side of the boundary: stage-role hints remain
  advisory, while downstream runtimes keep council choice and operational stop
  behavior.
- Bumped workspace version to `0.54.0` across all crates for the
  authority-zone-contract feature branch.

## [0.52.0] - 2026-05-14

Delivered specs:

- `specs/052-governed-expertise-inputs/`

Highlights:

- Added the governed expertise-input contract that classifies Canon
  `domain-language` and `domain-model` output through a stable
  `expertise_input` metadata carrier.
- Kept Canon artifact-first by reusing existing publication semantics while
  leaving runtime-role selection to Boundline.
- Bumped workspace version to `0.52.0` across all crates and assistant plugin
  manifests.

## [0.51.0] - 2026-05-14

Delivered specs:

- `specs/051-artifact-indexing-contract/`

Highlights:

- Extended the stable project-memory promotion contract with an artifact-indexing
  clarification layer instead of introducing a second normative contract surface.
- Defined explicit metadata-carrier and discovery rules for supported artifact
  classes, keeping managed-block lineage and packet sidecars as the canonical
  V1 carriers.
- Resolved ambiguous artifact vocabulary for downstream consumers and required
  explicit documentation of non-indexable artifact classes.
- Added independent comparison review and modified-file coverage closeout to the
  feature delivery plan.
- Bumped workspace version to `0.51.0` across all crates and assistant plugin
  manifests.

## [0.50.0] - 2026-05-13

Delivered specs:

- `specs/050-project-memory-control/`

Highlights:

- Published the stable owner-side project-memory and delivery-control contract
  under `tech-docs/integration/project-memory-promotion-contract.md`, with aligned
  feature-local supporting shapes for governed stage refs, promotion events,
  and evidence refs.
- Clarified Canon-owned target mapping for `tech-docs/project/` and `tech-docs/evidence/`,
  preserved producer-neutral managed blocks, and kept Canon as the producer of
  promotion policy while Boundline remains the consumer and orchestrator.
- Froze the lean V1 lineage set, documented additive-versus-breaking
  compatibility rules, and added self-describing `kind` fields to shared JSON
  contract examples without making them a required V1 validation field yet.
- Bumped workspace version to `0.50.0` across all crates, assistant plugin
  manifests, and runtime-compatibility references.

## [0.49.0] - 2026-05-13

Delivered specs:

- `specs/049-logical-packet-ordering/`

Highlights:

- Extended Canon packet ordering from filename prefixes alone into a full
  packet-order contract with required `primary_artifact` and `artifact_order`
  metadata for new packets.
- Added runtime `packet-metadata.json` emission across packet-emitting modes,
  preserved reader-facing contiguous numbering, and kept runtime sidecars such
  as `packet-metadata.json` and `view-manifest.json` outside the numbered
  packet body.
- Updated publish behavior to merge runtime ordering metadata into the
  published `packet-metadata.json`, with optional `publish_order` and
  `legacy_aliases` fields when compatibility or public ordering needs to stay
  explicit.
- Aligned status and summary surfaces so the primary artifact now resolves to
  the packet's real `01-*` entry point instead of older late-packet slug
  heuristics.
- Updated mode documentation and domain packet templates so `domain-language`
  and `domain-model` clearly distinguish vocabulary stabilization from concept
  modeling and document their ordered packet sequences.
- Bumped workspace version to `0.49.0` across all crates and assistant plugin
  manifests.

## [0.48.0] - 2026-05-13

Delivered specs:

- `specs/048-project-memory-promotion-policy/`

Highlights:

- Added `project-memory` publish profile for promoting governed output into
  project-visible knowledge surfaces with policy-aware routing onto named
  Canon-owned surfaces such as `tech-docs/project/product-context.md`,
  `tech-docs/project/architecture-map.md`, `tech-docs/project/delivery-map.md`,
  `tech-docs/project/audit-log.md`, and `tech-docs/project/open-risks.md`.
- Introduced six Canon-owned promotion states (`auto`, `auto-if-approved`,
  `pending-index`, `index-only`, `evidence-only`, `manual`) that determine
  whether a packet updates stable memory, pending surfaces, evidence-only
  surfaces, or requires manual handling.
- Added file-adjacent metadata sidecars (`<surface>.packet-metadata.json`) with
  full producer-owned traceability fields (contract_version, source_run, mode,
  profile, promotion_state, approval_state, readiness, published_at,
  update_strategy, source_artifacts).
- Implemented non-destructive update strategies: managed-block updates that
  preserve human-authored content, proposal-file emission for unsafe targets,
  and append-only index updates.
- Published versioned shared contract brief for cross-repo consumer consumption
  at `tech-docs/integration/project-memory-promotion-contract.md`.
- Added `--profile project-memory` argument to `canon publish` CLI command.
- Bumped workspace version to `0.48.0` across all crates and assistant plugin
  manifests.

## [0.47.0] - 2026-05-12

Delivered specs:

- `specs/047-domain-language-model-modes/`

Highlights:

- Added `domain-language` as a first-class governed Canon mode producing 10
  artifacts for ubiquitous-language stabilization: language-overview, domain-glossary,
  preferred-language, language-conflicts, contextual-meanings, business-language-rules,
  code-and-api-vocabulary, downstream-language-guidance, language-decision-record,
  and ai-provenance.
- Added `domain-model` as a first-class governed Canon mode producing 13
  artifacts for lightweight ontology concept modeling: model-overview, concept-catalog,
  relationship-map, bounded-context-map, lifecycle-and-state-model, domain-invariants,
  policy-and-constraint-rules, feature-impact-rules, code-data-alignment,
  model-gaps-and-risks, downstream-model-guidance, domain-model.json, and ai-provenance.
- Both modes support `canon inspect clarity`, `canon governance capabilities --json`,
  canonical input binding (`canon-input/domain-language.md`, `canon-input/domain-model.md`),
  and publish to `tech-docs/domain/language/` and `tech-docs/domain/model/` respectively.
- `domain-model.json` provides a machine-readable concept model with schema_version,
  concepts, relationships, invariants, and feature_impact_rules.
- Both modes default to `recommendation-only` execution posture.
- Added method TOML definitions, input templates, worked examples, and mode guide
  documentation for both modes.
- Bumped workspace version to `0.47.0` across all crates and assistant plugin manifests.

## [0.46.0] - 2026-05-12

Delivered specs:

- `specs/046-ordered-artifact-filenames/`

Highlights:

- Artifact filenames emitted by `contract_for_mode()` now carry ordinal prefixes
  (e.g., `01-problem-statement.md`, `02-constraints.md`) so that published
  packets display in a deterministic, Confluence-tree-style reading order.
- Added `artifact_slug()` utility and `slug()` methods on `ArtifactRequirement`
  and `ArtifactRecord` for prefix-agnostic matching in gates, renderers, and
  summarizers.
- Bumped workspace version to `0.46.0` across all crates and assistant plugin
  manifests.

## [0.45.0] - 2026-05-11

Delivered specs:

- `045` - Mode Publish Alignment

Highlights:

- Align `security-assessment` publish behavior with the documented operational packet posture for readable blocked or approval-gated runs.
- Correct assistant package publish examples so they use the real positional `canon publish <RUN_ID>` command shape.
- Advance the repository release line and close the slice with versioned validation evidence.

## [0.44.0] - 2026-05-10

Delivered specs:

- `044` - Assistant Plugin Packages

Highlights:

- Add Canon-owned assistant package folders, shared metadata, prompts, commands, and validation for Claude Code, Codex, Cursor, and Copilot command-pack support.
- Keep Canon CLI and governance adapter behavior authoritative while packaging governed methods for host discovery.
- Add repeatable validation for package metadata drift, required capability coverage, and prohibited positioning language.

## [0.43.0] - 2026-05-10

Delivered specs:

- `043` - Standard ADR Publish Artifacts

Highlights:

- Publish a standard ADR entry from `architecture` packets by default while keeping existing packet outputs authoritative.
- Allow `change` and `migration` packets to opt into the same ADR registry during publish.
- Keep ADR registry numbering fixed under `tech-docs/adr/` and leave unsupported modes outside ADR publication.

## [0.42.0] - 2026-05-08

Delivered specs:

- `042` - Pragmatic C4 Architecture Packets And Visual Artifacts

Highlights:

- Reframe the `architecture` packet around one primary handoff document so System Context, Container, and Deployment coverage are reviewable without opening every supporting file independently.
- Add machine-readable Mermaid view artifacts and a packet manifest so architecture documentation stays both human-readable and tool-friendly.
- Keep deeper C4 views and rendered SVG or PNG assets optional and evidence-driven instead of mandatory boilerplate.

## [0.41.0] - 2026-05-07

Delivered specs:

- `041` - Requirements PRD Publishing And Chat Publish Skill

Highlights:

- Add an additive `prd.md` to the `requirements` packet so published requirements runs now expose one consolidated product-facing document alongside the sectional artifact set.
- Add the `canon-publish` repo-local skill and embedded mirror so chat-first Copilot or Codex workflows can drive the real `canon publish` command explicitly.
- Clarify the publish UX across README, mode guidance, runtime compatibility surfaces, roadmap, and release metadata so users know artifacts land under `.canon/artifacts/` before publish materializes visible docs.

## [0.40.0] - 2026-05-03

Delivered specs:

- `040` - Governance Runtime Framing

Highlights:

- Reframe Canon's public identity around one explicit promise: it is the
  governed packet runtime for AI-assisted engineering, not a generic agent
  framework or opaque orchestration loop.
- Add a dedicated governance adapter integration guide that explains the
  machine-facing boundary, stable `v1` response fields, and external-orchestrator
  usage rules without splitting Canon into a separate runtime.
- Align the `0.40.0` release surface across workspace manifests, lockfile,
  runtime compatibility references, README, integration docs, roadmap,
  changelog, and focused validation evidence.

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
  date-prefixed descriptor folders under the existing `specs/` and `tech-docs/`
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
  `tech-docs/architecture/assessments/<RUN_ID>/`, and distinguish direct evidence
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
  `--system-context existing`, publish to `tech-docs/security-assessments/<RUN_ID>/`,
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