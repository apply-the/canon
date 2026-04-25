# Quickstart: PR Review Conventional Comments

## Goal

Exercise the Conventional Comments slice for `pr-review` and confirm the new
artifact remains additive, approval-safe, and publishable.

## Preconditions

- Canon CLI is installed for the workspace.
- The repository contains a base ref and a head ref with a bounded diff.
- The diff contains at least one review-relevant changed surface.

## High-Impact Review Flow

1. Run `pr-review` on a high-impact diff:

```bash
canon run \
  --mode pr-review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input refs/heads/main \
  --input HEAD
```

2. Confirm the emitted packet contains:

- `review-summary.md`
- `conventional-comments.md`

3. Confirm the run still waits for explicit disposition when must-fix findings
   remain unresolved.

4. Inspect the new artifact and verify:

- every entry uses a valid Conventional Comments kind
- changed surfaces remain visible
- no fake line anchors appear

## Publish Check

1. Publish the run:

```bash
canon publish <RUN_ID>
```

2. Confirm the published packet under `docs/reviews/prs/<RUN_ID>/` includes
   `conventional-comments.md` and that the artifact remains readable without
   internal runtime manifests.
