# Research: Requirements PRD Publishing And Chat Publish Skill

## Decision 1: Add `prd.md` to the requirements artifact contract

- **Decision**: Extend `Mode::Requirements` with an additive `prd.md` artifact instead of building a special publish-only post-processing path.
- **Rationale**: The generic publish pipeline already copies every persisted artifact. Adding `prd.md` to the contract lets generation, inspection, persistence, and publish stay consistent without a second artifact lifecycle.
- **Alternatives considered**:
  - Generate `prd.md` only during publish: rejected because inspect and artifact manifests would remain incomplete and the feature would split artifact truth across two phases.
  - Replace the sectional files with one PRD: rejected because downstream readers and tests already rely on the current packet structure.

## Decision 2: Render the consolidated PRD from the same authored evidence used by the sectional files

- **Decision**: Synthesize `prd.md` from the existing authored requirements sections and the same missing-body rules already used by `render_requirements_artifact_from_evidence()`.
- **Rationale**: Users want one readable product-facing document, but Canon still needs honest gaps and consistent section content. Reusing the current section extraction semantics preserves trustworthiness.
- **Alternatives considered**:
  - Use a separate PRD template disconnected from the sectional files: rejected because it risks drift and duplicated business logic.
  - Concatenate the already-rendered sectional files at publish time: rejected because it makes section ordering and summary framing harder to control and obscures authored-section provenance.

## Decision 3: Keep publish chat support as a repo-local skill, not a new runtime command

- **Decision**: Add a repo-local `canon-publish` skill mirrored into embedded skills for chat-first environments.
- **Rationale**: Canon already exposes the runtime command as `canon publish`; the missing piece is discoverable chat guidance, not a second execution path.
- **Alternatives considered**:
  - Add a chat-only adapter endpoint for publish: rejected because it duplicates CLI behavior and weakens the local-first contract.
  - Hide publish under an existing generic status or inspect skill: rejected because the final lifecycle step deserves a direct trigger surface.

## Decision 4: Preserve publish metadata and destination semantics additively

- **Decision**: Keep the existing default destination leaf and `packet-metadata.json` schema, only allowing the new PRD source and published file paths to appear additively.
- **Rationale**: Existing consumers can tolerate one extra artifact more safely than renamed files or metadata shape changes.
- **Alternatives considered**:
  - Rename the requirements publish directory to `prd`: rejected because it would silently break established expectations.
  - Add PRD-specific metadata fields immediately: rejected because the current metadata already covers source and destination traceability.

## Decision 5: Treat documentation and version alignment as part of the same slice

- **Decision**: Update README, mode guidance, changelog, and version metadata in the same feature as the code and skill changes.
- **Rationale**: The user-facing pain is partly runtime UX and partly discoverability; shipping only code would leave the same confusion in place.
- **Alternatives considered**:
  - Defer docs or version alignment to follow-up cleanup: rejected because the requested slice is explicitly end-to-end and release-facing.