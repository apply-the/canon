# Implementation Plan: Semantic Artifact Contract

**Branch**: `056-semantic-artifact-contract` | **Date**: 2026-05-17 | **Spec**: `/specs/056-semantic-artifact-contract/spec.md`
**Input**: Feature specification from `/specs/056-semantic-artifact-contract/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Define a Canon-owned semantic artifact contract that layers a typed
`semantic_descriptor` onto the existing project-memory promotion and
artifact-indexing surfaces without introducing a second discovery path or
turning Canon into a retrieval runtime. The implementation approach is to keep
publication routing and indexing semantics owned by the existing stable
contracts, document semantic eligibility and provenance boundaries per
artifact class, and wire any required metadata shape updates through the
existing runtime packet metadata sidecar and projection surfaces.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact because this slice extends Canon-owned
producer metadata and integration documentation for semantic eligibility and
provenance while preserving the current publication, indexing, and runtime
ownership boundaries.
**Scope In**:

- feature-local semantic contract and supporting design artifacts under
  `specs/056-semantic-artifact-contract/`
- alignment between semantic eligibility metadata, the stable project-memory
  promotion contract, and the 051 artifact-indexing contract
- typed metadata carrier expectations for semantic descriptor transport through
  Canon-owned packet metadata
- compatibility rules and rejection conditions for unsupported semantic
  contract lines or missing required semantic metadata

**Scope Out**:

- Boundline retrieval orchestration, ranking, chunking, or fallback behavior
- Canon-owned embeddings, vector indexes, retrieval daemons, or runtime
  registries
- new publish destinations outside current repo-visible docs and evidence
  surfaces
- redefining existing publication target classes or update strategies except
  where semantic alignment must reference them

**Invariants**:

- Canon remains the producer authority for semantic metadata, while Boundline
  and other consumers retain ownership of local fragment, ranking, and
  retrieval runtime behavior.
- Existing project-memory promotion and artifact-indexing contracts remain
  authoritative for publication routing, metadata carriers, and compatibility
  unless this slice explicitly realigns them in the same change.
- Semantic metadata must travel through existing Canon-owned metadata carriers
  instead of creating an independent discovery path.

**Decision Log**: `/specs/056-semantic-artifact-contract/decision-log.md`  
**Validation Ownership**: Generation is owned by the feature author through
the contract brief, plan, data model, and runtime metadata alignment notes;
validation is owned by an independent maintainer who performs contract diff
review, scenario walkthroughs, and metadata-shape checks against the stable
promotion and indexing surfaces.  
**Approval Gates**: No additional pre-implementation approval gate is required
for bounded-impact planning; merge readiness depends on independent maintainer
review of the contract alignment and validation report.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024, plus Markdown and JSON
integration contracts  
**Primary Dependencies**: existing workspace crates `canon-engine`,
`canon-cli`, and `canon-adapters`; `serde`, `serde_json`, `serde_yaml`,
`toml`, `thiserror`, `tracing`, `uuid`, and `time` for typed metadata and
published contract surfaces  
**Storage**: repository-published Markdown contracts under `docs/integration/`
and `specs/056-semantic-artifact-contract/`, plus JSON sidecars carried through
`packet-metadata.json` and adjacent published surface sidecars  
**Testing**: `cargo test`, `cargo nextest run`, contract-structure review, and
logical scenario walkthroughs recorded in `validation-report.md`  
**Target Platform**: local filesystem and repo-visible Markdown/JSON surfaces
consumed by Canon maintainers and downstream tools on macOS/Linux CI  
**Project Type**: Rust workspace CLI/engine/adapters project with published
integration contracts  
**Existing System Touchpoints**: `docs/integration/project-memory-promotion-contract.md`,
`specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`,
`specs/056-semantic-artifact-contract/contracts/semantic-artifact-contract.md`,
`crates/canon-engine/src/domain/artifact.rs`,
`crates/canon-engine/src/domain/publish_profile.rs`,
`crates/canon-engine/src/orchestrator/publish.rs`, and
`crates/canon-cli/src/commands/governance/projection.rs`  
**Performance Goals**: no new runtime throughput target; semantic metadata
lookup must remain deterministic, human-auditable, and compatible with the
existing packet metadata projection flow  
**Constraints**: preserve producer-only ownership boundaries, avoid introducing
new metadata carriers, keep semantic changes additive where possible, and honor
the repository Rust language rules against panic-prone control flow, magic
literals in owned logic, and unstable ad hoc serialization  
**Scale/Scope**: one semantic contract line covering four existing published
artifact classes, three semantically eligible surfaces, one explicitly excluded
surface, and one shared metadata transport path

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/056-semantic-artifact-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── semantic-artifact-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   ├── artifact.rs
│       │   └── publish_profile.rs
│       └── orchestrator/
│           └── publish.rs
├── canon-cli/
│   └── src/
│       └── commands/
│           └── governance/
│               └── projection.rs
└── canon-adapters/

docs/
└── integration/
  ├── project-memory-promotion-contract.md
  └── semantic-artifact-contract.md

specs/
├── 051-artifact-indexing-contract/
└── 056-semantic-artifact-contract/

tests/
├── contract/
└── integration/
```

**Structure Decision**: Keep this slice anchored in the existing Rust workspace
and integration-document layout. The implementation work is concentrated in
`crates/canon-engine/src/domain/`, `crates/canon-engine/src/orchestrator/`, the
CLI governance projection surface, and the integration/feature contract docs so
the semantic contract stays aligned with existing publish and indexing
semantics.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | Constitution gates pass without deviation for this planning slice. |
