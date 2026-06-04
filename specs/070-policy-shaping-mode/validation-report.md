# Validation Report: Policy Shaping Mode

## Validation Strategy

### Structural Validation
- **Goal**: Ensure the generated packet contains the required markdown artifacts (`01-policy-context.md`, `02-proposed-rule.md`, `03-conformance-impact.md`, `04-migration.md`, and `05-approval.md`).
- **Method**: The `canon` CLI will enforce output paths and fail the command if required artifacts are missing.

### Logical Validation
- **Goal**: Verify that the impact report matches the semantics of the draft policy.
- **Method**: LLM-backed `.agents/skills` will perform a semantic pass. The `canon` CLI will normalize their outputs to guarantee valid structured evidence and fail if the logic contains conflicts.

### Independent Validation
- **Goal**: Review the migration plan and broad-impact threshold limits.
- **Method**: Human operators must provide explicit `Systemic Impact` sign-off. If the impact exceeds the module/directory grouping threshold, a specific broad-impact approval gate will block finalization until explicitly overridden.
