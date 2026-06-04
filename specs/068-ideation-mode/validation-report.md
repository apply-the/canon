# Validation Report: Brainstorming Ideation Mode

## Structural Validation
- Ensure that the execution of `canon-cli brainstorm` creates the required artifacts: `01-context.md`, `02-options.md`, `03-tradeoffs.md`, `04-open-questions.md`, `05-spikes.md`.

## Logical Validation
- Ensure that the `options.md` and `tradeoffs.md` files contain at least 3 distinct conceptual approaches with pros, cons, and unknowns.
- Ensure that `spikes.md` contains a bounded experiment proposal if there are critical unknowns.

## Independent Validation
- A senior engineer must review the generated `brainstorming` packet to confirm that no schema mutations or implementation code were inadvertently produced, verifying the strict read-only nature of the mode.
