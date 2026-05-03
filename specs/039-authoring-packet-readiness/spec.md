# Feature Specification: Authoring Experience And Packet Readiness

**Feature Branch**: `039-authoring-packet-readiness`  
**Created**: 2026-05-02  
**Status**: Draft  
**Input**: User description: "Deliver feature 039 as a full authoring experience and packet readiness slice: unify template, example, clarity inspection, run, critique, and publish guidance across file-backed modes, strengthen carry-forward of missing context, assumptions, unresolved questions, and readiness deltas for file and directory authored inputs, preserve explicit authored-body honesty and existing mode boundaries, include an explicit 0.39.0 version bump task, docs/changelog task, roadmap cleanup, coverage for touched Rust files, clean clippy and cargo fmt closeout, and implement end-to-end without splitting into slices."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This work reshapes pre-run clarity,
authoring guidance, and release surfaces across existing file-backed modes, but
it does not introduce a new governed mode, new persistence family, or implicit
execution path.  
**Scope In**: file-backed authored-input lifecycle guidance; clarity inspection
summary data and rendering for single-file and directory-backed packets;
explicit classification of authoritative brief versus supporting or carried-
forward inputs; readiness-delta and next-step guidance for weak versus strong
authored packets; shared authoring docs, templates, examples, and inspect-
clarity skill guidance; focused Rust tests; docs, changelog, roadmap, and
explicit `0.39.0` release alignment.  
**Scope Out**: new mode families; changes to run-state enums or approval
semantics; automatic rewriting of `canon-input/`; hidden dereferencing of
`.canon/`, published packets, or active editor state; `pr-review` authoring
changes; new artifact file families beyond the documentation or contract
surfaces this feature needs.

**Invariants**:

- `## Missing Authored Body` remains stronger than generated filler; the system
  must keep missing authored sections explicit instead of inventing closure.
- The current-mode brief remains authoritative for readiness; carried-forward
  files may ground the packet, but they must not silently replace the current
  brief.
- Canon must not rewrite `canon-input/` automatically or infer authored input
  from `.canon/`, published docs, open tabs, or any other incidental surface.
- Directory-backed packet ergonomics must remain additive to existing canonical
  bindings and must not change mode boundaries or `pr-review` exclusion.

**Decision Traceability**: Decisions and validation evidence for this feature
MUST be recorded in
`specs/039-authoring-packet-readiness/decision-log.md` and
`specs/039-authoring-packet-readiness/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Understand Packet Readiness Before Run (Priority: P1)

As a maintainer authoring a Canon packet, I want `inspect clarity` to tell me
which authored input is authoritative, which inputs are only supporting or
carried-forward context, and what readiness delta still blocks a strong packet,
so I can strengthen the packet without guessing.

**Why this priority**: This is the narrowest runtime seam with the highest
authoring leverage. If Canon cannot explain packet shape and readiness honestly
before a run starts, the rest of the authoring lifecycle stays implicit.

**Independent Test**: A reviewer can run `canon inspect clarity` on a single-
file packet, a directory-backed carry-forward packet, and a weak multi-file
packet and determine the authoritative brief, support inputs, and next
authoring step directly from the clarity output.

**Acceptance Scenarios**:

1. **Given** a single-file authored packet, **When** `inspect clarity` runs,
   **Then** Canon reports the packet shape, treats that file as authoritative,
   and does not invent carried-forward context that is not present.
2. **Given** a directory-backed packet with `brief.md`, `source-map.md`, and
   `selected-context.md`, **When** `inspect clarity` runs, **Then** Canon
   treats `brief.md` as the readiness brief, surfaces `source-map.md` and
   `selected-context.md` as support context, and keeps any remaining readiness
   gaps explicit.
3. **Given** a directory-backed or multi-file packet without an obvious
   authoritative brief, **When** `inspect clarity` runs, **Then** Canon keeps
   the ambiguity explicit and recommends tightening the packet shape instead of
   guessing which file should control readiness.

---

### User Story 2 - Follow One Canonical Authoring Lifecycle (Priority: P2)

As a Canon author, I want the templates, examples, inspect-clarity guidance,
and mode docs to teach one canonical lifecycle from authored packet to
publishable output, so the path from weak brief to durable packet is explicit
across file-backed modes.

**Why this priority**: Runtime guidance is only half the problem. If the repo
surfaces still teach fragmented authoring patterns, maintainers will continue
to rely on tribal knowledge instead of a durable method.

**Independent Test**: A reviewer can compare the updated authoring guide,
carry-forward example, and inspect-clarity guidance and find one consistent
sequence: author the packet, inspect clarity, run the matching mode, critique
the output, and publish when the packet is truly ready.

**Acceptance Scenarios**:

1. **Given** the updated mode or authoring guides, **When** a maintainer reads
   the file-backed workflow, **Then** they see the same lifecycle language and
   honesty about current briefs, carried-forward context, and packet readiness.
2. **Given** the updated carry-forward example, **When** a reviewer inspects
   the packet shape, **Then** the roles of `brief.md`, `source-map.md`, and
   optional narrowed context are explicit and match runtime guidance.
3. **Given** the updated inspect-clarity skill guidance, **When** a user wants
   to strengthen a packet before running Canon, **Then** the skill points them
   to the same authoritative-brief and readiness-delta story that the runtime
   exposes.

---

### User Story 3 - Ship 039 As The Coherent Release Line (Priority: P3)

As a Canon maintainer, I want the release surfaces to describe the delivered
`0.39.0` authoring and packet-readiness contract, so the repository version,
docs, and roadmap match the runtime behavior actually shipped.

**Why this priority**: This feature is repository-facing product work. If the
version line, changelog, roadmap, and release guardrails stay on the previous
story, the slice is incomplete.

**Independent Test**: Release-alignment checks and human review show one
coherent `0.39.0` story across runtime compatibility references, README,
changelog, roadmap, and the feature validation report.

**Acceptance Scenarios**:

1. **Given** the updated branch, **When** a reviewer checks the version
   surfaces, **Then** the workspace, compatibility references, and release
   checks all point to `0.39.0`.
2. **Given** the updated docs and changelog, **When** they are reviewed,
   **Then** they describe the delivered authoring lifecycle and readiness
   behavior consistently.
3. **Given** the updated roadmap, **When** the maintainer checks remaining
   work, **Then** `039` is no longer listed as future work and no stale entry
   remains after the delivered slice.

### Edge Cases

- A directory-backed packet contains `source-map.md` but no `brief.md`; Canon
  must not treat provenance as current-mode readiness.
- A multi-file explicit `--input` invocation mixes a strong brief with weak
  supporting notes; Canon must keep the readiness delta on the missing current-
  mode content instead of declaring the packet publishable.
- A packet is materially closed but still uses carried-forward context; Canon
  must preserve that closure without implying the supporting files are the new
  authoritative brief.
- A directory packet contains no usable authored content except mutation
  payloads; Canon must keep the packet non-ready and surface the absence of a
  real authored brief.
- The inspect surface must stay honest when a packet is only structurally
  complete even if the docs or examples describe an ideal lifecycle.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `canon inspect clarity` MUST classify file-backed authored input
  packets into an explicit packet-shape summary that distinguishes at least the
  authoritative brief surface from supporting or carried-forward inputs.
- **FR-002**: When a directory-backed packet includes `brief.md`, Canon MUST
  treat that file as the authoritative current-mode brief for readiness
  guidance.
- **FR-003**: When a directory-backed packet includes `source-map.md` and
  `selected-context.md`, Canon MUST surface them as explicit support context,
  not as substitutes for the current-mode brief.
- **FR-004**: When Canon cannot determine an authoritative current-mode brief
  safely from the supplied file-backed inputs, it MUST keep that ambiguity
  explicit and recommend tightening the packet shape rather than inventing
  authority.
- **FR-005**: Clarity output MUST surface readiness deltas that separate
  missing current-mode authored content from merely carried-forward context or
  supporting notes.
- **FR-006**: Clarity reasoning and recommended-focus guidance MUST remain
  honest about missing authored content, assumptions, unresolved questions, and
  carried-forward defaults without implying hidden ingestion from `.canon/`,
  published packets, or other incidental files.
- **FR-007**: CLI clarity markdown rendering MUST expose the packet shape,
  authoritative-input guidance, and readiness-delta story in a readable,
  deterministic section.
- **FR-008**: Shared file-backed authoring docs MUST teach one explicit
  lifecycle that covers packet authoring, clarity inspection, governed run,
  critique, and publish.
- **FR-009**: Shared examples and carry-forward guidance MUST document the
  roles of `brief.md`, `source-map.md`, optional narrowed context, and honest
  readiness limits consistently with runtime behavior.
- **FR-010**: The inspect-clarity skill guidance MUST align with the delivered
  authoring lifecycle and the new packet-shape or readiness guidance.
- **FR-011**: Existing canonical mode bindings, directory preference rules,
  review-mode strictness, and `pr-review` clarity exclusion MUST remain
  unchanged by this feature.
- **FR-012**: Focused automated tests MUST cover single-file clarity,
  directory-backed carry-forward clarity, ambiguous directory-backed input, and
  CLI clarity rendering for the new guidance.
- **FR-013**: The implementation plan and tasks MUST include an explicit task
  to bump the workspace release line to `0.39.0` across manifests, runtime
  compatibility references, and release-alignment checks.
- **FR-014**: The implementation plan and tasks MUST include explicit tasks
  for docs, changelog, and roadmap updates.
- **FR-015**: The implementation plan and tasks MUST include explicit coverage,
  `cargo clippy`, and `cargo fmt` closeout for touched Rust files.
- **FR-016**: Validation evidence for this slice MUST record focused Rust
  checks, docs or skill review, release-alignment checks, coverage evidence,
  and an independent review of authoring-lifecycle honesty.

### Key Entities *(include if feature involves data)*

- **Authoring Lifecycle Summary**: The clarity-facing summary that describes
  packet shape, authoritative inputs, supporting inputs, carried-forward
  surfaces, readiness deltas, and the next authoring step.
- **Authoritative Brief Surface**: The current-mode authored input Canon treats
  as the readiness source of truth for a file-backed packet.
- **Supporting Input Surface**: Additional authored files that provide
  provenance, narrowed context, or auxiliary notes without replacing the
  authoritative brief.
- **Readiness Delta**: The explicit gap between what the packet already states
  and what must still be authored before Canon should treat it as strongly
  ready.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In focused clarity validation, 100% of tested single-file,
  directory-backed, and ambiguous authored-input scenarios expose the correct
  authoritative-versus-supporting input classification without hidden
  inference.
- **SC-002**: A reviewer can determine the packet shape, authoritative brief,
  and next authoring step from the clarity output alone in under 30 seconds for
  every focused validation scenario.
- **SC-003**: Shared docs, examples, and inspect guidance all present the same
  five-step authored lifecycle with zero contradictory statements about brief
  authority or carried-forward context.
- **SC-004**: Release-alignment checks confirm one coherent `0.39.0` story
  across workspace manifests, compatibility references, README, roadmap, and
  changelog.

## Validation Plan *(mandatory)*

- **Structural validation**: focused engine or CLI tests for clarity summary and
  rendering, release-surface alignment tests, `cargo fmt --check`, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **Logical validation**: targeted clarity scenarios for single-file,
  carry-forward directory, and ambiguous multi-file packets plus manual review
  of the authored lifecycle docs.
- **Independent validation**: a separate review of authoring-lifecycle honesty,
  authoritative-brief semantics, and release-surface coherence after
  implementation.
- **Evidence artifacts**: `specs/039-authoring-packet-readiness/validation-report.md`,
  focused Rust test output, `lcov.info`, and updated docs or release surfaces.

## Decision Log *(mandatory)*

- **D-001**: Extend the existing clarity-inspection summary and documentation
  surfaces rather than create a new authoring mode or hidden planner,
  **Rationale**: the runtime already understands canonical file bindings,
  missing-context honesty, and folder-backed packets, so the safest path is to
  make that existing authoring lifecycle explicit instead of widening the
  system.

## Non-Goals

- Add a new governed mode, new run-state family, or new persistence layout.
- Rewrite `canon-input/` automatically or synthesize authored packet content.
- Allow `.canon/`, published docs, or active editor state to become implicit
  authored-input sources.
- Change approval semantics, publish destinations, or `pr-review` input rules.

## Assumptions

- Existing clarity inspection already has enough authored packet context to
  explain authoritative versus supporting inputs without changing mode
  contracts.
- The main authoring gap is lifecycle clarity and packet-shape honesty, not a
  missing governed mode or missing artifact family.
- Shared docs and skill guidance can close most cross-mode confusion once they
  point to the same authoritative-brief and readiness-delta story.
- The workspace should advance to `0.39.0` for this slice, and release
  alignment must reflect that version rather than preserve the previous line.