# Feature Specification: Contextual Help And Documentation Architecture (Canon)

**Feature Branch**: `073-contextual-help-docs`

**Created**: 2026-06-06

**Status**: Draft

**Input**: User description: "Canon help-next — governance and documentation guidance surface for Canon modes, packet readiness, evidence, approvals, lineage, and promotion."

## Clarifications

### Session 2026-06-06

- Q: Should this feature be one monolithic cross-repo spec? → A: No. Canon owns `canon help-next` and mode/documentation diagnostics. Boundline owns `boundline help-next` and runtime/session diagnostics. Two separate specs, coordinated but independently owned.

## User Scenarios & Testing

### User Story 1 - Discover Next Governance Action From Any Mode (Priority: P1)

An operator in any Canon mode runs `canon help-next` and receives the current governance state, missing artifacts, the next recommended action, the exact mode or command, and a link to Canon mode documentation.

**Why this priority**: Canon has 18+ modes with different artifact requirements. Without a discoverable next step, operators cannot navigate the governance surface.

**Independent Test**: Enter review mode with an incomplete packet, run `canon help-next`, confirm it identifies missing documents and recommends the next authoring step.

**Acceptance Scenarios**:

1. **Given** a selected Canon mode with no packet yet, **When** the operator runs `canon help-next`, **Then** the output reports state=no-packet, lists required documents for the mode, and recommends creating the first ordered document.
2. **Given** a packet with missing required documents, **When** the operator runs `canon help-next`, **Then** the output identifies which documents are missing and in what order they are needed.
3. **Given** a packet with all documents present but no evidence, **When** the operator runs `canon help-next`, **Then** the output reports readiness=pending-evidence and recommends the evidence-gathering mode or command.
4. **Given** a packet that is ready for approval, **When** the operator runs `canon help-next`, **Then** the output reports readiness=ready, shows the approval state, and recommends the approval command.

---

### User Story 2 - Diagnose Promotion And Lineage Blockers (Priority: P2)

An operator wants to publish or promote a packet but is blocked by incomplete lineage, missing approvals, or promotion policy violations.

**Why this priority**: Promotion blockers are opaque without explicit diagnostics. Operators waste time guessing why a packet cannot be published.

**Independent Test**: Create a packet with missing lineage, run `canon help-next`, confirm it identifies the lineage gap.

**Acceptance Scenarios**:

1. **Given** a packet blocked from promotion due to incomplete lineage, **When** the operator runs `canon help-next`, **Then** the output identifies the missing lineage refs and the command to add them.
2. **Given** a packet awaiting approval, **When** the operator runs `canon help-next`, **Then** the output shows the approval state, required approver role, and the approval command.

---

### Edge Cases

- What happens when `help-next` is run outside any Canon workspace (no `.canon/` directory)?
- What happens when a mode has no packet template yet (e.g., a new mode without defaults)?
- How does the system handle a corrupt or unreadable packet file?

## Requirements

### Functional Requirements

- **FR-001**: The system MUST provide a `canon help-next` command that inspects the current workspace and governance state without mutating any files.
- **FR-002**: The system MUST detect and report at minimum: selected mode, packet existence, ordered document completeness, missing required documents, evidence presence, readiness state, approval state, lineage completeness, and promotion blockers.
- **FR-003**: For each detected state, the system MUST output: current governance state label, missing artifacts list, next recommended action, exact mode or command, and a link to relevant Canon documentation.
- **FR-004**: The system MUST support at least five core Canon modes (review, architecture, requirements, discovery, implementation).
- **FR-005**: The system MUST prioritize blocking conditions (missing required documents, failed approvals) over informational conditions.
- **FR-006**: Documentation links MUST be relative wiki paths or stable URLs that remain valid across releases.
- **FR-007**: The system MUST NOT mutate packet state, evidence, or mode configuration during `help-next` execution.

### Key Entities

- **CanonHelpNextState**: An enumeration of detectable governance states (no-mode, no-packet, incomplete-documents, pending-evidence, pending-approval, ready, blocked-promotion) with associated diagnostics.
- **CanonModeDiagnostic**: A finding about the current mode's packet/document/evidence/approval/lineage state with severity and repair guidance.
- **CanonHelpNextRecommendation**: The resolved next action including state label, mode/command, reason, and documentation link.

## Success Criteria

- **SC-001**: An operator in any of the five core Canon modes can identify the next recommended governance action within 15 seconds of running `canon help-next`.
- **SC-002**: 100% of missing-required-document scenarios produce a help-next output that identifies the missing document by name and order.
- **SC-003**: Promotion blockers are identified with lineage or approval detail in 100% of regression cases.

## Assumptions

- The existing mode templates, packet validation, and readiness surfaces provide the raw state data that `help-next` consumes.
- Documentation links reference the Canon wiki structure.
- Boundline `help-next` is a separate Boundline-owned feature spec (boundline/specs/073-contextual-help-docs/).
- The first slice implements help-next as a read-only diagnostic; interactive repair guidance is deferred.
