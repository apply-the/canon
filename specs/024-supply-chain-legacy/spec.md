# Feature Specification: Supply Chain And Legacy Analysis Mode

**Feature Branch**: `024-supply-chain-legacy`  
**Created**: 2026-04-29  
**Status**: Draft  
**Input**: User description: "Create Feature 024 Supply Chain And Legacy Analysis Mode as a governed analysis mode for existing repositories. Add a new `supply-chain-analysis` mode that orchestrates governed scanner-backed evidence for SBOM, vulnerability triage, license-compliance posture, and legacy-posture analysis while remaining recommendation-only and evidence-grounded. The very first task must bump Canon release-facing version surfaces to `0.24.0`. The final task must guarantee high coverage on all Rust files added or modified by the feature, update docs, examples, roadmap, and release-facing surfaces, run `cargo fmt`, and resolve all `cargo clippy` warnings and errors." 

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: systemic-impact. This slice introduces a new first-class governed analysis mode, extends Canon's adapter-orchestration surface to scanner-backed evidence collection, adds clarification and tool-availability prompting, and changes mode parsing, summaries, publish behavior, documentation, and repository-wide regression coverage.  
**Scope In**:

- Add a new governed mode named `supply-chain-analysis` to Canon's runtime, CLI-visible mode surface, and mode metadata.
- Emit a recommendation-only supply-chain packet grounded in authored inputs and scanner-backed evidence for SBOM, vulnerabilities, license posture, and legacy dependency health.
- Support canonical authored inputs at `canon-input/supply-chain-analysis.md` and `canon-input/supply-chain-analysis/`.
- Add the pre-run clarification flow for licensing posture, distribution model, ecosystem scope confirmation, excluded components, and non-OSS scanner opt-in.
- Add toolchain detection, missing-scanner prompts, recorded install or skip decisions, and explicit coverage-gap behavior when a required scanner is unavailable.
- Add mode-specific artifact contracts, renderer behavior, summaries, gate evaluation, publish destination, and focused automated tests.
- Add embedded skill sources, mirrored `.agents/skills/` guidance, templates, examples, mode-guide documentation, changelog updates, roadmap updates, and runtime-compatibility references for the new mode.
- Keep the release boundary explicit for `0.24.0`, including a required first task for version-surface updates and a required final closeout task for coverage, docs, examples, roadmap, formatting, and lint remediation.

**Scope Out**:

- Reimplementing SBOM, vulnerability, license, or outdated-dependency scanners inside Canon.
- Automatically installing tools, mutating dependencies, rewriting manifests, or applying remediation changes to the analyzed repository.
- Claiming legal clearance, compliance certification, or a complete human security or legal review substitute.
- Adding unrelated protocol, packaging, or distribution-channel work in this slice.
- Reworking unrelated mode contracts except where shared runtime lists, validation helpers, or release-facing documentation must recognize the new mode.

**Invariants**:

- `supply-chain-analysis` MUST remain recommendation-only and MUST NOT silently install scanners, rewrite dependencies, or imply autonomous remediation.
- Supply-chain findings, license conclusions, and legacy-posture claims MUST be backed by authored input plus tool output, or they MUST surface an explicit evidence or coverage gap instead of fabricated certainty.
- Canon MUST NOT guess materially decision-changing inputs such as project licensing posture or non-OSS tool authorization; unresolved mandatory inputs must remain explicit gaps.
- Existing modes, run-state semantics, `.canon/` storage contracts, and publish destinations for other modes MUST remain unchanged unless explicitly extended to include the new mode.

**Decision Traceability**: Decisions start in this specification and continue in `specs/024-supply-chain-legacy/decision-log.md`, with validation evidence recorded in `specs/024-supply-chain-legacy/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run A Governed Supply-Chain Packet (Priority: P1)

As a Canon maintainer or operator, I want to run a dedicated
`supply-chain-analysis` mode against an existing repository so Canon emits a
reviewable packet covering SBOM, vulnerability triage, license posture, and
legacy dependency health instead of forcing that work into another mode.

**Why this priority**: This is the core product outcome. Without a real runtime
mode that can persist and publish a supply-chain packet, the feature remains
roadmap text rather than a user-visible workflow.

**Independent Test**: With one complete authored brief plus a repository that
contains supported manifests, Canon can run `supply-chain-analysis`, persist a
readable packet under `.canon/`, preserve recommendation-only posture, and
publish the packet to its dedicated docs destination.

**Acceptance Scenarios**:

1. **Given** a complete authored supply-chain brief for an existing repository,
   **When** Canon runs in `supply-chain-analysis` mode, **Then** it emits the
   defined supply-chain artifacts, records scanner-backed evidence or explicit
   gaps, and persists the packet under a dedicated mode artifact directory.
2. **Given** a systemic-impact or red-zone supply-chain run, **When** Canon
   evaluates the packet, **Then** it requires the same explicit risk approval
   discipline used by other governed high-risk modes instead of silently
   completing.
3. **Given** an ecosystem surface where at least one required scanner is
   unavailable, **When** Canon completes the bounded analysis, **Then** the
   packet remains readable but marks the uncovered scanner surface with an
   explicit coverage gap instead of inventing scanner output.

---

### User Story 2 - Clarify Posture And Govern Missing Tool Decisions (Priority: P2)

As an assistant or maintainer, I want the mode to ask for materially missing
licensing or tool-policy inputs and record missing-scanner decisions so the
resulting packet is auditable instead of being built on silent defaults.

**Why this priority**: Supply-chain posture changes meaningfully based on the
project's licensing and distribution model. If those inputs are guessed, the
packet is not trustworthy.

**Independent Test**: A maintainer can provide the missing posture answers,
review the tool-availability prompt, choose to skip or replace a scanner, and
observe Canon record the decision and affected coverage gap in the resulting
packet.

**Acceptance Scenarios**:

1. **Given** a supply-chain brief that omits commercial-versus-OSS posture,
   **When** Canon prepares the run, **Then** it asks for that posture before
   scanner-backed analysis proceeds instead of guessing.
2. **Given** a required scanner that is missing from `PATH`, **When** Canon
   evaluates the repository ecosystems, **Then** it presents install guidance,
   skip, or replacement options and records the resulting decision in the run
   evidence.
3. **Given** a user who does not authorize non-OSS scanner proposals,
   **When** Canon suggests missing tools, **Then** it limits recommendations to
   OSS-compatible options and marks any unsupported commercial-only coverage as
   an explicit gap rather than weakening the stated policy.

---

### User Story 3 - Ship 0.24.0 With Coverage And Quality Gates Closed (Priority: P3)

As a maintainer shipping `0.24.0`, I want the version bump, implementation
tasks, high-coverage requirement, docs and roadmap updates, formatting, and
lint-clean closeout to be explicit in the same feature package so release
completion is not deferred cleanup.

**Why this priority**: The user explicitly constrained the task order: version
surfaces first, high-coverage plus docs and quality-gate closeout last. Those
requirements must be first-class planning and implementation artifacts.

**Independent Test**: A maintainer can inspect the generated task plan,
release-facing files, validation report, and coverage evidence and confirm that
the first task bumps Canon to `0.24.0` and the final closeout task proves high
coverage plus clean formatting and lint status for the touched surfaces.

**Acceptance Scenarios**:

1. **Given** the generated `tasks.md`, **When** a maintainer reviews Phase 0,
   **Then** the very first task explicitly updates Canon release-facing version
   surfaces to `0.24.0`.
2. **Given** the finalized feature branch, **When** a maintainer compares
   Cargo manifests, shared runtime references, and release-facing docs,
   **Then** they all report `0.24.0` consistently for this delivery.
3. **Given** the final closeout task list, **When** a maintainer reviews the
   last task, **Then** it explicitly requires high coverage on every Rust file
   added or modified by this feature, updates docs/examples/roadmap surfaces,
   runs `cargo fmt`, and resolves `cargo clippy` warnings or errors.

### Edge Cases

- The repository mixes multiple ecosystems, but the authored brief limits the
  scan to only one or two of them; Canon must preserve the declared boundary
  instead of silently broadening scope.
- The user refuses to answer commercial-versus-OSS posture or non-OSS tool
  authorization; the run must surface a durable missing-decision marker rather
  than guessing.
- A scanner is available for one ecosystem but missing for another; Canon must
  report partial coverage honestly instead of collapsing the packet into a
  binary success or failure.
- No vulnerabilities are reported, but dependency freshness and EOL posture are
  still poor; the packet must capture legacy modernization pressure even when
  CVE findings are empty.
- Vendored, generated, or third-party directories appear inside the repository;
  Canon must honor explicit exclusions so the packet stays bounded.
- Machine-readable SBOM outputs exist, but the human packet omits the link or
  provenance; Canon must preserve the missing-evidence signal rather than
  implying the human packet is complete.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST support a new mode named `supply-chain-analysis` in
  the core mode model, CLI parsing, orchestration dispatch, summaries, inspect
  surfaces, and publish flow.
- **FR-002**: `supply-chain-analysis` MUST require an explicit existing-system
  context and support canonical authored inputs at
  `canon-input/supply-chain-analysis.md` and
  `canon-input/supply-chain-analysis/`.
- **FR-003**: The new mode MUST detect supported repository ecosystems from the
  provided manifest surface and bind the resulting analysis to the declared
  repository scope.
- **FR-004**: Before scanner execution begins, Canon MUST collect or confirm
  the project's licensing posture, distribution model, ecosystem scope,
  out-of-scope components, and non-OSS tool-policy choice whenever those inputs
  are missing from the authored brief.
- **FR-005**: If a user declines to provide a materially decision-changing
  posture input, the emitted packet MUST surface an explicit
  `Missing Authored Decision` marker rather than guessing.
- **FR-006**: Canon MUST map each detected ecosystem to a bounded scanner set
  appropriate for SBOM generation, vulnerability triage, license analysis, and
  legacy posture assessment.
- **FR-007**: When a required scanner is unavailable, Canon MUST present
  structured install, skip, or replacement guidance and record the user's
  resulting decision in run evidence.
- **FR-008**: Canon MUST NOT execute scanner installation itself, and scanner
  suggestions MUST default to OSI-approved OSS tools unless the user has
  explicitly authorized non-OSS proposals.
- **FR-009**: The new mode MUST emit a dedicated artifact family containing
  `analysis-overview.md`, `sbom-bundle.md`, `vulnerability-triage.md`,
  `license-compliance.md`, `legacy-posture.md`, `policy-decisions.md`, and
  `analysis-evidence.md`.
- **FR-010**: The supply-chain packet MUST preserve authored sections verbatim
  when the canonical headings are present and non-empty.
- **FR-011**: When a required authored section, scanner result, or user policy
  input is missing, the emitted packet MUST surface an explicit missing-body,
  missing-decision, or coverage-gap marker rather than synthesizing plausible
  analysis.
- **FR-012**: `supply-chain-analysis` MUST remain recommendation-only in this
  slice and MUST NOT imply autonomous remediation, dependency upgrades, or
  compliance certification.
- **FR-013**: The new mode MUST participate in risk and readiness gating with a
  gate profile appropriate for a governed operational analysis packet.
- **FR-014**: `canon publish` MUST publish completed or publishable
  supply-chain-analysis packets under `docs/supply-chain/<RUN_ID>/`.
- **FR-015**: Embedded skill sources and mirrored `.agents/skills/` copies MUST
  define the same canonical authored sections, clarification rules, tool-policy
  posture, and failure guidance for `supply-chain-analysis`.
- **FR-016**: Shared runtime compatibility references, validation helpers, and
  shared skill runtime checks MUST recognize `supply-chain-analysis` as a
  supported governed mode.
- **FR-017**: Documentation, templates, and worked examples MUST describe the
  new mode, including artifact intent, clarification requirements, publish
  location, and recommendation-only posture.
- **FR-018**: Focused contract, renderer, direct-runtime, and publish tests
  MUST be added for the new mode, and broader regression coverage MUST include
  the new runtime surfaces.
- **FR-019**: The generated task plan MUST make the first task the explicit
  version bump to `0.24.0` across Canon release-facing surfaces.
- **FR-020**: The generated task plan MUST make the final closeout task the
  explicit high-coverage, docs/examples/roadmap update, `cargo fmt`, and
  `cargo clippy` remediation step.
- **FR-021**: Cargo manifests and release-facing repository references MUST
  report Canon version `0.24.0` consistently for this delivery.
- **FR-022**: Validation evidence for this feature MUST demonstrate at least
  85% line coverage for each Rust source file added or modified by this feature.
- **FR-023**: Existing modes, `.canon/` schema, publish destinations for other
  modes, and non-target runtime behavior MUST remain unchanged.

### Key Entities *(include if feature involves data)*

- **Supply Chain Analysis Packet**: The persisted packet emitted by the new
  mode for a specific repository surface, combining authored context and
  scanner-backed evidence.
- **Ecosystem Scope**: The bounded set of manifests and dependency ecosystems
  that the run is allowed to analyze.
- **Scanner Requirement**: A required tool capability mapped to an ecosystem and
  analysis objective such as SBOM generation, vulnerability triage, license
  analysis, or legacy posture.
- **Scanner Decision Record**: The recorded install, skip, or replacement
  choice for a missing or substituted scanner.
- **SBOM Reference**: The machine-readable SBOM artifact or attachment linked
  from the human packet.
- **Vulnerability Finding**: A scanner-backed dependency finding with severity,
  affected component, fix posture, and triage disposition.
- **License Finding**: A dependency-license result evaluated against the
  declared project licensing posture and distribution model.
- **Legacy Finding**: An outdated, end-of-life, or abandonment signal tied to a
  bounded modernization recommendation.
- **Coverage Gap**: An explicit record that a scanner, manifest surface, or
  user decision required for complete analysis was unavailable or out of scope.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `supply-chain-analysis` can be parsed, executed, summarized, and
  published as a first-class mode in focused runtime tests.
- **SC-002**: The mode has at least one focused positive-path validation that
  proves the expected supply-chain artifacts are emitted and publishable for a
  bounded repository surface.
- **SC-003**: The mode has focused negative-path validation that proves missing
  scanner coverage or missing mandatory authored decisions surface explicit gap
  markers instead of fabricated supply-chain conclusions.
- **SC-004**: Embedded skills, mirrored skills, templates, examples, shared
  runtime references, and release-facing docs all describe the same authored
  headings, clarification requirements, and publish surface with no unresolved
  drift.
- **SC-005**: Release-facing files, shared runtime references, and planning
  artifacts all report `0.24.0` consistently for this delivery.
- **SC-006**: Validation evidence records at least 85% line coverage for every
  Rust file added or modified by this feature.
- **SC-007**: The final validation suite for this feature completes with clean
  formatting and no unresolved `cargo clippy` warnings or errors.

## Validation Plan *(mandatory)*

- **Structural validation**: Version-surface consistency checks, shared skill
  validator execution, `cargo fmt --check`, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **Logical validation**: Focused contract, renderer, direct-runtime, and
  publish tests for `supply-chain-analysis`, plus regression checks for shared
  runtime compatibility, mode inspection, and skill materialization.
- **Independent validation**: Review of `spec.md`, `plan.md`, and `tasks.md`
  before implementation, followed by one positive packet walkthrough and one
  negative coverage-gap or missing-decision walkthrough by a separate review
  pass.
- **Evidence artifacts**: Validation results and findings recorded in
  `specs/024-supply-chain-legacy/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Deliver `supply-chain-analysis` as a dedicated governed mode
  rather than folding the behavior into `security-assessment`,
  **Rationale**: dependency posture, licensing, and SBOM work are distinct
  analysis workflows with different artifacts and review audiences.
- **D-002**: Keep scanner orchestration recommendation-only and forbid silent
  tool installation, **Rationale**: it preserves Canon's control model and
  avoids privileged side effects during governed runs.
- **D-003**: Treat `0.24.0` version updates and high-coverage closeout as
  explicit first and last task constraints rather than implicit release chores,
  **Rationale**: the requested delivery order is part of the contract for this
  feature.

## Non-Goals

- Performing automatic dependency upgrades, lockfile rewrites, or manifest
  remediation.
- Certifying legal compatibility, regulatory compliance, or vulnerability
  acceptability without human review.
- Adding unrelated package-manager distribution or protocol-interoperability
  work in this slice.
- Rewriting untouched mode artifact families just to normalize terminology.

## Assumptions

- The analyzed repository already contains at least one manifest or dependency
  surface Canon can detect and bind into a bounded run.
- The repo-local governed-adapter pipeline can accommodate scanner-backed reads
  without changing Canon's recommendation-only posture.
- Machine-readable SBOM outputs can be persisted as companion evidence without
  changing `.canon/` storage semantics for unrelated modes.
- Maintainers are willing to supply explicit licensing and tool-policy answers
  when the authored brief omits them.
