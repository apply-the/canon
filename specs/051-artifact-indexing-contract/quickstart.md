# Quickstart: Artifact Indexing Contract

## Goal

Validate that Canon exposes one stable artifact-indexing contract without
creating a second authority surface.

## Steps

1. Review `docs/integration/project-memory-promotion-contract.md` and confirm
   that it names the supported V1 artifact classes and their metadata carriers.
2. Review the derived briefs under
   `specs/051-artifact-indexing-contract/contracts/` and confirm they are
   labeled non-normative.
3. Run `cargo test -p canon-engine --lib publish_profile`.
4. Run the broader validation suite listed in
   `specs/051-artifact-indexing-contract/validation-report.md`.

## Expected Outcome

- Consumers can identify where Canon indexing metadata lives for each supported
  artifact class.
- `safety-net packets` does not appear as a supported Canon V1 artifact class.
- The stable Canon contract remains the only normative surface.