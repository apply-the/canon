# Architecture And Decisions

Canon architecture packets capture bounded structural decisions with rationale, alternatives, consequences, and downstream traceability.

## Architecture Packets

An architecture packet should answer:

- what decision or structural question is being governed?
- what system context applies?
- what constraints matter?
- which alternatives were considered?
- why was the selected option chosen?
- what consequences follow?
- what evidence supports the decision?
- what remains open?

The packet should be useful to implementers and reviewers, not just architects.

## Decision Records

A decision record preserves the reasoning behind a choice.

Useful decision content:

- decision title
- status
- context
- selected option
- rejected alternatives
- rationale
- consequences
- evidence
- approval state
- downstream impact

Decision records should be traceable to the Canon run and packet that produced them.

## Type 1 And Type 2 Decisions

Use decision reversibility to shape governance:

- **Type 1 decisions** are hard to reverse, high consequence, or expensive to unwind.
- **Type 2 decisions** are easier to reverse and can often be made with lighter governance.

Canon should make decision type explicit when it affects approval, evidence, or downstream reliance.

## Rationale

Rationale is the reason a decision was made. It should not be a restatement of the decision.

Good rationale connects:

- goals
- constraints
- tradeoffs
- evidence
- rejected alternatives
- risk posture

Weak rationale says only that an option is simpler, cleaner, or preferred without showing why.

## Alternatives

Alternatives keep architecture honest.

Record:

- what was considered
- why it was rejected
- what would make it viable later
- whether it has different risk or migration properties

If no alternatives are credible, explain why. Do not omit alternatives just to make the decision look obvious.

## Consequences

Consequences are the cost of the decision.

Include:

- implementation impact
- operational impact
- compatibility impact
- migration or rollback constraints
- new risks
- review or verification needs
- downstream packet dependencies

Consequences are often the most useful part of an architecture packet for future maintainers.

## Downstream Traceability

Architecture decisions should remain connected to:

- requirements or discovery packets
- domain-language or domain-model packets
- evidence refs
- implementation or backlog packets
- verification results
- project memory promotions

Downstream tools should consume the decision with its authority and approval state, not as detached prose.
