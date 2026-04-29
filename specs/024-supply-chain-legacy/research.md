# Research: Supply Chain And Legacy Analysis Mode

## Decision 1: Reuse Existing Shell And Filesystem Adapters

- **Decision**: Implement scanner-backed evidence collection by reusing the
  existing `ShellAdapter` for tool invocation and `FilesystemAdapter` for
  bounded repository reads.
- **Rationale**: The repo already supports governed shell and filesystem
  invocations, persists request and decision records, and distinguishes
  generation from validation without needing a new adapter class.
- **Alternatives considered**:
  - Create a dedicated scanner adapter: rejected because it widens the adapter
    surface before the repo proves the first supply-chain slice.
  - Parse lockfiles entirely inside Canon: rejected because it duplicates
    established tools and weakens evidence credibility.

## Decision 2: Keep Canonical Inputs On The Full Mode Name

- **Decision**: Use `canon-input/supply-chain-analysis.md` and
  `canon-input/supply-chain-analysis/` as the only canonical auto-bind
  locations.
- **Rationale**: Repo guidance already standardizes canonical input binding on
  the exact mode string, and deviating here would create avoidable drift in
  skills, runtime checks, and user expectations.
- **Alternatives considered**:
  - Shorten to `canon-input/supply-chain.md`: rejected because it would diverge
    from the repository's file-backed mode convention.

## Decision 3: Publish To `docs/supply-chain/<RUN_ID>/`

- **Decision**: Publish completed or publishable packets to
  `docs/supply-chain/<RUN_ID>/`.
- **Rationale**: The path is readable, avoids awkward pluralization, and keeps
  the documentation destination clearly tied to the domain without overloading
  the mode name.
- **Alternatives considered**:
  - `docs/supply-chain-analyses/<RUN_ID>/`: rejected because the path is noisy
    and harder to scan.
  - `docs/supply-chain-analysis/<RUN_ID>/`: rejected because the singular noun
    reads more like a source file bucket than a packet collection.

## Decision 4: Treat Missing Scanners As Coverage Gaps, Not Silent Success

- **Decision**: When a required scanner is missing or intentionally skipped,
  record a scanner decision artifact and surface an explicit coverage-gap marker
  in the affected packet sections.
- **Rationale**: The new mode must stay honest about partial analysis while
  still producing a reviewable packet. Tool unavailability is evidence, not a
  reason to fabricate clean results.
- **Alternatives considered**:
  - Hard-fail the entire run on any missing scanner: rejected because it hides
    useful bounded analysis that can still be reviewed.
  - Treat missing scanners as warnings only: rejected because it weakens the
    auditable boundary between covered and uncovered ecosystems.

## Decision 5: Keep Missing Tool Installation As A User Decision

- **Decision**: Canon will suggest install, skip, or replacement choices but
  never execute the installation itself.
- **Rationale**: This preserves recommendation-only posture, avoids privileged
  side effects, and keeps the user's tool-policy choice explicit in evidence.
- **Alternatives considered**:
  - Auto-install OSS tools by default: rejected because it changes the system
    under analysis and expands blast radius.
  - Forbid install guidance entirely: rejected because it would make the mode
    harder to use when scanners are missing.

## Decision 6: Start With The Seven-Artifact Packet From The Roadmap

- **Decision**: The first slice will emit `analysis-overview.md`,
  `sbom-bundle.md`, `vulnerability-triage.md`, `license-compliance.md`,
  `legacy-posture.md`, `policy-decisions.md`, and `analysis-evidence.md`.
- **Rationale**: The roadmap already establishes this artifact family, and it
  cleanly separates scope, evidence, findings, and policy decisions.
- **Alternatives considered**:
  - Collapse findings into fewer files: rejected because it would make review
    harder and blur legal, security, and modernization concerns.

## Decision 7: Make Coverage A Hard Closeout Criterion

- **Decision**: The final closeout will require evidence of at least 85% line
  coverage for each Rust file added or modified by this feature.
- **Rationale**: The user requested high coverage on the touched Rust files and
  the repo already benefits most from direct `EngineService` coverage rather
  than CLI-only smoke tests.
- **Alternatives considered**:
  - Use a workspace-wide aggregate threshold only: rejected because it can hide
    under-tested new runtime surfaces.
  - Leave coverage qualitative: rejected because it is not enforceable enough
    for this feature contract.