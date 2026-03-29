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

### Runnable Walkthroughs

- missing only `zone` after valid `owner` and `risk`
- retry after correcting one field without re-entering everything
- `canon-pr-review` with `master` and `HEAD`
- `canon-pr-review` with unresolved `main` in a master-only local repo
- missing file path for `canon-brownfield` or `canon-requirements`
- run-id-only correction for `canon-status` or `canon-resume`
