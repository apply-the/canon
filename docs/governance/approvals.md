# Approvals

In traditional CI/CD systems, approvals happen at the Pull Request level—often too late to correct underlying architectural missteps made by an AI agent. Canon shifts this mechanism left by enforcing **Runtime Approvals** directly within the agentic workflow.

## The Approval Gate

Whenever an agent attempts a transition that exceeds its configured risk threshold (as defined by the Canon Mode or the active Governance Packet), execution halts immediately.

The system transitions into a `blocked` state. To proceed, a human operator must explicitly grant an approval.

## `canon approve`

The primary interface for this is the `canon approve` command.

When invoked, the operator is presented with the irrefutable evidence of *why* the agent was blocked:
- The planned file mutations.
- The context used to generate the decision.
- The specific governance rule that triggered the block.

The operator must provide a justification to unblock the sequence. This justification isn't just a fleeting terminal input—it is captured, serialized, and appended to the current session trace.

## Accountability & Evidence

Because an approval is permanently linked to the evidence packet that requested it, Canon establishes complete accountability. If a destructive mutation is deployed, you can trace it back not just to the AI model that proposed it, but to the exact operator approval that authorized its execution.