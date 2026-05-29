# Hooks Schema Reference

Canon lifecycle hooks are declared in `.canon/hooks.toml`. Skills detect,
propose, and trace hooks at specific lifecycle points without auto-executing
them.

## Location

`.canon/hooks.toml` at the repository root (inside the `.canon/` directory).

## Full Schema

See `specs/061-skill-runtime-contracts/contracts/hooks-toml-schema.md` for the
complete v1 schema definition including:

- Supported events (`before_run`, `after_run`, `before_publish`,
  `after_publish`, `before_approve`, `after_approve`, `before_resume`,
  `after_resume`)
- Action fields (`id`, `command`, `description`, `optional`, `trusted`,
  `prompt`, `condition`, `mode_filter`)
- Confirmation rules matrix
- Trace recording format
- Security boundary (V1: hooks never auto-execute)

## Quick Reference

```toml
version = 1

[[hooks.<event>.actions]]
id = "unique-hook-id"
command = "shell command"
description = "What this does"
optional = true
trusted = false
mode_filter = ["implementation", "change"]
```

## Detection Flow

1. Parse `.canon/hooks.toml` (skip silently on error or missing file)
2. Match hooks by event and mode filter
3. Emit proposal block for each matching hook
4. Record trace in `ai-provenance.md` after resolution

## Confirmation Matrix

| `optional` | `trusted` | Behavior |
|------------|-----------|----------|
| true | true | Standard proposal; single confirmation |
| true | false | Standard proposal + extra confirmation |
| false | true | Mandatory; must acknowledge |
| false | false | Mandatory + extra confirmation; cannot skip |
