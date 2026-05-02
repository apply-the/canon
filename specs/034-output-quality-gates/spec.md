# Feature Specification: Output Quality Gates

**Feature Branch**: `034-output-quality-gates`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: User description: "Procedi con speckit per fare speckit-spec, poi speckit-plan, poi speckit-tasks e infine speckit-implements. Lavora su tutta la feature 034, non splittarla in slices. Voglio top output quality. Al solito, un task per fare bump della versione e uno per aggiornare tutte le docs impattate e il changelog. Infine coverage dei file rust modificati o creati e soluzione di problemi su clippy e esecuzione di cargo fmt. Poi pulisci la roadmap. Infine dammi il commit message."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: systemic-impact because this feature changes shared runtime quality posture across inspect surfaces, summary posture, authored-packet rendering, release-facing guidance, and validation expectations for multiple governed modes. It does not add a new mode or storage schema, but it changes how Canon distinguishes merely complete output from output that is materially useful or publication-ready.  
**Scope In**:

- Introduce a shared output-quality assessment that can distinguish structurally complete packets from materially useful and publishable packets.
- Surface that assessment through `inspect` results, runtime summaries, packet artifacts, and mode-facing guidance where Canon currently risks sounding stronger than the authored evidence supports.
- Tighten or replace fallback and summary behavior that still lets shallow, placeholder-heavy, or minimally evidenced packets read like strong outputs.
- Synchronize skill mirrors, templates, examples, README, guides, roadmap, changelog, and release-facing version anchors for the `0.34.0` delivery.
- Require explicit version bump, impacted-docs plus changelog, coverage for modified or created Rust files, `cargo clippy`, and `cargo fmt` as first-class delivery work.

**Scope Out**:

- Introducing a new governed mode, adapter kind, approval target, or publish destination.
- Changing `.canon/` persistence layout, run identity, or recommendation-only operating posture.
- Reopening already delivered reasoning-evidence mode expansion just to rename concepts without improving quality posture.
- Splitting this feature into separate runtime, docs, or authoring sub-slices; it ships as one end-to-end delivery.

**Invariants**:

- Canon MUST preserve explicit honesty markers such as `## Missing Authored Body`, `## Missing Evidence`, blocked posture, unsupported posture, and unresolved findings rather than softening them into optimistic prose.
- Canon MUST NOT let structural heading presence alone imply that a packet is materially useful or publishable.
- Canon MUST NOT fabricate tradeoffs, contradictions, or publishability evidence when the authored input remains shallow or materially closed.
- Existing `.canon/` storage, publish destinations, approval flows, and recommendation-only semantics MUST remain unchanged.
- Modified or newly created Rust files in the affected engine paths MUST receive focused automated coverage before the feature is complete.

**Decision Traceability**: Decisions for this feature start in this specification and continue in `specs/034-output-quality-gates/decision-log.md`, with validation and coverage evidence recorded in `specs/034-output-quality-gates/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Inspect Output Quality Before Trusting A Packet (Priority: P1)

As a Canon maintainer, I want a shared output-quality assessment visible before
or alongside generated packets so I can see whether a result is only
structurally complete, materially useful, or actually publishable without
reading every section manually.

**Why this priority**: If Canon cannot make this distinction explicit, the
system can still produce polished packets that look trustworthy while carrying
weak reasoning or placeholder evidence.

**Independent Test**: For representative authored inputs, a maintainer can run
the relevant inspect surface and receive an explicit quality posture plus named
evidence gaps or downgrade reasons that explain why the packet is not yet
publishable.

**Acceptance Scenarios**:

1. **Given** a brief with all required headings but thin reasoning content,
   **When** the maintainer inspects output quality, **Then** Canon reports that
   the packet is structurally complete but not materially useful or publishable.
2. **Given** a brief with concrete evidence, explicit tradeoffs, and no missing
   authored sections, **When** the maintainer inspects output quality,
   **Then** Canon reports a stronger posture with the evidence basis named.
3. **Given** a materially closed decision with one viable path,
   **When** the maintainer inspects output quality, **Then** Canon explains that
   closure is explicit without inventing additional alternatives.

---

### User Story 2 - Read Honest Quality Posture In Summaries And Artifacts (Priority: P2)

As a reviewer reading Canon outputs, I want summaries and packet artifacts to
state when a result is only structurally complete, when it is materially
useful, and when it is publishable, so polished filler cannot masquerade as a
high-quality packet.

**Why this priority**: The core product risk is not missing text but false
confidence. A strong-looking packet with weak substance is worse than an
obviously incomplete one.

**Independent Test**: With representative packets across affected mode
families, Canon emits summary headlines and artifact language that downgrade
shallow outputs, preserve honesty markers, and only present publishable posture
when the authored evidence actually supports it.

**Acceptance Scenarios**:

1. **Given** a packet family with placeholder-like or low-information sections,
   **When** Canon renders the summary and artifacts, **Then** the output names
   the quality gap instead of presenting the packet as ready.
2. **Given** a packet with explicit evidence, tradeoffs, and closure findings,
   **When** Canon renders the summary and artifacts, **Then** the output can say
   the packet is materially useful or publishable without generic caveats.
3. **Given** a fallback-heavy packet surface,
   **When** authored content is missing or weak, **Then** Canon uses explicit
   missing-body or downgrade language rather than synthetic planning prose.

---

### User Story 3 - Ship 0.34.0 With Synchronized Authoring Surfaces And Clean Roadmap (Priority: P3)

As a maintainer shipping the feature, I want the runtime contract, docs, skill
mirrors, version anchors, changelog, and roadmap to align with the delivered
quality posture so the repository presents one coherent `0.34.0` story.

**Why this priority**: In this repo, docs and skill mirrors are part of the
product contract. Output-quality improvements are incomplete if the authoring
surfaces and roadmap still describe the old posture.

**Independent Test**: A maintainer can inspect the generated tasks and final
validation report and confirm explicit work exists for version bump, impacted
docs plus changelog, roadmap cleanup, Rust coverage for touched files,
`cargo clippy`, and `cargo fmt`, and that the release-facing surfaces all align
to `0.34.0`.

**Acceptance Scenarios**:

1. **Given** the completed feature branch, **When** a maintainer inspects
   manifest versions, shared compatibility references, docs, skills, roadmap,
   and `CHANGELOG.md`, **Then** they consistently describe `0.34.0` and the new
   output-quality posture.
2. **Given** the generated task plan, **When** a maintainer reviews it,
   **Then** it includes explicit tasks for version bump, impacted docs plus
   changelog, roadmap cleanup, Rust coverage, `cargo clippy`, and `cargo fmt`.
3. **Given** the completed implementation, **When** a maintainer reviews the
   validation report, **Then** the touched runtime files show focused automated
   coverage and the closeout is clean on formatting and lint expectations.

### Edge Cases

- A packet has all expected sections but only one-line bullets with no rationale; Canon must downgrade it instead of treating section count as quality.
- A packet is materially closed by explicit constraints and should not be punished for lacking multiple alternatives; Canon must separate justified closure from shallow reasoning.
- A packet mixes strong sections with one critical missing-body marker; Canon must not treat the whole artifact as publishable without naming the blocking gap.
- Existing summary helpers already count non-placeholder items; the new posture must not regress unaffected modes that already rely on honest gap markers.
- Docs, mirrored skills, and runtime version anchors can drift independently; the feature must close them all in one release sweep.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST compute a shared output-quality assessment for targeted governed-mode packets using authored evidence rather than heading presence alone.
- **FR-002**: The shared assessment MUST distinguish at least three postures when relevant: `structurally-complete`, `materially-useful`, and `publishable`.
- **FR-003**: The system MUST surface downgrade reasons or missing evidence signals when a packet cannot be classified above `structurally-complete`.
- **FR-004**: Inspect surfaces for targeted authored modes MUST expose the output-quality posture together with the evidence or gaps that produced it.
- **FR-005**: Runtime mode summaries for targeted packet families MUST use the shared output-quality posture instead of generic ready-sounding language.
- **FR-006**: Rendered packet artifacts for targeted fallback-heavy families MUST preserve explicit missing-body or downgrade language rather than synthetic prose that reads like approved reasoning.
- **FR-007**: The system MUST treat materially closed decisions as a valid posture distinct from shallow or missing reasoning, so Canon does not invent unnecessary alternatives.
- **FR-008**: Existing honesty markers such as `## Missing Authored Body`, `## Missing Evidence`, blocked posture, unsupported posture, unresolved findings, and explicit no-contradiction signals MUST remain intact or become stricter.
- **FR-009**: The feature MUST reuse or extend existing placeholder-detection and authored-context parsing surfaces where possible instead of introducing a parallel ad hoc quality vocabulary in each mode.
- **FR-010**: Embedded skill sources and mirrored `.agents/skills/` copies for impacted modes MUST describe the new output-quality posture consistently with runtime behavior.
- **FR-011**: Impacted docs, templates, README, mode guidance, roadmap continuity, and `CHANGELOG.md` MUST describe the delivered `0.34.0` behavior consistently.
- **FR-012**: Cargo manifests, lockfile surfaces, shared compatibility references, and other release-facing version anchors MUST report `0.34.0` consistently.
- **FR-013**: The task plan for this feature MUST include explicit tasks for version bump, impacted docs plus changelog, roadmap cleanup, Rust coverage for modified or created files, `cargo clippy`, and `cargo fmt`.
- **FR-014**: Modified or newly created Rust files in the affected engine paths MUST receive focused automated validation coverage before completion.
- **FR-015**: Final structural validation for the feature MUST include clean `cargo fmt` and `cargo clippy` results.
- **FR-016**: Existing `.canon/` persistence, publish destinations, approval targets, and recommendation-only semantics MUST remain unchanged.

### Key Entities *(include if feature involves data)*

- **Output Quality Assessment**: A shared classification of a packet or inspect result that captures whether the output is structurally complete, materially useful, or publishable.
- **Quality Downgrade Reason**: An explicit statement explaining what missing evidence, missing authored content, placeholder density, or unresolved gap prevented a stronger posture.
- **Quality Evidence Signal**: A positive statement that names the concrete authored support, tradeoff coverage, closure basis, or review evidence that justified a stronger posture.
- **Material Closure Signal**: A signal that the decision or packet is deliberately bounded to one viable path and should not be judged by alternative-count heuristics.
- **Fallback Packet Surface**: A rendered artifact section that currently risks reading as valid reasoning when the authored body is absent or thin.
- **Mode Quality Contract**: The combined runtime, artifact, skill, template, and documentation behavior that explains what quality posture means for a given mode family.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every targeted packet family can surface a non-empty output-quality posture with explicit evidence or downgrade reasons on representative inputs.
- **SC-002**: Focused automated validation proves at least one positive and one negative classification path for each affected runtime quality seam.
- **SC-003**: No affected fallback artifact emits synthetic prose that can plausibly be mistaken for approved reasoning when the authored source is missing or shallow.
- **SC-004**: A maintainer can inspect a representative packet summary and determine within one read whether the output is only structurally complete, materially useful, or publishable.
- **SC-005**: Release-facing docs, skill mirrors, shared compatibility references, roadmap text, and `CHANGELOG.md` all align to `0.34.0` and the delivered output-quality contract.
- **SC-006**: All modified or newly created Rust files in the feature slice are exercised by focused automated validation and the final closeout is clean on `cargo fmt` and `cargo clippy`.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, repo-local skill validation, and spec artifact consistency checks.
- **Logical validation**: Focused tests for output-quality classification, inspect surfaces, summary posture, fallback rendering, version-anchor sync, and representative docs or skill contract expectations.
- **Independent validation**: A separate review of emitted packets and inspect results to confirm that Canon does not overstate quality or publishability when evidence is weak.
- **Evidence artifacts**: Validation results, coverage notes, and closeout findings recorded in `specs/034-output-quality-gates/validation-report.md`.

## Decision Log *(mandatory)*

- **D-001**: Model top output quality as a shared posture classification rather than one-off wording changes, **Rationale**: the defect is systemic false confidence, not isolated phrasing.
- **D-002**: Keep materially closed decisions distinct from shallow packets, **Rationale**: quality gates must not reward fake alternatives or punish justified constraint closure.
- **D-003**: Treat version bump, docs plus changelog sweep, roadmap cleanup, Rust coverage, `cargo clippy`, and `cargo fmt` as first-class delivery work, **Rationale**: release drift and validation gaps are known failure modes in this repository.

## Non-Goals

- Adding a new mode, publish destination, or approval capability.
- Rewriting unaffected packets purely for prose polish without changing quality posture.
- Changing `.canon/` runtime persistence or operational semantics.
- Introducing external network-backed evidence gathering or non-local execution dependencies.

## Assumptions

- Existing authored-context parsing and placeholder-detection helpers are sufficient foundations for a shared quality assessment without adding new persistence schema.
- Targeted inspect and summary surfaces can expose stronger quality posture without changing the underlying publish command semantics.
- `0.34.0` is the intended version anchor for this feature, so versioned docs and compatibility references are in scope.
- Focused automated coverage for touched Rust files is the required bar for this feature rather than whole-workspace coverage parity changes.
