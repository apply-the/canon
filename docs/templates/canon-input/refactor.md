# Refactor Brief

Use this brief to author a preserved-behavior matrix plus
structural-rationale packet that proves no new feature semantics are being
introduced.

Feature Slice: The concrete feature or component this preservation packet covers.
Primary Upstream Mode: change | implementation | direct
Upstream Sources:
- docs/refactors-or-implementation/<RUN_ID>/primary-artifact.md
Carried-Forward Invariants:
- Invariant 1 carried forward into this preservation packet.
Excluded Upstream Scope: The upstream material that remains out of scope here.

## Preserved Behavior
The externally visible behavior that must remain unchanged.

## Approved Exceptions
Explicitly approved deviations, if any.

## Refactor Scope
The bounded structural cleanup surface.

## Allowed Paths
- path/to/file.rs

## Structural Rationale
Why this cleanup is worth doing without adding feature behavior.

## Untouched Surface
What must not change as part of this refactor.

## Safety-Net Evidence
Tests, checks, or review hooks that must hold before cleanup.

## Regression Findings
Accepted regressions, or `none`.

## Contract Drift
State whether public contracts may change.

## Reviewer Notes
- Reviewer note or checkpoint.

## Feature Audit
State explicitly whether any new feature behavior is introduced.

## Decision
The bounded preservation decision for this packet.

Risk Level: bounded-impact
Zone: yellow