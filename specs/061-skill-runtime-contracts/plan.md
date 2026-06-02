# Implementation Plan: Canon Skill Runtime Contracts

**Branch**: `061-skill-runtime-contracts` | **Date**: 2026-05-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/061-skill-runtime-contracts/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Replace the existing key=value `check-runtime.sh` preflight output with a
structured JSON preflight script (`canon-preflight.sh` / `.ps1`), add
declarative `preflight:` YAML frontmatter to SKILL.md files, and introduce
`.canon/hooks.toml` lifecycle hooks with detect/propose semantics. The feature
modifies shell scripts and skill markdown only; no Rust crate changes.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact; modifies shared skill infrastructure
and shell scripts but does not touch the Rust CLI, governance semantics, or
packet contracts. Regression risk is limited to skill invocation reliability
across AI hosts.
**Scope In**: Preflight JSON script, PowerShell equivalent, declarative YAML
frontmatter in 3 skills (`canon-implementation`, `canon-change`,
`canon-publish`), `.canon/hooks.toml` schema, hook consumption pattern in
skills, validation script updates, test fixtures, documentation.
**Scope Out**: Rust CLI changes, CLI-enforced hook execution, plugin
marketplace, install/uninstall lifecycle, remote hooks, automatic mutation
hooks, new governance modes, new approval semantics, migration of all 20+
skills.

**Invariants**:

- Existing skills with prose-only preflight MUST continue working unchanged
  until explicitly migrated.
- Canon governance semantics, approval state, and packet contracts MUST NOT be
  affected by this feature.
- Hook execution MUST always leave an inspectable trace; no opaque side-effects.
- The Rust codebase under `crates/` MUST NOT be modified by this feature.
- `scripts/validate-canon-skills.sh` MUST pass for all skills after changes.

**Decision Log**: `specs/061-skill-runtime-contracts/decision-log.md`
**Validation Ownership**: Generation (AI + scripts) separated from validation
(skill validator, manual JSON schema checks, independent host testing).
**Approval Gates**: None required at bounded-impact; peer review of skill
changes sufficient.

## Technical Context

**Language/Version**: Bash 5.x (macOS/Linux), PowerShell 7.x (cross-platform)
**Primary Dependencies**: `jq` (JSON validation), `canon` CLI (version
detection), `git` (workspace detection), existing `check-runtime.sh` as
migration baseline
**Storage**: `.canon/hooks.toml` (TOML), preflight JSON to stdout (ephemeral),
hook traces in `ai-provenance.md` sidecar
**Testing**: `scripts/validate-canon-skills.sh`, `jq --exit-status` for JSON
schema, manual invocation under degraded conditions, host-portability testing
**Target Platform**: macOS, Linux, Windows (via PowerShell), AI hosts (Copilot,
Codex, Claude)
**Project Type**: Shell scripts + Markdown skill documents (no compiled code)
**Existing System Touchpoints**:
  - `.agents/skills/canon-shared/scripts/check-runtime.sh` (existing, to be
    extended/superseded by `canon-preflight.sh`)
  - `.agents/skills/canon-implementation/SKILL.md` (frontmatter addition)
  - `.agents/skills/canon-change/SKILL.md` (frontmatter addition)
  - `.agents/skills/canon-publish/SKILL.md` (frontmatter addition)
  - `scripts/validate-canon-skills.sh` (may need updates for YAML validation)
**Performance Goals**: Preflight script < 2s; hook detection < 200ms
**Constraints**: Partial-JSON on failure (never crash to stderr only), backward
compatible with existing skill validator
**Scale/Scope**: 3 skills migrated, 1 new script (bash + ps1), 1 new TOML
schema, 1 new hook consumption pattern

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/061-skill-runtime-contracts/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── preflight-json-schema.json
│   ├── preflight-yaml-contract.md
│   └── hooks-toml-schema.md
└── tasks.md
```

### Source Code (repository root)

```text
.agents/skills/canon-shared/scripts/
├── check-runtime.sh           (existing, unchanged for backward compat)
├── preflight-utils.sh         (new: shared utility functions for preflight)
├── preflight-utils.ps1        (new: PowerShell equivalent utilities)
├── canon-preflight.sh         (new: JSON preflight output)
└── canon-preflight.ps1        (new: PowerShell equivalent)

.agents/skills/canon-implementation/
└── SKILL.md                   (modified: add preflight: YAML block)

.agents/skills/canon-change/
└── SKILL.md                   (modified: add preflight: YAML block)

.agents/skills/canon-publish/
└── SKILL.md                   (modified: add preflight: YAML block)

tests/fixtures/
├── preflight/
│   ├── full-environment.json
│   ├── missing-canon.json
│   ├── missing-input.json
│   ├── partial-failure.json
│   └── ambiguous-input.json
└── hooks/
    ├── valid-hooks.toml
    ├── malformed-hooks.toml
    └── trusted-untrusted-mix.toml

tech-docs/guides/
└── skill-runtime-contracts.md (new: user-facing guide)
```

**Structure Decision**: Single-project flat layout. Shell scripts live under
`.agents/skills/canon-shared/scripts/` alongside the existing
`check-runtime.sh`. No new directories at repo root; fixtures and docs follow
existing conventions.

## Complexity Tracking

> No constitution violations. All gates pass.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| — | — | — |
