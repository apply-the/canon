# Contract: Runnable Skill Input Handling

## Purpose

Define the minimum typed-input and retry contract for executable Canon skills in
this patch.

## Applicability

Directly applicable:

- `canon-pr-review`
- `canon-brownfield`
- `canon-requirements`

Opportunistic reuse:

- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`
- `canon-inspect-artifacts`
- `canon-approve`
- `canon-resume`

## Input-Slot Contract

| Slot Kind | Required Behavior |
| --- | --- |
| `OwnerField` | Validate non-empty content and preserve the value after it passes |
| `RiskField` | Accept runtime-recognized risk tokens and normalize to canonical hyphenated form |
| `ZoneField` | Accept runtime-recognized zone tokens and normalize to canonical lowercase form |
| `RunIdInput` | Validate against `.canon/runs/<RUN_ID>` before Canon execution |
| `FilePathInput` | Validate against filesystem existence relative to repo root or explicit absolute path |
| `RefInput` | Validate through Git ref resolution, never through filesystem existence |
| `RefPairInput` | Validate base/head together after each side is individually classified as a ref |

## Interaction Contract

- Executable skills may ask incrementally for missing inputs.
- Skills must identify the exact missing or invalid slot.
- Skills must preserve already valid slots inside the current interaction.
- Skills must not ask the user to restate every field after one correction.
- Skills must not persist preserved input state outside the current
  interaction.

## Preflight Output Contract

The shared preflight helper returns key/value output with:

- required keys:
  - `STATUS`
  - `CODE`
  - `PHASE`
  - `COMMAND`
  - `REPO_ROOT`
  - `MESSAGE`
  - `ACTION`
- optional keys:
  - `FAILED_SLOT`
  - `FAILED_KIND`
  - `NORMALIZED_RUN_ID`
  - `NORMALIZED_INPUT_1`
  - `NORMALIZED_REF_1`
  - `NORMALIZED_REF_2`

Runtime-aligned normalization accepted by this patch:

- `RiskField` accepts `low-impact` and `LowImpact`, `bounded-impact` and
  `BoundedImpact`, `systemic-impact` and `SystemicImpact`
- `ZoneField` accepts `green` and `Green`, `yellow` and `Yellow`, `red` and
  `Red`

Ready responses may also emit helper diagnostics such as `VERSION_KIND` and
`DETECTED_VERSION`, but retry and validation logic must key off the required
contract fields above rather than those diagnostics.

`PHASE` must be:

- `preflight` for helper-side validation failures or ready states
- `canon-execution` for failures returned after Canon was actually invoked

## Failure Contract

| Status | Meaning | Required Guidance |
| --- | --- | --- |
| `cli-missing` | Canon unavailable | Show supported install step |
| `version-incompatible` | Canon contract mismatch | Show reinstall/update action |
| `wrong-repo-context` | Not in intended Git repo | Ask user to switch repo context |
| `repo-not-initialized` | `.canon/` missing | Route to `canon-init` |
| `missing-input` | Required slot absent | Ask only for missing slot |
| `invalid-input` | Present value malformed or unsupported | Ask only for failing slot |
| `invalid-ref` | Ref slot unresolved or unsupported | Ask only for failing ref slot |
| `missing-file` | File-path slot not found | Ask only for missing path |
| `malformed-ref-pair` | Base/head pair invalid together | Ask only for pair correction |

## Rendering Contract

Retry guidance must always include:

- what was preserved
- what still needs correction
- the exact Canon CLI form that the current binding accepts

Rendering order:

1. semantic prompt
2. exact Canon CLI form

Example:

```text
Preserved: owner reviewer, risk bounded-impact, zone yellow, head ref HEAD
Need: base ref
Retry using: --input refs/heads/master --input HEAD
```
