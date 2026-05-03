# Feature Specification: Governance Runtime Framing

**Feature Branch**: `040-governance-runtime-framing`  
**Created**: 2026-05-03  
**Status**: Draft  
**Input**: User description: "Reframe Canon as the governed packet runtime for AI-assisted engineering, tighten the boundary between human CLI and machine-facing governance adapter, add a dedicated governance adapter integration guide with request/response and lifecycle examples, align README and guides with the runtime positioning, document orchestrator-facing usage without turning Canon into a generic agent framework, and deliver the repo updates end-to-end including version bump, changelog, roadmap cleanup, tests, coverage, clippy, and formatting."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact, because this feature changes the public framing, release surface, and integration-facing guidance for Canon without intentionally changing the governed runtime semantics themselves.  
**Scope In**: README framing, getting-started and mode guidance, one dedicated governance adapter integration guide, version surfaces for the next release line, changelog and roadmap alignment, and documentation or test guardrails that keep those surfaces coherent.  
**Scope Out**: changing the governance adapter JSON schema, adding new runtime modes, introducing Synod-specific orchestration stages into Canon core docs, or changing approval semantics, packet persistence, or publish behavior.

**Invariants**:

- Canon MUST remain explicitly local-first, file-backed, and approval-aware rather than being described as a generic agent framework or opaque agent loop.
- The human CLI surface and the machine-facing governance adapter MUST continue to describe the same governed runtime rather than being documented as separate systems.
- The feature MUST preserve the existing governance adapter lifecycle and JSON contract semantics unless a change is explicitly specified elsewhere.

**Decision Traceability**: Design and framing decisions for this feature will be recorded in `specs/040-governance-runtime-framing/decision-log.md`, with release-facing alignment reflected in `CHANGELOG.md` and the delivered-roadmap closeout in `ROADMAP.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Clarify Canon's Product Identity (Priority: P1)

As a repository reader evaluating Canon for the first time, I want the opening docs to state clearly what Canon is, what it is not, and the simplest happy path, so I can understand the product without conflating it with a generic agent framework.

**Why this priority**: If Canon's identity is fuzzy at the entrypoint, every downstream guide inherits that ambiguity and orchestrator users will misclassify the product before they ever reach the adapter docs.

**Independent Test**: Read only the opening README and getting-started surfaces and confirm that a first-time reader can identify Canon as a governed packet runtime, can tell that it is not a generic agent framework, and can follow the human-driven happy path without consulting external chat context.

**Acceptance Scenarios**:

1. **Given** a first-time reader opens the repository, **When** they read the opening README framing, **Then** they can identify Canon as a governed, local-first runtime for AI-assisted engineering and see explicit non-goals around generic agent-framework positioning.
2. **Given** a human operator wants the shortest correct path, **When** they read the getting-started flow, **Then** they see a coherent sequence for `init`, `inspect clarity`, `run`, `status`, and `publish` without needing to infer the control story.

---

### User Story 2 - Document The Governance Adapter As The Machine Boundary (Priority: P2)

As an orchestrator or integration maintainer, I want a dedicated governance adapter guide with stable request or response examples, lifecycle states, and usage rules, so I can integrate Canon without scraping human CLI prose or treating Canon as the orchestrator itself.

**Why this priority**: The machine-facing adapter is the main integration boundary for external systems, but today its framing is spread across README and mode docs rather than one explicit integration surface.

**Independent Test**: Read the dedicated governance adapter guide and confirm that an external tool maintainer can distinguish `capabilities`, `start`, and `refresh`, can identify `status`, `approval_state`, `packet_readiness`, and `reason_code`, can find representative machine-facing examples for file-backed modes, and can see the current explicit boundary note for `pr-review`.

**Acceptance Scenarios**:

1. **Given** an orchestrator maintainer wants to drive Canon programmatically, **When** they read the adapter guide, **Then** they see the adapter commands, the stable response fields, and representative examples for multiple governed modes.
2. **Given** a reader is unsure whether Canon or an external orchestrator owns sequencing, **When** they read the human-vs-machine boundary guidance, **Then** they see that Canon is the governed runtime and not the higher-level orchestration system.

---

### User Story 3 - Ship A Coherent Public Release Surface (Priority: P3)

As a release maintainer, I want version surfaces, changelog, roadmap, and guardrail tests to move together with the new framing, so the next release reads as one coherent feature instead of a docs-only drift.

**Why this priority**: Public framing changes lose credibility when version, changelog, roadmap, and validation evidence remain on the previous release line or contradict each other.

**Independent Test**: Inspect the release-facing files for the new delivery line and run the required validation suite to confirm that the repo advertises one coherent post-feature state, including roadmap closeout and documentation guardrails.

**Acceptance Scenarios**:

1. **Given** the feature is implemented, **When** the release maintainer inspects version, changelog, and roadmap surfaces, **Then** they all reflect the same delivered `040` macrofeature and leave no stale roadmap entries behind.
2. **Given** future edits drift the README or governance adapter guidance, **When** the documentation guardrail tests run, **Then** they fail with actionable evidence instead of letting the framing silently diverge.

### Edge Cases

- What happens when the repository points to GitHub Releases as the canonical binary source but public release pages are not yet visibly populated at the moment a reader lands on the repo?
- How does the system handle future docs changes that accidentally describe `canon governance` as a separate orchestration product rather than the machine-facing surface of the same runtime?
- Which invariant is most likely to be stressed when a downstream orchestrator example starts leaking Synod-specific workflow assumptions into Canon core docs?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The opening README framing MUST describe Canon as a governed, local-first runtime for AI-assisted engineering work rather than as a generic agent framework.
- **FR-002**: The public framing MUST state explicit non-goals that prevent Canon from being described as an opaque agent loop or higher-level orchestrator.
- **FR-003**: The README and getting-started guidance MUST keep one short human-driven happy path that includes pre-run clarity inspection and the run-to-publish control story.
- **FR-004**: The documentation set MUST distinguish the human CLI surface from the machine-facing governance adapter without implying that they are separate runtimes.
- **FR-005**: The repository MUST include a dedicated governance adapter integration guide that documents `canon governance capabilities --json`, `canon governance start --json`, and `canon governance refresh --json`.
- **FR-006**: The governance adapter guide MUST describe the stable machine-facing fields `status`, `approval_state`, `packet_readiness`, `reason_code`, and canonical workspace-relative refs.
- **FR-007**: The governance adapter guide MUST include representative request or response examples for at least `change`, `implementation`, and `verification`, plus an explicit current-boundary note for `pr-review` until the adapter exposes diff-ref binding directly.
- **FR-008**: Core Canon docs MUST avoid embedding Synod-specific stage mapping or other orchestrator-specific sequencing into the Canon product definition.
- **FR-009**: The next workspace release line MUST be bumped explicitly and aligned across version-bearing repository surfaces.
- **FR-010**: The changelog MUST record the delivered `040` feature with a concise description of the runtime-framing and governance-adapter documentation work.
- **FR-011**: The roadmap MUST be cleaned so it reflects that no active roadmap entries remain after the delivered `040` feature.
- **FR-012**: Documentation or guardrail tests MUST be updated or added so future drift in the README and governance-adapter framing is caught automatically.
- **FR-013**: All touched Rust files MUST receive focused coverage validation as part of implementation closeout.
- **FR-014**: End-to-end implementation closeout MUST explicitly include `cargo fmt`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and recorded validation evidence.

### Key Entities *(include if feature involves data)*

- **Runtime Positioning Surface**: The set of user-facing documents that define what Canon is, what it is not, and how a human should approach the product.
- **Governance Adapter Guide**: The machine-facing integration document that explains adapter commands, lifecycle states, stable response fields, and representative examples.
- **Release Alignment Surface**: The version, changelog, and roadmap artifacts that signal whether the repository represents one coherent delivered release line.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A first-time reader can determine from the opening README and getting-started surfaces, without consulting external conversation context, that Canon is a governed packet runtime and not a generic agent framework.
- **SC-002**: A tool maintainer can identify from one dedicated integration guide the three governance adapter commands, the stable machine-facing fields they can rely on, and the rule for when to use `canon governance` instead of the human CLI.
- **SC-003**: The delivered repository state exposes one aligned `0.40.0` release surface across workspace version files, README delivery line, changelog, and roadmap.
- **SC-004**: Documentation guardrail tests and validation commands pass and provide durable evidence that the framing and adapter docs remain synchronized after implementation.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, markdown link and contract guardrail tests, and focused repository consistency checks for the new docs surfaces.
- **Logical validation**: Human-driven doc walkthroughs for the README happy path and governance adapter guide, plus focused automated tests that assert the new framing and integration docs stay synchronized.
- **Independent validation**: A separate review pass over the final README, getting-started guide, mode guide, and governance adapter guide against the feature spec and constitution invariants.
- **Evidence artifacts**: `specs/040-governance-runtime-framing/validation-report.md`, updated `CHANGELOG.md`, relevant test output, and any focused coverage measurements for touched Rust files.

## Decision Log *(mandatory)*

- **D-001**: Canon will be framed as the governed packet runtime for AI-assisted engineering rather than the universal product entrypoint across all higher-level systems, **Rationale**: this preserves a clean boundary between Canon's governance role and any external orchestrator such as Synod.

## Non-Goals

- Change the governance adapter JSON schema, approval states, or run lifecycle semantics.
- Add Synod-specific workflow staging, orchestration policy, or runtime ownership to Canon core docs.
- Expand the roadmap with new follow-on slices as part of this delivery.

## Assumptions

- Existing governance adapter behavior is already correct enough that this feature can document and stabilize the surface without adding new runtime semantics.
- The next delivered workspace version should be `0.40.0` to align the `040` macrofeature across release-facing surfaces.
- Canon's current README, getting-started guide, and mode guide remain the right public entry docs for human operators.
- The repository already has or can sustain Rust-based documentation guardrail tests for README and guide synchronization.
