# 02 - Completion Verification Gates

## Problem
Canon has a dedicated `verification` mode, but it does not yet impose a
cross-mode runtime rule that blocks success claims, task completion, approval
requests, or readiness transitions when the claimed outcome has not been proven
by fresh executable evidence.

This leaves a semantic gap between packet quality and packet finality:
`change`, `implementation`, `refactor`, `review`, and future orchestration
flows can accumulate good guidance and artifacts while still relying on stale,
partial, or inferred verification when reporting completion.

## Proposal
Introduce a cross-mode completion-verification contract enforced across
delivery flows, not a separate packet mode.

The governed workflow should require claim-matched, freshly executed
verification before Canon can:
- transition a packet to ready-for-review or ready-for-approval
- emit success language in summaries
- publish completion evidence as satisfied

*(Note: The actual execution of fresh commands and blocking of task completion
is deferred to the Boundline runtime via a shared bilateral contract `claim ->
proof -> evidence_ref`.)*

This feature should complement the existing `verification` mode rather than
replace it. `verification` remains the packet for challenging claims and
evidence deliberately; completion-verification gates become the always-on rule
that prevents unproven completion across ordinary delivery flows.

## Risk Profile

**Governance Zone**: Amber (cross-mode systemic rule change).
This feature mutates the runtime invariant surface for every existing mode. A
miscalibrated gate could block legitimate completions or create false confidence
in weak proof. Requires staged rollout with an opt-in period before hard
enforcement.

## Why Existing Modes Are Not Enough
- `verification` is a deliberate packet for challenging claims; it is not yet a
   runtime invariant applied automatically when any other mode says work is
   complete.
- `implementation`, `change`, `refactor`, and `review` can still produce strong
   packets with weak closeout language if fresh proof is not enforced.

## Dependencies

- **None upstream**: this feature has no prerequisites within the roadmap.
- **Downstream consumers**: 03 (Plan Progress and Handoff Schemas) depends on
   this feature so that progress packets can reference stable `evidence_ref`
   structures rather than vague completion claims.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `verification` | Complementary: `verification` is a packet for deliberate challenge; this feature is the always-on rule. |
| `implementation` | Constrained: implementation tasks cannot close without proving their claim. |
| `change` | Constrained: change packets cannot emit success without fresh evidence. |
| `refactor` | Constrained: refactor completion requires behavioral-equivalence proof. |
| `review` | Constrained: review readiness transitions require demonstrated correctness. |

## Entry Gates
- Any completion claim, readiness transition, approval request, or human-facing
   success summary triggers this feature automatically.
- Each claim must map to one or more proving commands before Canon can present
   the claim as satisfied.

## Operational Mechanics
- **Inputs**: current run context, claimed outcome, declared validation
  intent, returned `evidence_ref` metadata, and packet readiness state.
- **Workflow Steps**:
  1. **Claim Parsing**: The runtime derives the specific claim being made, such
     as "tests pass", "bug fixed", "contract aligned", "build clean", or
     "migration valid".
  2. **Proof Contract**: Canon establishes the `claim -> proof -> evidence_ref`
     contract structure.
  3. **Execution Deferral**: The actual selection and fresh execution of the
     falsifying command is owned by Boundline. Boundline blocks its task
     completion until it secures the proof.
  4. **Result Evaluation**: Canon evaluates the `evidence_ref` metadata
     returned by Boundline.
  5. **State Transition**: Only passing evidence allows readiness promotion,
     approval prompts, or publishable success summaries within Canon.
  6. **Failure Handling**: If verification metadata is missing or fails, Canon keeps
     the packet open and explicitly names the blocked claim.
- **Required Artifacts**: `completion-evidence.md` (ordered record of each
  claim, proving command, timestamp, and result) and `verification-blockers.md`
  when a run cannot close (findings-first list of unproven claims, failed
  commands, and next narrow step). These are linkable from implementation,
  change, refactor, and review packets.

## Exit Gates
- A broader claim cannot be discharged by a narrower command. For example,
   "build is clean" cannot be satisfied by a passing lint run.
- Fresh command output must exist in the current working state; historical green
   output is advisory only.
- If proof and claim do not match, Canon must downgrade the state to blocked
   rather than smoothing the mismatch into optimistic prose.

## Packet Shape
- `01-claims.md`: the claims Canon is about to make.
- `02-proof-map.md`: which command proves each claim and why that command is the
   right falsifier.
- `03-completion-evidence.md`: fresh results with exit codes and summary lines.
- `04-blockers.md`: unresolved claims, failed checks, and next proof step.

## Success Criteria

- Zero packets across any mode close with success language when the proving
  command has not been freshly executed in the current working state.
- The rate of "reopened because it was never actually verified" drops to zero.
- Downstream orchestration (03) can rely on task-complete meaning "proven
  complete" without adding its own ad-hoc verification layer.

## Adoption Notes

This feature should land as a runtime rule before Canon adds richer plan
orchestration. Otherwise, future multi-step execution flows will scale the
current ambiguity instead of removing it.

It should also integrate with any future task runner so that a task cannot be
marked complete simply because code changed or an agent reported success.
