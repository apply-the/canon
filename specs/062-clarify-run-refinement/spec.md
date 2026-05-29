# Feature Specification: Mode Clarification And Run Refinement

**Feature Branch**: `062-clarify-run-refinement`  
**Created**: 2026-05-29  
**Status**: Draft  
**Input**: User description: "let's proceed with the design you drafted and the clarification phase as described by speckit above for `requirements`, `discovery`, `system-shaping`, `architecture`, and `change`. It is important that canon gives a way to refine a run instead of doing always another (this applies to all modes I believe)"

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: systemic-impact. This slice changes governed clarification and continuation behavior across multiple existing modes, introduces a shared run-refinement contract, and affects how Canon preserves working state before and during runs.  
**Scope In**: a mode-specific working brief lifecycle for `requirements`, `discovery`, `system-shaping`, `architecture`, and `change`; explicit continue or refine identity continuity across all modes so follow-up clarification or additional authored context is not treated as an implicit new run; a clarification phase that fills and revises the targeted-mode working brief from source material plus user answers; explicit refine-the-current-run behavior instead of defaulting to a replacement run when the user intent is continuation; run or inspect surfaces that show authoritative working brief, supporting inputs, clarification answers, defaults, and remaining readiness delta; shared docs, templates, and evidence expectations needed to explain the new lifecycle.  
**Scope Out**: mutating raw `canon-input/` files, starter templates, or published packets in place; redesigning approvals, publish destinations, or artifact families unrelated to refinement; shipping first-class mode-specific clarification taxonomies for every non-targeted mode in the same slice; silently merging unrelated fresh work into an older run.

**Invariants**:

- The original authored input surface MUST remain immutable evidence; clarification updates MUST land in a run-scoped or draft-scoped working brief, not back into `canon-input/` or static templates.
- Once a governed run exists, refinement for the same work MUST preserve that work item's identity and traceability instead of silently creating a replacement run.
- Candidate detection is advisory; continuation requires explicit intent before Canon mutates an existing draft or run.
- Canon MUST preserve one explicit authoritative brief plus supporting inputs; supporting context MUST NOT replace the current-mode brief.
- Clarification and refinement MUST keep missing context, defaults, and unresolved questions explicit instead of fabricating readiness or certainty.
- Draft work item identity MAY change mode during pre-run clarification; governed run identity MUST NOT change mode after the run has started.

**Decision Traceability**: Decisions and validation evidence for this feature MUST be recorded in `specs/062-clarify-run-refinement/decision-log.md` and `specs/062-clarify-run-refinement/validation-report.md`, with runtime-facing evidence linked from the affected run or inspect surfaces.

## Clarifications

### Session 2026-05-29

- Q: When should Canon create the work identity that clarification refines? → A: Canon creates a durable draft work item when clarification starts, keeps the working brief, source-input snapshot references, clarification records, defaults, unresolved questions, and readiness delta on that draft, then promotes or binds that same identity into governed run lineage when the run starts. If multiple draft or run candidates match the user intent, Canon must ask for disambiguation before mutating state.
- Q: What should Canon do when clarification shows the current mode is wrong? → A: Canon may switch mode in place while the work is still a draft work item because clarification is still discovering the correct mode. After a governed run has started, Canon must not mutate that run into a different mode; it must create a linked successor work item for the new mode, carrying forward the working brief, clarification records, source input snapshot references, readiness delta, and rationale for the mode change. The original run remains inspectable and preserves its mode, evidence, approval, and lineage semantics. The successor records `carried_from` and `supersedes` lineage back to the original draft or run.
- Q: For non-targeted modes in this feature, what minimum behavior is required? → A: All modes get explicit continue or refine identity continuity in this feature so Canon no longer treats follow-up clarification or additional authored context as an implicit new run. Only `requirements`, `discovery`, `system-shaping`, `architecture`, and `change` get the first-class working-brief clarification lifecycle in this slice. For non-targeted modes, the minimum behavior is to detect likely continuation intent, preserve the current governed work identity when refining the same work, ask for disambiguation when multiple runs or fresh-versus-continuation intent is unclear, preserve approval, evidence, and gate semantics, expose current refinement state through status or inspect, and avoid silently creating replacement runs.
- Q: When Canon finds exactly one plausible draft or run candidate, how should it decide between continuing it and starting new work? → A: Canon should detect and surface the single likely draft or run candidate, but it should continue or refine that work item only when the user explicitly signals continuation, for example by saying "continue", "refine", "resume", "same run", or by providing a draft or run id. If the user provides a fresh request without an explicit continuation signal, Canon should start new work rather than mutating the existing candidate. Candidate detection is advisory; continuation requires explicit intent.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Refine The Same Work Item (Priority: P1)

As a Canon operator, I want clarification follow-up to refine the current governed work item instead of starting from scratch so I can preserve context, traceability, and progress when a packet is incomplete.

**Why this priority**: If Canon treats every clarification cycle as a new run, operators lose continuity immediately and the governed workflow becomes noisy, duplicative, and harder to trust.

**Independent Test**: Given a targeted mode packet that needs clarification, an operator can refine the same durable draft work item, answer the follow-up questions, and observe updated readiness without Canon silently creating a second run for the same intent.

**Acceptance Scenarios**:

1. **Given** a `requirements`, `discovery`, `system-shaping`, `architecture`, or `change` work item with unresolved readiness questions, **When** clarification starts or resumes, **Then** Canon creates or reuses the same durable draft work item and records answers against that same work identity instead of forcing a replacement run.
2. **Given** an existing run with previously captured clarification answers and defaults, **When** the operator inspects or resumes that work, **Then** Canon shows the prior refinement state and any remaining questions instead of restarting the conversation from zero.
3. **Given** a truly new request that is not a continuation of the current governed work, **When** the operator starts fresh work, **Then** Canon still allows a new run explicitly and does not silently attach the new intent to an older run.
4. **Given** Canon surfaces exactly one likely existing draft or run candidate, **When** the operator does not explicitly signal continuation, **Then** Canon starts new work and may present the candidate as a suggested continuation path, but it does not mutate that candidate.
5. **Given** a durable draft work item that is ready to start execution, **When** the operator starts the governed run, **Then** Canon promotes or binds that same work identity into the governed run lineage without implying approval, publish eligibility, or completed readiness before the run actually starts.

---

### User Story 2 - Update A Working Brief, Not The Source Inputs (Priority: P2)

As a Canon operator, I want clarification answers to populate a mode-specific working brief derived from the correct phase template so the governed packet can improve without rewriting the original source inputs.

**Why this priority**: Preserving input immutability while still accumulating clarification answers is the core product design change that makes the clarification phase compatible with Canon's evidence model.

**Independent Test**: Given a packet assembled from source files or inline input, an operator can enter clarification, see an authoritative working brief for the current mode, and confirm that source inputs remain unchanged while the working brief reflects the clarified state.

**Acceptance Scenarios**:

1. **Given** a targeted mode begins clarification from a canonical template, **When** Canon materializes the working brief, **Then** it seeds the mode's required sections with what is already known and leaves unresolved content explicit.
2. **Given** the operator answers or skips clarification questions, **When** Canon updates the working brief, **Then** it records accepted answers, applied defaults, and untouched open questions in the sections they affect.
3. **Given** the packet contains supporting inputs but no clear authoritative brief, **When** clarification begins, **Then** Canon surfaces that authority problem explicitly and requires a clear working brief instead of editing supporting files opportunistically.

---

### User Story 3 - Keep Refinement Behavior Consistent Across Modes (Priority: P3)

As a Canon maintainer, I want every mode to expose explicit continuation or refinement identity continuity, while the first full clarification-template workflow lands only for the targeted planning modes, so the lifecycle stays coherent across the product without widening this slice too far.

**Why this priority**: The targeted modes need deeper clarification behavior first, but the user expectation that Canon can refine current work rather than always spawn another run applies across the broader product.

**Independent Test**: A maintainer can inspect mode guidance and lifecycle outputs and confirm that targeted modes support working-brief clarification, while all other modes still provide an explicit refine or continue path for the current governed work instead of defaulting silently to a new run.

**Acceptance Scenarios**:

1. **Given** a non-targeted mode run still needs follow-up clarification or additional authored context, **When** the operator asks to continue that work, **Then** Canon detects likely continuation intent, offers an explicit refine or continue path tied to the current work item, and avoids assuming that a new run is required.
2. **Given** clarification reveals that the current targeted mode is wrong before the governed run starts, **When** Canon changes direction, **Then** it may switch the draft work item to the corrected mode in place while preserving the same draft identity and working brief.
3. **Given** clarification or later evidence reveals that an already-started governed run is in the wrong mode, **When** Canon redirects the work, **Then** it preserves the original run unchanged and creates a linked successor work item for the new mode with explicit carry-forward lineage and rationale.
4. **Given** docs, templates, and runtime summaries for the targeted modes, **When** a reviewer compares them, **Then** they all describe the same working-brief and refine-the-current-run lifecycle.

### Edge Cases

- The packet has immutable source inputs plus supporting files, but no single authoritative current-mode brief can be identified safely.
- A later clarification answer conflicts with an earlier accepted answer or with a declared invariant that the current work must preserve.
- The operator asks to refine a run that is already gated or awaiting approval; refinement must not bypass gate semantics or hide the existing state transition.
- Clarification shows that the current mode is wrong and the work should move earlier or sideways in the lifecycle; the collected context must remain visible and reusable.
- A mode switch discovered before run start must not create unnecessary successor churn when the draft identity can still change in place.
- A mode switch discovered after run start must preserve the original run intact and expose successor lineage clearly enough for inspection and audit.
- Canon finds exactly one likely continuation candidate, but the operator submits fresh work without an explicit continuation signal.
- Multiple recent runs match the user's phrasing; Canon must ask which governed work item to refine instead of guessing.
- The source input was inline text rather than a file-backed packet; Canon still needs an authoritative working brief that can be inspected later.
- A non-targeted mode receives follow-up context but does not yet support first-class working-brief clarification; Canon still has to preserve identity continuity and surface the current refinement state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST provide an explicit operator-facing refine, continue, or resume path whenever an existing governed work item is a plausible continuation target so user intent to continue the same work is not treated as an implicit request for a new run.
- **FR-002**: `requirements`, `discovery`, `system-shaping`, `architecture`, and `change` MUST support a clarification phase that operates on a mode-specific working brief derived from the current phase template.
- **FR-003**: When clarification starts for a targeted mode, Canon MUST create or reuse a durable draft work item that owns the authoritative working brief, source input snapshot references, clarification records, defaults, unresolved questions, and readiness delta for that work.
- **FR-004**: The clarification phase for targeted modes MUST populate the working brief with known source content, prior clarification answers, and explicit unresolved placeholders or defaults.
- **FR-005**: Canon MUST keep original authored inputs immutable during clarification and refinement; no accepted answer, default, or inferred update may rewrite `canon-input/`, starter templates, or published outputs in place.
- **FR-006**: Canon MUST distinguish authoritative working brief content from supporting inputs and MUST surface an explicit readiness gap when no authoritative current-mode brief can be identified.
- **FR-007**: Clarification questions for targeted modes MUST be bounded to decision-changing gaps that can change mode fit, scope boundaries, readiness state, required artifacts, approval or gate expectations, user-visible outputs, or other operator choices; Canon MUST NOT ask questions that only restate already-authoritative content or collect non-actionable trivia.
- **FR-008**: The targeted clarification phase MUST record, per question, the prompt, the answer or applied default, the affected section, and whether the item remains unresolved.
- **FR-009**: Accepted clarification answers and applied defaults MUST update the authoritative working brief for the same governed work item instead of creating a disconnected second draft.
- **FR-010**: When the operator starts the governed run from a durable draft work item, Canon MUST promote or bind that same work identity into the governed run lineage instead of creating a separate unrelated identity.
- **FR-011**: A durable draft work item MUST NOT imply approval state, publish eligibility, or completed packet readiness before the governed run actually starts.
- **FR-012**: If clarification reveals that the current mode is no longer the correct mode before the governed run starts, Canon MAY switch the draft work item to the corrected mode in place while preserving the same draft identity, working brief, and clarification history.
- **FR-013**: All modes MUST preserve explicit continuation identity continuity in the runtime and persistence model once the operator chooses to refine or continue existing work so follow-up clarification or additional authored context is not treated as an implicit new run.
- **FR-014**: Only `requirements`, `discovery`, `system-shaping`, `architecture`, and `change` receive the first-class working-brief clarification lifecycle in this slice; non-targeted modes remain continuity-capable without taking on full mode-specific working-brief clarification.
- **FR-015**: For non-targeted modes, the minimum required behavior MUST include likely continuation-candidate detection, advisory surfacing of the current governed work identity, and identity preservation when the operator is refining the same work.
- **FR-016**: Candidate detection is advisory; Canon MUST continue or refine an existing draft or run only when the operator explicitly signals continuation, and when continuation intent is ambiguous, or when multiple draft or run candidates plausibly match the user's intent, it MUST ask the operator to disambiguate before mutating state.
- **FR-017**: Refinement MUST preserve existing approval, evidence, and recommendation-only semantics; follow-up clarification MUST NOT bypass gates, erase prior findings, or hide prior state.
- **FR-018**: If the correct mode changes after a governed run has started, Canon MUST preserve the original run unchanged and create a linked successor work item for the new mode rather than mutating the run in place.
- **FR-019**: A successor work item created from a started governed run MUST carry forward the working brief, clarification records, source input snapshot references, readiness delta, and explicit rationale for the mode change.
- **FR-020**: A successor created for post-start mode change MUST record lineage fields that make the relationship explicit, including `carried_from` and `supersedes`, and MUST leave the original run inspectable under its original mode, evidence, approval, and lineage semantics.
- **FR-021**: Canon status or inspect surfaces MUST show the authoritative working brief, supporting inputs, recorded clarification answers, applied defaults, unresolved items, and current readiness delta for the governed work.
- **FR-022**: The working-brief lifecycle MUST support both file-backed and inline-started inputs by materializing a durable authoritative brief for the current mode.
- **FR-023**: Templates, examples, and mode guidance for the targeted modes MUST describe the same clarification lifecycle: seed a working brief, refine it through bounded questions, preserve immutable source inputs, and distinguish draft mode changes from governed carry-forward.
- **FR-024**: Validation evidence for this feature MUST cover all five targeted modes for working-brief clarification and MUST cover representative explicit refine or continue behavior across the broader non-targeted mode catalog, including at minimum `review`, `verification`, `implementation`, `refactor`, `incident`, and `migration`, plus the shared `resume` and `status` operator surfaces.
- **FR-025**: The feature MUST preserve current publish destinations, artifact families, and source-input honesty markers, and validation evidence MUST explicitly confirm that preservation unless an additive change is required to expose refinement state explicitly.

### Key Entities *(include if feature involves data)*

- **Draft Work Item**: The durable pre-run identity created when clarification starts, holding the working brief, source input snapshot references, clarification records, defaults, unresolved questions, and readiness delta without implying governed-run completion or approval state.
- **Governed Carry-Forward**: The post-run mode-change mechanism that preserves the original governed run and creates a linked successor work item in the corrected mode.
- **Working Brief**: The authoritative current-mode document Canon refines during clarification while leaving original inputs untouched.
- **Source Input Snapshot**: The immutable authored input surface that grounds the working brief and remains inspectable as evidence.
- **Clarification Record**: The durable log of questions asked, answers received, defaults applied, affected sections, and remaining unresolved items.
- **Readiness Delta**: The explicit summary of what still blocks or bounds the current governed work after clarification updates.
- **Carry-Forward Recommendation**: The structured recommendation that collected clarification context should move to another mode without losing continuity.
- **Run Refinement State**: The draft-scoped and later run-scoped representation of the authoritative working brief, supporting inputs, clarification history, and identity continuity for the current work item.
- **Successor Lineage Link**: The explicit relationship fields, including `carried_from` and `supersedes`, that connect a successor work item back to the draft or governed run it continues.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In validation scenarios for all five targeted modes, 100% of accepted clarification answers update the same authoritative working brief while leaving the original source inputs unchanged.
- **SC-002**: In validation scenarios where Canon surfaces a single likely candidate, 100% of refinements continue the same draft-to-run work identity only after an explicit continuation signal; fresh requests without that signal start new work, and no case silently mutates the surfaced candidate.
- **SC-003**: In validation scenarios where the correct mode changes before run start, 100% of draft work items switch mode in place without creating unnecessary successor churn.
- **SC-004**: In validation scenarios where the correct mode changes after run start, 100% of redirects preserve the original governed run and create a linked successor with explicit lineage.
- **SC-005**: In recorded validation walkthroughs, a reviewer can inspect a governed work item and identify the authoritative working brief, supporting inputs, clarification answers or defaults, and remaining readiness delta in under 2 minutes.
- **SC-006**: In ambiguous continuation-versus-new-work scenarios covered by the validation suite, Canon asks for disambiguation before mutating state or starting a new run in 100% of tested cases.

## Validation Plan *(mandatory)*

- **Structural validation**: contract checks for authoritative working brief selection, source-input immutability, clarification record shape, decision-changing question selection boundaries, continuation or refinement lifecycle state, preservation of publish destinations or artifact families or honesty markers, and docs or template alignment.
- **Logical validation**: focused scenarios for the five targeted modes, durable draft work item creation, draft-to-run promotion, in-place draft mode switching, single-candidate advisory detection versus explicit continuation intent, post-start governed carry-forward, ambiguity handling between continuation and new work, carry-forward when clarification changes the correct mode, and representative non-targeted continuity coverage for `review`, `verification`, `implementation`, `refactor`, `incident`, and `migration` plus the shared `resume` and `status` surfaces.
- **Operator walkthrough validation**: recorded reviewer walkthroughs for `status` and `inspect refinement` that prove SC-005 and capture the elapsed time, evidence path, and any ambiguity in locating the authoritative working brief.
- **Independent validation**: a separate review pass over lifecycle coherence, invariants, and cross-mode consistency after the implementation and documentation changes land.
- **Evidence artifacts**: `specs/062-clarify-run-refinement/validation-report.md`, mode-focused tests, lifecycle contract coverage, and updated templates or guidance demonstrating the working-brief flow.

## Decision Log *(mandatory)*

- **D-001**: Use a mode-specific working brief as the clarification target instead of mutating raw authored inputs, **Rationale**: this preserves Canon's evidence boundary while still allowing clarification answers and defaults to become durable, inspectable state.
- **D-002**: Create a durable draft work item when clarification starts and promote that same identity into governed run lineage when the run begins, **Rationale**: this preserves continuity across clarification and execution without pretending that pre-run draft state already carries approval or publish semantics.
- **D-003**: Treat a mode switch before run start as refinement and a mode switch after run start as governed carry-forward, **Rationale**: draft clarification should be able to correct direction without churn, while started governed runs must preserve lineage, approval semantics, and packet identity.

## Non-Goals

- Redesign every mode's question taxonomy in the same slice; only the five targeted modes receive first-class clarification-template behavior here.
- Mutate `canon-input/`, shared starter templates, or published packets in place as part of clarification.
- Remove explicit new-run behavior for truly new work.
- Collapse approvals, publish behavior, or unrelated artifact generation into the refinement feature.

## Assumptions

- The existing canonical templates for `requirements`, `discovery`, `system-shaping`, `architecture`, and `change` are sufficient starting points for creating authoritative working briefs.
- A folder-backed packet that contains `brief.md` remains the authoritative current-mode brief, while sibling files continue to act as supporting context.
- Existing inspect, status, and run lifecycle surfaces can be extended to expose refinement state without creating a parallel workflow product.
- Non-targeted modes can start with explicit continue or refine identity continuity before they receive deeper mode-specific clarification authoring behavior in later features.
- A mode switch before run start is refinement, while a mode switch after run start is governed carry-forward to a linked successor work item.
