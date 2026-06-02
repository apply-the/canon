# Skill Runtime Contracts

This guide explains the three runtime contract capabilities available to Canon
skills: structured preflight JSON, declarative preflight YAML, and lifecycle
hooks.

## Structured Preflight JSON

The `canon-preflight.sh` script (and `canon-preflight.ps1` on Windows) replaces
the legacy key=value output from `check-runtime.sh` with a structured JSON
snapshot of the environment.

### Usage

```bash
.agents/skills/canon-shared/scripts/canon-preflight.sh --mode <mode-name>
```

```powershell
.agents/skills/canon-shared/scripts/canon-preflight.ps1 -Mode <mode-name>
```

### Output Shape

The script emits JSON conforming to
`specs/061-skill-runtime-contracts/contracts/preflight-json-schema.json`:

```json
{
  "schema_version": 1,
  "timestamp": "2026-05-28T14:30:00Z",
  "mode": "implementation",
  "canon": {
    "available": true,
    "version": "0.63.0",
    "initialized": true,
    "error": null
  },
  "workspace": {
    "path": "/path/to/repo",
    "git_branch": "main",
    "git_user": "dev@example.com",
    "error": null
  },
  "input": {
    "file_exists": true,
    "file_path": "canon-input/implementation.md",
    "file_empty": false,
    "folder_exists": false,
    "folder_path": "canon-input/implementation/",
    "folder_empty": null,
    "resolved_path": "canon-input/implementation.md",
    "ambiguous": false,
    "error": null
  },
  "runs": {
    "active": 1,
    "pending_approvals": 0,
    "error": null
  }
}
```

### Partial Failure

The script always emits valid JSON, even when individual sections fail. Each
section has an `error` field that is null on success or contains a diagnostic
string on failure. The envelope (`schema_version`, `timestamp`, `mode`) is
always present.

### Performance

The script completes in under 2 seconds on a standard development machine.

## Declarative Preflight YAML

Skills that have been migrated to the runtime contracts system declare their
preflight requirements in YAML frontmatter:

```yaml
---
name: canon-implementation
description: Use when you need a governed implementation run...
preflight:
  requires_canon: true
  requires_initialized: true
  canonical_input: implementation
  system_context: existing
  risk_required: true
  zone_required: true
  owner_optional: true
---
```

### How It Works

When the AI assistant encounters a `preflight:` block:

1. It runs `canon-preflight.sh --mode <canonical_input>`.
2. It parses the JSON report.
3. For each declared requirement, it checks the corresponding JSON field.
4. It surfaces only the unmet requirements with targeted remediation hints.
5. It does NOT fall back to prose preflight instructions.

### Migrated Skills

In this release, three skills have been migrated:

- `canon-implementation`
- `canon-change`
- `canon-publish`

All other skills continue using prose-only preflight instructions unchanged.

### Adding Preflight to Other Skills

To migrate a skill, add the `preflight:` block to its YAML frontmatter and mark
the legacy prose section with:

```markdown
<!-- DEPRECATED: preflight behavior is governed by the 'preflight:' block.
     Do not use this prose as an execution contract. -->
```

## Lifecycle Hooks

Canon skills detect and propose lifecycle hooks declared in
`.canon/hooks.toml`. Hooks never auto-execute in V1.

### Setting Up Hooks

Create `.canon/hooks.toml`:

```toml
version = 1

[[hooks.after_publish.actions]]
id = "stage-docs"
command = "git add tech-docs/"
description = "Stage published documentation for commit"
optional = true
trusted = false

[[hooks.before_run.actions]]
id = "lint-input"
command = "scripts/lint-canon-input.sh"
description = "Lint canonical input before run starts"
optional = false
trusted = true
mode_filter = ["implementation", "change"]
```

### Supported Events

| Event | Fires When |
|-------|------------|
| `before_run` | Any governed run starts |
| `after_run` | Run reaches Completed state |
| `before_publish` | `canon publish` initiated |
| `after_publish` | `canon publish` completes |
| `before_approve` | `canon approve` initiated |
| `after_approve` | `canon approve` completes |
| `before_resume` | `canon resume` initiated |
| `after_resume` | `canon resume` completes |

### Hook Proposal

When a skill reaches a lifecycle point, it detects matching hooks and proposes
them with full transparency:

- The full command is shown.
- The working directory is explicit.
- Trusted/untrusted status is surfaced.
- Required hooks cannot be skipped without acknowledgement.

### Trace Recording

Every hook interaction (accepted, declined, or skipped) is recorded in the
run's `ai-provenance.md` with timestamp, exit code, and outcome.

### Security

- Hooks never auto-execute.
- Full command visibility is mandatory.
- `trusted: true` reduces confirmation friction only; it does not mean silent execution.
- Future versions may add allowlisting or sandboxing.

## Validation

Run the skill validator to confirm no regressions after changes:

```bash
scripts/validate-canon-skills.sh
```

Validate preflight JSON output:

```bash
.agents/skills/canon-shared/scripts/canon-preflight.sh --mode change | jq .
```
