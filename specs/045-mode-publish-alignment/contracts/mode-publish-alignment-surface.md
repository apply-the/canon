# Contract: Mode Publish Alignment Surface

## Purpose

Define the bounded operator and assistant-facing behavior that this feature must preserve or correct.

## Runtime Publish Contract

### Security Assessment Publishability

- `security-assessment` packets with readable persisted artifacts are publishable when the run state is:
  - `Completed`
  - `AwaitingApproval`
  - `Blocked`
- This exception applies only to readable packet publication.
- Publish still fails when no persisted publishable artifacts exist.
- Publish still writes the normal packet destination contents plus `packet-metadata.json`.

### Untouched Publish Rules

- Default publish destination roots remain:
  - `specs/` for `requirements`
  - `docs/architecture/decisions/` for `architecture`
  - existing family roots for all other modes
- `requirements` still publishes sectional packet files plus `prd.md`.
- `architecture` still publishes its existing C4 packet set and default ADR projection.
- `change` and `migration` still use `--adr` opt-in.
- Unsupported modes still reject `--adr`.

## Assistant Surface Contract

- Assistant-facing examples for publish use:

```text
canon publish <RUN_ID>
canon publish <RUN_ID> --adr
canon publish <RUN_ID> --to docs/custom/path
```

- Assistant-facing publish guidance must not use `canon publish --run <RUN_ID>`.

## Versioned Release Contract

- The delivery line for this slice is `0.45.0`.
- Touched release-governed files and assertions updated by this feature must align to `0.45.0` before closeout.

## Validation Expectations

- Focused security-assessment publish tests cover `AwaitingApproval`, `Blocked`, and non-operational regression protection.
- Assistant package validation confirms publish command examples remain aligned with the CLI.
- Final closeout includes formatter, linter, focused logical validation, and touched-file coverage review.