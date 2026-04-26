# Quickstart: Mode Authoring Specialization

## Goal

Verify the first-slice specialization for `requirements`, `discovery`, and `change` end to end.

## 1. Read the updated authored-input docs

- `defaults/embedded-skills/canon-requirements/skill-source.md`
- `defaults/embedded-skills/canon-discovery/skill-source.md`
- `defaults/embedded-skills/canon-change/skill-source.md`
- `docs/templates/canon-input/requirements.md`
- `docs/templates/canon-input/discovery.md`
- `docs/templates/canon-input/change.md`
- `docs/examples/canon-input/requirements-api-v2.md`
- `docs/examples/canon-input/discovery-legacy-migration.md`
- `docs/examples/canon-input/change-add-caching.md`

Confirm the skill guidance, template, and example agree on the canonical authored H2 headings for each mode.

## 2. Run the focused validation suite

Execute the targeted tests generated for this feature, including:

- contract coverage for the three first-slice modes
- renderer coverage for authored-body preservation and missing-body fallback
- end-to-end run coverage for each updated mode
- docs coverage for skill/template/example synchronization

## 3. Run the repository-level structural checks

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `/bin/bash scripts/validate-canon-skills.sh`
- confirm `scripts/validate-canon-skills.ps1` remains logically equivalent if validator logic changed

## 4. Perform an independent walkthrough

Use one updated example per first-slice mode to confirm the emitted packet is honest:

- complete authored example preserves verbatim body sections
- incomplete authored example is derived by removing one required H2 section from the updated example and then emits `## Missing Authored Body` that names the missing canonical heading
- reference modes remain unchanged