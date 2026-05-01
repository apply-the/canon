# Contract: Structured Publish Destination

## Scope

This contract defines the default external destination behavior for published
Canon packets in the first 029 slice.

## Destination Shape

- Default publish destinations MUST remain under the current external family
  roots already associated with each mode.
- The default leaf directory MUST use a structured shape equivalent to
  `<YYYY-MM-DD>-<descriptor>/` rather than `<RUN_ID>/`.
- The default descriptor MUST be human-readable and stable for the same run.
- Explicit `publish --to` overrides MUST bypass the default destination shape
  and write directly to the provided destination.

## Descriptor Rules

- Use persisted descriptive metadata when available.
- Fall back to a stable mode-derived descriptor when no descriptive metadata is
  available.
- When the default destination is already occupied by another run, suffix the
  leaf with a short run-id fragment rather than abandoning the structured path.
- The descriptor MUST NOT replace canonical run identity.
- The descriptor source MUST be independent from approval or run-state changes.

## Safety Rules

- Publishing MUST continue to fail when the destination already exists as a
  non-directory path.
- Existing publish eligibility rules for completed and approval-gated
  operational packets MUST remain unchanged.
- Structured default destinations MUST remain outside `.canon/`.