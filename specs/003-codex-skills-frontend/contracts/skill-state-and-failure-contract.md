# Contract: Skill State and Failure Handling

## Support-State Contract

| Support State | Allowed Behavior | Prohibited Behavior |
| --- | --- | --- |
| `available-now` | Start or inspect real Canon workflows | Fabricate evidence or skip preflight |
| `modeled-only` | Report that the workflow is not runnable end to end, explain what Canon knows, explain what is missing, and route to nearest supported workflow when useful | Start a fake run or imply end-to-end support |
| `intentionally-limited` | Report the current limited surface and nearest useful alternatives | Pretend the limitation does not exist |
| `experimental` | Report experimental status and unstable boundaries when explicitly used | Present unstable behavior as production-ready |

## Runtime Dependency Failure Contract

| Failure | Required Response |
| --- | --- |
| Canon CLI missing | Report that `canon` is not installed and show the supported install path |
| Canon version incompatible | Report detected version, expected version contract, and corrective action |
| Repo not initialized | Report that `.canon/` is missing for the requested workflow and point to `canon-init` |
| Wrong repo context | Report that the user is outside the intended repo root or workspace and must switch context |
| Missing run id or input file | Report the exact missing input and show the retry form |

## Compatibility Detection Contract

- Prefer `canon --version` when the installed Canon binary supports it.
- If `canon --version` is unavailable, fall back to a deterministic command
  contract probe defined in `runtime-compatibility.toml`.
- A compatibility failure may therefore mean either:
  - a detected semver mismatch, or
  - a Canon command-contract mismatch against the expected runnable surface
- The failure response must state which compatibility check failed and what the
  user should do next.

## Determinism Rules

- Failure responses must be actionable and specific.
- Failure responses must not fabricate a partial Canon result.
- Support-state responses must not fabricate a run id.
- All Canon skills remain discoverable through `$`; trust comes from explicit
  labels, not from hiding skills.
- Helper scripts may standardize wording, but Canon remains the runtime source
  of truth.
