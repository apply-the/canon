# Research: Standard ADR Publish Artifacts

## Decision 1: Generate ADRs at publish time from persisted packet artifacts

- **Decision**: Synthesize ADR files during `canon publish` instead of adding ADR documents to the runtime artifact contract under `.canon/`.
- **Rationale**: This keeps ADR output additive to the existing governed packet model, preserves `.canon/` as the authoritative runtime record, and avoids widening every decision-carrying mode's artifact contract just to support one repository-facing projection.
- **Alternatives considered**:
  - Add ADR artifacts to the persisted runtime contract for all supported modes: rejected because it would enlarge the runtime model, gating, and storage surface for a repository publication concern.
  - Create a dedicated `adr` mode: rejected because it would duplicate already-governed decision work instead of projecting durable records from existing packets.

## Decision 2: Use a fixed `docs/adr/` registry with numbered filenames

- **Decision**: Publish ADRs into `docs/adr/` using `ADR-XXXX-<slug>.md`, where `XXXX` is the next non-conflicting identifier derived from the existing registry.
- **Rationale**: A fixed repository-local registry matches common ADR practice, gives stable review references, and keeps durable decisions discoverable without coupling filenames to ephemeral run identifiers.
- **Alternatives considered**:
  - Reuse mode-specific publish folders such as `docs/architecture/decisions/`: rejected because architecture packet folders are run-scoped publish destinations, not a durable cross-run ADR register.
  - Name ADRs from run IDs or timestamps only: rejected because that weakens stable human referenceability and diverges from standard ADR numbering conventions.

## Decision 3: Make ADR export default for `architecture` and opt-in for `change` and `migration`

- **Decision**: `architecture` publishes always emit one ADR entry, while `change` and `migration` require an explicit publish flag to export an ADR.
- **Rationale**: Architecture decisions are durable by default, but many change and migration packets are tactical and should not automatically enter the repository's long-lived decision register.
- **Alternatives considered**:
  - Always export ADRs for `change` and `migration`: rejected because it would pollute the registry with short-lived or tactical decisions.
  - Make ADR export opt-in for every mode including `architecture`: rejected because the core goal of the feature is to establish `architecture` as the canonical ADR-producing mode.

## Decision 4: Add a single `--adr` publish flag for opt-in exports

- **Decision**: Extend `canon publish` with one boolean ADR export flag used by `change` and `migration`, while allowing `architecture` to ignore the flag because ADR publication is already default.
- **Rationale**: The current publish surface already owns durable repository output. One small flag keeps the user journey local to the existing command instead of adding another command family or separate post-processing step.
- **Alternatives considered**:
  - Add a separate `canon publish-adr` command: rejected because it fragments the publish experience and duplicates argument validation.
  - Encode ADR export through repository configuration only: rejected because the decision to register a tactical change or migration as a durable ADR should remain explicit at publish time.

## Decision 5: Keep the first ADR lifecycle state bounded to `Accepted`

- **Decision**: Generate publish-created ADRs with `Status: Accepted` in this slice.
- **Rationale**: A publishable packet is already the durable, reviewer-facing output of governed decision work. Limiting the initial lifecycle to accepted records keeps the feature standard-shaped without expanding into full ADR lifecycle editing, supersession, or rejection workflows.
- **Alternatives considered**:
  - Prompt for arbitrary status at publish time: rejected because it expands the CLI surface and lifecycle semantics beyond the agreed slice.
  - Derive multiple statuses from current run states: rejected because many standard ADR lifecycle states are repository-history concerns rather than run-state concerns.

## Decision 6: Keep the ADR registry path fixed even when packet publish uses `--to`

- **Decision**: `--to` continues to override the packet publish destination, but ADR registry output still lands in `docs/adr/`.
- **Rationale**: The ADR register is a durable repository index, not a per-run packet destination. A fixed path keeps ADR discovery and numbering coherent even when packet consumers want custom publish folders.
- **Alternatives considered**:
  - Mirror ADR files into the override directory: rejected because it fragments the register and weakens global numbering semantics.
  - Add a second ADR destination override: rejected because it expands the CLI surface before the base ADR workflow is validated.

## Decision 7: Reject ADR export requests for unsupported modes

- **Decision**: If the operator requests ADR export on an unsupported mode, publish returns a validation error instead of silently ignoring the request.
- **Rationale**: Explicit rejection keeps the mode boundary honest and makes unsupported behavior visible to users and tests.
- **Alternatives considered**:
  - Silently ignore the flag for unsupported modes: rejected because it hides a boundary violation and makes the CLI harder to reason about.
  - Infer ADR eligibility from artifact names alone: rejected because mode boundaries, not opportunistic text matches, control governance semantics.