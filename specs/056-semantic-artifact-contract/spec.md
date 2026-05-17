# Feature Specification: Semantic Artifact Contract

**Feature Branch**: `056-semantic-artifact-contract`  
**Created**: 2026-05-17  
**Status**: Draft  
**Input**: User description: "Define the Canon producer contract needed by Boundline S5.v2 semantic acceleration so repo-visible Canon artifacts can participate in local semantic retrieval with explicit eligibility, chunk lineage, provenance, and compatibility without turning Canon into a retrieval runtime."

## Governance Context *(mandatory)*

**Mode**: change
**Risk Classification**: bounded-impact because this slice extends Canon's
producer metadata contract for semantic eligibility and provenance without
making Canon responsible for retrieval runtime behavior, ranking, or local
index ownership
**Scope In**:

- Canon-owned semantic eligibility metadata for repo-visible published artifact
  classes that consumers may use for semantic retrieval
- Canon-defined provenance boundaries that downstream consumers must preserve
  when they fragment or locally reinterpret Canon content
- compatibility rules for additive versus breaking semantic metadata changes
- alignment between this new semantic contract, the project-memory promotion
  contract, and the existing artifact-indexing contract

**Scope Out**:

- Boundline retrieval orchestration, ranking, or stop semantics
- Canon-owned embeddings, vector indexes, or retrieval daemons
- remote provider configuration or hosted semantic services
- redefining existing V1 artifact classes outside explicit additive metadata
  changes

**Invariants**:

- Canon remains the semantic owner of published artifact metadata.
- Canon MUST NOT become the retrieval runtime or ranking authority for
  Boundline.
- Existing indexing and promotion contracts remain authoritative unless this
  slice realigns them explicitly in the same change.

**Decision Traceability**: Decisions for this slice are recorded in
`specs/056-semantic-artifact-contract/` and are intended to promote into a
stable Canon integration document once the contract is accepted.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publish One Semantic Eligibility Contract (Priority: P1)

As a Canon maintainer, I want one Canon-owned contract that states which
published artifact classes are semantically eligible and what provenance a
consumer must preserve, so downstream consumers do not infer semantic behavior
from prose or implementation details.

**Why this priority**: Without a stable semantic contract, every consumer will
invent different rules for which Canon artifacts may participate in semantic
retrieval.

**Independent Test**: A maintainer can read the feature-local contract and
determine, without source-code inspection, which Canon artifact classes are
eligible, excluded, or unsupported for semantic retrieval and what provenance
boundary applies to each.

**Acceptance Scenarios**:

1. **Given** a Canon artifact class listed in the contract, **When** a
   consumer reads the contract, **Then** the consumer can determine whether the
   class is semantically eligible and which provenance boundary it must retain.
2. **Given** an artifact class outside the supported semantic surface,
   **When** a consumer inspects the contract, **Then** the exclusion is
   explicit rather than inferred by omission.
3. **Given** a mixed-producer evidence document, **When** a consumer inspects
   the contract, **Then** it is clear which semantic metadata Canon owns and
   which consumer-side fragment logic remains outside Canon's scope.

---

### User Story 2 - Preserve Provenance Without Owning Retrieval Runtime (Priority: P1)

As a Boundline integrator, I want Canon semantic metadata to point back to the
originating Canon surface or managed block without telling Boundline how to
rank, chunk, or retrieve content, so I can preserve provenance without
transferring runtime control to Canon.

**Why this priority**: Canon must clarify producer semantics without becoming a
runtime orchestrator or retrieval policy engine.

**Independent Test**: A consumer can inspect the contract and determine how to
preserve source provenance for a semantically eligible Canon artifact while
still making its own local fragment, ranking, and fallback decisions.

**Acceptance Scenarios**:

1. **Given** a semantically eligible Canon artifact, **When** a consumer reads
   its documented semantic descriptor, **Then** the consumer can recover the
   semantic contract line, provenance reference, and provenance boundary.
2. **Given** a consumer derives local fragments from Canon content, **When** it
   uses the contract, **Then** the contract anchors those fragments to Canon
   provenance boundaries without requiring Canon to publish consumer fragment
   identifiers or ranking policy.
3. **Given** a consumer must reject a Canon candidate, **When** it reports the
   rejection, **Then** the contract supports explicit reasons such as excluded
   artifact class, unsupported contract line, or missing semantic metadata.

---

### User Story 3 - Version Semantic Metadata Safely (Priority: P2)

As a cross-repo maintainer, I want additive semantic metadata to stay backward
compatible and breaking changes to require a new contract line, so Boundline
can adopt semantic retrieval safely without destabilizing existing Canon
indexing consumers.

**Why this priority**: The contract only helps if consumers can tell which
changes are safe to ignore and which changes require explicit adoption.

**Independent Test**: A maintainer can read the compatibility rules and decide
whether a semantic metadata addition, removal, or rename is compatible with the
current contract line.

**Acceptance Scenarios**:

1. **Given** Canon adds a new optional semantic metadata field, **When** a V1
   consumer reads the compatibility rules, **Then** it can ignore the field
   without reinterpreting required semantics.
2. **Given** Canon removes or renames a required semantic field, **When** the
   change is proposed, **Then** the contract requires a new major contract line.
3. **Given** a consumer only supports the existing artifact-indexing contract,
   **When** Canon publishes semantic metadata additively, **Then** the existing
   consumer can remain on the older contract without semantic ambiguity.

### Edge Cases

- A Canon artifact is indexable under the existing V1 indexing contract but is
  explicitly excluded from semantic retrieval in the new semantic contract.
- A managed surface contains Canon and non-Canon blocks, and only the Canon
  portion is eligible for Canon-owned semantic metadata.
- A consumer derives smaller local fragments than Canon's semantic provenance
  boundary, and the contract must still keep upstream provenance stable.
- A future Canon change adds descriptive semantic labels but does not change
  eligibility or provenance meaning.
- A consumer encounters semantic metadata on an unsupported contract line and
  must reject it without guessing at Canon intent.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST publish a Canon-owned semantic artifact contract that
  extends, rather than replaces, the current project-memory promotion and
  artifact-indexing contracts.
- **FR-002**: Canon MUST define which supported repo-visible artifact classes
  are semantically eligible, semantically excluded, or outside the semantic
  contract surface.
- **FR-003**: Canon MUST define the provenance boundary a consumer is required
  to preserve for each semantically eligible artifact class.
- **FR-004**: Canon MUST define a typed semantic descriptor carried through the
  documented metadata carrier with these required facts: semantic contract
  line, semantic eligibility, semantic provenance boundary, and semantic
  provenance reference.
- **FR-005**: Canon MUST allow additive optional semantic labels or descriptive
  fields without changing the meaning of required semantic metadata.
- **FR-006**: Canon MUST state that consumers may derive local fragments from
  eligible Canon content, but Canon does not own consumer fragment identities,
  ranking policy, or retrieval policy.
- **FR-007**: Canon MUST define how mixed-producer surfaces expose Canon-owned
  semantic metadata without claiming ownership over non-Canon content.
- **FR-008**: Canon MUST define explicit unsupported or incompatible conditions
  for semantic consumers, including excluded artifact classes, missing required
  semantic metadata, and unsupported contract lines.
- **FR-009**: Canon MUST treat additive semantic metadata as backward-compatible
  and MUST require a new major contract line for removing or renaming required
  semantic fields.
- **FR-010**: Canon MUST preserve the boundary that semantic metadata is
  producer semantics only and MUST NOT define retrieval runtime behavior,
  vector index ownership, ranking policy, or remote-provider policy.
- **FR-011**: Canon MUST align this semantic contract with the existing stable
  project-memory promotion contract and the artifact-indexing contract so the
  three surfaces do not diverge.

### Key Entities *(include if feature involves data)*

- **Semantic Artifact Descriptor**: The Canon-owned metadata payload that tells
  consumers whether an artifact is semantically eligible and what provenance
  they must preserve.
- **Semantic Provenance Boundary**: The Canon-defined unit of authored meaning
  a consumer must retain when it locally fragments or reinterprets Canon
  content.
- **Eligible Artifact Class**: A supported Canon artifact class that may be
  consumed for semantic retrieval under the documented contract line.
- **Consumer Fragment**: A downstream local fragment derived from eligible
  Canon content that remains anchored to a Canon provenance boundary without
  becoming Canon-owned producer output.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A maintainer can identify the semantically eligible Canon
  artifact classes, exclusions, and provenance boundaries from the contract in
  under 10 minutes.
- **SC-002**: The contract leaves zero ambiguous semantic eligibility states
  for the supported artifact classes it names.
- **SC-003**: A consumer can determine required versus optional semantic
  metadata without reading Canon implementation code.
- **SC-004**: The contract clarifies semantic producer metadata without adding
  Canon retrieval-runtime responsibilities.

## Validation Plan *(mandatory)*

- **Structural validation**: Compare the semantic contract against the stable
  project-memory promotion contract and the feature-local artifact-indexing
  contract to confirm field-name, ownership, and compatibility alignment.
- **Logical validation**: Walk through one eligible artifact, one excluded
  artifact, one mixed-producer surface, and one unsupported contract-line
  scenario.
- **Independent validation**: Review by a separate maintainer who confirms the
  contract preserves Canon's producer boundary while remaining consumable by
  Boundline.
- **Evidence artifacts**: This `spec.md`, the feature-local contract brief,
  comparison notes against Canon 051 and the stable promotion contract, and the
  specification quality checklist.

## Decision Log *(mandatory)*

- **D-001**: Canon defines semantic eligibility and provenance, not consumer
  chunking or ranking, **Rationale**: producer semantics must remain stable
  without turning Canon into the retrieval runtime.
- **D-002**: Semantic metadata extends the existing indexing surface
  additively, **Rationale**: Boundline must be able to adopt the contract
  without destabilizing consumers that only need the V1 indexing baseline.

## Non-Goals

- Defining Boundline semantic ranking, fragment scoring, or fallback policy
- Publishing Canon-owned embeddings, vector indexes, or retrieval daemons
- Replacing the existing project-memory promotion contract or the artifact-
  indexing contract
- Turning Canon into the retrieval orchestrator for Boundline or any other
  consumer

## Assumptions

- Boundline S5.v2 will derive local semantic fragments itself and needs Canon
  only for semantic eligibility and provenance semantics.
- Existing V1 artifact classes remain the starting point for semantic
  eligibility instead of being replaced by a new artifact taxonomy.
- A future stable Canon integration path will promote this feature-local
  contract once the field set and ownership rules are accepted.
