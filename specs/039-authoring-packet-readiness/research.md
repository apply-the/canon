# Research: Authoring Experience And Packet Readiness

## Decision 1: Extend `inspect clarity` Instead Of Adding A New Authoring Command

- **Decision**: Add the authored packet-lifecycle view to the existing
  `inspect clarity` summary rather than introduce a separate pre-run command.
- **Rationale**: The runtime already computes missing-context findings,
  clarification questions, source inputs, output-quality posture, and
  recommended focus. Extending that existing surface keeps the workflow
  bounded, additive, and testable without creating a second pre-run contract.
- **Alternatives considered**:
  - Add a new `inspect authoring` surface: rejected because it would fragment
    pre-run inspection and duplicate the current clarity logic.
  - Keep lifecycle guidance only in docs: rejected because the runtime would
    still leave packet-shape and readiness ambiguity implicit.

## Decision 2: Infer Packet Roles Only From Explicit Input Shape

- **Decision**: Derive authoritative-brief and supporting-input roles from the
  explicit file-backed inputs Canon already binds, with `brief.md` preferred as
  the authoritative brief when it exists in a directory-backed packet.
- **Rationale**: This matches the existing carry-forward guidance and avoids
  hidden inference from `.canon/`, published docs, or incidental files. It also
  lets ambiguous packets stay explicit rather than pretending Canon knows the
  correct source of truth.
- **Alternatives considered**:
  - Require a brand-new metadata file for every folder-backed packet: rejected
    because it would widen the authored contract and break the bounded slice.
  - Treat every file in a directory as equally authoritative: rejected because
    it weakens readiness honesty and conflicts with the current `brief.md`
    guidance.

## Decision 3: Use Shared Lifecycle Docs Rather Than Editing Every Mode Surface

- **Decision**: Align one shared lifecycle story across the central mode guide,
  shared template-facing docs, the carry-forward example, and the
  `canon-inspect-clarity` skill instead of rewriting every mode-specific skill
  and template in this slice.
- **Rationale**: Feature `039` is about making the path from weak brief to
  publishable packet explicit. The highest-leverage surfaces are the shared
  ones that already mediate pre-run authoring decisions. This keeps the change
  broad enough to matter without turning into a repo-wide copy-edit campaign.
- **Alternatives considered**:
  - Update every mode-specific skill and example: rejected for this slice
    because it would spread the work too widely before the shared runtime
    contract is settled.
  - Ship runtime-only changes with no doc unification: rejected because the
    roadmap explicitly calls for one canonical authoring story.

## Decision 4: Treat `0.39.0` As A Real Release-Line Bump

- **Decision**: Advance the workspace release line and all matching compatibility
  or release-alignment surfaces to `0.39.0` as part of feature `039`.
- **Rationale**: The user requested an explicit version-bump task, and the
  previous slice already demonstrated that release-line drift causes confusion
  if the spec, docs, and tests are not updated together.
- **Alternatives considered**:
  - Keep the existing release line and only mention `039` in docs: rejected
    because it would repeat the same ambiguity that feature `038` exposed.