# Data Model: Review Mode Completion

## ReviewRunInput

- **Purpose**: Represents authored inputs that define a non-PR change package or artifact bundle for review.
- **Fields**:
  - `source_inputs`: persisted authored input paths Canon read for the run
  - `owner`: named human owner for the governed review
  - `risk`: Canon risk classification for the run
  - `zone`: Canon usage zone for the run
- **Rules**:
  - Inputs must stay outside `.canon/`
  - Inputs are document-backed, not diff refs

## VerificationRunInput

- **Purpose**: Represents authored claims, invariants, contracts, or evidence bundles targeted by verification mode.
- **Fields**:
  - `source_inputs`
  - `owner`
  - `risk`
  - `zone`
- **Rules**:
  - Inputs must stay outside `.canon/`
  - Inputs should remain bounded enough that unresolved findings can be tied to explicit source material

## ReviewPacket

- **Purpose**: Durable artifact family for `review`
- **Artifacts**:
  - `review-brief.md`
  - `boundary-assessment.md`
  - `missing-evidence.md`
  - `decision-impact.md`
  - `review-disposition.md`
- **State signals**:
  - `ready-with-review-notes`
  - `awaiting-disposition`
  - `accepted-with-approval`

## VerificationPacket

- **Purpose**: Durable artifact family for `verification`
- **Artifacts**:
  - `invariants-checklist.md`
  - `contract-matrix.md`
  - `adversarial-review.md`
  - `verification-report.md`
  - `unresolved-findings.md`
- **State signals**:
  - `verification-ready`
  - `verification-blocked`

## ReviewDispositionStatus

- **Purpose**: Encodes whether a `review` packet is ready, blocked for missing evidence, or waiting on explicit disposition approval.
- **Values**:
  - `ready-with-review-notes`
  - `awaiting-disposition`
  - `accepted-with-approval`

## VerificationVerdict

- **Purpose**: Encodes the current verification posture for the emitted packet.
- **Values**:
  - `supported`
  - `mixed`
  - `unsupported`
