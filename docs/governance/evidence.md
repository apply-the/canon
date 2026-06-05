# Evidence And Approvals

Canon separates generated content from accepted knowledge. Evidence and approvals are the boundary.

## What Counts As Evidence

Evidence is any specific material that supports or weakens a packet claim.

Common evidence:

- authored input briefs
- source files and code references
- command output
- test results
- CI checks
- logs and alerts
- architecture docs
- requirements docs
- reviewer comments
- incident timestamps
- dependency manifests and audit output
- prior governed packet refs

Good evidence is concrete, inspectable, and tied to a claim. Weak evidence is vague, inferred, or impossible to reproduce.

## How Evidence Is Attached

Evidence should remain traceable through the packet lifecycle.

When writing or reviewing a packet, make clear:

- which claim the evidence supports
- where the evidence came from
- whether the evidence is direct or inferred
- whether the evidence is current
- whether the evidence has gaps

Evidence does not need to prove every sentence. It does need to support decisions, findings, approvals, and downstream reliance.

## Approval States

Approval state records whether governed knowledge is accepted for downstream use.

Typical meanings:

- no approval needed for low-risk reusable knowledge
- approval requested when a gate applies
- approval granted when a human accepted the boundary
- approval rejected when downstream use should stop
- approval expired when prior approval no longer applies

Exact machine-facing values are part of the adapter contract. Canonical source: [Governance Adapter Integration](https://github.com/apply-the/canon/blob/0.68.0/tech-docs/integration/governance-adapter.md).

## Readiness States

Readiness says whether a packet can be consumed.

Common readiness posture:

- pending or incomplete: do not rely on it downstream
- reusable: can be used for the bounded purpose stated in the packet
- rejected: should not be consumed as accepted knowledge
- blocked: requires clarification, evidence, or approval

Readiness and approval are related but not identical. A packet may be structurally complete while still lacking approval. A packet may be approved for a narrow purpose while still unsuitable as broad project memory.

## Refinement Continuity And Approval Intent

- Continuity data and `refinement_state` improve traceability, but they do not imply automatic authorization to continue mutation.
- Suggested continuation is an operator hint, not implicit approval.
- Approval and continuation are separate decisions.
- Approval answers whether a gated boundary is accepted.
- Continuation answers whether to proceed in the same run context.
- Treat candidate continuation as advisory and require explicit human intent before resuming.

## What Remains Uncertain

A strong packet names uncertainty directly. It should distinguish:

- confirmed facts
- reasonable inferences
- open questions
- known gaps
- assumptions accepted for this run
- claims that require later verification

Hiding uncertainty makes downstream work brittle. Canon makes uncertainty visible so a later reader can decide whether the packet is safe to reuse.

## Downstream Consumption

Downstream work should consume approved and unapproved knowledge differently.

- Approved reusable packets can guide implementation, planning, or automation within their stated boundary.
- Incomplete packets can inform exploration but should not drive irreversible work.
- Rejected packets should be treated as negative evidence or historical context.
- Approval-gated packets should stop downstream mutation until approval is granted.

When in doubt, preserve the packet ref and ask for verification rather than turning uncertain prose into a standard.
