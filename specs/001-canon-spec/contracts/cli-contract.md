# CLI Contract: Canon v0.1

## Binary Identity

- Product name: `Canon`
- Primary binary name: `canon`
- Contract stability target: command names, top-level flags, exit codes, and
  machine-readable output shapes

## Global Behavior

- default output is human-readable text
- `--output json` and `--output yaml` are supported for status and inspection
  commands
- all mutating commands emit the run id they created or modified
- non-zero exit codes are semantically meaningful and test-covered

## Commands

### `init`

Purpose:

- create `.canon/`
- materialize built-in methods and policies
- validate runtime directory permissions

### `run`

Required flags:

- `--mode <mode>`
- `--risk <risk-class>`
- `--zone <usage-zone>`
- `--owner <owner-id>`

Optional flags:

- `--input <path-or-ref>` repeated
- `--exclude <path>` repeated
- `--policy-root <path>`
- `--method-root <path>`
- `--output <text|json|yaml>`

Behavior:

- creates a new run id
- captures context
- resolves policy
- writes the artifact contract
- executes the allowed steps for the selected mode
- blocks or completes according to gate outcomes

### `resume`

Required flags:

- `--run <run-id>`

Behavior:

- loads the stored run state
- checks fingerprint drift
- continues from the first incomplete or invalidated step

### `status`

Required flags:

- `--run <run-id>`

Behavior:

- prints current state, gate summary, approval status, and artifact status

### `approve`

Required flags:

- `--run <run-id>`
- `--gate <gate-kind>`
- `--by <approver-id>`
- `--decision <approve|reject>`
- `--rationale <text>`

Behavior:

- persists an `ApprovalRecord`
- links the record to the run and gate
- may unblock a `NeedsApproval` gate

### `verify`

Required flags:

- `--run <run-id>`
- `--layer <verification-layer>`

Behavior:

- executes or records one verification layer
- persists a `VerificationRecord`
- updates readiness if the layer satisfies outstanding requirements

### `inspect`

Subcommands:

- `inspect modes`
- `inspect methods`
- `inspect policies`
- `inspect artifacts --run <run-id>`

Behavior:

- exposes machine-readable introspection for local automation and debugging

## Exit Codes

- `0`: command succeeded and the run is in a valid progressed state
- `2`: policy or gate blocked progress
- `3`: explicit human approval required
- `4`: required adapter unavailable
- `5`: validation failed
- `6`: persistence failure or corrupted run state

## Command Contract Rules

- `run` must refuse to start without mode, risk, zone, and owner
- `resume` must refuse to continue a stale run without an explicit user choice
- `approve` cannot silently mutate artifacts or traces beyond the approval
  record it writes
- `verify` cannot pass readiness unless the run's required verification layers
  are satisfied
