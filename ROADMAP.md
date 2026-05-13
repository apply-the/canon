# Next Features

The most recent delivered roadmap entry is
`049-logical-packet-ordering`.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `system-assessment`, `migration`,
`supply-chain-analysis`, `domain-language`, and `domain-model`.

Delivered history through `049` is already tracked in `CHANGELOG.md` and the
feature directories under `specs/`. That delivered baseline now includes the
current governed mode catalog, publishable packet families, operator run and
status control, authored-input readiness guidance, architecture visual packets,
ADR publishing, project-memory promotion, and the ordered packet contract.

## Open Ideas

A future nice-to-have could add an optional interactive `canon init`
onboarding flow when Canon needs to configure explicit external integration
surfaces rather than only materialize local runtime state. If Canon later
needs to capture operator choices such as default AI target, external publish
or backlog handoff destinations, or credential references for MCP-backed
services such as Atlassian-family systems, an interactive configurator could
guide that setup while preserving the existing non-interactive CLI and
governance adapter contracts for automation. Any such onboarding should keep
secrets out of versioned repository state and prefer environment or
host-managed credential references over raw token persistence.

This roadmap remains intentionally sparse: a macrofeature only moves forward
once its bounds, artifact contract, and validation story are explicit.
