# Data Model: Canon Skill Runtime Contracts

**Branch**: `061-skill-runtime-contracts`
**Date**: 2026-05-28

## Entities

### 1. Preflight Report (JSON)

The structured environment snapshot emitted by `canon-preflight.sh`.

```json
{
  "schema_version": 1,
  "timestamp": "2026-05-28T14:30:00Z",
  "mode": "implementation",
  "canon": {
    "available": true,
    "version": "0.3.2",
    "initialized": true,
    "error": null
  },
  "workspace": {
    "path": "/path/to/repo",
    "git_branch": "061-skill-runtime-contracts",
    "git_user": "developer@example.com",
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
    "active": 2,
    "pending_approvals": 1,
    "error": null
  }
}
```

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_version` | integer | yes | Schema version for forward compatibility |
| `timestamp` | string (ISO 8601) | yes | When the report was generated |
| `mode` | string | yes | The `--mode` argument passed to the script |
| `canon.available` | boolean | yes | Whether `canon` binary is on PATH |
| `canon.version` | string or null | yes | Output of `canon --version` or null |
| `canon.initialized` | boolean | yes | Whether `.canon/` directory exists |
| `canon.error` | string or null | yes | Section-level error if check failed |
| `workspace.path` | string | yes | Absolute path to repo root |
| `workspace.git_branch` | string or null | yes | Current git branch |
| `workspace.git_user` | string or null | yes | Git user.email |
| `workspace.error` | string or null | yes | Section-level error if check failed |
| `input.file_exists` | boolean | yes | Whether mode file exists |
| `input.file_path` | string | yes | Expected file path for mode |
| `input.file_empty` | boolean or null | yes | Whether file is empty (null if not exists) |
| `input.folder_exists` | boolean | yes | Whether mode folder exists |
| `input.folder_path` | string | yes | Expected folder path for mode |
| `input.folder_empty` | boolean or null | yes | Whether folder is empty (null if not exists) |
| `input.resolved_path` | string or null | yes | Resolved canonical path (file-first) |
| `input.ambiguous` | boolean | yes | True when both file and folder exist |
| `input.error` | string or null | yes | Section-level error if check failed |
| `runs.active` | integer or null | yes | Count of active runs |
| `runs.pending_approvals` | integer or null | yes | Count of pending approvals |
| `runs.error` | string or null | yes | Section-level error if check failed |

**Validation rules**:
- `schema_version` MUST be present even on total failure
- At least one section MUST have non-null data (partial success)
- `error` field is mutually exclusive with section data being null
- `resolved_path` is null when neither file nor folder exists

**State transitions**: N/A (stateless snapshot)

### 2. Preflight Contract (YAML Frontmatter)

The declarative requirements block in SKILL.md frontmatter.

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

**Fields**:

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `requires_canon` | boolean | false | Whether canon binary must be available |
| `requires_initialized` | boolean | false | Whether `.canon/` must exist |
| `canonical_input` | string or null | null | Mode name for input detection |
| `system_context` | string or null | null | Required system-context value |
| `risk_required` | boolean | false | Whether risk must be provided |
| `zone_required` | boolean | false | Whether zone must be provided |
| `owner_optional` | boolean | true | Whether owner can be omitted |

**Validation rules**:
- All fields are optional; missing fields use defaults
- `canonical_input` must match a known Canon mode name when present
- `system_context` must be `"new"` or `"existing"` when present

### 3. Lifecycle Hook (TOML)

A hook entry in `.canon/hooks.toml`.

```toml
version = 1

[[hooks.after_publish.actions]]
id = "add-docs-to-git"
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
prompt = "Run input linter before starting?"
condition = ""
mode_filter = ["implementation", "change"]
```

**Fields**:

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `id` | string | yes | — | Unique hook identifier |
| `command` | string | yes | — | Shell command to propose |
| `description` | string | yes | — | Human-readable explanation |
| `optional` | boolean | yes | — | Whether hook can be skipped |
| `trusted` | boolean | no | false | Whether hook is pre-approved |
| `prompt` | string | no | null | Custom prompt for proposal |
| `condition` | string | no | null | Reserved for future conditions |
| `mode_filter` | array of string | no | null | Modes where hook applies (null = all) |

**Validation rules**:
- `id` must be unique within the event group
- `command` must not be empty
- `optional = false` means hook is mandatory (still proposed, not auto-executed)
- `trusted = false` (or missing) requires extra confirmation step
- `mode_filter` when present restricts hook to listed modes only

### 4. Hook Trace (in ai-provenance.md)

The record appended to the `ai-provenance.md` sidecar.

```markdown
## Hook Traces

### after_publish: add-docs-to-git

- **Event**: after_publish
- **Hook ID**: add-docs-to-git
- **Command**: `git add tech-docs/`
- **Trusted**: false
- **Proposal Outcome**: accepted
- **Execution Result**: success (exit code 0)
- **Timestamp**: 2026-05-28T14:35:00Z
```

**Fields**:

| Field | Description |
|-------|-------------|
| Event | Lifecycle event that triggered detection |
| Hook ID | Matches `id` from hooks.toml |
| Command | Full command that was proposed |
| Trusted | Whether hook was marked trusted |
| Proposal Outcome | accepted, declined, or skipped |
| Execution Result | success/failure with exit code, or "not executed" |
| Timestamp | When the trace was recorded |

## Relationships

```text
┌──────────────────────┐
│   canon-preflight.sh │──outputs──▶ Preflight Report (JSON)
└──────────────────────┘
           ▲
           │ reads preflight contract
┌──────────────────────┐
│   SKILL.md           │──declares──▶ Preflight Contract (YAML)
│   (frontmatter)      │
└──────────────────────┘
           │
           │ at lifecycle point
           ▼
┌──────────────────────┐
│   .canon/hooks.toml  │──declares──▶ Lifecycle Hooks
└──────────────────────┘
           │
           │ detect/propose/execute
           ▼
┌──────────────────────┐
│   ai-provenance.md   │◀──records── Hook Trace
└──────────────────────┘
```

## Entity Lifecycle

1. **Preflight Report**: Created fresh on every skill invocation; ephemeral
   (stdout only, not persisted).
2. **Preflight Contract**: Authored once per skill; updated on requirement
   changes; persisted in SKILL.md.
3. **Lifecycle Hook**: Authored by repo maintainer; persisted in
   `.canon/hooks.toml`; stable across runs.
4. **Hook Trace**: Created per hook execution attempt; appended to
   `ai-provenance.md`; persisted with run evidence.
