# Contract: Conventional Comments Artifact For PR Review

## Summary

The `pr-review` packet gains an additive artifact that renders persisted review
findings in a Conventional Comments shape while preserving the existing review
summary and approval workflow.

## Artifact Name

- Proposed first-slice artifact: `conventional-comments.md`

## Required Sections

- `## Summary`
- `## Evidence Posture`
- `## Conventional Comments`
- `## Traceability`

## Validation Gates

- `ReviewDisposition`
- `ReleaseReadiness`

## Content Expectations

The artifact must:

- begin with a summary that identifies the reviewed refs and evidence posture
- emit reviewer-facing entries using valid Conventional Comments kinds only
- preserve changed-surface traceability for each entry
- remain readable as standalone markdown outside the CLI
- stay host-agnostic and avoid fabricated line-level anchors

## Entry Shape

Each entry must contain:

- comment kind
- short reviewer-facing title
- rationale/details
- affected changed surfaces
- the fact that it was derived from governed `pr-review` evidence

In the first slice, one persisted finding yields one comment entry.

## Compatibility Expectations

- `review-summary.md` remains present and remains the primary artifact for
  status and next-step surfaces in the first slice.
- The new artifact publishes alongside the existing packet under
  `docs/reviews/prs/<RUN_ID>/`.
- Existing approval semantics for unresolved must-fix findings remain
  unchanged.
- The artifact does not become the primary status summary in the first slice.

## Non-Goals For This Contract

- host-specific export payloads
- inline code-host anchors
- replacing `review-summary.md`
