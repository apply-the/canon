# Data Model: Policy Shaping Mode

## Entities

### `DraftPolicy`
Represents the proposed governance rule change.
- **Attributes**:
  - `title` (String)
  - `mode` (String) - e.g., "policy-shaping"
  - `risk` (String) - Must be "Systemic Impact"
  - `scope_in` (List<String>)
  - `scope_out` (List<String>)
  - `invariants` (List<String>)
- **Format**: Markdown with YAML frontmatter.

### `ImpactReport`
Evidence describing current codebase violations against the draft policy.
- **Attributes**:
  - `total_violations` (Integer)
  - `affected_modules` (Integer)
  - `severity` (String)
  - `migration_risk` (String)
- **Format**: Markdown with YAML frontmatter. File-level details move to a machine-readable appendix.

### `MigrationPlan`
Strategy for transitioning legacy areas to compliance.
- **Attributes**:
  - `waiver_policy` (String)
  - `rollout_phases` (List<String>)
  - `debt_created` (String)
- **Format**: Markdown with YAML frontmatter.

### `PolicyDiff`
Semantic changes to the existing constitution.

## State Transitions
1. `Draft` -> `Validating` (CLI runs exploratory LLM validation)
2. `Validating` -> `Impact Reported` (Report generated)
3. `Impact Reported` -> `Migration Planned` (Migration strategy mapped)
4. `Migration Planned` -> `Approved` (Human Systemic Impact sign-off)
