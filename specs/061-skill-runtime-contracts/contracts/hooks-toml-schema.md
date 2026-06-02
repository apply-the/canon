# Hooks TOML Schema

The `.canon/hooks.toml` file declares lifecycle hooks that Canon skills detect
and propose at specific lifecycle points. Hooks never auto-execute in V1.

## File Location

`.canon/hooks.toml` at the repository root (inside the `.canon/` directory).

## Schema (version 1)

```toml
version = 1

# Event groups: before_run, after_run, before_publish, after_publish,
#               before_approve, after_approve, before_resume, after_resume

[[hooks.<event>.actions]]
id = "unique-hook-id"           # Required: unique within event group
command = "shell command"        # Required: command to propose
description = "What this does"  # Required: human-readable explanation
optional = true                 # Required: true = can skip, false = mandatory
trusted = false                 # Optional: default false; extra confirm if false
prompt = "Custom question?"     # Optional: override default proposal text
condition = ""                  # Optional: reserved for future condition eval
mode_filter = ["mode1", "mode2"] # Optional: restrict to listed modes (null = all)
```

## Supported Events

| Event | Lifecycle Point | Fires When |
|-------|----------------|------------|
| `before_run` | After preflight, before execution | Any governed run starts |
| `after_run` | After run completion | Run reaches Completed state |
| `before_publish` | Before publish output | `canon publish` initiated |
| `after_publish` | After publish success | `canon publish` completes |
| `before_approve` | Before approval recorded | `canon approve` initiated |
| `after_approve` | After approval success | `canon approve` completes |
| `before_resume` | Before resume continuation | `canon resume` initiated |
| `after_resume` | After resume completion | `canon resume` completes |

## Hook Execution Semantics

### Detection

The skill checks `.canon/hooks.toml` at the matching lifecycle point:

1. Parse TOML file (skip silently on parse error or missing file).
2. Find hooks matching current event (e.g., `hooks.after_publish.actions`).
3. Filter by `mode_filter` if present (skip hooks that don't match current mode).
4. Present matching hooks in order of declaration.

### Proposal Format

For each detected hook, the skill emits a proposal block:

```markdown
## Lifecycle Hook Detected

**Event**: after_publish
**Hook**: add-docs-to-git
**Command**: `git add tech-docs/`
**Working Directory**: <repo-root>
**Description**: Stage published documentation for commit
**Required**: No (optional)
**Trusted**: No

Proceed with this hook? [Yes / No / Skip all hooks]
```

### Confirmation Rules

| `optional` | `trusted` | Behavior |
|------------|-----------|----------|
| true | true | Standard proposal; single confirmation |
| true | false | Standard proposal + extra confirmation step |
| false | true | Mandatory proposal; must acknowledge before proceeding |
| false | false | Mandatory proposal + extra confirmation; cannot skip |

### Trace Recording

After proposal resolution, record in `ai-provenance.md`:

- Hook ID and event
- Full command shown
- Whether trusted
- Proposal outcome (accepted / declined / skipped)
- Execution result (if accepted): exit code, success/failure
- Timestamp

## Validation Rules

- File must be valid TOML; invalid TOML is skipped silently with diagnostic.
- `version` must be `1`; unknown versions are skipped with diagnostic.
- `id` must be unique within its event group.
- `command` must not be empty.
- Unknown fields are ignored (forward compatibility).
- Empty `[[hooks.<event>.actions]]` arrays are valid (no hooks for that event).

## Security Boundary (V1)

- Hooks NEVER auto-execute.
- Full command, working directory, description, and expected effect MUST be shown.
- `trusted: true` does NOT mean silent execution; it reduces confirmation friction only.
- Missing `trusted` field = `trusted: false`.
- Future slices may add allowlisting or sandboxing.
