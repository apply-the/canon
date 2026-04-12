# Validation Report: Codex Skills Frontend for Canon

## MVP Gate

The first release gate for this increment is Phase 0 + Phase 1 + Phase 2 only.
No work should proceed to later phases until:

- the full Canon skill taxonomy exists under `.agents/skills`
- runnable skills for `canon-init`, `canon-requirements`, `canon-status`,
  `canon-inspect-invocations`, and `canon-inspect-evidence` are in place
- non-runnable skills are discoverable and explicitly labeled
- a real `$` walkthrough in Codex has been recorded here

## Planned Structural Validation

- validate `.agents/skills` tree integrity
- validate one folder per declared Canon skill plus `canon-shared`
- validate metadata completeness for every `SKILL.md`
- validate that all Canon skills are discoverable and match the declared
  support-state policy
- validate shared reference files and helper script presence

## Planned Logical Validation

- walkthrough validation for `canon-init`
- walkthrough validation for `canon-requirements`
- walkthrough validation for `canon-status`
- walkthrough validation for `canon-inspect-invocations`
- walkthrough validation for `canon-inspect-evidence`
- walkthrough validation for `canon-approve` plus `canon-resume`
- walkthrough validation for `canon-brownfield`
- walkthrough validation for `canon-pr-review`
- action-chip contract validation for `Approve generation...`, `Resume run`,
  and `Inspect evidence` visibility and fallback text

## Planned Failure Validation

- missing `canon` CLI
- incompatible Canon version
- repo not initialized for Canon
- wrong repo context
- missing run id
- missing input file or ref

Incompatible version validation must cover both:

- explicit semver mismatch when `canon --version` is available
- command-contract mismatch when compatibility must be probed without a version
  command

## Planned Modeled-Only Honesty Validation

- `canon-architecture`
- `canon-review`
- `canon-verification`

Each of these must prove:

- no fake run id
- no fake Canon run summary
- clear support-state message
- clear statement of what Canon already knows about the mode
- clear statement of what is missing before it becomes runnable end to end
- nearest supported alternative when one is honest and useful

## Planned Independent Validation

- review skill descriptions for overlap and routing ambiguity
- review available-now emphasis and full-taxonomy discoverability against README
  and quickstart
- review failure messages for actionability and determinism
- review that support-state wrappers never claim Canon executed unsupported work
- review that chip-capable hosts still retain `Possible Actions:` and
  `Recommended Next Step:` as text fallback

## Planned MVP Walkthrough Record

The MVP walkthrough must record:

- how `$canon-init` handles a missing `.canon/`
- how `$canon-requirements` starts a real Canon run
- how `$canon-status` returns real run state
- how `$canon-inspect-invocations` and `$canon-inspect-evidence` point back to
  `.canon/`
- the exact stop decision before later phases are authorized

## Executed Structural Validation

- Shell validator executed with:
  `/bin/bash scripts/validate-canon-skills.sh`
- Result: pass
- Verified:
  - full `.agents/skills` taxonomy presence
  - required `SKILL.md` sections
  - support-state labeling
  - overlap boundaries for `review` vs `pr-review`, `refactor` vs `brownfield`,
    and `discovery` vs `requirements`
  - `canon-init` is explicitly forbidden from chaining into follow-up runs or
    claiming run-id/state output
  - fake-run prohibition for modeled-only and intentionally-limited skills
- PowerShell validator execution is still pending on a host with `pwsh`
  available; the `.ps1` validator file exists and mirrors the shell contract.

## Executed MVP Walkthrough

Walkthrough workspace: `/tmp/canon-skills-3dmb3Q`

1. Preflight for requirements before initialization:
   - command:
     `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh --command requirements --repo-root /tmp/canon-skills-3dmb3Q --require-init --owner staff-engineer --risk bounded-impact --zone yellow --input idea.md`
   - result: `repo-not-initialized`
   - code: `13`
   - action: `Run $canon-init or canon init in /tmp/canon-skills-3dmb3Q first.`
2. Runtime initialization:
   - command: `canon init --output json`
   - result: `.canon/` created successfully in the temp repository
3. Preflight after initialization:
   - result: `ready`
   - code: `0`
   - compatibility mode: `command-contract`
4. Requirements run:
   - command:
     `canon run --mode requirements --risk bounded-impact --zone yellow --owner staff-engineer --input idea.md --output json`
   - run id: `019d35d6-c118-7820-a906-bafcac824707`
   - final state from `canon status --run ... --output json`: `Completed`
5. Inspection surfaces:
   - `canon inspect invocations --run ... --output json` returned `4`
     invocation entries
   - `canon inspect evidence --run ... --output json` returned `1`
     evidence entry

## Executed Codex Walkthrough In A Real Repository

Walkthrough workspace: `/Users/rt/workspace/java-html-sanitizer`

1. `$canon-requirements` before initialization:
   - result: repo-not-initialized
   - behavior: no run started, no fake run id, actionable next step to
     `$canon-init`
2. `$canon-init`:
   - result: `.canon/` created successfully
   - boundary: no automatic follow-up workflow, no run id, no run state
3. `$canon-requirements` after initialization:
   - run id: `019d367b-62c2-7d23-8315-80e9ee319243`
   - state: `Completed`
4. `$canon-status`:
   - result: real Canon-backed state for the same run id
5. `$canon-inspect-invocations`:
   - result: `4` invocation records, including one denied workspace edit
6. `$canon-inspect-evidence`:
  - result: readable artifact provenance plus generation and validation lineage
   - artifact provenance: `6` requirements artifacts under `.canon/artifacts/.../requirements/`

This walkthrough confirms that the Codex-facing runnable skills stay
subordinate to the real Canon runtime and that artifacts are presented as
evidence of governed execution, not as the whole product.

## Executed Unblock And Continue Validation

Walkthrough workspace: `/tmp/canon-phase345-AASyAH/brownfield`

- command:
  `canon run --mode brownfield-change --risk systemic-impact --zone yellow --owner architect --input brownfield.md --output json`
- run id: `019d36b0-c957-7ae3-8d49-8a369b9cf750`
- initial state: `AwaitingApproval`
- pending approval target:
  `invocation:019d36b0-c957-7ae3-8d49-8a369b9cf750-generate`
- approval command:
  `canon approve --run 019d36b0-c957-7ae3-8d49-8a369b9cf750 --target invocation:019d36b0-c957-7ae3-8d49-8a369b9cf750-generate --by principal-engineer --decision approve --rationale "..."`
- continuation command:
  `canon resume --run 019d36b0-c957-7ae3-8d49-8a369b9cf750`
- final status:
  `Completed`
- emitted artifacts:
  `change-surface.md`, `decision-record.md`, `implementation-plan.md`,
  `legacy-invariants.md`, `system-slice.md`, `validation-strategy.md`

This confirms the unblock flow for invocation-scoped approvals:

- `canon-approve` stays tied to an explicit Canon target
- `canon-resume` continues the same run id
- `canon-inspect-artifacts` can point back to the emitted file set

## Executed Deeper Delivered-Mode Validation

Walkthrough workspace: `/tmp/canon-phase345-AASyAH/pr-review`

- command:
  `canon run --mode pr-review --risk bounded-impact --zone yellow --owner reviewer --input refs/heads/main --input HEAD --output json`
- run id: `019d36b0-c9ec-7443-b2dd-28ff8a64e6bb`
- initial state: `AwaitingApproval`
- approval target: `gate:review-disposition`
- approval command:
  `canon approve --run 019d36b0-c9ec-7443-b2dd-28ff8a64e6bb --target gate:review-disposition --by principal-engineer --decision approve --rationale "..."`
- post-approval status:
  `Completed`
- readable evidence surface:
  review artifacts under `.canon/artifacts/.../pr-review/`
- emitted review artifacts:
  `boundary-check.md`, `contract-drift.md`, `decision-impact.md`,
  `duplication-check.md`, `missing-tests.md`, `pr-analysis.md`,
  `review-summary.md`

This confirms the delivered `pr-review` skill boundary:

- the skill maps to a real diff-backed Canon run
- review evidence stays inspectable while readable file pointers stay under `.canon/artifacts/`
- `gate:review-disposition` approval may complete the run directly, without a
  required `resume`

## Public Output Boundary Validation

- Standard user-facing inspect and status output no longer expose
  `.canon/runs/.../evidence.toml` or related run-state TOML files as readable
  follow-on paths.
- Readable file pointers are limited to `.canon/artifacts/...` when Canon
  emitted them.
- Internal persistence under `.canon/runs/...` remains intact and is still
  covered by lower-level runtime contracts and contract tests.

## Action-Chip Contract Validation

- `Approve generation...` is defined as a governed approval affordance, not a
  blind `Proceed with generation` shortcut.
- `Resume run` remains tied to real continuation eligibility on the same run
  id.
- `Inspect evidence` remains the preferred gated-state affordance when Canon
  has not yet emitted a readable artifact packet.
- Text fallback through `Possible Actions:` and `Recommended Next Step:`
  remains mandatory even if a future host renders chips.

## Executed Failure Validation

Shell-side failure checks were executed with:

- `/bin/bash .agents/skills/canon-shared/scripts/check-runtime.sh ...`

Observed deterministic failures:

- missing CLI:
  `STATUS=cli-missing`, `CODE=10`, action points to the supported install
  command
- incompatible CLI contract:
  `STATUS=version-incompatible`, `CODE=11`, action points to reinstalling Canon
- wrong repo context:
  `STATUS=wrong-repo-context`, `CODE=12`, action points to switching into the
  intended Git repository
- missing `.canon/`:
  `STATUS=repo-not-initialized`, `CODE=13`, action points to `$canon-init`
- missing run id:
  `STATUS=missing-input`, `CODE=14`, action points to correcting the run id
- missing input file:
  `STATUS=missing-input`, `CODE=14`, action points to providing a valid file or
  ref

Failure validation root: `/tmp/canon-failure-sa3jCd`

## Executed Modeled-Only And Boundary Validation

- Shell validator executed with:
  `/bin/bash scripts/validate-canon-skills.sh`
- Result: pass
- Verified:
  - all Canon skills remain discoverable through `$`
  - `modeled-only` and `intentionally-limited` labels are explicit
  - support-state skills do not contain runnable `canon run` or fake `Run ID`
    forms
  - overlap boundaries stay explicit for:
    - `review` vs `pr-review`
    - `refactor` vs `brownfield`
    - `discovery` vs `requirements`
  - `canon-verification` remains intentionally limited rather than pretending a
    runnable `verify` flow

## Independent Review

Independent review was performed against the implemented skill set and
references rather than the planning language alone.

Findings:

- the strongest UX regression found during validation was `canon-init`
  chaining into a follow-up run; that is now fixed and structurally validated
- `canon-status` guidance needed tightening so completed ungated runs do not
  suggest unblock actions; the support documents now make those next steps
  conditional
- the remaining explicit risk is cross-platform closeout: PowerShell validation
  is still pending on a host with `pwsh`

## MVP Stop Decision

- Phase 0, Phase 1, and Phase 2 now have runnable substance.
- The first runnable `$` workflow set is defined and mapped cleanly to Canon
  CLI behavior.
- Later phases remain intentionally deferred until the Codex UX is reviewed
  against this MVP slice.

## Current Closeout Status

- Phase 3, Phase 4, and Phase 5 now have Unix-side implementation and validation
  evidence.
- Runnable skills map back to real Canon commands and runtime state.
- Support-state skills remain discoverable and do not fabricate runs.
- Artifacts are presented as durable evidence of governed execution, not as
  standalone product output.
- PowerShell validation remains the only explicit blocker to full closeout on
  this host.
- Open tasks blocked on that host limitation:
  - `T039`
  - `T070`
  - `T072`
  - `T073`
  - `T078`

## Revalidation After Repo-Relative Document Cleanup

Revalidated on `2026-03-29` after replacing committed absolute local paths with
repo-relative paths in the persisted planning and guidance artifacts.

- Shell structural validator rerun:
  `/bin/bash scripts/validate-canon-skills.sh`
- Result: pass
- Confirmed again:
  - Canon skill structure still validates after the documentation cleanup
  - support-state labels and overlap boundaries remain intact
  - fake-run protections still hold for modeled-only and intentionally-limited
    skills

Shell failure-path checks were also rerun explicitly on `2026-03-29`:

- missing CLI:
  reproduced with `PATH=/usr/bin:/bin`; returned `STATUS=cli-missing`,
  `CODE=10`
- incompatible CLI contract:
  reproduced with a temporary shim `canon` binary; returned
  `STATUS=version-incompatible`, `CODE=11`
- wrong repo context:
  reproduced from a non-git temp directory; returned
  `STATUS=wrong-repo-context`, `CODE=12`
- missing `.canon/`:
  reproduced from an uninitialized temp git repo; returned
  `STATUS=repo-not-initialized`, `CODE=13`
- missing run id:
  reproduced with `--run-id deadbeef`; returned `STATUS=missing-input`,
  `CODE=14`
- missing input file:
  reproduced with `--input missing.md`; returned `STATUS=missing-input`,
  `CODE=14`

Environment note:

- `pwsh` was installed on this host on `2026-03-29`, enabling direct
  PowerShell validation rather than leaving cross-platform closeout pending.

## Executed PowerShell Structural Validation

- PowerShell version:
  `PowerShell 7.6.0`
- PowerShell validator executed with:
  `pwsh -File scripts/validate-canon-skills.ps1`
- Result: pass
- Additional smoke validation:
  `pwsh -File .agents/skills/canon-shared/scripts/render-support-state.ps1`
  now renders correct `modeled-only` and `intentionally-limited` messages
- Validation fixes required before pass:
  - `scripts/validate-canon-skills.ps1` had PowerShell-specific parser and
    regex issues that did not appear in the shell validator
  - `.agents/skills/canon-shared/scripts/check-runtime.ps1` incorrectly
    evaluated the mode probe output and used an input parameter name that
    collided with PowerShell automatic variables
  - `.agents/skills/canon-shared/scripts/render-support-state.ps1` now accepts
    both the short parameter names and explicit aliases such as
    `-SupportState` and `-SkillName`

## Executed PowerShell Failure Validation

PowerShell-side failure checks were executed on `2026-03-29`.

Observed deterministic outcomes:

- ready:
  `STATUS=ready`, `CODE=0`
- missing CLI:
  `STATUS=cli-missing`, `CODE=10`
- incompatible CLI contract:
  `STATUS=version-incompatible`, `CODE=11`
- wrong repo context:
  `STATUS=wrong-repo-context`, `CODE=12`
- missing `.canon/`:
  `STATUS=repo-not-initialized`, `CODE=13`
- missing run id:
  `STATUS=missing-input`, `CODE=14`
- missing input file:
  `STATUS=missing-input`, `CODE=14`

These PowerShell checks now match the intended shell-side contract rather than
diverging on quoting or array-handling behavior.

## Final Closeout Status

- Shell and PowerShell structural validation both pass.
- Shell and PowerShell failure-path validation both produce deterministic,
  actionable outputs.
- Runnable skills remain Canon-backed.
- Support-state skills remain honest and non-fabricating.
- Persisted repository artifacts now use repo-relative paths instead of local
  absolute paths.
- No hidden blockers remain for this increment on the current host.
