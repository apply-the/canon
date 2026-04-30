# Feature Specification: System Assessment Mode

**Feature Branch**: `027-system-assessment-mode`  
**Created**: 2026-04-30  
**Status**: Draft  
**Input**: User description: "Add a read-only system-assessment mode for existing-system architecture analysis using ISO 42010 coverage, evidence, confidence, and gap reporting before downstream architecture/change work."

## Governance Context *(mandatory)*

**Mode**: change

**Risk Classification**: systemic-impact. This slice adds a new first-class
governed mode to Canon's runtime model, CLI mode surface, artifact contracts,
gate evaluation, publishing, authoring guidance, and regression matrix. The
change touches shared mode parsing, system-context validation, contract
materialization, summaries, docs, and release-facing version surfaces for
`0.26.0`.

**Scope In**:

- Add a new governed mode named `system-assessment` to the Canon runtime and
  CLI-visible mode surface.
- Require `--system-context existing` for the new mode and reject `new` or
  missing context for this slice.
- Support canonical authored inputs at `canon-input/system-assessment.md` and
  `canon-input/system-assessment/`.
- Emit a read-only assessment packet that covers overview, ISO 42010 coverage,
  asset inventory, functional view, component view, deployment view,
  technology view, integration view, risk register, and evidence ledger.
- Make explicit observed findings, inferred findings, and assessment gaps plus
  confidence by assessed surface part of the emitted packet instead of letting
  the mode imply full repo certainty.
- Publish completed packets under `docs/architecture/assessments/<RUN_ID>/`
  without changing `.canon/` runtime storage semantics.
- Add embedded and mirrored skill guidance, templates, examples, runtime
  compatibility references, and focused regression coverage for the new mode.
- Keep the `0.26.0` release boundary explicit with a first task for version
  bumping and a final compliance task for roadmap, docs, changelog, coverage,
  formatting, and lint clean-up.

**Scope Out**:

- Reworking the existing `architecture` mode into an as-is assessment mode or
  changing its decision-making posture.
- Implementing the separate roadmap item for structured external publish
  destinations.
- Adding live repository mining services, network calls, package-registry
  lookups, or scanner integrations.
- Delivering exhaustive large-repo traversal across all modes; this first slice
  records assessed scope, skipped areas, and confidence instead of promising
  complete coverage.
- Shipping generated diagrams, historical blame archaeology, or operational
  telemetry ingestion beyond authored inputs and bounded repository validation.

**Invariants**:

- `system-assessment` MUST remain read-only and MUST NOT present architectural
  decisions, migration plans, or change recommendations as if they were already
  approved conclusions.
- The existing `architecture` mode MUST remain the decision-shaped mode for
  tradeoffs, recommendations, and ADR-oriented packets.
- Every assessment claim MUST be grounded in authored input or explicit
  repository evidence, otherwise the packet MUST record an assessment gap or
  lower confidence instead of fabricating certainty.
- The new mode MUST require `system-context=existing` in this first slice.
- Existing modes, `.canon/` runtime layout, run identity, and approval posture
  MUST remain unchanged except for shared lists and documentation that now
  include `system-assessment`.
- Release closeout for `0.26.0` MUST include recorded evidence for tests,
  formatting, skill validation, and clippy warning or error resolution.

**Decision Traceability**: Decisions start in this specification and continue
in `specs/027-system-assessment-mode/decision-log.md`, with validation
evidence recorded in `specs/027-system-assessment-mode/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Assess An Existing System As-Is (Priority: P1)

As an architect or maintainer, I want a dedicated `system-assessment` mode
that explains what a codebase is today, with explicit coverage, confidence,
evidence, and gaps, before I move into `architecture`, `change`, `migration`,
or `security-assessment`.

**Why this priority**: This is the core product outcome. Without a real mode,
users still have to overload `architecture` for repo archaeology, which is the
problem the roadmap item is trying to solve.

**Independent Test**: With a bounded existing-system brief and a small seeded
repository, Canon can run `system-assessment`, persist the expected assessment
packet, and summarize the result as an as-is packet rather than a decision
packet.

**Acceptance Scenarios**:

1. **Given** a bounded system-assessment brief for an existing repository,
   **When** Canon runs in `system-assessment` mode with
   `--system-context existing`, **Then** it emits the defined assessment
   artifacts and records confidence plus coverage rather than design decisions.
2. **Given** a repository surface where some views cannot be grounded,
   **When** the assessment packet is rendered, **Then** the packet records
  explicit observed findings, inferred findings, and assessment gaps with
  surface-level confidence instead of implying exhaustive certainty.
3. **Given** a caller tries to run `system-assessment` with `--system-context new`,
   **When** Canon validates the request, **Then** the run is rejected before a
   packet is produced.

---

### User Story 2 - Author, Publish, And Reuse The Assessment Packet (Priority: P2)

As a maintainer or assistant, I want the new mode's skill, templates, example,
publish path, and runtime compatibility surfaces to agree on the assessment
contract so the packet is understandable and reusable across downstream
architecture work.

**Why this priority**: A runtime mode without aligned authoring and publish
surfaces will drift immediately and force users back into ad-hoc prompts.

**Independent Test**: A maintainer can inspect the new skill and example, run
the mode from a canonical input file, and publish the result to the expected
architecture assessment docs directory without hand-editing internal runtime
files.

**Acceptance Scenarios**:

1. **Given** the embedded and mirrored `system-assessment` skill files,
   **When** they are reviewed, **Then** they define the same canonical authored
   sections, `existing`-only posture, ISO 42010 framing, and downstream
   follow-on guidance.
2. **Given** a completed `system-assessment` run, **When** `canon publish` is
   invoked, **Then** the packet publishes under
   `docs/architecture/assessments/<RUN_ID>/`.
3. **Given** runtime compatibility and mode inspection surfaces, **When** they
   enumerate supported governed modes, **Then** `system-assessment` appears
   consistently in those shared lists.

---

### User Story 3 - Ship 0.26.0 With Coverage And Quality Gates Closed (Priority: P3)

As a maintainer shipping `0.26.0`, I want the version bump, targeted coverage,
roadmap/docs/changelog updates, formatting, and lint clean-up to be part of
the same feature package so release closeout happens with the mode delivery.

**Why this priority**: The user explicitly asked for the version bump as the
first task and the compliance plus clean-up work as the last task, so those
requirements have to be first-class rather than implied follow-up.

**Independent Test**: A maintainer can inspect versioned repository surfaces,
confirm the task plan ordering, and reproduce the targeted `system-assessment`
tests plus workspace formatting and lint validation recorded in the feature's
validation report.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** release-facing repository
   files are compared, **Then** they consistently report Canon version
   `0.26.0`.
2. **Given** the finalized implementation, **When** the maintainer runs the
   targeted `system-assessment` tests and the declared workspace quality gates,
   **Then** they pass and the results are recorded in the validation report.
3. **Given** the generated tasks list, **When** it is reviewed, **Then** the
   first task is the version bump and the last task closes roadmap, docs,
   changelog, coverage, formatting, and clippy warnings or errors.

### Edge Cases

- A repository is too large to assess completely in one pass; the packet must
  identify which paths and views were assessed, which were skipped, and how
  confidence changes because of the bounded sample.
- Deployment or runtime topology is not observable from the repository alone;
  the packet must record an assessment gap rather than inventing environments
  or network paths.
- A brief mixes as-is analysis with requested future-state decisions; the mode
  must stay assessment-shaped and push actual decisions to `architecture` or
  `change` as follow-on modes.
- Different repository inputs disagree about a component or integration
  boundary; the packet must surface the disagreement as an inferred finding or
  assessment gap instead of flattening it into a confident fact.
- Required authored sections are missing or empty; the packet must surface the
  explicit `## Missing Authored Body` behavior instead of fabricating coverage.
- A user tries to run the mode for a brand-new system; the request must fail on
  system-context validation before any artifact set is published.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST support a new mode named `system-assessment` in the
  core mode model, CLI parsing, orchestration dispatch, summaries, and publish
  flow.
- **FR-002**: `system-assessment` MUST require authored input and support
  canonical input locations at `canon-input/system-assessment.md` and
  `canon-input/system-assessment/`.
- **FR-003**: `system-assessment` MUST require explicit
  `--system-context existing` and MUST reject `new` or missing system context
  in this first slice.
- **FR-004**: The new mode MUST remain read-only and MUST NOT present
  architecture decisions, implementation plans, or approved change direction as
  assessment facts.
- **FR-005**: The new mode MUST emit a dedicated assessment packet artifact set
  containing `assessment-overview.md`, `coverage-map.md`,
  `asset-inventory.md`, `functional-view.md`, `component-view.md`,
  `deployment-view.md`, `technology-view.md`, `integration-view.md`,
  `risk-register.md`, and `assessment-evidence.md`.
- **FR-006**: The packet MUST use ISO 42010 language for stakeholders,
  concerns, views, assessed scope, skipped scope, and partial coverage.
- **FR-007**: `assessment-evidence.md` MUST distinguish observed findings,
  inferred findings, and assessment gaps and MUST record confidence by
  assessed surface.
- **FR-008**: `coverage-map.md` MUST identify which repository surfaces and
  view families were assessed, partially covered, skipped, or unobservable.
- **FR-009**: When canonical authored sections are present and non-empty, the
  emitted artifacts MUST preserve those authored sections verbatim.
- **FR-010**: When required authored sections are absent or empty, the emitted
  artifacts MUST surface the explicit `## Missing Authored Body` behavior
  rather than fabricating plausible assessment content.
- **FR-011**: `canon publish` MUST publish completed or publishable
  `system-assessment` packets under `docs/architecture/assessments/`.
- **FR-012**: Embedded skill sources and mirrored `.agents/skills/` copies
  MUST define the same canonical authored sections, input rules, ISO 42010
  framing, existing-system-only posture, and follow-on guidance.
- **FR-013**: Documentation and worked examples MUST describe the new mode,
  including the distinction from `architecture`, the publish location, and the
  bounded coverage posture for large or partially observable repositories.
- **FR-014**: Shared runtime compatibility references, mode inspection output,
  and bootstrap surfaces MUST recognize `system-assessment` as a supported
  governed mode.
- **FR-015**: Focused contract, run, renderer or artifact-shape, publish, and
  documentation tests MUST be added for `system-assessment`, and broader shared
  mode-list coverage MUST include the new mode.
- **FR-016**: The feature plan MUST include an explicit first task for bumping
  the workspace and compatibility surfaces to `0.26.0`.
- **FR-017**: Cargo manifests, changelog, README, roadmap, and release-facing
  compatibility references MUST report Canon version `0.26.0` consistently for
  this delivery.
- **FR-018**: Existing mode behavior, `.canon/` schema, and non-target publish
  roots MUST remain unchanged except where shared mode registries and docs now
  include `system-assessment`.

### Key Entities *(include if feature involves data)*

- **System Assessment Packet**: The persisted as-is packet emitted by the new
  mode for one bounded existing-system review.
- **Assessment Surface**: A declared repository path, subsystem, dependency, or
  runtime surface included in the assessment scope.
- **Coverage Entry**: A record that names one stakeholder concern or view,
  describes whether it is fully assessed, partially assessed, skipped, or
  unobservable, and explains why.
- **Asset Inventory Entry**: A named service, component, data store,
  integration, or operational surface assessed by the packet.
- **Finding Entry**: A classified finding labeled `observed`, `inferred`, or
  `assessment-gap`, tied to evidence and confidence.
- **View Artifact**: One of the functional, component, deployment, technology,
  or integration view outputs emitted for the assessment.
- **Risk Register Entry**: An observed architecture or operational risk that is
  visible from the assessment without pretending the follow-on decision has
  already been made.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `system-assessment` can be parsed, executed, summarized, and
  published as a first-class mode in focused runtime tests.
- **SC-002**: A positive-path run with a complete example brief emits the full
  ten-artifact assessment packet under `.canon/` and publishes it under
  `docs/architecture/assessments/<RUN_ID>/`.
- **SC-003**: A negative-path validation proves that missing required authored
  sections or invalid system context produce honest blockers or missing-body
  markers instead of fabricated assessment coverage.
- **SC-004**: Embedded skills, mirrored skills, templates, examples, shared
  mode lists, and runtime compatibility references describe the same input
  contract and posture with no unresolved drift.
- **SC-005**: Release-facing repository surfaces report `0.26.0`
  consistently for the delivered slice.
- **SC-006**: The targeted `system-assessment` test suite passes,
  `/bin/bash scripts/validate-canon-skills.sh` passes,
  `cargo fmt --check` passes, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  passes.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
  `/bin/bash scripts/validate-canon-skills.sh`, and focused checks for shared
  mode-list and runtime-compatibility surfaces.
- **Logical validation**: Focused contract, run, publish, docs, and mode-list
  tests for `system-assessment`, followed by the narrowest non-regression tests
  affected by shared mode registration.
- **Independent validation**: Review `spec.md`, `plan.md`, and `tasks.md`
  before implementation, then perform a read-only walkthrough of a published
  packet to confirm it stays as-is, uses ISO 42010 coverage language, and does
  not present invented certainty.
- **Evidence artifacts**: Validation results and findings recorded in
  `specs/027-system-assessment-mode/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Deliver `system-assessment` as a separate first-class mode instead
  of extending `architecture`, **Rationale**: the roadmap explicitly separates
  as-is assessment from decision-shaped architecture work, and keeping the
  modes distinct preserves clearer contracts, summaries, and follow-on posture.
- **D-002**: Keep the first slice authored-input and repository-bounded rather
  than trying to solve exhaustive large-repo traversal, **Rationale**: explicit
  coverage and gap reporting provide honest value immediately without widening
  this slice into a general context-window management feature.

## Non-Goals

- Overloading `architecture` to behave like an assessment mode.
- Implementing structured publish descriptors or non-run-id path naming in this
  slice.
- Adding live external evidence collectors, SCM mining, or network-dependent
  architecture discovery.
- Claiming operational, security, or deployment certainty when the repository
  does not expose enough evidence.
- Building diagrams or full-spectrum enterprise architecture reporting.

## Assumptions

- Canon's existing artifact-contract and authored-body preservation model can
  support a first read-only `system-assessment` packet without changing
  `.canon/` persistence schema.
- A bounded authored brief can provide enough stakeholder and concern context
  for the first slice while repository validation fills the rest of the honest
  evidence posture.
- Large repositories can be handled in this slice by explicit assessment scope,
  skipped-surface reporting, and confidence grading rather than exhaustive
  traversal.
- `docs/architecture/assessments/` is an acceptable publish root for the first
  delivered assessment packet until structured external publish destinations are
  addressed separately.
