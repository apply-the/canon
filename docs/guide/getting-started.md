# Getting Started

For a 5-minute technical setup with commands, use the [[Quick Start|Quick-Start]].

This path is for a user who wants to understand Canon by running one governed packet from authored input to inspection and publication.

This page is maintained against `0.63.0`.

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

## First Run

1. Install the `canon` CLI.
2. Initialize Canon inside the repository you want to govern. *(Note: Canon resolves the workspace automatically, so you can run it from any subdirectory once initialized).*
3. Write authored input under `canon-input/`.
4. Inspect clarity before running.
5. Start a governed run.
6. Inspect status, evidence, and artifacts.
7. Approve or resume if gates require it.
8. Publish the packet when it is reusable.

The canonical command walkthrough lives in the source repo: [Getting Started with Canon](https://github.com/apply-the/canon/blob/main/tech-docs/guides/getting-started.md).

The first `canon init` step now opens a guided assistant selector by default
when the current terminal supports the interactive flow. For scripts, CI, or
machine-readable output, use `canon init --non-interactive`.

## Choose A Mode

Pick the mode by the kind of knowledge you need, not by the file you happen to have open.

- Use `discovery` when the problem space is still ambiguous.
- Use `requirements` when scope, outcomes, and acceptance boundaries need to be stabilized.
- Use `domain-language` when terms are inconsistent across people, docs, APIs, or code.
- Use `domain-model` when concepts, relationships, and invariants need structure.
- Use `architecture` when the remaining work is a bounded structural decision.
- Use `change` when the structure is known and the task is a bounded modification.
- Use `verification` when a claim, packet, or quality signal needs direct challenge.

For the complete mode guide, use [[Canon Modes|Canon-Modes]] and the source guide: [Canon Mode Guide](https://github.com/apply-the/canon/blob/main/tech-docs/guides/modes.md).

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

Use [[Evidence And Approvals|Evidence-And-Approvals]] before publishing or promoting anything into project memory.

## What To Read Next

- [[Installation And Setup|Installation-And-Setup]] for setup details
- [[Canon Modes|Canon-Modes]] for mode selection
- [[Packets And Ordered Documents|Packets-And-Ordered-Documents]] for packet structure
- [[Examples]] for concrete scenarios
- [[Troubleshooting]] if a run is blocked or unclear
