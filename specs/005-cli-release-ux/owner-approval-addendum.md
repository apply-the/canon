# Addendum: Owner Fallback and Approval Boundary

## Purpose

This addendum records runtime UX decisions that are adjacent to the install
and release experience defined by Feature 005, while keeping them outside the
implementation scope of the Phase 1 installable CLI release.

Feature 005 establishes how Canon is installed, verified, and presented as a
serious CLI product. The decisions here explain how a later runtime feature
should reduce first-run friction around ownership and approvals without
smuggling runtime semantic changes into the release UX tranche.

## Scope Relationship to Feature 005

Feature 005 remains responsible for:

- install-first distribution
- release artifact contracts
- version visibility and verification
- compatibility messaging in shared skill preflight

This addendum does not change that scope. Instead, it records follow-on
runtime decisions that should be referenced by future implementation work once
the install-first baseline is in place.

Specifically, this addendum does not authorize:

- changing Canon mode semantics inside Feature 005
- treating provider-side PR approval as Canon-native approval evidence
- silently expanding release UX work into run-intake or `.canon/` persistence
  redesign

## Recorded Decisions

### A-001: Owner remains a human ownership marker, not a typed role

**Decision**: `owner` continues to mean the human who takes responsibility for
the run. It is not promoted into a role system and it does not imply approval
authority.

**Rationale**: current Canon policy checks need named human ownership, not a
separate role taxonomy.

**Consequences**:

- the persisted value can stay simple and readable
- approval authority remains modeled independently through approval records

### A-002: Owner and approver remain distinct identities

**Decision**: `owner` and approval `by` remain separate concepts even when the
same human may populate both values.

**Rationale**: a run owner carries accountability for the run, while an
approval record captures who explicitly allowed a gate or invocation target.

**Consequences**:

- Canon can model both single-person and multi-person teams without adding a
  forced segregation-of-duties rule
- approval trails remain explicit and auditable at the run level

### A-003: Recommended owner resolution order is explicit input, then Git local, then Git global, then targeted prompt

**Decision**: future runtime work should resolve `owner` in this order:

1. explicit CLI input
2. repository-local Git identity
3. global Git identity
4. targeted prompt or guided failure

**Rationale**: this minimizes unnecessary prompting while preferring the most
specific trustworthy identity available in the local environment.

**Consequences**:

- normal repo-based usage can avoid repeated owner prompts
- host-level fallback remains visible and intentionally weaker than repo-local
  identity

### A-004: Persist resolved owner as `Name <email>` when fully known

**Decision**: when both fields are present, the recommended persisted owner
format is `Name <email>`.

**Rationale**: the combined form is human-readable, disambiguating, and stable
enough for manifests and audit trails.

**Consequences**:

- manifests stay readable without a larger identity schema
- provenance, if later needed, should live in a separate field rather than be
  encoded into the owner string

### A-005: Use repo-local identity silently; confirm weaker fallbacks

**Decision**: future UX should auto-use a complete repo-local Git identity
without prompting, while global Git fallback or partial identity data should
trigger explicit confirmation or a guided prompt.

**Rationale**: the common path should be low-friction, but weaker inputs
should not be accepted opaquely.

**Consequences**:

- first-run UX improves without hiding identity quality
- Canon does not silently invent ownership from incomplete environment data

### A-006: Canon approvals remain repo-local, durable, and run-scoped

**Decision**: Canon approval state continues to be recorded as explicit
approval records attached to the run, not inferred from provider-side review
state.

**Rationale**: Canon gates consume local, durable, run-scoped evidence. A PR
approval in GitHub or another provider does not guarantee the same scope,
durability, or offline traceability.

**Consequences**:

- approval-gated runs still require Canon-native approval evidence
- PR-centric teams can keep using provider reviews, but those reviews are not
  automatically equivalent to Canon approvals

### A-007: Future PR approval integration must be explicit evidence import or bridging

**Decision**: if Canon later integrates provider review state, it must do so
through an explicit evidence import or adapter boundary, not through implicit
live coupling.

**Rationale**: explicit bridging preserves traceability, reproducibility, and
clear audit boundaries.

**Consequences**:

- runtime logic remains understandable offline
- Canon avoids hidden dependency on mutable provider state during gated runs

## Follow-On Implementation Targets

When this behavior is implemented in a later feature, the expected touchpoints
are:

- `crates/canon-cli/src/app.rs` for making `--owner` optional only after a
  runtime resolver exists
- `crates/canon-engine/src/orchestrator/service.rs` for owner resolution before
  classifier and gatekeeper evaluation
- `crates/canon-engine/src/orchestrator/classifier.rs` for applying ownership
  requirements to the resolved value
- `crates/canon-engine/src/persistence/manifests.rs` and
  `crates/canon-engine/src/domain/execution.rs` for persisting and propagating
  the resolved owner
- `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh` and
  `defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1` for
  keeping skill-side preflight aligned with runtime behavior

## Verification Expectations for the Follow-On Feature

- explicit `--owner` input must bypass Git lookup
- complete repo-local Git identity must populate owner without prompting
- global Git fallback must be explicit to the user
- partial identity data must not be accepted silently
- single-person teams may use the same identity as both owner and approver
- PR approval alone must not unblock Canon gates without an explicit bridge