# Contract: Output Quality Posture

## Purpose

Define how Canon classifies packet quality and how that posture must appear in
inspect results, runtime summaries, and fallback-heavy artifacts for feature
034.

## Shared Postures

### `structurally-complete`

- Use when required sections exist but the authored support is still too weak
  for Canon to imply strong reasoning quality.
- This posture MUST carry at least one explicit downgrade reason.
- This posture MUST NOT sound publish-ready in summaries or rendered artifacts.

### `materially-useful`

- Use when the packet has bounded intent, meaningful authored support, and no
  blocking honesty markers, but still lacks one or more conditions required for
  a `publishable` posture.
- This posture SHOULD name the strongest supporting evidence signals.
- This posture MAY still carry non-blocking downgrade reasons.

### `publishable`

- Use when the packet has explicit authored support strong enough that a reader
  can trust the packet without hidden context.
- This posture MUST NOT appear when a blocking honesty marker or unresolved
  critical downgrade reason remains.
- This posture is descriptive of packet quality and does not alter publish
  command semantics.

## Inspect Contract

- Targeted inspect results MUST include the computed posture.
- Targeted inspect results MUST include explicit evidence signals or downgrade
  reasons that explain the posture.
- Materially closed decisions MUST remain visible so Canon does not invent
  balanced alternatives solely to upgrade posture.

## Summary Contract

- Targeted runtime summaries MUST reflect the shared posture instead of generic
  ready-sounding language.
- Summaries MUST remain explicit when the packet is only
  `structurally-complete`.
- Summaries MAY name `publishable` only when the supporting evidence is
  explicit.

## Artifact Contract

- Fallback-heavy artifacts in scope MUST preserve explicit missing-body or
  downgrade language when authored support is missing or weak.
- Artifacts MUST NOT use synthetic prose that can be mistaken for approved
  reasoning.
- Existing honesty markers remain authoritative and must survive any posture
  upgrade logic.

## Release Alignment Contract

- Shared skill references, runtime-compatibility anchors, impacted docs,
  roadmap text, and changelog entries MUST describe the same posture semantics
  as the engine.
- The delivered release value for this feature is `0.34.0`.