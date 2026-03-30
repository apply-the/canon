# Contract: PR Review Ref Binding

## Purpose

Define the exact accepted ref forms and normalization behavior for
`canon-pr-review` in this patch.

## Accepted Forms

| User Input Form | Validation Rule | Canonical Form Passed to Canon | Result |
| --- | --- | --- | --- |
| `HEAD` | Accept literal `HEAD` | `HEAD` | Accepted |
| `refs/heads/<name>` | Must resolve with `git show-ref --verify --quiet` | Same value | Accepted |
| `<name>` | Resolve as local `refs/heads/<name>` | `refs/heads/<name>` | Accepted if local branch exists |

## Rejected Forms

| User Input Form | Failure Class | Guidance |
| --- | --- | --- |
| `refs/remotes/<remote>/<name>` | `invalid-ref` with `FAILED_KIND=unsupported-remote-ref` | Use a local branch or explicit `refs/heads/<name>` |
| `<remote>/<name>` when no matching local branch exists | `invalid-ref` with `FAILED_KIND=unsupported-remote-ref` | Use a local branch or explicit `refs/heads/<name>` |
| non-empty unresolved token | `invalid-ref` | Retry with an existing local branch or `HEAD` |
| empty token | `missing-input` | Provide the missing ref slot |

## Resolution Order

For each slot:

1. check for literal `HEAD`
2. check for exact `refs/heads/*`
3. check for exact local branch resolution as `refs/heads/<raw-value>`
4. classify remote-like forms
5. classify any remaining unresolved value as `invalid-ref`

This order is deterministic and must be identical in Bash and PowerShell.

## Enforcement Boundary

- `.agents/skills/canon-shared/scripts/check-runtime.sh` and
  `.agents/skills/canon-shared/scripts/check-runtime.ps1` are the source of
  truth for ref classification and normalization.
- `.agents/skills/canon-pr-review/SKILL.md` must mirror the helper-enforced
  contract and must not widen accepted ref forms in prose.

## Pair Rules

- `base-ref` and `head-ref` are validated as `RefPairInput`
- both sides must be present before Canon runs
- a valid side is preserved if the other side is missing or invalid
- if both sides normalize to the same canonical ref, return
  `malformed-ref-pair`

## Guidance Rules

- repo-aware suggestions may come from local branch discovery only
- no auto-substitution between `main` and `master`
- no automatic remote tracking or fetch behavior
- retry guidance must show the exact form passed to Canon

## Examples

### Accepted

```text
Input: base master, head HEAD
Normalized: base refs/heads/master, head HEAD
Canon CLI: canon run --mode pr-review ... --input refs/heads/master --input HEAD --output json
```

### Deterministic Invalid Ref

```text
Repo local heads: master
Input: base main, head HEAD
Result: STATUS=invalid-ref
Suggested retry: base master
Exact CLI form: --input refs/heads/master --input HEAD
```

### Unsupported Remote Ref

```text
Input: base origin/main, head HEAD
Result: STATUS=invalid-ref
FAILED_KIND=unsupported-remote-ref
Guidance: use a local branch or explicit refs/heads/<name>
```
