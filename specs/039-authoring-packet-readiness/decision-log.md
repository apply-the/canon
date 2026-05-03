# Decision Log: Authoring Experience And Packet Readiness

## D-001: Extend The Existing Clarity Surface

- **Decision**: Add the authoring-lifecycle summary to `inspect clarity`
  instead of creating a new authoring command.
- **Status**: accepted
- **Rationale**: The current clarity pipeline already computes the right class
  of signals for pre-run packet readiness, so the bounded change is to make
  that lifecycle explicit.
- **Consequences**: Renderer and contract updates must stay additive to the
  existing clarity payload.

## D-002: Keep Packet Authority Explicit

- **Decision**: Prefer `brief.md` as the authoritative current-mode brief when
  it exists in a directory-backed packet; otherwise keep authority explicit and
  never promote supporting inputs silently.
- **Status**: accepted
- **Rationale**: This matches the existing carry-forward guidance and preserves
  readiness honesty.
- **Consequences**: Ambiguous folder packets must stay visibly ambiguous even
  when they contain useful supporting notes.

## D-003: Use Shared Lifecycle Surfaces For Authoring Guidance

- **Decision**: Align the shared mode guide, template-facing docs,
  carry-forward example, and inspect-clarity skill instead of rewriting every
  mode-specific surface in this slice.
- **Status**: accepted
- **Rationale**: Shared lifecycle surfaces are the highest-leverage place to
  teach one canonical path from authored brief to publishable packet.
- **Consequences**: Mode-specific skill rewrites remain a follow-on concern only
  if the shared guidance proves insufficient.

## D-004: Keep Packet Shape Bound To Explicit Inputs

- **Decision**: Derive packet shape and supporting inputs only from the explicit
  authored inputs Canon receives, while still treating one directory root plus
  descendant file paths as one directory-backed packet.
- **Status**: accepted
- **Rationale**: Operators often inspect a packet root alongside one supporting
  file while iterating. Treating that shape as generic multi-input obscures the
  real packet boundary and creates false ambiguity.
- **Consequences**: `inspect clarity` keeps directory-backed packets stable,
  does not promote support files to authority, and does not search beyond the
  explicit input boundary.

## D-005: Ship 039 As The `0.39.0` Release Line And Clear The Roadmap

- **Decision**: Treat the authoring-lifecycle slice as the delivered `0.39.0`
  release line and remove Feature 039 from future-work listings once the
  release surfaces align.
- **Status**: accepted
- **Rationale**: The roadmap was intentionally capped at one remaining
  macrofeature, so delivery is incomplete until versioned release surfaces and
  roadmap state match the implementation.
- **Consequences**: `Cargo.toml`, `Cargo.lock`, runtime compatibility
  references, `README.md`, publication guides, `CHANGELOG.md`, release tests,
  and `ROADMAP.md` must move together.

## D-006: Keep Shared Lifecycle Assertions Stable Across Markdown Reflow

- **Decision**: Validate the inspect-clarity shared lifecycle through stable
  wording fragments instead of one exact long sentence.
- **Status**: accepted
- **Rationale**: Markdown line wrapping should not create false failures when
  the lifecycle contract remains unchanged.
- **Consequences**: Docs or skill sync tests still guard the required lifecycle
  semantics while staying resilient to formatting-only edits.