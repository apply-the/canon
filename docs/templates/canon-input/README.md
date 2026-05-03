# Canon Input Templates

These starter templates are the canonical authored-input entrypoints for
file-backed Canon modes.

## Shared Lifecycle

Across file-backed modes, keep one explicit path:

1. Choose the mode-specific template that matches the governed work.
2. Tighten the current-mode brief until it states the bounded work clearly.
3. Run `canon inspect clarity` on the canonical file or folder-backed packet.
4. Start the matching governed run only after the packet is ready enough for
   that mode.
5. Critique the emitted packet before you publish it.
6. Publish only when the packet is truly ready for wider readers.

## Authority Rules

- A single canonical file such as `canon-input/change.md` can be the whole
  current-mode brief.
- In a folder-backed packet, `brief.md` is the authoritative current-mode
  brief when it exists.
- Supporting files such as `source-map.md` and `selected-context.md` may ground
  the packet, but they do not replace the current-mode brief.
- If Canon cannot identify one authoritative brief safely, tighten the packet
  shape before you start the governed run.

## Honesty Rules

- Missing authored sections should stay explicit.
- `## Missing Authored Body` is stronger than generated filler.
- Canon must not infer current inputs from `.canon/`, published packets, open
  tabs, or any other incidental surface.