# Migration Brief

## Current State
[State the exact capability, data model, or architecture bounded by this migration currently.]

## Target State
[State the end-goal of the transition. What is the final form within this bounded surface?]

## Transition Boundaries
[Explicitly set the boundaries of the migration. What parts of the codebase, APIs, or data models are included and excluded?]

## Guaranteed Compatibility
[What backward compatibility is preserved? E.g., clients using v1 will not see errors during the transition.]

## Temporary Incompatibilities
[What behaviors, logs, or metrics will temporarily mismatch or break during cutover?]

## Coexistence Rules
[How will v1 and v2 coexist? Are there dual writes, shadowing, feature flags, or separate deployments?]

## Options Matrix
[Compare the credible migration options. Which option preserves the bounded surface best, and what tradeoff does each option carry?]

## Ordered Steps
[List the sequential phases from start to finish of the cutover.]
- Phase 1: ...
- Phase 2: ...

## Parallelizable Work
[What sub-tasks can be performed safely in parallel alongside the ordered steps?]

## Cutover Criteria
[What metrics, states, or logs must be met before fully switching traffic or logic to the target state?]

## Rollback Triggers
[What concrete signals should trigger fallback or rollback?]

## Fallback Paths
[What bounded fallback paths remain credible if the cutover regresses?]

## Re-Entry Criteria
[What must be true before re-attempting the migration after fallback?]

## Adoption Implications
[State how widely the target state should spread after this bounded migration succeeds, and what surfaces must remain deferred.]

## Verification Checks
[What will confirm that the new state operates correctly and within bounds?]

## Residual Risks
[What are the known risks that persist despite the plan? E.g., data skew during sync.]

## Release Readiness
[State the target release confidence or posture. Typically mentions fallback credibility constraints.]

## Migration Decisions
[What critical decisions have been fixed for how this cutover operates?]

## Tradeoff Analysis
[Explain the most important tradeoffs across compatibility, cutover speed, rollback safety, and operational burden.]

## Recommendation
[State the recommended migration option and why it is preferred for this bounded surface.]

## Ecosystem Health
[Summarize the readiness of dependent services, libraries, teams, or operational tooling that this migration relies on.]

## Deferred Decisions
[What choices are deliberately pushed to later migration phases?]

## Approval Notes
[What explicit sign-offs or reviews are necessary? E.g., Database Owner review required.]
