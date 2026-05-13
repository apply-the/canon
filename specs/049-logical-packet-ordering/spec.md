# Feature Specification: Logical Packet Ordering

**Feature Branch**: `049-logical-packet-ordering`  
**Created**: 2026-05-13  
**Status**: Draft  
**Input**: User description: "Establish Canon logical packet ordering across all modes, including numeric prefixes, primary artifact metadata, artifact order metadata, publish order preservation, status and inspect primary artifact surfacing, backward compatibility for legacy packet names, and documentation that distinguishes domain-language from domain-model."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact; the feature touches packet contracts, metadata, publish behavior, status and inspect summaries, and mode documentation across the mode catalog, but preserves Canon's governance semantics and historical run readability.  
**Scope In**: reader-facing packet artifact naming and sequencing for new packets, packet metadata fields for ordering, publish order preservation, status and inspect primary artifact surfacing, per-mode packet order documentation, backward-compatibility handling for legacy packet names, and clarification of `domain-language` versus `domain-model`.  
**Scope Out**: project-memory promotion policy changes, Boundline integration work, safety-net mode introduction, test generation features, workspace mutation outside Canon packet surfaces, voting workflows, TechDocs or Backstage export, and changes to Canon's core governance model.

**Invariants**:

- Existing governed runs must remain readable without rewrite; new ordering rules apply to newly emitted packets, while legacy packets continue to resolve through compatibility behavior.
- Packet readability must reflect intentional reading order rather than incidental alphabetical sort or internal implementation order.
- Sidecar artifacts such as packet metadata and AI provenance remain distinct from reader-facing packet-body artifacts.

**Decision Traceability**: `specs/049-logical-packet-ordering/decision-log.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read Packets In Intended Order (Priority: P1)

As a reviewer, I want every new Canon packet directory to show the intended reading order immediately so I can open the right artifact first and evaluate the packet without guessing.

**Why this priority**: Packet readability is the core user value. If the ordering is not explicit, the rest of the metadata and documentation improvements do not solve the main navigation problem.

**Independent Test**: Run representative modes and verify the emitted packet directory contains a `01-` primary artifact, contiguous numeric prefixes for reader-facing markdown artifacts, and an order that matches the documented packet sequence for that mode.

**Acceptance Scenarios**:

1. **Given** a completed requirements run, **When** a reviewer opens the emitted packet directory, **Then** the reader-facing markdown artifacts are prefixed with contiguous numeric ordering and `01-prd.md` is the first artifact.
2. **Given** a mode with optional artifacts, **When** one optional artifact is omitted, **Then** the emitted numbering remains contiguous and does not leave a misleading gap.

---

### User Story 2 - Surface Primary Artifact And Order In Metadata (Priority: P1)

As an assistant or downstream tool, I want Canon packet metadata to identify the primary artifact and ordered artifact list so I can summarize and hand off packet state without reconstructing the order heuristically.

**Why this priority**: Programmatic consumers need a stable contract, not filename guessing, to preserve packet semantics across status, inspect, publish, and downstream review flows.

**Independent Test**: Parse packet metadata for representative runs and verify `primary_artifact` and `artifact_order` are present, accurate, and aligned with emitted filenames.

**Acceptance Scenarios**:

1. **Given** a newly emitted architecture packet, **When** `packet-metadata.json` is read, **Then** it declares the primary artifact and the ordered artifact list using the same prefixed filenames written to disk.
2. **Given** a legacy packet with unprefixed filenames, **When** compatibility metadata is consulted, **Then** Canon can still resolve packet artifacts without requiring historical run rewrites.

---

### User Story 3 - Preserve Order During Publish And Summaries (Priority: P2)

As a developer or operator, I want publish, status, and inspect surfaces to preserve the logical packet order so repo-visible and terminal-visible packet summaries stay readable after handoff.

**Why this priority**: Even if packet directories are ordered correctly, handoff quality breaks if publish indexes or summaries reorder artifacts alphabetically or hide the primary artifact.

**Independent Test**: Publish representative runs and inspect their summaries, verifying numeric prefixes are preserved, generated indexes respect declared order, and status or inspect surfaces show the primary artifact first.

**Acceptance Scenarios**:

1. **Given** a completed run with ordered packet artifacts, **When** `canon publish` emits the public packet, **Then** the published output preserves numeric prefixes and declared artifact order.
2. **Given** a completed packet, **When** status or inspect surfaces summarize it, **Then** they surface the primary artifact first and provide a deterministic ordered artifact summary.

---

### User Story 4 - Clarify Domain Language Versus Domain Model (Priority: P2)

As a maintainer, I want Canon documentation to distinguish `domain-language` from `domain-model` so downstream planning and architecture work do not confuse ubiquitous language stabilization with concept modeling.

**Why this priority**: The feature introduces per-mode packet order documentation, and the two domain-oriented modes need explicit differentiation to avoid propagating conceptual drift.

**Independent Test**: Review mode documentation and verify it states that `domain-language` is the ubiquitous-language mode, `domain-model` is the lightweight ontology or concept-model mode, and each mode has a distinct ordered artifact list.

**Acceptance Scenarios**:

1. **Given** the mode guide, **When** a reader looks up `domain-language`, **Then** the documentation describes it as the ubiquitous-language mode and lists the ordered artifacts for that packet.
2. **Given** the mode guide, **When** a reader looks up `domain-model`, **Then** the documentation describes it as the lightweight ontology or concept-model mode and lists a distinct ordered artifact sequence.

### Edge Cases

- What happens when a mode omits one or more optional artifacts, especially architecture view artifacts? Canon must emit contiguous numbering for the remaining reader-facing artifacts.
- How does the system handle legacy packets with unprefixed filenames? Compatibility metadata or alias behavior must let Canon continue to read them without rewriting historical runs.
- Which invariant is most likely to be stressed by this case? The separation between reader-facing packet artifacts and sidecars is most at risk if metadata, provenance, or support files are accidentally pulled into the ordered body sequence.
- What happens if a mode evolves to emit more artifacts than a two-digit range comfortably signals? The ordering contract must remain explicit and extensible without breaking existing packet semantics.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST define one primary artifact for every packet-emitting mode, and the primary artifact for new packets MUST use the `01-` prefix.
- **FR-002**: Canon MUST emit numeric prefixes for reader-facing markdown artifacts in new packets so directory listing order matches the intended packet reading order.
- **FR-003**: Canon MUST derive packet reading order from an explicit mode-owned sequence rather than from alphabetical sort or incidental emission order.
- **FR-004**: Canon MUST preserve contiguous numbering when optional reader-facing artifacts are omitted from a packet.
- **FR-005**: Canon MUST keep sidecar artifacts distinct from reader-facing packet-body artifacts and MUST NOT require sidecars to adopt numeric prefixes unless they are intentionally part of the packet body.
- **FR-006**: Canon packet metadata MUST expose `primary_artifact` and `artifact_order` for newly emitted packets.
- **FR-007**: Canon metadata SHOULD expose compatibility aliases or equivalent lineage for legacy packet filenames when useful to preserve backward readability.
- **FR-008**: `canon publish` MUST preserve packet numeric prefixes and logical artifact order in published outputs and generated indexes.
- **FR-009**: Canon status and inspect surfaces MUST surface the primary artifact first when summarizing a packet.
- **FR-010**: Canon mode documentation MUST define the expected ordered artifact list for every packet-emitting mode.
- **FR-011**: Canon documentation MUST explain that `domain-language` is the ubiquitous-language mode and that `domain-model` is the lightweight ontology or concept-model mode.
- **FR-012**: Canon MUST preserve the ability to read existing packets with legacy artifact names without rewriting historical governed runs.
- **FR-013**: The logical-ordering contract for new packets MUST remain applicable to future modes added after this feature.

### Key Entities *(include if feature involves data)*

- **Packet Ordering Registry**: The Canon-owned definition of the intended artifact sequence for each packet-emitting mode, including which artifact is primary and which artifacts are optional.
- **Packet Ordering Metadata**: The metadata surface that records `primary_artifact`, `artifact_order`, and any compatibility aliases or publish-order hints required to preserve reading semantics.
- **Primary Artifact Descriptor**: The contract that identifies the first artifact a reviewer or downstream consumer should open for a packet.
- **Legacy Artifact Alias Map**: The compatibility record that maps old packet names to new ordered names when Canon needs to read or summarize historical runs.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of new reader-facing packet artifacts emitted across the current packet-emitting mode catalog use explicit numeric ordering and a `01-` primary artifact.
- **SC-002**: 100% of new packets expose `primary_artifact` and `artifact_order` in packet metadata.
- **SC-003**: Published packets and generated indexes preserve declared packet order for all representative publish scenarios exercised by validation.
- **SC-004**: Mode documentation lists ordered artifact sequences for the current packet-emitting modes and explicitly distinguishes `domain-language` from `domain-model`.

## Validation Plan *(mandatory)*

- **Structural validation**: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, schema or contract checks for packet metadata where applicable.
- **Logical validation**: targeted engine, integration, contract, publish, and status or inspect tests that verify contiguous numbering, primary artifact metadata, publish-order preservation, and legacy compatibility behavior.
- **Independent validation**: reviewer pass over generated packet directories, metadata, and docs plus coverage verification on touched files.
- **Evidence artifacts**: `specs/049-logical-packet-ordering/validation-report.md`, updated decision log entries, and validation command outputs recorded during implementation.

## Decision Log *(mandatory)*

- **D-001**: Create a new 049 feature rather than mutating the earlier 046 draft, **Rationale**: the attached scope is materially broader than ordered filenames alone and deserves a new authoritative packet-ordering contract.
- **D-002**: Treat sidecars as outside the ordered packet body by default, **Rationale**: packet readability depends on separating the reader-facing narrative from supporting metadata and provenance.

## Non-Goals

- Rewriting existing governed runs to match the new ordered contract.
- Changing Canon mode semantics, approval flow, or governance adapter behavior.
- Extending project-memory promotion policy or external integration behavior as part of this feature.

## Assumptions

- New packet ordering will apply to newly emitted packets, while historical packets remain readable through compatibility behavior rather than automatic migration.
- The current packet-emitting mode catalog is the scope baseline for documentation and validation; future modes should inherit the same ordering contract.
- Canon already has sufficient packet metadata infrastructure to extend with ordering fields rather than introducing an entirely separate metadata store.
- Publish, status, and inspect surfaces can consume declared ordering metadata rather than reconstructing order from filenames alone.
