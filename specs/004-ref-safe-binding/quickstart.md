# Quickstart: Implementing the Runnable Skill Trust-Repair Patch

## Goal

Apply the narrow patch that makes runnable skills collect inputs reliably,
preserve valid values across one correction round, and bind `canon-pr-review`
refs safely to the Canon CLI contract.

## Implementation Order

1. Update shared preflight in:
   - `.agents/skills/canon-shared/scripts/check-runtime.sh`
   - `.agents/skills/canon-shared/scripts/check-runtime.ps1`
2. Update direct-scope skill instructions:
   - `.agents/skills/canon-pr-review/SKILL.md`
   - `.agents/skills/canon-brownfield/SKILL.md`
   - `.agents/skills/canon-requirements/SKILL.md`
3. Apply opportunistic run-id improvements to:
   - `.agents/skills/canon-status/SKILL.md`
   - `.agents/skills/canon-inspect-invocations/SKILL.md`
   - `.agents/skills/canon-inspect-evidence/SKILL.md`
   - `.agents/skills/canon-inspect-artifacts/SKILL.md`
   - `.agents/skills/canon-approve/SKILL.md`
   - `.agents/skills/canon-resume/SKILL.md`
4. Update validators:
   - `scripts/validate-canon-skills.sh`
   - `scripts/validate-canon-skills.ps1`
5. Record outcomes in `validation-report.md`

## Key Implementation Rules

- Keep Canon CLI as the only execution engine.
- Preserve valid inputs only inside the current interaction.
- Use `--ref` for `canon-pr-review` preflight and keep `--input` for file paths.
- Accept only local ref forms in this patch.
- Show semantic retry guidance first and exact Canon CLI form second.

## Manual Verification Checklist

### Structural

```bash
/bin/bash scripts/validate-canon-skills.sh
pwsh -File scripts/validate-canon-skills.ps1
git diff --check
```

### Shared Preflight Probes

#### Bash: missing `zone` after valid `owner` and `risk`

```bash
/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh \
   --command requirements \
   --repo-root "$PWD" \
   --require-init \
   --owner reviewer \
   --risk bounded-impact \
   --input specs/004-ref-safe-binding/spec.md
```

Expected outcome:

- returns `STATUS=missing-input`
- returns `FAILED_SLOT=zone`
- preserves the provided owner and risk for retry rendering

#### Bash: missing file path while ownership metadata is valid

```bash
/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh \
   --command brownfield-change \
   --repo-root "$PWD" \
   --require-init \
   --owner reviewer \
   --risk bounded-impact \
   --zone yellow \
   --input missing-brief.md
```

Expected outcome:

- returns `STATUS=missing-file`
- returns `FAILED_SLOT=input-path`
- does not reclassify the failure as missing ownership metadata

#### Bash: `canon-pr-review` with local refs

```bash
/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh \
  --command pr-review \
  --repo-root "$PWD" \
  --require-init \
  --owner reviewer \
  --risk bounded-impact \
  --zone yellow \
  --ref master \
  --ref HEAD
```

Expected outcome:

- returns `STATUS=ready`
- emits canonical normalized ref output for the base/head pair

#### Bash: `canon-pr-review` with unresolved `main` in a master-only local repo

```bash
/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh \
  --command pr-review \
  --repo-root "$PWD" \
  --require-init \
  --owner reviewer \
  --risk bounded-impact \
  --zone yellow \
  --ref main \
  --ref HEAD
```

Expected outcomes:

- first case returns `STATUS=ready` and normalized ref output
- second case returns `STATUS=invalid-ref` and suggests the correct local
  branch without calling it a missing file

#### PowerShell: missing `zone` after valid `owner` and `risk`

```powershell
pwsh -File .agents/skills/canon-shared/scripts/check-runtime.ps1 \
   -Command requirements \
   -RepoRoot $PWD.Path \
   -RequireInit \
   -Owner reviewer \
   -Risk bounded-impact \
   -InputPath specs/004-ref-safe-binding/spec.md
```

Expected outcome:

- returns `STATUS=missing-input`
- returns `FAILED_SLOT=zone`
- preserves the provided owner and risk for retry rendering

#### PowerShell: missing file path while ownership metadata is valid

```powershell
pwsh -File .agents/skills/canon-shared/scripts/check-runtime.ps1 \
   -Command brownfield-change \
   -RepoRoot $PWD.Path \
   -RequireInit \
   -Owner reviewer \
   -Risk bounded-impact \
   -Zone yellow \
   -InputPath missing-brief.md
```

Expected outcome:

- returns `STATUS=missing-file`
- returns `FAILED_SLOT=input-path`
- does not reclassify the failure as missing ownership metadata

#### PowerShell: `canon-pr-review` with local refs and unresolved refs

```powershell
pwsh -File .agents/skills/canon-shared/scripts/check-runtime.ps1 \
   -Command pr-review \
   -RepoRoot $PWD.Path \
   -RequireInit \
   -Owner reviewer \
   -Risk bounded-impact \
   -Zone yellow \
   -RefName master, HEAD
```

```powershell
pwsh -File .agents/skills/canon-shared/scripts/check-runtime.ps1 \
   -Command pr-review \
   -RepoRoot $PWD.Path \
   -RequireInit \
   -Owner reviewer \
   -Risk bounded-impact \
   -Zone yellow \
   -RefName main, HEAD
```

Expected outcomes:

- first case returns `STATUS=ready` and normalized ref output
- second case returns `STATUS=invalid-ref` without any file-path wording

### Runnable Walkthroughs

- missing only `zone` after valid `owner` and `risk`
- retry after correcting one field without re-entering everything
- `canon-pr-review` with `master` and `HEAD`
- `canon-pr-review` with unresolved `main` in a master-only local repo
- missing file path for `canon-brownfield` or `canon-requirements`
- run-id-only correction for `canon-status` or `canon-resume`

For each walkthrough, capture:

- the reported `STATUS`
- the reported `PHASE`
- the `FAILED_SLOT` or normalized output keys when present
- the exact retry text shown to the user
