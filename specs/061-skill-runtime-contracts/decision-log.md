# Decision Log: Canon Skill Runtime Contracts

**Branch**: `061-skill-runtime-contracts`

## Design-Stage Decisions

### DL-001: Preflight JSON Output Format

**Decision**: Use a flat JSON object with four top-level sections (`canon`,
`workspace`, `input`, `runs`) rather than a nested hierarchy.

**Context**: The AI assistant needs to quickly map preflight YAML requirements
to JSON fields. Deep nesting adds parsing complexity without clarity benefit.

**Alternatives**:
- Deeply nested JSON with subsections: harder to grep/jq; rejected.
- Array of check results: loses semantic grouping; rejected.

**Consequences**: Each section can be independently checked and independently
errored. Simple `jq '.input.resolved_path'` access pattern.

---

### DL-002: Partial Failure Uses Section-Level Error Fields

**Decision**: Each section carries its own `"error"` field rather than a
top-level error array.

**Context**: C-002 requires partial JSON on failure. Co-locating error with
its section keeps the AI's evaluation logic local to each requirement check.

**Alternatives**:
- Top-level `"errors": [...]` array: loses locality; harder to map to YAML requirements.
- Separate `diagnostics` section: adds indirection; rejected.

**Consequences**: A section with `"error": "canon binary not found"` still has
`"available": false`; fields are populated to best-effort even on error.

---

### DL-003: Input Resolution Uses File-First Precedence

**Decision**: When both file and folder exist for a mode, `resolved_path`
points to the file and `ambiguous: true` is set.

**Context**: C-003 established this convention. File-backed input is the
simpler case and historically the more common pattern.

**Alternatives**:
- Folder-first: inconsistent with historical usage.
- Error on ambiguity: too strict; blocks valid states.
- No resolution (leave to caller): shifts complexity to every skill.

**Consequences**: Skills always get one path to consume. Ambiguity flag enables
warning without blocking.

---

### DL-004: hooks.toml Grouped by Event

**Decision**: Use `[[hooks.<event>.actions]]` array-of-tables grouping hooks by
their lifecycle event.

**Context**: Spec acceptance scenarios use this exact form
(`[[hooks.after_publish.actions]]`). Grouping by event makes detection at a
lifecycle point a single table lookup.

**Alternatives**:
- Flat `[[hook]]` with `event` field: requires filtering at every point.
- Separate files per event: filesystem clutter; harder to manage.

**Consequences**: Adding a new event is a new table group. Mode filtering is
per-hook, not per-event.

---

### DL-005: Trusted Field Defaults to False

**Decision**: Missing `trusted` field = `trusted: false`. Untrusted hooks
require extra confirmation.

**Context**: C-005 established this as the V1 security posture. Defense in
depth: even trusted hooks are proposed, never auto-executed.

**Alternatives**:
- Default true: violates principle of least privilege.
- No trusted field at all: loses future UX differentiation.

**Consequences**: Existing hooks without `trusted` field are safe by default.
Migration to trusted requires explicit opt-in by maintainer.

---

### DL-006: Hook Traces in ai-provenance.md

**Decision**: Hook execution traces are appended to the `ai-provenance.md`
sidecar alongside the run, under a `## Hook Traces` heading.

**Context**: C-001 decided this. The provenance sidecar already records
AI-generated lineage; hook traces are another form of execution evidence.

**Alternatives**:
- Separate `hook-traces.md`: fragments evidence; harder to find.
- TOML trace file: inconsistent with markdown sidecar convention.
- Append to run manifest: pollutes structured TOML with freeform data.

**Consequences**: Hook evidence is co-located and inspectable. The heading
convention enables future tooling to extract hook-specific traces.
