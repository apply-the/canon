# Research: Review Mode Completion

## Decision 1: Reuse the document-backed analysis pipeline for both modes

- **Decision**: Implement `review` and `verification` on the existing document-backed pattern used by `architecture` and `brownfield-change`: authored-input capture, AI generation, explicit critique, persisted artifacts, gate evaluation, and mode-result summarization.
- **Rationale**: This keeps the implementation local-first, artifact-first, and consistent with the rest of Canon. It also minimizes new orchestration concepts while still allowing the modes to differ through artifact contracts, gate profiles, and summaries.
- **Alternatives considered**:
  - Reuse `pr-review` directly: rejected because `review` is not diff-backed and must preserve distinct semantics.
  - Keep support-state wrappers only: rejected because it would not make the modes runnable.

## Decision 2: Use explicit non-AI validation paths where readiness requires independence

- **Decision**: Reuse non-AI validation paths, such as filesystem or shell-backed evidence capture, whenever release-readiness needs to prove independent validation against AI-generated outputs.
- **Rationale**: Canon policy already requires distinct validation for AI generation. Brownfield-change already demonstrates a workable pattern.
- **Alternatives considered**:
  - Treat AI critique alone as independent validation: rejected because it violates the configured independence policy.
  - Introduce a new human-review adapter now: rejected because it would broaden scope prematurely.

## Decision 3: Preserve dedicated review disposition semantics for `review`

- **Decision**: `review` will emit a dedicated `review-disposition.md` primary artifact and use `GateKind::ReviewDisposition` to preserve explicit approval handling for unresolved must-fix findings.
- **Rationale**: This reuses the core disposition concept from `pr-review` without pretending the output is a diff review summary.
- **Alternatives considered**:
  - Reuse `review-summary.md` unchanged: rejected because it would blur `review` and `pr-review`.

## Decision 4: `verification` blocks on unresolved findings instead of using disposition approval in the first slice

- **Decision**: `verification` will use release-readiness blocking when unresolved findings remain open, instead of introducing a separate approval loop in the first implementation slice.
- **Rationale**: The mode is challenge-oriented rather than disposition-oriented, and its first responsibility is to surface unresolved contradictions clearly.
- **Alternatives considered**:
  - Introduce a new approval gate kind for verification: rejected because it expands governance semantics before the base workflow exists.

## Decision 5: Documentation and roadmap alignment are part of the feature closeout, not optional polish

- **Decision**: `README.md`, `MODE_GUIDE.md`, `NEXT_FEATURES.md`, and the runnable skill contracts must be updated in the same feature tranche that changes runtime truth.
- **Rationale**: Canon explicitly forbids public guidance that misstates support state after behavior ships.
- **Alternatives considered**:
  - Defer docs until after runtime: rejected because it creates a knowingly false product surface for 0.7.0.
