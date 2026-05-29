# Research: Canon Skill Runtime Contracts

**Branch**: `061-skill-runtime-contracts`
**Date**: 2026-05-28
**Status**: Complete (all unknowns resolved)

## R-001: Partial-JSON Failure Pattern in Bash

**Decision**: Use per-section try/accumulate pattern with a guaranteed JSON
envelope on stdout. Each section (`canon`, `workspace`, `input`, `runs`)
populates independently; failures write an `"error"` field into that section
rather than aborting the script.

**Rationale**: The spec (FR-001, C-002) requires partial JSON on failure.
Bash lacks structured exceptions, so each section check runs in a subshell
or guarded block and appends its result to a growing JSON object. `jq` is
used for safe JSON construction where available; a fallback `json_escape()`
function handles environments without `jq`.

**Alternatives considered**:
- NDJSON streaming: harder for AI to parse as single object; rejected.
- Text output with regex: violates D-003 decision (JSON, not key=value).
- Trap-based exit handler: overly complex for partial success; rejected.

## R-002: TOML Schema for Lifecycle Hooks

**Decision**: Use `version = 1` at top level with `[[hooks.<event>.actions]]`
array-of-tables matching the acceptance scenarios in the spec (e.g.,
`[[hooks.after_publish.actions]]`).

**Rationale**: Canon's existing TOML conventions use `version` at top level
and `[[table]]` arrays for repeated entries (see `adapters.toml`,
`zones.toml`). The spec acceptance scenarios explicitly use
`[[hooks.after_publish.actions]]` and `[[hooks.before_run.actions]]` forms,
so the schema must match.

**Alternatives considered**:
- Flat `[[hook]]` with event field: loses grouping; conflicts with spec examples.
- Separate event definition table: overengineered for V1 scope.
- YAML for hooks: violates Canon's `.canon/` TOML convention (D-001).

## R-003: YAML Frontmatter Extension Safety

**Decision**: Adding `preflight:` as a nested YAML block inside existing
`---...---` fences is safe. The validator (`validate-canon-skills.sh`) uses
line-based grep for `name:`, `description:`, section headers, and fence
presence; it does not parse full YAML structure.

**Rationale**: Examined the validator source. It checks:
1. Fence presence (`^---$`)
2. Name matches directory (`^name: <skill>$`)
3. Description prefix (`^description: Use when `)
4. Section headers in body (`^## <Section>$`)

None of these patterns conflict with a `preflight:` block inside frontmatter.

**Alternatives considered**:
- Separate `.preflight.yaml` file: splits skill metadata; harder to discover.
- Inline comments in skill body: unstructured; not machine-parseable.
- New validator flag: useful later but not required for safe addition.

## R-004: Reuse from check-runtime.sh

**Decision**: `canon-preflight.sh` is a new script that reuses utility
functions (`trim()`, `is_placeholder()`, normalization helpers) from
`check-runtime.sh` via sourcing, but replaces the top-level flow and output
format entirely.

**Rationale**: The existing script outputs key=value format (incompatible with
FR-001 JSON requirement) and has mode-specific validation logic (ref
resolution, risk/zone inference) that belongs to run startup, not preflight
environment reporting. The utility functions are stable and well-tested.

**Alternatives considered**:
- Full copy-paste: DRY violation; maintenance burden.
- Modify check-runtime.sh in place: breaks existing skill consumers.
- Fold checks into each skill: violates DRY; no shared improvement path.

## R-005: Validator Compatibility

**Decision**: No changes to `validate-canon-skills.sh` are required for Phase A
(preflight script) or Phase B (YAML frontmatter addition). The validator is
transparent to new frontmatter fields.

**Rationale**: Validator only uses `grep` line matching. Future optional
enhancement: `--verify-preflight` flag that parses YAML with `yq` to confirm
structure. This is out of scope for this feature.

**Alternatives considered**:
- Pre-validate with yq: adds external dependency; overkill for V1.
- Rust-based validation: good long-term but out of scope (no crate changes).

## R-006: PowerShell Parity Strategy

**Decision**: `canon-preflight.ps1` mirrors `canon-preflight.sh` output shape
exactly, using `ConvertTo-Json` for safe JSON construction. PowerShell's
native JSON support eliminates the `jq` dependency concern.

**Rationale**: PowerShell has first-class JSON support via `ConvertTo-Json`
and `ConvertFrom-Json`. Error handling uses `try/catch` per section, mapping
directly to the partial-failure pattern.

**Alternatives considered**:
- Calling bash from PowerShell via WSL: fragile; not available everywhere.
- Single script with OS detection: complex; harder to test independently.

## R-007: Hook Consumption in Skills (Detect/Propose Pattern)

**Decision**: Skills check for `.canon/hooks.toml` at the relevant lifecycle
point, parse matching hooks for the current event and mode, and emit a
structured proposal block. The proposal format is a markdown block with
command, description, required/optional status, and trusted/untrusted
indication.

**Rationale**: The spec (FR-007, C-005) requires detect/explain/propose
semantics. Skills already emit structured markdown blocks. The trusted field
(C-005) adds a confirmation layer for untrusted hooks.

**Alternatives considered**:
- CLI-enforced hooks: requires Rust changes (out of scope per invariant).
- Hook daemon: overengineered for V1; violates bounded-impact risk.
- Auto-execution with opt-out: violates security model (C-005, FR-007).
