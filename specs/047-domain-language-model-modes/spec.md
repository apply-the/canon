# Feature Specification: Domain Language And Domain Model Modes

**Feature Branch**: `047-domain-language-model-modes`
**Created**: 2026-05-12
**Status**: Draft
**Input**: User description: "Add domain-language and domain-model as first-class governed Canon modes for ubiquitous language packets and lightweight ontology concept model packets"

## Governance Context

**Mode**: change
**Risk Classification**: systemic-impact: introduces two new first-class modes that add Mode enum variants, artifact contracts, renderers, gatekeepers, summarizers, publish targets, CLI surface, governance adapter capabilities, skills, templates, and examples across the entire Canon workspace.
**Scope In**: Add `domain-language` and `domain-model` as first-class Canon modes with full artifact contracts, renderers, gatekeepers, summarizers, publish targets, governance adapter support, `inspect clarity` support, CLI integration, skills, templates, and examples.
**Scope Out**: Full semantic-web ontology systems; RDF/OWL/SHACL; replacing existing modes; requiring domain-language or domain-model for every feature; producing implementation patches; localization/multilingual support; Boundline integration; `--from-run` seeding.

**Invariants**:

- All existing modes must continue to pass all tests unchanged.
- Artifact contracts must follow the established ordinal-prefixed filename convention.
- Missing authored sections must produce explicit `## Missing Authored Body` markers, never invented content.
- Both modes must appear in `canon governance capabilities --json`.
- Both modes must support `canon inspect clarity` for file-backed inputs.

**Decision Traceability**: `specs/047-domain-language-model-modes/`

## User Scenarios & Testing

### User Story 1 - Run domain-language mode (Priority: P1)

A domain lead wants to stabilize the shared vocabulary of a product area before downstream requirements, architecture, or change work. They author a brief at `canon-input/domain-language.md` with domain scope, candidate terms, ambiguities, and preferred language, then run `canon run --mode domain-language`.

**Why this priority**: This is the primary capability: producing a governed ubiquitous-language packet from authored input.

**Independent Test**: Run `canon run --mode domain-language` with a complete brief and verify all 10 artifacts are emitted with ordinal prefixes and correct content structure.

**Acceptance Scenarios**:

1. **Given** a workspace with `canon-input/domain-language.md` containing required authored sections, **When** `canon run --mode domain-language --risk bounded-impact --zone yellow --owner domain-lead --input canon-input/domain-language.md` is executed, **Then** Canon emits a governed packet with all 10 domain-language artifacts under `.canon/artifacts/`.
2. **Given** the same run, **When** the output is inspected, **Then** each artifact filename carries an ordinal prefix (e.g., `01-language-overview.md`, `02-domain-glossary.md`).

---

### User Story 2 - Run domain-model mode (Priority: P1)

An architect wants to formalize domain concepts, relationships, invariants, and feature-impact rules before architecture or backlog decomposition. They author a brief at `canon-input/domain-model.md` and run `canon run --mode domain-model`.

**Why this priority**: This is the second core capability, paired with domain-language.

**Independent Test**: Run `canon run --mode domain-model` with a complete brief and verify all 13 artifacts are emitted including the machine-readable `domain-model.json`.

**Acceptance Scenarios**:

1. **Given** a workspace with `canon-input/domain-model.md` containing required authored sections, **When** `canon run --mode domain-model --system-context existing --risk systemic-impact --zone red --owner architect --input canon-input/domain-model.md` is executed, **Then** Canon emits a governed packet with all 13 domain-model artifacts.
2. **Given** the completed run, **When** `domain-model.json` is inspected, **Then** it contains valid JSON with `schema_version`, `concepts`, `relationships`, `invariants`, and `feature_impact_rules` fields.

---

### User Story 3 - Publish domain packets (Priority: P2)

After a successful domain-language or domain-model run, the user publishes the packet to the default destination.

**Why this priority**: Publishing is essential for the packet to be useful outside `.canon/`.

**Independent Test**: Run a domain-language or domain-model run, then `canon publish <RUN_ID>` and verify files land in the correct default destination.

**Acceptance Scenarios**:

1. **Given** a completed domain-language run, **When** `canon publish <RUN_ID>` is executed, **Then** artifacts are published to `docs/domain/language/<YYYY-MM-DD>-<descriptor>/`.
2. **Given** a completed domain-model run, **When** `canon publish <RUN_ID>` is executed, **Then** artifacts are published to `docs/domain/model/<YYYY-MM-DD>-<descriptor>/`.

---

### User Story 4 - Inspect clarity for domain modes (Priority: P2)

A user wants to check their domain-language or domain-model brief before running to see what is missing or weak.

**Why this priority**: Inspect clarity support is required for all file-backed modes.

**Independent Test**: Run `canon inspect clarity --mode domain-language --input canon-input/domain-language.md` and verify structured output.

**Acceptance Scenarios**:

1. **Given** an incomplete domain-language brief, **When** `canon inspect clarity` is run, **Then** missing sections are identified and a quality posture is assigned.
2. **Given** a complete domain-model brief, **When** `canon inspect clarity` is run, **Then** the packet is classified as `structurally-complete` or higher.

---

### User Story 5 - Governance adapter support (Priority: P3)

An external orchestrator queries Canon's governance adapter for available capabilities and sees both domain modes listed.

**Why this priority**: Required for machine-facing integration but lower priority than core mode functionality.

**Independent Test**: Run `canon governance capabilities --json` and verify both modes appear in the output.

**Acceptance Scenarios**:

1. **Given** a Canon workspace, **When** `canon governance capabilities --json` is executed, **Then** both `domain-language` and `domain-model` appear in the capabilities list.

---

### Edge Cases

- What happens when required authored sections are missing? Canon must emit `## Missing Authored Body` markers.
- What happens when `domain-model` is run without an upstream `domain-language` packet? Canon must record language uncertainty explicitly instead of failing.
- What happens when `--system-context` is omitted for `domain-model`? Both `new` and `existing` should be accepted; omission defaults based on risk/mode policy.

## Requirements

### Functional Requirements

- **FR-001**: Canon MUST support `domain-language` as a first-class mode with all 10 artifacts defined in the product spec.
- **FR-002**: Canon MUST support `domain-model` as a first-class mode with all 13 artifacts defined in the product spec.
- **FR-003**: Both modes MUST accept input from `canon-input/<mode>.md` or `canon-input/<mode>/` with folder-backed packet support.
- **FR-004**: Both modes MUST preserve missing authored sections as explicit `## Missing Authored Body` blocks.
- **FR-005**: `domain-model` MUST emit `domain-model.json` as a stable machine-readable concept model.
- **FR-006**: Both modes MUST publish to their specified default targets (`docs/domain/language/` and `docs/domain/model/`).
- **FR-007**: Both modes MUST be included in `canon inspect clarity` for file-backed inputs.
- **FR-008**: Both modes MUST appear in `canon governance capabilities --json`.
- **FR-009**: Both modes MUST support `--system-context new|existing`, `--risk`, `--zone`, and `--owner` metadata.
- **FR-010**: Artifact filenames MUST follow the ordinal-prefix convention (e.g., `01-language-overview.md`).

### Key Entities

- **Mode::DomainLanguage**: Governed ubiquitous-language packet mode producing 10 artifacts.
- **Mode::DomainModel**: Governed ontology/concept model packet mode producing 13 artifacts including a JSON sidecar.

## Success Criteria

### Measurable Outcomes

- **SC-001**: `canon run --mode domain-language` produces all 10 artifacts with correct structure.
- **SC-002**: `canon run --mode domain-model` produces all 13 artifacts including valid `domain-model.json`.
- **SC-003**: `canon publish` places packets in the correct default destination directories.
- **SC-004**: `canon inspect clarity` returns structured quality assessment for both modes.
- **SC-005**: `canon governance capabilities --json` lists both new modes.
- **SC-006**: All existing 352+ tests continue to pass without modification.
- **SC-007**: New code maintains or improves the existing ~90% line coverage baseline.

## Validation Plan

- **Structural validation**: `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo fmt --check`
- **Logical validation**: Contract tests for artifact counts and names; integration tests for full runs; publish target tests; governance adapter capability tests; inspect clarity tests; unit tests for renderers and gatekeepers.
- **Independent validation**: `cargo nextest run` full suite must pass green including all new and existing tests.
- **Evidence artifacts**: Test results, coverage report, spec directory under `specs/047-domain-language-model-modes/`.

## Decision Log

- **D-001**: Both modes are separate Canon modes rather than sub-modes of an existing mode. **Rationale**: They answer fundamentally different questions (vocabulary vs. structure) and are useful at different maturity levels.
- **D-002**: `domain-model.json` uses a simple `schema_version: "1"` contract. **Rationale**: Avoid premature semantic-web complexity; keep the JSON shape practical and delivery-oriented.
- **D-003**: `--system-context` is optional for `domain-language` and supported for `domain-model`. **Rationale**: Language stabilization applies regardless of new/existing context; model formalization benefits from explicit context classification.

## Non-Goals

- Building a full semantic-web ontology system (RDF, OWL, SHACL, knowledge graphs).
- Requiring every feature to use domain-language or domain-model.
- Pretending unresolved language conflicts are resolved.
- Producing implementation patches from either mode.
- Certifying legal, compliance, or security correctness.
- `--from-run` seeding of domain-model from a published domain-language packet.
- Localization or multilingual support.
- Boundline integration consuming `domain-model.json`.

## Assumptions

- The existing Canon mode infrastructure (Mode enum, contract_for_mode, renderers, gatekeepers, summarizers, publish) supports adding new modes by following established patterns.
- Both modes follow the same recommendation-only persona boundary as other governed modes.
- The ordinal-prefix convention from 046 applies to all new artifact filenames.
- Templates and examples will follow established patterns in `docs/templates/` and `docs/examples/`.
