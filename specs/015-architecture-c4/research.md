# Research: Stronger Architecture Outputs (C4 Model)

## Decision: textual C4 over diagram syntax

**Decision**: emit C4 artifacts as plain Markdown with prose, lists, and tables. Do not require, generate, or render diagrams.

**Rationale**: keeps Canon's runtime dependency surface unchanged, keeps the artifact set diff-friendly and reviewable in plain text, and aligns with the existing critique-first artifacts in the architecture mode. Authored briefs MAY include Mermaid or PlantUML fenced code blocks; the renderer MUST preserve them verbatim because the authored-body extraction is text-preserving.

**Alternatives considered**:

- Generate Mermaid or PlantUML diagrams from authored input. Rejected: requires a new render pipeline, blurs the authored-body contract, and risks fabricated diagrams that imply structure not present in the brief.
- Emit a single combined `c4-views.md` artifact. Rejected: harder to inspect per view, breaks symmetry with the existing artifact-per-concern shape.

## Decision: emit alongside existing critique artifacts, not replace

**Decision**: the new C4 artifacts are added to the architecture artifact set. The existing five artifacts continue to be emitted with the same shape and gate associations.

**Rationale**: the architecture mode is critique-first by invariant. C4 is a communication shape, not a critique, so it must augment rather than replace decision/invariants/tradeoff/boundary/readiness artifacts.

**Alternatives considered**:

- Replace `boundary-map.md` with `container-view.md`. Rejected: boundary semantics and crossing rules are not the same as containers; replacing would silently lose information.
- Make C4 artifacts optional based on a brief flag. Rejected: optional artifacts complicate the inspect/publish contract and weaken the predictable artifact shape.

## Decision: explicit `## Missing Authored Body` marker on absence

**Decision**: when an authored C4 H2 section is absent or empty, the renderer emits the artifact with a `## Missing Authored Body` H2 section that names the expected source heading and tells the reader the AI did not author this section.

**Rationale**: matches the truthfulness pattern already used in `canon-backlog` and the operational packets. Skipping the artifact would change the artifact set shape; emitting fabricated content would be worse than nothing. An explicit marker keeps the artifact set predictable and makes the gap visible.

**Alternatives considered**:

- Skip emission when content is missing. Rejected: changes the artifact set shape based on input, complicates inspect/publish.
- Auto-fill from the run's `context_summary`. Rejected: violates the authored-body contract and produces generic text the reviewer cannot trust.

## Decision: canonical authored headings

**Decision**: extract authored content using the exact H2 headings `## System Context`, `## Containers`, and `## Components`. Heading variants such as `## C4 - System Context` or `## Container View` are not extracted.

**Rationale**: matches the existing `extract_marker` pattern used by other architecture and brief renderers. A strict heading set keeps the contract self-describing through the skill and template, and avoids fragile fuzzy matching.

**Alternatives considered**:

- Match a small set of synonyms. Rejected: increases ambiguity and complicates skill guidance.
- Match anything containing the keyword. Rejected: would falsely consume nearby unrelated sections.

## Decision: gate associations for new artifacts

**Decision**: associate the three new C4 artifacts with `GateKind::Architecture` plus, for `system-context.md`, `GateKind::Exploration` (boundary clarity), and for `component-view.md`, `GateKind::ReleaseReadiness` (the most granular view should be ready before downstream consumption). `container-view.md` uses `GateKind::Architecture` only.

**Rationale**: keeps the gate profile of the architecture mode itself unchanged but adds the new artifacts to existing gate evaluation surfaces consistent with current architecture artifacts.

**Alternatives considered**:

- Add a new `C4Coverage` gate. Rejected: out of scope for this slice; gating expansion is deferred.
- Leave the new artifacts unassociated with any gate. Rejected: would silently weaken the gate signal.
