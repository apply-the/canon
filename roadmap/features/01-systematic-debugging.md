# 01 - Systematic Debugging

## Problem
Currently, day-to-day bug fixes are routed through generic `change` or `implementation` modes. These modes do not strictly enforce debugging best practices. The `incident` mode exists, but it is tailored for reactive production crisis management (blast radius, containment), not standard systematic troubleshooting.

## Proposal
Introduce a dedicated `debugging` or `bugfix` mode inspired by systematic troubleshooting methodologies. It should enforce rigid packet gates before allowing code changes:
1. **Reproduction Gate**: The packet must provide verifiable evidence that the bug has been reproduced.
2. **TDD Gate**: The packet must include a failing (red) diagnostic test before altering production code.
3. **Root Cause Analysis**: The fix must be explicitly linked to the root cause identified during reproduction.

## Risk Profile

**Governance Zone**: Green (self-contained, no systemic mutation).
A debugging packet modifies only the narrowly scoped fix target and its
associated test harness. No policy, runtime rule, or cross-mode contract is
affected.

## Why Existing Modes Are Not Enough
- `change` assumes the boundary and preserved invariants are already understood.
- `implementation` assumes the task map is already approved and the work is no
  longer exploratory.
- `incident` optimizes for containment and follow-up, not for disciplined
  reproduction and source-level fault isolation.

## Dependencies

- **02 - Completion Verification Gates**: once landed, the debugging mode
  inherits the fresh-proof requirement automatically rather than implementing its
  own verification step.
- No hard prerequisite; this feature can start immediately.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `incident` | Adjacent but non-overlapping: `incident` is reactive crisis management; `debugging` is deliberate fault isolation. |
| `change` | Downstream: a proven fix may be packeted as a `change` for review. |
| `verification` | Consumed: the red/green harness is a specialized verification surface. |
| `implementation` | Boundary: if the fix grows beyond one scope, escalate to `implementation`. |

## Entry Gates
- A concrete symptom must exist: failing test, failing command, broken user
  path, or stack trace with enough detail to reproduce or to explain why
  reproduction is currently blocked.
- The packet must name the affected surface and the currently suspected blast
  radius before any fix is proposed.
- If the symptom cannot yet be reproduced consistently, the run remains in
  evidence-gathering state and cannot advance to a production-code fix.

## Operational Mechanics
- **Inputs**: Stack traces, user bug reports, failing CI logs (`bug-report.md`).
- **Workflow Steps**:
  1. **Hypothesis Generation**: The system evaluates the report and proposes 2-3 isolated failure hypotheses *before* browsing code extensively.
  2. **Reproduction Harness (Red State)**: The agent generates a minimal failing test (`reproduction_test.rs` or a discrete script) and records the `FAIL` evidence.
  3. **Isolation & Fix (Green State)**: The agent applies the fix exclusively to production code matching the root cause.
  4. **Verification**: The agent records the `PASS` evidence for the reproduction harness alongside confirming no regressions in the broader test suite.
  *(Note: Actual execution of these tests and orchestrator mechanics are deferred to the Boundline runtime; Canon purely validates the presence of the resulting `evidence_ref` and root-cause packet.)*
- **Required Artifacts**: `debugging-trace.md` (which documents the hypothesis, the reproduction steps, and the precise fix rationale) to be included in the final change packet.

## Exit Gates
- The packet must preserve one explicit root-cause statement, not merely a list
  of attempted fixes.
- The red-state reproduction must be captured before the green-state fix is
  accepted.
- The claimed fix must pass the reproduction harness plus one nearby regression
  or confidence check scoped to the affected surface.

## Packet Shape
- `01-context.md`: symptom, affected surface, suspected blast radius.
- `02-reproduction.md`: exact reproduction steps, failing commands, observed
  evidence.
- `03-root-cause.md`: traced source of failure and rejected hypotheses.
- `04-fix-decision.md`: bounded fix, tradeoffs, and why adjacent changes were
  not taken.
- `05-verification.md`: red/green proof plus any remaining uncertainty.

## Success Criteria

- Bug-fix packets routed through this mode achieve first-time resolution (no
  reopen within the same cycle) at a measurably higher rate than generic
  `change` packets.
- Every closed debugging packet contains a traceable root-cause statement linked
  to the reproduction harness.
- No debugging packet closes without a passing red-to-green transition captured
  in evidence.