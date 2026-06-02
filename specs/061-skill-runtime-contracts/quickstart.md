# Quickstart: Canon Skill Runtime Contracts

## Overview

This feature adds three capabilities to Canon skills:

1. **Structured preflight JSON** replacing key=value checks
2. **Declarative preflight YAML** in SKILL.md frontmatter
3. **Lifecycle hooks** with detect/propose semantics

## Phase A: Using the Preflight Script

Run the preflight script to get a structured environment snapshot:

```bash
# From repo root
.agents/skills/canon-shared/scripts/canon-preflight.sh --mode implementation
```

Output (JSON):
```json
{
  "schema_version": 1,
  "timestamp": "2026-05-28T14:30:00Z",
  "mode": "implementation",
  "canon": { "available": true, "version": "0.3.2", "initialized": true, "error": null },
  "workspace": { "path": "/path/to/repo", "git_branch": "main", "git_user": "dev@example.com", "error": null },
  "input": { "file_exists": true, "file_path": "canon-input/implementation.md", "file_empty": false, "folder_exists": false, "folder_path": "canon-input/implementation/", "folder_empty": null, "resolved_path": "canon-input/implementation.md", "ambiguous": false, "error": null },
  "runs": { "active": 1, "pending_approvals": 0, "error": null }
}
```

On PowerShell:
```powershell
.agents/skills/canon-shared/scripts/canon-preflight.ps1 -Mode implementation
```

## Phase B: Adding Preflight Contract to a Skill

Add a `preflight:` block to your SKILL.md frontmatter:

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

The AI assistant will:
1. Parse this block
2. Run `canon-preflight.sh --mode implementation`
3. Compare requirements against JSON output
4. Surface only the unmet requirements

## Phase C: Lifecycle Hooks

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

When a skill reaches the matching lifecycle point, it detects the hook,
shows the full command and description, and proposes execution. The trace
is recorded in `ai-provenance.md`.

## Validation

Run the skill validator to confirm no regressions:

```bash
scripts/validate-canon-skills.sh
```

Validate preflight JSON output:

```bash
.agents/skills/canon-shared/scripts/canon-preflight.sh --mode change | jq .
```

## Migration Notes

- Existing skills with prose-only preflight continue working unchanged.
- When adding `preflight:` YAML, mark legacy prose with deprecation comment:

```markdown
<!-- DEPRECATED: preflight behavior is governed by the `preflight:` block.
     Do not use this prose as an execution contract. -->
```

- Only 3 skills are migrated in this feature: `canon-implementation`,
  `canon-change`, `canon-publish`.
