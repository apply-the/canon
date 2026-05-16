# Next Features

The most recent delivered roadmap entry is
`050-project-memory-control`.

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

A tech-debt refactor should split the remaining large source files to match
the module decomposition already applied to `service.rs`, `summarizers.rs`,
and `markdown.rs`. Files above 1000 lines that still need decomposition, in
descending order:

| File | Lines |
|---|---|
| `orchestrator/gatekeeper.rs` | 2692 |
| `orchestrator/publish.rs` | 2455 |
| `service/clarity.rs` | 2235 |
| `commands/governance.rs` | 2039 |
| `persistence/store.rs` | 1584 |

Each split should follow the same Rust 2024 named-file-plus-sibling-directory
pattern used in the completed passes: no `mod.rs`, child modules use
`use super::*`, visibility promoted only where cross-sibling calls require it.
No behavior or API surface changes are in scope.

This roadmap remains intentionally sparse: a macrofeature only moves forward
once its bounds, artifact contract, and validation story are explicit.
