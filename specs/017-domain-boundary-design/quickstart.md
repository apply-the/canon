# Quickstart: Domain Modeling And Boundary Design

## Goal

Verify the domain-modeling first slice for `system-shaping`, `architecture`, and `change` end to end.

## 1. Read the updated authored-input guidance

- `defaults/embedded-skills/canon-system-shaping/skill-source.md`
- `defaults/embedded-skills/canon-architecture/skill-source.md`
- `defaults/embedded-skills/canon-change/skill-source.md`
- `docs/templates/canon-input/system-shaping.md`
- `docs/templates/canon-input/architecture.md`
- `docs/templates/canon-input/change.md`
- `docs/examples/canon-input/system-shaping-billing.md`
- `docs/examples/canon-input/architecture-state-management.md`
- `docs/examples/canon-input/change-add-caching.md`

Confirm the skills, templates, and examples agree on the domain-modeling sections for each target mode.

## 2. Run the focused validation suite

Execute the targeted tests generated for this feature, including:

- contract coverage for `system-shaping`, `architecture`, and `change`
- renderer coverage for the new domain-modeling sections and missing-body fallbacks
- end-to-end run coverage for the three target modes
- docs coverage for skill/template/example synchronization

## 3. Run repository-level structural checks

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `/bin/bash scripts/validate-canon-skills.sh`

## 4. Perform an independent walkthrough

Use one complete example per target mode and one derived negative fixture per mode to confirm:

- `system-shaping` emits `domain-model.md` with candidate bounded contexts, core/supporting hypotheses, ubiquitous language, and explicit domain invariants
- `architecture` emits `context-map.md` with bounded contexts, shared invariants, relationships, seams, and anti-corruption candidates
- `change` strengthens its existing packet with domain slice, domain invariants, ownership boundaries, and cross-context risks
- missing authored sections surface explicit incompleteness instead of fabricated certainty
- non-target modes remain unchanged