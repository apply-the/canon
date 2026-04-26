# Quickstart: Mode Authoring Specialization Follow-On

## Positive Scenario

1. Author the three input briefs using the updated canonical H2 contracts:
   - `canon-input/system-shaping.md`
   - `canon-input/implementation.md`
   - `canon-input/refactor.md`
2. Ensure each brief includes the full required authored sections for its emitted packet artifacts.
3. Run the relevant Canon mode with the normal risk, zone, and system-context inputs.
4. Inspect the emitted packet and confirm:
   - the targeted artifacts preserve authored section bodies verbatim
   - `system-shaping`, `implementation`, and `refactor` packet artifacts read like real authored packets rather than summary-driven filler
   - execution posture and approval behavior for `implementation` and `refactor` are unchanged

## Negative Scenario

1. Remove one required canonical heading from each targeted brief, for example:
   - `## Boundary Decisions` from `system-shaping`
   - `## Rollback Steps` from `implementation`
   - `## Decision` from `refactor`
2. Run the same mode flows again.
3. Inspect the emitted packet artifacts and confirm each affected artifact contains `## Missing Authored Body` naming the missing canonical heading.
4. Confirm the run remains blocked with an artifact-level gate blocker rather than silently advancing to approval-gated or completed state.
5. Repeat once with a near-match heading such as `## Rollback Plan` in place of `## Rollback Steps` and confirm Canon still reports the canonical heading as missing.

## Review Expectations

- A reviewer can identify the authored contract for each targeted mode from the skill, template, and example without reading source code.
- The packet makes missing authored context explicit instead of hiding it behind plausible generated prose.
- Incomplete targeted packets stay explicitly gate-blocked while naming the missing canonical heading.
- The runtime continues to honor the current approval and recommendation-only boundaries for execution-heavy modes.
- Roadmap and mode-guide text describe the delivered second slice without overstating rollout completion.