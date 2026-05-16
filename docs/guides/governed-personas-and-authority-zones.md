# Governed Personas And Authority Zones

Canon publishes first-slice authority semantics through
`authority-governance-v1` so downstream runtimes can consume stable governed
meaning without reading Canon source code.

## Authority Zones

- `green`: standard bounded delivery posture
- `yellow`: elevated review posture before delivery continues
- `red`: advisory-only packet; downstream delivery mutation should stop
- `restricted`: a human gate is still required before continuation

## Change Classes

- `low-impact`
- `bounded-impact`
- `systemic-impact`
- `critical-operations`

`change_class` is Canon-owned semantic vocabulary. It is not a direct runtime
instruction by itself.

## Intended Personas

Canon attaches one intended persona to each governed mode. First-slice personas
include:

- `product-strategist`
- `system-architect`
- `delivery-engineer`
- `verification-lead`
- `operations-governor`
- `domain-steward`

Canon may also publish `persona_anti_behaviors` to describe what the mode should
avoid. Missing anti-behaviors do not invalidate the required persona meaning.

## Advisory Stage Role Hints

`stage_role_hints` are optional advisory metadata. They can suggest downstream
review capability or posture, but they must never assign executable councils,
providers, models, or final decision authority.

## Consumer Expectations

Downstream consumers should:

- require the documented `authority-governance-v1` required fields
- ignore missing optional metadata safely
- reject unsupported contract lines
- keep runtime control, council selection, and stop behavior on the consumer side
