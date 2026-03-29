# Decision Log: Codex Skills Frontend for Canon

## D-001: Canon CLI stays the only execution engine

**Decision**: all supported skills are wrappers around Canon CLI, not a second
runtime.

**Rationale**: the feature improves UX, not runtime authority.

## D-002: Use explicit skill taxonomy, not a `canon` super-skill

**Decision**: keep one explicit skill per Canon workflow or operational action.

**Rationale**: Canon's mode semantics are a core product property and should
remain visible to users.

## D-003: Keep the full Canon taxonomy discoverable in phase 1

**Decision**: implement all Canon skills as discoverable repo-local skills and
use explicit support-state labeling rather than soft-hiding modeled workflows.

**Rationale**: the product correction requires discoverability for the whole
Canon taxonomy, and trust should come from honesty rather than omission.

## D-004: Keep helper scripts small and deterministic

**Decision**: create a shared support layer for preflight checks and standard
response shaping only.

**Rationale**: this reduces boilerplate without inventing a generic skill
framework.

## D-005: Treat modeled-only skills as support-state wrappers

**Decision**: modeled-only skills may exist in the taxonomy and repo layout,
but they never start Canon runs.

**Rationale**: honest support-state UX is better than silent omission or fake
behavior.

## D-006: Keep `canon-verification` intentionally limited but discoverable

**Decision**: `canon-verification` remains contract-defined, discoverable, and
explicitly `intentionally-limited` in phase 1.

**Rationale**: the `verify` surface is not yet implemented in Canon runtime and
must not create false expectations.

## D-007: Treat compatibility as semver first, command contract second

**Decision**: shared preflight checks should prefer `canon --version`, but
fallback to a deterministic CLI command-contract probe when the installed Canon
binary does not expose a version command.

**Rationale**: the current Canon CLI does not support `--version`, so the skill
layer needs a compatibility contract that is still actionable and honest.

## D-008: Use template stamping for the taxonomy sweep

**Decision**: the initial Canon skill taxonomy should be produced from one
shared `SKILL.md` template and a fixed field checklist, then refined only where
workflow behavior genuinely differs.

**Rationale**: this keeps the phase-1 taxonomy effort mechanical instead of
turning 19 skill folders into bespoke design work.

## D-009: Stop after the MVP Codex UX slice

**Decision**: implementation halts after Phase 2 until the first runnable `$`
workflow set has been exercised in real Codex usage and recorded in the
validation report.

**Rationale**: the value of this increment is discoverable, honest, Canon-backed
UX. That must be validated before spending time on lower-priority refinement.

## D-010: Use JSON-backed Canon commands for the runnable MVP set

**Decision**: the first runnable Codex skills should bind to Canon JSON output
surfaces so they can report real run ids, states, and evidence pointers without
re-parsing text.

**Rationale**: JSON output is the cleanest path to deterministic, Canon-backed
Codex summaries for the MVP skill set.

## D-011: `canon-init` is a hard stop, not a workflow chain

**Decision**: `canon-init` must execute only `canon init`, report the
initialization result, and stop. It may suggest follow-up skills, but it must
not start a requirements run or any other Canon workflow automatically.

**Rationale**: initialization is an enabling step, not a compound workflow. If
`canon-init` chains into a run, the skill layer stops being predictable.

## D-012: Status next steps must be conditional, not generic

**Decision**: `canon-status` may always point to inspection skills, but it must
only suggest `canon-approve` or `canon-resume` when Canon actually reports a
gated state, pending approvals, or an explicit approval target.

**Rationale**: suggesting unblock actions on a completed ungated run makes the
skill layer look sloppy and less trustworthy than the runtime it fronts.

## D-013: Invocation approvals and gate approvals have different follow-up paths

**Decision**: invocation-scoped approvals should point to `canon-resume`,
because Canon still needs continuation work after approval. Gate approvals such
as `gate:review-disposition` may complete the run immediately, so the next step
after approval becomes `canon-status`, with `canon-resume` only if Canon still
leaves the run incomplete.

**Rationale**: not every approval in Canon means the same thing operationally,
and the skill frontend must reflect that difference instead of flattening it.

## D-014: Modeled-only skills must state both current knowledge and missing execution

**Decision**: modeled-only skills must say what Canon already knows today about
the mode and what execution, gates, or evidence are still missing before the
mode becomes runnable end to end.

**Rationale**: a flat “not supported yet” message is discoverable, but not very
useful. Trust improves when the boundary is explicit and mode-specific.

## D-015: Closeout remains conditional on PowerShell validation

**Decision**: Unix-side implementation, walkthroughs, and documentation may
advance through Phase 5 and most of closeout, but the increment is not fully
closed until the PowerShell validator is executed on a host with `pwsh`.

**Rationale**: the frontend is intentionally cross-platform. Pretending the
Windows validation happened here would weaken the evidence model the increment
is supposed to strengthen.

## D-016: Documentation hygiene does not relax the cross-platform closeout gate

**Decision**: repo-relative path cleanup in persisted docs may be validated and
merged independently, but it does not satisfy or waive the pending PowerShell
validation required for full feature closeout.

**Rationale**: the documentation cleanup improves portability and repository
hygiene, but it does not add new Windows execution evidence. The closeout gate
must still track real cross-platform validation, not inferred confidence.

## D-017: Close cross-platform validation by fixing the PowerShell parity bugs

**Decision**: close the remaining frontend validation tasks only after running
the PowerShell validator on a real `pwsh` host and fixing any script parity
bugs exposed during that run.

**Rationale**: once `pwsh` became available on macOS, the remaining blocker was
not environment availability but correctness of the PowerShell scripts
themselves. Closing the increment required real execution plus targeted fixes in
`validate-canon-skills.ps1`, `check-runtime.ps1`, and
`render-support-state.ps1`, not a paper assertion of parity.
