# Data Model: pr-review Optional Inline Anchors

## Overview

This feature adds optional inline precision to the existing `pr-review` finding
model. The explicit `pr`/`file`/`surface` scope model remains mandatory for all
findings. Inline anchor data exists only when persisted diff evidence supports
one changed surface and one contiguous interval.

## Entities

### ReviewAnchor

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `surface` | repo-relative path | Yes | The single changed surface the finding can be anchored to. |
| `line_start` | positive integer | Yes | Inclusive 1-based starting line for the anchor. |
| `line_end` | positive integer | No | Inclusive 1-based ending line when the anchor spans multiple lines. |

Validation rules:

- `surface` must match one persisted changed surface already associated with the
  owning finding.
- `line_start` must be greater than zero.
- `line_end`, when present, must be greater than or equal to `line_start`.
- A single-line anchor is represented by omitting `line_end` or by treating a
  same-line range as a line anchor at render time.
- `ReviewAnchor` is invalid when the underlying evidence is ambiguous,
  cross-surface, stale, or split across disjoint spans.

### ReviewFinding

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `category` | enum | Yes | Existing structural review lens. |
| `severity` | enum | Yes | Existing review urgency classification. |
| `title` | string | Yes | Conventional Comment subject. |
| `details` | string | Yes | Reviewer-facing rationale. |
| `scope` | `ConventionalCommentScope` | Yes | Existing explicit `pr`/`file`/`surface` reach. |
| `changed_surfaces` | list of repo-relative paths | Yes | Existing evidence-backed surface trace. |
| `anchor` | `ReviewAnchor` | No | Optional inline precision derived from persisted diff evidence. |

Validation rules:

- Every finding must continue to carry exactly one explicit scope.
- `anchor` may only be present when it is compatible with `changed_surfaces`.
- `anchor.surface` must be one of the finding's `changed_surfaces`.
- Findings with zero changed surfaces may not carry an anchor.
- Findings spanning multiple unresolved surfaces must omit `anchor` and rely on
  explicit scope only.

### ConventionalCommentEntry

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | string | Yes | Existing Conventional Comments kind derived from finding severity and category. |
| `scope` | `ConventionalCommentScope` | Yes | Explicit reach shown in the artifact header. |
| `anchor_text` | string | No | Human-readable anchor text derived from `ReviewAnchor`. |
| `surfaces_text` | string | Yes | Existing changed-surface traceability output. |
| `why_text` | string | Yes | Existing rationale block. |

Validation rules:

- `anchor_text` must be omitted when `anchor` is absent.
- `anchor_text` must remain host-agnostic and readable without external tools.
- Artifact rendering must not change the final readiness disposition or primary
  artifact selection.

Example renderings:

- line precision: `Anchor: tests/reviewer.md:3`
- span precision: `Anchor: tests/reviewer.md:3-4`
- degraded output: omit `Anchor:` entirely and keep the derived `scope` visible
  when the finding is cross-surface, disjoint, stale, or lacks stored diff evidence

## Relationships

- One `ReviewPacket` contains many `ReviewFinding` records.
- One `ReviewFinding` may carry zero or one `ReviewAnchor`.
- One rendered `ConventionalCommentEntry` is derived from one `ReviewFinding`.
- Explicit scope is always rendered; anchor text is conditional on `ReviewAnchor`.

## Derivation State Flow

1. Persisted diff evidence and changed surfaces enter `ReviewPacket` creation.
2. Each `ReviewFinding` is created with explicit scope using the existing scope
   derivation rules.
3. Anchor eligibility is evaluated for that finding.
4. If the evidence resolves to one surface and one contiguous interval, create
   `ReviewAnchor`.
5. Otherwise, omit `ReviewAnchor` and retain scope-only output.

## Rejection Conditions

- The candidate interval spans multiple changed surfaces.
- The candidate interval resolves to multiple disjoint regions.
- The stored coordinates cannot be validated against the persisted diff.
- The finding has no changed surfaces.
- The derived location would require fabricated line numbers or host-specific
  offsets.