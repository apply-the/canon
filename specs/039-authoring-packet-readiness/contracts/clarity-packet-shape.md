# Contract: Clarity Packet Shape And Authority Guidance

## Purpose

Define the additive contract for how `inspect clarity` should explain file-
backed packet shape and authority in feature `039-authoring-packet-readiness`.

## Required Summary Elements

- packet shape classification
- authority status
- authoritative input list
- supporting input list
- readiness delta list
- next authoring step

## Required Behaviors

- Single-file packets may treat the only authored input as authoritative.
- Directory-backed packets prefer `brief.md` as authoritative when it exists.
- Supporting files such as `source-map.md` and `selected-context.md` remain
  explicit and must not replace the current-mode brief.
- If Canon cannot determine authority safely, the output must say so directly
  and recommend tightening the packet shape.

## Renderer Expectations

- CLI clarity markdown must render the new summary in a dedicated section.
- The section must be readable without requiring JSON inspection.
- The renderer must stay additive to the existing clarity output structure.

## Non-Goals

- No new inspect command
- No hidden packet rewriting
- No new implicit authored-input sources