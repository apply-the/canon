# Quickstart: Mode Authoring Specialization Completion

## Positive Scenario

1. Author the four input briefs using the updated canonical H2 contracts:
   - `canon-input/review.md`
   - `canon-input/verification.md`
   - `incident.md` or the canonical incident packet location used by your workflow
   - `migration.md` or the canonical migration packet location used by your workflow
2. Ensure each brief includes the full required authored sections for its emitted packet artifacts.
3. Run the relevant Canon mode with the normal risk, zone, and system-context inputs.
4. Inspect the emitted packet and confirm:
   - the targeted artifacts preserve authored section bodies verbatim
   - review and verification artifacts still read like critique-oriented packets
   - incident and migration artifacts still advertise recommendation-only posture where expected
   - release/docs surfaces report `0.20.0` and describe the specialization rollout as complete

## Negative Scenario

1. Remove one required canonical heading from each targeted brief, for example:
   - `## Final Disposition` from `review`
   - `## Overall Verdict` from `verification`
   - `## Stop Conditions` from `incident`
   - `## Rollback Triggers` from `migration`
2. Run the same mode flows again.
3. Inspect the emitted packet artifacts and confirm each affected artifact contains `## Missing Authored Body` naming the missing canonical heading.
4. Confirm the run remains blocked or approval-gated according to the current mode semantics rather than silently advancing as if the packet were complete.
5. Repeat once with a near-match heading, such as `## Rollback Plan` instead of `## Rollback Triggers`, and confirm Canon still reports the canonical heading as missing.

## Review Expectations

- A reviewer can identify the authored contract for each targeted mode from the skill, template, and example without reading Rust source.
- The packet makes missing authored context explicit instead of hiding it behind plausible generated prose.
- Review and verification packets keep explicit disposition/verdict surfaces rather than flattening into generic summaries.
- Incident and migration packets keep their current operational posture and publish behavior.
- Roadmap, changelog, guide, and compatibility references describe the rollout as complete and the release as `0.20.0`.