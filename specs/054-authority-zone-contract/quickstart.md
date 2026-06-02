# Quickstart: Authority Zone Contract

## Goal

Validate that Canon publishes `authority-governance-v1` as a typed, fail-closed
contract without taking over downstream runtime orchestration.

## Steps

1. Review `specs/054-authority-zone-contract/contracts/authority-governance-v1-contract.md` and confirm the required versus optional field profile is explicit.
2. Review `tech-docs/integration/governance-adapter.md` and the delivered personas or authority-zones guide to confirm the machine-facing and human-facing docs describe the same contract line.
3. Run `cargo test --test governance_adapter_surface --test mode_profiles --test policy_and_traces` and confirm the authority vocabulary, mode semantics, and adapter projection stay aligned.
4. Review `specs/054-authority-zone-contract/validation-report.md` for the broader validation suite and closeout evidence.

## Expected Outcome

- `authority-governance-v1` is recoverable from Canon docs and packet metadata.
- Missing required fields fail closed for downstream consumers.
- Missing optional provenance remains safely ignorable.
- `stage_role_hints` stay advisory and Canon does not become a runtime orchestrator.