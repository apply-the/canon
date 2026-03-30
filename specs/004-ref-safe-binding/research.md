# Research: Runnable Skill Interaction and Ref-Safe Input Binding

## Decision 1: Repair the current shared preflight surface instead of adding a new interaction framework

**Decision**: extend `.agents/skills/canon-shared/scripts/check-runtime.sh`
and `.agents/skills/canon-shared/scripts/check-runtime.ps1` with typed-slot
validation, normalization, and richer failure metadata rather than building a
generic form engine or a second runtime layer.

**Rationale**: the existing bug sits in the current wrapper boundary:
`canon-pr-review` sends refs through a file-path-oriented preflight path, and
runnable skill prose lacks a precise typed retry contract. The smallest
trust-repair patch is to correct that shared boundary and align skill
instructions to it.

**Alternatives considered**:

- Put all fixes in `SKILL.md` prose only. Rejected because the shared preflight
  would still misclassify ref inputs.
- Add a generic multi-step interaction engine. Rejected because it broadens
  scope and violates the “thin deterministic frontend” constraint.

## Decision 2: Keep accepted `canon-pr-review` refs local-only in this increment

**Decision**: accept only `HEAD`, explicit local refs in `refs/heads/*`, and
short local branch names that resolve to existing local heads. Reject remote
refs such as `origin/main` and `refs/remotes/origin/main` in this patch.

**Rationale**: the trust problem is ambiguity, not lack of cleverness. Remote
refs create intent ambiguity, especially when local and remote names diverge.
The Canon runtime does not require remote-ref support to review a diff-backed
range today, so the patch should close the bug with the most deterministic
local contract first.

**Alternatives considered**:

- Accept tracked remote refs and normalize them automatically. Rejected because
  that changes command intent too easily and requires more repo-state magic.
- Leave remote-ref handling unresolved until implementation. Rejected because
  the user explicitly asked the plan to close this point early.

## Decision 3: Use a fixed ref-resolution order and canonicalize to `refs/heads/*`

**Decision**: resolve ref slots in this order:

1. literal `HEAD`
2. exact `refs/heads/<name>`
3. short local branch names resolved as `refs/heads/<name>`
4. remote-like forms classified as unsupported
5. all other unresolved values classified as invalid refs

Short local branch names normalize to explicit `refs/heads/<name>` for the
actual Canon command.

**Rationale**: this keeps retry guidance and the executed Canon command aligned.
The user sees a stable canonical form, and the frontend never has to guess
between a ref token and a path token in `canon-pr-review`.

**Alternatives considered**:

- Pass short branch names through unchanged. Rejected because retry guidance
  would remain less exact than the binding contract.
- Auto-map `main` to `master` or the reverse. Rejected because branch aliasing
  changes intent and should require explicit user confirmation.

## Decision 4: Keep preserved valid inputs only for the current interaction

**Decision**: preserve valid typed inputs only within the active runnable-skill
interaction. Do not persist them in `.canon/`, repo files, or generic
cross-skill memory.

**Rationale**: preserving inputs across one correction round improves trust,
but persistence beyond that makes the frontend appear smarter than its real
contract. The patch is about reliable binding, not hidden statefulness.

**Alternatives considered**:

- Persist inputs in repo-local files. Rejected because it creates stale state
  and violates bounded-context expectations.
- Preserve inputs across later turns automatically. Rejected because it can
  trigger the wrong Canon command from stale context.

## Decision 5: Separate missing file, invalid ref, and generic missing-input failures

**Decision**: preflight failures will distinguish:

- `missing-input`
- `invalid-input`
- `invalid-ref`
- `missing-file`
- `malformed-ref-pair`
- existing environment failures such as `cli-missing`, `version-incompatible`,
  `wrong-repo-context`, and `repo-not-initialized`

**Rationale**: the current trust failure comes partly from telling the user the
wrong thing to fix. Separate classes make retry prompts specific and keep
preflight guidance aligned with the exact failing slot and input kind.

**Alternatives considered**:

- Keep one broad `missing-input` bucket. Rejected because refs and file paths
  need different corrective actions.
- Depend on freeform message parsing. Rejected because shell and PowerShell
  parity would be fragile.

## Decision 6: Reuse typed run-id handling opportunistically, but keep `canon-approve` extras skill-local

**Decision**: `RunIdInput` becomes shared across `canon-status`, inspect
skills, `canon-resume`, and `canon-approve`, while `TARGET`, `BY`,
`DECISION`, and `RATIONALE` remain skill-local fields in this increment.

**Rationale**: the shared value is exact run-id handling and retry correctness.
Expanding the shared layer to all approval fields would broaden the patch
without improving the core proving case.

**Alternatives considered**:

- Leave run-id skills untouched. Rejected because they benefit cheaply from the
  same retry discipline.
- Type every approve field in the shared layer now. Rejected because it adds
  scope without being necessary for `canon-pr-review` trust repair.
