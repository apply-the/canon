# Validation Report: Semantic Artifact Contract

## Validation Ownership

- Generation owner: feature author updating the contract brief, stable contract
  draft, runtime metadata model, and projection alignment.
- Validation owner: independent maintainer reviewing contract alignment,
  running scenario walkthroughs, and confirming that producer semantics do not
  become retrieval runtime behavior.

## Structural Validation

1. Compare `/specs/056-semantic-artifact-contract/contracts/semantic-artifact-contract.md`
   with `/tech-docs/integration/project-memory-promotion-contract.md` to confirm
   semantic metadata does not redefine publication target classes, update
   strategies, lineage ownership, or metadata carriers.
2. Compare the semantic contract with
   `/specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`
   and the typed metadata models in
   `/crates/canon-engine/src/domain/artifact.rs` and
   `/crates/canon-engine/src/domain/publish_profile.rs` to confirm artifact
   classes, carrier names, and discovery rules stay aligned.
3. Verify that any new JSON-facing semantic metadata is modeled with typed
   Rust structs and `serde` derives rather than ad hoc map assembly.

## Logical Validation

Validate these scenarios end to end:

1. `managed-surface` is semantically eligible and resolves to a Canon-owned
   surface or managed-block provenance boundary.
2. `proposal-artifact` is semantically eligible and uses the existing packet
   metadata carrier.
3. `evidence-bundle` is semantically eligible only when Canon can point to a
   stable evidence-facing provenance boundary.
4. `index-surface` is explicitly excluded from semantic retrieval.
5. A mixed-producer readable document applies Canon semantic metadata only to
   Canon-owned content.
6. An unsupported semantic contract line or missing required semantic field is
   rejected explicitly instead of being interpreted heuristically.

## Build And Test Validation

- `cargo test --no-run --all-targets`
- `cargo nextest run --workspace --all-features`
- targeted tests for any new publish metadata, packet metadata, or CLI
  governance projection behavior added during implementation

## Independent Validation

The independent reviewer must confirm all of the following:

- Canon remains the producer authority for semantic metadata only.
- Boundline or other consumers retain ownership of local fragments, ranking,
  and retrieval runtime behavior.
- Compatibility rules clearly distinguish additive changes from breaking
  required-field or meaning changes.
- Rejection conditions are explicit for excluded classes, unsupported contract
  lines, and missing required semantic metadata.

## Evidence Artifacts

- `/specs/056-semantic-artifact-contract/spec.md`
- `/specs/056-semantic-artifact-contract/plan.md`
- `/specs/056-semantic-artifact-contract/research.md`
- `/specs/056-semantic-artifact-contract/data-model.md`
- `/specs/056-semantic-artifact-contract/decision-log.md`
- `/specs/056-semantic-artifact-contract/contracts/semantic-artifact-contract.md`
- diff or review notes for `/tech-docs/integration/project-memory-promotion-contract.md`
  and the eventual stable semantic contract path

## Exit Criteria

- No unresolved contract-shape ambiguity remains for supported artifact
  classes.
- Metadata carrier and discovery rules remain aligned with the existing 051
  artifact-indexing contract.
- Semantic producer semantics remain additive and do not turn Canon into a
  retrieval runtime.