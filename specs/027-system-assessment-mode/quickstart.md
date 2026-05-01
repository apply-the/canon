# Quickstart: System Assessment Mode

## Goal

Exercise the first-slice `system-assessment` workflow from authored brief to
published as-is packet.

## Steps

1. Create or reuse a small seeded repository that has code, configuration, and
   at least one integration or deployment hint.
2. Author a `system-assessment.md` brief with the assessment objective,
   stakeholders, concerns, scope, evidence sources, and the required packet
   sections.
3. Run Canon with `--mode system-assessment --system-context existing` and the
   authored brief as input.
4. Inspect the emitted `.canon/artifacts/<RUN_ID>/system-assessment/` bundle.
5. Confirm the packet uses ISO 42010 coverage language and distinguishes
   observed findings, inferred findings, and assessment gaps.
6. Publish the completed run and verify the packet lands under
   `docs/architecture/assessments/<RUN_ID>/`.
7. Run the focused contract, integration, documentation, and shared-surface
   tests for the feature.

## Expected Result

- The mode is available as a first-class CLI surface.
- A valid run emits the full assessment artifact bundle and summarizes the
  result as an as-is packet rather than a decision packet.
- Invalid system context or missing required authored sections produce honest
  blockers or missing-body markers.
- Published packets live under the architecture assessment docs path.