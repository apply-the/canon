# Carry-Forward Packets for Implementation and Refactor

Use a folder-backed input packet when `implementation` or `refactor` needs to
carry forward decisions from an earlier `change`, `architecture`, or
`implementation` packet.

## Core Rule

The current-mode brief is authoritative. Upstream packets ground the brief, but
they do not replace it.

- `brief.md` must restate every field the current mode needs for readiness.
- `source-map.md` records the upstream packet, artifacts, and carried-forward
  decisions or invariants explicitly.
- `selected-context.md` is optional and exists only to narrow a broader
  upstream packet to the current feature slice.
- Real workspace mutation is optional and requires an executable local payload
  such as `patch.diff`, `mutation.patch`, or `execution.patch` inside the
  current packet. Without that payload, approved continuation stays
  recommendation-oriented and completes as `approved-recommendation`.

Canon still auto-binds only from `canon-input/implementation.md`,
`canon-input/implementation/`, `canon-input/refactor.md`, or
`canon-input/refactor/`. Do not expect Canon to infer current inputs from
`.canon/`, published `docs/` packets, or `@last`.

## Recommended Packet Layout

```text
canon-input/
  implementation/
    brief.md
    source-map.md
    selected-context.md
    patch.diff

  refactor/
    brief.md
    source-map.md
    selected-context.md
    patch.diff
```

## What Goes in brief.md

For `implementation`, restate the bounded task mapping, mutation bounds,
allowed paths, safety-net evidence, independent checks, and rollback notes.

For `refactor`, restate preserved behavior, approved exceptions, refactor
scope, untouched surface, safety-net evidence, contract drift, feature audit,
and the final preservation decision.

If you cannot restate those fields clearly, the packet is not ready for
`implementation` or `refactor` yet; go back to `change` and tighten the
boundary first.

If the intent is real bounded mutation after approval, place the executable
local patch payload in the same packet directory. If the packet contains only
`brief.md`, `source-map.md`, and optional narrowed context, Canon can still run
the governed continuation but it will stay non-mutating.

## What Goes in source-map.md

Record these items explicitly:

- upstream mode
- upstream run id or published packet path
- exact artifacts being carried forward
- which decisions or invariants are being reused
- which upstream scope is excluded from the current packet
- any local refinement applied in the current brief

## Broad Architecture Packets

When the upstream source is a broad architecture packet, narrow it before you
run `implementation` or `refactor`.

- Name the feature slice or component explicitly in `brief.md`.
- Put the excluded architecture scope in `Excluded Upstream Scope:`.
- Copy only the relevant excerpts into `selected-context.md` when the original
  packet is too broad to read safely as-is.

## Path Preference

Prefer published `docs/...` references in `source-map.md` when the packet needs
to travel across collaborators. Local `.canon/...` references are acceptable
for short-lived local continuation when the upstream run has not been
published yet.