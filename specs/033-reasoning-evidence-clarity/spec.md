# Feature Specification: Cross-Mode Reasoning Evidence And Clarity Expansion

**Feature Branch**: `033-reasoning-evidence-clarity`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Implement roadmap feature 033 end-to-end without splitting into slices: make cross-mode reasoning evidence and clarity expansion first-class across Canon runtime, contracts, gates, summarizers, renderers, and mode flows; extend inspect clarity and reasoning signals across the governed modes; eliminate or tighten placeholder-heavy packet fallbacks that still read like template compilation; update version, all impacted docs, templates, mirrored skills, examples, roadmap continuity, and changelog; require coverage for all modified or created Rust files; fix clippy issues and run cargo fmt as part of completion."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: systemic-impact because this feature changes shared runtime behavior across most governed modes, including clarity inspection, reasoning-signal reporting, summary posture, gate-visible packet honesty, renderer fallback behavior, skill guidance, release-facing docs, and validation expectations. It does not add a new mode or persistence schema, but it changes how Canon judges and presents quality across the existing mode surface.  
**Scope In**:

- Expand `inspect clarity` and `reasoning_signals` from the current limited set into the file-backed governed mode families that still lack first-class clarity inspection.
- Introduce a shared reasoning-evidence posture for packets and summaries so Canon can distinguish structurally complete packets from packets that are still shallow, contradictory, or weakly supported.
- Tighten or replace placeholder-heavy fallback packet sections that currently risk looking like real reasoning when the authored input is weak or missing.
- Preserve and strengthen honest output posture for review-family modes, including explicit contradictions, missing evidence, unresolved findings, and explicit no-issue statements when no contradiction exists.
- Synchronize embedded skills, mirrored `.agents/skills/`, templates, examples, roadmap text, impacted docs, and `CHANGELOG.md` with the same reasoning-evidence contract.
- Bump repository version surfaces to `0.33.0`, include an explicit docs plus changelog sweep, and require focused validation coverage for all modified or created Rust files together with `cargo clippy` and `cargo fmt` closeout.

**Scope Out**:

- Introducing a new governed runtime mode, adapter kind, persistence layout, approval target, or publish destination.
- Adding network-backed evidence gathering, authenticated external integrations, or non-local mutation capabilities.
- Rewriting unaffected modes only to normalize wording or persona language with no reasoning-contract change.
- Splitting this work into separate roadmap slices for runtime and authoring surfaces; this feature ships as one end-to-end change set.

**Invariants**:

- Canon MUST preserve or strengthen explicit honesty markers such as `## Missing Authored Body`, `## Missing Evidence`, blocked-gate posture, unsupported verdicts, and unresolved-findings signals; this feature MUST NOT replace them with softer prose.
- Canon MUST NOT fabricate contradictions, alternatives, or tradeoffs when the authored surface makes a decision materially closed or when the evidence basis is insufficient.
- Existing run identity, `.canon/` storage, publish semantics, approval flows, and recommendation-only posture MUST remain unchanged.
- Modified or newly created Rust files in the affected runtime paths MUST receive focused automated validation coverage before the feature is complete.

**Decision Traceability**: Decisions start in this specification and continue in `specs/033-reasoning-evidence-clarity/decision-log.md`, with validation evidence recorded in `specs/033-reasoning-evidence-clarity/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Inspect Clarity Across Governed Modes (Priority: P1)

As a Canon maintainer preparing a run, I want clarity inspection and reasoning
signals to exist across the relevant file-backed governed modes so I can see
missing context, weak reasoning posture, and required clarifications before a
packet enters the downstream system of record.

**Why this priority**: If pre-run clarity remains partial, Canon can still
produce polished but weak packets in modes that lack the current inspection
surface.

**Independent Test**: For one representative authored brief in each targeted
behavior family, a maintainer can run clarity inspection and receive a bounded
summary, missing-context findings when applicable, targeted clarification
questions when applicable, and explicit reasoning signals that describe what
the authored surface actually proves.

**Acceptance Scenarios**:

1. **Given** a file-backed mode that currently lacks clarity inspection,
   **When** the maintainer runs the new clarity path, **Then** Canon returns a
   mode-appropriate summary plus explicit reasoning signals instead of an
   unsupported-target error.
2. **Given** an authored brief with structurally present headings but weak
   evidence or shallow reasoning, **When** the maintainer inspects clarity,
   **Then** the result names the missing context or weak reasoning posture
   rather than reporting the brief as effectively complete.
3. **Given** a brief whose decision is already materially closed,
   **When** the maintainer inspects clarity, **Then** the reasoning signals say
   the choice is already bounded instead of implying that Canon found multiple
   viable alternatives.

---

### User Story 2 - Read Honest Reasoning In Emitted Packets (Priority: P2)

As a reviewer reading Canon outputs, I want packet summaries and artifacts to
show explicit reasoning evidence, contradiction handling, tradeoffs, missing
evidence, or materially-closed decision posture so a polished packet cannot
masquerade as a well-reasoned one.

**Why this priority**: This is the core product problem. A packet that sounds
strong without proving any reasoning is more dangerous than an obviously
incomplete packet.

**Independent Test**: With representative inputs for planning, review, and
analysis families, Canon emits packets and summaries that either preserve real
reasoning evidence or expose honest gap markers and downgraded posture instead
of placeholder-heavy or generic filler.

**Acceptance Scenarios**:

1. **Given** a targeted packet whose authored body is structurally complete but
   weak on contradictions, tradeoffs, or evidence grounding, **When** Canon
   emits the packet, **Then** the packet summary and next-step posture make the
   weakness explicit instead of presenting the packet as strongly ready.
2. **Given** review-family material where no contradiction is actually present,
   **When** Canon emits critique artifacts, **Then** it states that no direct
   contradiction was identified instead of fabricating one to satisfy a shape.
3. **Given** an affected fallback-heavy packet surface such as planning output
   with missing authored sections, **When** Canon emits the artifact, **Then**
   the output uses honest missing-body or closure language rather than generic
   placeholder prose that reads like authored reasoning.

---

### User Story 3 - Ship 0.33.0 With Synchronized Authoring Surfaces And Validation (Priority: P3)

As a maintainer shipping the feature, I want version surfaces, docs,
templates, mirrored skills, changelog entries, and validation records to align
with the new reasoning contract so the runtime fix and the authoring UX do not
drift apart.

**Why this priority**: In this repository, docs, templates, skill mirrors, and
validation are contract surfaces. The feature is incomplete if the runtime is
fixed but the guidance still invites template-filling behavior.

**Independent Test**: A maintainer can inspect the generated tasks and final
validation evidence and confirm explicit work exists for the `0.33.0` version
bump, the impacted-docs plus changelog sweep, coverage for modified or new
Rust files, `cargo clippy`, and `cargo fmt`, and that the user-facing
authoring surfaces describe the same reasoning-evidence posture as the engine.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects
   Cargo manifests, compatibility references, impacted docs, templates, skill
   mirrors, and `CHANGELOG.md`, **Then** they all describe `0.33.0` and the new
   reasoning-evidence posture consistently.
2. **Given** the generated task plan, **When** a maintainer reviews it,
   **Then** it includes explicit tasks for version bump, impacted docs plus
   changelog, coverage of modified or new Rust files, `cargo clippy`, and
   `cargo fmt`.
3. **Given** the completed implementation, **When** a maintainer reviews the
   validation evidence, **Then** the touched runtime paths have focused
   automated coverage and the overall closeout is clean on formatting and lint
   expectations.

### Edge Cases

- A packet includes all required headings but almost no substantive reasoning; Canon must mark the shallow posture explicitly instead of treating heading presence as sufficient strength.
- A decision surface is already materially closed to one viable option; Canon must say that directly instead of inventing rejected alternatives.
- Review or verification material genuinely has no contradiction; Canon must preserve an explicit “none found” path rather than manufacturing adversarial findings.
- Backlog or other fallback-heavy packet surfaces still lack authored decomposition details; Canon must expose the gap honestly without emitting generic slices that look user-approved.
- Shared renderer or summarizer helpers affect targeted and non-targeted modes; the feature must not create regressions outside the intended reasoning-contract change.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST extend `inspect clarity` and `reasoning_signals` beyond `requirements`, `discovery`, and `supply-chain-analysis` to the remaining in-scope file-backed governed modes.
- **FR-002**: For every in-scope clarity target, the system MUST emit a mode-appropriate summary, explicit missing-context findings when present, targeted clarification questions when present, and reasoning signals that describe what the authored input actually supports.
- **FR-003**: Reasoning signals MUST distinguish at least three states when relevant: grounded reasoning evidence present, weak or missing reasoning evidence present, and decision materially closed.
- **FR-004**: Packet summaries, result posture, or next-step guidance for targeted modes MUST NOT treat structural heading completeness as sufficient evidence of strong reasoning quality.
- **FR-005**: Planning, design, change, execution, and analysis packet families that claim reasoning value MUST preserve or surface explicit tradeoffs, rejected-option rationale, missing evidence, open questions, closure findings, or materially-closed decision statements where the mode contract expects them.
- **FR-006**: Review-family outputs, including `review`, `verification`, and `pr-review`, MUST preserve contradiction, missing-evidence, unresolved-findings, and explicit no-contradiction posture honestly instead of normalizing every output into a findings-shaped artifact.
- **FR-007**: Placeholder-heavy fallback packet sections in affected modes MUST be tightened, removed, or converted into explicit missing-body, closure, or gap language that cannot be mistaken for authored reasoning.
- **FR-008**: Existing `## Missing Authored Body`, `## Missing Evidence`, blocked-gate, unsupported-verdict, and unresolved-findings semantics MUST remain intact or become stricter; they MUST NOT be weakened by this feature.
- **FR-009**: Embedded skill sources and mirrored `.agents/skills/` copies for impacted modes MUST document the same reasoning-evidence, clarity, and honesty posture as the runtime behavior.
- **FR-010**: `docs/templates/canon-input/`, `docs/examples/canon-input/`, and impacted guidance docs MUST demonstrate the same reasoning-evidence and honest-gap handling expected by the runtime.
- **FR-011**: `ROADMAP.md`, `README.md`, `docs/guides/modes.md`, other impacted docs, and `CHANGELOG.md` MUST describe the delivered feature consistently with the final runtime behavior.
- **FR-012**: Cargo manifests, lockfile surfaces, shared compatibility references, and other release-facing version anchors MUST report `0.33.0` consistently for this feature.
- **FR-013**: The task plan for this feature MUST include an explicit version-bump task, an explicit impacted-docs-and-changelog task, an explicit coverage task for modified or created Rust files, and explicit `cargo clippy` plus `cargo fmt` tasks.
- **FR-014**: Modified or newly created Rust files in the affected runtime paths MUST receive focused automated validation coverage before the feature is complete.
- **FR-015**: Final structural validation for this feature MUST include clean `cargo fmt` and `cargo clippy` results.
- **FR-016**: Existing `.canon/` persistence, run identity, publish destinations, approval targets, and recommendation-only posture MUST remain unchanged.

### Key Entities *(include if feature involves data)*

- **Clarity Inspect Summary**: The pre-run inspection result containing mode summary, missing context, clarification questions, reasoning signals, and recommended focus.
- **Reasoning Signal**: An inspectable statement describing what the authored surface proves, what it does not prove, whether reasoning evidence is shallow, and whether a decision is already materially closed.
- **Reasoning Evidence Posture**: The packet-level assessment that distinguishes grounded reasoning from structurally complete but shallow output.
- **Honesty Marker**: An explicit signal such as `## Missing Authored Body`, `## Missing Evidence`, blocked posture, unsupported verdict, unresolved findings, or other explicit gap language that keeps Canon from inventing confidence.
- **Fallback Packet Surface**: A rendered artifact section that currently relies on generic placeholder prose when authored reasoning is missing or weak.
- **Mode Family Contract**: The combination of runtime behavior, artifact shape, skill guidance, template surface, and validation expectations for a governed mode or behavior family.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every in-scope file-backed governed mode supports clarity inspection and returns non-empty `reasoning_signals` for a representative authored input.
- **SC-002**: Each targeted behavior family has focused positive and negative automated validation proving that real reasoning evidence is preserved when present and honest gap posture appears when it is absent or weak.
- **SC-003**: No affected packet artifact relies on generic placeholder prose that could plausibly be mistaken for authored reasoning when required authored content is missing.
- **SC-004**: A maintainer can inspect representative emitted packets and identify either explicit reasoning evidence or explicit reasoned incompleteness without reopening chat history.
- **SC-005**: Release-facing version references, impacted docs, skill mirrors, templates, examples, roadmap continuity, and `CHANGELOG.md` all align to `0.33.0` and the delivered reasoning contract.
- **SC-006**: All modified or newly created Rust files in the slice are exercised by focused automated validation, and the final validation record is clean on formatting and lint expectations.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, repo-local skill validation, and consistency checks for spec, plan, and tasks artifacts.
- **Logical validation**: Focused tests for clarity inspection, renderer fallback behavior, packet summarizers, gate or readiness posture, and representative docs or skill-sync checks for every affected behavior family.
- **Independent validation**: Review of the spec, plan, and tasks before implementation, followed by an adversarial pass on representative packets to confirm that Canon neither fabricates reasoning nor hides weak support behind polished prose.
- **Evidence artifacts**: Validation results, coverage notes, and closeout findings recorded in `specs/033-reasoning-evidence-clarity/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Treat runtime behavior and authoring-surface synchronization as one feature rather than separate roadmap slices, **Rationale**: the user explicitly wants feature 033 delivered end-to-end, and the runtime contract is incomplete if prompts and templates drift.
- **D-002**: Reuse and extend the repository's existing honesty signals and review-family contradiction handling instead of inventing a parallel reasoning-quality mechanism, **Rationale**: Canon already has explicit blocked and unsupported semantics that are safer than a new soft-scoring vocabulary.
- **D-003**: Treat version bump, impacted docs plus changelog, coverage of modified or created Rust files, `cargo clippy`, and `cargo fmt` as first-class delivery work, **Rationale**: release-facing drift and validation gaps are already known contract risks in this repository.

## Non-Goals

- Introducing a new Canon mode, adapter class, or execution capability.
- Changing `.canon/` runtime persistence, publish destinations, approval semantics, or recommendation-only posture.
- Adding network-backed evidence collection or third-party integration work.
- Rewriting unaffected packet families purely for prose style or persona polish with no reasoning-contract impact.

## Assumptions

- The shared contract, renderer, summarizer, and gate architecture is flexible enough to carry reasoning-evidence posture across multiple modes without a new persistence schema.
- File-backed governed modes can share a common clarity-inspection pattern even if `pr-review` remains diff-backed and uses a different pre-run shape.
- `0.33.0` is the intended repository version for this feature, so versioned docs, compatibility references, and changelog updates are in scope.
- Focused automated validation plus coverage evidence for modified or created Rust files is sufficient to prove the affected runtime paths rather than requiring full branch-wide coverage parity.
