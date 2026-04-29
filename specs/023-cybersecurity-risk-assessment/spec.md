# Feature Specification: Cybersecurity Risk Assessment Mode

**Feature Branch**: `023-cybersecurity-risk-assessment`  
**Created**: 2026-04-28  
**Status**: Draft  
**Input**: User description: "Define, plan, task, and implement Feature 023 as a governed Cybersecurity Risk Assessment Mode with a required 0.22.0 release boundary, explicit change tasks, added coverage, passing tests, cargo fmt, and lint clean-up."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: systemic-impact. This slice introduces a new first-class governed mode into Canon's core runtime model, orchestration flow, publish surface, authoring guidance, and validation matrix. The change touches mode parsing, gate evaluation, artifact contracts, publishing, docs, skill materialization, and repository-wide regression coverage.  
**Scope In**:

- Add a new governed mode named `security-assessment` to the Canon runtime and CLI-visible mode surface.
- Emit a recommendation-only security packet grounded in authored repository inputs, with explicit threat, risk, mitigation, assumption, and evidence artifacts.
- Support canonical authored inputs at `canon-input/security-assessment.md` and `canon-input/security-assessment/`.
- Add mode-specific artifact contracts, renderer behavior, gate evaluation, publish destination, summaries, and focused automated tests.
- Add embedded and mirrored skill guidance, templates, examples, mode-guide documentation, changelog updates, and runtime compatibility references for the new mode.
- Keep the release boundary explicit for `0.22.0`, including version references, validation evidence, coverage growth, formatting, linting, and full test execution.

**Scope Out**:

- Adding live vulnerability scanners, package-registry crawlers, GitHub mining, network calls, or external security adapters.
- Delivering `supply-chain-analysis` or any other future roadmap mode in this slice.
- Performing compliance audits, certifying conformance, or claiming Canon can replace human security review.
- Auto-applying mitigations, mutating the workspace on behalf of the security packet, or changing `.canon/` persistence shape.
- Reworking unrelated mode contracts except where shared runtime lists, docs, or validation surfaces must recognize the new mode.

**Invariants**:

- `security-assessment` MUST remain recommendation-only and MUST NOT imply autonomous remediation or enforcement.
- Existing modes, run-state semantics, approval posture, publish behavior, and `.canon/` storage contracts MUST remain unchanged unless explicitly extended to include the new mode.
- Security findings, threats, mitigations, and compliance anchors MUST stay grounded in authored inputs or surface explicit evidence gaps instead of fabricated certainty.
- The new mode MUST preserve the critique-first `## Missing Authored Body` behavior when required authored sections are absent.
- Release closeout for `0.22.0` MUST include validation evidence for tests, formatting, linting, and mode-specific regression coverage.

**Decision Traceability**: Decisions start in this specification and continue in `specs/023-cybersecurity-risk-assessment/decision-log.md`, with validation evidence recorded in `specs/023-cybersecurity-risk-assessment/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run A Governed Security Assessment Packet (Priority: P1)

As a Canon maintainer or operator, I want to run a dedicated
`security-assessment` mode against a bounded authored brief so Canon emits a
reviewable security packet with threats, risks, mitigations, assumptions, and
evidence rather than forcing security work into another mode.

**Why this priority**: This is the core product outcome. Without a real runtime
mode, the feature is only planning text and does not satisfy the user-visible
workflow gap.

**Independent Test**: With one complete authored security brief and a green or
bounded-impact run, Canon can persist a readable packet under `.canon/`, report
recommendation-only posture, and publish the packet to a dedicated docs path.

**Acceptance Scenarios**:

1. **Given** a complete authored security-assessment brief for an existing
   system, **When** Canon runs in `security-assessment` mode, **Then** it emits
   the defined security artifacts, keeps the run recommendation-only, and
   records the packet under a dedicated mode artifact directory.
2. **Given** a systemic-impact or red-zone security-assessment run, **When**
   Canon evaluates the packet, **Then** it requires the same explicit risk
   approval discipline used by other governed high-risk modes instead of
   silently completing.
3. **Given** a security-assessment brief that omits a required authored
   section, **When** Canon renders the affected artifact, **Then** the packet
   surfaces `## Missing Authored Body` for that gap instead of inventing a
   threat or mitigation narrative.

---

### User Story 2 - Author And Publish The Security Packet Consistently (Priority: P2)

As an assistant or maintainer, I want embedded skills, mirrored skills,
templates, examples, runtime compatibility checks, and publish paths to agree
on the `security-assessment` authoring contract so the mode is discoverable and
usable end to end.

**Why this priority**: A new mode is incomplete if the runtime exists but the
authoring and publish surfaces drift or hide the required packet shape.

**Independent Test**: A maintainer can read the new skill and example, provide
the expected authored sections, run skill validation, and publish a packet to a
stable documentation destination without hand-editing runtime internals.

**Acceptance Scenarios**:

1. **Given** the new embedded and mirrored security skill files, **When** a
   maintainer reviews them, **Then** they declare the same canonical authored
   H2 sections, input locations, recommendation-only posture, and failure
   guidance.
2. **Given** a completed security-assessment run, **When** `canon publish` is
   invoked, **Then** the packet publishes under `docs/security-assessments/<RUN_ID>/`.
3. **Given** runtime compatibility or skill-sync validation, **When** the
   repository checks supported modes, **Then** `security-assessment` is included
   consistently in the shared mode lists.

---

### User Story 3 - Ship 0.22.0 With Coverage And Quality Gates Closed (Priority: P3)

As a maintainer shipping `0.22.0`, I want the version bump, change log, added
coverage, full test pass, formatting, and lint-clean status to be tracked in
the same feature package so release closeout is part of the implementation and
not deferred cleanup.

**Why this priority**: The user explicitly requested the release bump first and
the coverage plus verification tasks at the end. Those must be first-class work
items, not implicit follow-up.

**Independent Test**: A maintainer can inspect versioned repository surfaces,
read the task plan, and reproduce a clean run of the focused mode tests plus
workspace quality gates recorded in the validation report.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer compares
   release-facing files, **Then** they consistently report `0.22.0` for this
   delivery.
2. **Given** the finalized implementation, **When** the maintainer runs the
   targeted security-assessment tests and the full workspace validation suite,
   **Then** the results pass and are recorded in the validation report.
3. **Given** the final task list, **When** it is reviewed, **Then** it contains
   explicit tasks for the version bump, implementation changes, coverage growth,
   passing tests, `cargo fmt`, and clippy warning or error resolution.

### Edge Cases

- A user submits a security brief for a system that is still mostly undefined;
  the mode must redirect or block on missing bounded context instead of
  fabricating assets and trust boundaries.
- The authored brief mixes security findings with implementation tasks; the
  packet must remain recommendation-only and avoid pretending it can apply
  remediations.
- A brief includes compliance frameworks but no threat or evidence grounding;
  Canon must not present compliance anchors as an audit result.
- A high-risk assessment in red zone is started without approval; the packet
  must remain gated rather than silently treated as an informational report.
- A published packet must not collide with incident or migration publish paths.
- Runtime compatibility lists, mode parsing, and canonical input binding can
  drift independently if the new mode is added only in one layer.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST support a new mode named `security-assessment` in the
  core mode model, CLI parsing, orchestration dispatch, summaries, and publish
  flow.
- **FR-002**: `security-assessment` MUST require authored input and support
  canonical input locations at `canon-input/security-assessment.md` and
  `canon-input/security-assessment/`.
- **FR-003**: The new mode MUST emit a dedicated security packet artifact set
  covering scope framing, threat analysis, risk findings, mitigations, gaps,
  and evidence or compliance anchors.
- **FR-004**: The new mode MUST preserve authored sections verbatim when the
  canonical headings are present and non-empty.
- **FR-005**: When required authored sections are absent or empty, the emitted
  artifacts MUST surface the explicit `## Missing Authored Body` behavior rather
  than synthesizing plausible security analysis.
- **FR-006**: `security-assessment` MUST remain recommendation-only in this
  slice and MUST NOT imply autonomous remediation or execution.
- **FR-007**: The new mode MUST participate in risk and readiness gating with a
  gate profile appropriate for a governed operational security packet.
- **FR-008**: System-context validation for the new mode MUST stay explicit and
  aligned with the intended existing-system operational workflow.
- **FR-009**: `canon publish` MUST publish completed or publishable
  security-assessment packets under a dedicated `docs/security-assessments/`
  destination.
- **FR-010**: Embedded skill sources and mirrored `.agents/skills/` copies MUST
  define the same canonical authored sections, input rules, posture, and next
  steps for `security-assessment`.
- **FR-011**: Shared runtime compatibility references and runtime helper
  scripts MUST recognize `security-assessment` as a supported governed mode.
- **FR-012**: Documentation and worked examples MUST describe the new mode,
  including artifact intent, publish location, and recommendation-only posture.
- **FR-013**: Focused contract, renderer, docs, run, and publish tests MUST be
  added for the new mode, and broader regression coverage MUST include the new
  runtime surfaces.
- **FR-014**: The feature plan MUST include an explicit version-bump task for
  `0.22.0`, explicit change tasks, explicit coverage tasks, explicit test-pass
  verification tasks, explicit `cargo fmt` execution, and explicit clippy
  warning or error remediation.
- **FR-015**: Cargo manifests and release-facing repository references MUST
  report Canon version `0.22.0` consistently for this delivery.
- **FR-016**: Existing modes, `.canon/` schema, publish destinations for other
  modes, and non-target runtime behavior MUST remain unchanged.

### Key Entities *(include if feature involves data)*

- **Security Assessment Packet**: The persisted packet emitted by the new mode,
  containing the bounded security review for a specific system surface.
- **Asset Inventory Entry**: A named in-scope asset, service, data flow, or
  configuration surface the assessment evaluates.
- **Trust Boundary**: A boundary across which threats, attacker movement, or
  data classification risk are evaluated.
- **Threat Entry**: A concrete threat hypothesis tied to an asset, boundary, or
  attacker goal.
- **Risk Register Entry**: A likelihood and impact rated finding with owner,
  status, and mitigation linkage.
- **Mitigation Proposal**: A recommended control or change mapped to one or
  more identified risks while preserving recommendation-only posture.
- **Compliance Anchor**: A reference to a relevant standard or control family
  that informs the packet without claiming audit completion.
- **Evidence Gap**: An explicit statement that source grounding is missing,
  stale, or insufficient for a confident security conclusion.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: `security-assessment` can be parsed, executed, and summarized as a
  first-class mode in focused runtime tests.
- **SC-002**: The mode has at least one focused positive-path validation proving
  the expected security artifacts are emitted and publishable.
- **SC-003**: The mode has at least one focused negative-path validation proving
  missing authored sections surface explicit gap markers instead of fabricated
  threats or mitigations.
- **SC-004**: Embedded skills, mirrored skills, templates, examples, shared mode
  lists, and mode docs all describe the same input contract and posture with no
  unresolved drift.
- **SC-005**: Release-facing documentation and compatibility references report
  `0.22.0` consistently for the delivered slice.
- **SC-006**: The targeted security-assessment test suite passes, the full
  `cargo nextest run` suite passes, `cargo fmt --check` passes, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `/bin/bash scripts/validate-canon-skills.sh`, and focused checks for shared runtime compatibility surfaces.
- **Logical validation**: Focused contract, renderer, docs, run, and publish tests for `security-assessment`, followed by a full `cargo nextest run`.
- **Independent validation**: Review of `spec.md`, `plan.md`, and `tasks.md` before implementation, followed by a read-only packet walkthrough confirming recommendation-only posture, missing-body honesty, and publish-path correctness.
- **Evidence artifacts**: Validation results and findings recorded in `specs/023-cybersecurity-risk-assessment/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Deliver the first `security-assessment` slice as an authored-input,
  recommendation-only governed mode rather than adding live scanners or
  auto-remediation, **Rationale**: this gives Canon a real security workflow in
  `0.22.0` while keeping blast radius, trust boundaries, and validation scope
  manageable.

## Non-Goals

- Shipping `supply-chain-analysis`, package-manager distribution, or any other
  future roadmap mode in this slice.
- Adding network-reliant evidence collection, scanner integrations, or package
  inventory ingestion.
- Claiming audit certification, policy compliance sign-off, or autonomous
  remediation authority.
- Changing unrelated mode artifact families just to normalize language.

## Assumptions

- The existing authored-section preservation model used by other governed modes
  can support the initial security packet without a new persistence schema.
- A recommendation-only operational posture is sufficient for the first
  delivered security-assessment slice.
- Shared runtime surfaces such as canonical input binding, publish directories,
  summaries, and mode validation can be extended without changing existing mode
  behavior.
- The repository will treat `0.22.0` as the release identifier for this slice,
  so version references and release docs belong in scope.
