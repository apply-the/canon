# Feature Specification: Mode Publish Alignment

**Feature Branch**: `045-mode-publish-alignment`  
**Created**: 2026-05-11  
**Status**: Draft  
**Input**: User description: "Align Canon mode publish behavior and documentation so mode-specific packet outputs, publishability states, and special projected documents remain consistent across runtime contracts, docs, and release surfaces; include version bump and final validation expectations for coverage, formatter, and linter."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact because the slice changes publish gating, assistant-facing command references, release/version surfaces, and validation expectations without changing Canon's artifact families or approval model  
**Scope In**: security-assessment publish-state alignment, assistant publish command-surface alignment, version bump to `0.45.0`, validation/reporting closeout, and any directly affected tests/tech-docs/release assertions  
**Scope Out**: new mode families, new publish destinations, new ADR/PRD/C4 artifact projections, new CLI syntax, and broad documentation rewrites beyond touched surfaces

**Invariants**:

- Default publish destinations by mode MUST remain unchanged unless an explicit mismatch is already documented for this slice.
- Requirements `prd.md`, architecture C4 packet outputs, and ADR projection rules MUST remain exactly as currently implemented.
- Publish MUST continue to preserve governed artifacts under `.canon/` and only copy visible packets into repository destinations.
- `canon publish` MUST keep the positional `RUN_ID` command shape; assistant surfaces may not invent a divergent CLI contract.

**Decision Traceability**: Decisions and evidence for this slice will be recorded in `specs/045-mode-publish-alignment/decision-log.md` and `specs/045-mode-publish-alignment/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish Security Assessment Packets Consistently (Priority: P1)

As an operator reviewing a governed security-assessment run, I need the runtime publish gate to match the documented operational behavior so blocked or approval-gated readable packets can still be published for broader review.

**Why this priority**: This is a real runtime/documentation inconsistency affecting publish behavior, not just wording drift.

**Independent Test**: Create blocked and awaiting-approval security-assessment runs with readable artifacts and verify `canon publish <RUN_ID>` succeeds without changing other mode publish rules.

**Acceptance Scenarios**:

1. **Given** a security-assessment run in `AwaitingApproval` with a readable packet, **When** the operator runs `canon publish <RUN_ID>`, **Then** Canon publishes the packet to the documented family root instead of rejecting the run for not being `Completed`.
2. **Given** a security-assessment run in `Blocked` with a readable packet, **When** the operator runs `canon publish <RUN_ID>`, **Then** Canon publishes the packet and keeps the normal packet metadata sidecar.
3. **Given** a non-operational mode in `AwaitingApproval` or `Blocked`, **When** the operator runs `canon publish <RUN_ID>`, **Then** Canon still rejects the publish unless that mode was already allowed before this feature.

---

### User Story 2 - Keep Assistant Publish Guidance Honest (Priority: P2)

As an assistant user, I need package metadata and prompt-pack examples to reference the actual `canon publish` CLI syntax so generated guidance does not tell me to use unsupported flags.

**Why this priority**: Broken assistant guidance is less severe than runtime drift, but it directly causes operator confusion and invalid commands.

**Independent Test**: Validate the assistant package test suite and inspect assistant command surfaces to confirm publish examples use positional `RUN_ID` syntax rather than `--run`.

**Acceptance Scenarios**:

1. **Given** the assistant command metadata and prompt pack, **When** a publish example is surfaced, **Then** it uses `canon publish <RUN_ID>` and not a synthetic `--run` form.
2. **Given** the assistant package validation suite, **When** it reads the metadata and prompt pack, **Then** no publish command surface drifts from the shipped CLI contract.

---

### User Story 3 - Close The Slice As A Versioned Release Surface Update (Priority: P3)

As a maintainer, I need the version line, validation artifacts, and release-facing assertions to close consistently so this cleanup slice does not leave the repository in another partially aligned state.

**Why this priority**: The repository expects each delivery slice to bump the release line and capture validation evidence, and the user explicitly asked for that closeout discipline.

**Independent Test**: Run focused regressions plus formatter, linter, and touched-file coverage review, then confirm versioned docs and release assertions all reference `0.45.0` consistently.

**Acceptance Scenarios**:

1. **Given** the repository release surfaces, **When** the slice is complete, **Then** versioned references that belong to this feature align on `0.45.0`.
2. **Given** the final validation pass, **When** formatter, linter, tests, and coverage checks run, **Then** the evidence is captured in the feature validation report and the slice is ready for merge.

### Edge Cases

- What happens when a security-assessment run is blocked or awaiting approval but has no readable persisted artifacts yet?
- How does the system handle an assistant metadata surface that still references positional syntax correctly but a prompt example drifts back to `--run`?
- Which invariant is most likely to be stressed by this case? The rule that only the intended operational mode gains non-`Completed` publishability while all other mode gates remain unchanged.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST align security-assessment publish gating with the documented operational publish behavior for readable blocked or approval-gated packets.
- **FR-002**: System MUST preserve the existing non-`Completed` publish exceptions for incident, migration, system-assessment, and supply-chain-analysis while adding no unintended publish exceptions to other modes.
- **FR-003**: System MUST keep all existing default publish destinations unchanged across every mode.
- **FR-004**: System MUST keep requirements `prd.md`, architecture C4 packet outputs, and ADR projection behavior unchanged while fixing the identified drift.
- **FR-005**: Assistant command metadata and assistant prompt-pack publish examples MUST use the real positional `canon publish <RUN_ID>` contract.
- **FR-006**: The repository MUST bump the release line to `0.45.0` across affected version-governed surfaces touched by this slice.
- **FR-007**: The feature MUST record structural, logical, independent, and coverage validation evidence in a dedicated validation report before closeout.

### Key Entities *(include if feature involves data)*

- **Operational Publishability Rule**: The policy that decides which modes may publish readable packets before `Completed` and under which run states.
- **Assistant Publish Surface**: The assistant metadata, prompt-pack examples, and related validation expectations that expose `canon publish` usage to users.
- **Release Line Surface**: The set of versioned repository references, tests, and validation artifacts that must move together when the feature ships.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Focused security-assessment publish regressions pass for both `AwaitingApproval` and `Blocked` states with 0 failing assertions.
- **SC-002**: Assistant package validation passes with 0 publish-command-surface drift findings.
- **SC-003**: Formatter and linter both pass cleanly with 0 reported errors or warnings in the final validation tranche.
- **SC-004**: Every new or modified Rust source file touched by the slice reaches at least 95% line coverage, or any accepted exception is explicitly justified in the validation report before merge.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and any touched release-surface assertions.
- **Logical validation**: Focused `cargo test` coverage for security-assessment publish behavior, assistant package validation, and any directly affected tech-docs/release tests; finish with `cargo nextest run --workspace --all-features` if the environment remains stable enough.
- **Independent validation**: Readback comparison between documented publish semantics and runtime behavior after implementation, plus commit-closeout review of version surfaces.
- **Evidence artifacts**: `specs/045-mode-publish-alignment/validation-report.md`, `specs/045-mode-publish-alignment/decision-log.md`, generated design artifacts, and coverage evidence from `lcov.info` or focused touched-file analysis.

## Decision Log *(mandatory)*

- **D-001**: Treat this slice as bounded publish-alignment work rather than a broader mode redesign, **Rationale**: the confirmed drift is narrow and the repo already has aligned PRD, C4, and ADR special-output behavior that should not be reopened.

## Non-Goals

- Changing the publish destination family roots for any mode.
- Adding new projected document families beyond the already shipped PRD, C4, and ADR behaviors.
- Introducing a new `canon publish` CLI syntax or assistant-only alias layer.

## Assumptions

- Security-assessment should follow the same recommendation-only operational publish posture already documented beside other operational modes.
- The existing release line should advance from `0.44.0` to `0.45.0` for this feature slice.
- Focused test coverage exists or can be added locally for the touched publish and assistant surfaces without redesigning unrelated mode contracts.
- The remaining documented mode publish destinations are already authoritative unless the implementation shows another concrete mismatch during execution.
