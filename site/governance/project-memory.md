# Project Memory

Project memory is durable, governed knowledge that should remain visible inside a repository after an AI session ends.

It is not a transcript, scratchpad, or dumping ground. It is the curated memory surface for knowledge that downstream work may rely on.

## What Belongs In Project Memory

Good candidates:

- accepted domain language
- deprecated terms and replacement guidance
- domain concepts, relationships, and invariants
- approved architecture decisions
- Type 1 and Type 2 decision rationale
- security findings and residual risk
- migration rationale and compatibility constraints
- verification conclusions
- evidence summaries
- approval state and downstream reliance boundaries

The test is simple: should a future engineer, reviewer, or tool know this before making related changes?

## What Does Not Belong

Avoid promoting:

- raw generated drafts
- unresolved brainstorming
- stale packet fragments
- unreviewed claims
- temporary task notes
- duplicated README content
- low-value summaries of source files
- claims without evidence or approval posture

Project memory should reduce confusion, not create another place to search.

## Managed Blocks

Managed blocks are controlled regions of project memory that Canon or a Canon-aware process may update.

Use managed blocks when:

- the content is derived from governed packets
- updates need to preserve surrounding human-authored documentation
- downstream tools need stable anchors
- multiple promotion events may touch the same memory surface

Managed blocks should retain source refs and should not hide manual edits outside the block.

## Proposal Files

Proposal files are useful when knowledge is not ready to become accepted memory.

Use proposal files for:

- candidate domain terms
- draft architecture options
- unresolved migration strategies
- security findings awaiting review
- verification results requiring follow-up

Proposal files should make their pending status obvious and should link to the source packet.

## Pending Knowledge

Pending knowledge is valuable, but it must not masquerade as accepted truth.

Keep pending knowledge separate from accepted memory by naming:

- what is known
- what is inferred
- what remains open
- what evidence is missing
- who needs to approve it

Downstream tools should treat pending memory as advisory context, not as authority.

## Evidence Surfaces

Project memory should point to evidence rather than embed every raw detail.

Good evidence surfaces:

- packet refs
- document refs
- command summaries
- test result refs
- reviewer approval refs
- source examples

The memory page should explain the conclusion. The packet and evidence refs should support audit.

## Support For Downstream Tools

Project memory helps downstream tools by providing stable governed knowledge:

- Boundline can consume accepted standards and constraints.
- Coding agents can use approved architecture and domain rules.
- Review agents can check changes against known invariants.
- Verification agents can challenge claims using prior evidence.

Canon-aware consumers should respect readiness, approval state, and semantic authority metadata. They should not treat all Markdown as equal.
