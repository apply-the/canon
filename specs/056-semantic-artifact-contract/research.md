# Research: Semantic Artifact Contract

## Context

This feature extends Canon's producer-side metadata contract so repo-visible
Canon artifacts can participate in downstream semantic retrieval with explicit
eligibility, provenance boundaries, and compatibility rules. The research goal
was to resolve where the semantic contract should live, how it should travel
through existing Canon metadata, and which existing artifact classes should be
eligible or rejected.

## Decision 1: Promote semantic semantics through a dedicated integration contract

- Decision: Use a dedicated semantic contract surface that can eventually
  promote to `tech-docs/integration/semantic-artifact-contract.md`, while treating
  `tech-docs/integration/project-memory-promotion-contract.md` as the stable source
  for publication routing, promotion state, and metadata-carrier rules.
- Rationale: semantic eligibility and provenance are consumer-facing semantics,
  but overloading the promotion contract with retrieval-focused concerns would
  make publication and semantic ownership harder to reason about.
- Alternatives considered:
  - Keep semantic rules only in the feature-local contract: rejected because
    consumers need a stable integration path.
  - Fold semantic rules directly into the promotion contract: rejected because
    it blurs publication routing semantics with semantic eligibility semantics.

## Decision 2: Reuse the existing metadata carrier and discovery path

- Decision: Carry `semantic_descriptor` through the same packet metadata path
  already used for `artifact_indexing`, using the managed-surface envelope plus
  adjacent sidecar for `managed-surface` and `packet-metadata.json` sidecars
  for packet-style artifacts.
- Rationale: Canon already has a typed discovery path for published metadata.
  Reusing it avoids a second lookup mechanism and preserves current consumer
  discovery behavior.
- Alternatives considered:
  - Introduce a dedicated semantic sidecar: rejected because it creates a new
    discovery path and new sync risk.
  - Encode semantics only in Markdown prose: rejected because consumers must be
    able to distinguish required versus optional semantic facts without parsing
    prose.

## Decision 3: Semantic eligibility follows the existing artifact-class model

- Decision: Treat `managed-surface`, `proposal-artifact`, and
  `evidence-bundle` as semantically eligible classes, while explicitly marking
  `index-surface` as semantically excluded.
- Rationale: the first three classes can anchor a stable producer-owned unit of
  authored meaning, while index surfaces are visibility or discoverability
  aids rather than authoritative semantic content.
- Alternatives considered:
  - Make all indexable artifact classes semantically eligible: rejected because
    index surfaces would misleadingly appear suitable for retrieval ranking.
  - Leave exclusions implicit: rejected because consumers would have to infer
    semantic intent from omission.

## Decision 4: Keep provenance boundaries producer-owned and coarse enough to stay stable

- Decision: Standardize Canon-owned semantic provenance boundaries as
  `surface`, `managed_block`, and `section`, with downstream consumer
  fragments remaining out of scope for Canon ownership.
- Rationale: these boundary values match existing Canon publication shapes and
  allow consumers to derive smaller fragments while preserving a stable Canon
  provenance anchor.
- Alternatives considered:
  - Canon-owned consumer fragment identifiers: rejected because Canon must not
    own retrieval runtime state.
  - Paragraph or span-level producer boundaries: rejected because they are too
    volatile for a stable contract line.

## Decision 5: Unsupported conditions must be explicit and machine-auditable

- Decision: The contract will require consumers to reject candidates when the
  artifact class is semantically excluded, the semantic contract line is not
  supported, required semantic descriptor fields are missing, or the
  provenance reference does not resolve to a Canon-owned boundary.
- Rationale: explicit rejection conditions prevent consumers from guessing at
  Canon intent when metadata is incomplete or outside the supported surface.
- Alternatives considered:
  - Treat missing semantic metadata as soft warnings: rejected because the
    contract must make unsupported states unambiguous.
  - Rely on consumer-specific fallback behavior without Canon guidance:
    rejected because downstream rejection reasons must remain interoperable.

## Resolved Clarifications

- No unresolved `NEEDS CLARIFICATION` items remain for the planning slice.
- The existing stable promotion contract remains authoritative for publication
  target classes, update strategies, and lineage field ownership.
- The existing 051 artifact-indexing contract remains authoritative for
  artifact classes, metadata carriers, and discovery rules.
- The semantic contract adds producer-side semantic eligibility and provenance
  semantics without redefining retrieval runtime behavior.