# Feature Specification: Decision Alternatives, Pattern Choices, And Framework Evaluations

**Feature Branch**: `022-decision-alternatives`  
**Created**: 2026-04-27  
**Status**: Draft  
**Input**: User description: "Implement 022 Decision Alternatives, Pattern Choices, and Framework Evaluations across architecture, system-shaping, change, implementation, and migration with explicit option matrices, tradeoff analysis, ecosystem-health evidence, adoption guidance, and bounded persona guidance that stays subordinate to Canon governance. Keep review, pr-review, verification, incident, and remaining governed-mode persona rollout visible as follow-on work. Include release planning for a 0.22.0 version bump plus documentation, examples, and ROADMAP updates."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice deepens already-modeled Canon modes and their authored packet contracts without introducing a new runtime domain, new adapter class, or new approval semantics. The blast radius is bounded to authored decision/evaluation sections, markdown artifact preservation behavior, skill/template/example guidance, focused tests, and release-facing documentation and version references for the in-scope modes.  
**Scope In**:

- Extend `system-shaping`, `change`, `implementation`, and `migration` with explicit authored decision-alternatives or framework-evaluation packet shapes that let Canon preserve real options, tradeoffs, and recommendation rationale.
- Align `architecture` with the broader decision-alternatives feature so the already-delivered ADR/C4 option-analysis slice acts as the reference pattern instead of a one-off exception.
- Define explicit bounded personas for the in-scope modes and make the expected reviewer and operational personas discoverable for `review`, `pr-review`, `verification`, and `incident` guidance surfaces.
- Preserve authored comparison, evidence, and rationale sections verbatim in emitted artifacts when the canonical headings are present.
- Keep the critique-first posture honest when required comparison or evidence-grounding sections are absent.
- Update embedded skills, materialized skills, templates, worked examples, roadmap text, mode guidance, changelog entries, runtime compatibility references, and release-facing version surfaces for `0.22.0`.

**Scope Out**:

- Introducing the new `security-assessment` or `supply-chain-analysis` modes in this slice.
- Adding live external evidence collectors, registry scanners, GitHub mining adapters, or new network-visible protocol surfaces.
- Changing `.canon/` persistence layout, run identity, approval targets, publish destinations, or recommendation-only posture for operational modes.
- Reopening already-delivered authoring contracts for untouched modes except where guidance-only persona mapping or release/documentation sync is explicitly required.
- Packaging Canon for Homebrew, `winget`, Scoop, or other distribution channels in this slice.

**Invariants**:

- Canon MUST NOT fabricate multiple viable options when the authored brief or source surface makes the decision materially closed; the packet must say so explicitly instead.
- Persona guidance MUST remain subordinate to Canon contracts, missing-body or evidence-gap honesty markers, approval posture, and risk semantics.
- Existing runtime state transitions, publish destinations, evidence linkage, and recommendation-only boundaries MUST remain unchanged.
- Architecture's existing ADR/C4 behavior MUST remain intact while broader decision-alternatives support expands to other modes.
- Modes outside the explicitly targeted rollout MUST keep their observable runtime behavior unless a later scoped feature changes them.

**Decision Traceability**: Decisions start in this specification and continue in `specs/022-decision-alternatives/decision-log.md`, with validation evidence recorded in `specs/022-decision-alternatives/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Compare Real Structural Alternatives (Priority: P1)

As a Canon maintainer or operator, I want `system-shaping`, `architecture`, and
`change` packets to preserve real alternatives, decision drivers, tradeoffs,
and rejected options so reviewers can understand why a bounded structural or
design choice won.

**Why this priority**: This is the core value of the feature. Without explicit
alternatives and tradeoff preservation, Canon still jumps too quickly from a
brief to a single answer and loses the reasoning reviewers need.

**Independent Test**: With one authored brief each for `system-shaping`,
`architecture`, or `change` that names multiple viable options, the resulting
packet can be reviewed on its own to identify what alternatives existed, what
drove the choice, and why the chosen option won.

**Acceptance Scenarios**:

1. **Given** an authored `system-shaping` or `change` brief that compares more
   than one structural or pattern alternative, **When** Canon emits the packet,
   **Then** the packet preserves explicit decision summary, options matrix, and
   tradeoff rationale sections rather than flattening them into a single
   recommendation paragraph.
2. **Given** an authored `architecture` brief where the decision is already
   materially closed, **When** Canon emits the packet, **Then** the decision
   artifact states that only one viable option remains instead of inventing
   fake competitors.
3. **Given** a brief that omits one required comparison section, **When** Canon
   emits the packet, **Then** the affected artifact surfaces an explicit gap
   marker naming the missing authored section instead of fabricating the
   missing analysis.

---

### User Story 2 - Evaluate Concrete Stack And Migration Choices (Priority: P2)

As a delivery or migration owner, I want `implementation` and `migration`
packets to compare concrete framework, library, and platform choices with
ecosystem-health and adoption implications so the selected stack is visibly
grounded in constraints rather than vibes.

**Why this priority**: The highest-value follow-on from structural options is
concrete stack choice. These decisions become expensive quickly when the packet
does not capture ecosystem health, operational burden, and migration cost.

**Independent Test**: With one authored implementation or migration brief that
compares a bounded set of concrete stack options, the resulting packet can be
reviewed on its own to identify option fit, ecosystem-health reasoning,
adoption cost, and why the chosen option is preferable.

**Acceptance Scenarios**:

1. **Given** an authored `implementation` brief comparing candidate frameworks
   or libraries, **When** Canon emits the packet, **Then** it preserves explicit
   options, pros, cons, recommendation, ecosystem-health notes, and adoption
   implications for the named options.
2. **Given** an authored `migration` brief comparing coexistence or replacement
   paths, **When** Canon emits the packet, **Then** it preserves explicit
   migration tradeoffs, adoption burden, and rollback or coexistence
   consequences instead of collapsing them into a generic sequencing plan.
3. **Given** a concrete stack recommendation that lacks enough authored
   ecosystem evidence, **When** Canon emits the packet, **Then** the packet
   makes the evidence gap explicit instead of expressing unwarranted confidence.

---

### User Story 3 - Make Persona And Release Scope Explicit For 0.22.0 (Priority: P3)

As a maintainer shipping `0.22.0`, I want the in-scope decision modes, their
bounded personas, the remaining reviewer or operational personas, and the next
roadmap items to be explicit in docs and planning artifacts so future work does
not repeat first-slice ambiguity.

**Why this priority**: The repository just shipped a first-slice persona
rollout. If `022` lands without clarifying the next persona coverage and the
remaining roadmap, the next iteration will re-open the same scoping debate.

**Independent Test**: A maintainer can read the updated roadmap, mode guidance,
skills, templates, examples, changelog, and versioned compatibility references
and identify which modes gained option-analysis behavior now, which personas are
explicitly defined now, which remain follow-on, and that the repo reports
`0.22.0` consistently.

**Acceptance Scenarios**:

1. **Given** the completed repository artifacts, **When** a maintainer reviews
   `ROADMAP.md` and mode-facing guidance, **Then** `022` is clearly positioned
   as the just-delivered decision-alternatives slice and the remaining roadmap
   candidates stay visible.
2. **Given** `review`, `pr-review`, `verification`, and `incident` guidance
   surfaces, **When** a maintainer reads them, **Then** each mode has an
   explicit bounded persona description that does not imply extra authority or
   runtime behavior changes.
3. **Given** release-facing files and compatibility references, **When** a
   maintainer compares them, **Then** they all report `0.22.0` consistently for
   this slice.

### Edge Cases

- A decision brief compares only one genuinely viable option and one strawman;
  the packet must say the decision is effectively closed rather than pretending
  the matrix was balanced.
- An authored brief mixes structural options and concrete tool options in one
  packet, stressing the boundary between option-analysis and ecosystem-health
  evidence.
- The authored comparison uses near-match headings that should trigger the
  existing missing-body honesty behavior rather than silent preservation.
- A persona description pushes toward stronger certainty or seniority than the
  evidence supports.
- A migration brief includes coexistence steps but omits the adoption burden or
  rollback credibility needed to compare options honestly.
- An implementation brief names candidate tools but does not author enough
  ecosystem-health detail to justify the recommendation.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `system-shaping`, `architecture`, and `change` MUST support a
  canonical authored decision-alternatives shape that captures the decision,
  decision drivers, options considered, explicit tradeoffs, recommendation, and
  rejected alternatives.
- **FR-002**: `implementation` and `migration` MUST support a canonical
  authored framework-evaluation shape that captures concrete options,
  ecosystem-health reasoning, adoption or migration burden, recommendation, and
  implications of the selected option.
- **FR-003**: When a decision is materially closed to one viable option, the
  emitted packet MUST say so explicitly instead of fabricating additional
  alternatives.
- **FR-004**: The emitted artifacts for the in-scope modes MUST preserve the
  authored option-analysis sections verbatim when the canonical headings are
  present and non-empty.
- **FR-005**: When a required comparison or evidence-grounding section is
  absent or empty, the emitted packet MUST surface an explicit gap marker
  rather than synthesizing plausible reasoning.
- **FR-006**: `architecture` MUST retain its existing C4 and ADR behavior while
  aligning its decision-facing contract with the broader decision-alternatives
  feature language.
- **FR-007**: Embedded skill sources and materialized `.agents/skills/` copies
  for the in-scope modes MUST define the same canonical headings and bounded
  persona guidance.
- **FR-008**: The in-scope modes MUST define explicit bounded personas that fit
  their packet audience: architecture-decision, system-design, change-planning,
  delivery-lead, and migration-lead.
- **FR-009**: `review`, `pr-review`, `verification`, and `incident` guidance
  surfaces MUST declare explicit bounded personas in this slice even when their
  runtime artifact families do not otherwise change.
- **FR-010**: Persona guidance MUST remain guidance-only and MUST NOT weaken
  approval posture, risk semantics, missing-body honesty, or evidence
  requirements.
- **FR-011**: `docs/templates/canon-input/` and `docs/examples/canon-input/`
  for the in-scope modes MUST demonstrate the same canonical decision or
  framework-evaluation sections documented in the skill guidance.
- **FR-012**: Focused docs, renderer, run, and contract tests MUST exist for
  each runtime-targeted mode, covering both positive preservation and honest
  gap behavior for the new sections.
- **FR-013**: `ROADMAP.md`, `docs/guides/modes.md`, `README.md`, `CHANGELOG.md`,
  and shared runtime-compatibility references MUST describe this slice and the
  remaining roadmap candidates accurately.
- **FR-014**: Cargo manifests and release-facing repository references MUST
  report Canon version `0.22.0` consistently for this delivery.
- **FR-015**: Existing `.canon/` persistence, run identity, publish
  destinations, approval gates, recommendation-only posture, and non-target
  runtime behavior MUST remain unchanged.

### Key Entities *(include if feature involves data)*

- **Decision Alternatives Packet**: The authored packet structure that records
  the decision under review, viable alternatives, tradeoffs, recommendation,
  and rejection rationale.
- **Framework Evaluation Packet**: The authored packet structure that compares
  concrete tools or platforms, including ecosystem-health reasoning and
  adoption or migration burden.
- **Option Candidate**: A viable structural, pattern, framework, or platform
  choice being compared within a packet.
- **Mode Persona Profile**: The bounded authored counterpart for a mode,
  including intended audience, critique posture, and authority limits.
- **Gap Marker**: The explicit honesty signal emitted when authored comparison
  or evidence-grounding content is missing.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Each runtime-targeted mode in this slice has at least one focused
  positive validation proving authored option-analysis or framework-evaluation
  sections appear in emitted artifacts when present.
- **SC-002**: Each runtime-targeted mode in this slice has at least one focused
  negative validation proving missing required sections surface an explicit gap
  marker rather than fabricated prose.
- **SC-003**: The in-scope skills, templates, worked examples, and tests all
  describe the same authored headings and bounded personas with no unresolved
  drift.
- **SC-004**: A maintainer can inspect one emitted packet per targeted mode and
  identify the viable options, the selected option, the key tradeoffs, and why
  the rejected options lost without consulting chat history.
- **SC-005**: Release-facing documentation and compatibility references report
  `0.22.0` consistently, and the roadmap clearly shows the remaining feature
  candidates beyond `022`.
- **SC-006**: The targeted validation suite passes without changing runtime
  approval semantics, persistence layout, or non-target mode behavior.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`.
- **Logical validation**: Focused docs, contract, renderer, and run tests for
  `system-shaping`, `architecture`, `change`, `implementation`, and
  `migration`, plus targeted guidance validation for `review`, `pr-review`,
  `verification`, and `incident` persona text.
- **Independent validation**: Review of `spec.md`, `plan.md`, and `tasks.md`
  before implementation, followed by one realistic packet walkthrough and one
  negative missing-section walkthrough per targeted behavior group.
- **Evidence artifacts**: Validation results and findings recorded in
  `specs/022-decision-alternatives/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Keep the first `022` slice authored and evidence-grounded instead
  of adding live external evidence collectors, **Rationale**: this keeps the
  blast radius bounded while still improving option quality substantially.
- **D-002**: Treat `architecture` as the reference implementation and expand
  the decision-alternatives contract to the remaining targeted modes,
  **Rationale**: the repo already proved the pattern in one mode.
- **D-003**: Extend persona guidance beyond the first slice through skill and
  documentation surfaces before revisiting any runtime-visible persona model,
  **Rationale**: it addresses discoverability and audience fit without changing
  governance semantics.

## Non-Goals

- Shipping `security-assessment`, `supply-chain-analysis`, or package-manager
  distribution work in this slice.
- Introducing live registry, GitHub, or release-note scraping adapters.
- Changing recommendation-only posture, publish destinations, approval gates,
  or `.canon/` storage layout.
- Rewriting untouched mode artifact families just to normalize wording.

## Assumptions

- The authored-section preservation pattern already used in recent slices can
  be reused for new decision and framework-evaluation sections without
  altering Canon's runtime governance model.
- Reviewers value explicit rejected alternatives and adoption consequences more
  than generic summaries when judging engineering recommendations.
- Guidance-only persona rollout for `review`, `pr-review`, `verification`, and
  `incident` is sufficient in this slice; their runtime packet families do not
  need new decision-alternatives artifacts yet.
- The repository will treat `0.22.0` as the release identifier for this slice,
  so version and compatibility references belong in scope.
