# Implementation Plan: Project Memory And Delivery Control Contracts

**Branch**: `050-project-memory-control` | **Date**: 2026-05-13 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/050-project-memory-control/spec.md`

## Summary

Clarify and publish the Canon-owned contract bundle for repo-visible project
memory and delivery-control integration. This slice keeps Canon as the sole
owner of producer semantics while adding a stable consumer path,
producer-neutral managed blocks, a lighter V1 lineage contract, explicit target
mapping for `docs/project/` and `docs/evidence/`, and stronger compatibility
rules that Boundline can consume without redefining Canon behavior. The stable
contract path remains normative; feature-local contracts only elaborate that
source with examples and supporting shapes.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact; the slice changes the canonical
cross-repo contract surface and repo-visible publication semantics without
moving orchestration into Canon or changing `.canon/` runtime ownership.
**Scope In**:
- stable Canon documentation path for the owner-side contract bundle
- feature-local contract documents for promotion, governed stage refs,
  promotion events, and evidence refs
- producer-neutral managed-block markers for repo-visible docs
- minimum required V1 lineage fields and optional metadata split
- compatibility policy for additive versus breaking contract evolution
- default target mapping for `docs/project/` and `docs/evidence/`

**Scope Out**:
- Boundline workflow registry and planner behavior
- Boundline project-index and cluster semantics
- provider-runtime readiness, voting, or lifecycle-hook engines
- Backstage or TechDocs integration
- Canon as a delivery orchestrator

**Invariants**:

- `.canon/` remains the authoritative governed runtime and evidence store
- Canon owns producer-side contract semantics, promotion policy, and lineage generation
- consumers may depend on Canon contracts but may not redefine Canon semantics
- mixed Canon and Boundline evidence authorship does not transfer contract ownership away from Canon

**Decision Log**: `specs/050-project-memory-control/decision-log.md`  
**Validation Ownership**: The implementation generates contract docs and any
producer-side code changes; validation comes from contract review, `cargo fmt
--check`, `cargo clippy --workspace --all-targets --all-features -- -D
warnings`, focused publish-profile tests, and `cargo nextest run`.  
**Approval Gates**: Human review of the canonical contract text and any changed
producer-side policy behavior before merge.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024  
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `thiserror`,
`tracing`, `uuid`, `toml`  
**Storage**: local filesystem under `.canon/` plus repo-visible docs under
`docs/project/`, `docs/evidence/`, and `docs/integration/`  
**Testing**: `cargo test`, `cargo nextest run`, `cargo llvm-cov`, `cargo fmt
--check`, `cargo clippy --workspace --all-targets --all-features -- -D
warnings`  
**Target Platform**: macOS, Linux, and Windows developer workstations and CI  
**Project Type**: Rust CLI plus workspace libraries  
**Existing System Touchpoints**: `crates/canon-engine/src/domain/publish_profile.rs`,
`crates/canon-engine/src/orchestrator/publish.rs`,
`docs/integration/project-memory-promotion-contract.md`,
`specs/048-project-memory-promotion-policy/contracts/` as prior design context only,
and any docs or tests that assert current promotion semantics  
**Performance Goals**: no measurable runtime-latency target; changes must not
materially degrade existing publish-path responsiveness  
**Constraints**: preserve Canon ownership boundaries; do not change `.canon/`
runtime authority; do not require Boundline-specific code paths to understand
Canon-owned semantics  
**Scale/Scope**: 1 stable contract path, 1 feature-local contract bundle, 4
shared contract shapes, and existing publish-policy surfaces already shipped in
Canon 0.48.0+

`specs/048-project-memory-promotion-policy/contracts/` is treated as prior design
context. The canonical contract for this slice is owned by
`specs/050-project-memory-control/` and the stable published path under
`docs/integration/`.

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
specs/050-project-memory-control/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── contracts/
│   ├── project-memory-promotion-contract.md
│   ├── governed-stage-ref-contract.md
│   ├── promotion-event-contract.md
│   └── evidence-ref-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── domain/
│       │   └── publish_profile.rs
│       └── orchestrator/
│           └── publish.rs
docs/
└── integration/
    └── project-memory-promotion-contract.md
specs/
└── 048-project-memory-promotion-policy/
    └── contracts/ (prior design context only)
```

**Structure Decision**: Stay inside the existing Canon workspace and contract
documentation surfaces. This slice primarily changes stable docs, feature-local
contracts, and targeted producer-side policy touchpoints rather than adding new
crates or top-level systems.

## Complexity Tracking

No constitution violations identified.
