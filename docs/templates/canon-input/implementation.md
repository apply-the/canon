# Implementation Brief

Feature Slice: The concrete feature or component this execution packet covers.
Primary Upstream Mode: change | architecture | direct
Upstream Sources:
- docs/changes/<RUN_ID>/implementation-plan.md
Carried-Forward Decisions:
- Decision 1 carried forward into this execution packet.
Excluded Upstream Scope: The upstream material that remains out of scope here.
Task Mapping: 1. First bounded implementation step. 2. Second bounded implementation step.
Mutation Bounds: Files, modules, and interfaces this packet may touch.
Allowed Paths:
- path/to/file.rs
Safety-Net Evidence: Tests, checks, monitors, or review hooks that must exist before mutation.
Independent Checks:
- cargo test --test target_name
Rollback Triggers: Signals that require abandoning or reversing the bounded patch.
Rollback Steps: How to revert the bounded patch safely.
Risk Level: bounded-impact
Zone: yellow