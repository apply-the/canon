# Contract: Governed Reasoning Posture v2 Migration And Coexistence

## Summary

This contract defines how `governed_reasoning_posture_v1` and
`governed_reasoning_posture_v2` may coexist during release rollout and when
mixed inputs must be rejected.

## Coexistence Rule

- Canon may publish both `v1` and `v2` only when exactly one line is marked
  `active` and the other is explicitly marked `legacy`.
- There is no implicit fallback between lines.
- Consumers validate the active line they support and reject ambiguous or mixed
  publication states.

## Valid Publication States

- `v2` active, `v1` legacy
- `v1` active, `v2` unpublished
- `v1` legacy only in historical records, not as an active current line

## Invalid Publication States

- `v1` and `v2` both marked active
- `v1` and `v2` both published without an explicit active-versus-legacy marker
- a release surface that mixes `v1` and `v2` semantics inside one posture input
- a `v2`-required workflow receiving only `v1` data
- a `v1`-only consumer receiving `v2` data and attempting implicit downgrade

## Consumer Expectations

- A `v1`-only consumer rejects a `v2` payload with an explicit version-line
  incompatibility reason.
- A `v2`-capable consumer may accept `v2` when the line is active and may treat
  `v1` as legacy only when the release explicitly marks it as such.
- A workflow that explicitly requires `v2` rejects `v1` even if `v1` is still
  published as legacy.

## Migration Expectations

- Canon publishes the semantic delta from `v1` to `v2` in stable docs and
  release notes.
- Release metadata must align with the active line and its compatibility window.
- Example validation must include at least one valid dual-line release and one
  migration rejection case.

## Executable Fixture Expectations

- `dual-line-coexistence-valid`: accept because exactly one line is `active`
  and the other is `legacy`.
- `dual-line-coexistence-ambiguous`: reject because dual-line publication is
  ambiguous.
- `migration-rejection-v2-to-v1-consumer`: reject because a v1-only consumer
  cannot consume `v2`.
- `migration-rejection-v1-to-v2-required`: reject because a v2-required workflow
  cannot consume `v1`.

## Validation Expectations

Executable validation must reject:

- any dual-line release without exactly one active line and one legacy line
- implicit line fallback or precedence
- mixed publication states that prevent one authoritative consumer decision
- stale release metadata that names support inconsistent with the active line