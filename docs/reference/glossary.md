# Domain Language

Domain language is the shared vocabulary of a product or system area. Canon treats language as governed knowledge because unstable terms create unstable requirements, models, architecture, and implementation.

## Ubiquitous Language

Ubiquitous language is the vocabulary the team agrees to use consistently across:

- product conversations
- requirements
- UI text
- APIs
- code
- documentation
- reviews
- downstream guidance

Canon does not invent vocabulary from taste. It should ground terms in actual usage and make deliberate decisions about accepted, deprecated, and ambiguous language.

## Term Status

A domain-language packet should make term status explicit.

Useful statuses:

- accepted
- proposed
- ambiguous
- deprecated
- rejected

Status matters because downstream tools should not treat an ambiguous or proposed term as an accepted standard.

## Accepted Terms

Accepted terms should include:

- the term
- definition
- scope
- examples of correct use
- terms it replaces or excludes
- evidence or source refs

An accepted term becomes reusable project memory only when it has enough evidence and approval for its downstream purpose.

## Deprecated Terms

Deprecated terms should include:

- old term
- replacement term
- reason for deprecation
- affected docs, APIs, or code where known
- migration guidance

Deprecation is especially useful for Boundline guidance and review checks because it gives downstream tools something concrete to detect.

## Ambiguity Handling

Ambiguous terms should not be cleaned up silently.

Record:

- conflicting meanings
- where each meaning appears
- who or what uses each meaning
- risk caused by ambiguity
- recommended clarification path

Sometimes the correct output of a domain-language run is not a final vocabulary. It may be a focused set of language questions for discovery, requirements, or domain modeling.

## Relationship With Boundline

Boundline can consume accepted and deprecated domain language as guidance or guardrails during delivery work.

Canon should supply:

- accepted term definitions
- deprecated term mappings
- ambiguity warnings
- evidence refs
- approval state

Boundline should decide how to activate that guidance at runtime. Canon provides governed meaning, not runtime execution.

## When To Use `domain-language`

Use `domain-language` before downstream work when:

- stakeholders use different names for the same thing
- one term means multiple things
- code and product language diverge
- requirements are blocked by unclear vocabulary
- generated agents keep making terminology mistakes
- Boundline guidance needs accepted vocabulary
