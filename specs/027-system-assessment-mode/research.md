# Research: System Assessment Mode

## Decision 1: Keep `system-assessment` separate from `architecture`

- **Decision**: Deliver `system-assessment` as a new first-class governed mode
  instead of extending the existing `architecture` mode.
- **Rationale**: The roadmap explicitly distinguishes as-is understanding from
  decision-shaped architecture work. Keeping the modes separate avoids
  collapsing repo archaeology and future-state design into one contract.
- **Alternatives considered**:
  - Extend `architecture` with an as-is switch.
  - Reuse `discovery` for existing-system assessment.

## Decision 2: Require `system-context existing` in the first slice

- **Decision**: Restrict `system-assessment` to `--system-context existing`.
- **Rationale**: The packet is about current-state evidence, coverage, and
  gaps. Allowing `new` would mix speculative architecture shaping into a mode
  whose value depends on present-tense evidence.
- **Alternatives considered**:
  - Allow both `new` and `existing` immediately.
  - Make system context optional.

## Decision 3: Use ISO 42010 coverage language with five view families

- **Decision**: Model the first packet around ISO 42010 stakeholders,
  concerns, views, and coverage, starting with functional, component,
  deployment, technology, and integration views.
- **Rationale**: ISO 42010 gives the assessment a credible vocabulary for what
  was covered, what was skipped, and why. The five views are the highest-value
  engineering views that can be supported credibly in the first slice.
- **Alternatives considered**:
  - Emit a generic architecture summary with no explicit viewpoint language.
  - Attempt a broader enterprise architecture packet in the first slice.

## Decision 4: Make observed findings, inferred findings, and assessment gaps explicit packet concepts

- **Decision**: Require the packet to distinguish directly observed findings,
  bounded inferences, and assessment gaps, with confidence by assessed
  surface.
- **Rationale**: Large-repo and partially observable systems are central to the
  problem statement. The packet has to show what is known and what is not,
  instead of pretending the assessment saw everything.
- **Alternatives considered**:
  - Use generic prose confidence notes only.
  - Leave confidence and gap reporting to reviewers outside the packet.

## Decision 5: Publish under the architecture docs family

- **Decision**: Publish completed packets under
  `docs/architecture/assessments/<RUN_ID>/`.
- **Rationale**: The mode is about architecture understanding, but it is not a
  decision packet. Keeping it under the architecture family makes the
  relationship visible without conflating it with `docs/architecture/decisions/`.
- **Alternatives considered**:
  - Publish under `docs/system-assessments/<RUN_ID>/`.
  - Defer publishing until the separate structured-destination roadmap slice.

## Decision 6: Reuse the operational analysis mode pattern

- **Decision**: Implement `system-assessment` using the nearest stable pattern
  already used by `security-assessment` and `supply-chain-analysis`: authored
  inputs, recommendation-free analysis posture, explicit gates, publishable
  packet, and focused validation.
- **Rationale**: That pattern already fits read-only, evidence-oriented modes
  better than the planning-oriented `architecture` flow and keeps the blast
  radius bounded.
- **Alternatives considered**:
  - Invent a new runtime pipeline just for assessment.
  - Piggyback on the `architecture` service path even though the postures differ.

## Decision 7: Keep large-repo handling bounded in the first release

- **Decision**: Handle large or partially observable repositories through
  declared assessment scope, skipped-surface reporting, and confidence grading
  rather than trying to add general context-window management across all modes.
- **Rationale**: The user discussion identified large-repo support as broader
  than this single feature. Honest bounded coverage is deliverable now without
  widening the slice beyond the roadmap item.
- **Alternatives considered**:
  - Add cross-mode chunking or traversal orchestration as part of this feature.
  - Ignore large-repo posture entirely.