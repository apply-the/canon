# Packets And Ordered Documents

Canon packets are structured document sets. The structure matters because packets are meant to be read, reviewed, approved, published, and consumed later by people and tools.

## Packet Structure

A packet should carry:

- mode and purpose
- authored input snapshot
- ordered documents
- evidence refs
- approval and readiness state
- lineage and provenance
- metadata for downstream consumers

Generated packet artifacts usually begin under Canon-managed runtime paths. Publication moves reusable packets into visible repository documentation paths without losing run identity.

## Why Ordered Prefixes Matter

Names like `01-context.md`, `02-findings.md`, and `03-decisions.md` make the packet readable in a stable order.

The prefix prevents common problems:

- readers start with conclusions before context
- evidence is separated from claims
- generated artifacts accumulate without a usable sequence
- downstream tools must guess which document is primary

Ordered documents are especially important when packets become project memory or when another runtime consumes them.

## Mode-Specific Document Sets

The exact set depends on the mode:

- discovery packets emphasize context, findings, opportunities, and open questions
- requirements packets emphasize scope, acceptance, non-goals, and tradeoffs
- domain-language packets emphasize terms, status, ambiguity, and evidence
- domain-model packets emphasize concepts, relationships, invariants, and feature impact
- architecture packets emphasize decisions, alternatives, consequences, and rationale
- change and implementation packets emphasize target surface, invariants, execution guidance, and verification
- review and verification packets emphasize findings, checks, results, and acceptance posture
- operational packets emphasize impact, risk, containment, mitigation, and evidence

Use [[Canon Modes|Canon-Modes]] for the expected packet shape by mode.

## Relationship To Project Memory

Not every packet document belongs in project memory. Project memory should receive durable knowledge that downstream work may reuse.

Good promotion candidates:

- accepted domain terms
- stable invariants
- approved architecture decisions
- security findings with residual risk
- migration rationale
- verification conclusions

Poor promotion candidates:

- raw brainstorming
- unresolved speculation
- temporary run notes
- generated text with weak evidence
- duplicated source docs

## Avoid Artifact Sprawl

To avoid unordered artifact sprawl:

- keep one authoritative current-mode brief
- use ordered document names
- attach evidence close to the claims it supports
- publish only when readiness supports downstream use
- promote only durable knowledge into project memory
- preserve packet refs when copying or summarizing content

The goal is not more Markdown. The goal is governed knowledge that remains usable after the original AI session is gone.
