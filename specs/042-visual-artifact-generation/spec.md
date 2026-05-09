# Feature Specification: Pragmatic C4 Architecture Packets And Visual Artifacts

**Feature Branch**: `042-visual-artifact-generation`  
**Created**: 2026-05-08  
**Status**: Draft  
**Input**: User description: "Reshape Canon architecture documentation into a pragmatic C4 packet that is less fragmented, always centers System Context plus Container views, adds Deployment and optional Dynamic or Component views only when justified, publishes one consolidated human-readable architecture document, and emits machine-readable diagram artifacts using Mermaid by default with optional SVG or PNG renderings when supported."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact, because this feature changes the architecture packet contract, publish surface, and documentation outputs without changing Canon's approval model or expanding destructive execution behavior.  
**Scope In**: architecture-mode artifact packaging, consolidated architecture handoff documents, pragmatic C4 view selection rules, machine-readable visual artifacts, optional rendered image outputs, publish or manifest guidance, and validation coverage for the new packet shape.  
**Scope Out**: automatic code-level class diagrams, mandatory image rendering in every environment, non-architecture packet redesign outside explicitly related visual follow-ons, and any change that weakens existing governance honesty around missing evidence or blocked readiness.

**Invariants**:

- Architecture packets MUST remain honest about omitted, weak, or unsupported views instead of fabricating diagrams or pretending unsupported assets exist.
- Machine-readable architecture artifacts and manifests MUST remain inspectable even if a new consolidated human-readable document becomes the primary handoff surface.

**Decision Traceability**: Decisions will be recorded in this spec and the follow-on plan artifacts under `specs/042-visual-artifact-generation/`, with validation evidence recorded in the feature's validation artifacts once implementation begins.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read One Primary Architecture Packet (Priority: P1)

As an architect or technical reviewer, I want Canon to publish one primary architecture document instead of a flat pile of equally weighted files so I can understand the system boundaries, main containers, and deployment posture without reconstructing the packet manually.

**Why this priority**: The current architecture output is technically rich but operationally fragmented. A single primary packet is the fastest way to make the existing architectural evidence usable.

**Independent Test**: Run an architecture packet against a representative system and publish it. Verify that the published output contains one primary human-readable architecture document that summarizes included views, clearly references supporting artifacts, and makes omissions explicit.

**Acceptance Scenarios**:

1. **Given** a completed architecture run, **When** the packet is published, **Then** the output contains one primary architecture document that explains the system, the included views, and where supporting artifacts live.
2. **Given** a small or medium-complexity system, **When** Canon produces the architecture packet, **Then** the primary document includes at least System Context, Container, and Deployment coverage while explicitly calling out omitted deeper levels.

---

### User Story 2 - Keep The Packet Machine-Readable (Priority: P2)

As a tooling consumer or reviewer who wants durable artifacts, I want the architecture packet to include structured diagram sources and a machine-readable view manifest so the same packet remains usable in automation, documentation portals, and review tooling.

**Why this priority**: Human readability alone is not enough for governed packets. The output has to stay inspectable, renderable, and traceable by other tools.

**Independent Test**: Publish an architecture packet and verify that each included visual view has a machine-readable source representation, that the packet manifest lists which views were included or omitted, and that any rendered assets are discoverable by format.

**Acceptance Scenarios**:

1. **Given** a publishable architecture packet, **When** visual artifacts are emitted, **Then** the packet includes machine-readable diagram sources for each included view and a manifest describing available formats.
2. **Given** an environment where rendered images are unsupported or unjustified, **When** the packet is published, **Then** the packet still ships text-based diagram sources and explicitly reports that rendered assets were omitted.

---

### User Story 3 - Generate Only The C4 Depth That Helps (Priority: P3)

As an architecture author, I want Canon to default to the pragmatic C4 set instead of generating every level mechanically so the packet communicates architecture rather than creating documentation bureaucracy.

**Why this priority**: Overproducing component-level detail makes the packet noisy, harder to maintain, and less trustworthy. The pragmatic default should be useful before it is exhaustive.

**Independent Test**: Compare a simple system brief and a complex system brief. Verify that the simple case does not emit unnecessary component or code-level outputs, while the complex case can justify deeper views with explicit rationale.

**Acceptance Scenarios**:

1. **Given** a simple architecture brief, **When** Canon shapes the packet, **Then** it omits component or dynamic views unless the brief contains evidence that they are needed.
2. **Given** a more complex architecture brief with important internal or asynchronous flows, **When** Canon shapes the packet, **Then** it can include component or dynamic views and records why they were justified.

### Edge Cases

- What happens when a system is simple enough that a component view would add noise rather than clarity?
- How does the system handle environments that can produce Mermaid sources but cannot render SVG or PNG assets reliably?
- Which invariant is most likely to be stressed by this case? The honesty invariant is stressed most when a requested visual view lacks enough evidence or rendering support.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The architecture publish surface MUST provide one primary human-readable architecture document that acts as the default handoff artifact for reviewers.
- **FR-002**: The primary architecture document MUST always cover the System Context and Container views for publishable architecture packets.
- **FR-003**: The architecture packet MUST include Deployment coverage for publishable architecture documentation, or an explicit omission reason when the authored evidence cannot support deployment claims.
- **FR-004**: Component and Dynamic views MUST be generated only when the brief or inferred complexity justifies them, and the packet MUST explain when they are intentionally omitted.
- **FR-005**: The normal architecture packet MUST NOT require Level 4 code or class diagrams as part of the default C4 output.
- **FR-006**: The architecture packet MUST emit a machine-readable diagram source format for every included visual view, with Mermaid as the default text-based diagram contract.
- **FR-007**: When rendering support is available and the generated output meets quality thresholds, the packet MAY also publish rendered SVG or PNG assets for included views.
- **FR-008**: When rendered image assets are not available, the packet MUST remain complete and usable through the primary document, the machine-readable diagram sources, and explicit capability notes.
- **FR-009**: The packet MUST include a machine-readable manifest that lists included views, omitted views, available artifact formats, and confidence or evidence limits for each visual surface.
- **FR-010**: The consolidated primary architecture document MUST remain traceable to the governed runtime artifact set rather than bypassing existing publish, inspect, or readiness semantics.
- **FR-011**: Documentation for architecture-mode outputs MUST describe the pragmatic default view set as System Context plus Container plus Deployment, with Component or Dynamic views added only when justified.

### Key Entities *(include if feature involves data)*

- **Architecture Documentation Packet**: The published architecture handoff bundle containing one primary architecture document, supporting markdown artifacts, diagram sources, optional rendered assets, and manifest metadata.
- **Visual View Artifact**: A single architectural view such as System Context, Container, Deployment, Component, or Dynamic represented as governed prose, machine-readable diagram source, and optionally rendered images.
- **View Coverage Manifest**: A machine-readable record of which views were included, which were omitted, what formats exist for each view, and what confidence or evidence limits apply.
- **Capability Note**: An explicit artifact or manifest entry that explains why a rendered image or deeper view was omitted, degraded, or deferred.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of publishable architecture packets expose one primary architecture document as the default review entrypoint.
- **SC-002**: 100% of publishable architecture packets include System Context and Container coverage, plus Deployment coverage or an explicit omission reason.
- **SC-003**: 100% of included visual views expose a machine-readable source artifact and are recorded in a machine-readable manifest.
- **SC-004**: In validation scenarios for simple systems, Component and Level 4 outputs are omitted unless explicitly justified by the authored packet or detected complexity.

## Validation Plan *(mandatory)*

- **Structural validation**: Spec checklist validation now, followed by artifact-contract, snapshot, and publish-surface checks once implementation begins.
- **Logical validation**: Architecture run and publish scenarios covering a simple system, a more complex system, rendered-assets-supported environments, and text-only fallback environments.
- **Independent validation**: Human readback of a published demo architecture packet to confirm the primary document communicates the system without requiring a reviewer to open every supporting file.
- **Evidence artifacts**: `specs/042-visual-artifact-generation/spec.md`, `specs/042-visual-artifact-generation/checklists/requirements.md`, and follow-on plan or validation artifacts created during implementation.

## Decision Log *(mandatory)*

- **D-001**: The default architecture handoff should be one primary packet with layered supporting artifacts instead of a flat list of equally weighted files, **Rationale**: this keeps the packet readable for humans while preserving structured artifacts for machines.

## Non-Goals

- Generate every C4 level for every architecture packet regardless of system complexity.
- Turn Canon into a general-purpose UML or code-diagram generator.
- Promise SVG or PNG rendering in environments that only support text-based diagram artifacts.

## Assumptions

- Architecture-mode authored packets already contain enough bounded evidence to derive at least System Context and Container views for publishable cases.
- Mermaid is an acceptable default text-based diagram representation for human-readable markdown workflows and machine-readable downstream tooling.
- Optional rendered image assets can be capability-dependent without blocking the usefulness of the architecture packet.
- Existing publish and inspect flows remain the authoritative path for exposing architecture artifacts outside `.canon/`.
