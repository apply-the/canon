# Quickstart: Semantic Artifact Contract

## Goal

Deliver a Canon-owned semantic contract that makes semantic eligibility,
provenance boundaries, and compatibility rules explicit for repo-visible
published artifacts without turning Canon into a retrieval runtime.

## Inputs

- `/specs/056-semantic-artifact-contract/spec.md`
- `/specs/056-semantic-artifact-contract/research.md`
- `/specs/056-semantic-artifact-contract/data-model.md`
- `/tech-docs/integration/project-memory-promotion-contract.md`
- `/specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`

## Implementation Steps

1. Update the feature-local contract at
   `/specs/056-semantic-artifact-contract/contracts/semantic-artifact-contract.md`
   so it names the semantic descriptor fields, supported artifact classes,
   provenance boundaries, metadata carrier expectations, and explicit rejection
   conditions.
2. Promote or prepare the stable consumer-facing contract path under
   `/tech-docs/integration/semantic-artifact-contract.md`, keeping the promotion and
   artifact-indexing contracts authoritative for publication routing and
   metadata carrier semantics.
3. If the runtime metadata shape needs implementation support, add a typed
   semantic descriptor model in the Canon engine domain and thread it through
   the existing runtime packet metadata sidecar rather than creating a new
   discovery path.
4. Update any publication or projection surfaces that expose packet metadata,
   especially the publish pipeline and CLI governance projection, so the new
   semantic contract remains inspectable.
5. Record design decisions and validation evidence under the feature directory
   before proposing merge.

## Validation Steps

1. Structural review:
   compare the semantic contract against the stable promotion contract and the
   051 artifact-indexing contract to confirm alignment on artifact classes,
   metadata carriers, and compatibility rules.
2. Logical walkthrough:
   verify one `managed-surface`, one `proposal-artifact`, one
   `evidence-bundle`, and one excluded `index-surface` scenario.
3. Build and test validation:
   run `cargo test --no-run --all-targets` and any targeted tests added for
   publish metadata or governance projection surfaces.
4. Independent validation:
   request a separate maintainer review focused on producer-boundary
   preservation and explicit consumer rejection conditions.

## Expected Outputs

- feature-local semantic contract brief
- stable integration contract update or promotion-ready draft
- aligned runtime packet metadata design
- validation report with structural, logical, and independent checks