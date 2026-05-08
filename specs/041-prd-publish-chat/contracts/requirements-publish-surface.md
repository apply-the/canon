# Contract: Requirements Publish Surface

## Purpose

Define the user-visible contract for publishing a completed `requirements` run after this feature lands.

## Inputs

- A completed and publishable Canon run in `requirements` mode.
- A run identifier accepted by `canon publish`, such as the full run id, short id, UUID, or `@last` where supported by the CLI.
- An optional override destination supplied through `--to <PATH>`.

## Outputs

Publishing a completed `requirements` run MUST produce one directory containing:

- `problem-statement.md`
- `constraints.md`
- `options.md`
- `tradeoffs.md`
- `scope-cuts.md`
- `decision-checklist.md`
- `prd.md`
- `packet-metadata.json`

## Behavioral Rules

- The default destination remains the existing date-based requirements publish leaf under `specs/`.
- Any explicit `--to` destination receives the same file set.
- `prd.md` is additive and does not replace or rename the sectional files.
- `packet-metadata.json` continues to record the source artifacts under `.canon/artifacts/...`; the new PRD source path is additive.
- Publish continues to fail for runs that are incomplete, blocked, or approval-gated under existing Canon rules.

## Chat Skill Rules

- Chat guidance MUST map directly to `canon publish <RUN_ID>`.
- Chat guidance MAY mention `--to <PATH>` as an optional override.
- Chat guidance MUST tell the user that artifacts live first under `.canon/artifacts/` and become repo-visible only after publish.
- Chat guidance MUST NOT imply that publish can bypass approvals, critique, or other governance gates.