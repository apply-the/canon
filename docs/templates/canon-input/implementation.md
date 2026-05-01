# Implementation Brief

Feature Slice: The concrete feature or component this execution packet covers.
Primary Upstream Mode: change | architecture | direct
Upstream Sources:
- docs/changes/<RUN_ID>/implementation-plan.md
Carried-Forward Decisions:
- Decision 1 carried forward into this execution packet.
Excluded Upstream Scope: The upstream material that remains out of scope here.

## Task Mapping
1. First bounded implementation step.
2. Second bounded implementation step.

## Bounded Changes
- The bounded code or artifact slice this packet is allowed to change.

## Mutation Bounds
Files, modules, and interfaces this packet may touch.

## Allowed Paths
- path/to/file.rs

## Executed Changes
- Describe the bounded implementation change that will be applied or recommended.

## Candidate Frameworks
- Candidate 1 and why it remains viable.
- Candidate 2 and why it remains viable or risky.

## Options Matrix
- Option 1: The narrowest bounded implementation approach.
- Option 2: A broader abstraction or rollout path that is intentionally deferred.

## Decision Evidence
- Evidence source 1 grounding the recommendation.
- Evidence source 2 grounding the deferred option.

## Recommendation
- State which implementation option should proceed and why.

## Task Linkage
- Explain how the executed change maps back to the task plan.

## Completion Evidence
- Name the evidence that will prove the bounded change is done.

## Adoption Implications
- State how this bounded implementation should or should not spread to adjacent surfaces.

## Remaining Risks
- Residual risk 1.

## Ecosystem Health
- Summarize whether the surrounding libraries, modules, or platform constraints make this bounded choice healthy enough to proceed.

## Safety-Net Evidence
Tests, checks, monitors, or review hooks that must exist before mutation.

## Independent Checks
- cargo test --test target_name

## Rollback Triggers
Signals that require abandoning or reversing the bounded patch.

## Rollback Steps
How to revert the bounded patch safely.

Risk Level: bounded-impact
Zone: yellow