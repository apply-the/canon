# Feature Specification: High-Risk Operational Programs

**Feature Branch**: `014-high-risk-ops`  
**Created**: 2026-04-25  
**Status**: Draft  
**Input**: User description: "Proceed with High-Risk Operational Programs by promoting `incident` and `migration` from skeleton status to full depth and adding explicit artifact contracts for blast radius, containment, compatibility, sequencing, and fallback planning before resuming output-polish, packaging, or protocol work."

## Governance Context *(mandatory)*

**Mode**: change

**Risk Classification**: systemic-impact. This feature completes the two
remaining modeled operational modes, adds new high-risk artifact contracts and
gating expectations, and changes how Canon represents operational readiness,
containment, and rollback credibility. The work remains inside Canon's current
CLI-first runtime, evidence model, publish flow, and governed persistence, but
it changes cross-cutting product behavior rather than a single isolated output
surface.

**Scope In**:

- Promote `incident` from skeleton status to a first-class governed mode.
- Promote `migration` from skeleton status to a first-class governed mode.
- Define explicit authored-input and artifact expectations for blast radius,
  containment, compatibility, sequencing, and fallback planning.
- Define stricter approval and readiness expectations for high-risk
  operational workflows than for ordinary implementation or refactor paths.
- Ensure the new operational packets are inspectable, publishable, and
  reviewable as durable Canon artifacts.
- Update the documentation, defaults, skills, and policy surfaces needed to
  describe these modes honestly.

**Scope Out**:

- Introducing new modeled modes beyond `incident` and `migration`.
- Reworking already-delivered mode packets only for output polish or external
  standardization.
- Changing Canon's recommendation-only posture into direct operational
  automation.
- Delivering `security-assessment`, `supply-chain-analysis`, packaging, or
  protocol-interoperability work as part of this feature.
- Replacing Canon's run identity, publish model, or persistence layout.

**Invariants**:

- No operational run may advance without explicit mode, risk, zone, and
  artifact expectations.
- High-risk operational packets MUST expose blast radius, containment,
  sequencing, compatibility, fallback, and evidence gaps honestly; they MUST
  NOT imply readiness that the packet cannot support.
- `incident` and `migration` MUST remain recommendation-only in v0.x; Canon
  may govern decisions and evidence, but it MUST NOT perform privileged
  operational actions autonomously.
- Validation evidence MUST remain separate from generation behavior, and the
  new modes MUST NOT weaken gating, traceability, or artifact quality in modes
  Canon already ships.

**Decision Traceability**: Decisions for this feature are recorded in
`specs/014-high-risk-ops/decision-log.md` and cross-linked from the Canon
change run that implements the feature under `.canon/runs/<…>/decisions/`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Incident lead gets a governed containment packet (Priority: P1)

An incident lead has a live operational problem and wants Canon to produce a
governed packet that makes blast radius, containment steps, sequencing, and
fallback explicit before the team acts.

**Why this priority**: `incident` is one of the remaining unfinished modeled
modes, and it is the most time-sensitive operational workflow. If Canon cannot
govern this path credibly, the roadmap still has a major coverage gap.

**Independent Test**: Start an `incident` run from a bounded incident brief,
then confirm the completed packet exposes blast radius, containment,
sequencing, fallback, and gating posture clearly enough for a responder to act
without hidden state.

**Acceptance Scenarios**:

1. **Given** an incident brief with a bounded surface and named constraints,
   **When** the user starts an `incident` run, **Then** Canon emits a packet
   that includes explicit blast-radius, containment, sequencing, and fallback
   artifacts.
2. **Given** an incident brief with insufficient evidence to justify safe
   action, **When** the run is evaluated, **Then** Canon exposes the evidence
   gap or block explicitly instead of implying operational readiness.
3. **Given** a completed incident packet, **When** a reviewer inspects it,
   **Then** they can identify what to contain first, what might widen impact,
   and what fallback remains available.

---

### User Story 2 - Migration owner gets a compatibility-aware rollout packet (Priority: P2)

A migration owner needs Canon to govern a risky transition where sequencing,
compatibility, coexistence, and rollback credibility matter more than normal
change execution.

**Why this priority**: `migration` is the other unfinished modeled mode. The
product cannot claim full mode coverage while migration remains a skeleton,
especially because compatibility and fallback are first-order governance
concerns.

**Independent Test**: Start a `migration` run from a bounded migration brief,
then confirm the packet exposes compatibility, sequencing, fallback, and
release-readiness decisions clearly enough for a go/no-go review.

**Acceptance Scenarios**:

1. **Given** a migration brief with source state, target state, and rollout
   constraints, **When** the user starts a `migration` run, **Then** Canon
   emits a packet that includes explicit compatibility, sequencing, and
   fallback planning artifacts.
2. **Given** a migration that requires phased coexistence, **When** the packet
   is emitted, **Then** Canon makes compatibility and sequencing decisions
   visible instead of treating the work as an ordinary bounded change.
3. **Given** a migration with no credible rollback or fallback, **When** the
   packet is reviewed, **Then** Canon marks the operational risk explicitly
   rather than presenting a false ready-to-ship posture.

---

### User Story 3 - Approver reviews high-risk readiness from the packet alone (Priority: P3)

An approver or downstream reviewer wants the high-risk packet to be readable
without internal manifests or hidden runtime state so they can decide whether
to advance, pause, or reject the operational plan.

**Why this priority**: The feature only closes if the emitted packets remain
credible outside the runtime. If approval still depends on hidden context,
Canon has not delivered a trustworthy operational surface.

**Independent Test**: Publish completed and blocked `incident` or `migration`
packets, then confirm an independent reviewer can determine readiness, gaps,
and fallback posture from the published artifacts alone.

**Acceptance Scenarios**:

1. **Given** a published operational packet, **When** a reviewer opens it,
   **Then** they can identify risk, scope, fallback posture, and unresolved
   gaps without consulting internal run manifests.
2. **Given** an approval-gated or blocked operational run, **When** the packet
   is inspected, **Then** the gating reason is still visible and not softened
   by the summary surface.

### Edge Cases

- An `incident` brief describes a plausible containment step but does not
  provide enough evidence to estimate blast radius confidently.
- A `migration` brief includes a target state but leaves rollback or fallback
  materially undefined.
- Compatibility assumptions conflict across supplied source artifacts.
- Sequencing can be proposed only by taking a temporary compatibility risk
  that must remain explicit.
- A packet is publishable but still blocked because the evidence bundle does
  not justify release readiness.
- The same operational surface is affected by both containment urgency and
  migration sequencing pressure.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST support `incident` as a first-class governed mode
  rather than a skeleton placeholder.
- **FR-002**: Canon MUST support `migration` as a first-class governed mode
  rather than a skeleton placeholder.
- **FR-003**: An `incident` packet MUST include explicit artifacts for blast
  radius, containment, sequencing, and fallback planning.
- **FR-004**: A `migration` packet MUST include explicit artifacts for
  compatibility, sequencing, and fallback planning.
- **FR-005**: High-risk operational packets MUST expose the specific evidence,
  assumptions, and unresolved gaps that affect readiness.
- **FR-006**: When source inputs do not support a credible operational
  conclusion, Canon MUST block or downgrade the run explicitly rather than
  fabricate confidence.
- **FR-007**: `incident` and `migration` MUST apply approval and
  release-readiness expectations at least as strict as the current ordinary
  execution paths.
- **FR-008**: The new operational modes MUST remain inspectable, publishable,
  and resumable within Canon's existing governed runtime model.
- **FR-009**: The feature MUST preserve Canon's recommendation-only posture;
  the new modes may recommend and govern actions but MUST NOT directly execute
  privileged operational change.
- **FR-010**: Documentation, defaults, and skill guidance MUST reflect the
  delivered operational packet shape and its stronger containment,
  compatibility, sequencing, and fallback expectations.
- **FR-011**: Delivering these modes MUST NOT regress the gating,
  traceability, or artifact contracts of already delivered modes.

### Key Entities *(include if feature involves data)*

- **Incident Packet**: The governed operational artifact set for active
  response work, including impact, containment, sequencing, fallback, and
  readiness posture.
- **Migration Packet**: The governed operational artifact set for risky
  transition work, including compatibility, sequencing, fallback, and
  readiness posture.
- **Blast Radius Assessment**: A bounded statement of which systems,
  interfaces, or capabilities may be affected and how far impact can spread.
- **Containment Plan**: The ordered response steps intended to stop or limit
  the spread of impact.
- **Compatibility Assessment**: The statement of what must continue to work,
  what can coexist temporarily, and what breaks if sequencing is wrong.
- **Sequencing Plan**: The operationally ordered plan that defines what must
  happen first, what can happen in parallel, and what must wait.
- **Fallback Plan**: The rollback, stop, coexistence, or alternate-path plan
  used when the primary operational plan cannot proceed safely.
- **Operational Gate Decision**: The explicit approval, block, or
  release-readiness judgment tied to the packet evidence.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In acceptance runs, 100% of completed `incident` packets include
  explicit blast-radius, containment, sequencing, and fallback artifacts.
- **SC-002**: In acceptance runs, 100% of completed `migration` packets
  include explicit compatibility, sequencing, and fallback artifacts.
- **SC-003**: In blocked or downgraded high-risk runs, 100% of packets expose
  the evidence gap or gating reason explicitly instead of implying readiness.
- **SC-004**: An independent reviewer can determine whether to advance, pause,
  or reject a completed high-risk packet in under 10 minutes using the packet
  alone.
- **SC-005**: No previously delivered mode loses required artifacts,
  traceability, or gate enforcement as a side effect of this feature.

## Validation Plan *(mandatory)*

- **Structural validation**: Focused contract, mode-discovery, policy, and
  publish-surface checks for `incident` and `migration`, plus non-regression
  validation for existing mode catalogs and artifact expectations.
- **Logical validation**: End-to-end runs for `incident` and `migration`
  covering completed, blocked, downgraded, and approval-gated paths, with
  explicit checks for blast radius, containment, compatibility, sequencing,
  and fallback artifact visibility.
- **Independent validation**: Separate packet review to confirm a reader can
  assess operational readiness, scope, and fallback credibility without hidden
  runtime context.
- **Evidence artifacts**: Validation results and review notes will be recorded
  in `specs/014-high-risk-ops/validation-report.md` and linked from the
  implementing run evidence bundle.

## Decision Log *(mandatory)*

- **D-001**: Deliver `incident` and `migration` together as one high-risk
  operational completion feature, **Rationale**: they are the two remaining
  modeled operational modes, they share stronger containment, compatibility,
  sequencing, and fallback needs than ordinary execution paths, and the
  roadmap explicitly prioritizes them before output-polish, packaging, or
  protocol expansion.

## Non-Goals

- Adding new operational modes beyond `incident` and `migration`.
- Turning Canon into an autonomous incident-response or deployment system.
- Reworking delivered mode surfaces only to standardize output shape.
- Delivering packaging, distribution-channel, or protocol-interoperability
  enhancements in this slice.
- Folding `security-assessment` or `supply-chain-analysis` into the same
  implementation effort.

## Assumptions

- Canon will keep its recommendation-only posture for high-risk operational
  workflows in v0.x.
- Existing publish, inspect, resume, and evidence mechanisms will be extended
  rather than replaced.
- Operational briefs will provide at least a bounded incident or migration
  surface; when they do not, explicit blocking or downgrade behavior is the
  correct outcome.
- Reviewers use the emitted packet as the primary source for readiness,
  fallback, and gap assessment.
- Stronger gating for `incident` and `migration` is a product goal, not a
  regression, as long as the reasoning remains visible and traceable.
