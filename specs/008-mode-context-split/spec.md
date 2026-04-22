# Feature Specification: Mode Context Split

**Feature Branch**: `008-mode-context-split`  
**Created**: 2026-04-19  
**Status**: Draft  
**Input**: User description: "Refactor Canon mode semantics by renaming brownfield-change to change, introducing explicit system_context new|existing, updating runtime and public surfaces, and recovering validation coverage for the affected areas."

## Governance Context

**Mode**: architecture  
**Risk Classification**: systemic-impact because this feature changes Canon's public mode model, run context contract, CLI behavior, artifact namespaces, governed input bindings, documentation truth, and validation expectations across multiple shipped surfaces.  
**Scope In**:
- Replace `brownfield-change` with `change` as the public governed work type for bounded change planning
- Introduce explicit `system_context = new | existing` as a first-class runtime concept
- Apply the two-axis model consistently across run startup, persisted context, policy and gate inputs, emitted evidence, inspect surfaces, and public documentation
- Rename canonical authored-input and artifact namespaces from `brownfield-change` to `change`
- Expand automated validation so the touched runtime and CLI areas recover meaningful patch coverage instead of shipping with large untested branches

**Scope Out**:
- Redesigning the existing policy system beyond consuming the explicit context field
- Adding new governed modes beyond the requested rename to `change`
- Preserving backward compatibility for legacy `brownfield` or `greenfield` naming
- Redefining artifact schemas except where the rename or explicit system context makes a change unavoidable
- Adding implicit defaults for `system_context`

**Invariants**:

- Mode and system context MUST remain separate axes everywhere Canon persists, validates, or explains a run.
- Public Canon surfaces MUST stop exposing `brownfield` and `greenfield` terminology once this feature ships.
- `change` with `system_context = existing` MUST preserve the legacy bounded-change behavior that depended on preserved invariants, constrained change surface, readiness gates, and validation expectations.
- Modes that require `system_context` MUST fail fast when it is missing, and optional modes MUST not silently invent a value.
- Evidence, artifacts, and inspect flows MUST stay durable and coherent after the rename and context split.
- Validation and evidence review MUST remain distinct from generation so Canon does not collapse planning into unchecked output.

**Decision Traceability**: Decisions will be recorded in this specification, the follow-on plan and tasks artifacts for this feature, and the feature decision log. Validation evidence will be recorded through automated test and coverage outputs plus representative Canon runtime artifacts that show persisted context and renamed paths.

## User Scenarios & Testing

### User Story 1 - Start an Explicit Existing-System Change (Priority: P1)

A Canon operator wants to start a bounded change-planning run for an existing system using the new, semantically uniform mode model without losing the preserved-behavior safeguards that old brownfield runs enforced.

**Why this priority**: This is the core migration target. If `change + existing` is not trustworthy, the rename breaks the most important existing behavior instead of simplifying it.

**Independent Test**: Start a `change` run with `system_context = existing` against a valid change packet and verify that Canon accepts the run, persists the context, emits the renamed artifact namespace, and still enforces preserved-behavior and change-boundary expectations.

**Acceptance Scenarios**:

1. **Given** a bounded change packet for an existing system with preserved behavior and change-boundary inputs, **When** the user starts Canon in `change` mode with `system_context = existing`, **Then** Canon creates a run that preserves the same readiness and validation expectations previously enforced for `brownfield-change`.
2. **Given** a user tries to start `change` without `system_context`, **When** validation runs before execution, **Then** Canon rejects the request immediately with guidance that `system_context` is required for that mode.
3. **Given** a user tries to start `change` with `system_context = new`, **When** Canon validates the request, **Then** Canon rejects the combination with an explicit explanation that bounded change planning is defined only for existing systems in this release.

---

### User Story 2 - Use the Same Mode Model Across Context-Aware Workflows (Priority: P2)

A Canon operator wants all context-sensitive work types to expose the same two-axis model so they can understand mode selection without memorizing one-off legacy terms.

**Why this priority**: The refactor fails if only `change` is cleaned up while other modes still hide whether they apply to new or existing systems.

**Independent Test**: Exercise at least one required-context mode other than `change`, confirm it requires explicit `system_context`, and verify that optional-context modes continue to work without a forced value while still accepting an explicit value when supplied.

**Acceptance Scenarios**:

1. **Given** a user starts `architecture`, `system-shaping`, `implementation`, `refactor`, `migration`, or `incident` without `system_context`, **When** Canon validates the request, **Then** it rejects the request before run creation.
2. **Given** a user starts `discovery`, `requirements`, `review`, `verification`, or `pr-review` without `system_context`, **When** Canon validates the request, **Then** the run can proceed without Canon inventing a hidden context value.
3. **Given** a user supplies `system_context` for a run that accepts it, **When** the run is persisted and later inspected, **Then** the same explicit context value remains visible in the run record and downstream evidence surfaces.

---

### User Story 3 - Read Consistent Runtime and Documentation Surfaces (Priority: P3)

A maintainer wants Canon's documentation, canonical input paths, artifact paths, and inspectable outputs to describe the new model coherently so users never need to consult legacy `brownfield` or `greenfield` vocabulary.

**Why this priority**: The public model is only coherent if the runtime truth and the documentation truth change together.

**Independent Test**: Review the updated public help and documentation, verify the renamed input and artifact paths, and confirm that no public surface still instructs users to use `brownfield-change` or `greenfield` terminology.

**Acceptance Scenarios**:

1. **Given** a user reads the updated README or mode guide, **When** they need to choose between work type and system state, **Then** they can identify the two-axis model and the supported `new` versus `existing` combinations without consulting legacy terminology.
2. **Given** a user follows the canonical authored-input guidance for bounded change planning, **When** they prepare their packet, **Then** the documented location is `canon-input/change.md` or `canon-input/change/` rather than the removed legacy path.
3. **Given** a user inspects emitted artifacts or evidence for a change run, **When** they follow linked paths, **Then** the artifact namespace uses `change` and the persisted run context shows the explicit system context used for the run.

---

### User Story 4 - Trust the Refactor Through Regression Coverage (Priority: P4)

A maintainer reviewing this breaking change wants the validation suite to cover the new model deeply enough that Codecov-style patch regressions no longer show large uncovered branches in the touched runtime, gate, artifact, and CLI areas.

**Why this priority**: The requested refactor spans cross-cutting runtime behavior. Shipping it without materially stronger tests would make the semantic cleanup harder to trust than the current inconsistent model.

**Independent Test**: Run the automated validation suite and confirm it includes focused scenarios for mode rename, required-context validation, invalid combinations, persisted context, renamed paths, inspection outputs, and the preserved brownfield semantics now carried by `change + existing`.

**Acceptance Scenarios**:

1. **Given** the feature branch touches runtime, gate, artifact, and CLI surfaces, **When** automated validation completes, **Then** the changed branches introduced by the refactor are exercised by dedicated tests instead of relying only on broad smoke coverage.
2. **Given** the current patch coverage baseline is failing, **When** the feature is ready for review, **Then** the updated tests materially improve coverage for the touched surfaces and clear the agreed patch coverage threshold for the feature.

### Edge Cases

- What happens when a user provides `system_context` for an optional mode and later inspects or resumes that run?
- How does Canon respond when a user attempts the removed `brownfield-change` mode name or the legacy `canon-input/brownfield-change` path after the breaking rename?
- Which invariant is most likely to be stressed when `change + existing` must preserve legacy bounded-change behavior while every public path and label is being renamed?
- What happens when documentation or skills describe `system-shaping` as implicitly new-system only even though the new model makes context explicit?
- How does the system prevent context-aware policies or gates from reading a missing or invented `system_context` value during validation or evidence generation?

## Requirements

### Functional Requirements

- **FR-001**: Canon MUST define modes strictly as governed work types and MUST represent system state through a separate explicit `system_context` field.
- **FR-002**: Canon MUST remove `brownfield-change` from the public mode catalog and replace it with `change`.
- **FR-003**: Canon MUST reject public requests that still use `brownfield-change`, `brownfield`, or `greenfield` terminology in places where the new mode and context model is now authoritative.
- **FR-004**: Canon MUST expose `system_context` with exactly two supported values, `new` and `existing`, across run startup, persisted run state, and inspectable output surfaces.
- **FR-005**: Canon MUST require explicit `system_context` for `system-shaping`, `architecture`, `change`, `implementation`, `refactor`, `migration`, and `incident`.
- **FR-006**: Canon MUST allow `discovery`, `requirements`, `review`, `verification`, and `pr-review` to run without `system_context`, and MUST NOT invent a hidden default when the field is omitted.
- **FR-007**: When a mode requires `system_context` and the caller omits it, Canon MUST fail fast before run creation with corrective guidance.
- **FR-008**: Canon MUST persist the chosen `system_context` in the run context record and make it available to policies, gates, evidence generation, and artifact generation.
- **FR-009**: Canon MUST move the canonical authored-input location for bounded change planning to `canon-input/change.md` or `canon-input/change/`.
- **FR-010**: Canon MUST rename the bounded-change artifact namespace from `brownfield-change` to `change` everywhere artifacts are emitted, linked, or inspected.
- **FR-011**: `change` with `system_context = existing` MUST preserve the prior bounded-change semantics for preserved behavior, constrained change surface, validation expectations, and readiness gates.
- **FR-012**: `change` with `system_context = new` MUST be rejected explicitly in this release to keep the model minimal and to avoid inventing a second meaning for change planning.
- **FR-013**: Inspect, evidence, status, approval, and resume surfaces MUST continue to work with the renamed mode and MUST surface the explicit `system_context` value whenever it is part of the run record.
- **FR-014**: README, MODE_GUIDE, and NEXT_FEATURES MUST explain Canon as a two-axis model where mode describes the work type and `system_context` describes whether the target system is new or existing.
- **FR-015**: README and MODE_GUIDE MUST stop describing `system-shaping` as implicitly new-system work and MUST clarify that `change` is primarily for existing systems.
- **FR-016**: Public documentation and help output MUST contain no remaining references to `brownfield` or `greenfield` terminology after the refactor ships.
- **FR-017**: Automated validation MUST include focused regression scenarios for required-context enforcement, invalid mode and context combinations, persisted context visibility, renamed input paths, renamed artifact paths, and preserved `change + existing` behavior.
- **FR-018**: Automated validation for this feature MUST materially increase coverage for the touched runtime, gate, artifact, and CLI areas relative to the failing baseline that motivated the request.
- **FR-019**: The feature MUST add short technical comments at the semantic split points explaining why work type and system context are separate, why `brownfield` was removed, and why `greenfield` is intentionally absent from the public API.
- **FR-020**: Validation evidence for this feature MUST include a coverage report plus a concise change summary and modified-file inventory so reviewers can audit both semantic correctness and test depth.

### Key Entities

- **Mode**: The governed work type Canon is performing, such as `discovery`, `architecture`, `change`, or `review`.
- **System Context**: The explicit run attribute that states whether Canon is reasoning about a `new` or `existing` system.
- **Run Context Record**: The durable run metadata that binds mode, risk, zone, ownership, system context, and follow-up state into one inspectable record.
- **Change Packet**: The authored input and emitted artifact set for bounded change planning, including preserved behavior, change boundaries, and validation expectations.
- **Canonical Input Binding**: The authoritative user-facing location Canon recognizes for a given mode's authored packet.
- **Artifact Namespace**: The user-visible folder and link structure Canon uses when it persists artifacts for a run.
- **Validation Evidence Bundle**: The combined automated and runtime evidence reviewers use to confirm the renamed model is correct and sufficiently tested.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In validation scenarios for all required-context modes, 100% of requests missing `system_context` are rejected before run creation.
- **SC-002**: In validation scenarios for all optional-context modes, 100% of requests without `system_context` complete without Canon inventing a hidden context value.
- **SC-003**: Across representative change runs for existing systems, 100% of sampled runs preserve the bounded-change safeguards previously associated with `brownfield-change` while emitting the renamed `change` paths.
- **SC-004**: Public documentation, help output, canonical input guidance, and inspectable artifact paths contain zero remaining references to `brownfield` or `greenfield` terminology when the feature is ready for review.
- **SC-005**: The automated validation suite covers mode rename behavior, required and optional context rules, invalid combinations, persisted context visibility, and renamed path behavior, and the touched patch reaches at least 85% line coverage.
- **SC-006**: In a documentation review using the published matrix, reviewers can correctly classify whether `system-shaping`, `architecture`, and `change` apply to `new` and `existing` systems without consulting historical naming.

## Validation Plan

- **Structural validation**: Validate mode and context vocabulary across public help, canonical input guidance, artifact namespaces, persisted run context, policy and gate inputs, README, MODE_GUIDE, NEXT_FEATURES, and skill-facing runtime guidance.
- **Logical validation**: Execute automated scenarios for `change + existing`, rejected `change + new`, missing required `system_context`, optional-context modes without defaults, persisted context visibility, renamed input and artifact paths, and removal of legacy public terms.
- **Independent validation**: Run a separate review focused on semantic coherence of the two-axis model and a separate review of the coverage evidence for the touched runtime and CLI surfaces.
- **Evidence artifacts**: Preserve automated test results, coverage outputs, representative run context artifacts showing persisted `system_context`, and a concise modified-file inventory with the feature validation record.

## Decision Log

- **D-001**: Canon will separate governed work type from system state, **Rationale**: mixing those concerns in one mode label is the root semantic inconsistency this feature is fixing.
- **D-002**: `brownfield-change` will be replaced by `change` and `brownfield` and `greenfield` will be removed from the public API, **Rationale**: the new model should use precise, boring naming instead of overloaded historical jargon.
- **D-003**: `change + new` is explicitly disallowed in the first version of the split model, **Rationale**: the feature goal is semantic clarity, and allowing a second interpretation of `change` would reintroduce ambiguity immediately.
- **D-004**: Documentation, tests, and coverage recovery ship in the same feature slice as the semantic refactor, **Rationale**: the change is too cross-cutting to leave public guidance or validation depth behind.

## Non-Goals

- Redesigning the policy system
- Introducing new governed modes beyond renaming `brownfield-change` to `change`
- Preserving backward compatibility for legacy naming or paths
- Changing artifact schemas unless the rename or explicit context makes it unavoidable
- Adding implicit defaults for `system_context`
- Expanding `change` into a fully new-system planning workflow in this release

## Assumptions

- This feature is allowed to introduce breaking changes across Canon's public mode model and path conventions.
- Existing bounded-change validation and readiness logic is strong enough to be preserved under `change + existing` rather than redesigned.
- Policies, gates, evidence generation, and artifact generation can consume an explicit persisted system context without a separate policy architecture rewrite.
- README, MODE_GUIDE, NEXT_FEATURES, and skill-facing runtime guidance are the authoritative public documentation surfaces that must change in the same delivery tranche.
- The currently failing patch coverage signal is treated as a release-level quality concern for this feature, so additional targeted tests are part of the required scope rather than optional cleanup.
