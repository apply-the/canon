# Next Features

## Open Ideas

A future `pr-review` enhancement could make Conventional Comments carry an
explicit review scope instead of relying only on implied changed-surface
traceability. The first step should introduce a durable scope model that can
differentiate PR-level comments from file or surface-scoped comments while
remaining host-agnostic and preserving the current governed packet contract.
A later slice could add optional line or span anchors when Canon has durable,
persisted diff-coordinate evidence, but the system should continue to degrade
honestly to general or surface-scoped comments when inline positions are not
available.

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
