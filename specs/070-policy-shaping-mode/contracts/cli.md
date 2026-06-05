# Contracts: CLI Interface

## Canon CLI Interface for Policy Shaping

The `canon` CLI will expose a new command to govern the policy shaping lifecycle.

### Command Definition
```bash
canon policy-shaping [OPTIONS] <draft-policy-file>
```

### Arguments
- `<draft-policy-file>`: Path to the `02-proposed-rule.md` or `draft-policy.md`.

### Options
- `--dry-run`: Execute the validation pass but do not generate the final packet.
- `--approve`: Record the explicit Systemic Impact sign-off.
- `--threshold <N>`: Set the broad-impact approval threshold for number of affected files (default: 100).

### Output
- On success: Returns `0` exit code and paths to the generated `conformance-impact-report.md`, `04-migration.md`, and `policy-diff.md`.
- On failure/validation error: Returns non-zero exit code and outputs semantic evaluation errors from the `.agents/skills` subsystem.

### Serialization Contract
CLI configuration and state must use typed `serde` models in Rust, strictly avoiding `serde_json::Map` assembly for stable shapes.
