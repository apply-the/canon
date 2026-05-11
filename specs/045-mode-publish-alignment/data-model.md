# Data Model: Mode Publish Alignment

## PublishEligibilityRule

- **Purpose**: Describes whether a mode may publish readable packet artifacts before the run reaches `Completed`.
- **Fields**:
  - `mode`: Canon mode name.
  - `allowed_states`: Set of run lifecycle states that may publish readable packets.
  - `requires_artifacts`: Whether persisted readable artifacts must already exist.
  - `rationale`: Durable explanation for why the rule exists.
- **Relationships**:
  - Evaluated against a `RunStateSnapshot` during publish.
  - Must stay aligned with documented operational-mode guidance.
- **Validation Rules**:
  - Non-operational modes remain `Completed`-only unless explicitly authorized.
  - Operational exceptions may not change publish destination semantics.

## AssistantPublishReference

- **Purpose**: Represents any assistant-facing command example or metadata entry that teaches users how to invoke `canon publish`.
- **Fields**:
  - `surface_path`: Repository path containing the reference.
  - `command_text`: The visible command example.
  - `expected_shape`: Canonical positional `canon publish <RUN_ID>` contract.
- **Relationships**:
  - Consumed by assistant package validation tests.
  - Must mirror the CLI parser contract instead of defining its own syntax.
- **Validation Rules**:
  - May not include an invented `--run` flag.
  - Must remain compatible with the shipped CLI help and parser behavior.

## ReleaseLineSurface

- **Purpose**: Groups version-governed files that must move together when the feature ships.
- **Fields**:
  - `path`: Repository path.
  - `kind`: Manifest, doc, metadata, or test assertion.
  - `expected_version`: Version string for the current delivery line.
- **Relationships**:
  - Verified by focused release-surface tests or content assertions.
  - Recorded in the validation report during closeout.
- **Validation Rules**:
  - Touched version surfaces for the slice must align on `0.45.0`.
  - Any accepted exclusions must be documented explicitly in validation evidence.