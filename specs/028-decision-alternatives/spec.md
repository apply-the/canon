# Feature Specification: Decision Alternatives, Pattern Choices, And Framework Evaluations

**Feature Branch**: `028-decision-alternatives`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Implement 028 Decision Alternatives, Pattern Choices, and Framework Evaluations across system-shaping, change, implementation, and migration using the delivered architecture option-analysis baseline. Add read-only evidence collectors for registry, GitHub, release, and project-health signals. Keep bounded persona guidance subordinate to Canon governance. Include a 0.28.0 version bump, impacted docs and changelog updates, explicit tasks for version bump and documentation sweep, coverage for modified or new Rust files, clippy cleanup, and cargo fmt."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice deepens already-modeled Canon modes and their authored packet contracts without introducing a new runtime mode, adapter class, approval gate, or persistence layout. The blast radius stays bounded to authored section contracts, renderer preservation behavior, decision-evidence surfaces, documentation, release/version references, and focused runtime validation for the targeted modes.  
**Scope In**:

- Extend `system-shaping` and `change` packet shapes so they preserve bounded structural alternatives, pattern choices, tradeoff rationale, and explicit rejection logic.
- Extend `implementation` and `migration` packet shapes so they preserve bounded framework, library, and platform evaluations with explicit adoption or migration consequences.
- Keep `architecture` as the reference implementation for option-analysis language and align its surrounding guidance and regression coverage with the broader feature.
- Add explicit decision-evidence surfaces for the targeted modes so authored evidence references and missing-evidence honesty are first-class parts of the emitted packets.
- Keep persona guidance explicit for the targeted decision-heavy modes and ensure it remains subordinate to Canon contracts, risk posture, and evidence gaps.
- Update versioned repository surfaces to `0.28.0`, including Cargo manifests, runtime compatibility references, impacted guidance, examples, templates, and `CHANGELOG.md`.
- Add focused validation coverage for every modified or newly added Rust file, and close out the slice with `cargo fmt`, `cargo clippy`, and coverage validation.

**Scope Out**:

- Introducing a new governed runtime mode beyond the already-delivered mode set.
- Changing `.canon/` persistence layout, run identity, publish destinations, approval semantics, or recommendation-only posture.
- Adding write-capable external integrations, authenticated network adapters, or mutating package-manager automation.
- Rewriting untouched mode artifact families only for wording normalization.
- Reopening completed distribution work except where version, docs, or release-surface updates are directly required by `0.28.0`.

**Invariants**:

- Canon MUST NOT fabricate multiple viable options when the authored brief makes the decision materially closed; it must state that the choice is already bounded to one viable option.
- Persona guidance MUST remain guidance-only and MUST NOT override evidence requirements, approval posture, missing-section honesty, or risk semantics.
- Existing runtime state transitions, `.canon/` storage, publish paths, and approval targets MUST remain unchanged.
- Architecture's delivered ADR/C4 option-analysis baseline MUST remain intact while the broader decision-support shape expands to other modes.
- Any modified or newly created Rust runtime path MUST gain focused automated validation before the slice is considered complete.

**Decision Traceability**: Decisions start in this specification and continue in `specs/028-decision-alternatives/decision-log.md`, with validation evidence recorded in `specs/028-decision-alternatives/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Compare Real Structural Alternatives (Priority: P1)

As a Canon maintainer or reviewer, I want `system-shaping` and `change`
packets to preserve real structural or pattern alternatives, decision drivers,
tradeoffs, and rejected options so the packet explains why a bounded design
choice won instead of flattening the result into one recommendation.

**Why this priority**: This is the highest-value decision-support gap still
open after architecture's delivered option-analysis baseline. Without it,
structural and pattern-heavy work still loses the reasoning that reviewers need
to revisit decisions safely.

**Independent Test**: With one authored `system-shaping` brief and one authored
`change` brief that both name multiple viable alternatives, Canon emits packets
whose decision-facing artifacts preserve options, tradeoffs, recommendation,
and rejection rationale without changing any existing approval or publish
behavior.

**Acceptance Scenarios**:

1. **Given** an authored `system-shaping` brief that compares multiple
   structural options, **When** Canon emits the packet, **Then** the relevant
   artifacts preserve explicit decision summary, options considered, and
   tradeoff sections rather than collapsing them into a single paragraph.
2. **Given** an authored `change` brief where the real decision is between
   bounded design patterns, **When** Canon emits the packet, **Then** the
   packet preserves the chosen option, the rejected options, and the specific
   reasons they lost.
3. **Given** a brief where only one viable option remains, **When** Canon emits
   the packet, **Then** the packet states that the decision is materially
   closed instead of fabricating extra alternatives.

---

### User Story 2 - Evaluate Concrete Stack Choices With Honest Evidence (Priority: P2)

As a delivery or migration owner, I want `implementation` and `migration`
packets to compare concrete frameworks, libraries, or platform paths with
explicit evidence references, ecosystem posture, and adoption burden so the
recommended stack is visibly grounded and any missing evidence is called out
honestly.

**Why this priority**: Once structural choice is covered, the next expensive
failure mode is choosing a concrete stack without preserving why it fit the
constraints better than the alternatives.

**Independent Test**: With one authored `implementation` brief and one authored
`migration` brief that compare a bounded set of candidate tools or migration
paths, Canon emits packets that preserve framework evaluation sections,
decision evidence, adoption consequences, and explicit missing-evidence markers
where authored support is thin.

**Acceptance Scenarios**:

1. **Given** an authored `implementation` brief comparing concrete frameworks
   or libraries, **When** Canon emits the packet, **Then** it preserves options,
   pros, cons, recommendation, evidence references, and adoption implications
   for the named candidates.
2. **Given** an authored `migration` brief comparing coexistence, replacement,
   or modernization paths, **When** Canon emits the packet, **Then** it
   preserves migration burden, rollback or coexistence consequences, and why
   rejected paths lost.
3. **Given** an authored brief that lacks enough evidence references to support
   one recommendation, **When** Canon emits the packet, **Then** the packet
   marks the missing evidence explicitly instead of expressing unwarranted
   certainty.

---

### User Story 3 - Ship 0.28.0 With Aligned Docs, Versioning, And Validation (Priority: P3)

As a maintainer shipping this slice, I want the repository version surfaces,
impacted docs, examples, templates, and changelog to describe `0.28.0`
consistently, with task-level accountability for version bump, docs sweep, and
validation closeout, so the release surface stays trustworthy.

**Why this priority**: This repo treats release-facing docs and compatibility
references as contract surfaces. If the feature lands without version and docs
alignment, the runtime and release surface drift again.

**Independent Test**: A maintainer can inspect the updated version references,
read the impacted docs, and run the declared validation commands to confirm the
feature description, release version, and runtime compatibility all line up at
`0.28.0`.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects
   Cargo manifests, runtime compatibility references, and `CHANGELOG.md`,
   **Then** they all report `0.28.0` consistently for this slice.
2. **Given** the updated guidance and examples, **When** a maintainer reviews
   the impacted docs, **Then** the targeted modes describe the new decision and
   framework-evaluation shapes without contradicting runtime behavior.
3. **Given** the generated task list and completed implementation, **When** a
   maintainer reviews the final validation evidence, **Then** there is explicit
   coverage for modified Rust files plus successful `cargo fmt`, `cargo clippy`,
   and test or coverage closeout.

### Edge Cases

- A brief names one real option and one strawman; the packet must mark the decision as materially closed instead of pretending the matrix is balanced.
- An authored brief mixes structural alternatives and concrete tool choices; the packet must preserve the authored distinction instead of merging them into one vague comparison block.
- A required decision-analysis heading is nearly correct but not canonical; the packet must emit a missing-section honesty marker instead of preserving the near-match text silently.
- An implementation or migration brief links evidence sparsely or unevenly across candidates; the packet must expose the missing-evidence gap without weakening the recommendation-only posture.
- Version surfaces drift again during implementation; the release-facing docs, compatibility references, and changelog must fail the slice's validation expectations rather than ship inconsistent version anchors.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `system-shaping` and `change` MUST support canonical authored decision-analysis sections that capture the decision, decision drivers, options considered, tradeoffs, recommendation, and why the rejected options lost.
- **FR-002**: `implementation` and `migration` MUST support canonical authored framework-evaluation sections that capture candidate options, ecosystem-health posture, adoption or migration burden, recommendation, and evidence references.
- **FR-003**: The emitted packet for each targeted mode MUST preserve the authored decision-analysis or framework-evaluation sections verbatim when the canonical headings are present and non-empty.
- **FR-004**: When a required decision-analysis or evidence-grounding section is missing or empty, the emitted packet MUST surface an explicit honesty marker naming the missing authored section rather than fabricating content.
- **FR-005**: When the authored input makes the decision materially closed to one viable option, the packet MUST state that explicitly instead of inventing additional alternatives.
- **FR-006**: `architecture` MUST retain its delivered ADR/C4 option-analysis behavior and serve as the regression baseline for the broader decision-support language used in this slice.
- **FR-007**: Embedded skill sources and materialized `.agents/skills/` copies for the targeted modes MUST document the same canonical headings and bounded persona guidance.
- **FR-008**: Persona guidance for the targeted decision-heavy modes MUST be explicit about intended audience and critique posture, but MUST remain subordinate to Canon governance, risk controls, and evidence honesty.
- **FR-009**: Decision-evidence surfaces for the targeted modes MUST preserve authored evidence references and MUST expose missing evidence when the brief cannot support a strong comparison claim.
- **FR-010**: `docs/templates/canon-input/` and `docs/examples/canon-input/` for the targeted modes MUST demonstrate the same canonical decision or framework-evaluation sections documented in the skill guidance.
- **FR-011**: Focused docs, contract, renderer, and run tests MUST exist for each runtime-targeted mode in this slice, covering both positive preservation and honest missing-section behavior.
- **FR-012**: All modified or newly created Rust files in the targeted runtime paths MUST gain focused automated validation coverage that exercises the new behavior or guards against regressions.
- **FR-013**: `ROADMAP.md`, `README.md`, `docs/guides/modes.md`, impacted skill or template guidance, and `CHANGELOG.md` MUST describe this slice accurately and consistently.
- **FR-014**: Cargo manifests, lockfile surfaces, and shared runtime compatibility references MUST report Canon version `0.28.0` consistently for this delivery.
- **FR-015**: The task plan for this slice MUST include an explicit version-bump task, an explicit impacted-docs-and-changelog task, and explicit validation tasks for coverage, `cargo clippy`, and `cargo fmt`.
- **FR-016**: Existing `.canon/` persistence, publish destinations, approval gates, recommendation-only posture, and non-target mode runtime behavior MUST remain unchanged.

### Key Entities *(include if feature involves data)*

- **Decision Alternatives Packet**: The authored packet structure that records a bounded engineering decision, viable alternatives, tradeoffs, recommendation, and rejection rationale.
- **Framework Evaluation Packet**: The authored packet structure that compares concrete frameworks, libraries, or platform paths using explicit evidence and adoption consequences.
- **Option Candidate**: A viable structural, pattern, framework, or migration choice that is being compared within a decision packet.
- **Decision Evidence Reference**: An authored evidence anchor that supports a comparison claim, such as a source artifact, release note, registry page, or project-health citation.
- **Mode Persona Profile**: The bounded authored counterpart for a targeted mode, including intended audience, critique posture, and authority limits.
- **Gap Marker**: The explicit honesty signal emitted when required comparison or evidence sections are missing.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each runtime-targeted mode in this slice has at least one focused positive validation proving that authored decision-analysis or framework-evaluation sections appear in emitted artifacts when present.
- **SC-002**: Each runtime-targeted mode in this slice has at least one focused negative validation proving that missing required sections or missing evidence surface an explicit honesty marker instead of fabricated prose.
- **SC-003**: A maintainer can inspect one emitted packet per targeted mode and identify the viable options, the selected option, the key tradeoffs, and the evidence posture without consulting chat history.
- **SC-004**: Release-facing version surfaces, runtime compatibility references, and impacted docs report `0.28.0` consistently for this slice.
- **SC-005**: Every modified or newly added Rust runtime file in the slice is exercised by focused automated validation and the final validation record is clean on formatting and lint expectations.
- **SC-006**: Architecture's existing option-analysis baseline and non-target mode behavior continue to pass regression validation unchanged.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `/bin/bash scripts/validate-canon-skills.sh`, and repository consistency checks for spec, plan, and tasks artifacts.
- **Logical validation**: Focused docs, contract, renderer, and run tests for `system-shaping`, `change`, `implementation`, and `migration`, plus regression validation for `architecture` and targeted release-surface checks for versioned docs and compatibility references.
- **Independent validation**: Review of `spec.md`, `plan.md`, and `tasks.md` before implementation, followed by an adversarial pass on missing-evidence honesty and one realistic packet walkthrough per behavior group.
- **Evidence artifacts**: Validation results, coverage notes, lint closeout, and reviewer findings recorded in `specs/028-decision-alternatives/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Use `architecture` as the delivered regression baseline and focus new runtime behavior on `system-shaping`, `change`, `implementation`, and `migration`, **Rationale**: architecture already proved the option-analysis pattern and should anchor expansion instead of being re-designed.
- **D-002**: Keep this slice recommendation-only and documentation-heavy rather than introducing new approval or persistence semantics, **Rationale**: the user value is stronger decision packets, not a broader execution model.
- **D-003**: Treat version bump, impacted docs, changelog alignment, coverage, clippy, and fmt as first-class delivery work in this slice, **Rationale**: release-facing drift has already proven to be a real contract risk in this repository.

## Non-Goals

- Introducing a new Canon mode or reworking approval posture in this slice.
- Adding authenticated network scraping, mutating registry integrations, or package-manager automation beyond existing release channels.
- Reopening completed distribution work except where version and docs need to stay consistent with `0.28.0`.
- Rewriting untouched mode artifact families purely for prose normalization.

## Assumptions

- The authored-section preservation pattern already used in recent slices can be extended to additional decision-analysis headings without changing Canon's runtime governance model.
- Reviewers gain more value from explicit rejected options and evidence posture than from shorter but flatter recommendation summaries.
- Explicit decision-evidence references and missing-evidence honesty are sufficient for this slice even if richer live evidence harvesting is deferred to a later feature.
- The repository will treat `0.28.0` as the release identifier for this slice, so Cargo manifests, compatibility references, docs, and changelog updates are in scope.