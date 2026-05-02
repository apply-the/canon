# Feature Specification: Architecture Clarification, Assumptions, And Readiness Reroute

**Feature Branch**: `037-architecture-clarification-readiness`  
**Created**: 2026-05-02  
**Status**: Draft  
**Input**: User description: "Formalize architecture clarification, working assumptions, readiness posture, and mode reroute guidance so Canon asks only decision-changing questions, records answers/defaults durably, materializes assumptions and unresolved questions in readiness-assessment, degrades readiness honestly, and redirects to discovery, requirements, or system-shaping when the brief is not architecture-ready."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This slice changes architecture-mode clarity contracts, readiness artifact shape, guidance, and release-facing documentation, but it does not introduce a new governed mode, a new persistence subsystem, or a new approval lifecycle.  
**Scope In**: architecture-mode `inspect clarity` question shaping and reroute guidance; durable architecture readiness output for working assumptions, unresolved questions, readiness posture, and recommended next mode; architecture-mode artifact contracts and markdown rendering; architecture templates, examples, shared skill guidance, README or mode guidance, roadmap cleanup, changelog, and `0.37.0` version alignment; focused Rust validation coverage for touched files plus clean `cargo fmt` and `cargo clippy` closeout.  
**Scope Out**: creating a live interview workflow outside existing Canon surfaces; adding a new governed mode; introducing a separate clarification-state store; redesigning approval, risk, or `.canon/` persistence semantics; broad behavior changes across unrelated modes beyond bounded reuse of shared clarity helpers.

**Invariants**:

- Architecture mode MUST keep asking only questions whose answers can materially change the structural recommendation, readiness, or downstream mode choice.
- Canon MUST preserve explicit `## Missing Authored Body`, missing-context, and recommendation-only semantics rather than fabricating certainty or silently promoting weak briefs to publishable posture.
- Canon MUST reuse the existing run, artifact, approval, and `.canon/` persistence model instead of introducing a parallel clarification or workflow store.
- Mode reroute guidance MUST remain a recommendation to existing modes such as `discovery`, `requirements`, or `system-shaping`, not a new orchestration surface.

**Decision Traceability**: Decisions and closeout evidence for this feature MUST be recorded in `specs/037-architecture-clarification-readiness/decision-log.md` and `specs/037-architecture-clarification-readiness/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ask Only Decision-Changing Architecture Questions (Priority: P1)

As a Canon maintainer preparing an architecture run, I want `inspect clarity` to ask only the questions that can actually change the architectural decision and to tell me the default if I skip them so clarification stays bounded and actionable.

**Why this priority**: If Canon cannot distinguish real decision blockers from cosmetic gaps, the architecture flow degrades into interview churn and weakens the product boundary immediately.

**Independent Test**: Given an architecture brief that is structurally present but materially ambiguous, a maintainer can run `canon inspect clarity --mode architecture` and observe a bounded question set with explicit decision impact, default-if-skipped posture, and reroute guidance when the brief is not actually architecture-ready.

**Acceptance Scenarios**:

1. **Given** an architecture brief with canonical sections but unresolved scale, availability, or ownership assumptions that could change the recommendation, **When** `inspect clarity` runs, **Then** Canon returns only the decision-changing clarification questions and includes the default that would apply if each question is skipped.
2. **Given** an input that is still a product-framing or capability-shaping brief rather than a bounded architecture decision surface, **When** `inspect clarity` runs for architecture mode, **Then** Canon marks the packet as not architecture-ready and recommends reroute to `discovery`, `requirements`, or `system-shaping` instead of asking architecture-only balance questions.
3. **Given** an architecture brief that already materially closes the decision, **When** `inspect clarity` runs, **Then** Canon preserves that closure explicitly and does not manufacture unnecessary clarification churn.

---

### User Story 2 - Materialize Assumptions And Unresolved Questions In Readiness Output (Priority: P2)

As a downstream reviewer or implementer, I want the architecture readiness packet to show the working assumptions, unresolved questions, and reroute conditions that shaped the recommendation so I can judge whether the decision is publishable or only conditionally useful.

**Why this priority**: Once questions are bounded, the next product value is making the resulting assumptions and unresolved conditions durable in the packet rather than hiding them inside generic blocker prose or chat history.

**Independent Test**: Given an architecture run, a reviewer can open `readiness-assessment.md` and see explicit sections for readiness posture, working assumptions, unresolved questions, blockers, accepted risks, and recommended next mode without consulting chat history.

**Acceptance Scenarios**:

1. **Given** an architecture brief that still depends on unanswered decision-changing factors, **When** the architecture packet is generated, **Then** `readiness-assessment.md` materializes working assumptions and unresolved questions explicitly instead of collapsing them into generic blocker text.
2. **Given** clarification answers or defaults that constrain the recommendation, **When** `readiness-assessment.md` is rendered, **Then** it records the resulting readiness posture and makes clear which assumptions still bound the decision.
3. **Given** an architecture packet that should be rerouted because the brief remains under-bounded, **When** `readiness-assessment.md` is rendered, **Then** it recommends the next existing mode and explains why architecture is premature.

---

### User Story 3 - Keep Templates, Docs, And Roadmap Aligned With The 0.37.0 Contract (Priority: P3)

As a Canon maintainer, I want the templates, skills, docs, roadmap, and release surfaces to describe the same `0.37.0` architecture clarification contract that the runtime and inspect surfaces actually ship.

**Why this priority**: This slice is about product discipline. If the runtime says one thing and the docs or templates say another, the feature is incomplete.

**Independent Test**: A reviewer can inspect the architecture template or example, shared mode guidance, roadmap, changelog, and validation report and confirm they all describe one coherent `0.37.0` story about bounded clarification, explicit assumptions, honest readiness, and reroute guidance.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a reviewer checks the architecture template, example input, and mode or README guidance, **Then** they all describe bounded clarification, working assumptions, readiness posture, and reroute behavior consistently.
2. **Given** the updated roadmap, **When** a maintainer checks the next-feature section, **Then** the now-delivered architecture clarification slice is no longer left as an implicit future gap and the roadmap remains forward-looking after `0.37.0`.
3. **Given** the completed validation report, **When** a reviewer inspects it, **Then** it records the real contract tests, clarity or artifact tests, docs review, coverage notes, and independent review outcome for this slice.

### Edge Cases

- The brief has the canonical architecture headings, but the authored bodies are still too shallow to support a structural recommendation.
- The brief is materially closed, but it still contains authored open questions that must remain visible without reopening false balance.
- A missing canonical heading should emit `## Missing Authored Body` and downgrade readiness honestly instead of being converted into a working assumption.
- The same under-bounded brief could be a `requirements` problem in one case and a `system-shaping` problem in another; reroute guidance must explain the trigger instead of naming a mode arbitrarily.
- A skipped clarification default must never silently upgrade the output to `publishable` when the resulting recommendation is still bounded by unresolved assumptions.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `canon inspect clarity --mode architecture` MUST surface only clarification questions whose answers can materially change the architectural recommendation, readiness posture, or next-mode recommendation.
- **FR-002**: Each architecture clarification question summary MUST identify the question prompt, why it matters, the packet surface it affects, and the explicit default that applies if the question is skipped.
- **FR-003**: Architecture clarity inspection MUST preserve materially closed decisions and MUST NOT synthesize clarification churn solely to force balanced-looking output.
- **FR-004**: Architecture clarity inspection MUST distinguish between a brief that is structurally sufficient but materially ambiguous and a brief that is not actually architecture-ready.
- **FR-005**: When a brief is not architecture-ready, Canon MUST recommend reroute to an existing mode such as `discovery`, `requirements`, or `system-shaping` and MUST explain the trigger for that reroute.
- **FR-006**: Mode reroute guidance MUST remain recommendation-only and MUST NOT create a new workflow, gate, or persistence subsystem.
- **FR-007**: `readiness-assessment.md` for architecture mode MUST include explicit sections for readiness posture, working assumptions, unresolved questions, blockers, accepted risks, and recommended next mode.
- **FR-008**: Architecture readiness output MUST make clear when a recommendation remains bounded by assumed defaults or unresolved questions.
- **FR-009**: Working assumptions in architecture readiness output MUST represent explicit temporary defaults or evidence-backed assumptions rather than replacements for missing canonical authored sections.
- **FR-010**: Missing canonical authored sections in architecture inputs MUST continue to surface as `## Missing Authored Body` rather than being converted into generated assumptions.
- **FR-011**: The architecture artifact contract MUST validate the updated readiness-assessment section requirements.
- **FR-012**: Architecture markdown rendering MUST materialize the updated readiness-assessment shape from the bounded clarification or readiness context without inventing unsupported certainty.
- **FR-013**: Shared architecture guidance, templates, examples, and skill instructions MUST align on the bounded clarification loop, explicit defaults, honest readiness downgrade, and reroute guidance.
- **FR-014**: The feature MUST preserve existing architecture mode artifacts, C4 artifacts, gates, approvals, and `.canon/` runtime layout unless an explicit contract change in this slice requires an additive update.
- **FR-015**: The feature MUST preserve current recommendation-only posture for architecture mode and MUST NOT turn Canon into a live interview orchestrator.
- **FR-016**: Cargo manifests, runtime compatibility references, impacted docs, and `CHANGELOG.md` MUST align to `0.37.0` for this delivery.
- **FR-017**: The generated task plan for this feature MUST include an explicit version-bump task and an explicit impacted-docs-plus-changelog task.
- **FR-018**: The generated task plan for this feature MUST include explicit coverage, `cargo clippy`, and `cargo fmt` closeout tasks.
- **FR-019**: Modified or newly created Rust files in this slice MUST receive focused automated validation coverage before the feature is complete.
- **FR-020**: Validation evidence for this slice MUST record the real clarity-contract checks, architecture artifact checks, docs or roadmap review, and independent review outcome.

### Key Entities *(include if feature involves data)*

- **Architecture Clarification Question Summary**: The inspect-facing record for one decision-changing question, including prompt, decision impact, affected packet surface, and default-if-skipped behavior.
- **Working Assumption Entry**: A durable statement explaining which temporary assumption or default currently bounds the architecture recommendation.
- **Unresolved Architecture Question**: A durable open issue that remains material to architecture readiness after authored input review and any applied defaults.
- **Architecture Readiness Record**: The readiness artifact that summarizes posture, assumptions, unresolved questions, blockers, accepted risks, and recommended next mode for an architecture packet.
- **Mode Reroute Recommendation**: A recommendation that an under-bounded brief should move to `discovery`, `requirements`, or `system-shaping` before architecture mode is treated as ready.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In all targeted architecture clarity tests, 100% of returned clarification question summaries include an explicit default-if-skipped and packet-impact explanation.
- **SC-002**: In all tested materially closed architecture briefs, Canon returns zero synthetic clarification questions added only for balance and preserves closure explicitly.
- **SC-003**: A reviewer can open `readiness-assessment.md` for a generated architecture packet and identify the working assumptions, unresolved questions, readiness posture, and recommended next mode in under 2 minutes.
- **SC-004**: Under-bounded architecture briefs in the validation suite are rerouted to an existing earlier mode with explicit rationale rather than being treated as architecture-ready.

## Validation Plan *(mandatory)*

- **Structural validation**: architecture artifact contract checks, clarity JSON contract checks, template or doc consistency checks, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **Logical validation**: focused Rust tests for architecture clarity question shaping, materially closed preservation, reroute guidance, readiness-assessment rendering, and docs or version alignment scenarios.
- **Independent validation**: a separate review pass over the architecture clarification contract, readiness semantics, reroute guidance, roadmap cleanup, and recorded evidence after implementation lands.
- **Evidence artifacts**: `specs/037-architecture-clarification-readiness/validation-report.md`, targeted Rust tests, updated docs and templates, and `lcov.info` for touched Rust files.

## Decision Log *(mandatory)*

- **D-001**: Extend the existing architecture clarity and readiness surfaces instead of creating a new interview mode or clarification store, **Rationale**: the product already has `inspect clarity`, architecture readiness artifacts, and earlier-mode reroute concepts, so the highest-value next step is to tighten those existing surfaces rather than widen the workflow model.

## Non-Goals

- Introduce a new governed mode or a new approval class for clarification.
- Add a live interview engine, conversational session manager, or background question queue.
- Redesign architecture C4 artifacts, approval semantics, or `.canon/` persistence beyond the bounded readiness and clarity changes required by this slice.
- Broaden this slice into a full cross-mode clarification redesign outside the bounded architecture and shared-helper surfaces needed to keep behavior coherent.

## Assumptions

- Architecture mode remains the correct target only when the decision surface, structural options, and invariants are already bounded enough to compare.
- Existing modes `discovery`, `requirements`, and `system-shaping` are the appropriate reroute destinations when architecture input is under-bounded for different reasons.
- `readiness-assessment.md` is the right durable packet surface for expressing architecture assumptions and unresolved questions without adding a new persistence family.
- Focused Rust validation for this slice can be delivered through repository-owned contract and run tests plus docs or version-alignment checks.
