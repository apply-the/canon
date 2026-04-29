# Contract: Supply Chain Packet Shape

## Purpose

Define the external packet contract that `supply-chain-analysis` must emit so a
reviewer can inspect the bounded supply-chain posture without reading chat or
raw scanner output first.

## Artifact Family

- `analysis-overview.md`
- `sbom-bundle.md`
- `vulnerability-triage.md`
- `license-compliance.md`
- `legacy-posture.md`
- `policy-decisions.md`
- `analysis-evidence.md`

## Required Sections

### `analysis-overview.md`

- `Summary`
- `Declared Scope`
- `Licensing Posture`
- `Distribution Model`
- `Ecosystems In Scope`
- `Out Of Scope Components`

### `sbom-bundle.md`

- `Summary`
- `SBOM Outputs`
- `Covered Manifest Surface`
- `Coverage Gaps`

### `vulnerability-triage.md`

- `Summary`
- `Findings By Severity`
- `Exploitability Notes`
- `Triage Decisions`
- `Coverage Gaps`

### `license-compliance.md`

- `Summary`
- `Compatibility Classes`
- `Flagged Incompatibilities`
- `Obligations`
- `Coverage Gaps`

### `legacy-posture.md`

- `Summary`
- `Outdated Dependencies`
- `End Of Life Signals`
- `Abandonment Signals`
- `Modernization Slices`

### `policy-decisions.md`

- `Summary`
- `Scanner Decisions`
- `Accepted Risks`
- `Deferred Actions`
- `Open Policy Questions`

### `analysis-evidence.md`

- `Summary`
- `Source Inputs`
- `Tool Versions`
- `Independent Checks`
- `Deferred Verification`

## Publish Contract

- Publish destination: `docs/supply-chain/<RUN_ID>/`
- Recommendation-only posture must remain explicit in the packet summary.
- Missing scanner or posture coverage must be called out in the affected
  artifact instead of being hidden in raw evidence.

## Contract Notes

- Machine-readable SBOMs may persist as companion artifacts or payload
  references, but the human packet must link them from `sbom-bundle.md`.
- Coverage-gap and missing-decision markers are part of the contract, not a
  failure to deliver the packet.