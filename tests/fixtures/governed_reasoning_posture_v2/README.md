# Governed Reasoning Posture v2 Fixtures

This directory contains the machine-checkable fixture corpus for the
`governed_reasoning_posture_v2` contract.

Each fixture file must declare:

- `example_id`
- `expected_validation_result`
- `expected_reason`
- `contract_lines_involved`

Fixture files are added incrementally to cover:

- valid v2 payloads
- malformed selector, independence, confidence-handoff, provenance, and
  compatibility-window payloads
- stale or contradictory release metadata
- dual-line coexistence and migration rejection scenarios