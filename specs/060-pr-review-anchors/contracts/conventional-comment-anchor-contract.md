# Conventional Comment Anchor Contract

## Purpose

Define the bounded, host-agnostic contract for optional inline anchors in
`pr-review` Conventional Comments. This contract extends the existing explicit
scope model but does not replace it.

## Scope

- Applies only to `pr-review` Conventional Comments.
- Applies only when Canon already has persisted diff evidence for the finding.
- Covers the typed anchor shape, eligibility rules, degradation rules, and
  rendered artifact expectations.

## Contract Entities

### ReviewAnchor

| Field | Cardinality | Meaning |
|-------|-------------|---------|
| `surface` | Required | One repo-relative changed surface owned by the finding. |
| `line_start` | Required | Inclusive 1-based start line. |
| `line_end` | Optional | Inclusive 1-based end line for a contiguous span. |

Contract rules:

- `surface` must be present in the finding's persisted `changed_surfaces`.
- Omitted `line_end` means the anchor is a single-line anchor.
- Present `line_end` must be greater than or equal to `line_start`.
- Multi-surface or disjoint intervals are out of contract for this slice.

### Conventional Comment Entry

Every rendered Conventional Comment entry must continue to include:

- comment kind
- explicit `scope`
- title
- rationale (`Why`)
- changed surfaces

An entry may additionally include one rendered anchor when `ReviewAnchor` is
present and valid.

## Eligibility Rules

An inline anchor is eligible only when all of the following are true:

1. The finding resolves to one concrete changed surface.
2. The persisted diff evidence resolves to one contiguous interval.
3. The interval can be expressed as one line or one inclusive line range.
4. The derived coordinates do not require host-specific placement semantics.

## Degradation Rules

The renderer must omit the anchor and fall back to explicit scope-only output
when any of the following applies:

- no durable coordinate evidence exists
- the finding spans multiple changed surfaces
- the candidate position is stale or cannot be validated against persisted diff evidence
- the candidate location would require multiple disjoint ranges
- the only available precision is host-specific rather than artifact-stable

## Rendered Artifact Contract

The rendered anchor must be human-readable and host-agnostic.

Canonical textual forms:

- line anchor: `<surface>:<line_start>`
- span anchor: `<surface>:<line_start>-<line_end>`

Rendering expectations:

- explicit scope remains mandatory even when an anchor is present
- anchor text augments location precision but does not redefine the comment's reach
- rendered output must remain legible without code-host tooling or deep links

Reference examples:

- anchored line comment: `praise(scope:surface): No material duplication concerns inferred` plus `Anchor: tests/reviewer.md:3`
- anchored span comment: `praise(scope:surface): No material duplication concerns inferred` plus `Anchor: tests/reviewer.md:3-4`
- degraded scope-only comment: `praise(scope:file): No material duplication concerns inferred` with no `Anchor:` line when evidence spans `src/reviewer.rs` and `tests/reviewer.md`

## Compatibility Guarantees

- Existing comments without inline evidence remain valid and must continue to
  render without anchors.
- Existing explicit `pr`/`file`/`surface` scope semantics remain authoritative.
- `review-summary.md` remains the primary artifact and is unaffected by anchor presence.

## Deferred Work

- multiple anchors per finding
- host-specific thread or deep-link projections
- retroactive anchor derivation for historical packets without persisted evidence