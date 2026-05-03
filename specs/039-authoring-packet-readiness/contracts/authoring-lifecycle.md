# Contract: Shared Authoring Lifecycle

## Purpose

Define the shared lifecycle story that file-backed Canon surfaces must teach in
feature `039-authoring-packet-readiness`.

## Lifecycle Steps

1. Author or tighten the current-mode packet.
2. Run `canon inspect clarity` on the canonical file or folder-backed inputs.
3. Start the matching governed run only after the packet is clear enough for
   the intended mode.
4. Critique the generated packet or emitted artifacts instead of treating the
   first output as final.
5. Publish only when the packet is actually ready for broader consumption.

## Authority Rules

- The current-mode brief is authoritative for readiness.
- `brief.md` is authoritative when a folder-backed packet includes it.
- `source-map.md` and `selected-context.md` are supporting inputs that may
  ground the packet but must not silently satisfy readiness.
- Canon must keep packet authority explicit when the supplied inputs are
  ambiguous.

## Honesty Rules

- `## Missing Authored Body` remains the preferred signal for absent authored
  sections.
- Clarity guidance may describe readiness deltas and next steps, but it must
  not invent authored content or imply hidden input sources.
- Docs, examples, and skills must describe the same lifecycle without
  contradicting the runtime behavior.