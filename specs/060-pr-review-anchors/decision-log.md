# Decision Log: pr-review Optional Inline Anchors

## D-001: Keep explicit scope as the primary review-reach contract

- Date: 2026-05-19
- Decision: Inline anchors will augment, not replace, the existing explicit
  `pr`/`file`/`surface` scope model.
- Context: `053-pr-review-scope` already delivered the explicit reach contract.
  The follow-on feature only adds optional inline precision.
- Alternatives considered:
  - Replace scope with anchors when precision exists.
  - Hide scope whenever an anchor is present.
- Rationale: Scope and location answer different reviewer questions. Scope stays
  meaningful even when no durable inline precision exists.
- Consequences: Every rendered comment must continue to show explicit scope.

## D-002: Treat the persisted diff bundle as the only anchor authority

- Date: 2026-05-19
- Decision: Anchor derivation will rely only on persisted changed surfaces and
  the stored zero-context diff patch collected during `pr-review`.
- Context: The feature requires durable, evidence-backed inline precision.
- Alternatives considered:
  - Recompute line positions from live repository state.
  - Use host APIs to infer inline locations.
- Rationale: Persisted diff evidence is the narrowest durable input Canon owns
  without widening the trust boundary.
- Consequences: Rebased or stale coordinates must degrade to scope-only output.

## D-003: Limit the first slice to one anchor per finding

- Date: 2026-05-19
- Decision: A finding may carry zero or one inline anchor; multiple anchors per
  finding are deferred.
- Context: Some findings may touch more than one changed region or more than one
  changed surface.
- Alternatives considered:
  - Support multiple anchors immediately.
  - Emit a synthetic merged span for disjoint regions.
- Rationale: A single optional anchor keeps the contract small and avoids
  inventing broader precision than the evidence supports.
- Consequences: Disjoint or cross-surface evidence degrades to scope-only comments.

## D-004: Keep the first design slice inside review-domain and artifact surfaces

- Date: 2026-05-19
- Decision: The first implementation slice will focus on the diff input
  contract boundary, `ReviewFinding` domain ownership, Conventional Comments
  rendering, targeted tests, and reviewer guidance mirrors.
- Context: The feature aims to improve reviewer-facing packet usefulness with a
  bounded blast radius.
- Alternatives considered:
  - Expand machine-facing CLI or packet-metadata projection surfaces now.
  - Combine this slice with host-specific export work.
- Rationale: The reviewer-visible artifact is the primary value surface, and a
  narrower slice preserves bounded-impact execution.
- Consequences: If downstream machine-facing consumers later need anchor
  projections, that work should be specified as a separate follow-on.

## D-005: Derive anchors only from stored added-side hunk intervals

- Date: 2026-05-19
- Decision: The implementation derives `ReviewAnchor` coordinates only from
  added-side `@@ ... +start[,count] @@` intervals in the persisted zero-context
  patch.
- Context: The stored patch already exists in the `pr-review` evidence bundle,
  but it must not be stretched into host-specific or reconstructed positions.
- Alternatives considered:
  - Infer anchors from removed-side ranges or mixed old/new offsets.
  - Re-open the repository to recalculate coordinates from live files.
- Rationale: Added-side intervals are the only durable, packet-owned source for
  reviewer-visible line precision in this slice.
- Consequences: Deletion-only hunks, mismatched surfaces, and stale packet
  evidence degrade to scope-only output.

## D-006: Render anchors as explicit host-agnostic text lines

- Date: 2026-05-19
- Decision: Conventional Comments render anchors on a dedicated `Anchor:` line
  using `surface:start` or `surface:start-end` text while leaving the comment
  header and `scope` annotation unchanged.
- Context: Published packets must stay readable outside a code host.
- Alternatives considered:
  - Embed host-specific deep links.
  - Replace the scope label with coordinate-only output.
- Rationale: A dedicated text line keeps the artifact readable in markdown,
  terminal, and published-doc contexts without weakening the existing scope
  contract.
- Consequences: Reviewers always see explicit scope first and can treat the
  anchor as optional precision rather than as the whole reach model.

## D-007: Reuse the existing diff-capture path without shell or orchestrator changes

- Date: 2026-05-19
- Decision: The anchor slice reuses the existing `git diff --unified=0` capture
  in `shell.rs` and existing `ReviewPacket::from_evidence` plumbing in
  `mode_pr_review.rs` without new adapter-side or orchestration-side behavior.
- Context: The persisted patch and changed-surface inputs required by the feature
  already reach the review domain.
- Alternatives considered:
  - Add a second adapter payload for pre-parsed coordinates.
  - Compute anchors in the shell adapter instead of the review domain.
- Rationale: Keeping anchor derivation in the review domain preserves a single
  owner for finding scope, optional precision, and degradation rules.
- Consequences: Validation can stay focused on review-domain logic and rendered
  artifacts instead of expanding the execution surface.