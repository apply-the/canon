# Getting Started

For a 5-minute technical setup with commands, use the [Quickstart](./quickstart).

This path is for a user who wants to understand Canon by running one governed packet from authored input to inspection and publication.

## What Canon Is

Canon is the governed packet runtime for AI-assisted engineering work. You use it inside a repository to create explicit, traceable packets for work such as requirements, architecture, change planning, verification, security assessment, and migration.

The Canon CLI owns the governance surface:

- run identity
- mode, owner, risk, and zone
- authored input snapshots
- evidence and invocation records
- readiness and approval state
- publication into visible repository paths

The AI assistant supplies the reasoning content. Canon records the governed shape, boundaries, and traceability.

## The Lifecycle

The process of creating and publishing governed work always follows this sequence:

1. **Authored Input:** Write a brief under `canon-input/`.
2. **Inspect Clarity:** Verify the inputs before running.
3. **Run:** Start a governed session.
4. **Inspect:** Review status, evidence, and artifacts.
5. **Approve:** Review and unblock the agent when human judgment is needed.
6. **Publish:** Promote the final artifacts into your repository's permanent memory.

## Choose A Mode

Pick the mode by the kind of knowledge you need, not by the file you happen to have open.

- Use `discovery` when the problem space is still ambiguous.
- Use `requirements` when scope, outcomes, and acceptance boundaries need to be stabilized.
- Use `domain-language` when terms are inconsistent across people, docs, APIs, or code.
- Use `domain-model` when concepts, relationships, and invariants need structure.
- Use `architecture` when the remaining work is a bounded structural decision.
- Use `change` when the structure is known and the task is a bounded modification.
- Use `verification` when a claim, packet, or quality signal needs direct challenge.

For the complete mode guide, see [Canon Modes](./canon-modes).

## Inspect The Produced Packet

After a run, do not treat the first artifact as accepted knowledge. Inspect:

- **status**: whether the run is complete, blocked, or awaiting approval
- **invocations**: what Canon attempted and recorded
- **evidence**: what supports the packet
- **artifacts**: which documents were emitted
- **readiness**: whether the packet is reusable or still incomplete
- **approval state**: whether a human gate still applies

This inspection habit is the practical difference between generated prose and governed engineering knowledge.

## Understand Readiness, Evidence, And Approval

Readiness says whether the packet can be reused. Evidence says why its claims should be trusted. Approval says whether the organization has accepted the packet for downstream reliance.

These are separate signals. A packet can be well written but weakly evidenced. It can have evidence but still require approval. It can be approved for one downstream use without becoming a universal standard.

Use your repository's constitution and [Core Concepts](./core-concepts) before publishing or promoting anything into project memory.

## What To Read Next

- [Installation](./installation) for CLI installation details
- [First Workspace](./first-workspace) to understand repository resolution and setup
- [Core Concepts](./core-concepts) for the underlying mechanics
- [Common Workflows](./common-workflows) for concrete scenarios
