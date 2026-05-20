# Next Features

## Recently Landed

Canon 0.60.0 shipped optional inline anchors for `pr-review` Conventional
Comments when persisted diff evidence resolves to one changed surface and one
contiguous changed interval. The rendered comment still keeps explicit derived
scope in every case and degrades cleanly to scope-only output when anchor
precision would be cross-surface, stale, ambiguous, or otherwise unsupported.

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
