# Next Features

One proposed roadmap entry now follows the delivered
`041-prd-publish-chat` slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `system-assessment`, `migration`, and
`supply-chain-analysis`.

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

## Proposed: `042-visual-artifact-generation`

Canon currently preserves authored markdown diagram syntax such as Mermaid
verbatim inside publishable packets, but it does not yet generate first-class
visual artifacts of its own.

The next macrofeature should explore governed generation of diagrams and image
assets for architecture and adjacent document-heavy modes when the authored
packet is strong enough to justify them.

Initial scope candidates:

- Generate canonical diagram sources for architecture packets, including C4 and
  context-map views, instead of relying only on authored fenced blocks.
- Extend the same visual generation posture to adjacent packet families such as
  backlog outputs when the packet naturally contains dependency, sequencing, or
  capability-map structure.
- Materialize publishable visual assets such as SVG or PNG alongside markdown
  packets so diagrams can travel as durable documentation artifacts.
- Keep governance posture explicit by surfacing assumptions, evidence limits,
  and honest degradation when Canon cannot justify a trustworthy visual.

This roadmap remains intentionally sparse: a macrofeature only moves forward
once its bounds, artifact contract, and validation story are explicit.
