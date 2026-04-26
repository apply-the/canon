# Decision Log: Domain Modeling And Boundary Design

- **D-001**: Keep the first slice bounded to `system-shaping`, `architecture`, and `change`.
  - **Rationale**: Highest leverage for domain-boundary quality while keeping risk bounded.
  - **Alternatives considered**: extend all remaining modes; focus on architecture only.

- **D-002**: Keep domain modeling inside existing modes rather than creating a new Canon mode.
  - **Rationale**: Preserves the current mode-driven workflow and avoids widening governance semantics.
  - **Alternatives considered**: introduce a standalone domain-modeling mode.

- **D-003**: Add one dedicated `domain-model.md` artifact to `system-shaping` and one dedicated `context-map.md` artifact to `architecture`.
  - **Rationale**: Makes the new material first-class without over-fragmenting the packet.
  - **Alternatives considered**: only extend existing artifacts; add multiple new artifacts per mode.

- **D-004**: Strengthen existing `change` artifacts instead of adding a second change packet family.
  - **Rationale**: The current packet already exposes the bounded surface; domain modeling should sharpen it, not duplicate it.
  - **Alternatives considered**: add a standalone `domain-slice.md` artifact.

- **D-005**: Surface uncertain or weak boundaries explicitly rather than forcing crisp but unsupported domain partitions.
  - **Rationale**: Critique-first honesty is more important than apparent completeness.
  - **Alternatives considered**: always synthesize a single best boundary map.

- **D-006**: Require `domain-model.md` at `Exploration` and `Architecture` gates for `system-shaping`, and require `context-map.md` at `Architecture` and `Risk` gates for `architecture`.
  - **Rationale**: The new artifacts carry the boundary, ownership, and invariant evidence that those gates already depend on; making the gate mapping explicit avoids implementation drift.
  - **Alternatives considered**: leaving gate ownership implicit in code only; delaying gate binding until after renderer work lands.

- **D-007**: Render `system-shaping/domain-model.md` from authored H2 sections with an `Intent`/`Constraint` summary instead of synthesizing a second freeform analysis artifact.
  - **Rationale**: This keeps the new artifact critique-first, traceable to authored input, and compatible with the existing missing-body honesty contract.
  - **Alternatives considered**: dumping the entire brief into the new artifact; generating an unconstrained second narrative from Copilot output.

- **D-008**: Render `architecture/context-map.md` from authored H2 sections with a `Decision focus`/`Constraint` summary while keeping the existing C4 artifact family additive.
  - **Rationale**: This keeps context boundaries traceable to authored input, aligns the new context map with the existing architecture packet, and avoids fabricating a second unconstrained narrative.
  - **Alternatives considered**: collapsing domain boundary detail into `boundary-map.md`; treating the context map as a generated-only artifact disconnected from authored input.

- **D-009**: Strengthen `change` through `Domain Slice`, `Domain Invariants`, `Cross-Context Risks`, and `Boundary Tradeoffs` inside the existing packet and extend summary surfaces to reflect those additions.
  - **Rationale**: The bounded-change packet stays the single review surface while making domain impact and cross-context pressure explicit in both artifacts and run summaries.
  - **Alternatives considered**: adding a standalone `domain-slice.md` artifact; leaving mode summaries unchanged and forcing reviewers to infer the new boundary evidence from raw markdown files.