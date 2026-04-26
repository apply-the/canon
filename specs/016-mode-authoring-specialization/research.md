# Research: Mode Authoring Specialization

## Decision 1: Bound the first slice to requirements, discovery, and change

- **Decision**: Deliver the first authored-body specialization slice only for `requirements`, `discovery`, and `change`.
- **Rationale**: These modes still rely heavily on summary-derived or placeholder renderer output, so the slice yields visible value without reopening already-delivered reference implementations (`backlog`, `architecture`, `pr-review`).
- **Alternatives considered**:
  - Update all remaining modes in one slice: rejected because the risk would exceed bounded-impact and make task review noisy.
  - Update only one mode: rejected because the feature would look mode-specific instead of establishing a reusable specialization pattern.

## Decision 2: Reuse one shared missing-body marker across all authored-body modes

- **Decision**: Use a shared `## Missing Authored Body` marker for new authored-body fallbacks, and keep any existing mode-specific constant aliasing that literal when compatibility matters.
- **Rationale**: A single marker simplifies tests, docs, and reviewer expectations across modes while preserving existing architecture behavior.
- **Alternatives considered**:
  - Unique marker per mode: rejected because it fragments the honesty contract and complicates tests.
  - Keep only the C4 constant name: rejected because the new slice is broader than architecture.

## Decision 3: Renderer specialization should operate on authored context, not only generated summaries

- **Decision**: Update the relevant renderer call paths so authored `context_summary` content reaches the per-mode renderer, then extract canonical H2 sections from that authored body.
- **Rationale**: `requirements` currently renders from generated evidence, and `change`/`discovery` mix authored and derived summaries unevenly. The specialization feature only works if the renderer sees the original authored structure.
- **Alternatives considered**:
  - Preserve summary-only rendering and improve prompts: rejected because it still allows generic filler and hides missing authored content.
  - Parse authored sections only in the orchestrator and bypass the renderer: rejected because the authored-body contract belongs in the artifact rendering layer.

## Decision 4: Update the existing template and example filenames instead of creating a new folder hierarchy

- **Decision**: Modify the current docs inputs in `docs/templates/canon-input/*.md` and `docs/examples/canon-input/*.md` for the targeted modes.
- **Rationale**: The repository already uses a stable per-mode docs naming convention, and adding a second hierarchy would create drift and duplicate discoverability paths.
- **Alternatives considered**:
  - Introduce nested `<mode>/brief.md` paths for all three modes: rejected because it creates needless migration work and parallel conventions.

## Decision 5: Validator scripts change only if new skill wording requires it

- **Decision**: Treat `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1` as unchanged by default; only touch them if the new skill sections introduce validator-sensitive phrases.
- **Rationale**: The validator already checks support-state, overlap boundaries, and fake-run protections. Expanding it without necessity adds maintenance risk.
- **Alternatives considered**:
  - Preemptively add validator rules for the new authoring sections: rejected because the first slice does not need a new validator contract unless a real gap appears.