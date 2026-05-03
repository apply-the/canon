# Contract: Governance Adapter Documentation Surface

## Purpose

Define the minimum documentation surface that must exist for Canon's machine-facing governance adapter after feature 040 lands.

## Required Commands

- `canon governance capabilities --json`
- `canon governance start --json < request.json`
- `canon governance refresh --json < request.json`

## Required Stable Fields

The integration docs must describe these machine-facing response fields explicitly:

- `status`
- `approval_state`
- `packet_readiness`
- `reason_code`
- canonical workspace-relative packet or document refs

## Required Framing Rules

- The docs must say that `canon governance` is the machine-facing boundary around the same runtime used by the human CLI.
- The docs must say that Canon is not a generic agent framework or opaque orchestration loop.
- The docs must not claim that Canon itself is the higher-level orchestrator.

## Required Examples

The integration docs must include representative start or refresh examples, response examples, or both for:

- `change`
- `implementation`
- `verification`

The integration docs must also include an explicit current-boundary note for `pr-review` so external orchestrators do not assume the `v1` request envelope already exposes diff-ref binding directly.

## Drift Conditions

The documentation surface is out of contract if any of the following are true:

- One of the three commands is missing
- One of the stable fields is undocumented
- Human-vs-machine boundary guidance is absent or contradictory
- Canon is described as a generic agent framework or as the orchestrator itself
- The examples omit one of the required governed modes