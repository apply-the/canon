# Contract: Scanner Intake And Coverage Gap Handling

## Purpose

Define the user-intake and evidence-recording behavior that governs missing
posture inputs and missing-scanner decisions for `supply-chain-analysis`.

## Required Clarification Inputs

- `Licensing Posture`: `commercial`, `oss-permissive`, `oss-copyleft`, or `mixed`
- `Distribution Model`: distributed artifact versus internal-only usage
- `Ecosystem Confirmation`: detected ecosystems kept or removed from scope
- `Out Of Scope Components`: vendored, generated, or third-party exclusions
- `Non-OSS Tool Policy`: whether non-OSS scanner proposals are allowed

## Decision Semantics

- Canon must not guess `Licensing Posture`.
- Canon must not guess `Non-OSS Tool Policy`.
- If either remains unresolved, the run records a durable
  `Missing Authored Decision` marker.
- If a required scanner is missing, the user decision must be one of:
  - `installed`
  - `skipped`
  - `replaced`

## Coverage Gap Rules

- `skipped` and `replaced` decisions must produce a coverage-gap record naming:
  - the affected ecosystem
  - the missing or replaced scanner capability
  - the impacted packet artifacts
  - the next action needed for full coverage
- Coverage gaps must appear in the human packet, not only in raw evidence.

## Evidence Recording

- Scanner decisions are persisted as run evidence and referenced from
  `policy-decisions.md`.
- Tool stderr or failure output is evidence and must remain inspectable.
- Canon can propose install commands, but the install itself remains a user
  action outside Canon's automated execution path.