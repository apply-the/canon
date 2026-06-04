# Backlog Handoff Fixtures

These fixtures document the authored backlog shapes used by the handoff
contract tests.

## Handoff-capable packet

- Full planning packet
- Stable `SLICE-...` identifiers repeated across slices, dependencies,
  sequencing, and acceptance anchors
- `Execution Handoff` section selects one slice
- Explicit implementation artifact references
- Independent verification anchors

## Handoff-unavailable packet

- Full planning packet
- Stable slice identifiers may still be present
- No admissible slice because implementation refs or independent verification
  anchors are missing
- No `execution-handoff.md`

## Closure-limited packet

- Risk-only packet
- No full decomposition artifacts
- No `execution-handoff.md`
