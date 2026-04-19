# Decision Log: Review Mode Completion

## D-001: One shared runtime substrate for both modes

- **Decision**: Build `review` and `verification` on a shared document-backed orchestration pattern.
- **Rationale**: Minimizes new runtime concepts while preserving mode-specific artifact and gate semantics.

## D-002: Review disposition remains an explicit approval-aware surface

- **Decision**: Keep `GateKind::ReviewDisposition` for `review` and expose it through `review-disposition.md`.
- **Rationale**: Reuses existing approval concepts without conflating `review` with `pr-review`.

## D-003: Verification unresolved findings block readiness in the first slice

- **Decision**: Use release-readiness blocking instead of inventing a new verification-specific approval gate in the first tranche.
- **Rationale**: Keeps the first slice smaller and aligned to the mode's challenge-first purpose.

## D-004: 0.7.0 documentation alignment is required feature scope

- **Decision**: User-facing docs and roadmap text must ship with the runtime truth update.
- **Rationale**: Canon's public guidance must not lag behind shipped support state.

## D-005: Clean-path review and verification prompts must stay semantically neutral

- **Decision**: Adapter summaries for successful `review` and `verification` paths must avoid blocker keywords unless the authored packet actually warrants them.
- **Rationale**: Result rendering and gate evaluation intentionally treat phrases like missing evidence, unsupported claims, contradictions, and unresolved findings as semantic blockers. Clean-path prompt text therefore has to stay neutral or the runtime falsely reports gated or blocked outcomes.

## D-006: Support-state truth must be enforced across skills, preflight, and validators

- **Decision**: `canon-review` and `canon-verification` ship as `available-now` executable wrappers, and that truth must be reflected consistently across materialized skills, embedded skill sources, shared preflight scripts, validator expectations, and shared references.
- **Rationale**: Canon's discoverability surface is part of the product contract. Leaving stale modeled-only or intentionally-limited labels in any one surface undermines user trust and produces invalid preflight behavior even when the runtime is correct.
