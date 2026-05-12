# Next Features

The most recent delivered roadmap entry is
`047-domain-language-model-modes`.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `system-assessment`, `migration`,
`supply-chain-analysis`, `domain-language`, and `domain-model`.

Delivered work through `037` already covers the governed mode surface,
publishable packet families, clarity and readiness honesty, the governance
adapter, structured publish destinations, and release provenance across
distribution channels.

Delivered work through `038` already covers the operator-facing run and status
control story, ordered next-step guidance, and coherent review-first ergonomics
across completed, blocked, approval-gated, and resumed runs.

Delivered work through `039` now also covers the shared authored-input
lifecycle, packet-shape and authority guidance for `inspect clarity`, and
coherent shared docs or skill guidance for moving from weak authored packets to
publishable packet outputs.

Delivered work through `040` now also frames Canon as the governed packet
runtime for AI-assisted engineering, makes the human-vs-machine boundary
explicit, and adds a dedicated governance adapter integration guide for
external orchestrators.

Delivered work through `041` now also adds an additive published `prd.md` for
requirements packets, exposes publish as a chat-first skill, and makes the
runtime-artifact versus published-doc boundary explicit across the docs.

Delivered work through `042` now also reframes architecture packets around one
primary `architecture-overview.md` handoff, records required System Context,
Container, and Deployment coverage with Mermaid sidecars, and keeps optional
deeper C4 views evidence-driven instead of mandatory boilerplate.

Delivered work through `043` now also projects standard Nygard-style ADRs into
the fixed `docs/adr/` registry: `architecture` publishes them by default,
`change` and `migration` can opt in with `publish --adr`, numbering stays
sequential without rewriting gaps, and unsupported modes remain outside the
registry boundary.

Canon now emits governed Mermaid sources for the core architecture views it can
justify from the authored packet and records unsupported SVG or PNG targets
explicitly instead of pretending they exist.

Future roadmap entries can build on that base rather than reopening the
question of whether architecture packets should have a single primary review
entrypoint.

Delivered work through `046` now prefixes every artifact filename with a
two-digit ordinal (e.g., `01-problem-statement.md`, `02-constraints.md`) so
that published packets display in a deterministic, Confluence-tree-style
reading order. Gates, renderers, and summarizers match on the unprefixed slug;
published file paths carry the ordinal prefix.

Delivered work through `047` now adds two new first-class governed modes:
`domain-language` for stabilizing shared vocabulary (10 artifacts including
glossary, preferred language, conflicts, contextual meanings, and code/API
vocabulary), and `domain-model` for formalizing lightweight ontology concept
models (13 artifacts including concept catalog, relationship map, bounded
context map, lifecycle/state model, domain invariants, feature-impact rules,
and a machine-readable `domain-model.json` sidecar). Both modes support
inspect clarity, governance adapter capabilities, canonical input binding,
and publish to `docs/domain/language/` and `docs/domain/model/`.

This roadmap remains intentionally sparse: a macrofeature only moves forward
once its bounds, artifact contract, and validation story are explicit.
