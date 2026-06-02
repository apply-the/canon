# Research: Mode Publish Alignment

## Decision 1: Align runtime to the documented `security-assessment` publish posture

- **Decision**: Treat `security-assessment` like the other documented recommendation-only operational packets that may publish readable artifacts from `AwaitingApproval` or `Blocked` when artifacts already exist.
- **Rationale**: The documentation and skill guidance already classify `security-assessment` with operational packets intended for wider review before full completion. Changing the docs would preserve drift instead of resolving the operator-facing mismatch.
- **Alternatives considered**:
  - Narrow the docs and skills to require `Completed` for `security-assessment`: rejected because it contradicts the documented operational review posture and provides less utility to operators.
  - Broaden all non-completed publish exceptions: rejected because the audit only confirmed one missing mode and the slice should stay bounded.

## Decision 2: Fix assistant publish syntax in metadata and prompt surfaces, not in the CLI

- **Decision**: Update assistant package metadata and prompt-pack examples to use positional `canon publish <RUN_ID>` syntax.
- **Rationale**: The shipped CLI already uses a positional run id. The drift is in assistant-facing guidance, not in the command parser.
- **Alternatives considered**:
  - Add `--run` support to the CLI: rejected because it broadens the public surface and creates an unnecessary second syntax for the same command.
  - Leave the drift and document it informally: rejected because package metadata is part of the supported user surface.

## Decision 3: Advance the repository release line to `0.45.0` as part of the slice

- **Decision**: Bump the repository version and directly governed release-surface references to `0.45.0` while capturing validation evidence under the feature packet.
- **Rationale**: The repository uses versioned feature slices with aligned tech-docs/tests/release metadata, and the user explicitly requested the bump and closeout discipline.
- **Alternatives considered**:
  - Keep `0.44.0`: rejected because it would ship behavior/doc changes without advancing the release line.
  - Delay the bump to a later feature: rejected because the current slice already touches governed release-facing surfaces.