# Research: PR Review Conventional Comments

## R-001: Deliver Conventional Comments as an additive artifact

- **Decision**: Add a reviewer-facing artifact to the `pr-review` packet rather
  than replacing `review-summary.md`.
- **Rationale**: Status, inspect, approval, and next-step surfaces already rely
  on `review-summary.md` as the canonical summary. Replacing it would widen the
  blast radius without increasing review fidelity.
- **Alternatives considered**:
  - Replace `review-summary.md` entirely. Rejected because it couples formatting
    work to lifecycle semantics.
  - Render Conventional Comments only in CLI output. Rejected because the
    feature must remain publishable and artifact-first.

## R-002: Keep the first slice surface-scoped, not line-scoped

- **Decision**: Conventional Comments entries will reference changed surfaces
  and persisted findings, not fake line numbers or code-host anchors.
- **Rationale**: The current `ReviewPacket` stores changed surfaces and review
  findings, but it does not persist enough structured position data to support
  truthful inline comments.
- **Alternatives considered**:
  - Infer approximate patch-line anchors from diff hunks. Rejected because the
    current packet model does not make those anchors durable or governable.
  - Emit host-specific comment payloads immediately. Rejected because that is a
    protocol/export feature, not an artifact-shape feature.

## R-003: Use deterministic finding-to-comment mapping

- **Decision**: Map persisted review findings to Conventional Comments kinds
  using deterministic rules grounded in severity and category.
- **Rationale**: Reviewer-facing output must be stable across runs and tests,
  and it must not soften must-fix findings into generic prose.
- **Alternatives considered**:
  - Freeform model-selected comment kinds. Rejected because it would make tests
    and publish behavior unstable.
  - A single `issue` label for every finding. Rejected because it discards the
    expressive value of Conventional Comments for suggestions, questions, and
    note-level observations.

## R-004: First-slice mapping table

- **Decision**: Use the following deterministic mapping in the first slice:

  | Review finding | Conventional Comments kind |
  |----------------|----------------------------|
  | `MustFix + BoundaryCheck` | `issue` |
  | `MustFix + ContractDrift` | `issue` |
  | `MustFix + MissingTests` | `todo` |
  | `MustFix + DecisionImpact` | `question` |
  | `Note + DuplicationCheck` | `praise` |
  | any other `Note` finding | `thought` |

- **Rationale**: This preserves severity intent for must-fix findings, gives a
  useful positive/note shape for the current note-only packet, and stays stable
  under test.
- **Alternatives considered**:
  - Map every must-fix finding to `issue`. Rejected because it removes the
    useful distinction between required follow-up work and unanswered decision
    prompts.
  - Use `suggestion` for `MissingTests`. Rejected because it softens a current
    must-fix condition.
