# Research: Mode Context Split

## Decision 1: Model `system_context` as a first-class typed runtime field

- **Decision**: Introduce a dedicated `SystemContext` enum with values `new` and `existing`, thread it through `RunRequest`, persisted run context, manifest-facing summaries, gate contexts, and inspectable outputs.
- **Rationale**: A typed field removes the semantic overload currently embedded in `brownfield-change`, makes the two-axis model explicit, and keeps future policy and gate use consistent without inventing inferred defaults.
- **Alternatives considered**:
  - Keep system state implicit inside mode names: rejected because that is the inconsistency the feature exists to remove.
  - Store `system_context` as an untyped free-form string: rejected because it weakens validation and invites silent drift in persisted state and CLI parsing.

## Decision 2: Remove `brownfield-change` from every public contract, not only the mode enum

- **Decision**: Rename the public mode to `change` and update public gate labels, skill names, canonical input hints, artifact namespaces, docs, and inspect surfaces in the same tranche.
- **Rationale**: `brownfield` is still part of the public API if it survives in gate names, skill entry points, file hints, or artifact paths. The semantic cleanup is incomplete unless users can learn the new model without encountering legacy jargon.
- **Alternatives considered**:
  - Alias `brownfield-change` to `change`: rejected because the feature explicitly allows breaking changes and requires removal of legacy public naming.
  - Leave internal or semi-public labels like `BrownfieldPreservation` exposed: rejected because gate names and skill names surface in user-visible outputs and approvals.

## Decision 3: Preserve brownfield behavior only through `change + existing`

- **Decision**: Move the current brownfield step sequence, artifact contract, preserved-behavior expectations, and readiness logic onto `change` runs where `system_context = existing`, and explicitly reject `change + new` before run creation.
- **Rationale**: This preserves the bounded-change workflow users already rely on while keeping the new model minimal and semantically precise. Supporting `change + new` now would introduce a second meaning for the same mode before there is evidence that Canon needs it.
- **Alternatives considered**:
  - Allow `change + new` with a thin implementation: rejected because it would blur the meaning of `change` and weaken invariants.
  - Split into separate `change-existing` and `change-new` modes: rejected because it recreates the same naming problem on a new axis.

## Decision 4: Treat required and optional context as a mode-level contract matrix

- **Decision**: Enforce a static required-versus-optional matrix in classifier and CLI validation: `system-shaping`, `architecture`, `change`, `implementation`, `refactor`, `migration`, and `incident` require explicit `system_context`; `discovery`, `requirements`, `review`, `verification`, and `pr-review` may omit it and persist `None` without invented defaults.
- **Rationale**: The contract is easy to explain, easy to test, and future-proof for modeled modes that already have a clear dependence on system state even if they are not full-depth today.
- **Alternatives considered**:
  - Infer context from mode or input content: rejected because the feature explicitly forbids silent defaults.
  - Require `system_context` for every mode: rejected because exploratory and review-only workflows do not always have a meaningful system-state dimension.

## Decision 5: Rename canonical input and artifact paths without dual-writing

- **Decision**: Move bounded-change authored input to `canon-input/change.md` or `canon-input/change/` and move emitted artifacts to `.canon/artifacts/<RUN_ID>/change/`, with no compatibility copy or alias for old paths.
- **Rationale**: The feature permits breaking changes, and dual-writing would complicate persistence and inspection logic while leaving the old vocabulary alive in practice.
- **Alternatives considered**:
  - Maintain both old and new paths temporarily: rejected because it prolongs ambiguity and doubles validation surface.
  - Migrate historical runs in place: rejected because the feature is scoped to future runtime behavior, not to retroactive repository history rewrites.

## Decision 6: Coverage recovery is part of the design, not post-implementation cleanup

- **Decision**: Add targeted contract and integration tests for mode parsing, required-context enforcement, invalid combinations, persisted `context.toml`, renamed artifact paths, inspect output, gate evaluation, adapter prompt summaries, and CLI output formatting, then validate with workspace coverage.
- **Rationale**: The current failing patch coverage shows the touched runtime branches are under-tested. Because the feature modifies public contracts and internal semantics simultaneously, the safest implementation strategy is to grow tests alongside the refactor.
- **Alternatives considered**:
  - Rely on `cargo nextest run` alone: rejected because broad regression suites do not guarantee adequate branch coverage in the changed files.
  - Add only one new end-to-end change test: rejected because the coverage gaps are spread across classifier, gatekeeper, persistence, output rendering, and adapter surfaces.
