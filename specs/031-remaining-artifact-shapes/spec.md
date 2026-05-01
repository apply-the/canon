# Feature Specification: Remaining Industry-Standard Artifact Shapes

**Feature Branch**: `031-remaining-artifact-shapes`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Complete the remaining industry-standard artifact shapes rollout for implementation, refactor, and verification with persona-bounded guidance, preserved canonical authored H2 contracts, focused contract/render/run validation, and explicit tasks for version bump, impacted docs plus changelog updates, coverage for modified or new Rust files, cargo clippy cleanup, and cargo fmt execution."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because this slice deepens already-modeled Canon packet contracts, skill guidance, renderers, tests, and release-facing surfaces for existing modes without introducing a new runtime mode, approval gate, persistence layout, or publish destination behavior  
**Scope In**:

- complete the remaining artifact-shape plus persona rollout for `implementation`, `refactor`, and `verification`
- align skill guidance, templates, worked examples, renderers, contract fixtures, and focused validation for the targeted modes
- update release-facing repository surfaces to `0.31.0`, including Cargo manifests, compatibility references, impacted docs, roadmap text, and `CHANGELOG.md`
- require explicit task planning for version bump, impacted docs plus changelog, coverage for modified or new Rust files, `cargo clippy`, and `cargo fmt`

**Scope Out**:

- introducing a new Canon mode or changing mode lifecycle semantics
- changing `.canon/` persistence, `run_id` identity, publish destinations, approval targets, or recommendation-only posture
- widening this slice to unrelated roadmap work such as distribution, package-manager, or new analysis-mode features
- rewriting non-targeted packet families purely for wording normalization

**Invariants**:

- Persona guidance MUST remain subordinate to Canon's canonical artifact contracts, explicit gap markers, risk posture, and evidence rules.
- `implementation`, `refactor`, and `verification` MUST preserve their existing governance semantics, including behavior-preservation expectations for refactors and blocked or unresolved-posture honesty for verification work.
- Modes outside `implementation`, `refactor`, and `verification` MUST keep their current observable behavior unless a later scoped change expands coverage.
- Runtime state transitions, `.canon/` storage layout, publish contracts, and approval semantics MUST remain unchanged.

**Decision Traceability**: Decisions start in this specification and continue in `specs/031-remaining-artifact-shapes/decision-log.md`, with validation evidence recorded in `specs/031-remaining-artifact-shapes/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Shape Implementation For Delivery Work (Priority: P1)

As a Canon maintainer using `implementation`, I want the packet to read like a
real delivery-planning artifact with task mapping, contract-test intent,
implementation notes, and bounded framework-evaluation guidance so execution
choices are explicit without degrading Canon's governance posture.

**Why this priority**: `implementation` is the highest-leverage remaining mode
because it directly affects how delivery recommendations are consumed after
change planning is complete.

**Independent Test**: With one representative implementation brief, Canon emits
an implementation packet that preserves authored task mapping, test intent,
implementation notes, and any real framework evaluation without requiring chat
context to understand the recommendation.

**Acceptance Scenarios**:

1. **Given** an authored implementation brief with the canonical headings,
   **When** Canon emits the packet, **Then** the packet reads like a delivery
   lead artifact with explicit task mapping, contract-test planning, and
   implementation notes while preserving the authored H2 contract.
2. **Given** an implementation brief where the concrete stack choice is already
   materially fixed, **When** Canon emits the packet, **Then** it states that
   the decision is bounded or closed instead of inventing a comparison matrix.
3. **Given** an implementation brief that omits a required authored section,
   **When** the packet is rendered, **Then** Canon still emits the existing
   explicit missing-body signal instead of backfilling the gap with
   persona-shaped prose.

---

### User Story 2 - Shape Refactor For Preserved Behavior (Priority: P2)

As a maintainer using `refactor`, I want the packet to read like a preserved
behavior matrix plus structural-rationale record so the artifact makes it clear
which invariants must hold, what mechanism changes are proposed, and why the
refactor does not silently add feature scope.

**Why this priority**: `refactor` is the most governance-sensitive remaining
authoring surface because the packet must improve structure without weakening
the no-feature-addition posture.

**Independent Test**: With one representative refactor brief, Canon emits a
packet whose preserved-behavior and structural-rationale surfaces are readable
as a maintenance artifact and still preserve Canon's behavior-preservation
contract.

**Acceptance Scenarios**:

1. **Given** an authored refactor brief with the canonical refactor headings,
   **When** Canon emits the packet, **Then** it preserves a clear invariant vs
   mechanism matrix and structural rationale rather than flattening the content
   into a generic summary.
2. **Given** a refactor brief whose authored sections are incomplete,
   **When** the packet is emitted, **Then** Canon still preserves explicit gap
   markers instead of inferring a stronger refactor plan than the authored
   material supports.
3. **Given** a refactor brief that risks slipping into feature work,
   **When** the packet is emitted, **Then** the artifact still foregrounds
   preserved behavior and makes any scope pressure inspectable rather than
   normalizing it.

---

### User Story 3 - Shape Verification For Claims And Evidence (Priority: P3)

As a maintainer using `verification`, I want the packet to read like a
claims-evidence-independence matrix so supported claims, missing evidence,
independence posture, and unresolved findings are directly inspectable without
losing Canon's verification honesty.

**Why this priority**: `verification` is the final remaining artifact-shape gap
and has the strictest requirement to stay adversarial and evidence-driven.

**Independent Test**: With one representative verification brief, Canon emits a
verification packet that preserves explicit claims, evidence, independence
checks, and unresolved-support posture in a reviewer-native matrix form.

**Acceptance Scenarios**:

1. **Given** an authored verification brief with the canonical headings,
   **When** Canon emits the packet, **Then** the artifact reads like a
   verifier-native claims and evidence matrix with independence posture made
   explicit.
2. **Given** a verification brief with unsupported or weakly supported claims,
   **When** the packet is emitted, **Then** Canon preserves explicit missing
   evidence or unresolved-finding signals rather than implying closure.
3. **Given** a verification brief missing a required authored section,
   **When** the packet is rendered, **Then** Canon emits the existing explicit
   missing-body behavior instead of inventing validating language.

---

### User Story 4 - Ship 0.31.0 With Aligned Docs And Validation (Priority: P4)

As a maintainer shipping this slice, I want versioned repository surfaces,
docs, changelog, coverage, lint, and formatting work to be explicit in the
feature workflow so the release surface remains trustworthy outside the code
diff.

**Why this priority**: This repository treats release-facing docs and
validation evidence as contract surfaces. The slice is incomplete if the code,
docs, versioning, and validation closeout drift apart.

**Independent Test**: A maintainer can inspect the final tasks and validation
evidence and confirm explicit work exists for the `0.31.0` version bump,
impacted docs plus changelog updates, modified-Rust-file coverage, `cargo
clippy`, and `cargo fmt`.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects Cargo
   manifests, compatibility references, docs, and `CHANGELOG.md`, **Then** they
   all report `0.31.0` consistently for this slice.
2. **Given** the generated task plan, **When** a maintainer reviews it,
   **Then** it includes an explicit version-bump task, an explicit impacted
   docs plus changelog task, a coverage task for modified or new Rust files,
   and explicit `cargo clippy` plus `cargo fmt` tasks.
3. **Given** the completed implementation, **When** a maintainer inspects the
   validation record, **Then** the targeted runtime paths have focused positive
   and negative validation plus clean lint and formatting closeout.

### Edge Cases

- An `implementation` brief names one real delivery path and no viable stack alternatives; the packet must state that the decision is already bounded instead of fabricating extra options.
- A `refactor` brief contains mechanism changes but weak invariant articulation; the packet must expose missing-body or preservation gaps rather than implying safe behavior preservation.
- A `verification` brief contains strong claims with sparse evidence; the packet must preserve explicit unsupported or unresolved posture rather than sounding complete.
- Targeted modes share renderer helpers with non-targeted modes; the slice must not introduce shape regressions in untouched packet families.
- Version surfaces or docs drift during implementation; validation must fail the slice rather than silently shipping inconsistent `0.31.0` references.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST define an explicit authored persona for each
  in-scope remaining mode: `implementation`, `refactor`, and `verification`.
- **FR-002**: The system MUST define an explicit industry-standard packet shape
  for each in-scope remaining mode.
- **FR-003**: The system MUST map `implementation` to a delivery-planning
  packet shape that preserves task mapping, contract-test intent,
  implementation notes, and bounded framework or library evaluation where the
  authored brief presents a real decision.
- **FR-004**: The system MUST map `refactor` to a preserved-behavior matrix
  plus structural-rationale packet while preserving Canon's no-feature-addition
  and invariant-first posture.
- **FR-005**: The system MUST map `verification` to a claims-evidence-
  independence matrix while preserving Canon's explicit unsupported,
  unresolved, and independence-posture semantics.
- **FR-006**: Persona guidance MUST remain guidance-only and MUST NOT override
  canonical artifact contracts, evidence rules, blocked-state honesty, or
  explicit missing-authored-body behavior.
- **FR-007**: For every in-scope remaining mode, the renderer MUST preserve the
  declared shaped authored sections when present and MUST emit the existing
  explicit gap signals when required authored sections are absent.
- **FR-008**: When an `implementation` brief does not contain a real multiple-
  option stack decision, the packet MUST say the decision is materially bounded
  or closed instead of inventing extra alternatives.
- **FR-009**: `refactor` packets MUST continue to make preserved behavior and
  scope boundaries inspectable and MUST NOT normalize feature expansion.
- **FR-010**: `verification` packets MUST continue to surface missing evidence,
  unsupported claims, unresolved findings, or blocked posture whenever the
  authored material cannot justify closure.
- **FR-011**: Repo-local skill sources and materialized `.agents/skills/`
  copies for the targeted modes MUST document the same canonical headings,
  persona guidance, and boundary conditions.
- **FR-012**: `docs/templates/canon-input/`, `docs/examples/canon-input/`, and
  other impacted operator-facing docs MUST demonstrate the same canonical
  sections documented in the targeted skill guidance.
- **FR-013**: The system MUST provide focused positive-path and negative-path
  validation evidence for `implementation`, `refactor`, and `verification`.
- **FR-014**: Modes outside `implementation`, `refactor`, and `verification`
  MUST remain behaviorally unchanged unless a later feature explicitly expands
  coverage.
- **FR-015**: Cargo manifests, lockfile surfaces, runtime compatibility
  references, impacted docs, and `CHANGELOG.md` MUST align to `0.31.0` for
  this slice.
- **FR-016**: The generated task plan MUST include an explicit version-bump
  task, an explicit impacted-docs-and-changelog task, a coverage task for
  modified or new Rust files, and explicit `cargo clippy` plus `cargo fmt`
  tasks.
- **FR-017**: Modified or newly created Rust files in this slice MUST receive
  focused automated validation coverage before the feature is complete.
- **FR-018**: Final validation for this slice MUST include formatting and lint
  closeout plus focused test or coverage checks that exercise the targeted
  runtime behavior.
- **FR-019**: This slice MUST NOT change `.canon/` persistence, canonical
  `run_id` identity, publish destinations, approval semantics, or
  recommendation-only posture.

### Key Entities *(include if feature involves data)*

- **Implementation Delivery Packet**: the shaped implementation artifact that
  records task mapping, test intent, implementation notes, and any bounded
  stack-evaluation content.
- **Refactor Preservation Matrix**: the shaped refactor artifact that ties
  preserved behavior and invariants to the structural or mechanism changes
  under consideration.
- **Verification Evidence Matrix**: the shaped verification artifact that ties
  claims, supporting evidence, independence posture, and unresolved findings
  together in one inspectable surface.
- **Mode Persona Profile**: the authored counterpart for a mode, including
  intended audience, critique posture, and explicit limits on implied
  authority.
- **Gap Marker**: the explicit honesty signal emitted when a required authored
  section or evidence basis is missing.
- **Validation Evidence Record**: the durable artifact set that captures
  positive-path, negative-path, and non-regression validation for this slice.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: For each in-scope remaining mode, at least one representative
  authored brief can be turned into a packet that an independent reviewer
  judges as both contract-compliant and audience-appropriate without extra chat
  context.
- **SC-002**: Negative-path validation for each in-scope remaining mode
  continues to surface explicit missing-content or unresolved-support markers
  whenever required authored sections or evidence are absent.
- **SC-003**: A maintainer can inspect one emitted packet for each targeted
  mode and identify the intended shape, persona, and governance posture within
  two minutes without consulting chat history.
- **SC-004**: Release-facing version surfaces, runtime compatibility
  references, and impacted docs consistently describe `0.31.0` and this slice's
  remaining artifact-shape contract.
- **SC-005**: Every modified or newly created Rust file in the slice is
  exercised by focused automated validation and the final closeout is clean on
  `cargo fmt` and `cargo clippy` expectations.
- **SC-006**: Existing validation for non-targeted modes shows no newly
  introduced behavioral regressions attributable to this slice.

## Validation Plan *(mandatory)*

- **Structural validation**: repository consistency checks for spec, plan, and
  tasks artifacts; version-surface consistency checks; `/bin/bash
  scripts/validate-canon-skills.sh`; and `cargo fmt --check`.
- **Logical validation**: focused contract, renderer, and run tests for
  `implementation`, `refactor`, and `verification`, plus targeted regression
  checks for shared renderer behavior and release-facing documentation
  alignment.
- **Independent validation**: review of `spec.md`, `plan.md`, and `tasks.md`
  before implementation, followed by an adversarial pass on missing-body,
  unsupported-claim, and preservation-honesty behavior.
- **Evidence artifacts**: `specs/031-remaining-artifact-shapes/decision-log.md`,
  `specs/031-remaining-artifact-shapes/validation-report.md`,
  `specs/031-remaining-artifact-shapes/tasks.md`, and any focused walkthrough
  or regression artifacts created during planning and implementation.

## Decision Log *(mandatory)*

- **D-001**: Complete the artifact-shapes rollout by targeting
  `implementation`, `refactor`, and `verification`, **Rationale**: the roadmap
  now identifies these as the remaining high-leverage follow-on modes for the
  feature family.
- **D-002**: Treat version bump, docs plus changelog alignment, Rust-file
  coverage, `cargo clippy`, and `cargo fmt` as first-class tasks in this slice,
  **Rationale**: release-facing drift and validation debt are known contract
  risks in this repository.
- **D-003**: Preserve existing verification honesty and refactor preservation
  semantics instead of widening runtime behavior, **Rationale**: the user value
  is better shaped packets, not new execution or approval semantics.

## Non-Goals

- Introduce a new Canon mode or alter mode lifecycle behavior as part of this
  slice.
- Change `.canon/` storage, run identity, publish destinations, or approval
  posture.
- Reopen unrelated distribution, packaging, or release-channel work beyond the
  updates strictly required for `0.31.0` alignment.
- Rewrite untouched mode artifact families purely for prose normalization.

## Assumptions

- The roadmap entries for remaining artifact-shape rollout are the
  authoritative product context for this feature.
- Existing authored-section preservation patterns can be extended to the
  remaining targeted modes without changing Canon's runtime governance model.
- Repo-local skills plus materialized `.agents/skills/` copies remain the
  primary authoring surface for assistants consuming Canon guidance.
- This delivery will ship as `0.31.0`, so Cargo manifests, compatibility
  references, docs, and changelog updates are in scope.
