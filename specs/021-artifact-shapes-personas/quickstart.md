# Quickstart: Industry-Standard Artifact Shapes With Personas

## Goal

Validate the first slice of persona-aware packet shaping for `requirements`,
`architecture`, and `change`.

## Steps

1. Review the feature intent in `spec.md`, `plan.md`, and `decision-log.md`.
2. Update the first-slice skill source and mirrored skill files so each mode
   declares both its packet shape and its bounded persona.
3. Confirm the existing artifact contract and markdown rendering surfaces
   already preserve the shaped sections and missing-content honesty; only touch
   runtime code if focused validation disproves that assumption.
4. Update operator-facing docs so the new mode-to-shape and mode-to-persona
   mapping is discoverable.
5. Run focused validation for the touched slice:
   - `/bin/bash scripts/validate-canon-skills.sh`
   - `cargo test --test requirements_authoring_docs --test requirements_authoring_renderer --test requirements_authoring_run --test architecture_c4_docs --test architecture_decision_shape_docs --test architecture_c4_renderer --test architecture_c4_run --test change_authoring_docs --test change_authoring_renderer --test change_authoring_run`
   - `cargo test --test requirements_authoring_docs --test change_authoring_docs --test discovery_authoring_docs --test mode_authoring_follow_on_docs`
6. Run full repository closeout checks before merge:
   - `cargo fmt`
   - `cargo test`

## Expected Results

- Requirements packets read like PRD-oriented product artifacts.
- Architecture packets read like C4 plus ADR decision artifacts.
- Change packets read like ADR-style bounded change packets.
- Missing authored content remains explicitly visible.
- Non-targeted modes remain unchanged.
