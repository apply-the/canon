# Preflight YAML Contract

The `preflight:` block in SKILL.md YAML frontmatter declares what the skill
requires from the runtime environment. The AI assistant parses this block,
evaluates it against the preflight JSON report, and surfaces only the
unmet requirements before proceeding.

## Contract Shape

```yaml
preflight:
  requires_canon: <boolean>       # canon binary must be on PATH
  requires_initialized: <boolean> # .canon/ directory must exist
  canonical_input: <mode-name>    # mode name for input file/folder detection
  system_context: <string|null>   # "new" or "existing" (null = not required)
  risk_required: <boolean>        # user must provide risk classification
  zone_required: <boolean>        # user must provide zone classification
  owner_optional: <boolean>       # owner may be omitted (default: true)
```

## Evaluation Rules

When the AI encounters a `preflight:` block, it:

1. Runs `canon-preflight.sh --mode <canonical_input>` (or `.ps1` on Windows).
2. Parses the JSON report.
3. For each declared requirement, checks the corresponding JSON field:
   - `requires_canon: true` checks `canon.available == true`
   - `requires_initialized: true` checks `canon.initialized == true`
   - `canonical_input` checks `input.resolved_path != null`
   - `system_context` checks that the user supplied matching context
   - `risk_required: true` checks that risk was provided or can be prompted
   - `zone_required: true` checks that zone was provided or can be prompted
4. For each unmet requirement, surfaces a targeted error with remediation hint.
5. Does NOT fall back to prose preflight instructions when YAML block is present.

## Precedence

When both `preflight:` YAML and legacy prose preflight instructions exist:

- YAML is authoritative.
- AI MUST NOT interpret or execute legacy prose as a runtime contract.
- Legacy prose MUST be marked with deprecation comment during migration:

```markdown
<!-- DEPRECATED: preflight behavior is governed by the `preflight:` block.
     Do not use this prose as an execution contract. -->
```

## Error Surfacing Examples

```text
Preflight check failed:
- Canon is not initialized (.canon/ directory missing)
  Suggested fix: Run `$canon-init` to initialize Canon in this repository.

- Canonical input missing (canon-input/implementation.md not found)
  Suggested fix: Author the implementation brief at canon-input/implementation.md
  or use canon-input/implementation/ for a folder-backed packet.
```

## Compatibility

- Skills without a `preflight:` block continue to use legacy prose; no change.
- The `preflight:` block is additive; removing it reverts to prose behavior.
- The JSON schema version (`schema_version: 1`) is checked before field access.
