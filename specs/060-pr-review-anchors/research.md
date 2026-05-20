# Research: pr-review Optional Inline Anchors

## Decision 1: Use the persisted zero-context diff as the only inline-anchor evidence source

- Decision: Derive candidate inline anchors only from the existing `pr-review`
  diff evidence already captured through `git diff --unified=0`, together with
  the finding's persisted `changed_surfaces`.
- Rationale: The current diff pipeline already produces the narrowest durable
  evidence Canon owns for `pr-review`. Reusing that input keeps the feature
  bounded, avoids host-specific dependencies, and aligns anchor generation with
  the same persisted evidence bundle that already drives review findings.
- Alternatives considered:
  - Recompute anchors from live repository state during rendering: rejected
    because rebases, worktree drift, or renamed files would make the output less
    durable and would break the evidence-first boundary.
  - Query code-host APIs for inline positions: rejected because the feature must
    remain host-agnostic and usable outside GitHub/GitLab-specific contexts.

## Decision 2: Model anchor precision as an optional typed field on each review finding

- Decision: Introduce a typed optional `ReviewAnchor` owned by the `pr-review`
  domain and attach it directly to each `ReviewFinding`.
- Rationale: Anchor precision is a property of an individual finding, not a
  packet-global concern. Keeping it on the finding preserves the existing
  deterministic packet model, allows serde-backed stable shapes, and keeps the
  renderer free from heuristic recomputation.
- Alternatives considered:
  - Store anchors only in rendered Markdown: rejected because that would bury a
    stable semantics decision inside the renderer and prevent consistent packet
    behavior across future consumers.
  - Add a packet-global anchor map keyed by title or category: rejected because
    it weakens traceability and risks drift between the finding and its anchor.

## Decision 3: Accept only one-surface, contiguous intervals; degrade otherwise

- Decision: Emit an inline anchor only when a finding resolves to one concrete
  changed surface and one contiguous interval that can be expressed as a single
  line or a single span.
- Rationale: This preserves the repo's existing honest-degradation rule. Any
  evidence that spans multiple files, multiple disjoint hunks, or stale diff
  coordinates is too ambiguous to present as a precise inline anchor.
- Alternatives considered:
  - Collapse multiple disjoint hunks into one synthetic span: rejected because
    it fabricates precision and can visually imply unchanged lines are part of
    the finding.
  - Emit multiple anchors for one finding in the first slice: rejected because
    it expands both the data model and the reader contract beyond the bounded
    follow-on identified in the feature spec.

## Decision 4: Keep the contract host-agnostic and anchored to explicit scope

- Decision: The rendered Conventional Comment contract will keep explicit scope
  mandatory and add an optional human-readable anchor formatted as a repo-
  relative surface plus one line or inclusive line range.
- Rationale: Readers must understand the artifact without platform-specific UI
  conventions. Explicit scope remains the durable baseline even when inline
  precision is unavailable.
- Alternatives considered:
  - Replace scope with inline anchors when anchors exist: rejected because scope
    and anchor answer different questions; one describes reach, the other
    describes location.
  - Use host-specific syntax for inline comments: rejected because the feature
    must remain portable in published packets.

## Decision 5: Keep runtime blast radius narrow in the first implementation slice

- Decision: The first implementation slice should focus on `shell.rs` as the
  input contract boundary, `review/findings.rs` as the typed domain owner,
  `artifacts/markdown/governance.rs` as the output surface, and targeted tests
  plus skill guidance mirrors.
- Rationale: These are the smallest surfaces that can deliver the feature while
  preserving the existing readiness and artifact-order contracts.
- Alternatives considered:
  - Expand packet metadata or CLI output projections in the same slice: rejected
    because no current requirement demands a new machine-facing projection, and
    widening the contract would increase systemic surface area without unlocking
    the core reviewer-facing value.

## Resolved Clarifications

- No `NEEDS CLARIFICATION` markers remain for planning.
- The feature will preserve the existing explicit scope contract and add only
  optional line/span precision where persisted evidence is sufficient.